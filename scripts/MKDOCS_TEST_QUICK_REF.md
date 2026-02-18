# MkDocs Test Quick Reference

## Quick Commands

```bash
# Full test with clean install
make docs-test-clean

# Standard test (assumes venv exists)
make docs-test

# Quick test (skip serve and unit tests)
make docs-test-quick

# Direct script access
./scripts/test-mkdocs-setup.sh [OPTIONS]
```

## Test Flags

| Flag | Description |
|------|-------------|
| `--skip-install` | Skip MkDocs installation |
| `--skip-serve` | Skip local server test (for CI/CD) |
| `--skip-build` | Skip HTML build test |
| `--skip-pdf` | Skip PDF generation test |
| `--skip-links` | Skip link validation |
| `--skip-tests` | Skip unit tests |
| `--clean` | Clean artifacts before testing |

## What Gets Tested

1. ✓ MkDocs installation
2. ✓ Virtual environment creation
3. ✓ Local server (port 8000)
4. ✓ HTML site build
5. ✓ Site structure validation
6. ✓ PDF generation
7. ✓ PDF structure
8. ✓ Internal links
9. ✓ Unit tests

## Common Use Cases

### CI/CD Pipeline
```bash
./scripts/test-mkdocs-setup.sh --skip-serve
```

### Quick Validation
```bash
./scripts/test-mkdocs-setup.sh --skip-serve --skip-tests
```

### After Documentation Changes
```bash
make docs-test-quick
```

### Before Commit
```bash
make docs-test-clean
```

## Outputs

- ✓ Green = Passed
- ✗ Red = Failed  
- ℹ Blue = Info
- ⚠ Orange = Warning

## Exit Codes

- `0` = All tests passed
- `1` = One or more tests failed

## File Locations

- Virtual Environment: `mkdocs-venv/`
- HTML Output: `site/`
- PDF Output: `site/pdf/testcase-manager-documentation.pdf`

## Requirements

- Python 3.8+
- curl
- make

Optional:
- pdfinfo (for detailed PDF validation)

## Troubleshooting

**Port 8000 in use?**
```bash
./scripts/test-mkdocs-setup.sh --skip-serve
```

**Venv not found?**
```bash
make docs-install
./scripts/test-mkdocs-setup.sh --skip-install
```

**PDF not generated?**
```bash
# Check plugin installed
mkdocs-venv/bin/pip list | grep mkdocs-with-pdf

# Rebuild
make docs-build-pdf
```

## Full Documentation

See `scripts/README_MKDOCS_TEST.md` for complete documentation.
