# GitLab CI Pages Job Validation Test

## Overview

The GitLab CI Pages job validation test (`test_gitlab_ci_pages_e2e.sh`) provides comprehensive end-to-end validation of the `.gitlab-ci.yml` configuration file to ensure the GitLab Pages deployment job is correctly configured and functional.

## Test Location

```
tests/integration/test_gitlab_ci_pages_e2e.sh
```

## Purpose

This test validates the GitLab CI Pages job configuration through multiple verification steps:

1. **GitLab CI YAML Syntax** - Validates proper YAML syntax
2. **Docker Image Configuration** - Verifies python:3.11 image is used
3. **pip Install Validation** - Tests pip install from requirements.txt in CI container
4. **Environment Variables** - Verifies ENABLE_PDF_EXPORT=1 is set
5. **Build Command** - Validates mkdocs build --site-dir public command
6. **Artifacts Configuration** - Confirms public/ directory export with 30-day expiration
7. **Cache Configuration** - Validates pip cache configuration
8. **Branch Restrictions** - Ensures job only runs on main branch
9. **Full Pipeline Simulation** - Simulates complete GitLab CI pipeline execution

## Usage

### Basic Execution

```bash
make gitlab-ci-test
```

or

```bash
./tests/integration/test_gitlab_ci_pages_e2e.sh
```

### With Options

```bash
# Keep temporary directories for debugging
./tests/integration/test_gitlab_ci_pages_e2e.sh --no-remove
```

## Prerequisites

### Required Software

- **Docker**: Must be installed and running
- **Python 3**: Required for YAML syntax validation (optional)
- **bash 3.2+**: Required for script execution

### Required Files

- `.gitlab-ci.yml` - GitLab CI configuration file
- `requirements.txt` - Python dependencies for MkDocs
- `mkdocs.yml` - MkDocs configuration file
- `docs/` directory - Documentation source files

## Test Steps

### Test 1: Prerequisites Check

Verifies:
- Docker is installed and accessible
- Docker daemon is running
- GitLab CI config file exists
- Requirements file exists

### Test 2: GitLab CI YAML Syntax

Validates:
- YAML file can be parsed without errors
- File structure is valid

Uses Python's `yaml` library if available.

### Test 3: Docker Image Configuration

Validates:
- Pages job uses `python:3.11` Docker image
- Image specification matches GitLab CI best practices

### Test 4: pip Install in CI Container

Validates:
- requirements.txt can be installed in python:3.11 container
- All dependencies are compatible with the container environment
- MkDocs is successfully installed and accessible

Tests actual installation in the same Docker image used in CI.

### Test 5: ENABLE_PDF_EXPORT Environment Variable

Validates:
- `ENABLE_PDF_EXPORT=1` is set in the pages job script
- Environment variable is properly configured for PDF generation

### Test 6: mkdocs build Command

Validates:
- Command uses `--site-dir public` flag for GitLab Pages
- Build command generates output in public/ directory
- Generated files include index.html and PDF documentation

Tests actual build execution in python:3.11 container.

### Test 7: Artifacts Configuration

Validates:
- Artifacts configuration exports `public/` directory
- Artifacts expire in 30 days
- Configuration matches GitLab Pages requirements

### Test 8: Cache Configuration

Validates:
- Cache is configured for `~/.cache/pip` directory
- Cache configuration follows GitLab CI best practices
- Configuration can improve build performance

### Test 9: Branch Restrictions

Validates:
- Pages job only runs on `main` branch
- Branch restriction is properly configured with `only:` directive

### Test 10: Full Pipeline Simulation

Simulates complete GitLab CI pipeline:
1. Pulls python:3.11 Docker image
2. Installs dependencies from requirements.txt
3. Builds documentation with ENABLE_PDF_EXPORT=1
4. Verifies public/ directory contains expected files
5. Confirms PDF documentation is generated

## Expected Outputs

### Success Output

```
=== GitLab CI Pages Job End-to-End Validation Test ===
[INFO] Project root: /path/to/project
[INFO] GitLab CI config: /path/to/project/.gitlab-ci.yml
[INFO] Expected Docker image: python:3.11

=== Test 1: Checking Prerequisites ===
✓ Docker is installed
✓ Docker daemon is running
✓ GitLab CI config found
✓ Requirements file found

=== Test 2: Validate GitLab CI YAML Syntax ===
[INFO] Checking .gitlab-ci.yml syntax...
✓ GitLab CI YAML syntax is valid

=== Test 3: Validate Docker Image Configuration ===
[INFO] Checking pages job uses python:3.11 image...
✓ Pages job uses python:3.11 Docker image

=== Test 4: Validate pip install in CI Container ===
[INFO] Testing pip install from requirements.txt in python:3.11 container...
✓ pip install from requirements.txt works in python:3.11 container

=== Test 5: Validate ENABLE_PDF_EXPORT Environment Variable ===
[INFO] Checking ENABLE_PDF_EXPORT=1 is set in pages job script...
✓ ENABLE_PDF_EXPORT=1 environment variable is set

=== Test 6: Validate mkdocs build Command ===
[INFO] Checking mkdocs build --site-dir public command...
✓ mkdocs build uses --site-dir public
[INFO] Testing mkdocs build generates output in public/ directory...
✓ mkdocs build generates output in public/ directory

=== Test 7: Validate Artifacts Configuration ===
[INFO] Checking artifacts paths configuration...
✓ Artifacts exports public/ directory
[INFO] Checking artifacts expiration...
✓ Artifacts expire in 30 days

=== Test 8: Validate Cache Configuration ===
[INFO] Checking pip cache configuration...
✓ Cache configured for pip dependencies
[INFO] Testing cache effectiveness in Docker environment...
[INFO] First pip install took 15s (without cache)
✓ Cache configuration is present and can be used in CI environment

=== Test 9: Validate Branch Restrictions ===
[INFO] Checking job only runs on main branch...
✓ Pages job only runs on main branch

=== Test 10: Simulate Complete GitLab CI Pipeline ===
[INFO] Running full CI pipeline simulation...
[INFO] Step 1: Pulling python:3.11 image...
✓ Docker image pulled successfully
[INFO] Step 2: Installing dependencies with pip...
✓ Dependencies installed successfully
[INFO] Step 3: Building documentation with ENABLE_PDF_EXPORT=1...
✓ Documentation built successfully
[INFO] Step 4: Verifying public/ directory contents...
✓ public/ directory contains expected files

=== Test Summary ===

[INFO] Total tests passed: 14
[INFO] Total tests failed: 0

✓ All GitLab CI Pages job validation tests passed!
```

