# Docker Volume Mount and Permissions Test

End-to-end integration test for Docker MkDocs volume mount and permissions functionality.

## Overview

This test script validates that Docker volume mounts work correctly for the MkDocs documentation system, ensuring:

1. **docs/ directory volume mount** - Files can be edited from host while container runs
2. **site/ directory output permissions** - Generated files have correct permissions for host user
3. **mkdocs.yml volume mount** - Configuration updates are reflected in real-time
4. **README.md volume mount** - README files can be edited and updates are detected
5. **README_INSTALL.md volume mount** - Installation documentation can be edited
6. **Non-root user compatibility** - Container user doesn't cause permission conflicts
7. **File deletion without sudo** - Generated files can be deleted from host without elevated privileges

## Test Categories

### Volume Mount Tests
- Create files in docs/ from host while container is running
- Edit files in docs/ from host while container is running
- Modify mkdocs.yml and verify live reload
- Modify README.md and verify updates
- Modify README_INSTALL.md and verify updates

### Permission Tests
- Verify site/ directory has correct owner and group
- Check generated files are readable and writable by host user
- Verify non-root container user doesn't cause conflicts
- Test file deletion without sudo

### Configuration Tests
- Verify Makefile has correct volume mount configuration
- Check Dockerfile uses non-root user
- Validate all required volume mounts are present

## Running the Test

### Quick Start

```bash
# Via Makefile (recommended)
make docs-docker-test-volumes

# Direct execution
./tests/integration/test_docker_volume_permissions_e2e.sh
```

### Prerequisites

1. **Docker installed and running**
   ```bash
   docker --version
   docker info
   ```

2. **Docker image built**
   ```bash
   make docs-docker-build
   ```

3. **Port 8000 available**
   - Test uses port 8000 for development server
   - Ensure no other service is using this port

### Test Options

#### Standard Run
```bash
make docs-docker-test-volumes
```

#### Keep Temporary Files
Useful for debugging test failures:
```bash
./tests/integration/test_docker_volume_permissions_e2e.sh --no-remove
```

#### Verbose Output
See detailed information about each test step:
```bash
./tests/integration/test_docker_volume_permissions_e2e.sh --verbose
```

#### Combined Options
```bash
./tests/integration/test_docker_volume_permissions_e2e.sh --no-remove --verbose
```

## Test Execution Flow

### 1. Prerequisites Check (Test 1)
- Verifies Docker is installed and running
- Checks Docker image exists
- Verifies port 8000 is available
- Backs up original files

### 2. Clean Existing Build (Test 2)
- Removes any existing site/ directory
- Tests deletion without sudo
- Ensures clean test environment

### 3. Start Development Server (Test 3)
- Starts MkDocs server with volume mounts
- Waits for server to be ready (max 30s)
- Registers cleanup handlers

### 4-5. docs/ Volume Mount Tests (Tests 4-5)
- Creates new file in docs/ from host
- Verifies file has correct permissions
- Checks server detects file creation
- Modifies file from host while container runs
- Verifies server detects modifications

### 6-8. Configuration Volume Mount Tests (Tests 6-8)
- Modifies mkdocs.yml from host
- Verifies configuration updates
- Modifies README.md from host
- Modifies README_INSTALL.md from host
- Checks server handles all modifications
- Restores original files after each test

### 9. Build Site for Permission Testing (Test 9)
- Stops development server cleanly
- Builds documentation site
- Creates site/ directory with generated files

### 10. site/ Directory Permission Tests (Test 10)
- Checks site/ directory exists
- Verifies directory is accessible
- Checks owner and group (with macOS compatibility)
- Tests generated files are readable and writable
- Validates multiple sample files

### 11. Non-Root User Tests (Test 11)
- Verifies Dockerfile uses non-root user
- Checks container runs as UID 1000 (not root)
- Tests host user can write to site/ directory
- Ensures no permission conflicts

### 12. Deletion Without sudo Test (Test 12)
- Attempts to delete site/ directory from host
- Verifies deletion succeeds without sudo
- Tests permission fix strategies if deletion fails
- Provides cleanup instructions if needed

### 13. Clean Up Test Files (Test 13)
- Removes test file from docs/
- Verifies cleanup succeeds

### 14. Makefile Configuration Check (Test 14)
- Verifies make docs-docker-serve target exists
- Checks all required volume mounts are configured
- Validates Makefile syntax

## Test Output

### Success Output
```
=== Test Summary ===
[INFO] Total tests: 14
[INFO] Tests passed: 14
[INFO] Tests failed: 0

✓ All Docker volume mount and permissions tests passed successfully!

[INFO] Volume mounts are working correctly:
[INFO]   ✓ docs/ directory allows file editing from host
[INFO]   ✓ site/ directory output has correct permissions
[INFO]   ✓ mkdocs.yml updates configuration in real-time
[INFO]   ✓ README.md volume mount works correctly
[INFO]   ✓ README_INSTALL.md volume mount works correctly
[INFO]   ✓ Non-root user doesn't cause permission conflicts
[INFO]   ✓ Generated files can be deleted without sudo

[INFO] You can now use: make docs-docker-serve
[INFO] Edit files in docs/, mkdocs.yml, README*.md and see live updates
```

