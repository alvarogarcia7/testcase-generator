# Docker Development Server Tests

This document describes the end-to-end integration tests for the Docker MkDocs development server.

## Overview

The Docker development server test suite (`test_docker_serve_e2e.sh`) validates that the MkDocs development server runs correctly inside a Docker container with all required functionality.

## Test File Location

```
tests/integration/test_docker_serve_e2e.sh
```

## Running the Tests

### Quick Start

```bash
# Run all Docker server tests
make docs-docker-test-serve

# Or run directly
./tests/integration/test_docker_serve_e2e.sh
```

### Test Options

```bash
# Run with verbose output
./tests/integration/test_docker_serve_e2e.sh --verbose

# Keep temporary files for debugging
./tests/integration/test_docker_serve_e2e.sh --no-remove

# Combine options
./tests/integration/test_docker_serve_e2e.sh --verbose --no-remove
```

## Prerequisites

Before running the tests, ensure:

1. **Docker is installed and running**
   ```bash
   docker --version
   docker info
   ```

2. **Docker image is built**
   ```bash
   make docs-docker-build
   ```

3. **curl is available** (for HTTP testing)
   ```bash
   curl --version
   ```

4. **Port 8000 is available** (not in use by another process)

## Test Coverage

The test suite validates the following functionality:

### Test 1: Prerequisites Check
- Docker installation and daemon status
- curl availability for HTTP testing
- Docker image existence (`testcase-manager-docs:latest`)
- Port 8000 availability

### Test 2: Server Startup
- Server starts successfully in Docker container
- Server process runs in background
- Server startup completes within timeout (30s)
- Server logs indicate successful startup
- Server listens on 0.0.0.0:8000 inside container

### Test 3: Documentation Accessibility
- HTTP GET request to `http://localhost:8000/` succeeds
- Response contains valid HTML
- Response has substantial content (>100 bytes)
- Material theme is detected in response

### Test 4: Multiple Endpoints
- Tests various documentation endpoints
- Validates at least one endpoint is accessible
- Tests both existing and non-existing pages

### Test 5: Concurrent Request Handling
- Launches multiple concurrent requests (5 by default)
- Validates server handles concurrent load
- Confirms all or most requests succeed

### Test 6: Live Editing (Volume Mounts)
- Creates temporary markdown file in docs/
- Verifies file is accessible from container
- Tests server detects file changes
- Validates server triggers rebuild
- Confirms server remains stable after edits
- Cleans up test files

### Test 7: Server Log Analysis
- Checks server logs for errors
- Counts warnings in logs
- Validates clean operation without errors

### Test 8: Clean Shutdown
- Sends SIGTERM to server process
- Validates server stops within timeout (10s)
- Confirms graceful shutdown

### Test 9: Custom Port Binding
- Tests custom port mapping (8080:8000)
- Validates server starts on custom port
- Tests accessibility on custom port
- Confirms clean shutdown

### Test 10: Make Command Verification
- Validates `make docs-docker-serve` configuration
- Documents expected behavior

### Test 11: Complete Log Display
- Displays full server logs for review

## Expected Output

Successful test run:

```
=== Docker MkDocs Development Server End-to-End Test ===
[INFO] Project root: /path/to/project
[INFO] Docker image: testcase-manager-docs:latest
[INFO] Server URL: http://localhost:8000

=== Test 1: Checking Prerequisites ===
✓ Docker is installed
✓ Docker daemon is running
✓ curl is installed
✓ Docker image exists: testcase-manager-docs:latest
✓ Port 8000 is available

=== Test 2: Start MkDocs Development Server ===
[INFO] Starting MkDocs development server...
[INFO] Server started with PID: 12345
✓ Server process started
[INFO] Waiting for server to start (max 30s)...
✓ Server started successfully after 3s

=== Test 3: Test Documentation Accessibility with curl ===
✓ Successfully retrieved index page with curl
✓ Response has substantial content (54321 bytes)
✓ Response contains HTML
✓ Material theme detected in response

=== Test 4: Test Multiple Documentation Endpoints ===
[INFO] Accessible endpoints: 3/4
✓ At least one endpoint is accessible

=== Test 5: Test Concurrent Request Handling ===
[INFO] Successful concurrent requests: 5/5
✓ All concurrent requests succeeded

=== Test 6: Test Live Editing (Volume Mount) ===
✓ Test file created successfully
✓ Server detected file change and triggered rebuild
✓ Server still running after file modifications

=== Test 7: Verify Server Logs Show No Errors ===
[INFO] Errors found: 0
[INFO] Warnings found: 0
✓ No errors found in server logs
✓ No warnings found in server logs

=== Test 8: Test Clean Server Shutdown (SIGTERM) ===
✓ Server stopped cleanly after 2s

=== Test 9: Test Custom Port Binding ===
✓ Custom port server started successfully after 3s
✓ Server accessible on custom port 8080
✓ Custom port server stopped successfully

=== Test 10: Test 'make docs-docker-serve' Command ===
✓ make docs-docker-serve command is properly configured in Makefile

=== Test Summary ===
[INFO] Total tests: 11
[INFO] Tests passed: 11
[INFO] Tests failed: 0

✓ All Docker development server tests passed successfully!

[INFO] The Docker MkDocs development server is working correctly:
[INFO]   ✓ Server starts on port 8000
[INFO]   ✓ Documentation is accessible at http://localhost:8000
[INFO]   ✓ Volume mounts allow live editing without rebuild
[INFO]   ✓ Server handles concurrent requests
[INFO]   ✓ Server stops cleanly with SIGTERM
[INFO]   ✓ Custom port binding works
[INFO]   ✓ Server logs show no errors

[INFO] You can now use: make docs-docker-serve
```

