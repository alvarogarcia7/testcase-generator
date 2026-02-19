# Docker PDF Build E2E Test

## Overview

The Docker PDF build end-to-end test (`test_docker_pdf_build_e2e.sh`) validates that PDF documentation generation works correctly inside Docker containers. This test ensures that the `make docs-docker-build-pdf` command with `ENABLE_PDF_EXPORT=1` produces a valid, complete PDF with proper structure and content.

## Purpose

This test verifies:

1. **PDF Generation**: Runs `make docs-docker-build-pdf` with ENABLE_PDF_EXPORT=1
2. **File Location**: Verifies PDF is generated at `site/pdf/testcase-manager-documentation.pdf`
3. **Table of Contents**: Tests PDF contains TOC with correct depth (toc_level: 3)
4. **Content Filtering**: Verifies implementation notes are excluded (IMPLEMENTATION_*.md, *_SUMMARY.md)
5. **Structure Validation**: Validates PDF structure using pdfinfo via Docker alpine container
6. **Size Validation**: Tests PDF file size is reasonable (2-10MB)
7. **Error Checking**: Verifies PDF generation doesn't fail with WeasyPrint errors
8. **Format Validation**: Tests that `file site/pdf/*.pdf` shows valid PDF

## Usage

### Basic Usage

```bash
# Run the test
./tests/integration/test_docker_pdf_build_e2e.sh

# Or via Makefile
make docs-docker-test-pdf
```

### Advanced Options

```bash
# Keep temporary files for debugging (don't remove after test)
./tests/integration/test_docker_pdf_build_e2e.sh --no-remove

# Enable verbose output (shows detailed debug information)
./tests/integration/test_docker_pdf_build_e2e.sh --verbose

# Combine options
./tests/integration/test_docker_pdf_build_e2e.sh --no-remove --verbose
```

## Prerequisites

Before running this test, ensure:

1. **Docker is installed and running**: The test requires Docker daemon to be active
2. **Docker image is built**: Run `make docs-docker-build` to build the image first
3. **Sufficient disk space**: PDF generation requires space for site/ directory and PDF file
4. **Network access**: Docker may need to pull alpine image for pdfinfo validation

## Test Workflow

### Phase 1: Prerequisites Check (Tests 1-2)
1. Verify Docker is installed and running
2. Check Docker image exists (testcase-manager-docs:latest)
3. Verify mkdocs.yml and docs/ directory exist
4. Validate PDF configuration in mkdocs.yml
5. Check for with-pdf plugin configuration
6. Verify ENABLE_PDF_EXPORT environment variable setup
7. Confirm output path and TOC settings
8. Validate excludes_children patterns

### Phase 2: Build and Generation (Tests 3-7)
9. Clean existing site/ directory
10. Run `make docs-docker-build-pdf` with ENABLE_PDF_EXPORT=1
11. Monitor for WeasyPrint errors during build
12. Verify site/ and site/pdf/ directories created
13. Check PDF file exists at expected location
14. Validate PDF file size (2-10MB range)
15. Verify file type using `file` command

### Phase 3: Content Validation (Tests 8-12)
16. Run pdfinfo via Docker alpine container
17. Extract and verify PDF metadata (title, author, pages)
18. Check page count is reasonable (minimum 10 pages)
19. Extract text using pdftotext
20. Verify Table of Contents structure
21. Check for major documentation sections
22. Test implementation notes exclusion
23. Verify IMPLEMENTATION_*.md and *_SUMMARY.md patterns not present

### Phase 4: Reliability Testing (Tests 13-16)
24. Test multiple sequential PDF builds
25. Verify timestamp updates between builds
26. Confirm file sizes remain consistent
27. Test cleanup with `make docs-clean`
28. Verify PDF removed with site/ directory
29. Rebuild and verify recreation
30. Final PDF validation and structure check

## Expected Output

### Successful Test Run

