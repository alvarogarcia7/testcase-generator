# Docker Development Environment - Quick Start

## TL;DR

```bash
# Quick start - builds and launches dev environment
./scripts/docker-dev-quick-start.sh

# Or manually:
make docker-build-dev    # Build dev image
make docker-run-dev      # Run dev container
```

## What is it?

`Dockerfile.dev` creates a development Docker image that:
- ✅ Extends `testcase-manager:latest` base image
- ✅ Adds development tools (vim, curl, gdb, strace, etc.)
- ✅ Includes watch mode prerequisites (inotify-tools, make)
- ✅ Provides pre-configured shell with helpful aliases
- ✅ Sets up development-friendly CMD with auto-setup
- ✅ Includes debugging utilities and monitoring tools

## Quick Commands

| Command | Description |
|---------|-------------|
| `make docker-build-dev` | Build development image (with verification) |
| `make docker-run-dev` | Run interactive development session |
| `./scripts/docker-dev-quick-start.sh` | Interactive setup wizard |
| `./scripts/build-dev-docker.sh` | Build with full verification |

## Inside the Container

When you enter the development container, you have access to:

### Shell Commands
```bash
show-help              # Show all available commands
show-binaries          # List all available binaries
dev-status            # Show environment status
quick-test            # Run quick validation tests
```

### Project Commands
```bash
validate-file <file>   # Validate a YAML test case
generate-script <file> # Generate bash script from test case
run-orchestrator <file># Run test orchestrator
watch-yaml            # Start watch mode for continuous validation
run-tests             # Run full test suite
build-project         # Build all binaries (debug)
build-release         # Build all binaries (release)
lint-project          # Run linter
```

### Useful Aliases
```bash
ll                    # ls -lah with colors
tcm                   # testcase-manager shortcut
validate              # validate-yaml with default schema
```

## Development Workflow

### 1. Start Development Session
```bash
# Using make
make docker-run-dev

# Or directly
docker run -it --rm -v $(pwd):/app testcase-manager:dev
```

### 2. Inside Container - First Steps
```bash
# See available commands
show-help

# Check environment
dev-status

# List all tools
show-binaries
```

### 3. Validate Test Cases
```bash
# Validate a single file
validate-file testcases/example.yml

# Start watch mode for continuous validation
watch-yaml
```

### 4. Generate and Run Tests
```bash
# Generate bash script from test case
generate-script testcases/example.yml

# Run with orchestrator
run-orchestrator testcases/example.yml
```

### 5. Run Tests
```bash
# Run full test suite
run-tests

# Or use cargo directly
cargo test
```

## What's Included

### Development Tools
- **Editors**: vim (with .vimrc), nano
- **Network**: curl, wget, netcat, tcpdump, telnet
- **Debugging**: gdb, valgrind, strace, lsof
- **Monitoring**: htop, procps
- **Shell**: tmux, screen, bash-completion
- **Utilities**: tree, less, jq, man-db

### Watch Mode Support
- **inotify-tools**: File system monitoring
- **make**: Build system
- **watch-yaml-files.sh**: YAML validation watch script

### Pre-configured Environment
- **RUST_BACKTRACE=1**: Enable Rust backtraces
- **RUST_LOG=debug**: Debug-level logging
- **VERBOSE=1**: Verbose output
- **DEBUG=1**: Debug mode enabled

### Development Directories
- `/app/dev-workspace` - Development workspace
- `/app/logs` - Log files
- `/app/tmp` - Temporary files

## Examples

### Continuous Development
```bash
# Terminal 1: Watch mode for auto-validation
make docker-run-dev
# Inside container:
watch-yaml

# Terminal 2 (on host): Edit files
vim testcases/my-test.yml
# Changes are automatically validated
```

### Running Specific Tests
```bash
docker run --rm -v $(pwd):/app testcase-manager:dev \
  cargo test test_name
```

### Debugging
```bash
make docker-run-dev
# Inside container:
gdb --args /usr/local/bin/tcm --help
strace /usr/local/bin/tcm --version
htop  # Monitor processes
```

### Building Inside Container
```bash
make docker-run-dev
# Inside container:
build-project      # Debug build
build-release      # Release build
lint-project       # Lint
```

## Troubleshooting

### Base Image Not Found
```bash
# Build base image first
make docker-build

# Then build dev image
make docker-build-dev
```

### Permission Issues
```bash
# Run with user mapping
docker run -it --rm \
  -v $(pwd):/app \
  -u $(id -u):$(id -g) \
  testcase-manager:dev
```

### Rebuild from Scratch
```bash
# Clean rebuild
docker build --no-cache -f Dockerfile.dev -t testcase-manager:dev .
```

## File Structure

```
.
├── Dockerfile.dev                    # Development Dockerfile
├── DOCKER_DEV_SETUP.md              # Complete documentation
├── DOCKER_DEV_README.md             # This file (quick reference)
├── scripts/
│   ├── build-dev-docker.sh          # Build with verification
│   └── docker-dev-quick-start.sh    # Interactive setup
└── Makefile                          # Contains docker-build-dev target
```

## Image Details

- **Base**: testcase-manager:latest
- **Size**: ~1.2-1.5 GB (includes all dev tools)
- **OS**: Debian Bookworm
- **Shell**: bash with custom configuration

## Documentation

- **Complete Guide**: [DOCKER_DEV_SETUP.md](DOCKER_DEV_SETUP.md)
- **Base Docker**: [DOCKER_BUILD_INSTRUCTIONS.md](DOCKER_BUILD_INSTRUCTIONS.md)
- **Development Guide**: [AGENTS.md](AGENTS.md)
- **Project README**: [README.md](README.md)

## FAQ

**Q: Do I need the base image?**  
A: Yes, the dev image extends `testcase-manager:latest`. Build it with `make docker-build` or the quick-start script will build it for you.

**Q: How do I save my changes?**  
A: Use volume mounts (`-v $(pwd):/app`) so changes persist on your host.

**Q: Can I use my own editor?**  
A: Yes! Edit files on your host, they'll be reflected in the container via volume mount.

**Q: How do I stop the container?**  
A: Type `exit` or press Ctrl+D.

**Q: Where are the binaries?**  
A: All binaries are in `/usr/local/bin/` and already in PATH.

## Next Steps

1. Run quick start: `./scripts/docker-dev-quick-start.sh`
2. Read complete guide: `cat DOCKER_DEV_SETUP.md`
3. Start developing: `make docker-run-dev`
4. Get help inside: `show-help`

Happy developing! 🚀
