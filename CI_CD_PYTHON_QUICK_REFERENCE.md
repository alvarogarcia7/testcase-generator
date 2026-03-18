# CI/CD Python 3.14 Quick Reference

## Quick Start

### GitLab CI

```yaml
before_script:
  # Install uv package manager
  - curl -LsSf https://astral.sh/uv/install.sh | sh
  - export PATH="$HOME/.cargo/bin:$PATH"
  # Install Python 3.14 and sync dependencies
  - uv python install 3.14
  - uv sync --python 3.14

script:
  # Run Python scripts
  - uv run python3.14 scripts/my_script.py
```

### GitHub Actions

```yaml
- name: Install uv
  run: curl -LsSf https://astral.sh/uv/install.sh | sh

- name: Setup Python 3.14 with uv
  run: |
    export PATH="$HOME/.cargo/bin:$PATH"
    uv python install 3.14
    uv sync --python 3.14

- name: Run Python script
  run: |
    export PATH="$HOME/.cargo/bin:$PATH"
    uv run python3.14 scripts/my_script.py
```

## Common Commands

### Install uv
```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

### Add uv to PATH
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### Install Python 3.14
```bash
uv python install 3.14
```

### Set Python 3.14 as default for project
```bash
uv python install --default 3.14
```

### Sync dependencies
```bash
# Use exact versions from uv.lock
uv sync --frozen

# Use exact versions for specific Python version
uv sync --frozen --python 3.14

# Update dependencies if needed (not recommended in CI)
uv sync --python 3.14
```

### Run Python script
```bash
# Recommended: Use uv run
uv run python3.14 script.py

# Alternative: Activate venv first
source .venv/bin/activate
python3.14 script.py
```

### Install additional dependencies
```bash
# Add to project (updates pyproject.toml)
uv add package-name>=1.0.0

# Install in current environment only
uv pip install package-name
```

## Caching Strategies

### GitLab CI Cache

```yaml
cache:
  key: "${CI_COMMIT_REF_SLUG}-python"
  paths:
    - .venv                      # Python virtual environment
    - $HOME/.cargo/bin/uv        # uv binary
    - $HOME/.local/share/uv      # uv data (Python versions)
  policy: pull-push
```

### GitHub Actions Cache

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

## Docker Integration

### Dockerfile Example

```dockerfile
# Install uv package manager
COPY --from=ghcr.io/astral-sh/uv:latest /uv /uvx /bin/

# Copy Python project files
COPY pyproject.toml uv.lock ./

# Sync dependencies
RUN uv sync

# Install Python 3.14 and set as default
RUN uv python install --default 3.14

# Create global symlinks (optional, for convenience)
RUN ln -sf $(uv python find 3.14) /usr/local/bin/python3.14 && \
    ln -sf /usr/local/bin/python3.14 /usr/local/bin/python3 && \
    ln -sf /usr/local/bin/python3.14 /usr/local/bin/python

# Re-sync with Python 3.14
RUN uv sync --frozen --python 3.14
```

### Docker Verification

```bash
# Verify Python 3.14 is available
docker run --rm image:tag python3.14 --version

# Verify uv is available
docker run --rm image:tag uv --version

# Run Python script in container
docker run --rm image:tag uv run python3.14 script.py
```

## Troubleshooting

### uv not found

**Problem**: `bash: uv: command not found`

**Solution**:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### Python 3.14 not found

**Problem**: `error: No interpreter found for Python 3.14`

**Solution**:
```bash
uv python install 3.14
```

### Module not found

**Problem**: `ModuleNotFoundError: No module named 'xyz'`

**Solution**:
```bash
# Ensure dependencies are synced
uv sync --python 3.14

# Or install missing package
uv pip install xyz
```

### Wrong Python version

**Problem**: Script runs with Python 3.12 instead of 3.14

**Solution**:
```bash
# Always use explicit version
uv run python3.14 script.py

# Or verify active Python version
uv run python3.14 --version
```

### Cache not working

**Problem**: CI job reinstalls everything every time

**Solution**:

**GitLab CI**:
```yaml
cache:
  paths:
    - .venv
    - $HOME/.local/share/uv
```

**GitHub Actions**:
```yaml
- uses: actions/cache@v4
  with:
    path: |
      ~/.local/share/uv
      .venv
    key: ${{ runner.os }}-uv-${{ hashFiles('uv.lock') }}
```

## Best Practices

### 1. Always use explicit Python version
```bash
# Good
uv run python3.14 script.py

# Avoid (may use wrong version)
python script.py
```

### 2. Use frozen dependencies in CI
```bash
# Lock file ensures reproducible builds
uv sync --frozen --python 3.14
```

### 3. Cache strategically
- Cache `.venv` for dependency artifacts
- Cache `~/.local/share/uv` for Python versions
- Use lock file hash in cache key

### 4. Export PATH in scripts
```bash
# At the beginning of CI jobs
export PATH="$HOME/.cargo/bin:$PATH"
```

### 5. Verify installation
```bash
# After setup, verify everything works
uv --version
uv run python3.14 --version
uv run python3.14 -c "import yaml; print(yaml.__version__)"
```

## Performance Tips

### 1. Use --frozen in CI
```bash
# Skips dependency resolution (faster)
uv sync --frozen --python 3.14
```

### 2. Separate Python setup from script execution
```yaml
# GitLab CI
before_script:
  - uv python install 3.14
  - uv sync --frozen --python 3.14

script:
  - uv run python3.14 script1.py
  - uv run python3.14 script2.py
```

### 3. Use Docker layer caching
```dockerfile
# Copy dependency files first (separate layer)
COPY pyproject.toml uv.lock ./
RUN uv sync --frozen

# Copy source code later
COPY . .
```

### 4. Parallel dependency installation
```bash
# uv automatically parallelizes, but ensure network access
uv sync --python 3.14  # Already parallel
```

## Environment Variables

### CI Environment
```bash
# Ensure uv uses project directory
export UV_PROJECT_ENVIRONMENT=.venv

# Disable uv cache (if causing issues)
export UV_NO_CACHE=1

# Use specific Python version
export UV_PYTHON=3.14
```

### Docker Environment
```dockerfile
# Set in Dockerfile
ENV UV_PROJECT_ENVIRONMENT=/app/.venv
ENV UV_PYTHON=3.14
```

## Migration from pip/pip-tools

### Old (pip)
```bash
python3 -m pip install -r requirements.txt
python3 script.py
```

### New (uv)
```bash
uv sync --python 3.14
uv run python3.14 script.py
```

### Old (venv + pip)
```bash
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
python script.py
```

### New (uv)
```bash
uv sync --python 3.14
uv run python3.14 script.py
```

## Additional Resources

- **uv Documentation**: https://docs.astral.sh/uv/
- **Python 3.14 Release Notes**: https://docs.python.org/3.14/whatsnew/3.14.html
- **Project Documentation**: See `AGENTS.md` Python 3.14 Environment Setup section
- **CI/CD Migration Guide**: See `CI_CD_PYTHON_314_MIGRATION.md`
