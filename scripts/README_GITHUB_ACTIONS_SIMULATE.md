# GitHub Actions Workflow Simulation Script

## Overview

This script simulates the GitHub Actions documentation workflow locally, allowing you to test the workflow configuration before pushing changes to GitHub.

## Script Location

**Path:** `scripts/simulate-github-actions-workflow.sh`

## Purpose

The simulation script allows you to:
1. Test the GitHub Actions workflow locally without committing to GitHub
2. Validate workflow configuration changes before deployment
3. Debug workflow issues in a controlled environment
4. Verify artifact generation and structure
5. Test both standard and PDF documentation builds

## Features

- **Local Execution**: Runs the workflow on your local machine using Docker
- **Full Workflow Simulation**: Simulates build and build-pdf jobs
- **Artifact Generation**: Generates the same artifacts as GitHub Actions would
- **Detailed Logging**: Provides step-by-step output with timing information
- **Cleanup Management**: Automatically cleans up temporary files
- **Debug Mode**: Option to preserve temporary files for debugging

## Usage

### Basic Usage

```bash
# Run simulation with default settings
./scripts/simulate-github-actions-workflow.sh

# Or use the Makefile target
make github-actions-simulate
```

### Advanced Options

```bash
# Clean previous artifacts before running
./scripts/simulate-github-actions-workflow.sh --clean

# Keep temporary files for debugging
./scripts/simulate-github-actions-workflow.sh --keep-temp

# Enable verbose output
./scripts/simulate-github-actions-workflow.sh --verbose

# Combine options
./scripts/simulate-github-actions-workflow.sh --clean --verbose

# Show help message
./scripts/simulate-github-actions-workflow.sh --help
```

## What It Simulates

### Build Job (Standard Documentation)
1. **Set up Python 3.11**: Pulls python:3.11 Docker image
2. **Install dependencies**: Runs `pip install -r requirements.txt`
3. **Build documentation**: Runs `mkdocs build`
4. **Verify artifacts**: Checks site/ directory structure
5. **Copy artifacts**: Copies site/ to project directory

### Build-PDF Job (PDF Documentation)
1. **Clean previous build**: Removes site/ directory
2. **Set environment**: Sets ENABLE_PDF_EXPORT=1
3. **Build with PDF**: Runs `mkdocs build` with PDF export
4. **Verify PDF artifacts**: Checks site/pdf/ directory
5. **Update artifacts**: Updates site/ with PDF version

## Prerequisites

- **Docker**: Must be installed and running
- **Bash 3.2+**: Compatible with macOS and Linux
- **Network Access**: Required to pull Docker images
- **Disk Space**: Sufficient space for Docker images and build artifacts

## Output

### Simulation Steps

The script outputs detailed information about each step:

```
=== GitHub Actions Workflow Simulation ===
[INFO] Project root: /path/to/project
[INFO] Docker image: python:3.11
[INFO] Output directory: /path/to/project/site

=== Step 1: Checking Prerequisites ===
✓ Docker is installed
✓ Docker daemon is running
✓ All required files found
✓ docs/ directory found

=== Step 2: Setting Up Python Environment ===
✓ Docker image pulled successfully

=== Step 3: Preparing Working Directory ===
✓ Working directory prepared: /tmp/tmp.XXXXXX

=== Step 4: Installing Dependencies ===
✓ Dependencies installed successfully (15s)

=== Step 5: Building Documentation ===
✓ Documentation built successfully (8s)

=== Step 6: Verifying Artifacts ===
✓ site/ directory created
✓ index.html found
[INFO] Total files generated: 127
[INFO] Total size: 2.1M

=== Step 7: Copying Artifacts ===
✓ Artifacts copied to project directory

=== Step 8: Building PDF Documentation ===
✓ PDF documentation built successfully (12s)
✓ PDF documentation directory found
✓ PDF files generated: 1
✓ Site updated with PDF documentation

=== Simulation Summary ===
✓ GitHub Actions workflow simulated successfully!

[INFO] Workflow steps completed:
[INFO]   ✓ Python 3.11 environment set up
[INFO]   ✓ Dependencies installed (15s)
[INFO]   ✓ Documentation built (8s)
[INFO]   ✓ Artifacts verified and copied
[INFO]   ✓ PDF documentation built (12s)

[INFO] Output directory: /path/to/project/site
[INFO] You can view the generated site by running:
[INFO]   python3 -m http.server 8000 --directory /path/to/project/site

=== Workflow Configuration ===
[INFO] Concurrency group: pages
[INFO] Cancel in progress: false (prevents conflicting deployments)
[INFO] Triggers: push to main, workflow_dispatch
```

