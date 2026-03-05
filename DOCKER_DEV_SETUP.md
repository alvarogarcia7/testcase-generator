# Docker Development Environment Setup

## Overview

The `Dockerfile.dev` creates a development-focused Docker image that extends the base `testcase-manager:latest` image with additional development tools, debugging utilities, and helpful configurations.

## Quick Start

```bash
# 1. Build the development image (this will also build the base image if needed)
./scripts/build-dev-docker.sh

# 2. Run the development container
docker run -it --rm testcase-manager:dev

# 3. Run with volume mount for local development
docker run -it --rm -v $(pwd):/app testcase-manager:dev
```

## What's Included

### Development Tools

- **Editors**: vim, nano
- **Network Tools**: curl, wget, netcat, tcpdump, dnsutils, telnet
- **Process & System Tools**: htop, procps, strace, lsof
- **Debugging Tools**: gdb, valgrind
- **Shell Utilities**: tmux, screen, bash-completion
- **File Utilities**: tree, less, jq
- **Development Utilities**: man-db

### Watch Mode Prerequisites

- **inotify-tools**: For file system monitoring
- **make**: For running Makefile commands
- All watch mode scripts from the base image

### Development Directories

- `/app/dev-workspace` - Development workspace for temporary files and experiments
- `/app/logs` - Log files directory
- `/app/tmp` - Temporary files directory

### Configuration Files

#### `.bashrc` (Development Shell Configuration)

Custom bash configuration with:
- Colored prompt and terminal support
- Extended history (10,000 commands)
- Bash completion enabled
- Helpful aliases and functions

#### `.vimrc` (Vim Configuration)

Basic vim setup with:
- Syntax highlighting
- Line numbers
- Smart indentation (2 spaces for YAML, 4 for others)
- Search highlighting
- Mouse support
- Trailing whitespace indicators

#### `.dev-config` (Development Environment Variables)

Pre-configured environment variables:
- `VERBOSE=1` - Enable verbose logging
- `SCHEMA_FILE=/app/schemas/schema.json` - Default schema path
- `DEBUG=1` - Enable debug mode
- `EDITOR=vim` - Default editor
- `WATCH_DIR=/app/testcases` - Watch directory
- `TEST_OUTPUT_DIR=/app/logs` - Test output location

### Development Scripts

#### `dev-setup`

Initializes the development environment:
- Verifies all binaries are accessible
- Checks required directories
- Validates watch mode prerequisites
- Sources development configuration

```bash
# Run setup
dev-setup
```

#### `quick-test`

Runs quick validation tests:
- Checks binary availability
- Validates schema file exists
- Checks testcases directory
- Verifies watch mode script

```bash
# Run quick tests
quick-test
```

#### `dev-status`

Shows current environment status:
- System information
- Disk usage
- Memory info
- Rust toolchain version
- Git status (if in a git repo)
- Running processes

```bash
# Show status
dev-status
```

### Shell Aliases

#### Basic Commands
- `ll` - List all files in long format with details
- `la` - List all files including hidden
- `l` - List files in column format
- `grep`, `fgrep`, `egrep` - Colored grep variants

#### Project Commands
- `tcm` - Shortcut for testcase-manager
- `validate` - Quick validation with default schema
- `watch-yaml` - Start watch mode for YAML files
- `run-tests` - Run the full test suite
- `build-project` - Build all binaries (debug mode)
- `build-release` - Build all binaries (release mode)
- `lint-project` - Run linter
- `clean-project` - Clean build artifacts

### Shell Functions

#### `validate-file <yaml-file>`

Validate a single YAML test case file:
```bash
validate-file testcases/my-test.yml
```

#### `generate-script <test-case-yaml>`

Generate bash script from a test case:
```bash
generate-script testcases/my-test.yml
```

#### `run-orchestrator <test-case-yaml>`

Run test orchestrator on a test case:
```bash
run-orchestrator testcases/my-test.yml
```

#### `show-binaries`

List all available binaries with descriptions:
```bash
show-binaries
```

