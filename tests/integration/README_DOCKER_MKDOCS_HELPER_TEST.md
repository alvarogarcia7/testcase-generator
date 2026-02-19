# Docker MkDocs Helper Script Test Documentation

## Overview

The `test_docker_mkdocs_helper_e2e.sh` script provides comprehensive end-to-end testing for the `docker-mkdocs.sh` helper script. It validates all commands, options, and error handling scenarios.

## Test Script Location

```
tests/integration/test_docker_mkdocs_helper_e2e.sh
```

## What It Tests

The test script validates the following functionality:

### 1. Help and Usage Display
- `--help` flag displays usage correctly
- `help` command shows usage information
- `-h` flag shows usage information
- No arguments displays usage
- Unknown commands show error and usage

### 2. Build Command
- `build` command builds Docker image successfully
- Proper logging with colored output
- Verbose mode with `--verbose` flag
- Image creation verification

### 3. Status Command
- `status` displays image information
- Shows container status
- Handles missing images gracefully

### 4. Serve Command
- `serve` starts development server
- Server accessible at http://localhost:8000
- Colored output in logs
- Live reload functionality
- `serve --port 8080` uses custom port
- Server responds to HTTP requests

### 5. Build Commands
- `build-site` generates static HTML documentation
- `build-pdf` generates documentation with PDF export
- Proper logging and colored output
- Site directory creation
- PDF file generation and validation

### 6. Docker Compose Commands
- `compose-build` builds static site using Docker Compose
- `compose-pdf` builds with PDF using Docker Compose
- `compose-up` starts development server
- `compose-down` stops services cleanly
- Service accessibility verification

### 7. Clean Command
- `clean` removes Docker image
- Removes site/ directory
- Proper logging output

### 8. Error Handling
- Handles missing Docker image gracefully
- Error messages are informative
- Usage displayed on errors

### 9. Verbose Mode
- `--verbose` flag enables detailed output
- Additional logging information displayed

### 10. Command Sequences
- Multiple commands run in sequence successfully
- State persists correctly between commands

## Prerequisites

### Required Software

- **Docker**: Docker Engine must be installed and running
- **Docker Compose**: docker-compose must be installed (for compose tests)
- **curl**: For testing HTTP endpoints
- **bash 3.2+**: Compatible with macOS and Linux

### Required Files

- `scripts/docker-mkdocs.sh`: The helper script being tested
- `Dockerfile.mkdocs`: Docker image definition
- `docker-compose.mkdocs.yml`: Docker Compose configuration
- `scripts/lib/logger.sh`: Logging library

### Permissions

- The test script must be executable: `chmod +x tests/integration/test_docker_mkdocs_helper_e2e.sh`
- Docker commands must be available (user in docker group or running as root)

## Usage

### Basic Usage

Run all tests:

```bash
./tests/integration/test_docker_mkdocs_helper_e2e.sh
```

### With Verbose Output

Enable detailed logging:

```bash
./tests/integration/test_docker_mkdocs_helper_e2e.sh --verbose
```

### Preserve Temporary Files

Keep temporary files for debugging:

```bash
./tests/integration/test_docker_mkdocs_helper_e2e.sh --no-remove
```

### Combined Options

```bash
./tests/integration/test_docker_mkdocs_helper_e2e.sh --verbose --no-remove
```

## Test Execution Flow

### Phase 1: Prerequisites (Tests 1-6)
1. Check Docker installation
2. Check Docker daemon status
3. Verify helper script exists
4. Test help flags and usage display
5. Test error handling for unknown commands
6. Clean existing resources

### Phase 2: Build and Status (Tests 7-9)
7. Build Docker image
8. Build with verbose flag
9. Check status command

### Phase 3: Site Generation (Tests 10-11)
10. Build static HTML site
11. Build documentation with PDF

### Phase 4: Server Testing (Tests 12-13)
12. Start development server on default port
13. Start server on custom port

### Phase 5: Docker Compose (Test 14)
14. Test all Docker Compose commands:
    - compose-build
    - compose-pdf
    - compose-up
    - compose-down

### Phase 6: Cleanup and Final Tests (Tests 15-20)
15. Test clean command
16. Test error handling
17. Test no arguments behavior
18. Rebuild for final verification
19. Final status check
20. Test command sequences

## Output Format

The test script uses the logger library for consistent output:

### Success Messages
```
✓ Message indicating success
```

