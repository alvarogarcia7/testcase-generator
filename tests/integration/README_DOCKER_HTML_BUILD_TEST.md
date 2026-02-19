# Docker Container HTML Build End-to-End Test

## Overview

The `test_docker_html_build_e2e.sh` script provides comprehensive end-to-end testing for Docker container HTML builds. It validates that `make docs-docker-build-site` works correctly, verifies the generated HTML structure, tests file ownership and permissions, and ensures multiple sequential builds work properly.

## Quick Start

```bash
# Run the HTML build test
./tests/integration/test_docker_html_build_e2e.sh

# Run with verbose output
./tests/integration/test_docker_html_build_e2e.sh --verbose

# Keep temporary files for debugging
./tests/integration/test_docker_html_build_e2e.sh --no-remove

# Run with both flags
./tests/integration/test_docker_html_build_e2e.sh --verbose --no-remove
```

## What It Tests

### 1. Prerequisites Check
- Docker installation and daemon running
- Docker image exists (`testcase-manager-docs:latest`)
- Required project files exist (`mkdocs.yml`, `docs/` directory)

### 2. Clean Existing Site Directory
- Removes any existing `site/` directory
- Ensures clean state before build
- Verifies cleanup works correctly

### 3. Run Docker Build
- Executes `make docs-docker-build-site`
- Captures build output for debugging
- Verifies build completes successfully
- Tests Docker volume mounting works correctly

### 4. Verify Site Directory Creation
- Confirms `site/` directory exists
- Checks directory structure is created
- Counts files to ensure sufficient content
- Validates minimum file count (5+ files)

### 5. Verify HTML Files
- Checks for `index.html` existence
- Checks for `404.html` (optional but recommended)
- Counts total HTML files generated
- Ensures minimum HTML file count (3+ files)

### 6. Verify Markdown to HTML Conversion
- Confirms markdown files from `docs/` are converted
- Checks `index.md` → `index.html` conversion
- Validates subdirectory conversions:
  - `getting-started/`
  - `user-guide/`
  - `cli-tools/`
  - `features/`
- Reports HTML file counts per section

### 7. Verify Assets (CSS, JS, Images)
- Checks CSS files are copied and exist
- Verifies JavaScript files are present
- Validates common asset directories:
  - `assets/`
  - `stylesheets/`
  - `javascripts/`
  - `css/`
  - `js/`
- Checks for search index (`search/search_index.json`)
- Reports asset counts

### 8. Verify Ownership and Permissions
Critical test for host-container interaction:
- Verifies `site/` directory is readable from host
- Confirms `site/` directory is writable from host
- Checks all files are readable from host perspective
- Tests ability to create new files in `site/`
- Reports ownership information (user:group)
- Validates write permissions

### 9. Verify HTML Content Quality
- Checks for valid HTML structure:
  - `<html>` tag present
  - `<head>` section exists
  - `<body>` section exists
- Verifies Material theme assets
- Confirms search functionality
- Validates HTML is well-formed

### 10. Test Multiple Sequential Builds
Critical robustness test:
- Runs first build (already done in Test 3)
- Runs second build and verifies:
  - Build completes successfully
  - `site/` directory remains valid
  - Files are updated (timestamp check)
- Runs third build for consistency
- Ensures no corruption or permission issues

### 11. Verify Directory Structure with 'ls -la site/'
- Runs `ls -la site/` to display full directory listing
- Shows file permissions, ownership, sizes
- Lists subdirectories with file counts
- Provides visual verification of structure

### 12. Test Cleanup of site/ Directory
- Tests `make docs-clean` command
- Verifies `site/` directory is removed
- Confirms no files remain
- Rebuilds site to verify recovery
- Ensures cleanup and rebuild cycle works

### 13. Comprehensive Site Structure Verification
Final validation of complete structure:
- Checks expected directories exist:
  - `assets/`
  - `search/`
- Checks expected files exist:
  - `index.html`
  - `sitemap.xml`
  - `sitemap.xml.gz` (optional)
- Verifies navigation sections:
  - `getting-started/`
  - `user-guide/`
  - `cli-tools/`
  - `features/`
  - `development/`
- Reports statistics on found sections

## Test Output

The test provides detailed, color-coded output:

- ✓ **Green checkmark**: Test passed
- ✗ **Red X**: Test failed
- ℹ **Blue info**: Informational message
- **[WARNING]**: Warning message (non-critical)
- **=== Section ===**: Test section header

### Example Output

