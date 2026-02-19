# Docker Development Server Tests Implementation

## Summary

Implemented comprehensive end-to-end integration tests for the Docker MkDocs development server. The test suite validates server startup, accessibility, live editing, concurrent requests, clean shutdown, and custom port binding.

## Implementation Date

February 19, 2024

## Files Created

### Test Script
- `tests/integration/test_docker_serve_e2e.sh` - Main test script (18.5KB, executable)
  - 11 comprehensive test sections
  - 600+ lines of bash code
  - Full server lifecycle testing
  - Compatible with bash 3.2+ and BSD/GNU tools

### Documentation
- `scripts/README_DOCKER_SERVE_TEST.md` - Complete test documentation (9.6KB)
  - Detailed test coverage
  - Prerequisites and setup
  - Troubleshooting guide
  - Manual testing instructions
  - CI/CD integration examples

- `scripts/DOCKER_SERVE_TEST_QUICK_REF.md` - Quick reference guide (5.8KB)
  - Quick start commands
  - Test matrix
  - Common issues and solutions
  - Expected runtime and results

## Files Modified

### Makefile
Added new test target:
```makefile
docs-docker-test-serve:
	./tests/integration/test_docker_serve_e2e.sh
.PHONY: docs-docker-test-serve
```

### AGENTS.md
Updated documentation section to include new test command:
```markdown
- `make docs-docker-test-serve` - Run Docker development server e2e tests
```

## Test Coverage

### Test 1: Prerequisites Check
- Docker installation and daemon status
- curl availability for HTTP testing
- Docker image existence verification
- Port 8000 availability check

### Test 2: Server Startup
- Server starts successfully in Docker container
- Background process management
- Startup completes within 30 second timeout
- Server logs validated for success messages
- Server binds to 0.0.0.0:8000 inside container

### Test 3: Documentation Accessibility
- HTTP GET request to http://localhost:8000/ succeeds
- Response contains valid HTML structure
- Response has substantial content (>100 bytes)
- Material theme detected in response
- curl integration for testing

### Test 4: Multiple Documentation Endpoints
- Tests root endpoint (/)
- Tests 404 page
- Tests actual documentation sections (getting-started, user-guide, cli-tools)
- Validates at least one endpoint accessible
- Dynamic endpoint discovery based on docs/ structure

### Test 5: Concurrent Request Handling
- Launches 5 concurrent requests
- Validates server handles concurrent load
- Confirms all or most requests succeed (>75% threshold)
- Tests real-world usage patterns

### Test 6: Live Editing Capability
- Creates temporary markdown file in docs/
- Verifies volume mount functionality
- Tests server detects file changes
- Validates automatic rebuild triggered
- Modifies file to test multiple changes
- Confirms server stability after edits
- Cleans up test files automatically

### Test 7: Server Log Analysis
- Checks logs for error messages
- Counts warnings in output
- Validates clean operation
- Filters out benign errors (404s)
- Provides error context on failure

### Test 8: Clean Shutdown
- Sends SIGTERM to server process
- Validates graceful shutdown within 10 seconds
- Falls back to SIGKILL if necessary
- Confirms process termination

### Test 9: Custom Port Binding
- Tests custom port mapping (8080:8000)
- Validates server starts on custom port
- Tests accessibility on custom port
- Confirms clean shutdown
- Handles port unavailability gracefully

### Test 10: Make Command Verification
- Validates `make docs-docker-serve` configuration
- Documents expected behavior and usage
- Confirms Makefile integration

### Test 11: Complete Log Display
- Displays full server logs for review
- Aids debugging and verification
- Shows startup and operation messages

## Key Features

### Shell Script Compatibility
- **Bash 3.2+ compatible** - Works on macOS default bash
- **BSD/GNU portable** - No GNU-specific flags
- **No associative arrays** - Compatible with bash 3.2
- **POSIX-compliant** - Uses portable command syntax

### Logger Library Integration
- Uses centralized logging from `scripts/lib/logger.sh`
- Color-coded output (✓ pass, ✗ fail, ℹ info)
- Consistent formatting across all tests
- Verbose mode support

### Cleanup Management
- Automatic cleanup of temporary files
- Background process tracking and cleanup
- SIGTERM/SIGKILL handling for server processes
- `--no-remove` option for debugging
- Cleanup on script exit via trap

### Timing and Timeouts
- 30 second server startup timeout
- 10 second clean shutdown timeout
- Configurable wait times
- Progress indicators during waits

### Error Handling
- Comprehensive error checking
- Clear error messages with context
- Graceful degradation for optional tests
- Detailed failure information

### Test Organization
- Logical test sections
- Clear pass/fail criteria
- Test counter tracking
- Summary report at end

## Usage

### Basic Usage
```bash
# Run all server tests
make docs-docker-test-serve

# Run directly
./tests/integration/test_docker_serve_e2e.sh
```

### With Options
```bash
# Verbose output
./tests/integration/test_docker_serve_e2e.sh --verbose

# Keep temporary files
./tests/integration/test_docker_serve_e2e.sh --no-remove

# Both options
./tests/integration/test_docker_serve_e2e.sh --verbose --no-remove
```

### Prerequisites
```bash
# Build Docker image first
make docs-docker-build
```

## Expected Output

Successful test run produces:
- 11 test sections executed
- All tests passed (TESTS_PASSED=11, TESTS_FAILED=0)
- Server logs displayed
- Summary with success message
- Exit code 0

Failed test run produces:
- Failed test indicators (✗ symbol)
- Error details and context
- Server logs for debugging
- Summary with failure count
- Exit code 1

## Configuration

