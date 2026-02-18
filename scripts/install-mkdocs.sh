#!/usr/bin/env bash
#
# install-mkdocs.sh - Install MkDocs and plugins for documentation generation
#
# This script creates a Python virtual environment and installs MkDocs with
# required plugins for building project documentation, including PDF generation.
#
# Usage:
#   ./scripts/install-mkdocs.sh [OPTIONS]
#
# Options:
#   --clean         Remove existing virtual environment before installing
#   --help          Show this help message
#
# Environment:
#   PYTHON          Python executable to use (default: python3)

set -e

# Get the script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source logger library
source "$SCRIPT_DIR/lib/logger.sh" || {
    echo "[ERROR] Failed to source logger library" >&2
    exit 1
}

# Configuration
VENV_DIR="$PROJECT_ROOT/mkdocs-venv"
REQUIREMENTS_FILE="$PROJECT_ROOT/requirements.txt"
PYTHON_CMD="${PYTHON:-python3}"

# Show help message
show_help() {
    cat << EOF
Usage: $0 [OPTIONS]

Install MkDocs and plugins for documentation generation.

OPTIONS:
    --clean         Remove existing virtual environment before installing
    --help          Show this help message

ENVIRONMENT:
    PYTHON          Python executable to use (default: python3)

EXAMPLES:
    # Install MkDocs and plugins
    $0

    # Clean install (remove existing virtual environment)
    $0 --clean

INSTALLED PACKAGES:
    mkdocs>=1.5.0              Static site generator for documentation
    mkdocs-material>=9.5.0     Material Design theme for MkDocs
    mkdocs-with-pdf>=0.9.3     PDF export plugin for MkDocs
    markdown>=3.5              Markdown processing library
    pymdown-extensions>=10.7   Markdown extensions for MkDocs

VIRTUAL ENVIRONMENT:
    Location: $VENV_DIR
    Activation: source $VENV_DIR/bin/activate

EOF
}

# Check if Python is installed
check_python() {
    if ! command -v "$PYTHON_CMD" >/dev/null 2>&1; then
        log_error "Python is not installed or not found: $PYTHON_CMD"
        log_error "Please install Python 3.8 or later, or set PYTHON environment variable"
        exit 1
    fi
    
    local python_version
    python_version=$("$PYTHON_CMD" --version 2>&1)
    log_info "Python version: $python_version"
    
    # Check if Python version is 3.8 or later
    local version_check
    version_check=$("$PYTHON_CMD" -c "import sys; print(int(sys.version_info >= (3, 8)))" 2>/dev/null || echo "0")
    if [ "$version_check" != "1" ]; then
        log_error "Python 3.8 or later is required"
        exit 1
    fi
    
    pass "Python version is compatible"
}

# Check if requirements.txt exists
check_requirements_file() {
    if [ ! -f "$REQUIREMENTS_FILE" ]; then
        log_error "Requirements file not found: $REQUIREMENTS_FILE"
        exit 1
    fi
    
    log_info "Requirements file: $REQUIREMENTS_FILE"
    pass "Requirements file found"
}

# Remove existing virtual environment
clean_venv() {
    if [ -d "$VENV_DIR" ]; then
        log_info "Removing existing virtual environment: $VENV_DIR"
        rm -rf "$VENV_DIR"
        pass "Removed existing virtual environment"
    else
        log_info "No existing virtual environment to remove"
    fi
}

# Create virtual environment
create_venv() {
    log_info "Creating virtual environment: $VENV_DIR"
    
    if "$PYTHON_CMD" -m venv "$VENV_DIR"; then
        pass "Created virtual environment"
    else
        log_error "Failed to create virtual environment"
        log_error "Try installing python3-venv: apt-get install python3-venv (Debian/Ubuntu)"
        exit 1
    fi
}

# Upgrade pip
upgrade_pip() {
    log_info "Upgrading pip..."
    
    if "$VENV_DIR/bin/pip" install --upgrade pip >/dev/null 2>&1; then
        local pip_version
        pip_version=$("$VENV_DIR/bin/pip" --version)
        pass "Upgraded pip: $pip_version"
    else
        log_warning "Failed to upgrade pip, continuing anyway"
    fi
}

# Install dependencies
install_dependencies() {
    log_info "Installing dependencies from requirements.txt..."
    
    if "$VENV_DIR/bin/pip" install -r "$REQUIREMENTS_FILE"; then
        pass "Installed dependencies successfully"
    else
        log_error "Failed to install dependencies"
        exit 1
    fi
}

# Verify installation
verify_installation() {
    log_info "Verifying MkDocs installation..."
    
    if [ ! -x "$VENV_DIR/bin/mkdocs" ]; then
        log_error "MkDocs executable not found in virtual environment"
        exit 1
    fi
    
    local mkdocs_version
    mkdocs_version=$("$VENV_DIR/bin/mkdocs" --version 2>&1)
    pass "MkDocs installed: $mkdocs_version"
    
    # List installed packages
    log_info "Installed packages:"
    "$VENV_DIR/bin/pip" list | grep -E "^(mkdocs|markdown|pymdown)" | while IFS= read -r line; do
        log_info "  $line"
    done
}

# Show post-installation instructions
show_instructions() {
    cat << EOF

Installation complete!

Activation:
  # Activate virtual environment
  source $VENV_DIR/bin/activate

  # Or use direct path
  $VENV_DIR/bin/mkdocs --version

Usage Examples:
  # Serve documentation locally
  $VENV_DIR/bin/mkdocs serve

  # Build static site
  $VENV_DIR/bin/mkdocs build

  # Build with PDF generation
  $VENV_DIR/bin/mkdocs build
  # (PDF plugin will generate PDF if configured in mkdocs.yml)

  # Clean build directory
  $VENV_DIR/bin/mkdocs build --clean

Deactivation:
  # When done, deactivate virtual environment
  deactivate

Documentation:
  MkDocs:          https://www.mkdocs.org/
  Material Theme:  https://squidfunk.github.io/mkdocs-material/
  PDF Plugin:      https://github.com/orzih/mkdocs-with-pdf

EOF
}

# Main installation logic
main() {
    local clean_install=false
    
    # Parse command line arguments
    while [ $# -gt 0 ]; do
        case "$1" in
            --clean)
                clean_install=true
                ;;
            --help|-h)
                show_help
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
        shift
    done
    
    section "Starting MkDocs installation"
    
    # Check Python installation
    check_python
    echo
    
    # Check requirements file
    check_requirements_file
    echo
    
    # Clean existing virtual environment if requested
    if [ "$clean_install" = true ]; then
        section "Cleaning existing installation"
        clean_venv
        echo
    fi
    
    # Create virtual environment if it doesn't exist
    if [ ! -d "$VENV_DIR" ]; then
        section "Creating virtual environment"
        create_venv
        echo
        
        section "Upgrading pip"
        upgrade_pip
        echo
    else
        log_info "Virtual environment already exists: $VENV_DIR"
        log_info "Use --clean to remove and recreate"
        echo
    fi
    
    # Install dependencies
    section "Installing dependencies"
    install_dependencies
    echo
    
    # Verify installation
    section "Verifying installation"
    verify_installation
    echo
    
    # Show instructions
    show_instructions
    
    pass "MkDocs installation complete"
    exit 0
}

# Run main function
main "$@"
