# Docker MkDocs Configuration Validation Test

## Overview

The Docker MkDocs configuration validation test (`test_docker_mkdocs_config_validation_e2e.sh`) is an end-to-end integration test that comprehensively validates the MkDocs configuration within a Docker container environment.

## Purpose

This test ensures that:
1. MkDocs configuration is valid and can build without errors
2. All navigation paths point to existing documentation files
3. Material theme is properly configured and loads without errors
4. All required markdown extensions are configured correctly
5. PDF plugin configuration includes proper exclusion patterns
6. Search plugin is properly configured

## Test Categories

### 1. Prerequisites Check
- Verifies Docker is installed and daemon is running
- Confirms Docker image `testcase-manager-docs:latest` exists
- Validates mkdocs.yml configuration file exists

### 2. Configuration Validation
- Runs `mkdocs build --strict` to validate configuration
- Ensures build passes without warnings or errors

### 3. Navigation Paths Validation
- Extracts all navigation entries from mkdocs.yml
- Verifies each referenced file exists in the project
- Reports missing files if any

### 4. Material Theme Validation
- Confirms Material theme is configured
- Tests Material theme can be imported in container
- Validates theme features are configured:
  - navigation.tabs
  - navigation.sections
  - navigation.expand
  - navigation.top
  - search.suggest
  - search.highlight
  - content.code.copy
- Checks palette configuration exists

### 5. Markdown Extensions Validation

#### Required Extensions
- **pymdownx.highlight**: Code highlighting with syntax support
- **pymdownx.superfences**: Enhanced fenced code blocks with custom fences
- **admonition**: Note/warning/tip callout boxes
- **tables**: GitHub-flavored Markdown tables
- **toc**: Table of contents generation

#### Extension-Specific Tests

**pymdownx.highlight**:
- Validates configuration options:
  - `anchor_linenums`: Anchor line numbers for linking
  - `line_spans`: Line spans for styling
  - `pygments_lang_class`: Language class for Pygments
- Tests Python import succeeds

**pymdownx.superfences**:
- Validates configuration exists
- Checks custom_fences configuration
- Verifies Mermaid diagram support
- Tests Python import succeeds

**admonition**:
- Validates extension is configured
- Checks pymdownx.details for collapsible admonitions
- Tests Python import succeeds

**tables**:
- Validates extension is configured
- Tests Python import succeeds

**toc**:
- Validates extension is configured
- Checks permalink configuration
- Verifies toc_depth setting
- Tests Python import succeeds

### 6. PDF Plugin Validation
- Confirms with-pdf plugin is configured
- Validates `enabled_if_env` environment variable control
- Checks output_path configuration
- Verifies exclude_pages patterns (IMPLEMENTATION files)
- Validates excludes_children patterns
- Tests Python import succeeds

### 7. Search Plugin Validation
- Confirms search plugin is configured
- Validates language setting
- Checks separator configuration

### 8. Build Tests
- Tests complete documentation build in container
- Validates generated files exist (index.html, search index, assets)
- Runs strict mode build to catch any warnings
- Verifies MkDocs command accessibility

## Usage

### Basic Usage
```bash
# Run the test
make docs-docker-test-config

# Or run directly
./tests/integration/test_docker_mkdocs_config_validation_e2e.sh
```

### Keep Temporary Files
```bash
# Keep temporary files for debugging
./tests/integration/test_docker_mkdocs_config_validation_e2e.sh --no-remove
```

## Prerequisites

1. Docker must be installed and running
2. Docker image must be built first:
   ```bash
   make docs-docker-build
   ```

## Test Output

The test provides color-coded output:
- ✓ Green checkmark: Test passed
- ✗ Red X: Test failed
- ℹ Blue info: Informational message
- Yellow section headers: Test sections

### Example Output
```
=== Docker MkDocs Configuration Validation Test ===
[INFO] Project root: /path/to/project
[INFO] Image name: testcase-manager-docs:latest
[INFO] MkDocs config: mkdocs.yml

=== Test 1: Checking Prerequisites ===
✓ Docker is installed
✓ Docker daemon is running
✓ Docker image testcase-manager-docs:latest exists
✓ MkDocs configuration found

=== Test 2: Validate MkDocs Configuration with --strict ===
[INFO] Running: docker run --rm testcase-manager-docs:latest mkdocs build --strict
✓ MkDocs configuration is valid (--strict mode passed)

=== Test 3: Verify All Navigation Paths Point to Existing Files ===
[INFO] Found 35 navigation entries
[INFO] Existing files: 35
[INFO] Missing files: 0
✓ All navigation paths point to existing files

...

=== Test Summary ===
[INFO] Total tests: 15
[INFO] Tests passed: 15
[INFO] Tests failed: 0

✓ All Docker MkDocs configuration validation tests passed!
```

## Exit Codes

- **0**: All tests passed
- **1**: One or more tests failed

## Integration with CI/CD

This test can be integrated into CI/CD pipelines:

```yaml
# GitLab CI example
test:mkdocs-config:
  stage: test
  script:
    - make docs-docker-build
    - make docs-docker-test-config
  only:
    changes:
      - mkdocs.yml
      - docs/**/*
      - Dockerfile.mkdocs
      - requirements.txt
```

## Troubleshooting

### Docker Image Not Found
```
✗ Docker image testcase-manager-docs:latest not found
[ERROR] Please build the image first: make docs-docker-build
```
**Solution**: Build the Docker image first with `make docs-docker-build`

### Missing Navigation Files
```
✗ 2 navigation path(s) point to missing files
[ERROR] Missing files:
[ERROR]   - getting-started/quickstart.md
[ERROR]   - features/new-feature.md
```
**Solution**: Either create the missing files or remove them from mkdocs.yml navigation

### Configuration Validation Failed
```
✗ MkDocs configuration validation failed (--strict mode)
[ERROR] Build output:
ERROR - Config value: 'nav'. Error: ...
```
**Solution**: Review the error output and fix the configuration issue in mkdocs.yml

### Extension Import Failed
```
✗ pymdownx.highlight cannot be imported
[ERROR] Import error: ModuleNotFoundError: No module named 'pymdownx'
```
**Solution**: Update requirements.txt and rebuild the Docker image

## Related Documentation

- [MkDocs Setup Guide](../docs/MKDOCS_GUIDE.md)
- [Docker MkDocs Setup](../docs/DOCKER_MKDOCS.md)
- [MkDocs Test Documentation](README_MKDOCS_TEST.md)

## Maintenance

When adding new markdown extensions or plugins:
1. Update mkdocs.yml with the new configuration
2. Add corresponding validation tests to this script
3. Update requirements.txt if new Python packages are needed
4. Rebuild Docker image and run tests
5. Update this documentation with new test cases

## Technical Details

### Script Structure
- **Language**: Bash (compatible with bash 3.2+)
- **Dependencies**: Docker, logger.sh library
- **Test Framework**: Custom bash-based testing
- **Cleanup**: Automatic cleanup of temporary files

### Validation Methods
- **Configuration**: mkdocs build --strict
- **File Existence**: File system checks
- **Module Imports**: Python import tests in container
- **Build Success**: Exit code validation
- **Pattern Matching**: grep-based configuration verification

### Performance
- Typical runtime: 30-60 seconds
- Most time spent on Docker container operations
- Can run in parallel with other test suites

## Version History

- **v1.0**: Initial implementation
  - Complete configuration validation
  - All markdown extensions tested
  - PDF and search plugin validation
  - Navigation path verification
