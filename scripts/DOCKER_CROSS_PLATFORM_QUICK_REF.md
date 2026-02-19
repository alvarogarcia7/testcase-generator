# Docker Cross-Platform Compatibility - Quick Reference

## Overview

Comprehensive cross-platform Docker compatibility testing for macOS and Linux.

## Quick Commands

```bash
# Run cross-platform test
make docs-docker-test-cross-platform

# Or directly
./tests/integration/test_docker_cross_platform_e2e.sh

# With options
./tests/integration/test_docker_cross_platform_e2e.sh --verbose --no-remove
```

## What's Tested

### ✓ Build Compatibility
- `make docs-docker-build` works identically on macOS and Linux
- Image builds produce consistent results
- Container user configuration (non-root)

### ✓ Volume Mounts
- `make docs-docker-serve` volume mounts work on both platforms
- `make docs-docker-build-site` generates files correctly
- Files are accessible from host on both platforms

### ✓ Permissions
- site/ directory permissions correct on macOS and Linux
- Host can read/write/delete generated files
- No sudo required for cleanup

### ✓ Script Compatibility
- docker-mkdocs.sh works with BSD utilities (macOS)
- docker-mkdocs.sh works with GNU utilities (Linux)
- No bash 4.0+ features (compatible with macOS bash 3.2)

### ✓ Docker Compose
- docker-compose.mkdocs.yml syntax valid
- `make docs-compose-build-site` works on both platforms
- All compose services work correctly

### ✓ Path Handling
- No path separator issues (/ vs \)
- Makefile uses $(PWD) for cross-platform paths
- Docker Compose uses relative paths

### ✓ Platform-Specific Features
- macOS: Docker Desktop, osxfs, BSD utilities
- Linux: Native Docker, direct mounts, GNU utilities
- WSL: Detected and tested appropriately

## Platform Differences (Expected)

### macOS with Docker Desktop

**Characteristics:**
- Uses osxfs for volume mounts
- BSD command-line tools (sed, grep, stat, find)
- bash 3.2 by default
- Files appear owned by host user (osxfs handles this)
- Slightly slower volume mount performance

**Commands:**
```bash
# Check platform
uname -s  # Darwin

# Check Docker Desktop
docker info | grep "Docker Desktop"

# Check tool variants
sed --version 2>&1 | head -1   # BSD
grep --version 2>&1 | head -1  # BSD
```

### Linux with Native Docker

**Characteristics:**
- Direct volume mounts with host filesystem
- GNU command-line tools
- Modern bash (typically 4.0+)
- Files owned by container UID (typically 1000)
- Native performance

**Commands:**
```bash
# Check platform
uname -s  # Linux

# Check if in WSL
grep -i microsoft /proc/version

# Check SELinux
getenforce  # Enforcing, Permissive, or Disabled
```

## Portability Requirements

### Shell Script Compatibility

✓ **Do Use:**
```bash
sed -E                    # Works on BSD and GNU
grep with POSIX regex     # Portable regex
$(cd "$(dirname "$0")" && pwd)  # Portable dir detection
case/esac for logic      # Bash 3.2+ compatible
```

❌ **Don't Use:**
```bash
sed -r                   # GNU-only (use sed -E)
grep -P                  # Perl regex, GNU-only
readlink -f              # GNU-only
declare -A               # Bash 4.0+ (use eval)
```

### Platform Detection

```bash
# Detect OS
OS_TYPE="$(uname -s)"
case "$OS_TYPE" in
    Linux*)   PLATFORM="linux" ;;
    Darwin*)  PLATFORM="macos" ;;
    *)        PLATFORM="unknown" ;;
esac

# Detect stat variant
if stat --version 2>/dev/null | grep -q "GNU"; then
    STAT_FORMAT="-c %s"  # GNU
else
    STAT_FORMAT="-f %z"  # BSD
fi
```

## Common Issues and Solutions

### Issue: Port Already in Use

**Error:**
```
Port 8000 is already in use
```

**Solution:**
```bash
# macOS
lsof -ti:8000 | xargs kill

# Linux
netstat -tulpn | grep 8000
# Or
ss -tulpn | grep 8000
```

### Issue: Permission Denied

**Error:**
```
rm: site/: Permission denied
```

**Solution:**
```bash
# Fix permissions
chmod -R u+w site/
rm -rf site/

# Or force removal
sudo rm -rf site/
```