### Failure Output

If any test fails, the output will show:

```
✗ [Test description]
[ERROR] Detailed error message

=== Test Summary ===

[INFO] Total tests passed: 10
[INFO] Total tests failed: 4

✗ Some GitLab CI Pages job validation tests failed
```

## Validation Coverage

### GitLab CI Configuration

- ✅ YAML syntax validation
- ✅ Docker image specification (python:3.11)
- ✅ Script commands validation
- ✅ Environment variables (ENABLE_PDF_EXPORT=1)
- ✅ Artifacts configuration (public/ directory, 30 days expiration)
- ✅ Cache configuration (~/.cache/pip)
- ✅ Branch restrictions (only: main)

### Container Environment

- ✅ Python 3.11 environment compatibility
- ✅ pip package installation
- ✅ MkDocs installation and execution
- ✅ PDF generation dependencies

### Build Process

- ✅ Documentation build command
- ✅ Output directory structure (public/)
- ✅ Generated files validation (index.html, PDF)
- ✅ Build process end-to-end simulation

## Troubleshooting

### Docker Not Available

**Error**: `Docker is not installed or not in PATH`

**Solution**: Install Docker from https://www.docker.com/get-started and ensure it's in your PATH.

### Docker Daemon Not Running

**Error**: `Docker daemon is not running`

**Solution**: Start Docker Desktop or Docker service on your system.

### GitLab CI Config Not Found

**Error**: `GitLab CI config not found at .gitlab-ci.yml`

**Solution**: Ensure you're running the test from the project root or the `.gitlab-ci.yml` file exists.

### pip Install Fails

**Error**: `pip install failed in python:3.11 container`

**Solution**: Check requirements.txt for syntax errors or incompatible package versions.

### Build Fails

**Error**: `mkdocs build failed to generate output in public/ directory`

**Solution**: 
- Check mkdocs.yml configuration
- Verify all required documentation files exist in docs/
- Check for MkDocs or plugin errors in verbose output

## Integration with CI/CD

This test is designed to be run in CI/CD pipelines to validate GitLab CI configuration changes before they affect production deployments.

### Example CI Usage

```yaml
test:
  stage: test
  script:
    - make gitlab-ci-test
```

## Performance Considerations

- **Duration**: Typical execution time is 30-60 seconds
- **Docker pulls**: First run may be slower due to image downloads
- **Caching**: Subsequent runs are faster with Docker layer caching
- **Resources**: Requires ~500MB disk space for Docker images and temporary files

## Maintenance

### Updating Expected Values

If GitLab CI configuration changes, update the following variables in the test script:

```bash
IMAGE_NAME="python:3.11"              # Docker image
EXPECTED_SITE_DIR="public"             # Output directory
EXPECTED_ARTIFACT_EXPIRE="30 days"     # Artifact expiration
EXPECTED_CACHE_PATH="\$HOME/.cache/pip" # Cache path
```

### Adding New Tests

To add new validation tests:

1. Add new test section with incremented number
2. Update test counter logic
3. Document new test in this README
4. Update test summary section

## Related Documentation

- [GitLab CI/CD Pages Documentation](https://docs.gitlab.com/ee/user/project/pages/)
- [MkDocs Documentation](https://www.mkdocs.org/)
- [Docker Documentation](https://docs.docker.com/)
- [AGENTS.md](../AGENTS.md) - Project commands and requirements

## Exit Codes

- `0` - All tests passed
- `1` - One or more tests failed

## Logging

The test uses the centralized logging library (`scripts/lib/logger.sh`) for consistent output:

- `log_info` - Informational messages
- `log_warning` - Warning messages
- `log_error` - Error messages
- `pass` - Successful test results (green checkmark)
- `fail` - Failed test results (red X)
- `section` - Test section headers

## Cleanup

The test automatically cleans up temporary directories and files on exit unless `--no-remove` flag is provided.

Temporary directories created:
- Build test directory
- CI simulation directory
- pip cache test directory

## Compatibility

- ✅ macOS (bash 3.2+)
- ✅ Linux (bash 3.2+)
- ✅ BSD variants
- ✅ Docker Desktop
- ✅ Docker Engine

## Security Considerations

- Tests run in isolated Docker containers
- Temporary files are cleaned up automatically
- No network access required beyond Docker Hub image pulls
- No modifications to project files during testing

## License

This test script is part of the testcase-manager project and follows the project's license terms.
