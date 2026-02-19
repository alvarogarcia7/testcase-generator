# Docker Cross-Platform Compatibility Implementation

## Overview

Comprehensive end-to-end test for Docker cross-platform compatibility between macOS and Linux, ensuring all Docker-based MkDocs documentation builds work identically across platforms.

## Files Created

### Test Script
- **tests/integration/test_docker_cross_platform_e2e.sh**
  - Main test script (862 lines)
  - 15 comprehensive test sections
  - Platform detection and compatibility checks
  - Generates detailed compatibility report
  - Executable: `chmod +x` applied

### Documentation
- **tests/integration/README_DOCKER_CROSS_PLATFORM_TEST.md**
  - Complete guide (480+ lines)
  - Detailed test coverage documentation
  - Platform-specific behavior explanation
  - Troubleshooting guide
  - CI/CD integration examples

- **tests/integration/DOCKER_CROSS_PLATFORM_TEST_QUICK_REF.md**
  - Quick reference (180+ lines)
  - Common commands and usage
  - Expected results and troubleshooting
  - Fast lookup for developers

- **scripts/DOCKER_CROSS_PLATFORM_QUICK_REF.md**
  - Scripts directory reference (300+ lines)
  - Portability requirements
  - Platform detection patterns
  - CI/CD integration examples

### Configuration Updates
- **Makefile** - Added `docs-docker-test-cross-platform` target
- **AGENTS.md** - Added test command documentation

## Test Coverage

### 1. Platform Detection (Test 1)
- Detects OS: Linux, macOS (Darwin), Windows, Unknown
- Identifies architecture: x86_64, arm64, etc.
- Detects Docker Desktop on macOS
- Identifies utility variants:
  - sed (BSD vs GNU)
  - grep (BSD vs GNU)
  - stat (BSD vs GNU)
  - find (BSD vs GNU)
- Reports host user UID/GID

### 2. Prerequisites Check (Test 2)
- Verifies Docker installed and running
- Checks Docker version
- Verifies docker-compose installed (optional)
- Checks docker-mkdocs.sh helper script exists
- Displays versions for debugging

### 3. Path Separator Handling (Test 3)
- Tests path format (forward slashes)
- Checks for Windows-style backslashes
- Verifies PWD environment variable
- Checks Makefile uses $(PWD)
- Ensures cross-platform path compatibility

### 4. Resource Cleanup (Test 4)
- Removes existing site/ directory
- Removes existing Docker image
- Handles permission issues during cleanup
- Prepares clean environment for testing

### 5. Docker Build Test (Test 5)
- Tests `make docs-docker-build` on current platform
- Times the build process
- Verifies image creation
- Checks image metadata (size, ID, creation time)
- Captures build logs for troubleshooting

### 6. Build Consistency (Test 6)
- Tests MkDocs accessibility in container
- Verifies MkDocs version
- Checks container runs as non-root user
- Validates user configuration (UID, username)

### 7. Volume Mount Testing (Test 7)
- Tests `make docs-docker-build-site`
- Verifies site/ directory creation
- Times the build process
- Validates volume mounts work correctly
- Tests file generation from container to host

### 8. Permission Validation (Test 8)
- Platform-specific permission checks
- **macOS**: osxfs permission handling, Docker Desktop behavior
- **Linux**: Direct mount ownership, UID/GID matching
- Tests host read/write access
- Tests file deletion without sudo
- Verifies index.html generation and readability
- Uses platform-appropriate stat commands

### 9. Helper Script Compatibility (Test 9)
- Tests docker-mkdocs.sh script execution
- Verifies help command works
- Verifies status command works
- Checks for GNU/BSD compatibility issues:
  - `sed -r` (GNU-only) vs `sed -E` (portable)
  - `grep -P` (GNU-only Perl regex)
  - `readlink -f` (GNU-only)
  - `declare -A` (bash 4.0+, not on macOS)
- Reports compatibility warnings

### 10. Development Server Test (Test 10)
- Tests `make docs-docker-serve` volume mounts
- Starts development server in background
- Waits for server startup
- Tests live reload by creating files
- Verifies server stability
- Tests file modification while running
- Properly stops server after test
- Skips if port already in use

### 11. Docker Compose Test (Test 11)
- Only runs if docker-compose is installed
- Validates docker-compose.mkdocs.yml syntax
- Tests `make docs-compose-build-site`
- Verifies compose volume mounts
- Checks site/ directory generation
- Cleans up compose services
- Gracefully skips if compose not available

### 12. Path Separator Verification (Test 12)
- Checks Makefile for path separator issues
- Verifies no Windows-style backslashes
- Checks $(PWD) usage in Makefile
- Validates docker-compose.mkdocs.yml paths
- Verifies relative path usage (./)