### Issue: SELinux (Linux Only)

**Error:**
```
Permission denied in container
```

**Solution:**
```bash
# Check SELinux status
getenforce

# Add :z flag to volume mounts
docker run -v ./docs:/docs/docs:z ...

# Or set SELinux to permissive
sudo setenforce 0
```

### Issue: Docker Not Running

**Error:**
```
Cannot connect to Docker daemon
```

**Solution:**
```bash
# macOS
open -a Docker

# Linux
sudo systemctl start docker
sudo systemctl enable docker
```

## Test Output

### Success Output
```
✓ Platform detected: macOS
✓ Docker is installed
✓ Docker image built successfully
✓ Volume mounts work correctly
✓ File permissions are correct
✓ No path separator issues
✓ All 15 tests passed
```

### Compatibility Report
```
Docker Cross-Platform Compatibility Report
==========================================

Platform Information:
- Operating System: macOS (Darwin)
- Architecture: arm64
- Docker Desktop: Yes

Utility Variants:
- sed: BSD
- grep: BSD
- stat: BSD

Test Results:
- Tests Passed: 15
- Tests Failed: 0
```

## Quick Checks

### Verify Docker Build
```bash
# Build image
make docs-docker-build

# Check image
docker images testcase-manager-docs:latest

# Test image
docker run --rm testcase-manager-docs:latest mkdocs --version
```

### Verify Volume Mounts
```bash
# Build site
make docs-docker-build-site

# Check output
ls -la site/
cat site/index.html | head

# Test permissions
echo "test" > site/.test
rm site/.test
```

### Verify Compose
```bash
# Validate syntax
docker-compose -f docker-compose.mkdocs.yml config

# Build with compose
make docs-compose-build-site

# Check output
ls -la site/
```

## CI/CD Integration

### GitLab CI
```yaml
test:docker-cross-platform:
  stage: test
  script:
    - make docs-docker-test-cross-platform
  artifacts:
    when: always
    paths:
      - /tmp/test_*/compatibility_report.txt
```

### GitHub Actions
```yaml
- name: Docker Cross-Platform Test
  run: make docs-docker-test-cross-platform

- name: Upload Compatibility Report
  if: always()
  uses: actions/upload-artifact@v3
  with:
    name: compatibility-report
    path: /tmp/test_*/compatibility_report.txt
```

## Performance Notes

### macOS Volume Mount Performance
- Docker Desktop uses osxfs (may be slower)
- Typical build time: 30-60 seconds
- Consider using cached builds for development

### Linux Native Performance
- Direct volume mounts (fast)
- Typical build time: 15-30 seconds
- No virtualization overhead

## Related Commands

```bash
# View all Docker tests
make docs-docker-test          # Basic Docker test
make docs-docker-test-volumes  # Volume permissions test
make docs-compose-test         # Docker Compose test
make docs-docker-test-cross-platform  # This test

# Clean up
make docs-docker-clean         # Remove Docker image
make docs-clean               # Remove site/ directory
rm -rf site/                  # Manual cleanup
```

## Documentation

- Full guide: `tests/integration/README_DOCKER_CROSS_PLATFORM_TEST.md`
- Quick ref: `tests/integration/DOCKER_CROSS_PLATFORM_TEST_QUICK_REF.md`
- AGENTS.md: Build/lint/test requirements

## Exit Codes

- `0` - All tests passed
- `1` - One or more tests failed
- `2+` - Script error or prerequisites not met

## Support Matrix

| Platform | Docker | Compose | Status |
|----------|--------|---------|--------|
| macOS (Intel) | ✓ | ✓ | Fully supported |
| macOS (Apple Silicon) | ✓ | ✓ | Fully supported |
| Linux (x86_64) | ✓ | ✓ | Fully supported |
| Linux (ARM64) | ✓ | ✓ | Fully supported |
| WSL 2 | ✓ | ✓ | Fully supported |
| Windows (native) | - | - | Not tested |

## Troubleshooting Checklist

- [ ] Docker is installed and running
- [ ] Docker Desktop started (macOS)
- [ ] Port 8000 is available
- [ ] No conflicting containers running
- [ ] Sufficient disk space (>2GB)
- [ ] User has Docker permissions (Linux: in docker group)
- [ ] SELinux not blocking (Linux)
- [ ] File sharing enabled (macOS: Docker Desktop preferences)
