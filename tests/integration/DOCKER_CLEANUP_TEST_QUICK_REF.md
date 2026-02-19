# Docker Cleanup Test - Quick Reference

## Run Tests

```bash
# Full test suite
make docs-docker-test-cleanup

# With verbose output
./tests/integration/test_docker_cleanup_e2e.sh --verbose

# Keep temp files (debugging)
./tests/integration/test_docker_cleanup_e2e.sh --no-remove
```

## What's Tested

1. ✅ `make docs-docker-clean` removes image
2. ✅ `make docs-clean` removes site/ directory
3. ✅ `./scripts/docker-mkdocs.sh clean` removes both
4. ✅ Containers auto-cleanup with `--rm` flag
5. ✅ Disk space < 1GB (image + site)
6. ✅ No dangling images after cleanup
7. ✅ No dangling volumes after cleanup
8. ✅ `docker system df` verification
9. ✅ Idempotent cleanup (can run multiple times)
10. ✅ Complete cleanup verification

## Cleanup Commands

```bash
# Remove site/ directory
make docs-clean

# Remove Docker image
make docs-docker-clean

# Remove everything (recommended)
./scripts/docker-mkdocs.sh clean
```

## Quick Checks

```bash
# Check image exists
docker images testcase-manager-docs:latest

# Check site directory
ls -ld site/

# Check disk usage
docker system df

# Check dangling images
docker images -f "dangling=true"

# Check dangling volumes
docker volume ls -f "dangling=true"
```

## Troubleshooting

### Image won't remove

```bash
# Force remove
docker rmi -f testcase-manager-docs:latest

# Check for running containers
docker ps -a --filter "ancestor=testcase-manager-docs:latest"

# Remove containers first
docker rm -f $(docker ps -a --filter "ancestor=testcase-manager-docs:latest" -q)
```

### Site directory won't remove

```bash
# Check permissions
ls -ld site/

# Force remove
sudo rm -rf site/

# Check for locks
lsof +D site/ 2>/dev/null
```

### Disk space issues

```bash
# Clean Docker system
docker system prune -a -f --volumes

# Rebuild from scratch
make docs-docker-build
```

## Resource Limits

- **Docker image**: < 800 MB (typical: 400-600 MB)
- **site/ directory**: < 50 MB (typical: 10-30 MB)
- **Combined total**: < 1 GB (typical: 450-650 MB)

## Test Execution Flow

```
Prerequisites → Initial State → Build Resources → 
Test Disk Usage → Test Container Cleanup → 
Test Site Cleanup → Test Image Cleanup → 
Test Script Cleanup → Check Dangling Resources → 
Verify docker system df → Test Idempotent Cleanup → 
Final Verification → Report
```

## Expected Output (Success)

```
=== Test Summary ===
[INFO] Total tests: 15
[INFO] Tests passed: 15
[INFO] Tests failed: 0

✓ All Docker cleanup and resource management tests passed successfully!
```

## Common Issues

| Issue | Solution |
|-------|----------|
| Image still exists | Run `docker rmi -f testcase-manager-docs:latest` |
| Site not removed | Run `rm -rf site/` |
| Dangling images | Run `docker image prune -f` |
| Disk space high | Run `docker system prune -a -f` |
| Test fails repeatedly | Clean everything and rebuild |

## File Locations

- **Test script**: `tests/integration/test_docker_cleanup_e2e.sh`
- **Documentation**: `tests/integration/README_DOCKER_CLEANUP_TEST.md`
- **Helper script**: `scripts/docker-mkdocs.sh`
- **Makefile target**: `docs-docker-test-cleanup`

## Integration

### CI/CD

```yaml
# GitLab CI
test:docker-cleanup:
  script:
    - make docs-docker-test-cleanup

# GitHub Actions
- name: Test Docker Cleanup
  run: make docs-docker-test-cleanup
```

## Related Tests

- `make docs-docker-test` - Docker MkDocs setup
- `make docs-docker-test-html` - HTML build
- `make docs-docker-test-volumes` - Volume permissions

## Quick Debug

```bash
# Run with full verbosity and keep temp files
VERBOSE=1 ./tests/integration/test_docker_cleanup_e2e.sh --no-remove --verbose

# Check what's left
docker images testcase-manager-docs:latest
ls -la site/
docker system df -v
```

## Success Indicators

- ✅ All 15 tests pass
- ✅ Exit code 0
- ✅ No images remain
- ✅ No site/ directory
- ✅ No dangling resources
- ✅ Docker system df shows cleanup

## Time to Complete

- **Average**: 2-3 minutes
- **With builds**: 4-5 minutes
- **First run**: 5-8 minutes

## Prerequisites

- Docker installed and running
- Helper script exists (`scripts/docker-mkdocs.sh`)
- Sufficient disk space (2GB free recommended)
- No other processes using the image/containers
