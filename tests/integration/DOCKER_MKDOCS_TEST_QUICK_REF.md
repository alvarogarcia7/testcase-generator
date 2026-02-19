# Docker MkDocs Test Quick Reference

## Commands

```bash
# Run full test suite
make docs-docker-test

# Keep temp files for debugging
./tests/integration/test_docker_mkdocs_e2e.sh --no-remove

# Clean and rebuild
make docs-docker-clean && make docs-docker-test
```

## Test Checklist

- [ ] Prerequisites (Docker, Dockerfile)
- [ ] Dockerfile syntax and best practices
- [ ] Image builds successfully
- [ ] Image tagged as testcase-manager-docs:latest
- [ ] Image size < 1GB
- [ ] Python packages: mkdocs, mkdocs-material, mkdocs-with-pdf
- [ ] System libs: libcairo2, libpango, libffi-dev
- [ ] Non-root user 'mkdocs' (UID 1000)
- [ ] ENABLE_PDF_EXPORT=0 by default
- [ ] Docker inspect shows correct config
- [ ] Documentation builds successfully
- [ ] Generated HTML is valid

## Validation Points

### Dockerfile Best Practices
```bash
# Should have
FROM python:3.12-slim          # Specific version
WORKDIR /docs                  # Set working directory
RUN apt-get ... && rm -rf ...  # Cleanup in same layer
pip install --no-cache-dir     # No pip cache
useradd -m -u 1000 mkdocs      # Non-root user
USER mkdocs                    # Switch to non-root
EXPOSE 8000                    # Document port
ENV ENABLE_PDF_EXPORT=0        # Default environment
```

### Required Dependencies

**Python:**
- mkdocs >= 1.5.0
- mkdocs-material >= 9.5.0
- mkdocs-with-pdf >= 0.9.3
- markdown >= 3.5
- pymdown-extensions >= 10.7

**System:**
- libcairo2 (Cairo graphics)
- libpango-1.0-0 (Pango text)
- libpangocairo-1.0-0 (Pango Cairo)
- libgdk-pixbuf2.0-0 (GDK Pixbuf)
- libffi-dev (FFI development)
- shared-mime-info (MIME types)

## Quick Verification

```bash
# Check image exists
docker images testcase-manager-docs:latest

# Check image size
docker images testcase-manager-docs:latest --format "{{.Size}}"

# Check Python packages
docker run --rm testcase-manager-docs:latest pip list | grep mkdocs

# Check system packages
docker run --rm testcase-manager-docs:latest dpkg -l | grep -E "cairo|pango|ffi"

# Check user
docker run --rm testcase-manager-docs:latest whoami
# Should output: mkdocs

# Check environment
docker run --rm testcase-manager-docs:latest bash -c 'echo $ENABLE_PDF_EXPORT'
# Should output: 0

# Test build
docker run --rm \
  -v "$(pwd)/docs:/docs/docs" \
  -v "$(pwd)/mkdocs.yml:/docs/mkdocs.yml" \
  -v "$(pwd)/site:/docs/site" \
  testcase-manager-docs:latest
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Docker not running | `docker info` to check, start Docker |
| Build fails | Check network, verify base image |
| Image too large | Review installed packages, multi-stage build |
| Missing Python pkg | Update requirements.txt |
| Missing system lib | Add to apt-get install |
| Permission errors | Verify chown command |
| PDF fails | Check Cairo/Pango libs |
| Wrong user | Check USER directive |

## Expected Results

**Image Size:** ~500-800 MB (< 1GB limit)

**Build Time:** 
- First build: 3-5 minutes
- Cached build: 10-30 seconds

**Test Duration:** ~30-60 seconds

## CI/CD Integration

```yaml
# GitLab CI
docker-docs-test:
  stage: test
  script:
    - make docs-docker-test
  only:
    changes:
      - Dockerfile.mkdocs
      - requirements.txt
      - mkdocs.yml
```

```yaml
# GitHub Actions
- name: Test Docker MkDocs
  run: make docs-docker-test
```

## Related Files

- `Dockerfile.mkdocs` - Docker image definition
- `requirements.txt` - Python dependencies
- `mkdocs.yml` - MkDocs configuration
- `tests/integration/test_docker_mkdocs_e2e.sh` - Test script
- `tests/integration/README_DOCKER_MKDOCS_TEST.md` - Full documentation
