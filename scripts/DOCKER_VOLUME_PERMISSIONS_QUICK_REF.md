# Docker Volume Mount and Permissions Test - Quick Reference

Fast reference for the Docker MkDocs volume mount and permissions end-to-end test.

## Quick Commands

```bash
# Run test
make docs-docker-test-volumes

# Run with verbose output
./tests/integration/test_docker_volume_permissions_e2e.sh --verbose

# Run keeping temp files for debugging
./tests/integration/test_docker_volume_permissions_e2e.sh --no-remove --verbose
```

## Prerequisites

1. Docker installed and running
2. Docker image built: `make docs-docker-build`
3. Port 8000 available

## What This Test Validates

| Category | What's Tested |
|----------|---------------|
| **Volume Mounts** | docs/, mkdocs.yml, README.md, README_INSTALL.md |
| **Live Editing** | File creation and modification from host while container runs |
| **Permissions** | site/ output has correct permissions for host user |
| **Non-Root User** | Container user (UID 1000) doesn't cause conflicts |
| **File Deletion** | Generated files can be deleted without sudo |
| **Configuration** | Makefile and Dockerfile are correctly configured |

## Test Structure

```
14 Total Tests:
├── Test 1:  Prerequisites check
├── Test 2:  Clean existing site/ directory
├── Test 3:  Start development server
├── Test 4:  Create file in docs/ from host
├── Test 5:  Edit file from host while container runs
├── Test 6:  Modify mkdocs.yml from host
├── Test 7:  Modify README.md from host
├── Test 8:  Modify README_INSTALL.md from host
├── Test 9:  Stop server and build site
├── Test 10: Check site/ permissions
├── Test 11: Verify non-root user
├── Test 12: Delete generated files without sudo
├── Test 13: Clean up test files
└── Test 14: Verify Makefile configuration
```

## Quick Troubleshooting

| Problem | Solution |
|---------|----------|
| Port 8000 in use | `lsof -ti:8000 \| xargs kill -9` |
| Image not found | `make docs-docker-build` |
| Permission denied | `chmod -R u+w site/ && rm -rf site/` |
| Server won't start | Check logs, rebuild image |

## Success Criteria

- ✓ All 14 tests pass
- ✓ Files can be created/edited from host
- ✓ site/ directory has correct permissions
- ✓ Generated files deletable without sudo
- ✓ No permission conflicts

## Execution Time

**60-90 seconds** typical execution time

## Exit Codes

- `0` = All tests passed
- `1` = One or more tests failed

## Related Tests

- `make docs-docker-test` - Main Docker MkDocs tests
- `make docs-docker-test-serve` - Development server tests
- `make docs-docker-test-config` - Configuration validation
- `make docs-compose-test` - Docker Compose tests

## Files Modified During Test

**Temporary Changes (auto-restored):**
- `docs/.test_volume_permissions_$$` (created and deleted)
- `mkdocs.yml` (modified and restored)
- `README.md` (modified and restored)
- `README_INSTALL.md` (modified and restored)

**Build Artifacts (cleaned up):**
- `site/` (created and deleted)

## Verbose Mode Details

Shows additional information:
- Server startup logs
- File permission details
- Volume mount configuration
- Individual file test results
- Makefile docker run commands

## Platform Notes

### Linux
- Volume mounts preserve host user ownership
- Container UID 1000 typically matches host user
- Direct file ownership, no issues expected

### macOS
- Docker Desktop uses osxfs
- File ownership may differ from host UID
- osxfs handles permissions automatically
- Tests account for this behavior

## Test Script Location

- **Script:** `tests/integration/test_docker_volume_permissions_e2e.sh`
- **Documentation:** `scripts/README_DOCKER_VOLUME_PERMISSIONS_TEST.md`
- **Makefile:** `make docs-docker-test-volumes`

## Manual Cleanup

If test fails and leaves artifacts:

```bash
# Stop any running containers
docker ps -a | grep testcase-manager-docs | awk '{print $1}' | xargs docker rm -f

# Remove site directory
rm -rf site/

# Remove test files
rm -f docs/.test_volume_permissions_*

# Restore original files from git
git checkout mkdocs.yml README.md README_INSTALL.md
```

## Common Test Patterns

### Check Volume Mount Works
```bash
# Start server
make docs-docker-serve

# In another terminal, edit a file
echo "# Test" >> docs/test.md

# Check if server detects change in first terminal logs
```

### Check Permissions
```bash
# Build site
make docs-docker-build-site

# Check ownership
ls -la site/

# Try to delete
rm -rf site/  # Should work without sudo
```

### Check Non-Root User
```bash
# Check container user
docker run --rm testcase-manager-docs:latest id -u
# Should output: 1000 (not 0)
```

## Integration with Workflow

This test should be run:
- After building Docker image
- Before deploying documentation
- In CI/CD pipeline for pull requests
- When modifying volume mount configuration
- When updating Dockerfile user configuration

## Test Output Example

```
=== Docker MkDocs Volume Mount and Permissions Test ===
[INFO] Project root: /path/to/project
[INFO] Docker image: testcase-manager-docs:latest
[INFO] Server URL: http://localhost:8000
[INFO] Host user: 501:20

=== Test 1: Checking Prerequisites ===
✓ Docker is installed
✓ Docker daemon is running
✓ Docker image exists: testcase-manager-docs:latest
✓ Port 8000 is available
✓ Original files backed up

=== Test 2: Clean Existing site/ Directory ===
✓ No existing site/ directory to clean
✓ site/ directory is clean

[... additional test output ...]

=== Test Summary ===
[INFO] Total tests: 14
[INFO] Tests passed: 14
[INFO] Tests failed: 0

✓ All Docker volume mount and permissions tests passed successfully!
```

## Full Documentation

For complete details, see: [README_DOCKER_VOLUME_PERMISSIONS_TEST.md](README_DOCKER_VOLUME_PERMISSIONS_TEST.md)
