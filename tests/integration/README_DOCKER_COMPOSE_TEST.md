# Docker Compose MkDocs Workflow E2E Test

## Overview

The `test_docker_compose_mkdocs_e2e.sh` script provides comprehensive end-to-end testing for the Docker Compose workflow of the MkDocs documentation system. It validates that all three Docker Compose services work correctly and share the same Docker image.

## What It Tests

### 1. Docker Compose Configuration Validation
- Validates `docker-compose.mkdocs.yml` syntax using `docker-compose config`
- Verifies all three services are properly defined:
  - `mkdocs` - Development server with live reload
  - `mkdocs-build` - Static HTML site generation
  - `mkdocs-build-pdf` - PDF documentation generation
- Confirms all services use the same image: `testcase-manager-docs:latest`

### 2. Volume Mount Verification
- Tests that all services have correct volume mounts:
  - `./docs:/docs/docs` - Documentation source files
  - `./mkdocs.yml:/docs/mkdocs.yml` - Configuration file
  - `./site:/docs/site` - Generated site output
  - `./README.md:/docs/README.md` - Main README
  - `./README_INSTALL.md:/docs/README_INSTALL.md` - Install instructions
- Verifies volume mounts are writable and readable from host
- Tests live editing capability with volume mounts

### 3. HTML Site Generation Service (mkdocs-build)
- Runs `make docs-compose-build-site`
- Verifies `site/` directory is created
- Checks `site/index.html` exists with proper content
- Validates HTML structure and Material theme assets
- Tests multiple sequential builds
- Verifies file permissions and ownership

### 4. PDF Generation Service (mkdocs-build-pdf)
- Runs `make docs-compose-build-pdf`
- Verifies `ENABLE_PDF_EXPORT=1` environment variable is set
- Checks PDF is generated at `site/pdf/testcase-manager-documentation.pdf`
- Validates PDF file format and size
- Tests PDF accessibility from host
- Verifies PDF contains expected content

### 5. Development Server Service (mkdocs)
- Starts server with `docker-compose up -d mkdocs`
- Verifies server starts and becomes accessible on port 8000
- Tests HTTP requests to `http://localhost:8000/`
- Validates live reload by creating test files
- Checks service logs for errors
- Tests concurrent request handling

### 6. Service Lifecycle Management
- Tests `make docs-compose-up` starts the mkdocs service
- Tests `make docs-compose-down` stops all services cleanly
- Verifies no containers remain running after shutdown
- Tests service restart and cleanup

### 7. Complete Workflow Integration
- Runs all compose targets in sequence:
  1. Build HTML site
  2. Clean and rebuild with PDF
  3. Start development server
  4. Stop all services
- Verifies outputs at each stage

## Usage

### Basic Test Run

```bash
# Run the complete test suite
./tests/integration/test_docker_compose_mkdocs_e2e.sh

# Or use the make target
make docs-compose-test
```

### With Verbose Output

```bash
# See detailed output including service logs
./tests/integration/test_docker_compose_mkdocs_e2e.sh --verbose
```

### Keep Temporary Files

```bash
# Don't remove temporary test files (useful for debugging)
./tests/integration/test_docker_compose_mkdocs_e2e.sh --no-remove
```

## Prerequisites

The test checks for these prerequisites automatically:

1. **Docker** - Must be installed and daemon running
2. **docker-compose** - Must be installed
3. **curl** - For testing HTTP requests
4. **Docker Image** - `testcase-manager-docs:latest` must exist
   - If missing, run: `make docs-docker-build`

## Test Structure

The test is organized into 20 comprehensive test sections:

1. **Prerequisites Check** - Verify tools and environment
2. **Syntax Validation** - Validate docker-compose.yml syntax
3. **Service Definitions** - Check all services are defined
4. **Volume Configuration** - Verify volume mounts
5. **Image Existence** - Ensure Docker image exists
6. **Cleanup** - Remove existing site/ directory
7. **Container Cleanup** - Stop any running containers
8. **HTML Build Test** - Test mkdocs-build service
9. **HTML Volume Mounts** - Verify volume accessibility
10. **PDF Cleanup** - Clean for PDF test
11. **PDF Build Test** - Test mkdocs-build-pdf service
12. **PDF Volume Mounts** - Verify PDF accessibility
13. **Server Startup** - Test mkdocs service startup
14. **Server Accessibility** - Test HTTP requests
15. **Live Reload** - Test volume-based live editing
16. **Service Logs** - Check for errors
17. **Service Shutdown** - Test clean shutdown
18. **Image Sharing** - Verify same image used
19. **Display Config** - Show docker-compose.yml
20. **Full Workflow** - Run all targets in sequence

## Expected Output

### Successful Test Run

