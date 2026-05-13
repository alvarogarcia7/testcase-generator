# Using Projector for Build Result Caching

This document explains how to use the Projector tool to cache and track Rust compilation results across commits and worktrees in the testcase-generator project.

## Overview

Projector is a project health tracking tool that maintains a SQLite database of build, test, and lint check results per commit. This enables:

- **Result Caching**: Track which commits have successful builds to avoid redundant compilations
- **Cross-Worktree Visibility**: See build status across multiple git worktrees and branches
- **Performance Tracking**: Monitor compilation times and test success rates over time
- **CI Integration**: Log results in CI/CD pipelines for automated tracking

## Setup

### Installation

Projector has already been installed and configured for this project. The local database is located at:

```
.projector.db
```

### Configured Project

The testcase-generator project has been registered with Projector with:

- **Project Name**: `testcase-generator`
- **Description**: TCMS Test Case Manager - Workspace f728-in-the-test-acce
- **Repository**: Current directory

### Configured Checks

Four checks are tracked:

| Check | Type | Description | Mandatory |
|-------|------|-------------|-----------|
| build | Mandatory | `cargo build --workspace` (debug) | Yes |
| tests | Mandatory | `cargo test --workspace --all-features` | Yes |
| lint | Optional | `cargo clippy` with warnings as errors | No |
| release-build | Optional | `cargo build --workspace --release` | No |

### Configured Worktrees

Two worktrees are tracked:

- `main` - Main branch
- `f728-in-the-test-acce` - Current feature branch

## Usage

### Automatic Logging with Helper Script

The simplest way to use Projector is with the helper script:

```bash
# Log all check results for current branch (auto-detected)
./scripts/log-build-results.sh

# Log results for specific worktree
./scripts/log-build-results.sh main

# Log results for specific check only
./scripts/log-build-results.sh main build
./scripts/log-build-results.sh f728-in-the-test-acce test

# Log specific mode
./scripts/log-build-results.sh main lint
./scripts/log-build-results.sh main release
```

**What the script does:**

1. Auto-detects current git branch (if not provided)
2. Runs the requested build/test/lint checks
3. Logs pass/fail results to the Projector database
4. Displays the current build status in a table

### Manual Logging with projector CLI

If you prefer to run checks manually and log results afterward:

```bash
# Run checks independently
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings

# Log results if they passed
proj log testcase-generator f728-in-the-test-acce \
  --sha $(git rev-parse HEAD) \
  --ci build=pass \
  --ci tests=pass \
  --ci lint=pass
```

For failed checks:

```bash
proj log testcase-generator f728-in-the-test-acce \
  --sha $(git rev-parse HEAD) \
  --ci build=fail:"compilation error in module X" \
  --ci tests=warn:"coverage dropped 5%"
```

### View Build Status

#### Current Status Per Worktree

```bash
proj status testcase-generator
```

Shows the latest build status for all worktrees:

```
              testcase-generator
┌──────────────────────┬──────┬────────┬──────┬──────────────┐
│ Worktree             │ SHA  │ Build  │ Test │ Lint         │
├──────────────────────┼──────┼────────┼──────┼──────────────┤
│ main                 │ a1b2 │ ✓ pass │ ✗ fail │ – skip    │
│ f728-in-the-test-acce│ 9f8e │ ✓ pass │ ✓ pass │ ⚠ warn    │
└──────────────────────┴──────┴────────┴──────┴──────────────┘
```

#### History for Specific Worktree

```bash
proj status testcase-generator f728-in-the-test-acce
```

Shows all commits on the feature branch with their check results.

#### Specific Commit Details

```bash
proj status testcase-generator f728-in-the-test-acce abc123def456
```

Shows detailed information about a specific commit.

### Generate Reports

#### Table Report

```bash
proj report testcase-generator
```

Generates a formatted table of all results.

#### CSV Export

```bash
proj report testcase-generator --format csv > build-report.csv
```

Export results for spreadsheet analysis.

#### JSON Export

```bash
proj report testcase-generator --format json > build-report.json
```

Export structured data for programmatic processing.

#### Filtered Reports

```bash
# Results only from main branch since January 2026
proj report testcase-generator --worktree main --since 2026-01-01 --format json
```

## Build Result Caching Strategy

### Checking Before Rebuilding

To determine if a commit's build has already been cached:

```bash
# Check if specific commit has passed build check
proj status testcase-generator f728-in-the-test-acce $(git rev-parse HEAD)

# If output shows "build: ✓ pass", the build was cached
```

### Using Cached Results

Example workflow:

