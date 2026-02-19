# Docker Cleanup and Resource Management E2E Test

## Overview

This test suite validates Docker cleanup and resource management for the MkDocs documentation system. It ensures that cleanup commands work correctly and that no Docker resources are leaked.

## Test Script

**Location**: `tests/integration/test_docker_cleanup_e2e.sh`

## Running the Tests

### Quick Start

```bash
# Run the full cleanup test suite
make docs-docker-test-cleanup

# Or run directly
./tests/integration/test_docker_cleanup_e2e.sh
```

### Command Line Options

- `--verbose` - Enable verbose output for detailed logging
- `--no-remove` - Keep temporary files after test completion (for debugging)

### Examples

```bash
# Run with verbose output
./tests/integration/test_docker_cleanup_e2e.sh --verbose

# Run without removing temporary files (debugging)
./tests/integration/test_docker_cleanup_e2e.sh --no-remove

# Combine options
./tests/integration/test_docker_cleanup_e2e.sh --verbose --no-remove
```

## What is Tested

### 1. Docker Image Cleanup (`make docs-docker-clean`)

**Test**: Verifies that `make docs-docker-clean` removes the testcase-manager-docs image

**Coverage**:
- Image exists before cleanup
- Command executes successfully
- Image is removed after cleanup
- No errors during removal

### 2. Site Directory Cleanup (`make docs-clean`)

**Test**: Verifies that `make docs-clean` removes the site/ directory

**Coverage**:
- site/ directory exists before cleanup
- Command executes successfully
- site/ directory is completely removed
- No leftover files or directories

### 3. Comprehensive Cleanup (`./scripts/docker-mkdocs.sh clean`)

**Test**: Verifies that the helper script removes both Docker image and generated files

**Coverage**:
- Both image and site/ exist before cleanup
- Script executes successfully
- Docker image is removed
- site/ directory is removed
- Single command cleans all resources

### 4. Automatic Container Cleanup (`--rm` flag)

**Test**: Verifies that stopped containers are cleaned up automatically

**Coverage**:
- Containers use `--rm` flag in all docker run commands
- No stopped containers remain after execution
- Container count before and after is identical
- Automatic cleanup on container exit

### 5. Disk Space Usage

**Test**: Verifies that combined disk usage (image + site) is reasonable (< 1GB)

**Coverage**:
- Docker image size measurement
- site/ directory size measurement
- Combined size calculation
- Threshold validation (< 1GB)
- Size reporting in human-readable format

### 6. Dangling Images

**Test**: Verifies that no dangling images are left after cleanup

**Coverage**:
- Count dangling images before operations
- Count dangling images after cleanup
- Verify no new dangling images created
- List dangling images if found

### 7. Dangling Volumes

**Test**: Verifies that no volumes are created or left dangling

**Coverage**:
- Count volumes before operations
- Count volumes after cleanup
- Check for dangling volumes specifically
- Verify no volume leaks

### 8. Docker System df Verification

**Test**: Uses `docker system df` to verify overall resource usage

**Coverage**:
- Display initial system state
- Display final system state after cleanup
- Compare resource usage
- Verify no unexpected resource growth

### 9. Idempotent Cleanup

**Test**: Verifies cleanup commands can be run multiple times safely

**Coverage**:
- Run cleanup on already-clean state
- Verify no errors occur
- Verify state remains clean
- Test all three cleanup methods

### 10. Comprehensive Verification

**Test**: Final verification of complete cleanup

**Coverage**:
- Image removed
- site/ removed
- No stopped containers
- Clean final state
- Docker system df validation

## Test Flow

```
1. Prerequisites Check
   ├── Docker installed
   ├── Docker daemon running
   └── Helper script exists

2. Initial State Capture
   ├── Count existing images
   ├── Count dangling images
   ├── Count volumes
   └── Capture docker system df

3. Setup Test Resources
   ├── Build Docker image
   ├── Build site/ directory
   └── Measure sizes

4. Disk Space Test
   ├── Measure image size
   ├── Measure site size
   └── Verify total < 1GB

5. Container Auto-Cleanup Test
   ├── Run container with --rm
   ├── Verify execution
   └── Verify auto-removal

6. Site Cleanup Test (make docs-clean)
   ├── Verify site/ exists
   ├── Run cleanup command
   └── Verify site/ removed

7. Image Cleanup Test (make docs-docker-clean)
   ├── Verify image exists
   ├── Run cleanup command
   └── Verify image removed

8. Comprehensive Cleanup Test (docker-mkdocs.sh clean)
   ├── Rebuild image and site
   ├── Run helper script cleanup
   ├── Verify image removed
   └── Verify site/ removed

9. Dangling Resources Check
   ├── Check dangling images
   ├── Check dangling volumes
   └── Verify no leaks

10. Docker System df Verification
    └── Display resource usage

11. Idempotent Cleanup Test
    ├── Run cleanup again
    ├── Verify no errors
    └── Verify state remains clean

12. Final Verification
    ├── Confirm clean state
    └── Display summary
```

## Expected Results

### Success Criteria

All tests should pass with output similar to:

```
=== Test Summary ===
[INFO] Total tests: 15
[INFO] Tests passed: 15
[INFO] Tests failed: 0

✓ All Docker cleanup and resource management tests passed successfully!

[INFO] Docker cleanup commands are working correctly:
[INFO]   • make docs-clean              - Removes site/ directory
[INFO]   • make docs-docker-clean       - Removes Docker image
[INFO]   • ./scripts/docker-mkdocs.sh clean - Removes both
[INFO]   • Automatic container cleanup with --rm flag works
[INFO]   • No resource leaks detected
```

### Key Metrics

