# Docker Compose Test Quick Reference

## Quick Start

```bash
# Run complete test suite
make docs-compose-test

# Or run directly
./tests/integration/test_docker_compose_mkdocs_e2e.sh
```

## Test Options

```bash
# Verbose output (shows service logs)
./tests/integration/test_docker_compose_mkdocs_e2e.sh --verbose

# Keep temporary files for debugging
./tests/integration/test_docker_compose_mkdocs_e2e.sh --no-remove

# Both options combined
./tests/integration/test_docker_compose_mkdocs_e2e.sh --verbose --no-remove
```

## What It Tests

1. **Config Validation** - `docker-compose config` syntax check
2. **Service Definitions** - All 3 services (mkdocs, mkdocs-build, mkdocs-build-pdf)
3. **Volume Mounts** - All 5 volume mappings work correctly
4. **HTML Build** - `make docs-compose-build-site` generates site/
5. **PDF Build** - `make docs-compose-build-pdf` generates PDF with ENABLE_PDF_EXPORT=1
6. **Dev Server** - `make docs-compose-up` starts with live reload on port 8000
7. **Shutdown** - `make docs-compose-down` stops cleanly
8. **Image Sharing** - All services use testcase-manager-docs:latest

## Prerequisites

Must have:
- Docker (daemon running)
- docker-compose
- curl
- Image: `testcase-manager-docs:latest` (run `make docs-docker-build`)

## Common Issues

### Port 8000 in use
```bash
docker-compose -f docker-compose.mkdocs.yml down
```

### Image not found
```bash
make docs-docker-build
```

### Permission errors
```bash
rm -rf site/
chmod -R u+w .
```

## Output Locations

- **Test logs**: `/tmp/tmp.XXXXXX/`
- **HTML site**: `./site/`
- **PDF**: `./site/pdf/testcase-manager-documentation.pdf`
- **Service logs**: `docker-compose logs`

## Test Targets

20 comprehensive tests covering:
- Configuration syntax ✅
- Service definitions ✅
- Volume mounts (3 services) ✅
- HTML generation ✅
- PDF generation ✅
- Live reload server ✅
- Service lifecycle ✅
- Full workflow integration ✅

## Success Indicators

```
✓ docker-compose.mkdocs.yml syntax is valid
✓ mkdocs service starts with live reload
✓ mkdocs-build service generates site/ directory
✓ mkdocs-build-pdf service generates PDF with ENABLE_PDF_EXPORT=1
✓ Volume mounts work correctly for all three services
✓ make docs-compose-down stops services cleanly
✓ All services share the same image: testcase-manager-docs:latest
```

## Debugging

```bash
# View service logs
docker-compose -f docker-compose.mkdocs.yml logs mkdocs

# Check running containers
docker-compose -f docker-compose.mkdocs.yml ps

# Validate config manually
docker-compose -f docker-compose.mkdocs.yml config

# Test individual services
docker-compose -f docker-compose.mkdocs.yml run --rm mkdocs-build
docker-compose -f docker-compose.mkdocs.yml run --rm mkdocs-build-pdf
docker-compose -f docker-compose.mkdocs.yml up mkdocs
```

## Expected Duration

~3-5 minutes depending on documentation size

## Exit Codes

- `0` = All tests passed ✅
- `1` = Test failure ❌

## Related Tests

- `make docs-docker-test` - Docker image validation
- `make docs-docker-test-html` - HTML build test
- `make docs-docker-test-pdf` - PDF generation test
- `make docs-docker-test-serve` - Development server test

## CI/CD Integration

```yaml
test-compose:
  script:
    - make docs-docker-build
    - make docs-compose-test
```
