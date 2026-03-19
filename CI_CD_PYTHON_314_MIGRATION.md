# CI/CD Pipeline Migration to Python 3.14 and uv Package Manager

## Overview

This document describes the updates made to CI/CD pipelines to use Python 3.14 and the uv package manager, ensuring Docker builds and test runs work correctly in automated environments.

## Changes Made

### 1. GitLab CI Pipeline (.gitlab-ci.yml)

#### rust:build-test-lint Job
- **Added Python 3.14 setup in before_script**:
  - Install uv package manager via curl
  - Add uv to PATH
  - Install Python 3.14 using `uv python install 3.14`
  - Sync Python dependencies using `uv sync --python 3.14`

- **Updated Python script invocations**:
  - Changed `python3 scripts/ci_parse_test_results.py` → `uv run python3.14 scripts/ci_parse_test_results.py`
  - Changed `python3 scripts/ci_compare_coverage.py` → `uv run python3.14 scripts/ci_compare_coverage.py`
  - Changed `python3 scripts/ci_post_coverage_comment.py` → `uv run python3.14 scripts/ci_post_coverage_comment.py`

- **Updated after_script**:
  - Add PATH export for uv
  - Changed `python3 scripts/ci_post_mr_comment.py` → `uv run python3.14 scripts/ci_post_mr_comment.py`
  - Changed `python3 scripts/ci_post_clippy_comment.py` → `uv run python3.14 scripts/ci_post_clippy_comment.py`

- **Added caching for Python environments**:
  - Cache `.venv` directory
  - Cache `$HOME/.cargo/bin/uv` binary
  - Cache `$HOME/.local/share/uv` data directory

#### docker:build Job
- **Added Python 3.14 verification**:
  - Verify `python3.14 --version` in built Docker image
  - Verify `python3 --version` in built Docker image
  - Verify `uv --version` in built Docker image

#### docker:verify Job
- **Added Python 3.14 setup in before_script**:
  - Install curl and uv package manager
  - Sync Python dependencies using `uv sync --python 3.14`

- **Updated Python script invocations**:
  - Changed `python3 scripts/ci_docker_security_scan.py` → `uv run python3.14 scripts/ci_docker_security_scan.py`

#### Commented-out Performance and Report Jobs
Updated for future use when uncommented:

- **performance:baseline**:
  - Install uv and Python 3.14
  - Install psutil using `uv pip install psutil`
  - Run benchmark with `uv run python3.14 scripts/ci_benchmark.py`

- **performance:compare**:
  - Install uv and Python 3.14
  - Install psutil using `uv pip install psutil`
  - Run benchmark and comparison with `uv run python3.14`

- **report:summary**:
  - Install uv and Python 3.14
  - Run report generation with `uv run python3.14`

#### Global Variables
- **Added documentation comment**:
  ```yaml
  # Python 3.14 is managed via uv package manager
  # All Python scripts should be run with: uv run python3.14 script.py
  ```

### 2. GitHub Actions Workflow (.github/workflows/workspace.yml)

#### CI Job Steps

- **Added uv installation step**:
  ```yaml
  - name: Install uv
    run: curl -LsSf https://astral.sh/uv/install.sh | sh
  ```

- **Added Python caching step**:
  ```yaml
  - name: Cache uv and Python
    uses: actions/cache@v4
    with:
      path: |
        ~/.cargo/bin/uv
        ~/.local/share/uv
        .venv
      key: ${{ runner.os }}-uv-${{ hashFiles('**/pyproject.toml', '**/uv.lock') }}
      restore-keys: |
        ${{ runner.os }}-uv-
  ```

- **Added Python 3.14 setup step**:
  ```yaml
  - name: Setup Python 3.14 with uv
    run: |
      export PATH="$HOME/.cargo/bin:$PATH"
      uv python install 3.14
      uv sync --python 3.14
  ```

- **Updated E2E tests step**:
  - Export PATH for uv before running tests

#### Global Environment Variables
- **Added documentation comment**:
  ```yaml
  # Python 3.14 is managed via uv package manager
  # All Python scripts should be run with: uv run python3.14 script.py
  ```

### 3. Docker Configuration

The Dockerfile was already properly configured for Python 3.14 and uv (no changes needed):

- Copies uv binary from official uv image
- Syncs Python dependencies with `uv sync`
- Installs Python 3.14 with `uv python install --default 3.14`
- Creates global symlinks for python3.14, python3, and python
- Re-syncs dependencies with Python 3.14

## Benefits

### 1. Performance
- **uv** is 10-100x faster than pip for dependency resolution
- Caching of `.venv` and uv data directories reduces setup time
- Lock files ensure reproducible builds

### 2. Reliability
- Consistent Python 3.14 environment across all CI jobs
- Lock files prevent dependency drift
- Explicit Python version management

### 3. Maintainability
- Single tool (uv) for Python version and package management
- Clear documentation in CI configuration
- Easy to update dependencies via `uv sync`

## CI Job Flow

### GitLab CI

```
docker:build (Python 3.14 in Docker image)
  ↓
rust:build-test-lint (uv + Python 3.14 installed)
  ↓
docker:verify (uv + Python 3.14 installed)
  ↓
rust:e2e-tests
```

### GitHub Actions

```
Install uv
  ↓
Cache uv and Python environments
  ↓
Setup Python 3.14 with uv
  ↓
Run all tests and coverage
  ↓
Run E2E tests
```

## Testing Verification

All CI/CD jobs now:

1. ✅ Install uv package manager
2. ✅ Install Python 3.14 using uv
3. ✅ Sync dependencies from pyproject.toml and uv.lock
4. ✅ Run Python scripts with `uv run python3.14`
5. ✅ Cache Python environments for faster builds

## Migration Checklist

- [x] Update GitLab CI rust:build-test-lint job
- [x] Update GitLab CI docker:verify job
- [x] Update GitLab CI docker:build job (verification)
- [x] Update GitLab CI commented-out jobs (performance, report)
- [x] Add Python/uv caching to GitLab CI
- [x] Update GitHub Actions CI job
- [x] Add Python/uv caching to GitHub Actions
- [x] Add documentation comments to CI files
- [x] Verify Docker configuration (already correct)

## Future Improvements

1. **Docker image caching**: Consider caching Python dependencies layer in Docker build
2. **Parallel Python installs**: If multiple jobs need Python, consider using a pre-built image
3. **Version pinning**: Consider pinning uv version in CI for extra stability

## References

- uv documentation: https://docs.astral.sh/uv/
- Python 3.14 setup guide: See `AGENTS.md` Python 3.14 Environment Setup section
- pyproject.toml: Defines Python dependencies
- uv.lock: Lock file with pinned dependency versions
