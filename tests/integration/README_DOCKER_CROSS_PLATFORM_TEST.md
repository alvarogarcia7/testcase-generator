# Docker Cross-Platform Compatibility Test

End-to-end integration test for Docker cross-platform compatibility between macOS and Linux.

## Overview

This test validates that Docker-based MkDocs documentation builds work identically on both macOS and Linux platforms, with proper handling of platform-specific differences in:

- File system mounting and permissions
- Command-line utility variants (BSD vs GNU)
- Path separators and path handling
- Docker Desktop vs native Docker behavior
- Volume mount performance and ownership

## Test Coverage

### 1. Platform Detection
- Detects operating system (macOS, Linux, Windows)
- Identifies architecture (x86_64, arm64, etc.)
- Detects Docker Desktop on macOS
- Identifies utility variants (BSD vs GNU sed, grep, stat, find)
- Reports host user UID/GID

### 2. Docker Build Compatibility
- Tests `make docs-docker-build` works identically on both platforms
- Verifies image builds produce consistent results
- Checks image size and metadata
- Validates MkDocs installation in container
- Verifies non-root user configuration

### 3. Volume Mount Compatibility
- Tests `make docs-docker-build-site` volume mounts
- Validates that file paths work correctly across platforms
- Checks site/ directory creation and accessibility
- Verifies host can read/write/delete generated files
- Tests volume mounts with spaces in paths (macOS specific)

### 4. Permission Handling
- Checks site/ directory ownership on both platforms
- Validates platform-specific permission models:
  - **Linux**: Direct volume mounts, ownership matches host UID/GID
  - **macOS**: osxfs with transparent permission handling
- Tests write access from host
- Tests deletion without sudo

### 5. Script Compatibility
- Tests docker-mkdocs.sh script with platform utilities
- Checks for BSD/GNU compatibility issues:
  - `sed -r` (GNU) vs `sed -E` (BSD)
  - `grep -P` (GNU-only Perl regex)
  - `readlink -f` (GNU-only)
  - `declare -A` (bash 4.0+, macOS has bash 3.2)
- Verifies script help and status commands work

### 6. Development Server
- Tests `make docs-docker-serve` volume mounts
- Validates live reload functionality
- Tests creating/modifying files from host while server runs
- Verifies server stability across platforms

### 7. Docker Compose Compatibility
- Validates docker-compose.mkdocs.yml syntax
- Tests `make docs-compose-build-site`
- Verifies compose services work on both platforms
- Checks volume mounts in compose configuration

### 8. Path Separator Handling
- Verifies no Windows-style backslashes in paths
- Checks Makefile uses $(PWD) for cross-platform paths
- Validates Docker Compose uses relative paths (./)
- Tests that volume mount syntax is correct

### 9. Platform-Specific Behavior
- **macOS**:
  - Detects Docker Desktop
  - Checks osxfs volume mount behavior
  - Tests spaces in paths
  - Notes file system type
- **Linux**:
  - Checks SELinux status
  - Verifies Docker socket permissions
  - Detects WSL (Windows Subsystem for Linux)
  - Tests direct volume mount behavior

### 10. Cleanup and Permissions
- Tests cleanup of generated files
- Verifies files can be deleted without sudo
- Tests permission fixing with chmod if needed

## Usage

### Basic Usage

```bash
# Run the test
./tests/integration/test_docker_cross_platform_e2e.sh

# Or using make
make docs-docker-test-cross-platform
```

### Options

```bash
# Keep temporary files for debugging
./tests/integration/test_docker_cross_platform_e2e.sh --no-remove

# Enable verbose output
./tests/integration/test_docker_cross_platform_e2e.sh --verbose

# Combine options
./tests/integration/test_docker_cross_platform_e2e.sh --no-remove --verbose
```

## Platform-Specific Behavior

### macOS with Docker Desktop

**File Ownership:**
- Docker Desktop uses osxfs for volume mounts
- Files appear to be owned by host user (transparent virtualization)
- Actual ownership in container may differ but is handled by osxfs

**Performance:**
- Volume mount performance may be slower than Linux
- File watching (live reload) works but may have slight delay

**Compatibility:**
- BSD variants of command-line tools (sed, grep, stat, find)
- bash 3.2 is the default (no bash 4.0+ features)
- Docker Desktop handles most compatibility issues automatically

### Linux with Native Docker

**File Ownership:**
- Direct volume mounts with host filesystem
- Container files owned by container user UID (typically 1000)
- Host files maintain their original ownership

**Performance:**
- Native volume mount performance (no virtualization overhead)
- File watching is immediate and efficient

**Compatibility:**
- GNU variants of command-line tools
- Modern bash version (typically 4.0+)
- May require SELinux volume mount flags (:z or :Z) if enforcing

### Windows Subsystem for Linux (WSL)

When running in WSL:
- Behaves like native Linux
- Docker socket typically managed by WSL integration
- Volume mounts work normally
- Full Linux compatibility

## Expected Results

### Success Criteria

All tests should pass on both macOS and Linux:
- ✓ Docker image builds successfully
- ✓ Volume mounts work correctly
- ✓ Files have correct permissions
- ✓ Host can read/write/delete generated files
- ✓ No path separator issues
- ✓ Scripts compatible with platform utilities
- ✓ Development server works with live reload
- ✓ Docker Compose commands work correctly

### Platform Differences (Normal)

Some differences are expected and normal:

**File Ownership:**
- macOS: Files appear owned by host user (osxfs handles this)
- Linux: Files owned by container UID (typically 1000)