```
=== Docker Container PDF Generation End-to-End Test ===
[INFO] Project root: /path/to/testcase-manager
[INFO] Docker image: testcase-manager-docs:latest
[INFO] PDF file: /path/to/site/pdf/testcase-manager-documentation.pdf
[INFO] Expected PDF size: 2MB - 10MB

=== Test 1: Checking Prerequisites ===
✓ Docker is installed
✓ Docker daemon is running
✓ Docker image exists: testcase-manager-docs:latest
✓ mkdocs.yml found
✓ docs/ directory found

=== Test 2: Verify mkdocs.yml PDF Configuration ===
✓ mkdocs-with-pdf plugin configured
✓ PDF export enabled via ENABLE_PDF_EXPORT environment variable
✓ PDF output path configured correctly
✓ Table of contents depth set to 3
✓ excludes_children configuration found
✓ IMPLEMENTATION_*.md files excluded from PDF
✓ *_SUMMARY.md files excluded from PDF

=== Test 4: Run 'make docs-docker-build-pdf' with ENABLE_PDF_EXPORT=1 ===
✓ Makefile target sets ENABLE_PDF_EXPORT=1
✓ make docs-docker-build-pdf completed successfully

=== Test 5: Verify No WeasyPrint Errors ===
✓ No WeasyPrint errors or warnings

=== Test 7: Verify PDF File Generation ===
✓ PDF file exists: /path/to/site/pdf/testcase-manager-documentation.pdf
[INFO] PDF file size: 5 MB (5242880 bytes)

=== Test 8: Verify PDF File Type with 'file' Command ===
✓ File is recognized as a valid PDF

=== Test 10: Validate PDF Structure Using pdfinfo ===
✓ pdfinfo command executed successfully
✓ PDF has title: Test Case Manager Documentation
✓ PDF has author: Test Case Manager Team
✓ PDF has sufficient pages (45)
✓ PDF version: 1.4

=== Test Summary ===
[INFO] Total tests: 16
[INFO] Tests passed: 16
[INFO] Tests failed: 0

✓ All Docker PDF generation tests passed successfully!
```

### Failed Test Example

```
=== Test 7: Verify PDF File Generation ===
✗ PDF file not found at expected location
[ERROR] Contents of site/pdf/ directory:
total 0
drwxr-xr-x 2 user user 4096 Jan 1 12:00 .

=== Test Summary ===
[ERROR] 3 test(s) failed
[INFO] Please review the output above and fix the issues
```

## Test Coverage

### Configuration Tests
- ✓ mkdocs-with-pdf plugin presence
- ✓ ENABLE_PDF_EXPORT environment variable
- ✓ PDF output path configuration
- ✓ TOC level setting (3 levels deep)
- ✓ excludes_children patterns

### Build Process Tests
- ✓ Docker image availability
- ✓ Build command execution
- ✓ WeasyPrint error detection
- ✓ Directory structure creation
- ✓ File generation completion

### PDF Quality Tests
- ✓ File format validation
- ✓ File size reasonableness (2-10MB)
- ✓ Metadata completeness
- ✓ Page count validation
- ✓ PDF version compatibility

### Content Tests
- ✓ Table of Contents presence
- ✓ Major section inclusion
- ✓ Implementation notes exclusion
- ✓ Text extraction capability
- ✓ Content structure validation

### Reliability Tests
- ✓ Multiple builds consistency
- ✓ Timestamp updates
- ✓ Cleanup functionality
- ✓ Recreation after cleanup
- ✓ File permission handling

## PDF Structure Validation

The test uses Docker alpine container with poppler-utils to validate PDF structure:

```bash
docker run --rm -v $(pwd)/site:/site alpine sh -c \
  "apk add poppler-utils && pdfinfo /site/pdf/*.pdf"
```

### Validated Properties

- **Title**: Test Case Manager Documentation
- **Author**: Test Case Manager Team
- **Pages**: Minimum 10 pages expected
- **PDF Version**: 1.4 or higher
- **File Size**: Reported by pdfinfo
- **Optimization**: Whether PDF is optimized

## Implementation Notes Exclusion

The test verifies that the following patterns are properly excluded from the PDF:

### Pattern Matching
- `IMPLEMENTATION_*.md` - All implementation notes files
- `*_SUMMARY.md` - All summary files

### Specific Files Checked
- IMPLEMENTATION_COMPLETE
- IMPLEMENTATION_SUMMARY
- DOCUMENTATION_SUMMARY
- DOCKER_CLEANUP_SUMMARY

### Verification Method

The test extracts text from the PDF using pdftotext and searches for these patterns. If found, warnings are issued but the test may still pass if the patterns appear as content references rather than separate pages.

## Troubleshooting

### Docker Image Not Found

**Error**: `Docker image not found: testcase-manager-docs:latest`

**Solution**:
```bash
# Build the Docker image first
make docs-docker-build
```

### PDF File Size Too Small

**Error**: `PDF file is too small (1MB < 2MB)`

