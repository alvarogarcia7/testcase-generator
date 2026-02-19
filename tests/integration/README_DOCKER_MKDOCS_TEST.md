# Docker MkDocs End-to-End Test

## Overview

The `test_docker_mkdocs_e2e.sh` script provides comprehensive end-to-end testing for the Docker-based MkDocs documentation setup. It validates the Dockerfile configuration, builds the Docker image, and verifies all dependencies and configurations are correct.

## Quick Start

```bash
# Run the full test suite
make docs-docker-test

# Or run directly
./tests/integration/test_docker_mkdocs_e2e.sh

# Keep temporary files for debugging
./tests/integration/test_docker_mkdocs_e2e.sh --no-remove
```

## What It Tests

### 1. Prerequisites Check
- Docker installation and daemon status
- Dockerfile.mkdocs file existence
- Required tools availability

### 2. Dockerfile Syntax Validation
- Valid Dockerfile syntax
- Best practices compliance:
  - Specific Python version (not `latest`)
  - WORKDIR directive usage
  - Cleanup of apt lists
  - `--no-cache-dir` for pip installations
  - Non-root user creation
  - Proper USER directive
  - EXPOSE port 8000
  - ENABLE_PDF_EXPORT environment variable

### 3. Docker Image Build
- Successful build via `make docs-docker-build`
- Image creation and tagging as `testcase-manager-docs:latest`

### 4. Image Verification
- Image exists in Docker registry
- Image is properly tagged
- Image metadata is correct

### 5. Image Size Validation
- Image size is under 1GB limit
- Reports actual size in MB and GB
- Warns if size is excessive

### 6. Python Dependencies
Verifies installation of:
- `mkdocs` - Core documentation generator
- `mkdocs-material` - Material theme
- `mkdocs-with-pdf` - PDF export plugin
- Related packages (markdown, pymdown-extensions)

### 7. System Dependencies for PDF
Verifies installation of:
- `libcairo2` - Cairo graphics library
- `libpango-1.0-0` - Pango text rendering
- `libffi-dev` - Foreign Function Interface library
- Library accessibility via ldconfig

### 8. Non-Root User Configuration
- Container runs as user `mkdocs`
- User ID is 1000 (for host compatibility)
- Home directory is `/home/mkdocs`
- `/docs` directory ownership
- Proper permissions

### 9. Environment Variables
- `ENABLE_PDF_EXPORT` defaults to 0
- Environment variables can be overridden
- Docker environment is properly configured

### 10. Docker Inspect Verification
- Exposed ports configuration
- Working directory (/docs)
- Default command (mkdocs build)
- User configuration
- Image labels and metadata

### 11. Functionality Testing
- Creates minimal test documentation
- Builds documentation inside container
- Verifies HTML output generation
- Validates generated content

### 12. Image Information Display
- Lists all images with the name
- Shows detailed inspect output
- Displays configuration summary

## Test Output

The test provides detailed, color-coded output:

- ✓ **Green checkmark**: Test passed
- ✗ **Red X**: Test failed
- ℹ **Blue info**: Informational message
- **[WARNING]**: Warning message
- **=== Section ===**: Test section header

## Exit Codes

- `0`: All tests passed successfully
- `1`: One or more tests failed

## Requirements

### System Requirements
- Docker installed and running
- Bash 3.2+ (macOS compatible)
- Standard Unix utilities (grep, sed, awk)

### Build Requirements
- Dockerfile.mkdocs in project root
- requirements.txt with Python dependencies
- mkdocs.yml configuration file

## Examples

### Full Test with Clean Build
```bash
# Clean existing image first
make docs-docker-clean

# Run full test suite
make docs-docker-test
```

### Debug Failed Tests
```bash
# Keep temporary files for inspection
./tests/integration/test_docker_mkdocs_e2e.sh --no-remove

# Check the temporary directory
# (path is shown in test output)
```

### Manual Image Inspection
```bash
# After test runs, inspect the image
docker images testcase-manager-docs:latest
docker inspect testcase-manager-docs:latest

# Run a shell in the container
docker run -it --rm testcase-manager-docs:latest bash
```

## Integration with CI/CD

The test is designed to work in CI/CD pipelines:

```yaml
test-docker-docs:
  script:
    - make docs-docker-test
  artifacts:
    when: on_failure
    paths:
      - docker-build-log.txt
```

## Troubleshooting

### Test Fails on Image Size
- Review Dockerfile for unnecessary packages
- Consider using multi-stage builds
- Remove unnecessary files in the same RUN command
- Use `.dockerignore` to exclude build artifacts

### Python Dependencies Not Found
- Verify `requirements.txt` is complete
- Check pip installation in Dockerfile
- Ensure `--no-cache-dir` is used

### PDF Dependencies Missing
- Verify system packages in Dockerfile
- Check apt-get install command
- Ensure library cleanup doesn't remove required files

### Permission Issues
- Verify `useradd` command in Dockerfile
- Check `chown` command for `/docs` directory
- Ensure `USER mkdocs` directive is present

### Build Fails
- Check Docker daemon is running
- Verify network connectivity for package downloads
- Review Dockerfile syntax
- Check base image availability

## Related Documentation

- [Docker MkDocs Setup](../../README_DOCKER_MKDOCS.md)
- [MkDocs Test Guide](../../scripts/README_MKDOCS_TEST.md)
- [Dockerfile Best Practices](https://docs.docker.com/develop/develop-images/dockerfile_best-practices/)
- [AGENTS.md](../../AGENTS.md) - Project commands reference

## Test Maintenance

### Adding New Tests
1. Add new test section in the script
2. Use the logger library functions (pass, fail, info, section)
3. Update test counters (TESTS_PASSED, TESTS_FAILED)
4. Document new tests in this README

### Updating for New Dependencies
1. Update Python dependency checks in Test 6
2. Update system dependency checks in Test 7
3. Update Dockerfile best practices checks in Test 2
4. Update this README with new validation steps

### Version Compatibility
The test is designed to be:
- **Platform-agnostic**: Works on Linux and macOS
- **Docker-version-agnostic**: Uses standard Docker commands
- **Shell-compatible**: Uses bash 3.2+ compatible syntax

## License

This test script follows the same license as the main project.