### Failure Messages
```
✗ Message indicating failure
```

### Informational Messages
```
[INFO] Informational message
```

### Section Headers
```
=== Section Title ===
```

### Test Summary

At the end of execution, you'll see:

```
=== Test Summary ===
[INFO] Total tests: 20
[INFO] Tests passed: 20
[INFO] Tests failed: 0

✓ All docker-mkdocs.sh helper script tests passed successfully!
```

## Exit Codes

- **0**: All tests passed
- **1**: One or more tests failed

## Temporary Files

The test script creates temporary files in a directory like:

```
/tmp/tmp.XXXXXXXXXX/
```

Files created:
- `help_output.txt`: Output from --help flag
- `build_output.txt`: Output from build command
- `serve_output.txt`: Output from serve command
- `status_output.txt`: Output from status command
- `clean_output.txt`: Output from clean command
- Various other command outputs

These are automatically cleaned up unless `--no-remove` is specified.

## Background Processes

The test script manages background processes for server testing:
- Development servers are started in background
- PIDs are tracked for cleanup
- Servers are automatically stopped after testing
- Cleanup happens even on script interruption

## Common Issues and Solutions

### Issue: Port Already in Use

**Error**: Port 8000 or 8080 already in use

**Solution**: The script attempts to kill processes using these ports automatically. If this fails, manually stop the processes:

```bash
# Find and kill process on port 8000
lsof -ti:8000 | xargs kill -9

# Or stop Docker Compose services
docker-compose -f docker-compose.mkdocs.yml down
```

### Issue: Docker Not Running

**Error**: Docker daemon is not running

**Solution**: Start Docker:

```bash
# On macOS with Docker Desktop
open -a Docker

# On Linux with systemd
sudo systemctl start docker
```

### Issue: Permission Denied

**Error**: Permission denied when accessing Docker

**Solution**: Add your user to the docker group:

```bash
sudo usermod -aG docker $USER
newgrp docker
```

### Issue: Image Build Fails

**Error**: Build command fails

**Solution**: Check Dockerfile.mkdocs exists and is valid:

```bash
# Test Dockerfile syntax
docker build -f Dockerfile.mkdocs -t test-build .

# Check for common issues
cat Dockerfile.mkdocs
```

### Issue: Docker Compose Not Available

**Error**: docker-compose command not found

**Solution**: The test skips Docker Compose tests if not available. Install Docker Compose if needed:

```bash
# Check installation
which docker-compose

# Install if needed (varies by OS)
# See: https://docs.docker.com/compose/install/
```

## Interpreting Results

### All Tests Pass

When all tests pass, you'll see:

```
✓ All docker-mkdocs.sh helper script tests passed successfully!

[INFO] docker-mkdocs.sh is working correctly:
  ✓ --help displays usage correctly
  ✓ build command builds Docker image with proper logging
  ✓ serve command starts server with colored output
  ✓ serve --port 8080 uses custom port
  ✓ build-site generates static HTML documentation
  ✓ build-pdf generates documentation with PDF export
  ✓ status command shows image and container information
  ✓ clean command removes image and site/ directory
  ✓ Docker Compose commands work correctly
  ✓ Error handling works as expected
  ✓ --verbose flag enables verbose output
```

This indicates the helper script is fully functional.

### Some Tests Fail

When tests fail, you'll see:

```
✗ Some docker-mkdocs.sh helper script tests failed!

[ERROR] 3 test(s) failed
[INFO] Please review the output above and fix the issues
```

Review the output to identify which specific checks failed and why.

## Integration with CI/CD

This test can be integrated into CI/CD pipelines:

### GitLab CI

```yaml
test:docker-mkdocs-helper:
  stage: test
  image: docker:latest
  services:
    - docker:dind
  before_script:
    - apk add --no-cache bash curl
  script:
    - ./tests/integration/test_docker_mkdocs_helper_e2e.sh
  tags:
    - docker
```

### GitHub Actions

```yaml
- name: Test Docker MkDocs Helper Script
  run: |
    ./tests/integration/test_docker_mkdocs_helper_e2e.sh
```

## Debugging Failed Tests

### Enable Verbose Mode

Run with verbose output to see detailed information:

```bash
./tests/integration/test_docker_mkdocs_helper_e2e.sh --verbose
```

### Preserve Temporary Files

Keep temporary files for inspection:

