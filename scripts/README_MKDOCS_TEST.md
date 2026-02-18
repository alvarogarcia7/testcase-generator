# MkDocs End-to-End Testing

This document describes the end-to-end testing script for the MkDocs documentation setup.

## Overview

The `test-mkdocs-setup.sh` script performs comprehensive testing of the MkDocs documentation system, validating:

1. **Installation**: Verifies MkDocs and plugins install correctly
2. **Virtual Environment**: Confirms virtualenv creation and structure
3. **Local Server**: Tests documentation serving at localhost:8000
4. **HTML Build**: Validates static site generation
5. **PDF Export**: Confirms PDF documentation generation
6. **Link Validation**: Checks internal links in HTML
7. **Test Integration**: Ensures documentation changes don't break tests

## Quick Start

### Full Test Suite

Run all tests with a clean install:

```bash
make docs-test-clean
```

### Standard Test

Run all tests (assumes virtualenv exists):

```bash
make docs-test
```

### Quick Test

Skip serve and unit tests for faster validation:

```bash
make docs-test-quick
```

## Command-Line Usage

### Direct Script Execution

```bash
./scripts/test-mkdocs-setup.sh [OPTIONS]
```

### Available Options

| Option | Description |
|--------|-------------|
| `--skip-install` | Skip docs-install step (assumes venv exists) |
| `--skip-serve` | Skip docs-serve test (useful in CI/CD) |
| `--skip-build` | Skip docs-build test |
| `--skip-pdf` | Skip docs-build-pdf test |
| `--skip-links` | Skip link checking |
| `--skip-tests` | Skip running unit tests |
| `--clean` | Clean all artifacts before starting |
| `--help` | Show help message |

### Examples

#### Full test with clean install
```bash
./scripts/test-mkdocs-setup.sh --clean
```

#### Skip serve test (for CI/CD environments)
```bash
./scripts/test-mkdocs-setup.sh --skip-serve
```

#### Quick validation (skip serve and tests)
```bash
./scripts/test-mkdocs-setup.sh --skip-serve --skip-tests
```

#### Test only build and PDF generation
```bash
./scripts/test-mkdocs-setup.sh --skip-install --skip-serve --skip-tests
```

## Test Steps

The script performs the following tests in order:

### 1. Install MkDocs (Optional)
- Runs `make docs-install`
- Creates Python virtual environment
- Installs MkDocs and required plugins
- **Skip with**: `--skip-install`

### 2. Verify Virtual Environment
- Checks virtualenv directory exists
- Verifies MkDocs executable is present
- Tests MkDocs version command
- **Always runs** (required for all other tests)

### 3. Serve Documentation (Optional)
- Starts MkDocs development server on port 8000
- Waits for server to become responsive (10s timeout)
- Tests accessibility at http://localhost:8000
- Validates expected content is present
- Tests navigation links (Getting Started, User Guide, etc.)
- Verifies page rendering
- Stops server after tests complete
- **Skip with**: `--skip-serve`

### 4. Build HTML Site (Optional)
- Runs `make docs-build`
- Generates static HTML in `site/` directory
- **Skip with**: `--skip-build`

### 5. Verify Site Structure (Optional)
- Checks `site/` directory exists
- Verifies `index.html` is present and contains expected content
- Validates asset directories (CSS, JS, assets)
- Confirms section directories exist (getting-started, user-guide, cli-tools, features, development)
- **Skip with**: `--skip-build`

### 6. Build PDF Documentation (Optional)
- Runs `make docs-build-pdf`
- Generates PDF with table of contents and all sections
- **Skip with**: `--skip-pdf`

### 7. Verify PDF Generation (Optional)
- Checks PDF exists at `site/pdf/testcase-manager-documentation.pdf`
- Validates PDF is not empty
- Verifies PDF header is valid
- Extracts PDF information with `pdfinfo` (if available)
- **Skip with**: `--skip-pdf`

### 8. Test Internal Links (Optional)
- Scans all HTML files for links
- Validates internal links point to existing files
- Reports broken links (may include false positives)
- **Skip with**: `--skip-links`

### 9. Run Unit Tests (Optional)
- Runs `make test`
- Ensures documentation changes don't break existing tests
- **Skip with**: `--skip-tests`

## Output

The script uses color-coded output for easy interpretation:

- ✓ **Green**: Test passed
- ✗ **Red**: Test failed
- ℹ **Blue**: Informational message
- **Yellow**: Section headers
- **Orange**: Warnings

## Exit Codes

- `0`: All tests passed successfully
- `1`: One or more tests failed

