# Docker MkDocs Helper Script Test - Quick Reference

## Quick Start

```bash
# Run all tests
./tests/integration/test_docker_mkdocs_helper_e2e.sh

# Run with verbose output
./tests/integration/test_docker_mkdocs_helper_e2e.sh --verbose

# Debug mode (preserve temp files)
./tests/integration/test_docker_mkdocs_helper_e2e.sh --verbose --no-remove
```

## What It Tests

| Test # | Category | Description |
|--------|----------|-------------|
| 1 | Prerequisites | Docker installed and running |
| 2-4 | Help/Usage | --help, -h, help command display |
| 5 | Error Handling | Unknown command handling |
| 6 | Setup | Clean existing resources |
| 7-8 | Build | Image build and verbose mode |
| 9 | Status | Status command output |
| 10 | Build Site | Static HTML generation |
| 11 | Build PDF | PDF generation |
| 12 | Serve | Development server (default port) |
| 13 | Serve Port | Development server (custom port) |
| 14 | Compose | All Docker Compose commands |
| 15 | Clean | Resource cleanup |
| 16 | Error Handling | Missing image handling |
| 17 | Usage | No arguments behavior |
| 18 | Rebuild | Final image rebuild |
| 19 | Final Status | Status after rebuild |
| 20 | Sequences | Multiple commands |

## Test Coverage

### Commands Tested
- ✅ `--help`, `-h`, `help`
- ✅ `build` (with `--verbose`)
- ✅ `serve` (default port)
- ✅ `serve --port 8080` (custom port)
- ✅ `build-site`
- ✅ `build-pdf`
- ✅ `status`
- ✅ `clean`
- ✅ `compose-build`
- ✅ `compose-pdf`
- ✅ `compose-up`
- ✅ `compose-down`

### Features Tested
- ✅ Colored output
- ✅ Proper logging
- ✅ Error handling
- ✅ Verbose mode
- ✅ Custom ports
- ✅ Image creation
- ✅ Site generation
- ✅ PDF generation
- ✅ Server startup
- ✅ Background processes
- ✅ Cleanup

## Expected Duration

| Phase | Time (First Run) | Time (Subsequent) |
|-------|------------------|-------------------|
| Prerequisites | < 1 min | < 1 min |
| Build | 5-10 min | 1-2 min |
| Site Tests | 2-3 min | 1-2 min |
| Server Tests | 2-3 min | 1-2 min |
| Compose Tests | 2-3 min | 1-2 min |
| Cleanup | < 1 min | < 1 min |
| **Total** | **10-20 min** | **5-10 min** |

## Prerequisites

```bash
# Check Docker
docker info

# Check Docker Compose
docker-compose --version

# Check curl
which curl

# Check script exists
ls -la scripts/docker-mkdocs.sh

# Make executable if needed
chmod +x scripts/docker-mkdocs.sh
chmod +x tests/integration/test_docker_mkdocs_helper_e2e.sh
```

## Common Commands

### Test Helper Script
```bash
# Show help
./scripts/docker-mkdocs.sh --help

# Build image
./scripts/docker-mkdocs.sh build

# Check status
./scripts/docker-mkdocs.sh status

# Start server
./scripts/docker-mkdocs.sh serve

# Build site
./scripts/docker-mkdocs.sh build-site

# Clean up
./scripts/docker-mkdocs.sh clean
```

### Debug Test Failures
```bash
# Run with verbose output
./tests/integration/test_docker_mkdocs_helper_e2e.sh --verbose

# Keep temp files
./tests/integration/test_docker_mkdocs_helper_e2e.sh --no-remove

# Check Docker status
docker ps -a
docker images

# Check logs
docker logs <container-id>
docker-compose -f docker-compose.mkdocs.yml logs
```

### Clean Up
```bash
# Stop all containers
docker-compose -f docker-compose.mkdocs.yml down
docker ps -q --filter "ancestor=testcase-manager-docs:latest" | xargs -r docker stop

# Remove image
docker rmi testcase-manager-docs:latest

# Remove site
rm -rf site/

# Kill processes on ports
lsof -ti:8000 | xargs -r kill -9
lsof -ti:8080 | xargs -r kill -9
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | All tests passed ✓ |
| 1 | One or more tests failed ✗ |

## Output Examples

### Success
```
✓ All docker-mkdocs.sh helper script tests passed successfully!

[INFO] docker-mkdocs.sh is working correctly:
  ✓ --help displays usage correctly
  ✓ build command builds Docker image with proper logging
  ✓ serve command starts server with colored output
  ...
```

### Failure
```
✗ Some docker-mkdocs.sh helper script tests failed!

[ERROR] 3 test(s) failed
[INFO] Please review the output above and fix the issues
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Port in use | `lsof -ti:8000 \| xargs kill -9` |
| Docker not running | Start Docker Desktop / `systemctl start docker` |
| Permission denied | Add user to docker group: `sudo usermod -aG docker $USER` |
| Image build fails | Check `Dockerfile.mkdocs` exists and is valid |
| Compose not available | Install docker-compose or skip those tests |

## Test Results Location

Temporary files created in: `/tmp/tmp.XXXXXXXXXX/`

Files include:
- `help_output.txt` - Help command output
- `build_output.txt` - Build command output
- `serve_output.txt` - Serve command output
- `status_output.txt` - Status command output
- `clean_output.txt` - Clean command output
- And more...

Use `--no-remove` to preserve these files.

## Integration

### CI/CD
```yaml
# GitLab CI
test:docker-helper:
  script:
    - ./tests/integration/test_docker_mkdocs_helper_e2e.sh

# GitHub Actions
- run: ./tests/integration/test_docker_mkdocs_helper_e2e.sh
```

### Makefile
```makefile
test-docker-helper:
	./tests/integration/test_docker_mkdocs_helper_e2e.sh
```

## Related Tests

- `test_docker_mkdocs_e2e.sh` - Tests Docker image itself
- `test_docker_compose_mkdocs_e2e.sh` - Tests Docker Compose workflow
- `test_docker_serve_e2e.sh` - Tests development server

## Quick Checks

### Before Running Tests
```bash
# All green? Ready to test!
docker info && \
docker-compose --version && \
test -x scripts/docker-mkdocs.sh && \
echo "✓ Ready to run tests"
```

### After Running Tests
```bash
# Verify image exists
docker images testcase-manager-docs:latest

# Verify site was built
ls -la site/index.html

# Verify PDF was generated
ls -la site/pdf/*.pdf
```

## Test Pattern

Each test follows this pattern:

1. **Section Header**: `section "Test N: Description"`
2. **Log Action**: `log_info "Testing: command"`
3. **Execute Test**: Run command and capture output
4. **Verify Result**: Check output/state
5. **Update Counter**: Increment pass/fail counter
6. **Log Details**: Show verbose output if enabled

## Key Metrics

- **20 test sections**
- **40+ individual checks**
- **12 commands tested**
- **3 server configurations tested**
- **4 Docker Compose commands tested**

## Full Documentation

For complete documentation, see:
`tests/integration/README_DOCKER_MKDOCS_HELPER_TEST.md`
