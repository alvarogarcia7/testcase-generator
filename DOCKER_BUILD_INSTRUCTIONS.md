# Docker Build Instructions

## Quick Start

To build and verify the Docker image:

```bash
# 1. Build the Docker image
docker build -t testcase-manager:latest .

# 2. Verify the build (check all binaries are present)
./scripts/verify-docker.sh

# 3. Run the container
docker run -it --rm testcase-manager:latest
```

## What Gets Built

The Docker image includes:

### All 8 Binaries (in /usr/local/bin)
1. **tcm** - Test Case Manager (main tool)
2. **test-executor** - Execute tests with JSON logging
3. **test-verify** - Verify test execution logs
4. **test-orchestrator** - Coordinate test workflows
5. **validate-yaml** - YAML validation
6. **validate-json** - JSON validation
7. **trm** - Test Run Manager
8. **editor** - Interactive editor

### Data Files
- `/app/data/` - Test schemas and templates

### Documentation
- `/root/README.md` - Complete usage guide with all binary descriptions and quick start commands

## Dockerfile Features

### Multi-Stage Build
1. **deps stage** - Builds dependencies (cached)
2. **builder stage** - Builds application
3. **runtime stage** - Minimal runtime image with only binaries

### Clean Image
- No build artifacts (*.d files, deps/, build/, etc.)
- No source code
- No documentation files (except generated README)
- Only runtime dependencies (git)

### Optimizations
- Dependency caching for faster rebuilds
- Explicit binary copying (no wildcards)
- Minimal base image (debian:bookworm-slim)
- Only necessary runtime tools

## Verification

The `verify-docker.sh` script checks:
- ✓ Image exists
- ✓ All 8 binaries present and executable
- ✓ Data directory exists  
- ✓ README guide present
- ✓ Binaries can execute (tcm --version)
- ✓ No unwanted files in /usr/local/bin

## Build Options

### Standard Build
```bash
docker build -t testcase-manager:latest .
```

### No Cache (Clean Build)
```bash
docker build --no-cache -t testcase-manager:latest .
```

### With Custom Tag
```bash
docker build -t myregistry/testcase-manager:v1.0.0 .
```

### Build and Push to Registry
```bash
docker build -t myregistry/testcase-manager:latest .
docker push myregistry/testcase-manager:latest
```

## Running the Container

### Interactive Shell
```bash
docker run -it --rm testcase-manager:latest
```

### Run Specific Command
```bash
docker run --rm testcase-manager:latest tcm --help
```

### With Volume Mount
```bash
docker run -it --rm \
  -v $(pwd)/testcases:/app/testcases \
  testcase-manager:latest
```

### With Git Configuration
```bash
docker run -it --rm \
  -e GIT_AUTHOR_NAME="Your Name" \
  -e GIT_AUTHOR_EMAIL="you@example.com" \
  -v $(pwd)/testcases:/app/testcases \
  testcase-manager:latest
```

## Troubleshooting

### Build Fails
```bash
# Clean everything and rebuild
docker system prune -a
docker build --no-cache -t testcase-manager:latest .
```

### Binary Not Found
```bash
# Check what's in the image
docker run --rm testcase-manager:latest ls -la /usr/local/bin
```

### Verification Fails
```bash
# Rebuild and verify
docker build -t testcase-manager:latest .
./scripts/verify-docker.sh
```

## Documentation

- **Complete Docker Guide**: [docs/DOCKER.md](docs/DOCKER.md)
- **In-Container README**: Run `docker run --rm testcase-manager:latest cat /root/README.md`
- **Project README**: [README.md](README.md)
- **Implementation Summary**: [DOCKER_CLEANUP_SUMMARY.md](DOCKER_CLEANUP_SUMMARY.md)

## Build Time

Approximate build times:
- **First build**: 5-10 minutes (depends on system)
- **Rebuild with cache**: 1-2 minutes
- **Rebuild without cache**: 5-10 minutes

## Size

Expected image size: ~500MB-1GB (includes Rust binaries and runtime)
- Builder images are discarded (not in final image)
- Only runtime stage is kept
- Debian bookworm-slim base (~80MB) + binaries + git

## CI/CD Integration

Example GitHub Actions workflow:

```yaml
name: Docker Build and Test

on: [push, pull_request]

jobs:
  docker:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Build Docker Image
        run: docker build -t testcase-manager:latest .
      
      - name: Verify Image
        run: ./scripts/verify-docker.sh
      
      - name: Test Binary Execution
        run: |
          docker run --rm testcase-manager:latest tcm --version
          docker run --rm testcase-manager:latest test-executor --version
```

## Next Steps

After building successfully:

1. Run the verification script
2. Test the container interactively
3. Read the in-container README
4. Try running test commands
5. Mount your test case directory and start working!