### Failure Output
When tests fail, the output includes:
- Detailed error messages
- Relevant log excerpts
- Permission details
- Cleanup instructions

## Tested Scenarios

### Volume Mount Scenarios
1. **File Creation from Host**
   - Create new markdown file in docs/
   - Verify container can access it
   - Check live reload detects it

2. **File Editing from Host**
   - Edit existing files while container runs
   - Verify changes are detected
   - Check server remains stable

3. **Configuration Updates**
   - Modify mkdocs.yml
   - Verify server reloads configuration
   - Check no errors occur

4. **README Updates**
   - Modify README.md and README_INSTALL.md
   - Verify changes are picked up
   - Check documentation rebuilds

### Permission Scenarios
1. **Site Directory Ownership**
   - Check owner matches host user (Linux)
   - Accept Docker Desktop behavior (macOS)
   - Verify directory is accessible

2. **Generated File Permissions**
   - Test index.html is readable
   - Test 404.html is readable
   - Verify files are writable by host

3. **Non-Root User**
   - Container runs as UID 1000
   - No permission conflicts occur
   - Host can modify generated files

4. **File Deletion**
   - Delete site/ without sudo
   - Verify complete removal
   - Test permission fix strategies

## Platform Differences

### Linux Behavior
- Volume mounts preserve host user ownership
- Container user (UID 1000) matches typical user UID
- Files are directly owned by host user
- Deletion works without issues

### macOS Behavior
- Docker Desktop uses osxfs for volume mounts
- File ownership may differ from host UID
- osxfs handles permission mapping automatically
- Deletion typically works but ownership differs

The test accounts for these differences and validates correct behavior on both platforms.

## Common Issues

### Issue: Port 8000 Already in Use
**Error:**
```
✗ Port 8000 is already in use
```

**Solution:**
```bash
# Find and stop the process using port 8000
lsof -ti:8000 | xargs kill -9
# Or use a different port
docker run -p 8080:8000 ...
```

### Issue: Docker Image Not Found
**Error:**
```
✗ Docker image not found: testcase-manager-docs:latest
```

**Solution:**
```bash
# Build the Docker image first
make docs-docker-build
```

### Issue: Permission Denied on site/
**Error:**
```
✗ Failed to delete site/ directory without sudo
```

**Solution:**
```bash
# Fix permissions and delete
chmod -R u+w site/
rm -rf site/

# Or use sudo if absolutely necessary
sudo rm -rf site/
```

### Issue: Server Won't Start
**Error:**
```
✗ Server process died during startup
```

**Solution:**
1. Check server logs in test output
2. Verify mkdocs.yml is valid
3. Ensure all dependencies are installed in image
4. Rebuild Docker image: `make docs-docker-build`

## Integration with CI/CD

### GitLab CI
```yaml
test:docker-volumes:
  stage: test
  script:
    - make docs-docker-build
    - make docs-docker-test-volumes
  only:
    - merge_requests
    - main
```

### GitHub Actions
```yaml
- name: Test Docker Volume Mounts
  run: |
    make docs-docker-build
    make docs-docker-test-volumes
```

## Cleanup

The test automatically cleans up:
- Background server processes
- Temporary directories
- Test files in docs/
- Backup files

If cleanup fails (e.g., test interrupted):
```bash
# Manual cleanup
docker ps -a | grep testcase-manager-docs | awk '{print $1}' | xargs docker rm -f
rm -rf site/
rm -f docs/.test_volume_permissions_*
```

## Related Documentation

- [Docker MkDocs Test](README_MKDOCS_TEST.md) - Main MkDocs test suite
- [Docker Serve Test](README_DOCKER_SERVE_TEST.md) - Development server tests
- [Docker Config Validation](README_DOCKER_CONFIG_VALIDATION_TEST.md) - Configuration tests

## Test File Location

- **Test Script:** `tests/integration/test_docker_volume_permissions_e2e.sh`
- **Makefile Target:** `docs-docker-test-volumes`
- **Logger Library:** `scripts/lib/logger.sh`

## Exit Codes

- **0** - All tests passed
- **1** - One or more tests failed

## Test Duration

Typical execution time: **60-90 seconds**

Breakdown:
- Prerequisites check: 2-3 seconds
- Server startup: 5-10 seconds
- Volume mount tests: 25-30 seconds (includes wait times for live reload)
- Build and permission tests: 15-20 seconds
- Configuration checks: 2-3 seconds
- Cleanup: 5-10 seconds

## Version History

- **v1.0** - Initial implementation
  - Volume mount testing for docs/, mkdocs.yml, README files
  - Permission validation for site/ output
  - Non-root user compatibility
  - File deletion without sudo validation