```bash
#!/bin/bash
CURRENT_SHA=$(git rev-parse HEAD)

# Check if build is already cached (passed on this commit)
if proj status testcase-generator f728-in-the-test-acce "$CURRENT_SHA" 2>/dev/null | grep -q "build.*pass"; then
    echo "Build cache hit! Skipping compilation..."
    exit 0
else
    echo "Build cache miss. Running compilation..."
    cargo build --workspace

    # Log results
    ./scripts/log-build-results.sh f728-in-the-test-acce build
fi
```

### Cache Invalidation

The Projector database tracks results per commit SHA. Cache is automatically "invalidated" when:

- Moving to a different commit
- Modifying code (new commit SHA)
- Running `cargo clean` (resets compilation artifacts regardless of cache)

## Integration with CI/CD

### GitHub Actions Example

```yaml
name: Build and Cache

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Check projector cache
        run: |
          # Try to use cache if available
          if proj status testcase-generator ${{ github.ref }} ${{ github.sha }} | grep -q "build.*pass"; then
            echo "Using cached build"
            exit 0
          fi

      - name: Run build with projector logging
        run: ./scripts/log-build-results.sh ${{ github.ref }}
        if: failure() != true  # Only if cache check didn't pass
```

### GitLab CI Example

```yaml
build:
  script:
    # Log results to projector
    - ./scripts/log-build-results.sh $(git rev-parse --abbrev-ref HEAD)
  artifacts:
    reports:
      dotenv: build-status.env
  after_script:
    # Export results for downstream jobs
    - proj report testcase-generator --format json > build-report.json
```

## Database Schema

The Projector database stores:

| Table | Purpose |
|-------|---------|
| projects | Registered projects |
| worktrees | Git branches/worktrees per project |
| checks | Build/test/lint checks |
| commits | Commit snapshots (SHA, message, author) |
| check_results | Individual check results per commit |
| sync_log | History of database sync operations |
| conflict_log | Conflict resolution history |

To examine the database directly:

```bash
sqlite3 .projector.db "SELECT * FROM check_results LIMIT 10;"
```

## Best Practices

### 1. Log After Every Build

Always log results to maintain cache accuracy:

```bash
# After making changes
cargo build --workspace
./scripts/log-build-results.sh f728-in-the-test-acce build
```

### 2. Run Full Suite Before Commits

Before committing, run the full check suite:

```bash
./scripts/log-build-results.sh f728-in-the-test-acce all
```

### 3. Use Mandatory Checks

Mandatory checks (build, tests) will cause CI to fail if they don't pass, preventing broken code from merging.

### 4. Monitor Trends

Use CSV/JSON exports to track compilation time trends:

```bash
# Export weekly reports
proj report testcase-generator --format csv --since 2026-05-06 > week.csv
```

### 5. Clean Database Periodically

Archive old checks when they're no longer relevant:

```bash
# Archive a check
proj check archive testcase-generator old-check

# List archived checks
proj check list testcase-generator --show-archived
```

## Troubleshooting

### Projector command not found

Install projector:

```bash
cd ~/repos/projector
uv pip install -e .
```

Or run via uv:

```bash
uv run --project ~/repos/projector proj status testcase-generator
```

### Database locked

The database is single-writer, multiple-reader. If you see "database is locked":

1. Check if another projector command is running
2. Wait a moment and retry
3. Reset the database if needed: `rm .projector.db && proj init --local`

### Results not being logged

Ensure:

1. You're in the testcase-generator directory
2. Current branch matches a configured worktree
3. Projector is installed and accessible
4. The local `.projector.db` file exists

Debug:

```bash
proj project show testcase-generator
proj worktree list testcase-generator
proj check list testcase-generator
```

### Cached results are stale

Projector caches per commit SHA. If you rebuild the same commit:

```bash
# Force rebuild even if cached
cargo clean
cargo build --workspace

# Update cache
./scripts/log-build-results.sh f728-in-the-test-acce build
```

## Advanced Usage

### Syncing Across Machines

Export database from one machine:

```bash
proj sync export --output ~/Dropbox/projector.db
```

Import on another machine:

```bash
proj sync import ~/Dropbox/projector.db
```

Or via SSH:

```bash
scp user@other-machine:~/.projector/projector.db /tmp/remote.db
proj sync import /tmp/remote.db
```

### Removing Project Data

Delete all tracked data for the project:

```bash
proj project remove testcase-generator --yes
```

This will permanently delete all check results and commit history.

## Reference

For complete Projector documentation, see:

- `~/repos/projector/README.md` - Full documentation
- `~/repos/projector/QUICKREF.md` - Quick reference guide
- `~/repos/projector/IMPLEMENTATION.md` - Architecture details

## Summary

Projector provides:

✓ Build result caching per commit
✓ Cross-worktree visibility
✓ CI/CD integration
✓ Historical tracking
✓ Cross-machine sync
✓ Multiple export formats

Use `./scripts/log-build-results.sh` for simple result logging, or the `proj` CLI directly for advanced operations.