### 13. Platform-Specific Checks (Test 13)
**macOS-specific:**
- Detects Docker Desktop
- Identifies file system type
- Tests volume mounts with spaces in paths
- Notes osxfs behavior

**Linux-specific:**
- Checks SELinux status and enforcement
- Verifies Docker socket permissions
- Detects WSL (Windows Subsystem for Linux)
- Notes direct volume mount behavior

### 14. Cleanup Testing (Test 14)
- Tests deletion of site/ directory
- Verifies no sudo required
- Times the deletion process
- Tests permission fixing if needed
- Validates complete cleanup

### 15. Compatibility Report (Test 15)
- Generates comprehensive compatibility report
- Includes platform information
- Lists utility variants detected
- Shows Docker and Compose versions
- Reports image information
- Summarizes test results
- Adds platform-specific notes
- Saves report to temp directory

## Key Features

### Cross-Platform Path Handling
- Detects path separators automatically
- Uses $(PWD) for Makefile compatibility
- Validates Docker volume mount syntax
- Tests relative paths in compose files

### BSD/GNU Utility Compatibility
- Detects utility variants automatically
- Uses portable command syntax
- Platform-specific stat commands
- Warns about incompatible constructs

### Permission Model Differences
- **macOS**: osxfs transparent permissions
- **Linux**: Direct mount with UID/GID
- Platform-appropriate validation
- Tests write/delete from host

### Docker Desktop vs Native
- Detects Docker Desktop on macOS
- Identifies native Docker on Linux
- Tests platform-specific features
- Documents expected differences

### Comprehensive Reporting
- Platform detection summary
- Utility variant identification
- Docker version information
- Test pass/fail statistics
- Platform-specific notes
- Saved to file for reference

## Usage Examples

### Basic Usage
```bash
# Run via make
make docs-docker-test-cross-platform

# Run directly
./tests/integration/test_docker_cross_platform_e2e.sh

# With verbose output
./tests/integration/test_docker_cross_platform_e2e.sh --verbose

# Keep temp files for debugging
./tests/integration/test_docker_cross_platform_e2e.sh --no-remove

# Combined options
./tests/integration/test_docker_cross_platform_e2e.sh --verbose --no-remove
```

### CI/CD Integration

**GitLab CI:**
```yaml
test:docker-cross-platform:
  stage: test
  script:
    - make docs-docker-test-cross-platform
  artifacts:
    when: always
    paths:
      - /tmp/test_*/compatibility_report.txt
```

**GitHub Actions:**
```yaml
- name: Docker Cross-Platform Test
  run: make docs-docker-test-cross-platform

- name: Upload Report
  if: always()
  uses: actions/upload-artifact@v3
  with:
    name: compatibility-report
    path: /tmp/test_*/compatibility_report.txt
```

## Platform-Specific Behavior

### macOS with Docker Desktop

**Expected Behavior:**
- osxfs handles volume mount permissions transparently
- Files appear owned by host user
- BSD variants of command-line tools
- bash 3.2 by default (no bash 4.0+ features)
- Slightly slower volume mount performance

**Test Adaptations:**
- Uses BSD stat format (-f instead of -c)
- Expects osxfs permission handling
- Tests spaces in paths
- Notes file system virtualization

### Linux with Native Docker

**Expected Behavior:**
- Direct volume mounts with host filesystem
- Files owned by container UID (typically 1000)
- GNU variants of command-line tools
- Modern bash version (4.0+)
- Native performance

**Test Adaptations:**
- Uses GNU stat format (-c)
- Expects UID/GID matching
- Checks SELinux status
- Tests direct mount behavior
- Detects WSL environment

## Compatibility Requirements

### Shell Script Requirements
- Bash 3.2+ compatible (macOS default)
- No bash 4.0+ features (no `declare -A`)
- Use `sed -E` (not `sed -r`)
- Avoid `grep -P` (Perl regex)
- Avoid `readlink -f` (GNU-only)
- Platform-specific stat commands
- Test on both platforms

### Docker Requirements
- Docker installed and running
- User has Docker permissions
- Port 8000 available (for serve test)
- Sufficient disk space (>2GB)
- File sharing enabled (macOS)

### Optional Requirements
- docker-compose (for compose tests)
- curl (for server connectivity tests)
- lsof or netstat (for port checking)

## Test Output