#### `show-help`

Display development environment help:
```bash
show-help
```

## Usage Examples

### Interactive Development Session

```bash
# Start development container with volume mount
docker run -it --rm -v $(pwd):/app testcase-manager:dev

# Inside the container:
show-help              # See available commands
show-binaries          # List all tools
validate-file testcases/example.yml
generate-script testcases/example.yml
watch-yaml             # Start watch mode
```

### Running Tests in Container

```bash
# Run tests
docker run --rm -v $(pwd):/app testcase-manager:dev run-tests

# Run with specific test
docker run --rm -v $(pwd):/app testcase-manager:dev \
  cargo test test_name
```

### Building in Container

```bash
# Build project
docker run --rm -v $(pwd):/app testcase-manager:dev build-project

# Build release binaries
docker run --rm -v $(pwd):/app testcase-manager:dev build-release
```

### Debugging

```bash
# Run with debugging tools
docker run -it --rm -v $(pwd):/app testcase-manager:dev

# Inside container:
# Use gdb for debugging
gdb --args /usr/local/bin/tcm --help

# Trace system calls
strace /usr/local/bin/tcm --version

# Monitor processes
htop
```

### Watch Mode Development

```bash
# Start container with watch mode
docker run -it --rm -v $(pwd):/app testcase-manager:dev watch-yaml

# Or with verbose output
docker run -it --rm -v $(pwd):/app testcase-manager:dev \
  bash -c "cd /app && make watch-verbose"
```

## Environment Variables

The development container sets these environment variables:

- `RUST_BACKTRACE=1` - Enable Rust backtraces
- `RUST_LOG=debug` - Set Rust logging to debug level
- `CARGO_TERM_COLOR=always` - Always use colored cargo output

Additional variables from `.dev-config`:
- `VERBOSE=1` - Enable verbose logging
- `DEBUG=1` - Enable debug mode
- `EDITOR=vim` - Default editor
- `SCHEMA_FILE=/app/schemas/schema.json` - Default schema path

## Building the Development Image

### Automatic Build (Recommended)

```bash
# Build both base and dev images with verification
./scripts/build-dev-docker.sh
```

This script will:
1. Check for base image, build if missing
2. Build development image
3. Verify all development tools are installed
4. Verify development scripts exist
5. Verify development directories exist
6. Verify configuration files exist
7. Run quick-test to validate setup
8. Verify base binaries still work

### Manual Build

```bash
# Build base image first (if not already built)
docker build -t testcase-manager:latest .

# Build development image
docker build -f Dockerfile.dev -t testcase-manager:dev .
```

### Build with No Cache

```bash
# Clean build
docker build --no-cache -f Dockerfile.dev -t testcase-manager:dev .
```

## Verification

After building, verify the setup:

```bash
# Run build script (includes verification)
./scripts/build-dev-docker.sh

# Or run quick test manually
docker run --rm testcase-manager:dev quick-test

# Or run dev-setup
docker run --rm testcase-manager:dev dev-setup
```

## Image Information

### Size
- **Base Image**: ~500MB-1GB
- **Dev Image**: ~1.2-1.5GB (includes all development tools)

### Build Time
- **First Build**: 2-5 minutes (if base image exists)
- **Rebuild with Cache**: 30-60 seconds
- **Rebuild without Cache**: 2-5 minutes

### Base Image
- **FROM**: testcase-manager:latest
- **Base OS**: Debian Bookworm (from base image)
- **Shell**: bash with custom configuration

## Development Workflow

### Recommended Workflow

1. **Start Development Container**
   ```bash
   docker run -it --rm -v $(pwd):/app testcase-manager:dev
   ```

2. **Initialize Environment**
   ```bash
   # Run automatically on container start, or manually:
   dev-setup
   ```

3. **Check Status**
   ```bash
   dev-status
   show-help
   ```

4. **Develop and Test**
   ```bash
   # Edit files on host, they'll be reflected in container
   validate-file testcases/new-test.yml
   generate-script testcases/new-test.yml
   ```