```
=== Docker Container HTML Build End-to-End Test ===
[INFO] Project root: /path/to/project
[INFO] Docker image: testcase-manager-docs:latest
[INFO] Site directory: /path/to/project/site

=== Test 1: Checking Prerequisites ===
✓ Docker is installed
✓ Docker daemon is running
✓ Docker image exists: testcase-manager-docs:latest
✓ mkdocs.yml found
✓ docs/ directory found

=== Test 3: Run 'make docs-docker-build-site' ===
[INFO] Running: make docs-docker-build-site
✓ make docs-docker-build-site completed successfully

=== Test 4: Verify Site Directory Creation ===
✓ site/ directory exists
[INFO] Total files in site/: 142
✓ site/ directory has sufficient files (142)

[... additional test output ...]

=== Test Summary ===
[INFO] Total tests: 13
[INFO] Tests passed: 13
[INFO] Tests failed: 0

[INFO] Final site/ directory statistics:
[INFO]   Total files: 142
[INFO]   Total directories: 28
[INFO]   HTML files: 45
[INFO]   CSS files: 12
[INFO]   JS files: 18
[INFO]   Total size: 3.2M

✓ All Docker HTML build tests passed successfully!

[INFO] The Docker container HTML build is working correctly:
[INFO]   ✓ site/ directory created with complete HTML structure
[INFO]   ✓ All markdown files converted to HTML
[INFO]   ✓ Assets (CSS, JS) copied correctly
[INFO]   ✓ Correct ownership and permissions from host
[INFO]   ✓ Multiple sequential builds work correctly
[INFO]   ✓ Cleanup of site/ directory works
```

## Exit Codes

- `0`: All tests passed successfully
- `1`: One or more tests failed

## Requirements

### System Requirements
- Docker installed and running
- Bash 3.2+ (macOS compatible)
- Standard Unix utilities (find, grep, stat, ls, du)
- Docker image already built (`testcase-manager-docs:latest`)

### Build Requirements
- Dockerfile.mkdocs built into image
- mkdocs.yml configuration file
- docs/ directory with markdown files
- Properly configured Makefile with `docs-docker-build-site` target

## Command Line Options

### --verbose
Enables verbose output showing additional debugging information:
- Build output from Docker commands
- Detailed file listings
- Debug messages

```bash
./tests/integration/test_docker_html_build_e2e.sh --verbose
```

### --no-remove
Keeps temporary files after test completes for debugging:
- Build output logs
- Temporary test files

```bash
./tests/integration/test_docker_html_build_e2e.sh --no-remove
```

## Examples

### Basic Test Run
```bash
# Ensure Docker image is built first
make docs-docker-build

# Run the HTML build test
./tests/integration/test_docker_html_build_e2e.sh
```

### Full Test with Clean Build
```bash
# Clean everything
make docs-clean
make docs-docker-clean

# Rebuild Docker image
make docs-docker-build

# Run HTML build test
./tests/integration/test_docker_html_build_e2e.sh
```

### Debug Failed Build
```bash
# Run with verbose output and keep temp files
./tests/integration/test_docker_html_build_e2e.sh --verbose --no-remove

# Check the site directory
ls -la site/

# Manually inspect build output
cat /tmp/build_output_* 2>/dev/null
```

### Test Ownership Issues
```bash
# Run the test
./tests/integration/test_docker_html_build_e2e.sh

# Check ownership
ls -la site/
stat site/index.html

# Try to modify from host
echo "test" > site/test.txt
rm site/test.txt
```

### Test Multiple Builds
```bash
# The script automatically tests this, but you can manually verify:
make docs-docker-build-site
ls -la site/
sleep 2
make docs-docker-build-site
ls -la site/
```

## Integration with CI/CD

The test can be integrated into CI/CD pipelines:

### GitLab CI Example
```yaml
test-docker-html-build:
  stage: test
  script:
    - make docs-docker-build
    - ./tests/integration/test_docker_html_build_e2e.sh
  artifacts:
    when: always
    paths:
      - site/
    expire_in: 1 week
  dependencies:
    - docs-docker-build
```

### GitHub Actions Example
```yaml
- name: Build Docker Image
  run: make docs-docker-build

- name: Test Docker HTML Build
  run: ./tests/integration/test_docker_html_build_e2e.sh

- name: Upload Site Artifacts
  if: always()
  uses: actions/upload-artifact@v3
  with:
    name: documentation-site
    path: site/
```

## Troubleshooting

### Test Fails on Site Directory Creation
**Problem**: `site/` directory not created after build

**Solutions**:
- Check Docker volume mounting in Makefile
- Verify `docs-docker-build-site` target syntax
- Ensure Docker image contains mkdocs
- Check container permissions

```bash
# Manual verification
docker run --rm \
  -v "$(PWD)/docs:/docs/docs" \
  -v "$(PWD)/mkdocs.yml:/docs/mkdocs.yml" \
  -v "$(PWD)/site:/docs/site" \
  testcase-manager-docs:latest mkdocs build
```

### Ownership/Permission Issues
**Problem**: Files owned by wrong user or not readable/writable from host

**Solutions**:
- Check Docker user configuration (should be `mkdocs` with UID 1000)
- Verify volume mount points in Makefile
- Check host user UID matches container UID