## Requirements

### System Requirements

- **Python 3.8+**: Required for MkDocs
- **curl**: For testing HTTP endpoints
- **make**: For running Makefile targets

### Optional Tools

- **pdfinfo**: For detailed PDF validation (from poppler-utils)
  ```bash
  # Ubuntu/Debian
  sudo apt-get install poppler-utils
  
  # macOS
  brew install poppler
  ```

## CI/CD Integration

### GitHub Actions

```yaml
- name: Test MkDocs setup
  run: |
    ./scripts/test-mkdocs-setup.sh --skip-serve
```

### GitLab CI

```yaml
test-docs:
  stage: test
  script:
    - ./scripts/test-mkdocs-setup.sh --skip-serve
  artifacts:
    paths:
      - site/
      - site/pdf/testcase-manager-documentation.pdf
    expire_in: 7 days
```

### Skip Serve in CI/CD

The `--skip-serve` option is recommended for CI/CD environments because:
- Avoids port conflicts in containerized builds
- Reduces test time
- Eliminates need for curl/network testing
- HTML build verification is sufficient

## Troubleshooting

### Virtual Environment Not Found

**Error**: `Virtual environment not found: mkdocs-venv`

**Solution**: Run with clean install or install first:
```bash
./scripts/test-mkdocs-setup.sh --clean
# Or
make docs-install
./scripts/test-mkdocs-setup.sh --skip-install
```

### Server Start Timeout

**Error**: `Server failed to start within 10s`

**Solution**: 
- Check port 8000 is not already in use: `lsof -i :8000`
- Increase timeout in script (edit `SERVE_TIMEOUT` variable)
- Skip serve test: `./scripts/test-mkdocs-setup.sh --skip-serve`

### PDF Not Generated

**Error**: `PDF not found: site/pdf/testcase-manager-documentation.pdf`

**Solution**:
- Ensure PDF plugin is installed: `mkdocs-venv/bin/pip list | grep mkdocs-with-pdf`
- Check for build errors: `make docs-build-pdf`
- Verify `ENABLE_PDF_EXPORT=1` is set in environment

### Broken Links Reported

**Warning**: `Found X potential broken links (may be false positives)`

**Notes**:
- Some link formats may not be detected correctly
- External links are not checked
- Fragment-only links (#section) are ignored
- Review reported links manually

### Tests Fail After Documentation Changes

**Error**: `make test failed`

**Solution**:
- Review test output for specific failures
- Ensure documentation changes didn't introduce syntax errors
- Verify file paths and references are correct
- Update tests if documentation structure changed

## File Locations

- **Script**: `scripts/test-mkdocs-setup.sh`
- **Virtual Environment**: `mkdocs-venv/`
- **HTML Output**: `site/`
- **PDF Output**: `site/pdf/testcase-manager-documentation.pdf`
- **Configuration**: `mkdocs.yml`
- **Requirements**: `requirements.txt`

## Makefile Targets

| Target | Description |
|--------|-------------|
| `make docs-test` | Run full test suite |
| `make docs-test-clean` | Run full test suite with clean install |
| `make docs-test-quick` | Run quick test (skip serve and tests) |
| `make docs-install` | Install MkDocs and plugins |
| `make docs-serve` | Serve documentation locally |
| `make docs-build` | Build HTML documentation |
| `make docs-build-pdf` | Build HTML + PDF documentation |
| `make docs-clean` | Remove site/ directory |

## Best Practices

### Development Workflow

1. **Initial Setup**:
   ```bash
   make docs-install
   ```

2. **Test Changes**:
   ```bash
   make docs-test-quick
   ```

3. **Full Validation**:
   ```bash
   make docs-test
   ```

4. **Before Commit**:
   ```bash
   make docs-test-clean
   make test
   ```

### Pre-Commit Checklist

- [ ] Run `make docs-install` if dependencies changed
- [ ] Run `make docs-test` to validate all functionality
- [ ] Review any warnings about broken links
- [ ] Ensure `make test` passes
- [ ] Verify PDF generation succeeded
- [ ] Check HTML output in browser

## Related Documentation

- [MkDocs Guide](../docs/MKDOCS_GUIDE.md) - Comprehensive MkDocs documentation
- [Installation Script](./install-mkdocs.sh) - MkDocs installation script
- [AGENTS.md](../AGENTS.md) - Project documentation guidelines

## Support

For issues or questions:
1. Check troubleshooting section above
2. Review script output for detailed error messages
3. Verify requirements are installed
4. Test individual commands manually
