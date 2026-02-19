# Docker HTML Build Test - Quick Reference

## Quick Commands

```bash
# Run the test
./tests/integration/test_docker_html_build_e2e.sh

# With verbose output
./tests/integration/test_docker_html_build_e2e.sh --verbose

# Keep temp files for debugging
./tests/integration/test_docker_html_build_e2e.sh --no-remove

# Prerequisites
make docs-docker-build  # Build Docker image first
```

## What Gets Tested

| Test | What It Checks | Critical? |
|------|----------------|-----------|
| 1. Prerequisites | Docker installed, image exists, files present | ✓ Yes |
| 2. Clean Site | Removes existing site/ directory | ✓ Yes |
| 3. Docker Build | `make docs-docker-build-site` works | ✓ Yes |
| 4. Site Creation | site/ directory created with files | ✓ Yes |
| 5. HTML Files | index.html and other HTML files exist | ✓ Yes |
| 6. MD to HTML | Markdown files converted to HTML | ✓ Yes |
| 7. Assets | CSS, JS files copied correctly | ✓ Yes |
| 8. Permissions | Correct ownership from host perspective | ✓ Yes |
| 9. HTML Quality | Valid HTML structure and content | ✓ Yes |
| 10. Multiple Builds | Sequential builds work correctly | ✓ Yes |
| 11. Directory Listing | `ls -la site/` shows structure | No |
| 12. Cleanup | `make docs-clean` removes site/ | ✓ Yes |
| 13. Structure | Complete navigation and file structure | No |

## Expected Output

```bash
=== Test Summary ===
[INFO] Total tests: 13
[INFO] Tests passed: 13
[INFO] Tests failed: 0

[INFO] Final site/ directory statistics:
[INFO]   Total files: 100+
[INFO]   Total directories: 20+
[INFO]   HTML files: 30+
[INFO]   CSS files: 10+
[INFO]   JS files: 10+
[INFO]   Total size: 2-5M

✓ All Docker HTML build tests passed successfully!
```

## Common Issues & Fixes

### Issue: Docker Image Not Found
```bash
# Fix: Build the image
make docs-docker-build
```

### Issue: Permission Denied
```bash
# Fix: Check ownership
ls -la site/
# Should be owned by your user, not root

# Fix ownership (Linux only if needed)
sudo chown -R $USER:$USER site/
```

### Issue: Site Directory Not Created
```bash
# Fix: Check volume mounts in Makefile
# Should have: -v "$(PWD)/site:/docs/site"

# Test manually
docker run --rm \
  -v "$(PWD)/docs:/docs/docs" \
  -v "$(PWD)/mkdocs.yml:/docs/mkdocs.yml" \
  -v "$(PWD)/site:/docs/site" \
  testcase-manager-docs:latest mkdocs build
```

### Issue: Missing CSS/JS Files
```bash
# Fix: Verify Material theme in image
docker run --rm testcase-manager-docs:latest pip show mkdocs-material

# Rebuild image if needed
make docs-docker-clean
make docs-docker-build
```

### Issue: Multiple Builds Fail
```bash
# Fix: Clean between builds
make docs-clean
make docs-docker-build-site
```

## Verification Steps

### 1. Manual Verification After Test
```bash
# Check site exists
ls -la site/

# Check key files
ls site/index.html
ls site/404.html
ls -R site/assets/

# Check structure
find site/ -type f -name "*.html" | head -10
find site/ -type f -name "*.css" | head -5
find site/ -type f -name "*.js" | head -5
```

### 2. Test Content Quality
```bash
# Check HTML is valid
grep "<html" site/index.html
grep "<head>" site/index.html
grep "<body>" site/index.html

# Check for Material theme
grep -i "material" site/index.html

# Check for search
grep -i "search" site/index.html
ls site/search/search_index.json
```

### 3. Test Permissions
```bash
# Should be readable
cat site/index.html > /dev/null

# Should be writable
touch site/test.txt && rm site/test.txt
```

## Integration with Workflow

### Before Committing Changes
```bash
# 1. Build Docker image
make docs-docker-build

# 2. Test HTML build
./tests/integration/test_docker_html_build_e2e.sh

# 3. If tests pass, commit
git add .
git commit -m "Update documentation"
```

### In CI/CD Pipeline
```yaml
# GitLab CI
test-docker-html:
  script:
    - make docs-docker-build
    - ./tests/integration/test_docker_html_build_e2e.sh
```

## File Locations

| File | Purpose |
|------|---------|
| `tests/integration/test_docker_html_build_e2e.sh` | Main test script |
| `tests/integration/README_DOCKER_HTML_BUILD_TEST.md` | Detailed documentation |
| `tests/integration/DOCKER_HTML_BUILD_TEST_QUICK_REF.md` | This file |
| `Makefile` | Contains `docs-docker-build-site` target |
| `Dockerfile.mkdocs` | Docker image definition |
| `site/` | Generated HTML output (created by test) |

## Key Makefile Targets

```bash
# Build Docker image (run first)
make docs-docker-build

# Build site in Docker (what the test runs)
make docs-docker-build-site

# Clean site directory
make docs-clean

# Clean Docker image
make docs-docker-clean
```

## Success Criteria

✅ **Test passes when:**
- All 13 test sections complete without critical failures
- site/ directory is created with correct structure
- All markdown files converted to HTML
- Assets (CSS, JS) are present and accessible
- Files have correct ownership and permissions from host
- Multiple sequential builds work without errors
- Cleanup and rebuild cycle works correctly

❌ **Test fails when:**
- Docker not installed or not running
- Docker image not built
- site/ directory not created
- Missing HTML files
- Missing or corrupted assets
- Permission/ownership issues
- Sequential builds fail
- Cleanup doesn't work

## Tips

- **Always build Docker image first**: `make docs-docker-build`
- **Use verbose mode for debugging**: `--verbose`
- **Check site/ manually after test**: `ls -la site/`
- **Clean between tests if needed**: `make docs-clean`
- **Test takes 1-3 minutes**: Be patient with multiple builds
- **Check Docker logs on failure**: Look at build output

## Related Documentation

- Full test documentation: `tests/integration/README_DOCKER_HTML_BUILD_TEST.md`
- Docker MkDocs setup: `README_DOCKER_MKDOCS.md`
- Docker MkDocs test: `tests/integration/README_DOCKER_MKDOCS_TEST.md`
- Project commands: `AGENTS.md`
