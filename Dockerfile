# Stage 1: deps - Build dependencies only
FROM rust:1.92-bookworm as deps

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy src/main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    mkdir -p src/bin && \
    echo "fn main() {}" > src/bin/validate-yaml.rs

# Build dependencies (this will be cached)
RUN cargo build --release

# Stage 2: builder - Build the actual application
FROM rust:1.92-bookworm as builder

WORKDIR /app

# Copy dependencies from deps stage
COPY --from=deps /app/target target
COPY --from=deps /usr/local/cargo /usr/local/cargo

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy full source
COPY src ./src
COPY examples ./examples
COPY tests ./tests

# Build the application against cached dependencies
RUN cargo build --release

# Stage 3: runtime - Final lightweight image
FROM debian:bookworm-slim as runtime

# Install runtime dependencies: git, inotify-tools for watch mode, and make
RUN apt-get update && \
    apt-get install -y git inotify-tools make && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy only the compiled binaries (not auxiliary build files)
COPY --from=builder /app/target/release/testcase-manager /usr/local/bin/tcm
COPY --from=builder /app/target/release/validate-yaml /usr/local/bin/
COPY --from=builder /app/target/release/validate-json /usr/local/bin/
COPY --from=builder /app/target/release/trm /usr/local/bin/
COPY --from=builder /app/target/release/test-verify /usr/local/bin/
COPY --from=builder /app/target/release/test-executor /usr/local/bin/
COPY --from=builder /app/target/release/editor /usr/local/bin/
COPY --from=builder /app/target/release/test-orchestrator /usr/local/bin/

# Copy data directory
COPY data ./data

# Copy scripts directory for watch and validation functionality
COPY scripts ./scripts

# Copy Makefile for convenient commands
COPY Makefile ./Makefile

