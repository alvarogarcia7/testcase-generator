# Docker Guide

This guide explains how to build and use the Test Case Manager Docker image.

## Overview

The Docker image provides a complete environment for test case management with all necessary tools and dependencies pre-installed.

### What's Included

- **All binaries**: tcm, test-executor, test-verify, validate-yaml, validate-json, trm, editor, test-orchestrator
- **Git**: For version control integration
- **Data directory**: Test schemas and templates at `/app/data`
- **README**: Quick reference guide at `/root/README.md`

### Image Characteristics

- **Base**: Debian Bookworm Slim (lightweight)
- **Size**: Optimized with multi-stage build
- **Rust**: Built with Rust 1.92
- **Working directory**: `/app`

## Building the Image

### Quick Build

```bash
./scripts/build-docker.sh
```

### Manual Build

```bash
docker build -t testcase-manager:latest .
```

The build process uses multi-stage building for optimal image size:
1. **Stage 1 (deps)**: Builds dependencies only (cached layer)
2. **Stage 2 (builder)**: Builds the full application
3. **Stage 3 (runtime)**: Minimal runtime image with only binaries and data

## Verifying the Build

After building, verify everything is correct:

```bash
./scripts/verify-docker.sh
```

This checks:
- All 8 binaries are present
- Data directory exists
- README guide is created
- Binaries execute correctly
- No extra unwanted files

## Running the Container

### Interactive Mode

Start an interactive shell with tcm:

```bash
docker run -it --rm testcase-manager:latest
```

### Run Specific Commands

Execute a specific binary:

```bash
# View help
docker run --rm testcase-manager:latest tcm --help

# List test cases
docker run --rm testcase-manager:latest tcm list

# View README
docker run --rm testcase-manager:latest cat /root/README.md
```

### With Volume Mounting

Mount a local directory for persistent test cases:

```bash
docker run -it --rm \
  -v $(pwd)/testcases:/app/testcases \
  testcase-manager:latest
```

### With Git Configuration

Pass git configuration for commits:

```bash
docker run -it --rm \
  -e GIT_AUTHOR_NAME="Your Name" \
  -e GIT_AUTHOR_EMAIL="you@example.com" \
  -v $(pwd)/testcases:/app/testcases \
  testcase-manager:latest
```

## Available Binaries

All binaries are in `/usr/local/bin` and available in PATH:

| Binary | Description |
|--------|-------------|
| `tcm` | Main Test Case Manager (alias for testcase-manager) |
| `test-executor` | Execute tests with JSON logging |
| `test-verify` | Verify test execution logs and generate reports |
| `test-orchestrator` | Coordinate complex test workflows |
| `validate-yaml` | Validate YAML syntax and structure |
| `validate-json` | Validate JSON syntax and structure |
| `trm` | Test Run Manager |
| `editor` | Interactive test case editor |

## Usage Examples

### Create a Test Case

```bash
docker run -it --rm \
  -v $(pwd)/testcases:/app/testcases \
  testcase-manager:latest tcm create --id TC_001
```

### Execute Tests

```bash
docker run --rm \
  -v $(pwd)/testcases:/app/testcases \
  testcase-manager:latest test-executor execute testcases/TC_001.yml
```

### Verify Test Results

```bash
docker run --rm \
  -v $(pwd)/testcases:/app/testcases \
  -v $(pwd)/logs:/app/logs \
  testcase-manager:latest test-verify single \
    --log logs/TC_001_execution_log.json \
    --test-case-id TC_001
```

## Dockerfile Structure

The Dockerfile is optimized for:

1. **Clean image**: Only binaries are copied, no build artifacts
2. **Layer caching**: Dependencies are built in a separate stage
3. **Security**: No extra files or garbage in the root directory
4. **Documentation**: README guide is generated during build

### Key Changes from Previous Version

- **Fixed binary copying**: Now explicitly copies each binary by name instead of using wildcard
- **Renamed binary**: `testcase-manager` is copied as `tcm` for convenience
- **Added README**: Generated at `/root/README.md` with all links and usage info
- **Clean root**: No extra build artifacts or temporary files

## Troubleshooting

### Binary Not Found

If a binary is missing, rebuild the image:
```bash
./scripts/build-docker.sh
./scripts/verify-docker.sh
```

### Permission Issues

The container runs as root by default. If you need different permissions:
```bash
docker run -it --rm -u $(id -u):$(id -g) testcase-manager:latest
```

### Volume Mount Issues

Ensure the mounted directory exists:
```bash
mkdir -p testcases
docker run -it --rm -v $(pwd)/testcases:/app/testcases testcase-manager:latest
```

## Development

### Rebuilding After Code Changes

```bash
# Clean build with no cache
docker build --no-cache -t testcase-manager:latest .

# Or use the script
./scripts/build-docker.sh
```

### Inspecting the Image

```bash
# List all files in /usr/local/bin
docker run --rm testcase-manager:latest ls -la /usr/local/bin

# Check image size
docker images testcase-manager:latest

# Inspect layers
docker history testcase-manager:latest
```

## CI/CD Integration

### Building in CI

```yaml
# Example GitHub Actions
- name: Build Docker Image
  run: docker build -t testcase-manager:latest .

- name: Verify Image
  run: ./scripts/verify-docker.sh

- name: Push to Registry
  run: |
    docker tag testcase-manager:latest myregistry/testcase-manager:latest
    docker push myregistry/testcase-manager:latest
```

### Running Tests in CI

```yaml
- name: Run Tests
  run: |
    docker run --rm \
      -v $PWD/testcases:/app/testcases \
      testcase-manager:latest test-executor execute testcases/
```

## Best Practices

1. **Use volume mounts** for persistent data
2. **Set git config** when using git features
3. **Run verification** after every build
4. **Tag versions** for production use
5. **Use --rm flag** to auto-remove containers
6. **Mount only needed directories** for security

## Additional Resources

- Main README: `/root/README.md` (in container)
- Project README: `README.md` (in repository)
- Test schemas: `/app/data/*.json` (in container)
- Build script: `scripts/build-docker.sh`
- Verify script: `scripts/verify-docker.sh`
