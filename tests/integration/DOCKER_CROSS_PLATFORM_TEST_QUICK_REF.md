# Docker Cross-Platform Compatibility Test - Quick Reference

## Quick Start

```bash
# Run the test
make docs-docker-test-cross-platform

# Or directly
./tests/integration/test_docker_cross_platform_e2e.sh

# With verbose output
./tests/integration/test_docker_cross_platform_e2e.sh --verbose

# Keep temp files for debugging
./tests/integration/test_docker_cross_platform_e2e.sh --no-remove
```

## What It Tests

✓ `make docs-docker-build` works on macOS and Linux  
✓ `make docs-docker-serve` volume mounts work on both platforms  
✓ site/ directory permissions correct on both platforms  
✓ docker-mkdocs.sh works with BSD and GNU utilities  
✓ Docker Compose commands work on both platforms  
✓ No path separator issues (/ vs \)  
✓ Platform-specific features work correctly  

## Platform Differences (Normal)

### macOS with Docker Desktop
- Uses osxfs for volume mounts (transparent permissions)
- BSD command-line tools (sed, grep, stat, find)
- bash 3.2 (no bash 4.0+ features)
- Files appear owned by host user
- Slightly slower volume mount performance

### Linux with Native Docker
- Direct volume mounts (files owned by container UID 1000)
- GNU command-line tools
- Modern bash (4.0+)
- Native performance
- May need SELinux flags (:z or :Z) if enforcing

## Common Commands

```bash
# View compatibility report
cat /tmp/test_*/compatibility_report.txt

# Clean up manually if needed
rm -rf site/
docker rmi testcase-manager-docs:latest

# Fix permissions if needed
chmod -R u+w site/ && rm -rf site/

# Check platform
uname -s  # Darwin (macOS) or Linux

# Check Docker
docker info | grep "Operating System"
```

## Key Test Sections

1. **Platform Detection** - Identifies OS, architecture, utilities
2. **Docker Build** - Tests image builds consistently
3. **Volume Mounts** - Tests file access and generation
4. **Permissions** - Tests read/write/delete from host
5. **Script Compatibility** - Tests helper scripts work on both
6. **Development Server** - Tests live reload works
7. **Docker Compose** - Tests compose commands if available
8. **Path Separators** - Verifies correct path syntax
9. **Platform-Specific** - Tests platform features
10. **Cleanup** - Tests file deletion without sudo

## Expected Results

All 15 test sections should pass:
- ✓ Platform detected correctly
- ✓ Docker prerequisites met
- ✓ Paths use forward slashes
- ✓ Docker image builds successfully
- ✓ Volume mounts work correctly
- ✓ File permissions correct for platform
- ✓ Helper script compatible
- ✓ Development server works
- ✓ Docker Compose works (if installed)
- ✓ No path separator issues
- ✓ Platform-specific checks pass
- ✓ Files can be deleted without sudo
- ✓ Compatibility report generated

## Troubleshooting

### Port Already in Use
```bash
# Find process using port 8000
lsof -ti:8000  # or netstat -an | grep 8000
# Kill the process or use different port
```

### Permission Denied
```bash
# Fix permissions
chmod -R u+w site/
rm -rf site/
```

### Docker Not Running
```bash
# macOS: Start Docker Desktop
open -a Docker

# Linux: Start Docker service
sudo systemctl start docker
```

### SELinux Issues (Linux)
```bash
# Check status
getenforce

# If enforcing, add :z to volume mounts
docker run -v ./docs:/docs/docs:z ...
```

## Compatibility Issues to Avoid

### ❌ Don't Use (Not Portable)
```bash
sed -r          # GNU-only (use sed -E)
grep -P         # GNU-only Perl regex
readlink -f     # GNU-only
declare -A      # bash 4.0+ (use eval with dynamic vars)
stat -c         # GNU-only (use -c on Linux, -f on macOS)
```

### ✓ Do Use (Portable)
```bash
sed -E          # Works on both BSD and GNU
grep with POSIX regex
$(cd "$(dirname "$0")" && pwd)  # Instead of readlink -f
eval for dynamic variables
Platform detection for stat
```

## Report Location

After test completes, find detailed report at:
```
$TEMP_DIR/compatibility_report.txt
```

Example: `/tmp/test_docker_cross_platform_XYZ/compatibility_report.txt`

## Integration Examples

### GitLab CI
```yaml
test:cross-platform:
  script:
    - make docs-docker-test-cross-platform
```

### GitHub Actions
```yaml
- name: Cross-Platform Test
  run: make docs-docker-test-cross-platform
```

### Pre-Push Hook
```bash
#!/usr/bin/env bash
make docs-docker-test-cross-platform
```

## Time Estimates

- First run (with image build): 5-8 minutes
- Subsequent runs: 3-5 minutes
- With verbose output: +1 minute

## Exit Codes

- `0` - All tests passed
- `1` - Tests failed
- Other - Prerequisites not met or error

## Related Documentation

- Full guide: `README_DOCKER_CROSS_PLATFORM_TEST.md`
- Docker tests: `README_DOCKER_MKDOCS_TEST.md`
- Volume tests: `README_DOCKER_VOLUME_PERMISSIONS_TEST.md`
- Compose tests: `README_DOCKER_COMPOSE_TEST.md`
