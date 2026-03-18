#!/usr/bin/env bash
set -e

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source logger
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

# Navigate to project root
cd "$PROJECT_ROOT"

log_info "Setting up Python 3.14 environment..."

# Check if uv is installed
if ! command -v uv > /dev/null 2>&1; then
    log_error "uv is not installed. Please install uv first:"
    log_error "  curl -LsSf https://astral.sh/uv/install.sh | sh"
    exit 1
fi

log_info "uv found: $(which uv)"

# Sync dependencies first
log_info "Syncing Python dependencies with uv..."
uv sync

# Install Python 3.14 and set as default
log_info "Installing Python 3.14 and setting as default..."
uv python install --default 3.14

# Get the Python 3.14 path
PYTHON_314_PATH=$(uv python find 3.14 2>/dev/null || echo "")

if [ -z "$PYTHON_314_PATH" ]; then
    log_error "Failed to find Python 3.14 after installation"
    exit 1
fi

log_info "Python 3.14 installed at: $PYTHON_314_PATH"

# Verify Python 3.14 is working
log_info "Verifying Python 3.14..."
if uv run python3.14 --version > /dev/null 2>&1; then
    PYTHON_VERSION=$(uv run python3.14 --version)
    log_info "✓ Python 3.14 is working: $PYTHON_VERSION"
else
    log_error "Failed to verify Python 3.14"
    exit 1
fi

# Re-sync to ensure all dependencies are installed with Python 3.14
log_info "Re-syncing dependencies with Python 3.14..."
uv sync --python 3.14

log_info "✓ Python 3.14 environment setup complete!"
log_info ""
log_info "To use Python 3.14 in your shell, run:"
log_info "  uv run python3.14 <script.py>"
log_info ""
log_info "Or activate the virtual environment:"
log_info "  source .venv/bin/activate"
log_info "  python3.14 --version"
