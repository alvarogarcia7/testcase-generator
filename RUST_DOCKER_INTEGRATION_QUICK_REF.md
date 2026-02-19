# Rust and Docker Integration Test - Quick Reference

## Run the Test

```bash
make rust-docker-integration-test
```

## What It Tests

1. ✅ `.dockerignore.mkdocs` excludes Rust artifacts (`target/`, `Cargo.lock`, `src/`)
2. ✅ `Dockerfile.mkdocs` is properly configured
3. ✅ `.gitignore` includes documentation artifacts (`site/`, `mkdocs-venv/`)
4. ✅ `make build` works correctly
5. ✅ `make test` passes
6. ✅ `make lint` passes
7. ✅ AGENTS.md documents all Docker commands
8. ✅ Docker builds don't interfere with Rust artifacts
9. ✅ Makefile has both Rust and Docker targets
10. ✅ No conflicts between Rust and Docker artifacts

## Command Options

```bash
# Verbose output
./tests/integration/test_rust_docker_integration_e2e.sh --verbose

# Keep temporary files for debugging
./tests/integration/test_rust_docker_integration_e2e.sh --no-remove

# Both options
./tests/integration/test_rust_docker_integration_e2e.sh --verbose --no-remove
```

## Quick Troubleshooting

### Test 1 Fails
**Problem**: `.dockerignore.mkdocs` missing Rust exclusions  
**Fix**: Add `target/`, `Cargo.lock`, `src/` to `.dockerignore.mkdocs`

### Test 3 Fails
**Problem**: `.gitignore` missing documentation exclusions  
**Fix**: Add `mkdocs-venv/`, `site/` to `.gitignore`

### Test 4-6 Fail
**Problem**: Rust build/test/lint issues  
**Fix**: Fix Rust errors first, then re-run

### Test 7 Fails
**Problem**: AGENTS.md missing Docker commands  
**Fix**: Update AGENTS.md with missing commands

### Test 8 Fails
**Problem**: Docker build includes Rust artifacts  
**Fix**: Verify `.dockerignore.mkdocs` is properly configured

## Files Created

- `tests/integration/test_rust_docker_integration_e2e.sh` - Test script
- `tests/integration/README_RUST_DOCKER_INTEGRATION_TEST.md` - Detailed documentation
- `RUST_DOCKER_INTEGRATION_IMPLEMENTATION.md` - Implementation summary
- `RUST_DOCKER_INTEGRATION_QUICK_REF.md` - This file

## Files Modified

- `Makefile` - Added `rust-docker-integration-test` target
- `AGENTS.md` - Added documentation for new test command

## Exit Codes

- `0` - All tests passed ✅
- `1` - One or more tests failed ❌

## Expected Duration

- Without Docker: ~2-5 minutes
- With Docker: ~5-10 minutes

## CI/CD Integration

Add to your pipeline:

```yaml
rust_docker_integration:
  script:
    - make rust-docker-integration-test
```

## Related Documentation

- [Full Test Documentation](tests/integration/README_RUST_DOCKER_INTEGRATION_TEST.md)
- [Implementation Summary](RUST_DOCKER_INTEGRATION_IMPLEMENTATION.md)
- [AGENTS.md](AGENTS.md) - All available commands
- [Docker MkDocs README](README_DOCKER_MKDOCS.md)