**Utility Variants:**
- macOS: BSD variants of sed, grep, stat, find
- Linux: GNU variants of sed, grep, stat, find

**Performance:**
- macOS: Slightly slower volume mount performance
- Linux: Native performance, no virtualization overhead

## Compatibility Report

The test generates a detailed compatibility report including:

```
Docker Cross-Platform Compatibility Report
==========================================

Platform Information:
- Operating System: macOS (Darwin)
- Architecture: arm64
- Docker Desktop: Yes

Utility Variants:
- sed: BSD
- grep: BSD
- stat: BSD
- find: BSD

Docker Information:
- Docker Version: Docker version 24.0.0, build 1234567
- Docker Compose: docker-compose version 1.29.2

Image Information:
- Image: testcase-manager-docs:latest
- Size: 450MB
- ID: abc123def456

Test Results:
- Tests Passed: 15
- Tests Failed: 0

Platform-Specific Notes:
- macOS uses osxfs for volume mounts (transparent permission handling)
- File ownership may differ from host user due to osxfs virtualization
- Docker Desktop handles most compatibility issues automatically
- Volume mount performance may be slower than Linux
```

Report saved to: `$TEMP_DIR/compatibility_report.txt`

## Troubleshooting

### Common Issues

**Port Already in Use:**
```
Error: Port 8000 is already in use
```
Solution: Stop any running MkDocs server or specify different port

**Permission Denied:**
```
Error: Cannot delete site/ directory
```
Solution: Run `chmod -R u+w site/ && rm -rf site/`

**Docker Not Running:**
```
Error: Docker daemon is not running
```
Solution: Start Docker Desktop (macOS) or Docker service (Linux)

**SELinux Issues (Linux):**
```
Error: Permission denied in container
```
Solution: Add `:z` or `:Z` to volume mounts if SELinux is enforcing

### Platform-Specific Troubleshooting

**macOS:**
- Ensure Docker Desktop is running
- Check File Sharing preferences in Docker Desktop
- Restart Docker Desktop if volume mounts fail

**Linux:**
- Check Docker socket permissions: `ls -l /var/run/docker.sock`
- Verify user is in docker group: `groups`
- Check SELinux status: `getenforce`

## Integration

### CI/CD Integration

This test can be run in CI/CD pipelines to verify cross-platform compatibility:

```yaml
# GitLab CI example
test:docker-cross-platform:
  stage: test
  script:
    - make docs-docker-test-cross-platform
  only:
    - main
    - merge_requests
```

```yaml
# GitHub Actions example
- name: Test Docker Cross-Platform Compatibility
  run: make docs-docker-test-cross-platform
```

### Pre-Commit Hook

Add to `.git/hooks/pre-push`:

```bash
#!/usr/bin/env bash
echo "Running Docker cross-platform tests..."
make docs-docker-test-cross-platform
```

## Related Tests

- `test_docker_mkdocs_e2e.sh` - Basic Docker MkDocs setup test
- `test_docker_volume_permissions_e2e.sh` - Detailed volume permission test
- `test_docker_compose_mkdocs_e2e.sh` - Docker Compose workflow test
- `test_docker_serve_e2e.sh` - Development server test

## Test Implementation Details

### Test Structure

1. **Platform Detection**: Identify OS, architecture, and utilities
2. **Prerequisites**: Check Docker, docker-compose, helper scripts
3. **Path Handling**: Verify path separator usage
4. **Docker Build**: Test image build consistency
5. **Volume Mounts**: Test site/ directory generation
6. **Permissions**: Test file access and deletion
7. **Script Compatibility**: Test helper script with platform utils
8. **Development Server**: Test live reload and volume mounts
9. **Docker Compose**: Test compose commands if available
10. **Path Separators**: Verify Makefile and compose file syntax
11. **Platform-Specific**: Run platform-specific checks
12. **Cleanup**: Test file deletion and cleanup
13. **Report**: Generate compatibility report
14. **Summary**: Display test results

### Test Duration

- **Typical runtime**: 3-5 minutes
- **With verbose output**: 4-6 minutes
- **First run (image build)**: 5-8 minutes

### Exit Codes

- `0`: All tests passed
- `1`: One or more tests failed
- Non-zero: Script error or prerequisites not met

## Contributing

When adding Docker functionality:

1. **Test on both platforms**: Verify on macOS and Linux
2. **Use portable syntax**: Avoid GNU/BSD-specific commands
3. **Check helper script**: Ensure docker-mkdocs.sh is compatible
4. **Update this test**: Add new test cases as needed
5. **Document differences**: Note platform-specific behavior

## Shell Script Compatibility

All Docker scripts must follow these requirements:

- **Bash 3.2+ compatible**: macOS ships with bash 3.2
- **No bash 4.0+ features**: Avoid associative arrays (`declare -A`)
- **Use `sed -E`**: Not `sed -r` (GNU-only)
- **Avoid `grep -P`**: Perl regex not available on BSD
- **Avoid `readlink -f`**: GNU-only, not on BSD
- **Portable stat**: Use platform-specific formats
- **Test both platforms**: Run on macOS and Linux when possible

## References

- [Docker Documentation](https://docs.docker.com/)
- [Docker Desktop for Mac](https://docs.docker.com/desktop/mac/)
- [Docker on Linux](https://docs.docker.com/engine/install/)
- [Docker Compose](https://docs.docker.com/compose/)
- [Bash Scripting Guide](https://www.gnu.org/software/bash/manual/)
- [BSD vs GNU Differences](https://itectec.com/unixlinux/unix-linux-what-are-the-differences-between-bsd-and-gnu-utils/)
