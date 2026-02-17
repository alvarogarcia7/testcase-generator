# Docker Watch Mode Setup

This document describes the watch mode implementation in the Docker container.

## Overview

The Docker image now includes full watch mode support using inotify-tools to monitor YAML files and automatically validate them when changes are detected.

## What Was Added

### 1. Runtime Dependencies
- **inotify-tools**: Linux file system monitoring tool for instant change detection
- **make**: Build automation tool to support `make watch` command

### 2. Scripts and Files
- **scripts/**: Complete scripts directory copied into the container
- **Makefile**: Build and workflow automation file
- **watch-yaml helper**: `/usr/local/bin/watch-yaml` convenience script

### 3. Documentation
- **DOCKER_WATCH_GUIDE.md**: Quick reference for Docker watch mode
- **scripts/WATCH_MODE_GUIDE.md**: Updated with Docker-specific information
- **README.md**: Container documentation includes watch mode section

## Usage

### Quick Start
```bash
# Build the Docker image
docker build -t testcase-manager:latest .

# Run with watch mode (mount your testcases directory)
docker run -it --rm -v $(pwd)/testcases:/app/testcases testcase-manager:latest watch-yaml
```

### Alternative Commands
```bash
# Using make
docker run -it --rm -v $(pwd)/testcases:/app/testcases testcase-manager:latest make watch

# Using the script directly
docker run -it --rm -v $(pwd)/testcases:/app/testcases testcase-manager:latest ./scripts/watch-yaml-files.sh

# Watch a custom directory
docker run -it --rm -v $(pwd)/custom:/app/custom testcase-manager:latest bash -c \
    "SCHEMA_FILE=schemas/schema.json ./scripts/validate-files.sh --pattern '\.ya?ml$' --validator ./scripts/validate-yaml-wrapper.sh --watch custom/"
```

## How It Works

1. **inotify** monitors the specified directory recursively for file changes
2. When a YAML file is modified, created, or moved:
   - A small delay (0.1s) ensures the file is fully written
   - The validate-yaml binary validates the file against the schema
   - Results are displayed with color-coded output (green ✓ or red ✗)
3. Validation cache persists between runs for fast re-validation
4. Press Ctrl+C to stop watch mode

## Key Features

- **Instant Feedback**: Changes detected and validated immediately
- **Smart Caching**: Two-layer cache (mtime + hash) avoids redundant validations
- **Real-time Display**: Color-coded output shows validation status instantly
- **Persistent Cache**: Validation results persist between watch sessions
- **Pattern Matching**: Only files matching `*.yaml` or `*.yml` are monitored

## Testing

Run the verification script after building:
```bash
./scripts/verify-docker.sh
```

This checks:
- All binaries are present (including watch-yaml)
- inotify-tools is installed
- make is installed
- Scripts directory exists and is executable
- Makefile is present
- Documentation files exist

## File Structure in Container

```
/app/
├── data/                    # Schema and data files
├── scripts/                 # Validation and watch scripts
│   ├── validate-files.sh   # Core validation framework
│   ├── watch-yaml-files.sh # YAML watch wrapper
│   └── WATCH_MODE_GUIDE.md # Comprehensive guide
├── Makefile                 # Build automation
├── DOCKER_WATCH_GUIDE.md    # Docker-specific guide
└── testcases/               # Mounted from host

/usr/local/bin/
├── tcm                      # Main tool
├── watch-yaml               # Watch mode helper
└── ... (other binaries)
```

## Dockerfile Changes Summary

1. Added inotify-tools and make to runtime dependencies
2. Copy scripts directory and make all scripts executable
3. Copy Makefile for convenient commands
4. Create watch-yaml helper script in /usr/local/bin
5. Create DOCKER_WATCH_GUIDE.md quick reference
6. Update main README with watch mode documentation

## Benefits

- **Development Efficiency**: Instant validation feedback during development
- **CI/CD Ready**: Same validation scripts work in both dev and CI
- **No Setup Required**: All dependencies pre-installed in the container
- **Cross-Platform**: Works consistently in Docker regardless of host OS
- **Minimal Overhead**: Efficient inotify-based monitoring with smart caching
