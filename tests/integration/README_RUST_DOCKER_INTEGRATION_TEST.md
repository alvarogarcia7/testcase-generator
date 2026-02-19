# Rust and Docker Integration Test

End-to-end integration test for verifying that the Docker documentation setup properly integrates with the Rust project without any interference.

## Overview

This test suite validates that:
1. Documentation Docker commands don't interfere with the Rust build process
2. `.dockerignore.mkdocs` excludes Rust build artifacts (`target/`, `Cargo.lock`, `src/`)
3. `.dockerignore.mkdocs` is used by `Dockerfile.mkdocs`
4. `site/` and `mkdocs-venv/` are in `.gitignore`
5. Rust toolchain (`make build`, `make test`, `make lint`) remains unaffected by Docker documentation setup
6. AGENTS.md Docker commands documentation is accurate

## Quick Start

Run the integration test:

```bash
make rust-docker-integration-test
```

Or run directly:

```bash
./tests/integration/test_rust_docker_integration_e2e.sh
```

With verbose output:

```bash
./tests/integration/test_rust_docker_integration_e2e.sh --verbose
```

## Test Cases

### Test 1: Verify .dockerignore.mkdocs Excludes Rust Build Artifacts

Checks that `.dockerignore.mkdocs` contains:
- `target/` - Rust build output directory
- `Cargo.lock` - Rust dependency lock file
- `src/` - Rust source code

**Why This Matters**: Ensures Docker documentation builds don't copy unnecessary Rust artifacts, reducing build context size and build time.

### Test 2: Verify Dockerfile.mkdocs Uses .dockerignore.mkdocs

Verifies that `Dockerfile.mkdocs` exists and is properly configured to use `.dockerignore.mkdocs`.

**Why This Matters**: Ensures the Docker build process uses the correct ignore file for documentation builds.

### Test 3: Verify .gitignore Includes site/ and mkdocs-venv/

Checks that `.gitignore` contains:
- `mkdocs-venv/` - Python virtual environment for MkDocs
- `site/` - MkDocs build output directory

Also verifies Rust artifacts are gitignored:
- `target/` - Rust build output
- `Cargo.lock` - Rust dependency lock file (optional for applications)

**Why This Matters**: Prevents documentation build artifacts from being committed to the repository.

### Test 4: Run 'make build' to Verify Rust Toolchain Unaffected

Runs `make build` to ensure the Rust build process works correctly.

**Why This Matters**: Confirms that Docker documentation setup doesn't break the Rust build process.

### Test 5: Run 'make test' to Verify Rust Tests Unaffected

Runs `make test` to ensure all Rust tests pass.

**Why This Matters**: Confirms that Docker documentation setup doesn't interfere with Rust test execution.

### Test 6: Run 'make lint' to Verify Rust Lint Unaffected

Runs `make lint` to ensure Rust code quality checks pass.

**Why This Matters**: Confirms that Docker documentation setup doesn't affect Rust linting and formatting.

### Test 7: Verify AGENTS.md Docker Commands Documentation

Checks that AGENTS.md documents all Docker-related commands:
- `docs-docker-build` - Build Docker documentation image
- `docs-docker-serve` - Serve documentation in Docker
- `docs-docker-build-site` - Build documentation site in Docker
- `docs-docker-build-pdf` - Build documentation with PDF in Docker
- `docs-docker-clean` - Clean up Docker documentation resources
- `docs-docker-test` - Run Docker documentation tests

Also checks for Docker Compose commands:
- `docs-compose-up` - Start documentation server with Docker Compose
- `docs-compose-build-site` - Build site with Docker Compose
- `docs-compose-build-pdf` - Build site with PDF using Docker Compose
- `docs-compose-down` - Stop Docker Compose services

**Why This Matters**: Ensures developers have accurate documentation for all Docker-related commands.

### Test 8: Test Docker Build Doesn't Interfere with Rust Artifacts

Performs an actual Docker build and verifies:
- `target/` directory still exists after Docker build
- Docker image doesn't contain `target/` directory
- Rust artifacts remain untouched after Docker operations

**Why This Matters**: Confirms Docker builds don't accidentally modify or delete Rust build artifacts.

### Test 9: Verify Makefile Has Both Rust and Docker Targets

Checks that the Makefile contains:

**Rust targets**:
- `build` - Build Rust project
- `test` - Run Rust tests
- `lint` - Run Rust linter
- `clippy` - Run Rust Clippy

**Docker targets**:
- `docs-docker-build` - Build Docker documentation image
- `docs-docker-serve` - Serve documentation in Docker
- `docs-docker-build-site` - Build documentation site in Docker

**Why This Matters**: Ensures both Rust and Docker workflows are accessible through Make.

### Test 10: Verify No Conflicts Between Rust and Docker Artifacts

Checks for potential conflicts:
- `site/` is not inside `target/`
- `mkdocs-venv/` is not inside `target/`
- No Rust source files (`.rs`) in `docs/` directory

**Why This Matters**: Prevents accidental conflicts between Rust and documentation build artifacts.

## Command Line Options

### `--verbose`

Enable verbose output for detailed debugging:

```bash
./tests/integration/test_rust_docker_integration_e2e.sh --verbose
```

### `--no-remove`

Keep temporary files for debugging:

