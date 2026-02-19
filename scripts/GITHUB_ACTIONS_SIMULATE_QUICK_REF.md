# GitHub Actions Workflow Simulation - Quick Reference

## Purpose
Simulates GitHub Actions workflow locally for testing before pushing to GitHub.

## Quick Start

```bash
# Basic simulation
make github-actions-simulate

# Or run directly
./scripts/simulate-github-actions-workflow.sh

# With options
./scripts/simulate-github-actions-workflow.sh --clean --verbose
```

## Options

- `--clean` - Remove previous artifacts before running
- `--keep-temp` - Keep temporary files for debugging
- `--verbose` - Enable detailed output
- `--help` - Show help message

## What It Does

1. **Setup** - Pulls python:3.11 Docker image
2. **Install** - Runs `pip install -r requirements.txt`
3. **Build** - Runs `mkdocs build`
4. **Verify** - Checks site/ directory structure
5. **Copy** - Copies artifacts to project directory
6. **Build PDF** - Runs `ENABLE_PDF_EXPORT=1 mkdocs build`
7. **Update** - Updates site/ with PDF version

## Output

- **site/** - Generated documentation website
- **site/pdf/** - PDF documentation (if successful)

## View Results

```bash
# Option 1: Local web server
python3 -m http.server 8000 --directory site
# Then open http://localhost:8000

# Option 2: Direct file opening
open site/index.html
```

## Prerequisites

- Docker installed and running
- Bash 3.2+
- Internet access (for Docker image pulls)
- Sufficient disk space

## Workflow Configuration

- **Concurrency**: pages group
- **Cancel in progress**: false
- **Triggers**: push to main, workflow_dispatch

## Common Options

```bash
# Clean and rebuild
./scripts/simulate-github-actions-workflow.sh --clean

# Debug with preserved files
./scripts/simulate-github-actions-workflow.sh --keep-temp

# Verbose output for troubleshooting
./scripts/simulate-github-actions-workflow.sh --verbose

# Combine options
./scripts/simulate-github-actions-workflow.sh --clean --verbose --keep-temp
```

## Expected Output

```
=== GitHub Actions Workflow Simulation ===
✓ Docker is installed
✓ Dependencies installed successfully (15s)
✓ Documentation built successfully (8s)
✓ PDF documentation built successfully (12s)
✓ GitHub Actions workflow simulated successfully!
```

## Timing

Typical execution times:
- Dependencies install: 10-20s
- Documentation build: 5-10s
- PDF build: 10-15s
- Total: 25-45s

## Files Used

- `.github/workflows/docs.yml` - Workflow configuration
- `requirements.txt` - Python dependencies
- `mkdocs.yml` - MkDocs configuration
- `docs/` - Documentation source files

## Cleanup

Automatic cleanup is performed unless `--keep-temp` is used.

Manual cleanup:
```bash
rm -rf site/
```

## Common Issues

### Docker not running
```bash
# macOS: Start Docker Desktop
# Linux: sudo systemctl start docker
```

### PDF not generated
This is expected without system dependencies. The script will show:
```
⚠ PDF documentation skipped (requires system dependencies)
```

### Build failures
```bash
# Test dependencies
pip install -r requirements.txt
mkdocs build
```

## Related Commands

```bash
# Run validation test
make github-actions-test

# View detailed docs
cat scripts/README_GITHUB_ACTIONS_SIMULATE.md
```

## Documentation

See `scripts/README_GITHUB_ACTIONS_SIMULATE.md` for detailed documentation.
