# Docker PDF Build Test - Quick Reference

## Quick Start

```bash
# Prerequisites
make docs-docker-build

# Run test
make docs-docker-test-pdf

# Or run directly
./tests/integration/test_docker_pdf_build_e2e.sh
```

## Command Options

```bash
# Basic run
./tests/integration/test_docker_pdf_build_e2e.sh

# Keep temp files (debugging)
./tests/integration/test_docker_pdf_build_e2e.sh --no-remove

# Verbose output
./tests/integration/test_docker_pdf_build_e2e.sh --verbose

# Both options
./tests/integration/test_docker_pdf_build_e2e.sh --no-remove --verbose
```

## Test Phases

| Phase | Tests | What It Does |
|-------|-------|--------------|
| **Prerequisites** | 1-2 | Checks Docker, image, config files |
| **Build & Generation** | 3-7 | Runs PDF build, validates output |
| **Content Validation** | 8-12 | Checks PDF structure, TOC, exclusions |
| **Reliability** | 13-16 | Tests rebuilds, cleanup, recreation |

## Key Validations

### ✓ PDF Generation
- Runs `make docs-docker-build-pdf` with `ENABLE_PDF_EXPORT=1`
- PDF created at `site/pdf/testcase-manager-documentation.pdf`
- No WeasyPrint errors during build

### ✓ PDF Quality
- File size: 2-10MB (configurable)
- Valid PDF format (verified with `file` command)
- Minimum 10 pages
- Contains metadata (title, author)

### ✓ Content Structure
- Table of Contents with 3 levels depth
- Major sections present (Getting Started, User Guide, etc.)
- Implementation notes excluded (IMPLEMENTATION_*.md, *_SUMMARY.md)

### ✓ Validation Tools
- `pdfinfo` - PDF metadata and structure
- `pdftotext` - Text extraction and content verification
- `file` command - Format validation

## Expected Results

### Success
```
✓ All Docker PDF generation tests passed successfully!
[INFO] Total tests: 16
[INFO] Tests passed: 16
[INFO] Tests failed: 0
Exit code: 0
```

### Failure
```
✗ Some Docker PDF generation tests failed!
[ERROR] N test(s) failed
Exit code: 1
```

## Common Issues

### Issue: Docker image not found
**Solution**: `make docs-docker-build`

### Issue: PDF too small (< 2MB)
**Causes**: Incomplete build, WeasyPrint errors, missing content
**Solution**: Check build output, rebuild Docker image

### Issue: WeasyPrint errors
**Causes**: Missing fonts, image problems, CSS issues
**Solution**: Verify system dependencies in Dockerfile.mkdocs

### Issue: pdfinfo fails
**Causes**: Network issues, Docker problems
**Solution**: Test `docker pull alpine` and `apk add poppler-utils`

### Issue: Multiple builds fail
**Causes**: Permissions, disk space, volume mounts
**Solution**: `make docs-clean && docker system prune -f`

## File Locations

```
site/
└── pdf/
    └── testcase-manager-documentation.pdf   # Generated PDF

tests/integration/
├── test_docker_pdf_build_e2e.sh            # Test script
├── README_DOCKER_PDF_BUILD_TEST.md         # Full docs
└── DOCKER_PDF_BUILD_TEST_QUICK_REF.md     # This file
```

## Configuration

### mkdocs.yml Settings
```yaml
plugins:
  - with-pdf:
      enabled_if_env: ENABLE_PDF_EXPORT
      output_path: pdf/testcase-manager-documentation.pdf
      toc_level: 3
      excludes_children:
        - 'IMPLEMENTATION_*.md'
        - '*_SUMMARY.md'
```

### Makefile Target
```makefile
docs-docker-build-pdf:
	docker run --rm -e ENABLE_PDF_EXPORT=1 \
		-v "$(PWD)/site:/docs/site" \
		testcase-manager-docs:latest
```

## Test Coverage Matrix

| Category | Test Count | Coverage |
|----------|-----------|----------|
| Prerequisites | 2 | Docker, config files |
| Build Process | 5 | Execution, errors, directories |
| PDF Quality | 4 | Format, size, metadata |
| Content | 3 | TOC, sections, exclusions |
| Reliability | 2 | Rebuilds, cleanup |
| **Total** | **16** | **Complete workflow** |

## Size Guidelines

| Range | Status | Meaning |
|-------|--------|---------|
| < 2MB | ⚠️ Warning | Possibly incomplete |
| 2-5MB | ✓ Good | Normal for moderate docs |
| 5-10MB | ✓ Good | Normal for extensive docs |
| > 10MB | ⚠️ Warning | May have issues |

## Excluded Patterns

The test verifies these are NOT in the PDF:
- `IMPLEMENTATION_*.md`
- `*_SUMMARY.md`
- Specific implementation files listed in mkdocs.yml

## Dependencies

### Required for Test
- Docker (running)
- testcase-manager-docs:latest image
- mkdocs.yml configuration
- docs/ directory with content

### Required in Docker Image
- Python 3.12
- mkdocs >= 1.5.0
- mkdocs-material >= 9.5.0
- mkdocs-with-pdf >= 0.9.3
- System libraries: libcairo2, libpango, libffi-dev

### Required for Validation
- Alpine Linux (pulled by test)
- poppler-utils (installed by test)

## CI/CD Integration

### GitLab CI
```yaml
test:docker-pdf:
  script:
    - make docs-docker-build
    - make docs-docker-test-pdf
  artifacts:
    paths:
      - site/pdf/testcase-manager-documentation.pdf
```

### GitHub Actions
```yaml
- run: make docs-docker-build
- run: make docs-docker-test-pdf
- uses: actions/upload-artifact@v3
  with:
    name: pdf
    path: site/pdf/*.pdf
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | All tests passed |
| 1 | Tests failed or prerequisites not met |

## Related Commands

```bash
# Build Docker image
make docs-docker-build

# Generate PDF only
make docs-docker-build-pdf

# Test Docker setup
make docs-docker-test

# Test HTML generation
make docs-docker-test-html

# Clean up
make docs-clean
make docs-docker-clean
```

## Timing

- **Image build**: 2-5 minutes (first time)
- **PDF generation**: 1-3 minutes
- **Full test run**: 5-10 minutes
- **Subsequent runs**: 2-5 minutes (cached)

## Best Practices

1. ✓ Build image before testing
2. ✓ Check disk space (need ~100MB+)
3. ✓ Use `--verbose` for debugging
4. ✓ Clean between tests for consistency
5. ✓ Verify PDF opens after generation
6. ✓ Monitor WeasyPrint warnings

## Support

- Full documentation: `README_DOCKER_PDF_BUILD_TEST.md`
- Docker test docs: `README_DOCKER_MKDOCS_TEST.md`
- HTML test docs: `README_DOCKER_HTML_BUILD_TEST.md`
- Logger library: `scripts/lib/logger.sh`
