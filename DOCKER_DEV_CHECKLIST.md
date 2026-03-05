# Docker Development Environment - Verification Checklist

## Pre-Build Verification

- [ ] Base Dockerfile exists: `Dockerfile`
- [ ] Development Dockerfile exists: `Dockerfile.dev`
- [ ] Build script exists: `scripts/build-dev-docker.sh`
- [ ] Build script is executable: `ls -l scripts/build-dev-docker.sh`
- [ ] Quick start script exists: `scripts/docker-dev-quick-start.sh`
- [ ] Quick start script is executable: `ls -l scripts/docker-dev-quick-start.sh`

## Build Verification

Run the build script:
```bash
./scripts/build-dev-docker.sh
```

The script should verify:
- [ ] Base image exists or is built
- [ ] Development image builds successfully
- [ ] Image size is reported (~1.2-1.5GB)

## Development Tools Verification

The build script checks these tools automatically:
- [ ] vim
- [ ] curl
- [ ] wget
- [ ] htop
- [ ] strace
- [ ] gdb
- [ ] tmux
- [ ] tree
- [ ] jq
- [ ] inotifywait (for watch mode)
- [ ] make (for Makefile commands)

## Development Scripts Verification

The build script checks these scripts automatically:
- [ ] dev-setup
- [ ] quick-test
- [ ] dev-status

## Development Directories Verification

The build script checks these directories automatically:
- [ ] /app/dev-workspace
- [ ] /app/logs
- [ ] /app/tmp

## Configuration Files Verification

The build script checks these files automatically:
- [ ] /root/.bashrc
- [ ] /root/.vimrc
- [ ] /app/.dev-config

## Manual Testing

### 1. Run Quick Test
```bash
docker run --rm testcase-manager:dev quick-test
```

Expected output:
- [ ] Binary availability test passes
- [ ] Schema file check passes
- [ ] Testcases directory check passes
- [ ] Watch mode script check passes

### 2. Run Dev Setup
```bash
docker run --rm testcase-manager:dev dev-setup
```

Expected output:
- [ ] All binaries verified
- [ ] All directories verified
- [ ] Watch mode prerequisites verified

### 3. Interactive Session
```bash
docker run -it --rm -v $(pwd):/app testcase-manager:dev
```

Inside container, test:
- [ ] `show-help` displays help message
- [ ] `show-binaries` lists all binaries
- [ ] `dev-status` shows environment status
- [ ] `ll` works (colored ls)
- [ ] `validate-file` function exists
- [ ] `watch-yaml` alias works

### 4. Test Base Binaries
```bash
docker run --rm testcase-manager:dev tcm --version
docker run --rm testcase-manager:dev test-executor --help
docker run --rm testcase-manager:dev validate-yaml --help
docker run --rm testcase-manager:dev test-orchestrator --help
```

Expected:
- [ ] All binaries execute without errors

### 5. Test Volume Mount
```bash
docker run -it --rm -v $(pwd):/app testcase-manager:dev ls -la /app
```

Expected:
- [ ] Local files are visible in container

### 6. Test Watch Mode Prerequisites
```bash
docker run --rm testcase-manager:dev which inotifywait
docker run --rm testcase-manager:dev which make
```

Expected:
- [ ] Both commands return paths

### 7. Test Environment Variables
```bash
docker run --rm testcase-manager:dev env | grep RUST_
docker run --rm testcase-manager:dev env | grep VERBOSE
```

Expected:
- [ ] RUST_BACKTRACE=1
- [ ] RUST_LOG=debug
- [ ] CARGO_TERM_COLOR=always

## Makefile Targets Verification

### 1. Build Target
```bash
make docker-build-dev
```

Expected:
- [ ] Runs build script
- [ ] All verifications pass

### 2. Run Target
```bash
make docker-run-dev
```

Expected:
- [ ] Starts interactive container
- [ ] Volume mounted to /app
- [ ] Can exit cleanly

## Documentation Verification

- [ ] DOCKER_DEV_SETUP.md exists and is complete
- [ ] DOCKER_DEV_README.md exists and is complete
- [ ] IMPLEMENTATION_DOCKER_DEV.md exists
- [ ] DOCKER_DEV_CHECKLIST.md exists (this file)
- [ ] AGENTS.md updated with Docker commands
- [ ] Makefile contains docker-build-dev target
- [ ] Makefile contains docker-run-dev target

## Quick Start Script Verification

Run the quick start script:
```bash
./scripts/docker-dev-quick-start.sh
```

Expected:
- [ ] Checks Docker installation
- [ ] Checks Docker daemon
- [ ] Checks for base image
- [ ] Checks for dev image
- [ ] Offers to build if needed
- [ ] Shows usage options
- [ ] Offers interactive session
- [ ] Displays quick reference

## Integration Testing

### Test 1: Fresh Build
```bash
# Remove images
docker rmi testcase-manager:dev testcase-manager:latest

# Build from scratch
./scripts/build-dev-docker.sh
```

Expected:
- [ ] Base image builds
- [ ] Dev image builds
- [ ] All verifications pass

### Test 2: Development Workflow
```bash
# Start container
make docker-run-dev

# Inside container:
show-help
dev-status
quick-test
validate-file testcases/example.yml  # if file exists
exit
```

Expected:
- [ ] All commands work
- [ ] No errors
- [ ] Clean exit

### Test 3: Watch Mode
```bash
# Start container
docker run -it --rm -v $(pwd):/app testcase-manager:dev

# Inside container:
watch-yaml  # Ctrl+C to stop after verifying it starts
```

Expected:
- [ ] Watch mode starts
- [ ] Can be stopped with Ctrl+C

## Cleanup Verification

After all tests, verify cleanup:
```bash
# Check running containers
docker ps

# Check images
docker images | grep testcase-manager
```

Expected:
- [ ] No dangling containers
- [ ] Images exist: testcase-manager:latest and testcase-manager:dev

## Final Checklist

- [ ] All files created
- [ ] All scripts executable
- [ ] Build script runs successfully
- [ ] Quick start script runs successfully
- [ ] Development image builds on top of base image
- [ ] All development tools installed
- [ ] All development scripts present
- [ ] All configuration files present
- [ ] All directories created
- [ ] Base binaries still work
- [ ] Volume mounts work
- [ ] Watch mode prerequisites installed
- [ ] Documentation complete
- [ ] Makefile targets work
- [ ] AGENTS.md updated

## Success Criteria

All items above should be checked off. If any item fails:

1. Check the relevant documentation
2. Review error messages
3. Verify base image is built
4. Try rebuilding without cache:
   ```bash
   docker build --no-cache -f Dockerfile.dev -t testcase-manager:dev .
   ```

## Quick Reference

### Key Files
- `Dockerfile.dev` - Development Dockerfile
- `scripts/build-dev-docker.sh` - Build and verify
- `scripts/docker-dev-quick-start.sh` - Interactive setup
- `DOCKER_DEV_SETUP.md` - Complete guide
- `DOCKER_DEV_README.md` - Quick reference

### Key Commands
- `make docker-build-dev` - Build dev image
- `make docker-run-dev` - Run dev container
- `./scripts/build-dev-docker.sh` - Build with verification
- `./scripts/docker-dev-quick-start.sh` - Interactive setup

### Inside Container
- `show-help` - Show all commands
- `dev-status` - Environment status
- `quick-test` - Run tests
- `show-binaries` - List binaries

---

**Status**: ☐ Not Started | ☑ Complete

**Last Updated**: [Date of implementation]

**Verified By**: [Your name]