## Common Issues and Solutions

### Port Already in Use

**Issue**: Port 8000 is already in use

**Solution**:
```bash
# Find process using port 8000
lsof -ti:8000

# Kill the process
kill $(lsof -ti:8000)

# Or use a different port
docker run --rm -p 8080:8000 ... testcase-manager-docs:latest mkdocs serve -a 0.0.0.0:8000
```

### Docker Image Not Found

**Issue**: `testcase-manager-docs:latest` image not found

**Solution**:
```bash
# Build the Docker image first
make docs-docker-build
```

### Server Startup Timeout

**Issue**: Server does not start within 30 seconds

**Solution**:
1. Check Docker container logs
2. Verify volume mounts are correct
3. Ensure mkdocs.yml is valid
4. Check for port conflicts

### Live Edit Not Detected

**Issue**: File changes not triggering rebuild

**Solution**:
1. Verify volume mounts in docker run command
2. Check file permissions
3. Ensure docs/ directory is properly mounted
4. Review server logs for rebuild messages

## Server Configuration

The test validates the following server configuration:

### Volume Mounts
```
-v "$PROJECT_ROOT/docs:/docs/docs"
-v "$PROJECT_ROOT/mkdocs.yml:/docs/mkdocs.yml"
-v "$PROJECT_ROOT/README.md:/docs/README.md"
-v "$PROJECT_ROOT/README_INSTALL.md:/docs/README_INSTALL.md"
```

### Port Binding
```
-p 8000:8000  # Default
-p 8080:8000  # Custom port example
```

### Server Command
```
mkdocs serve -a 0.0.0.0:8000
```

## Manual Testing

You can manually test the server:

### Start Server
```bash
make docs-docker-serve
```

### Access Documentation
Open browser to: http://localhost:8000

### Test Live Editing
1. Edit a file in `docs/`
2. Save the file
3. Observe automatic rebuild in server logs
4. Refresh browser to see changes

### Test with curl
```bash
# Get index page
curl http://localhost:8000/

# Test specific endpoint
curl http://localhost:8000/getting-started/

# Save response to file
curl http://localhost:8000/ -o index.html
```

### Stop Server
Press `Ctrl+C` in the terminal running the server

## Integration with CI/CD

The test is designed to run in CI/CD environments:

```yaml
# GitLab CI example
test-docker-serve:
  stage: test
  script:
    - make docs-docker-build
    - make docs-docker-test-serve
  tags:
    - docker
```

## Troubleshooting

### Enable Verbose Output

```bash
./tests/integration/test_docker_serve_e2e.sh --verbose
```

This will show:
- Detailed server logs
- Response content samples
- File system operations
- Timing information

### Keep Temporary Files

```bash
./tests/integration/test_docker_serve_e2e.sh --no-remove
```

This preserves:
- Server logs: `$TEMP_DIR/server.log`
- curl output: `$TEMP_DIR/curl_output.html`
- Test status files: `$TEMP_DIR/concurrent_*.status`

### Check Server Logs

Server logs are saved to temporary directory during tests:

```bash
# Run with --no-remove to keep logs
./tests/integration/test_docker_serve_e2e.sh --no-remove

# Logs will be in displayed temp directory
cat /tmp/tmp.XXXXXX/server.log
```

## Related Documentation

- **Docker Setup**: See `README_DOCKER_MKDOCS.md`
- **MkDocs Tests**: See `README_MKDOCS_TEST.md`
- **Docker Image Tests**: See `test_docker_mkdocs_e2e.sh`
- **HTML Build Tests**: See `test_docker_html_build_e2e.sh`

## Test Maintenance

When updating the tests:

1. **Follow shell script compatibility guidelines** in `AGENTS.md`
2. **Use the centralized logging library** from `scripts/lib/logger.sh`
3. **Test on both macOS and Linux** for portability
4. **Verify syntax** with `make verify-scripts`
5. **Update documentation** when adding new tests

## Contributing

When adding new test cases:

1. Follow the existing test structure
2. Use descriptive test names and section headers
3. Provide clear pass/fail messages
4. Update this README with new test coverage
5. Test on both macOS and Linux if possible
