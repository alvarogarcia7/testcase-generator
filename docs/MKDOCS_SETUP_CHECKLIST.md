# MkDocs Setup Checklist

Use this checklist to verify your MkDocs documentation setup is working correctly.

## Prerequisites

- [ ] Python 3.8+ installed
- [ ] curl installed (for testing HTTP endpoints)
- [ ] make installed
- [ ] Repository cloned locally

## Installation

- [ ] Run `make docs-install`
- [ ] Verify no errors during installation
- [ ] Check virtualenv created at `mkdocs-venv/`
- [ ] Verify MkDocs executable: `mkdocs-venv/bin/mkdocs --version`

## Local Development Server

- [ ] Run `make docs-serve`
- [ ] Open browser to http://localhost:8000
- [ ] Verify home page loads
- [ ] Check navigation menu appears
- [ ] Test "Getting Started" link works
- [ ] Test "User Guide" link works
- [ ] Test "CLI Tools" link works
- [ ] Test "Features" link works
- [ ] Test "Development" link works
- [ ] Test "Examples" link works
- [ ] Verify page content renders correctly
- [ ] Test search functionality
- [ ] Test light/dark theme toggle
- [ ] Stop server (Ctrl+C)

## HTML Build

- [ ] Run `make docs-build`
- [ ] Verify no errors during build
- [ ] Check `site/` directory created
- [ ] Verify `site/index.html` exists
- [ ] Open `site/index.html` in browser
- [ ] Test navigation works in built site
- [ ] Verify all pages accessible
- [ ] Check assets loaded (CSS, JS, images)

## Site Structure

- [ ] Verify `site/index.html` exists
- [ ] Check `site/getting-started/` directory
- [ ] Check `site/user-guide/` directory
- [ ] Check `site/cli-tools/` directory
- [ ] Check `site/features/` directory
- [ ] Check `site/development/` directory
- [ ] Check `site/examples/` directory
- [ ] Verify CSS/JS directories exist
- [ ] Check assets directory exists

## PDF Generation

- [ ] Run `make docs-build-pdf`
- [ ] Verify no errors during build
- [ ] Check PDF exists at `site/pdf/testcase-manager-documentation.pdf`
- [ ] Verify PDF is not empty (file size > 0)
- [ ] Open PDF in viewer
- [ ] Verify cover page appears
- [ ] Check table of contents included
- [ ] Verify all sections present in PDF
- [ ] Test internal PDF links work
- [ ] Check page numbers appear
- [ ] Verify formatting looks correct

## Link Validation

- [ ] Run automated link check (if available)
- [ ] Manually click navigation links
- [ ] Test internal page links
- [ ] Verify cross-references work
- [ ] Check anchor links (#section)
- [ ] Test links in tables
- [ ] Verify code block links
- [ ] Check external links (optional)

## Content Validation

- [ ] Verify home page content accurate
- [ ] Check Getting Started guide complete
- [ ] Review User Guide sections
- [ ] Validate CLI Tools documentation
- [ ] Check Features documentation
- [ ] Review Development guides
- [ ] Verify Examples section
- [ ] Check code snippets render correctly
- [ ] Verify tables display properly
- [ ] Check admonitions (notes, warnings) render
- [ ] Test syntax highlighting works

## Automated Testing

- [ ] Run `make docs-test-quick` (fast validation)
- [ ] Run `make docs-test` (full test)
- [ ] Run `make docs-test-clean` (clean install test)
- [ ] Verify all tests pass
- [ ] Review any warnings
- [ ] Check test output for errors

## Unit Tests

- [ ] Run `make test`
- [ ] Verify all unit tests pass
- [ ] Check no regressions introduced
- [ ] Review test output

## Clean Up

- [ ] Run `make docs-clean` to remove build artifacts
- [ ] Verify `site/` directory removed
- [ ] Test rebuild works after clean

## CI/CD Integration (Optional)

- [ ] Add test to CI/CD pipeline
- [ ] Use `--skip-serve` flag in CI/CD
- [ ] Verify pipeline runs successfully
- [ ] Check artifacts uploaded (HTML, PDF)
- [ ] Test deployment to GitHub/GitLab Pages

## Common Issues

### Port 8000 in Use
- [ ] Check if another process using port 8000
- [ ] Stop conflicting process or use `--skip-serve`
- [ ] Run: `lsof -i :8000` to find process

### Virtual Environment Issues
- [ ] Remove `mkdocs-venv/` directory
- [ ] Run `make docs-install --clean`
- [ ] Verify Python version >= 3.8

### PDF Not Generated
- [ ] Check `mkdocs-with-pdf` plugin installed
- [ ] Run: `mkdocs-venv/bin/pip list | grep mkdocs-with-pdf`
- [ ] Verify `ENABLE_PDF_EXPORT=1` environment variable set
- [ ] Check for build errors in output

### Broken Links
- [ ] Review link warnings from test script
- [ ] Manually verify reported broken links
- [ ] Update links in documentation
- [ ] Rebuild and retest

## Best Practices

- [ ] Test after every documentation change
- [ ] Use `make docs-test-quick` during development
- [ ] Run full test suite before committing
- [ ] Review PDF output periodically
- [ ] Keep virtual environment updated
- [ ] Clean build artifacts regularly

## Sign-Off

Date: _________________

Tested by: _________________

Results:
- [ ] All checks passed
- [ ] Issues found (document below)

Issues/Notes:
_________________________________________________________________
_________________________________________________________________
_________________________________________________________________
_________________________________________________________________

## Related Documentation

- [MkDocs Test Guide](../scripts/README_MKDOCS_TEST.md)
- [MkDocs Quick Reference](../scripts/MKDOCS_TEST_QUICK_REF.md)
- [MkDocs Guide](MKDOCS_GUIDE.md)
- [AGENTS.md](../AGENTS.md)

## Support

For help with setup:
1. Review troubleshooting in test documentation
2. Check error messages in test output
3. Verify all requirements installed
4. Test commands individually