### Success Example
```
=== Docker Cross-Platform Compatibility Test ===
✓ Platform detected: macOS
✓ Docker is installed
✓ Docker daemon is running
✓ Path uses forward slashes
✓ Docker image built successfully in 45s
✓ Volume mounts work correctly
✓ File permissions are correct
✓ Helper script compatible
✓ Development server works
✓ Docker Compose works
✓ No path separator issues
✓ Platform-specific checks pass
✓ Files can be deleted without sudo
✓ Compatibility report generated

=== Test Summary ===
Total tests: 15
Tests passed: 15
Tests failed: 0
Platform: macOS

✓ All Docker cross-platform compatibility tests passed successfully!
```

### Compatibility Report Example
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
- Docker Version: Docker version 24.0.0
- Docker Compose: docker-compose version 1.29.2

Image Information:
- Image: testcase-manager-docs:latest
- Size: 450MB
- ID: abc123def456

Test Results:
- Tests Passed: 15
- Tests Failed: 0

Platform-Specific Notes:
- macOS uses osxfs for volume mounts
- File ownership may differ from host user
- Docker Desktop handles most compatibility issues
- Volume mount performance may be slower than Linux
```

## Error Handling

### Port Already in Use
- Detects port conflicts before starting server
- Skips serve test if port unavailable
- Provides clear error message

### Permission Issues
- Attempts chmod before deletion
- Provides fallback cleanup methods
- Documents manual cleanup steps

### Docker Not Running
- Checks Docker availability early
- Provides platform-specific start commands
- Exits early with clear error

### Platform Detection
- Handles unknown platforms gracefully
- Runs basic tests regardless
- Documents detected configuration

## Test Duration

- **First run** (with image build): 5-8 minutes
- **Subsequent runs** (cached image): 3-5 minutes
- **With verbose output**: +1 minute
- **Platform-specific variations**:
  - macOS: Slightly longer due to osxfs
  - Linux: Faster with native mounts

## Integration with Existing Tests

### Related Tests
- `test_docker_mkdocs_e2e.sh` - Basic Docker setup
- `test_docker_volume_permissions_e2e.sh` - Detailed permissions
- `test_docker_compose_mkdocs_e2e.sh` - Compose workflow
- `test_docker_serve_e2e.sh` - Development server
- `test_docker_html_build_e2e.sh` - HTML generation
- `test_docker_pdf_build_e2e.sh` - PDF generation

### Test Hierarchy
1. **Basic**: test_docker_mkdocs_e2e.sh
2. **Platform**: test_docker_cross_platform_e2e.sh ← This test
3. **Specialized**: Other Docker tests

## Maintenance

### Adding New Tests
1. Add new test section with clear description
2. Increment TESTS_PASSED or TESTS_FAILED appropriately
3. Update documentation
4. Test on both macOS and Linux

### Updating for New Platforms
1. Add platform detection in Test 1
2. Add platform-specific checks in Test 13
3. Update compatibility report template
4. Document platform differences

### Compatibility Checks
When adding Docker functionality:
1. Test on macOS (BSD utilities, bash 3.2)
2. Test on Linux (GNU utilities, modern bash)
3. Update this test if needed
4. Update helper scripts for compatibility
5. Document platform differences

## Success Criteria

All 15 test sections must pass:
1. ✓ Platform detection
2. ✓ Prerequisites check
3. ✓ Path handling
4. ✓ Resource cleanup
5. ✓ Docker build
6. ✓ Build consistency
7. ✓ Volume mounts
8. ✓ Permissions
9. ✓ Script compatibility
10. ✓ Development server
11. ✓ Docker Compose (if available)
12. ✓ Path separator verification
13. ✓ Platform-specific checks
14. ✓ Cleanup testing
15. ✓ Compatibility report

## Known Limitations

1. **Windows native not tested** - Only WSL supported on Windows
2. **Compose optional** - Tests skip if not installed
3. **Port conflict** - Server test skips if port in use
4. **Network required** - For Docker image layers
5. **Disk space** - Requires >2GB for image and site

## Future Enhancements

Potential improvements:
1. Test with different Docker versions
2. Test with Docker Compose v2
3. Add Windows native Docker support
4. Test with custom ports
5. Add performance benchmarks
6. Test with multi-platform images
7. Add ARM-specific tests
8. Test with rootless Docker

## References

- Docker Documentation: https://docs.docker.com/
- Docker Desktop for Mac: https://docs.docker.com/desktop/mac/
- Docker on Linux: https://docs.docker.com/engine/install/
- Docker Compose: https://docs.docker.com/compose/
- Bash Compatibility: https://www.gnu.org/software/bash/manual/
- BSD vs GNU: https://itectec.com/unixlinux/unix-linux-what-are-the-differences-between-bsd-and-gnu-utils/