```bash
./tests/integration/test_docker_mkdocs_helper_e2e.sh --no-remove --verbose
```

Then examine the temporary directory path shown in the output.

### Run Individual Commands

Test specific commands manually:

```bash
# Test help
./scripts/docker-mkdocs.sh --help

# Test build
./scripts/docker-mkdocs.sh build

# Test status
./scripts/docker-mkdocs.sh status

# Test serve
./scripts/docker-mkdocs.sh serve
```

### Check Docker Status

Verify Docker is working:

```bash
# Check Docker daemon
docker info

# Check Docker Compose
docker-compose --version

# List images
docker images

# List running containers
docker ps
```

### Check Logs

Examine container logs:

```bash
# Get container ID
docker ps -a

# View logs
docker logs <container-id>

# Or use Docker Compose
docker-compose -f docker-compose.mkdocs.yml logs
```

## Performance Considerations

### Test Duration

Expected test duration:
- **Fast tests** (help, status, error handling): < 1 minute
- **Build tests** (first time): 5-10 minutes
- **Build tests** (subsequent): 1-2 minutes
- **Server tests**: 1-2 minutes
- **Compose tests**: 2-3 minutes
- **Full suite**: 10-20 minutes (first run), 5-10 minutes (subsequent runs)

### Resource Usage

The test requires:
- **Disk space**: ~1 GB for Docker image
- **Memory**: ~512 MB for running containers
- **Network**: Internet access for first build (downloading base images and dependencies)

## Related Documentation

- [Docker MkDocs Test](README_DOCKER_MKDOCS_TEST.md) - Tests for Docker image itself
- [Docker Compose Test](README_DOCKER_COMPOSE_TEST.md) - Tests for Docker Compose workflow
- [Docker Serve Test Quick Reference](../scripts/DOCKER_SERVE_TEST_QUICK_REF.md) - Quick reference for development server tests

## Test Coverage

This test provides comprehensive coverage of:

- ✅ Command-line interface
- ✅ Help and usage display
- ✅ Build functionality
- ✅ Server functionality
- ✅ Site generation
- ✅ PDF generation
- ✅ Docker Compose integration
- ✅ Status reporting
- ✅ Cleanup operations
- ✅ Error handling
- ✅ Verbose mode
- ✅ Custom port handling
- ✅ Background process management

## Maintenance

### Adding New Tests

To add new tests:

1. Choose appropriate test number and section
2. Add section header: `section "Test N: Description"`
3. Implement test logic
4. Update test counter: `TESTS_PASSED=$((TESTS_PASSED + 1))` or `TESTS_FAILED=$((TESTS_FAILED + 1))`
5. Update this documentation

### Updating Tests

When updating `docker-mkdocs.sh`:

1. Review changes for new functionality
2. Add or update tests as needed
3. Ensure backward compatibility tests still pass
4. Update documentation

### Test Maintenance Checklist

- [ ] All test sections have clear descriptions
- [ ] Test counters are correct
- [ ] Temporary files are properly cleaned up
- [ ] Background processes are properly managed
- [ ] Error handling is comprehensive
- [ ] Documentation is up to date
- [ ] Examples are accurate

## Quick Reference

### Run Tests
```bash
./tests/integration/test_docker_mkdocs_helper_e2e.sh
```

### Verbose Mode
```bash
./tests/integration/test_docker_mkdocs_helper_e2e.sh --verbose
```

### Debug Mode
```bash
./tests/integration/test_docker_mkdocs_helper_e2e.sh --verbose --no-remove
```

### Check Prerequisites
```bash
# Check Docker
docker info

# Check script exists
ls -la scripts/docker-mkdocs.sh

# Check permissions
test -x scripts/docker-mkdocs.sh && echo "Executable" || echo "Not executable"
```

### Clean Up After Failed Test
```bash
# Stop containers
docker-compose -f docker-compose.mkdocs.yml down

# Remove image
docker rmi testcase-manager-docs:latest

# Remove site directory
rm -rf site/
```

## Summary

The `test_docker_mkdocs_helper_e2e.sh` script provides comprehensive testing of the `docker-mkdocs.sh` helper script, ensuring all commands, options, and error handling work correctly. It follows best practices for shell script testing and integrates with the project's logging infrastructure.

For questions or issues, refer to the troubleshooting section or examine the test output with `--verbose` flag enabled.
