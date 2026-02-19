# GitLab CI Pages Job Validation Test - Quick Reference

## Quick Start

```bash
# Run the test
make gitlab-ci-test

# Or directly
./tests/integration/test_gitlab_ci_pages_e2e.sh

# Keep temporary files for debugging
./tests/integration/test_gitlab_ci_pages_e2e.sh --no-remove
```

## What It Tests

| Test | Validates |
|------|-----------|
| 1. Prerequisites | Docker installed, daemon running, files exist |
| 2. YAML Syntax | .gitlab-ci.yml is valid YAML |
| 3. Docker Image | Uses python:3.11 image |
| 4. pip Install | requirements.txt installs in python:3.11 |
| 5. Environment | ENABLE_PDF_EXPORT=1 is set |
| 6. Build Command | mkdocs build --site-dir public |
| 7. Artifacts | public/ exported, 30 days expiration |
| 8. Cache | pip cache configured (~/.cache/pip) |
| 9. Branch | Only runs on main branch |
| 10. Full Simulation | Complete CI pipeline execution |

## Key Validations

### GitLab CI Configuration
- ✅ YAML syntax
- ✅ python:3.11 Docker image
- ✅ pip install from requirements.txt
- ✅ ENABLE_PDF_EXPORT=1 environment variable
- ✅ mkdocs build --site-dir public command
- ✅ Artifacts: public/ directory, 30 days expiration
- ✅ Cache: ~/.cache/pip
- ✅ Branch restriction: only main

### Container Environment
- ✅ Python 3.11 compatibility
- ✅ MkDocs installation
- ✅ PDF generation support

### Build Process
- ✅ Documentation builds successfully
- ✅ public/ directory created
- ✅ index.html generated
- ✅ PDF documentation generated

## Prerequisites

- Docker installed and running
- .gitlab-ci.yml file
- requirements.txt file
- mkdocs.yml configuration
- docs/ directory

## Common Issues

| Error | Solution |
|-------|----------|
| Docker not found | Install Docker and ensure it's in PATH |
| Docker daemon not running | Start Docker Desktop or Docker service |
| pip install fails | Check requirements.txt for errors |
| Build fails | Check mkdocs.yml and docs/ directory |

## Output

### Success
```
✓ All GitLab CI Pages job validation tests passed!
Total tests passed: 14
Total tests failed: 0
```

### Failure
```
✗ Some GitLab CI Pages job validation tests failed
Total tests passed: X
Total tests failed: Y
```

## Performance

- **Duration**: 30-60 seconds
- **Resources**: ~500MB disk space
- **Network**: Only for Docker image pulls

## Exit Codes

- `0` - All tests passed
- `1` - One or more tests failed

## See Also

- [Full Documentation](README_GITLAB_CI_TEST.md)
- [GitLab CI Configuration](.gitlab-ci.yml)
- [AGENTS.md](../AGENTS.md)