- **Docker image size**: Should be < 800 MB
- **site/ directory size**: Should be < 50 MB
- **Combined size**: Should be < 1 GB
- **Container cleanup**: 100% automatic (--rm flag)
- **Dangling images**: 0 new dangling images after cleanup
- **Dangling volumes**: 0 new dangling volumes after cleanup

## Cleanup Commands Reference

### Make Targets

```bash
# Remove site/ directory only
make docs-clean

# Remove Docker image only
make docs-docker-clean

# Both targets can be run in sequence
make docs-clean docs-docker-clean
```

### Helper Script

```bash
# Remove both image and site/ directory
./scripts/docker-mkdocs.sh clean

# This is equivalent to:
# 1. make docs-docker-clean
# 2. make docs-clean
```

## Troubleshooting

### Test Failures

#### Image Not Removed

If the image removal test fails:

```bash
# Manually check image status
docker images testcase-manager-docs:latest

# Manually remove if needed
docker rmi testcase-manager-docs:latest

# Check for running containers
docker ps -a --filter "ancestor=testcase-manager-docs:latest"

# Remove containers if needed
docker rm -f <container_id>
```

#### Site Directory Not Removed

If the site/ directory removal test fails:

```bash
# Check directory permissions
ls -ld site/

# Check directory contents
ls -la site/

# Manually remove if needed
rm -rf site/

# Check for file locks
lsof +D site/ 2>/dev/null
```

#### Dangling Images

If dangling images are detected:

```bash
# List dangling images
docker images -f "dangling=true"

# Remove dangling images
docker image prune -f

# Deep clean (removes all unused images)
docker image prune -a -f
```

#### Disk Space Issues

If disk space validation fails:

```bash
# Check Docker disk usage
docker system df

# Check detailed usage
docker system df -v

# Clean up all Docker resources
docker system prune -a -f --volumes

# Rebuild from scratch
make docs-docker-build
```

### Debugging

Enable verbose mode for detailed output:

```bash
./tests/integration/test_docker_cleanup_e2e.sh --verbose
```

Keep temporary files for inspection:

```bash
./tests/integration/test_docker_cleanup_e2e.sh --no-remove --verbose
```

## Integration with CI/CD

### GitLab CI

Add to `.gitlab-ci.yml`:

```yaml
test:docker-cleanup:
  stage: test
  script:
    - make docs-docker-test-cleanup
  tags:
    - docker
```

### GitHub Actions

Add to `.github/workflows/test.yml`:

```yaml
- name: Test Docker Cleanup
  run: make docs-docker-test-cleanup
```

## Resource Limits

### Recommended Limits

- **Docker image**: < 800 MB (actual ~400-600 MB)
- **site/ directory**: < 50 MB (actual ~10-30 MB)
- **Combined**: < 1 GB (actual ~450-650 MB)

### Monitoring

The test automatically monitors:

1. Initial resource state
2. Peak resource usage
3. Final resource state after cleanup
4. Resource deltas

Output includes:

```
Initial docker system df output:
TYPE            TOTAL     ACTIVE    SIZE      RECLAIMABLE
Images          10        2         5.5GB     4.8GB (87%)
Containers      5         0         100MB     100MB (100%)
Local Volumes   3         1         500MB     200MB (40%)
Build Cache     0         0         0B        0B

Final docker system df output:
TYPE            TOTAL     ACTIVE    SIZE      RECLAIMABLE
Images          9         2         4.9GB     4.2GB (85%)
Containers      5         0         100MB     100MB (100%)
Local Volumes   3         1         500MB     200MB (40%)
Build Cache     0         0         0B        0B
```

## Best Practices

### Regular Cleanup

Run cleanup regularly during development:

```bash
# Daily cleanup routine
make docs-clean docs-docker-clean

# Or use the helper script
./scripts/docker-mkdocs.sh clean
```

### Before Rebuilding

Always clean before rebuilding:

```bash
# Clean first
make docs-clean docs-docker-clean

# Then rebuild
make docs-docker-build
make docs-docker-build-site
```

### Resource Monitoring

Monitor Docker resource usage:

```bash
# Check disk usage
docker system df

# Check detailed usage
docker system df -v

# Monitor in real-time
watch -n 1 docker system df
```

## Test Maintenance

### Adding New Cleanup Tests

When adding new Docker-based features:

1. Add cleanup test to verify resources are freed
2. Check for dangling images/containers/volumes
3. Verify idempotent cleanup
4. Update disk space limits if needed

### Updating Thresholds

If resource requirements change:

1. Update `MAX_TOTAL_SIZE_BYTES` in test script
2. Document new limits in this README
3. Verify all tests pass with new limits
4. Update CI/CD configurations

## Related Documentation

- [Docker MkDocs Setup](README_DOCKER_MKDOCS_TEST.md)
- [Docker HTML Build Tests](README_DOCKER_HTML_BUILD_TEST.md)
- [Docker Volume Tests](tests/integration/README.md)
- [Main Test Documentation](tests/integration/README.md)

## Quick Reference

| Command | Purpose | What it Removes |
|---------|---------|-----------------|
| `make docs-clean` | Remove site directory | `site/` only |
| `make docs-docker-clean` | Remove Docker image | Image only |
| `./scripts/docker-mkdocs.sh clean` | Complete cleanup | Image + site/ |
| `docker system prune -a -f` | Deep Docker cleanup | All unused Docker resources |

## Exit Codes

- `0` - All tests passed
- `1` - One or more tests failed

## Test Statistics

- **Total test sections**: 15
- **Individual test cases**: 40+
- **Average execution time**: 2-5 minutes
- **Docker commands executed**: 30+
- **File operations**: 10+

## Support

For issues or questions:

1. Check the troubleshooting section
2. Run with `--verbose` flag
3. Review Docker logs: `docker logs <container_id>`
4. Check Docker system status: `docker system df -v`
5. Review test output in detail