```bash
./tests/integration/test_rust_docker_integration_e2e.sh --no-remove
```

## Prerequisites

### Required

- Bash 3.2+ (macOS/Linux compatible)
- Rust toolchain (cargo, rustc)
- Make

### Optional (for Docker tests)

- Docker (for Test 8)

If Docker is not installed, Test 8 will be skipped with a warning.

## Exit Codes

- `0` - All tests passed
- `1` - One or more tests failed

## Expected Output

### Success

```
════════════════════════════════════════════════════════════════
 Rust Project and Docker Documentation Integration Test
════════════════════════════════════════════════════════════════
ℹ Project root: /path/to/testcase-manager

════════════════════════════════════════════════════════════════
 Test 1: Verify .dockerignore.mkdocs Excludes Rust Build Artifacts
════════════════════════════════════════════════════════════════
ℹ Checking .dockerignore.mkdocs content...
✓ .dockerignore.mkdocs excludes target/
✓ .dockerignore.mkdocs excludes Cargo.lock
✓ .dockerignore.mkdocs excludes src/

...

════════════════════════════════════════════════════════════════
 Test Summary
════════════════════════════════════════════════════════════════
ℹ Total tests: 10
ℹ Tests passed: 10
ℹ Tests failed: 0

ℹ Rust and Docker Integration Summary:
ℹ   ✓ .dockerignore.mkdocs excludes Rust artifacts (target/, Cargo.lock, src/)
ℹ   ✓ .gitignore includes documentation artifacts (site/, mkdocs-venv/)
ℹ   ✓ Rust build process is unaffected by Docker documentation setup
ℹ   ✓ Rust tests pass without interference
ℹ   ✓ Rust lint passes without interference
ℹ   ✓ AGENTS.md documents Docker commands accurately
ℹ   ✓ Docker builds don't interfere with Rust artifacts
ℹ   ✓ No conflicts between Rust and Docker artifact directories

✓ All Rust and Docker integration tests passed successfully!

ℹ The documentation Docker setup is properly integrated:
ℹ   • Docker builds exclude Rust artifacts
ℹ   • Rust toolchain is unaffected by Docker setup
ℹ   • Documentation artifacts are properly gitignored
ℹ   • AGENTS.md provides accurate command documentation
```

### Failure

If any test fails, the output will show:
- Which test failed
- Error details
- Relevant log output
- Remediation suggestions

## Integration with CI/CD

This test should be run as part of the CI/CD pipeline to ensure:
1. Docker documentation setup doesn't break Rust builds
2. All required files are properly configured
3. No conflicts exist between Rust and Docker artifacts

Add to your CI/CD pipeline:

```yaml
rust_docker_integration:
  script:
    - make rust-docker-integration-test
```

## Troubleshooting

### Test 1 Fails: .dockerignore.mkdocs Missing Entries

**Problem**: `.dockerignore.mkdocs` doesn't exclude required Rust artifacts.

**Solution**: Add the following lines to `.dockerignore.mkdocs`:
```
target/
Cargo.lock
src/
```

### Test 3 Fails: .gitignore Missing Entries

**Problem**: `.gitignore` doesn't include documentation artifacts.

**Solution**: Add the following lines to `.gitignore`:
```
mkdocs-venv/
site/
```

### Test 4-6 Fail: Rust Build/Test/Lint Issues

**Problem**: Rust toolchain has errors unrelated to Docker setup.

**Solution**: Fix the Rust errors first, then re-run the integration test.

### Test 7 Fails: AGENTS.md Missing Commands

**Problem**: AGENTS.md doesn't document all Docker commands.

**Solution**: Update AGENTS.md to include the missing commands under the appropriate sections.

### Test 8 Fails: Docker Build Issues

**Problem**: Docker build fails or includes Rust artifacts.

**Solution**: 
1. Ensure `.dockerignore.mkdocs` is correctly configured
2. Verify the Docker build copies `.dockerignore.mkdocs` to `.dockerignore`
3. Check Dockerfile.mkdocs doesn't explicitly copy excluded directories

### Test 9 Fails: Makefile Missing Targets

**Problem**: Makefile doesn't have required Rust or Docker targets.

**Solution**: Add the missing targets to the Makefile.

### Test 10 Fails: Artifact Conflicts

**Problem**: Rust and Docker artifacts are in conflicting locations.

**Solution**: Ensure `site/` and `mkdocs-venv/` are at the project root, not inside `target/`.

## Related Documentation

- [Docker MkDocs Documentation](../../README_DOCKER_MKDOCS.md)
- [Docker .dockerignore Test](./README_DOCKER_DOCKERIGNORE_TEST.md)
- [AGENTS.md](../../AGENTS.md)
- [Main README](../../README.md)

## Test Implementation

The test is implemented in `test_rust_docker_integration_e2e.sh` and uses:
- Centralized logging library (`scripts/lib/logger.sh`)
- Temporary file cleanup
- Parallel test execution where possible
- Cross-platform compatibility (macOS/Linux)

## Maintenance

When adding new Docker commands or Rust toolchain features:
1. Update this test to verify integration
2. Update AGENTS.md documentation
3. Ensure `.dockerignore.mkdocs` excludes any new Rust artifacts
4. Ensure `.gitignore` includes any new Docker artifacts
