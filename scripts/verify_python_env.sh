#!/usr/bin/env bash
set -e

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source logger
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

log_info "Verifying Python 3.14 environment..."

# Check if we're in Docker or local
if [ -f /.dockerenv ]; then
    log_info "Running in Docker environment"
    IN_DOCKER=true
else
    log_info "Running in local environment"
    IN_DOCKER=false
fi

# Test 1: Check if python3.14 is available
log_info "Test 1: Checking python3.14 availability..."
if command -v python3.14 > /dev/null 2>&1; then
    PYTHON_PATH=$(which python3.14)
    log_info "✓ python3.14 found at: $PYTHON_PATH"
else
    log_error "✗ python3.14 not found in PATH"
    exit 1
fi

# Test 2: Verify python3.14 --version works
log_info "Test 2: Verifying python3.14 --version..."
if PYTHON_VERSION=$(python3.14 --version 2>&1); then
    log_info "✓ Python version: $PYTHON_VERSION"
    
    # Check if it's actually Python 3.14
    if echo "$PYTHON_VERSION" | grep -q "Python 3\.14"; then
        log_info "✓ Correct Python 3.14 version"
    else
        log_error "✗ Expected Python 3.14, got: $PYTHON_VERSION"
        exit 1
    fi
else
    log_error "✗ Failed to get Python version"
    exit 1
fi

# Test 3: Check if python3 points to python3.14
log_info "Test 3: Checking python3 symlink..."
if command -v python3 > /dev/null 2>&1; then
    PYTHON3_VERSION=$(python3 --version 2>&1)
    log_info "  python3 version: $PYTHON3_VERSION"
    
    if echo "$PYTHON3_VERSION" | grep -q "Python 3\.14"; then
        log_info "✓ python3 points to Python 3.14"
    else
        log_warning "⚠ python3 does not point to Python 3.14"
    fi
else
    log_warning "⚠ python3 not found in PATH"
fi

# Test 4: Check if uv is available (local environment only)
if [ "$IN_DOCKER" = false ]; then
    log_info "Test 4: Checking uv package manager..."
    if command -v uv > /dev/null 2>&1; then
        UV_PATH=$(which uv)
        log_info "✓ uv found at: $UV_PATH"
        
        # Test 5: Check if uv can find Python 3.14
        log_info "Test 5: Verifying uv can find Python 3.14..."
        if UV_PYTHON=$(uv python find 3.14 2>&1); then
            log_info "✓ uv Python 3.14 path: $UV_PYTHON"
        else
            log_error "✗ uv cannot find Python 3.14"
            exit 1
        fi
        
        # Test 6: Check if uv run python3.14 works
        log_info "Test 6: Verifying uv run python3.14..."
        if UV_RUN_VERSION=$(uv run python3.14 --version 2>&1); then
            log_info "✓ uv run python3.14 version: $UV_RUN_VERSION"
        else
            log_error "✗ uv run python3.14 failed"
            exit 1
        fi
    else
        log_warning "⚠ uv not found (optional for local development)"
    fi
fi

# Test 7: Check Python dependencies
log_info "Test 7: Checking Python dependencies..."
if python3.14 -c "import yaml, jsonschema; print('✓ Python dependencies available')" 2>&1; then
    log_info "✓ Required Python packages are installed"
else
    log_error "✗ Some Python dependencies are missing"
    log_error "  Run: make setup-python (local) or rebuild Docker image"
    exit 1
fi

log_info ""
log_info "========================================="
log_info "✓ All Python 3.14 verification tests passed!"
log_info "========================================="
