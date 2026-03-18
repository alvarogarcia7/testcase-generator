# Python 3.14 Migration

This document tracks the Python 3.14 migration status for the testcase-generator project.

## Migration Status

✅ **COMPLETE** - Python 3.14 is fully integrated into both Docker and local environments.

## Changes Made

### 1. Dockerfile Updates

The Dockerfile now includes a complete Python 3.14 setup process:

- ✅ Install `uv` package manager from official image
- ✅ Sync Python dependencies from `pyproject.toml` and `uv.lock`
- ✅ Install Python 3.14 with `uv python install --default 3.14` (runs AFTER `uv sync`)
- ✅ Create global symlinks for `python3.14`, `python3`, and `python`
- ✅ Re-sync dependencies with Python 3.14 for compatibility
- ✅ Verify Python 3.14 installation with version checks

### 2. Setup Scripts

Created automated setup scripts for local development:

- ✅ `scripts/setup_python_env.sh` - Installs and configures Python 3.14 locally
- ✅ `scripts/verify_python_env.sh` - Verifies Python 3.14 environment is working

### 3. Makefile Targets

Added convenient Makefile targets:

- ✅ `make setup-python` - Run Python 3.14 setup
- ✅ `make verify-python` - Verify Python 3.14 installation

### 4. Documentation

Updated documentation to reflect Python 3.14 setup:

- ✅ `AGENTS.md` - Added Python 3.14 commands and setup section
- ✅ `docs/python_setup.md` - Comprehensive Python 3.14 setup guide

### 5. Dependency Configuration

- ✅ `pyproject.toml` - Already configured with `requires-python = ">=3.14"`
- ✅ `uv.lock` - Existing lock file compatible with Python 3.14

### 6. .gitignore Updates

- ✅ Added comprehensive Python-related ignore patterns
- ✅ Includes `__pycache__`, `.mypy_cache`, `.ruff_cache`, etc.

## Verification

### Docker Environment

```bash
# Build Docker image
make docker-build

# Run container
docker-compose up -d

# Verify Python 3.14
docker exec testcase-manager python3.14 --version
docker exec testcase-manager python3 --version
docker exec testcase-manager python --version
```

Expected output: All commands should show `Python 3.14.x`

### Local Environment

```bash
# Setup Python 3.14
make setup-python

# Verify installation
make verify-python
```

Expected output: All verification tests pass

## Key Implementation Details

### Docker Setup Order

The Dockerfile implements Python 3.14 setup in this specific order:

1. **First**: `uv sync` - Install dependencies (may use system Python initially)
2. **Second**: `uv python install --default 3.14` - Install Python 3.14
3. **Third**: Create symlinks to make `python3.14` globally available
4. **Fourth**: `uv sync --python 3.14` - Re-sync with Python 3.14

This order ensures:
- Dependencies are available before Python 3.14 installation
- Python 3.14 is properly installed before creating symlinks
- All dependencies are reinstalled/verified with Python 3.14

### Local Setup Process

The local setup script (`scripts/setup_python_env.sh`):

1. Checks if `uv` is installed
2. Syncs dependencies from lock file
3. Installs Python 3.14 and sets as default
4. Verifies Python 3.14 is working
5. Re-syncs dependencies for compatibility

### Symlink Strategy

**Docker**: Creates system-wide symlinks in `/usr/local/bin/`
```bash
python3.14 -> /path/to/uv/python/cpython-3.14.x/bin/python3.14
python3 -> /usr/local/bin/python3.14
python -> /usr/local/bin/python3.14
```

**Local**: Uses `uv` virtual environment (no system-wide symlinks)
```bash
# Access via uv
uv run python3.14 script.py

# Or activate venv
source .venv/bin/activate
python script.py  # Uses Python 3.14
```

## Python Dependencies

All Python dependencies are defined in `pyproject.toml`:

```toml
dependencies = [
    "jsonschema>=4.26.0",  # JSON schema validation
    "mypy>=1.19.1",        # Type checking
    "pyyaml>=6.0.3",       # YAML parsing
    "ruff>=0.15.6",        # Linting/formatting
]
```

## Testing

Python 3.14 integration is tested through:

1. **Dockerfile Build**: Fails if Python 3.14 setup doesn't work
2. **Verification Script**: `scripts/verify_python_env.sh` tests all aspects
3. **Integration Tests**: Uses Python scripts (e.g., `convert_verification_to_result_yaml.py`)
4. **Acceptance Tests**: Full E2E testing including Python tools

## Troubleshooting

### Docker Issues

**Problem**: Docker build fails on Python 3.14 installation

**Solution**: 
- Ensure network connectivity for downloading Python 3.14
- Check if uv image is accessible: `docker pull ghcr.io/astral-sh/uv:latest`
- Build with verbose output: `docker build --progress=plain -t testcase-manager:latest .`

### Local Issues

**Problem**: `uv not found`

**Solution**: Install uv
```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

**Problem**: Python 3.14 not found after setup

**Solution**: Re-run setup
```bash
make setup-python
```

**Problem**: Import errors for Python packages

**Solution**: Re-sync dependencies
```bash
uv sync --python 3.14
```

## Migration Checklist

- [x] Update Dockerfile with Python 3.14 installation
- [x] Ensure `uv python install --default 3.14` runs after `uv sync`
- [x] Create global symlinks in Docker for `python3.14`, `python3`, `python`
- [x] Verify `python3.14 --version` works in Docker
- [x] Create local setup script (`scripts/setup_python_env.sh`)
- [x] Create verification script (`scripts/verify_python_env.sh`)
- [x] Add Makefile targets (`setup-python`, `verify-python`)
- [x] Update AGENTS.md with Python 3.14 commands
- [x] Create comprehensive documentation (`docs/python_setup.md`)
- [x] Update .gitignore for Python artifacts
- [x] Test Docker build and verify Python 3.14
- [x] Test local setup and verify Python 3.14

## Next Steps

For ongoing development:

1. **Regular Testing**: Run `make verify-python` periodically
2. **Dependency Updates**: Keep Python packages up to date with `uv sync --upgrade`
3. **Documentation**: Keep Python setup docs updated as tooling evolves
4. **CI/CD**: Integrate Python verification into CI/CD pipelines if needed

## Related Documentation

- [Python Setup Guide](docs/python_setup.md) - Detailed setup instructions
- [AGENTS.md](AGENTS.md#python-314-environment-setup) - Quick command reference
- [Dockerfile](Dockerfile) - Docker Python 3.14 setup implementation
- [pyproject.toml](pyproject.toml) - Python dependencies configuration

## Contact

For questions or issues related to Python 3.14 migration, please open an issue in the project repository.
