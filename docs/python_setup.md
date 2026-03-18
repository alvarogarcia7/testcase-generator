# Python 3.14 Environment Setup

This document describes how to set up and verify Python 3.14 for both local development and Docker environments.

## Overview

The project requires Python 3.14 for various utility scripts and CI/CD tools. Python 3.14 is managed using the [uv](https://github.com/astral-sh/uv) package manager, which provides fast and reliable Python package management.

## Quick Start

### Local Environment

```bash
# Install and configure Python 3.14
make setup-python

# Verify the installation
make verify-python
```

### Docker Environment

Python 3.14 is automatically configured during Docker image build:

```bash
# Build the Docker image
make docker-build

# Run the Docker container
docker-compose up -d

# Verify Python 3.14 in the container
docker exec testcase-manager python3.14 --version
```

## Detailed Setup

### Prerequisites

1. **uv Package Manager**: Install uv if not already available
   ```bash
   curl -LsSf https://astral.sh/uv/install.sh | sh
   ```

2. **Python 3.14**: Will be automatically installed by uv during setup

### Local Setup Process

The `make setup-python` command executes `scripts/setup_python_env.sh`, which:

1. Checks if uv is installed
2. Syncs Python dependencies from `pyproject.toml` and `uv.lock`
3. Installs Python 3.14 using `uv python install --default 3.14`
4. Verifies Python 3.14 is working with `uv run python3.14 --version`
5. Re-syncs dependencies to ensure compatibility with Python 3.14

### Docker Setup Process

The Dockerfile includes the following Python 3.14 setup steps:

```dockerfile
# Install uv package manager
COPY --from=ghcr.io/astral-sh/uv:latest /uv /uvx /bin/

# Copy Python project files
COPY pyproject.toml uv.lock ./

# First, sync dependencies (this may use system Python initially)
RUN uv sync

# Then install Python 3.14 and set as default
RUN uv python install --default 3.14

# Verify Python 3.14 is available and working
RUN uv run python3.14 --version

# Create symlinks to make python3.14 available globally
RUN ln -sf $(uv python find 3.14) /usr/local/bin/python3.14 && \
    ln -sf /usr/local/bin/python3.14 /usr/local/bin/python3 && \
    ln -sf /usr/local/bin/python3.14 /usr/local/bin/python

# Verify global Python 3.14 is available
RUN python3.14 --version && python3 --version && python --version

# Re-sync to ensure all dependencies are installed with Python 3.14
RUN uv sync --python 3.14
```

## Verification

### Verification Script

Run the verification script to check if Python 3.14 is properly configured:

```bash
# Local environment
make verify-python

# Or run the script directly
./scripts/verify_python_env.sh
```

The verification script checks:

1. **Python 3.14 availability**: Ensures `python3.14` command exists
2. **Version verification**: Confirms `python3.14 --version` returns Python 3.14.x
3. **Symlink verification**: Checks if `python3` points to Python 3.14
4. **uv integration** (local only): Verifies uv can find and run Python 3.14
5. **Dependencies**: Confirms required Python packages are installed

### Expected Output

```
[INFO] Verifying Python 3.14 environment...
[INFO] Running in local environment
[INFO] Test 1: Checking python3.14 availability...
[INFO] ✓ python3.14 found at: /Users/user/.local/share/uv/python/cpython-3.14.0-macos-aarch64-none/bin/python3.14
[INFO] Test 2: Verifying python3.14 --version...
[INFO] ✓ Python version: Python 3.14.0
[INFO] ✓ Correct Python 3.14 version
[INFO] Test 3: Checking python3 symlink...
[INFO]   python3 version: Python 3.14.0
[INFO] ✓ python3 points to Python 3.14
[INFO] Test 4: Checking uv package manager...
[INFO] ✓ uv found at: /Users/user/.local/bin/uv
[INFO] Test 5: Verifying uv can find Python 3.14...
[INFO] ✓ uv Python 3.14 path: /Users/user/.local/share/uv/python/cpython-3.14.0-macos-aarch64-none/bin/python3.14
[INFO] Test 6: Verifying uv run python3.14...
[INFO] ✓ uv run python3.14 version: Python 3.14.0
[INFO] Test 7: Checking Python dependencies...
[INFO] ✓ Required Python packages are installed
[INFO]
[INFO] =========================================
[INFO] ✓ All Python 3.14 verification tests passed!
[INFO] =========================================
```

## Python Dependencies

The project uses minimal Python dependencies defined in `pyproject.toml`:

```toml
[project]
name = "testcase-generator"
version = "0.1.0"
requires-python = ">=3.14"
dependencies = [
    "jsonschema>=4.26.0",
    "mypy>=1.19.1",
    "pyyaml>=6.0.3",
    "ruff>=0.15.6",
]
```

### Dependency Usage

- **pyyaml**: YAML parsing for `convert_verification_to_result_yaml.py`
- **jsonschema**: JSON schema validation for test case validation
- **mypy**: Static type checking for Python scripts
- **ruff**: Fast Python linter and formatter

## Usage

### Running Python Scripts

#### Local Environment

```bash
# Using uv (recommended)
uv run python3.14 scripts/convert_verification_to_result_yaml.py

# Using virtual environment
source .venv/bin/activate
python3.14 scripts/convert_verification_to_result_yaml.py

# Direct invocation (after setup-python)
python3.14 scripts/convert_verification_to_result_yaml.py
```

#### Docker Environment

```bash
# Python 3.14 is available directly
docker exec testcase-manager python3.14 scripts/convert_verification_to_result_yaml.py

# All symlinks work
docker exec testcase-manager python3 scripts/convert_verification_to_result_yaml.py
docker exec testcase-manager python scripts/convert_verification_to_result_yaml.py
```

## Troubleshooting

### Issue: `uv not found`

**Solution**: Install uv package manager
```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

### Issue: `Python 3.14 not found after setup`

**Solution**: Ensure uv can install Python 3.14
```bash
# Check if Python 3.14 is available
uv python list

# Install Python 3.14 explicitly
uv python install 3.14

# Set as default
uv python pin 3.14
```

### Issue: `ImportError: No module named 'yaml'`

**Solution**: Re-sync dependencies
```bash
# Local environment
make setup-python

# Or manually
uv sync --python 3.14
```

### Issue: Docker build fails on Python setup

**Solution**: Check Docker logs and ensure network connectivity
```bash
# Build with verbose output
docker build --progress=plain -t testcase-manager:latest .

# Check if uv image is accessible
docker pull ghcr.io/astral-sh/uv:latest
```

### Issue: `python3` doesn't point to Python 3.14

**Local Environment**:
The local setup doesn't create system-wide symlinks. Use `uv run python3.14` or activate the virtual environment:
```bash
source .venv/bin/activate
python --version  # Should show Python 3.14
```

**Docker Environment**:
This should not happen as symlinks are created during build. If it does, rebuild the Docker image:
```bash
make docker-build
```

## Advanced Usage

### Using Different Python Versions

While the project requires Python 3.14, you can test with other versions:

```bash
# Install another Python version
uv python install 3.13

# Run script with specific version
uv run --python 3.13 python scripts/some_script.py
```

### Updating Dependencies

```bash
# Update all dependencies
uv sync --upgrade

# Update specific dependency
uv add pyyaml@latest

# Lock dependencies
uv lock
```

### Development Setup

For active Python development:

```bash
# Activate virtual environment
source .venv/bin/activate

# Install development dependencies (if any)
uv sync --all-extras

# Run type checking
mypy scripts/

# Run linting
ruff check scripts/
```

## Integration with CI/CD

The Python 3.14 setup is integrated into the project's CI/CD pipeline:

- **Pre-commit**: Verification runs before commits
- **Docker Build**: Automated Python 3.14 installation
- **Acceptance Tests**: Uses Python scripts for validation

Ensure Python 3.14 is properly configured before running:
```bash
make verify-python
make test
make acceptance-test
```

## See Also

- [uv Documentation](https://github.com/astral-sh/uv)
- [Python 3.14 Release Notes](https://www.python.org/downloads/release/python-3140/)
- [Project Commands (AGENTS.md)](../AGENTS.md#commands)
- [Report Generation Documentation](report_generation.md)