5. **Watch Mode for Continuous Validation**
   ```bash
   watch-yaml
   ```

6. **Run Tests**
   ```bash
   run-tests
   ```

### Debugging Workflow

1. **Start with Debug Environment**
   ```bash
   docker run -it --rm -v $(pwd):/app \
     -e RUST_BACKTRACE=full \
     -e RUST_LOG=trace \
     testcase-manager:dev
   ```

2. **Use Debugging Tools**
   ```bash
   # Trace system calls
   strace -o trace.log /usr/local/bin/tcm --version
   
   # Use gdb
   gdb --args /usr/local/bin/tcm --help
   
   # Monitor with htop
   htop
   ```

3. **Check Logs**
   ```bash
   # View logs
   tail -f /app/logs/*.log
   
   # Parse with jq
   cat /app/logs/test.json | jq '.'
   ```

## Troubleshooting

### Base Image Not Found

```bash
# Build base image first
docker build -t testcase-manager:latest .

# Then build dev image
docker build -f Dockerfile.dev -t testcase-manager:dev .
```

### Tools Not Found

```bash
# Rebuild without cache
docker build --no-cache -f Dockerfile.dev -t testcase-manager:dev .
```

### Permission Issues

```bash
# Run with user mapping
docker run -it --rm \
  -v $(pwd):/app \
  -u $(id -u):$(id -g) \
  testcase-manager:dev
```

### Volume Mount Issues

```bash
# Ensure absolute path
docker run -it --rm \
  -v "$(pwd)":/app \
  testcase-manager:dev
```

## CI/CD Integration

Example for GitHub Actions:

```yaml
name: Test in Development Container

on: [push, pull_request]

jobs:
  dev-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Build Development Image
        run: ./scripts/build-dev-docker.sh
      
      - name: Run Tests in Container
        run: |
          docker run --rm -v $(pwd):/app testcase-manager:dev run-tests
      
      - name: Verify Development Setup
        run: |
          docker run --rm testcase-manager:dev dev-setup
          docker run --rm testcase-manager:dev quick-test
```

## Tips and Best Practices

### 1. Use Volume Mounts for Development
Always mount your local directory to persist changes:
```bash
docker run -it --rm -v $(pwd):/app testcase-manager:dev
```

### 2. Leverage Shell Aliases
Use the pre-defined aliases for faster development:
```bash
ll                    # List files
validate-file test.yml # Quick validation
watch-yaml            # Start watch mode
```

### 3. Keep Container Running
For active development, keep one container running instead of starting/stopping:
```bash
# Terminal 1: Keep container running
docker run -it --rm -v $(pwd):/app testcase-manager:dev

# Terminal 2: Execute commands in running container
docker exec -it <container-id> bash
```

### 4. Use tmux for Multiple Sessions
Inside the container, use tmux for multiple terminal sessions:
```bash
# Start tmux
tmux

# Create new window: Ctrl-b c
# Switch windows: Ctrl-b n
# Split pane: Ctrl-b %
```

### 5. Check Status Regularly
Use dev-status to monitor the environment:
```bash
dev-status
```

## Related Documentation

- **Base Image**: [DOCKER_BUILD_INSTRUCTIONS.md](DOCKER_BUILD_INSTRUCTIONS.md)
- **Complete Docker Guide**: [docs/DOCKER.md](docs/DOCKER.md)
- **Watch Mode**: [DOCKER_WATCH_SETUP.md](DOCKER_WATCH_SETUP.md)
- **Development Guide**: [AGENTS.md](AGENTS.md)
- **Project README**: [README.md](README.md)

## Summary

The development Docker image provides a complete, pre-configured environment for developing and testing the testcase-manager project with:

✓ All development tools installed
✓ Pre-configured editors and shell
✓ Helpful aliases and functions
✓ Watch mode support
✓ Debugging utilities
✓ Development scripts
✓ Automatic environment setup

Start developing in seconds with a single command!