## Generated Artifacts

After running the simulation, you'll find:

- **site/**: Complete documentation website (same as GitHub Actions artifact)
- **site/index.html**: Main documentation page
- **site/pdf/**: PDF documentation directory (if PDF build succeeded)
- **site/assets/**: CSS, JavaScript, and other assets

## Viewing Generated Documentation

### Option 1: Local Web Server

```bash
# Start a local web server
python3 -m http.server 8000 --directory site

# Open in browser
open http://localhost:8000
```

### Option 2: Direct File Opening

```bash
# macOS
open site/index.html

# Linux
xdg-open site/index.html

# Windows
start site/index.html
```

## Differences from GitHub Actions

While the simulation closely mirrors GitHub Actions behavior, there are some differences:

### Similarities
- ✓ Same Docker base image (python:3.11)
- ✓ Same dependency installation method
- ✓ Same build commands
- ✓ Same artifact structure
- ✓ Same environment variables

### Differences
- ✗ System dependencies for PDF may not be installed
- ✗ GitHub Actions-specific environment variables not available
- ✗ GitHub Pages deployment not simulated
- ✗ Artifact upload/download not performed

## Workflow Configuration Displayed

The script shows relevant workflow configuration:

- **Concurrency Group**: `pages` - prevents multiple deployments
- **Cancel in Progress**: `false` - allows deployments to complete
- **Triggers**: `push to main`, `workflow_dispatch` - when workflow runs

## Troubleshooting

### Docker Issues

**Problem**: Docker daemon not running

```bash
# Start Docker
# macOS: Start Docker Desktop
# Linux: sudo systemctl start docker
```

**Problem**: Permission denied

```bash
# Add user to docker group (Linux)
sudo usermod -aG docker $USER
# Then log out and back in
```

### Build Issues

**Problem**: Dependencies fail to install

```bash
# Check requirements.txt is valid
pip install -r requirements.txt

# Update pip
pip install --upgrade pip
```

**Problem**: MkDocs build fails

```bash
# Validate mkdocs.yml
mkdocs build --strict

# Check for missing files
ls -la docs/
```

### PDF Generation Issues

**Problem**: PDF not generated

This is expected if system dependencies aren't installed. The script will show:
```
⚠ PDF documentation skipped (requires system dependencies)
```

To install system dependencies (Ubuntu/Debian):
```bash
sudo apt-get update
sudo apt-get install -y libcairo2 libpango-1.0-0 libpangocairo-1.0-0 \
    libgdk-pixbuf2.0-0 libffi-dev shared-mime-info
```

## Cleanup

### Automatic Cleanup

By default, temporary files are automatically cleaned up when the script exits.

### Manual Cleanup

If you used `--keep-temp`, clean up manually:

```bash
# Remove generated artifacts
rm -rf site/

# Remove any remaining temp directories
# (shown in script output when using --keep-temp)
rm -rf /tmp/tmp.XXXXXX
```

## Integration with Development Workflow

### Recommended Workflow

1. **Make changes** to workflow, documentation, or configuration
2. **Run simulation** to validate changes locally
3. **Review artifacts** to ensure correct generation
4. **Commit changes** if simulation succeeds
5. **Push to GitHub** to trigger actual workflow

### Example Development Session

```bash
# Make changes to workflow
vim .github/workflows/docs.yml

# Simulate workflow
./scripts/simulate-github-actions-workflow.sh --clean --verbose

# Review artifacts
open site/index.html

# If successful, commit and push
git add .github/workflows/docs.yml
git commit -m "Update GitHub Actions workflow"
git push origin main
```

## Related Files

- **Workflow**: `.github/workflows/docs.yml`
- **Test Script**: `tests/integration/test_github_actions_docker_workflow_e2e.sh`
- **Requirements**: `requirements.txt`
- **MkDocs Config**: `mkdocs.yml`
- **Logger Library**: `scripts/lib/logger.sh`

## Related Documentation

- [GitHub Actions Test Documentation](README_GITHUB_ACTIONS_TEST.md)
- [GitLab CI Simulation](README_GITLAB_CI_SIMULATE.md)
- [MkDocs Test Documentation](README_MKDOCS_TEST.md)

## Environment Variables

The script sets the following environment variables during simulation:

- `ENABLE_PDF_EXPORT=1`: Enables PDF generation in build-pdf job

## Notes

- The simulation uses the same Docker image as GitHub Actions (python:3.11)
- Temporary files are stored in system temp directory
- The script is compatible with both macOS and Linux
- All timing information is logged for performance analysis
- The script follows the same logger library conventions as other project scripts