# Make scripts executable
RUN chmod +x scripts/*.sh && \
    find scripts -type f -name "*.sh" -exec chmod +x {} \;

# Create a helper script for easy watch mode usage
RUN cat > /usr/local/bin/watch-yaml << 'WATCHEOF'
#!/bin/bash
# Helper script to start watch mode easily
cd /app
exec ./scripts/watch-yaml-files.sh "$@"
WATCHEOF

RUN chmod +x /usr/local/bin/watch-yaml

# Create watch mode quick reference
RUN cat > /app/DOCKER_WATCH_GUIDE.md << 'WATCHGUIDE'
# Docker Watch Mode Quick Guide

## Starting Watch Mode

The easiest way to start watch mode in Docker:

\`\`\`bash
docker run -it --rm -v $(pwd)/testcases:/app/testcases testcase-manager:latest watch-yaml
\`\`\`

This will:
1. Mount your local testcases/ directory into the container
2. Start monitoring for changes to YAML files
3. Automatically validate files when they change
4. Show real-time validation results

## How It Works

- Uses **inotify** (Linux file monitoring) to detect changes instantly
- Validates files against the JSON schema in data/schema.json
- Caches validation results for fast re-validation
- Shows color-coded output: ✓ PASSED (green) or ✗ FAILED (red)

## Alternative Methods

### Using make
\`\`\`bash
docker run -it --rm -v $(pwd)/testcases:/app/testcases testcase-manager:latest make watch
\`\`\`

### Using the script directly
\`\`\`bash
docker run -it --rm -v $(pwd)/testcases:/app/testcases testcase-manager:latest ./scripts/watch-yaml-files.sh
\`\`\`

### Watch a custom directory
\`\`\`bash
docker run -it --rm -v $(pwd)/custom:/app/custom testcase-manager:latest bash -c \
    "SCHEMA_FILE=data/schema.json ./scripts/validate-files.sh --pattern '\.ya?ml$' --validator ./scripts/validate-yaml-wrapper.sh --watch custom/"
\`\`\`

## Tips

- Press **Ctrl+C** to stop watch mode
- The cache persists between watch sessions for fast re-validation
- Only files matching `*.yaml` or `*.yml` are monitored
- Changes are detected instantly with minimal delay

## Workflow

1. Start watch mode in one terminal
2. Edit your YAML files in your editor
3. Save the file
4. See instant validation feedback in the watch terminal
5. Fix any errors and save again
6. Repeat until validation passes

For more details, see /app/scripts/WATCH_MODE_GUIDE.md inside the container.
WATCHGUIDE

# Create README guide at ~/README.md
RUN mkdir -p /root && cat > /root/README.md << 'EOF'
# Test Case Manager - Docker Container Guide

Welcome to the Test Case Manager Docker container! This container includes all the tools needed for managing and executing test cases.

## Available Binaries

All binaries are installed in `/usr/local/bin` and available in your PATH:

### Primary Tool
- **tcm** - Test Case Manager (main interactive tool)
  - Full name: `testcase-manager`
  - Create, edit, and manage test cases interactively
  - Database-backed condition selection with fuzzy search
  - Git integration for version control
  - Usage: `tcm --help`

### Test Execution & Verification
- **test-executor** - Automated test execution from YAML files
  - Generate bash scripts from test cases
  - Execute tests with JSON logging
  - Usage: `test-executor --help`

- **test-verify** - Test verification and reporting
  - Verify test execution logs against test cases
  - Generate JUnit XML reports for CI/CD
  - Batch verification mode
  - Usage: `test-verify --help`

- **test-orchestrator** - Test orchestration and coordination
  - Coordinate complex test workflows
  - Usage: `test-orchestrator --help`

### Validation Tools
- **validate-yaml** - YAML file validation
  - Validate YAML syntax and structure
  - Usage: `validate-yaml <file.yaml>`

- **validate-json** - JSON file validation
  - Validate JSON syntax and structure
  - Usage: `validate-json <file.json>`

### Additional Tools
- **trm** - Test Run Manager
  - Manage test execution runs
  - Usage: `trm --help`

- **editor** - Interactive test case editor
  - Edit test cases with enhanced interface
  - Usage: `editor --help`

- **watch-yaml** - Watch mode helper
  - Monitor testcases/ directory and auto-validate YAML files
  - Uses inotify for instant feedback on file changes
  - Usage: `watch-yaml`

## Quick Start

### Create a new test case
```bash
tcm create --id TC_001
```

### Build test sequences interactively
```bash
tcm build-sequences-with-steps
```

### List all test cases
```bash
tcm list
```

### Execute a test case
```bash
test-executor execute testcases/my_test.yml
```

### Verify test execution
```bash
test-verify single --log test_execution_log.json --test-case-id TC_001
```

### Watch mode for YAML files
```bash
# Watch testcases/ directory for changes and auto-validate (easiest method)
watch-yaml

# Or use make
make watch

# Or use the script directly
./scripts/watch-yaml-files.sh

# Watch a custom directory
SCHEMA_FILE=data/schema.json ./scripts/validate-files.sh \
    --pattern '\.ya?ml$' \
    --validator ./scripts/validate-yaml-wrapper.sh \
    --watch custom-dir/
```

**Docker Usage:**
```bash
# Mount your testcases directory and start watch mode
docker run -it --rm -v $(pwd)/testcases:/app/testcases testcase-manager:latest watch-yaml
```

## Data Directory

Test case data is located at: `/app/data`

## Git Integration

Git is pre-installed for version control integration. Initialize a repository:
```bash
cd /app
git init
git config user.name "Your Name"
git config user.email "your.email@example.com"
```

## Links & Documentation

- **GitHub Repository**: https://github.com/yourusername/testcase-manager
- **Test Case Schema**: `/app/data/testcase_schema.json`
- **Execution Log Schema**: `/app/data/test_execution_log_schema.json`

## Container Information

- **Base Image**: Debian Bookworm Slim
- **Rust Version**: 1.92
- **Working Directory**: `/app`
- **User**: root

## Watch Mode

This container includes a powerful watch mode for continuous YAML validation:
- See `/app/DOCKER_WATCH_GUIDE.md` for quick start
- See `/app/scripts/WATCH_MODE_GUIDE.md` for comprehensive documentation
- Uses inotify for instant file change detection
- Automatic validation with color-coded real-time feedback

## Tips

1. Use `tcm --verbose` for detailed logging
2. All commands support `--help` for usage information
3. Tab completion works for file paths and commands
4. Interactive mode automatically detects TTY and falls back to numbered selection in non-TTY environments
5. Use `watch-yaml` for instant validation feedback during development

For more detailed documentation, visit the GitHub repository or run `tcm --help`.
EOF

# Set default command
CMD ["tcm"]
