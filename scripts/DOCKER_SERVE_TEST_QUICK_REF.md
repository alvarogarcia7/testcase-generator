# Docker Serve Test Quick Reference

Quick reference for Docker MkDocs development server testing.

## Quick Start

```bash
# Build image first
make docs-docker-build

# Run all server tests
make docs-docker-test-serve

# Run with options
./tests/integration/test_docker_serve_e2e.sh --verbose --no-remove
```

## What Gets Tested

| Test | What It Validates |
|------|-------------------|
| **Prerequisites** | Docker installed, image exists, port available |
| **Server Startup** | Server starts in 30s, listens on 0.0.0.0:8000 |
| **Accessibility** | curl can fetch pages, HTML is valid |
| **Multiple Endpoints** | Various docs paths are accessible |
| **Concurrent Requests** | Server handles 5+ simultaneous requests |
| **Live Editing** | Volume mounts work, file changes trigger rebuild |
| **Error Logs** | No errors in server output |
| **Clean Shutdown** | SIGTERM stops server within 10s |
| **Custom Ports** | Can bind to different host ports (e.g., 8080) |
| **Make Command** | `make docs-docker-serve` is configured |

## Server Configuration Tested

```bash
# Standard port binding
docker run --rm -p 8000:8000 \
  -v "$PWD/docs:/docs/docs" \
  -v "$PWD/mkdocs.yml:/docs/mkdocs.yml" \
  -v "$PWD/README.md:/docs/README.md" \
  -v "$PWD/README_INSTALL.md:/docs/README_INSTALL.md" \
  testcase-manager-docs:latest mkdocs serve -a 0.0.0.0:8000

# Custom port binding (8080 -> 8000)
docker run --rm -p 8080:8000 \
  -v "$PWD/docs:/docs/docs" \
  -v "$PWD/mkdocs.yml:/docs/mkdocs.yml" \
  testcase-manager-docs:latest mkdocs serve -a 0.0.0.0:8000
```

## Test Flow

1. **Check prerequisites** - Docker, curl, image, port
2. **Start server** - Launch in background, wait for startup
3. **Test accessibility** - curl http://localhost:8000/
4. **Test endpoints** - Multiple documentation paths
5. **Concurrent requests** - Launch 5 simultaneous requests
6. **Live editing** - Create/modify file, check rebuild
7. **Check logs** - Verify no errors
8. **Shutdown** - Send SIGTERM, wait max 10s
9. **Custom port** - Test 8080:8000 binding
10. **Verify make** - Check Makefile configuration
11. **Display logs** - Show full server output

## Command Options

```bash
# Verbose output (shows detailed logs)
./tests/integration/test_docker_serve_e2e.sh --verbose

# Keep temporary files (for debugging)
./tests/integration/test_docker_serve_e2e.sh --no-remove

# Both options
./tests/integration/test_docker_serve_e2e.sh --verbose --no-remove
```

## Success Criteria

- ✓ All 11 test sections pass
- ✓ Server starts within 30 seconds
- ✓ curl can fetch documentation
- ✓ Concurrent requests all succeed
- ✓ File changes detected and trigger rebuild
- ✓ No errors in logs
- ✓ Server stops cleanly within 10 seconds
- ✓ Custom port binding works

## Common Issues

### Port in Use
```bash
# Find and kill process
lsof -ti:8000 | xargs kill
```

### Image Not Found
```bash
# Build image
make docs-docker-build
```

### Server Won't Start
```bash
# Check logs with verbose
./tests/integration/test_docker_serve_e2e.sh --verbose --no-remove

# Check logs manually
cat /tmp/tmp.XXXXXX/server.log
```

### Live Edit Not Working
- Verify volume mounts
- Check file permissions
- Review server logs for rebuild messages

## Manual Server Testing

```bash
# Start server
make docs-docker-serve

# Test with curl
curl http://localhost:8000/

# Edit a file
echo "Test content" >> docs/test.md

# Watch for rebuild in server logs
# Refresh browser to see changes

# Stop server
# Press Ctrl+C
```

## What Each Test Does

### Test 1: Prerequisites (Quick)
- Checks Docker, curl, image, port

### Test 2: Server Startup (30s timeout)
- Starts server in background
- Waits for "Serving on" message

### Test 3: Accessibility (After 2s wait)
- curl http://localhost:8000/
- Validates HTML response

### Test 4: Multiple Endpoints (Quick)
- Tests /, /404.html, and docs sections

### Test 5: Concurrent Requests (Quick)
- Launches 5 parallel curl requests

### Test 6: Live Editing (10s total)
- Creates test file
- Waits 5s for rebuild
- Modifies file
- Waits 5s for rebuild
- Deletes test file

### Test 7: Log Analysis (Quick)
- Searches logs for errors/warnings

### Test 8: Clean Shutdown (10s timeout)
- Sends SIGTERM
- Waits for process to stop

### Test 9: Custom Port (30s startup + tests)
- Starts server on port 8080
- Tests accessibility
- Stops server

### Test 10: Make Command (Quick)
- Validates Makefile configuration

### Test 11: Display Logs (Quick)
- Shows complete server output

## Expected Runtime

- **Normal run**: ~1-2 minutes
- **With custom port test**: +30 seconds
- **Verbose mode**: Same duration, more output

## Temporary Files Created

```
/tmp/tmp.XXXXXX/
├── server.log              # Main server output
├── custom_server.log       # Custom port server output
├── curl_output.html        # HTTP response
└── concurrent_*.status     # Concurrent test results
```

Plus one temporary test file in docs/:
```
docs/.test_edit_XXXXX       # Created and deleted during live edit test
```

## Integration Points

- **Logger library**: `scripts/lib/logger.sh`
- **Make target**: `docs-docker-test-serve`
- **Docker image**: `testcase-manager-docs:latest`
- **Server command**: `mkdocs serve -a 0.0.0.0:8000`

## Related Commands

```bash
# Build Docker image
make docs-docker-build

# Run server manually
make docs-docker-serve

# Run all Docker tests
make docs-docker-test          # Image build tests
make docs-docker-test-html     # HTML build tests
make docs-docker-test-pdf      # PDF build tests
make docs-docker-test-serve    # Server tests (this)

# Clean up
make docs-docker-clean
```

## Exit Codes

- `0` - All tests passed
- `1` - One or more tests failed

## For More Information

See `scripts/README_DOCKER_SERVE_TEST.md` for complete documentation.