**Possible causes**:
- Build process interrupted
- WeasyPrint errors during generation
- Missing documentation content
- Incomplete dependency installation

**Solution**:
```bash
# Check build output for errors
make docs-docker-build-pdf

# Rebuild Docker image if needed
make docs-docker-clean
make docs-docker-build
```

### WeasyPrint Errors

**Error**: WeasyPrint warnings or errors in build output

**Common issues**:
- Missing fonts
- Image loading problems
- CSS rendering issues

**Solution**:
```bash
# Check Dockerfile.mkdocs has all required system dependencies
grep -A 10 "apt-get install" Dockerfile.mkdocs

# Required packages:
# - libcairo2
# - libpango-1.0-0
# - libpangocairo-1.0-0
# - libgdk-pixbuf2.0-0
# - libffi-dev
```

### pdfinfo Not Available

**Error**: `pdfinfo command failed`

**Solution**:
The test automatically installs poppler-utils in an alpine container. If this fails, check Docker network connectivity:

```bash
# Test alpine image pull
docker pull alpine

# Test package installation
docker run --rm alpine sh -c "apk add poppler-utils"
```

### Multiple Build Failures

**Error**: Second or third build fails

**Possible causes**:
- File permission issues
- Disk space exhaustion
- Docker volume mount problems

**Solution**:
```bash
# Clean up completely
make docs-clean
docker system prune -f

# Verify disk space
df -h

# Check Docker volume mounts
docker volume ls
```

## File Size Guidelines

### Expected Range: 2-10MB

- **Under 2MB**: Likely incomplete or missing content
- **2-5MB**: Normal for moderate documentation
- **5-10MB**: Normal for extensive documentation with images
- **Over 10MB**: May indicate issues or very large documentation

### Factors Affecting Size

1. **Number of pages**: More content = larger file
2. **Images**: High-resolution images increase size significantly
3. **Code blocks**: Syntax highlighting adds to file size
4. **Fonts**: Embedded fonts contribute to overall size
5. **Optimization**: Non-optimized PDFs are larger

## Integration with CI/CD

This test can be integrated into CI/CD pipelines:

### GitLab CI Example

```yaml
test:docker-pdf:
  stage: test
  image: docker:latest
  services:
    - docker:dind
  before_script:
    - make docs-docker-build
  script:
    - make docs-docker-test-pdf
  artifacts:
    paths:
      - site/pdf/testcase-manager-documentation.pdf
    expire_in: 1 week
```

### GitHub Actions Example

```yaml
- name: Test Docker PDF Generation
  run: |
    make docs-docker-build
    make docs-docker-test-pdf
- name: Upload PDF Artifact
  uses: actions/upload-artifact@v3
  with:
    name: documentation-pdf
    path: site/pdf/testcase-manager-documentation.pdf
```

## Related Tests

- `test_docker_mkdocs_e2e.sh`: Tests Docker image build and dependencies
- `test_docker_html_build_e2e.sh`: Tests HTML site generation in Docker
- `test-mkdocs-setup.sh`: Tests local MkDocs installation and setup

## Dependencies

### System Dependencies (in Docker)
- Python 3.12
- libcairo2
- libpango-1.0-0
- libpangocairo-1.0-0
- libgdk-pixbuf2.0-0
- libffi-dev
- shared-mime-info

### Python Dependencies
- mkdocs >= 1.5.0
- mkdocs-material >= 9.5.0
- mkdocs-with-pdf >= 0.9.3
- markdown >= 3.5
- pymdown-extensions >= 10.7

### Test Dependencies
- Docker Engine
- Alpine Linux (for pdfinfo validation)
- poppler-utils (installed in test container)

## Best Practices

1. **Run Prerequisites First**: Always build Docker image before testing
2. **Check Disk Space**: Ensure sufficient space for site/ directory and PDF
3. **Review Logs**: Use `--verbose` flag when debugging issues
4. **Clean Between Tests**: Run `make docs-clean` between test runs for consistency
5. **Verify Configuration**: Check mkdocs.yml PDF settings before testing
6. **Monitor Build Time**: PDF generation can take several minutes
7. **Validate Output**: Always verify PDF opens correctly after generation

## Exit Codes

- `0`: All tests passed successfully
- `1`: One or more tests failed
- `1`: Prerequisites not met (Docker not installed, image missing, etc.)

## Maintenance

This test should be updated when:
- PDF plugin configuration changes
- New exclusion patterns are added
- Expected file size range changes
- PDF structure requirements change
- Docker image dependencies are updated