```
=== Docker Compose MkDocs Workflow End-to-End Test ===
Project root: /path/to/project
Compose file: /path/to/docker-compose.mkdocs.yml
Image name: testcase-manager-docs:latest

=== Test 1: Checking Prerequisites ===
✓ Docker is installed
✓ Docker daemon is running
✓ docker-compose is installed
✓ curl is installed
✓ docker-compose.mkdocs.yml found

[... additional test output ...]

=== Test Summary ===
Total tests: 20
Tests passed: 20
Tests failed: 0

✓ All Docker Compose MkDocs workflow tests passed successfully!

The Docker Compose workflow is working correctly:
  ✓ docker-compose.mkdocs.yml syntax is valid
  ✓ mkdocs service starts with live reload
  ✓ mkdocs-build service generates site/ directory
  ✓ mkdocs-build-pdf service generates PDF with ENABLE_PDF_EXPORT=1
  ✓ Volume mounts work correctly for all three services
  ✓ make docs-compose-down stops services cleanly
  ✓ All services share the same image: testcase-manager-docs:latest

Available commands:
  - make docs-compose-up           # Start development server
  - make docs-compose-build-site   # Build HTML site
  - make docs-compose-build-pdf    # Build with PDF
  - make docs-compose-down         # Stop services
```

## Test Validation Criteria

### HTML Site Generation
- `site/` directory created
- `site/index.html` exists and is valid HTML
- File count > 5 in site directory
- Files readable and writable from host
- Material theme assets present

### PDF Generation
- `site/pdf/testcase-manager-documentation.pdf` exists
- PDF file size between 2-10 MB (warning if outside)
- Valid PDF format confirmed by `file` command
- PDF readable from host

### Development Server
- Server starts within 30 seconds
- Accessible at `http://localhost:8000/`
- Returns valid HTML content
- Live reload detects file changes
- No errors in service logs

### Service Management
- All services stop within 15 seconds
- No containers running after shutdown
- Services can be restarted successfully

## Integration with CI/CD

The test is designed to work in CI/CD pipelines:

```yaml
# Example GitLab CI configuration
test-docker-compose:
  stage: test
  script:
    - make docs-docker-build
    - make docs-compose-test
  artifacts:
    when: always
    paths:
      - site/
      - site/pdf/
```

## Troubleshooting

### Port Already in Use

If port 8000 is already in use:
```bash
# Find and stop the process using port 8000
lsof -ti:8000 | xargs kill -9

# Or stop docker-compose services
docker-compose -f docker-compose.mkdocs.yml down
```

### Docker Image Not Found

```bash
# Build the Docker image first
make docs-docker-build
```

### Volume Mount Permissions

If you encounter permission issues:
```bash
# Ensure the site/ directory is writable
chmod -R u+w site/

# Or remove and let Docker recreate it
rm -rf site/
```

### Service Won't Start

```bash
# Check service logs
docker-compose -f docker-compose.mkdocs.yml logs mkdocs

# Try rebuilding the image
make docs-docker-build
```

### Tests Fail But Manual Commands Work

Run with `--verbose` to see detailed output:
```bash
./tests/integration/test_docker_compose_mkdocs_e2e.sh --verbose
```

Keep temporary files for inspection:
```bash
./tests/integration/test_docker_compose_mkdocs_e2e.sh --no-remove
```

## Related Documentation

- **Docker Compose File**: `docker-compose.mkdocs.yml`
- **Dockerfile**: `Dockerfile.mkdocs`
- **Makefile Targets**: See `Makefile` for compose targets
- **Other Docker Tests**:
  - `test_docker_mkdocs_e2e.sh` - Docker image validation
  - `test_docker_html_build_e2e.sh` - HTML build via Docker
  - `test_docker_pdf_build_e2e.sh` - PDF generation via Docker
  - `test_docker_serve_e2e.sh` - Development server via Docker

## Test Coverage

The test provides comprehensive coverage of:
- ✅ Configuration file syntax validation
- ✅ Service definition verification
- ✅ Volume mount functionality
- ✅ Environment variable configuration
- ✅ HTML site generation
- ✅ PDF generation with WeasyPrint
- ✅ Development server with live reload
- ✅ Service lifecycle (start, stop, restart)
- ✅ Image sharing across services
- ✅ File permissions and ownership
- ✅ Error detection in service logs
- ✅ Complete workflow integration

## Performance Characteristics

- **Test Duration**: ~3-5 minutes (depends on documentation size)
- **Disk Usage**: Creates temporary files in `/tmp`
- **Network**: Tests HTTP requests on localhost:8000
- **Docker Resources**: Runs multiple services sequentially

## Exit Codes

- `0` - All tests passed
- `1` - One or more tests failed

## Maintenance Notes

The test is self-contained and uses the logger library for consistent output formatting. It automatically cleans up temporary files and Docker containers unless `--no-remove` is specified.

Updates to `docker-compose.mkdocs.yml` should be accompanied by corresponding updates to this test to maintain comprehensive coverage.
