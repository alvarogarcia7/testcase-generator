# Docker MkDocs Config Validation Quick Reference

## Quick Start

```bash
# Build Docker image first
make docs-docker-build

# Run configuration validation test
make docs-docker-test-config
```

## What It Tests

✅ **Configuration Validation**
- `mkdocs build --strict` passes without errors
- All configuration values are valid

✅ **Navigation Files**
- All nav entries point to existing files
- No broken documentation links

✅ **Material Theme**
- Theme loads without errors
- Features properly configured
- Palette settings valid

✅ **Markdown Extensions**
- `pymdownx.highlight` - Code syntax highlighting
- `pymdownx.superfences` - Enhanced code blocks & Mermaid
- `admonition` - Callout boxes
- `tables` - GitHub-style tables  
- `toc` - Table of contents

✅ **PDF Plugin**
- Conditional enablement via env var
- Exclusion patterns working
- Python module imports

✅ **Search Plugin**
- Language and separator configured
- Module loads correctly

✅ **Build Validation**
- Complete build succeeds
- Strict mode passes (no warnings)
- Output files generated

## Commands

```bash
# Run test
make docs-docker-test-config

# Run with temp file preservation
./tests/integration/test_docker_mkdocs_config_validation_e2e.sh --no-remove

# Direct Docker validation
docker run --rm testcase-manager-docs:latest mkdocs build --strict
```

## Common Issues

### Image Not Found
```bash
make docs-docker-build
```

### Missing Nav Files
Edit `mkdocs.yml` or create missing files

### Extension Errors
1. Update `requirements.txt`
2. Rebuild image: `make docs-docker-build`
3. Rerun test: `make docs-docker-test-config`

## Test Output

- ✓ Green = Pass
- ✗ Red = Fail
- ℹ Blue = Info
- Yellow = Section

## Test Sections

1. Prerequisites Check
2. Strict Mode Validation
3. Navigation Path Verification
4. Material Theme Check
5. Markdown Extensions
6. pymdownx.highlight Details
7. pymdownx.superfences Details
8. Admonitions Check
9. Tables Extension
10. TOC Configuration
11. PDF Plugin Setup
12. Search Plugin Setup
13. Build Test
14. Strict Build Test
15. Command Access Test

## Exit Codes

- `0` = All tests passed ✓
- `1` = Tests failed ✗

## Related Commands

```bash
make docs-docker-build          # Build image
make docs-docker-test           # All Docker tests
make docs-docker-test-html      # HTML build test
make docs-docker-test-pdf       # PDF generation test
make docs-docker-test-serve     # Server test
```

## File Locations

- Test Script: `tests/integration/test_docker_mkdocs_config_validation_e2e.sh`
- Documentation: `scripts/README_DOCKER_CONFIG_VALIDATION_TEST.md`
- Config File: `mkdocs.yml`
- Dockerfile: `Dockerfile.mkdocs`

## CI/CD Integration

```yaml
test:mkdocs-config:
  script:
    - make docs-docker-build
    - make docs-docker-test-config
```

## Typical Runtime

30-60 seconds (depends on Docker performance)