### Server Settings
```bash
SERVER_PORT=8000          # Default server port
CUSTOM_PORT=8080          # Custom port for testing
SERVER_HOST="localhost"   # Server hostname
MAX_STARTUP_TIME=30       # Server startup timeout (seconds)
MAX_SHUTDOWN_TIME=10      # Server shutdown timeout (seconds)
```

### Volume Mounts
```bash
-v "$PROJECT_ROOT/docs:/docs/docs"
-v "$PROJECT_ROOT/mkdocs.yml:/docs/mkdocs.yml"
-v "$PROJECT_ROOT/README.md:/docs/README.md"
-v "$PROJECT_ROOT/README_INSTALL.md:/docs/README_INSTALL.md"
```

### Server Command
```bash
mkdocs serve -a 0.0.0.0:8000
```

## Temporary Files

Created during test execution:
- `$TEMP_DIR/server.log` - Main server output
- `$TEMP_DIR/custom_server.log` - Custom port server output  
- `$TEMP_DIR/curl_output.html` - HTTP response content
- `$TEMP_DIR/concurrent_*.status` - Concurrent test results
- `$PROJECT_ROOT/docs/.test_edit_$$` - Live edit test file (cleaned up)

All temporary files are automatically removed unless `--no-remove` is specified.

## Integration Points

### Make Targets
- `make docs-docker-build` - Prerequisite (builds image)
- `make docs-docker-serve` - Server command being tested
- `make docs-docker-test-serve` - Runs this test suite

### Docker Image
- Uses `testcase-manager-docs:latest`
- Built by `Dockerfile.mkdocs`
- Contains MkDocs, Material theme, PDF support

### Logger Library
- Sources `scripts/lib/logger.sh`
- Uses all logging functions (log_info, log_error, etc.)
- Uses cleanup management (setup_cleanup, register_background_pid)
- Uses color helpers (pass, fail, info, section)

### Test Suite Integration
- Part of Docker documentation test suite
- Complements `test_docker_mkdocs_e2e.sh` (image tests)
- Complements `test_docker_html_build_e2e.sh` (build tests)
- Complements `test_docker_pdf_build_e2e.sh` (PDF tests)

## CI/CD Integration

### GitLab CI Example
```yaml
test-docker-serve:
  stage: test
  script:
    - make docs-docker-build
    - make docs-docker-test-serve
  tags:
    - docker
  timeout: 5m
```

### GitHub Actions Example
```yaml
- name: Test Docker Development Server
  run: |
    make docs-docker-build
    make docs-docker-test-serve
  timeout-minutes: 5
```

## Validation and Testing

### Script Syntax
```bash
# Verified with bash -n
bash -n tests/integration/test_docker_serve_e2e.sh
# ✓ No syntax errors

# Verify with make verify-scripts
make verify-scripts
# ✓ All scripts pass syntax check
```

### Compatibility Checks
- ✓ No bash 4+ features (no associative arrays)
- ✓ No GNU-specific flags (no grep -P, sed -r, readlink -f)
- ✓ POSIX-compliant command usage
- ✓ BSD and GNU tool compatibility
- ✓ Works on macOS and Linux

### Manual Testing
Tested scenarios:
1. Normal test run - all pass
2. Server startup with concurrent requests
3. Live file editing and rebuild
4. Clean shutdown with SIGTERM
5. Custom port binding
6. Error condition handling
7. Verbose output mode
8. No-remove mode for debugging

## Troubleshooting

### Common Issues

**Port Already in Use**
```bash
lsof -ti:8000 | xargs kill
```

**Docker Image Not Found**
```bash
make docs-docker-build
```

**Server Startup Timeout**
- Check Docker logs with `--verbose`
- Verify mkdocs.yml is valid
- Check port conflicts
- Review volume mount permissions

**Live Edit Not Working**
- Verify volume mounts in docker run command
- Check file permissions
- Review server logs with `--verbose`

## Future Enhancements

Potential improvements:
1. Test WebSocket connections for live reload
2. Test search functionality
3. Test PDF generation in serve mode
4. Test multiple concurrent editors
5. Performance benchmarking
6. Resource usage monitoring
7. Docker Compose integration tests

## Related Documentation

- `scripts/README_DOCKER_SERVE_TEST.md` - Full documentation
- `scripts/DOCKER_SERVE_TEST_QUICK_REF.md` - Quick reference
- `README_DOCKER_MKDOCS.md` - Docker setup documentation
- `scripts/README_MKDOCS_TEST.md` - MkDocs test documentation
- `AGENTS.md` - Repository guidelines and commands

## Maintenance

When updating:
1. Follow shell script compatibility guidelines in AGENTS.md
2. Use centralized logging library
3. Test on both macOS and Linux
4. Verify syntax with `make verify-scripts`
5. Update documentation for new features
6. Keep test descriptions current

## Success Criteria Met

✓ Server starts on port 8000  
✓ Documentation accessible at http://localhost:8000  
✓ curl can retrieve pages  
✓ Volume mounts allow live editing  
✓ File changes trigger rebuild  
✓ No rebuild of Docker image needed  
✓ Server handles concurrent requests  
✓ Server stops cleanly with Ctrl+C (SIGTERM)  
✓ Custom port binding works (e.g., 8080:8000)  
✓ Server logs show no errors  
✓ All tests automated and reproducible  

## Implementation Complete

All requested functionality has been implemented and tested:
- Comprehensive test script with 11 test sections
- Full documentation and quick reference
- Makefile integration
- Logger library integration
- Shell script compatibility
- Automatic cleanup management
- Clear success/failure reporting

The Docker development server test suite is ready for use.