```bash
# Check container user
docker run --rm testcase-manager-docs:latest id

# Check site ownership
ls -la site/

# Fix ownership if needed (Linux)
sudo chown -R $USER:$USER site/
```

### Missing Assets
**Problem**: CSS/JS files not copied to site/

**Solutions**:
- Verify mkdocs.yml theme configuration
- Check Material theme installation in Docker image
- Ensure assets are in docs/ directory
- Rebuild Docker image

```bash
# Check theme in container
docker run --rm testcase-manager-docs:latest pip show mkdocs-material

# Rebuild image
make docs-docker-clean
make docs-docker-build
```

### HTML Not Generated
**Problem**: Markdown files not converted to HTML

**Solutions**:
- Check mkdocs.yml nav configuration
- Verify markdown files exist in docs/
- Check mkdocs build output for errors
- Ensure file permissions allow reading

```bash
# Test build manually with verbose output
make docs-docker-build-site 2>&1 | tee build.log

# Check for errors
grep -i error build.log
```

### Multiple Builds Fail
**Problem**: Second or third build fails or corrupts site/

**Solutions**:
- Clean site/ before each build
- Check for permission changes between builds
- Verify Docker volume mounting consistency
- Check for file locking issues

```bash
# Clean and rebuild
make docs-clean
make docs-docker-build-site

# Check if issue persists
./tests/integration/test_docker_html_build_e2e.sh
```

### Cleanup Doesn't Work
**Problem**: `make docs-clean` doesn't remove site/ or leaves files

**Solutions**:
- Check Makefile docs-clean target
- Verify permissions on site/ directory
- Check for open file handles
- Use force removal if needed

```bash
# Check what's preventing removal
lsof +D site/ 2>/dev/null

# Force removal
rm -rf site/

# Verify Makefile target
make docs-clean
```

## Platform-Specific Notes

### macOS (BSD)
- Uses `stat -f %m` for modification time
- File ownership may differ from Linux
- Case-insensitive filesystem by default

### Linux (GNU)
- Uses `stat -c %Y` for modification time
- Standard file ownership semantics
- Case-sensitive filesystem

The test script is designed to work on both platforms using portable shell syntax.

## Related Documentation

- [Docker MkDocs Setup](../../README_DOCKER_MKDOCS.md)
- [Docker MkDocs E2E Test](README_DOCKER_MKDOCS_TEST.md)
- [MkDocs Test Guide](../../scripts/README_MKDOCS_TEST.md)
- [AGENTS.md](../../AGENTS.md) - Project commands reference

## Test Maintenance

### Adding New Tests
1. Add new test section after existing tests
2. Use logger library functions:
   - `section "Test Name"` for headers
   - `pass "message"` for successes
   - `fail "message"` for failures
   - `log_info "message"` for information
   - `log_warning "message"` for warnings
3. Update test counters: `TESTS_PASSED` or `TESTS_FAILED`
4. Document new tests in this README

### Updating for New Features
1. Update directory structure checks for new sections
2. Add new asset type checks if needed
3. Update HTML content validation for new features
4. Update this README with new validations

### Version Compatibility
The test is designed to be:
- **Platform-agnostic**: Works on Linux and macOS
- **Docker-version-agnostic**: Uses standard Docker commands
- **Shell-compatible**: Uses bash 3.2+ compatible syntax
- **Portable**: Uses POSIX-compliant commands where possible

## Test Architecture

### Logger Library Integration
Uses the centralized logger library (`scripts/lib/logger.sh`):
- Consistent output formatting
- Color-coded messages
- Cleanup management for temporary files
- Error handling support

### Test Structure
1. **Setup Phase**: Check prerequisites and prepare environment
2. **Execution Phase**: Run builds and generate content
3. **Verification Phase**: Validate outputs and structure
4. **Cleanup Phase**: Test cleanup and recovery
5. **Reporting Phase**: Display results and statistics

### Error Handling
- Exits on critical failures (missing Docker, image not found)
- Continues through non-critical failures (optional files)
- Tracks pass/fail counts for comprehensive reporting
- Provides detailed error messages with context

## Performance Considerations

### Test Duration
- Typical run time: 1-3 minutes
- Depends on site size and Docker performance
- Multiple builds add 30-60 seconds

### Resource Usage
- Disk space: Requires space for site/ directory (typically 5-20MB)
- Memory: Docker container memory (typically < 512MB)
- CPU: Brief spikes during mkdocs build

### Optimization Tips
- Use Docker image cache for faster builds
- Keep docs/ directory organized
- Minimize unnecessary files in docs/
- Use `.dockerignore` to exclude build artifacts

## License

This test script follows the same license as the main project.

## Contributing

When contributing improvements to this test:
1. Maintain backward compatibility
2. Follow existing code style and patterns
3. Update this README with changes
4. Test on both Linux and macOS if possible
5. Ensure all tests pass before submitting
