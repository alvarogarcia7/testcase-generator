# Docker .dockerignore Optimization Test

## Overview

This document describes the end-to-end integration test for Docker `.dockerignore` optimization in the MkDocs Docker build setup.

## Test Script

- **Location**: `tests/integration/test_docker_dockerignore_e2e.sh`
- **Purpose**: Validate that `.dockerignore.mkdocs` properly excludes unnecessary files and optimizes Docker build performance

## Test Coverage

### 1. Prerequisites Check
- Verifies Docker is installed and running
- Confirms `Dockerfile.mkdocs` and `.dockerignore.mkdocs` exist

### 2. .dockerignore Content Verification
- Validates that `.dockerignore.mkdocs` excludes required directories:
  - `target/` (Rust build artifacts)
  - `src/` (source code)
  - `tests/` (test files)
  - `testcases/` (test cases)
- Checks for additional recommended exclusions:
  - `.git/` (version control)
  - `*.profraw` (coverage files)
  - `examples/*.sh` (example scripts)
  - `backlog/` (backlog files)
  - `scripts/` (build scripts)

### 3. Build Performance Testing
- **First Build**: Measures time for initial build with `.dockerignore` (should be < 5 minutes)
- **Incremental Build**: Tests layer caching with minor changes (should be < 1 minute)
- Validates build completes successfully
- Records build times and image size

### 4. Docker Build Context Size
- Extracts and analyzes build context size from Docker output
- Verifies context is reasonably sized (MB or kB, not GB)
- Confirms large directories are excluded from context

### 5. Image Content Verification
- Validates excluded files are NOT present in the image:
  - `src/` directory
  - `tests/` directory
  - `testcases/` directory
  - `target/` directory
  - `.git/` directory
- Confirms required files ARE present:
  - `docs/` directory
  - `mkdocs.yml` file

### 6. Layer Caching Test
- Creates temporary documentation change
- Rebuilds image to test layer caching
- Measures rebuild time
- Counts number of cached layers
- Validates caching improves build performance

### 7. Comparison Without .dockerignore
- Builds image without `.dockerignore` for comparison
- Compares build context sizes
- Compares image sizes
- Calculates context size reduction percentage

### 8. Actual Docker Build Command
- Runs the exact command specified in requirements: `docker build -f Dockerfile.mkdocs -t test-mkdocs .`
- Monitors build output
- Verifies image creation
- Displays build statistics

### 9. Cleanup and Restoration
- Restores original `.dockerignore` configuration
- Cleans up test images
- Removes temporary files

## Usage

### Basic Test Run
```bash
./tests/integration/test_docker_dockerignore_e2e.sh
```

### Verbose Mode
```bash
./tests/integration/test_docker_dockerignore_e2e.sh --verbose
```

### Keep Temporary Files
```bash
./tests/integration/test_docker_dockerignore_e2e.sh --no-remove
```

## Performance Expectations

### First Build
- **Maximum Time**: 5 minutes (300 seconds)
- **Expected Time**: 2-4 minutes
- **Context Size**: < 50 MB (with proper .dockerignore)

### Incremental Build
- **Maximum Time**: 1 minute (60 seconds)
- **Expected Time**: 10-30 seconds
- **Cached Layers**: > 5 layers

## Exit Codes

- `0`: All tests passed
- `1`: One or more tests failed

## Test Output

The test provides detailed output including:
- Individual test results (✓/✗)
- Build times for each build
- Docker build context sizes
- Image sizes
- Number of cached layers
- Context size reduction percentage
- Final summary statistics

## Prerequisites

- Docker installed and running
- Sufficient disk space for Docker images (> 1 GB recommended)
- Network access for pulling base images
- Write permissions in project directory

## Dependencies

- `scripts/lib/logger.sh`: Centralized logging library
- `Dockerfile.mkdocs`: MkDocs Docker build file
- `.dockerignore.mkdocs`: Docker ignore file for MkDocs builds

## Related Files

- `Dockerfile.mkdocs`: The Dockerfile being tested
- `.dockerignore.mkdocs`: The dockerignore file being validated
- `tests/integration/test_docker_mkdocs_e2e.sh`: General Docker MkDocs tests
- `tests/integration/test_docker_html_build_e2e.sh`: HTML build tests

## Troubleshooting

### Build Timeout
If builds exceed time limits:
1. Check Docker daemon resources (CPU, memory)
2. Review network connection for base image pulls
3. Clear Docker build cache: `docker builder prune -f`

### Context Size Too Large
If context size is in GB range:
1. Verify `.dockerignore.mkdocs` is being used
2. Check for large files in project root
3. Add additional patterns to `.dockerignore.mkdocs`

### Layer Caching Not Working
If incremental builds are slow:
1. Ensure Dockerfile layers are properly ordered
2. Verify `.dockerignore` hasn't changed between builds
3. Check Docker build cache status

## Integration with CI/CD

This test can be integrated into CI/CD pipelines:

```yaml
test-docker-dockerignore:
  script:
    - ./tests/integration/test_docker_dockerignore_e2e.sh
  timeout: 10 minutes
```

## Maintenance

Update this test when:
- Adding new large directories to the project
- Modifying `.dockerignore.mkdocs` patterns
- Changing Docker build process
- Adjusting performance requirements
