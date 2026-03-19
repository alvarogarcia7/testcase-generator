#!/usr/bin/env bash
set -e

# Get script directory and source logger
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

# Default to local installation
INSTALL_MODE="local"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --local)
            INSTALL_MODE="local"
            shift
            ;;
        --global)
            INSTALL_MODE="global"
            shift
            ;;
        --help)
            echo "Usage: $0 [--local|--global]"
            echo ""
            echo "Install loc (lines of code counter) tool"
            echo ""
            echo "Options:"
            echo "  --local   Install to ~/.cargo/bin (default)"
            echo "  --global  Install globally (requires sudo)"
            echo "  --help    Show this help message"
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

section "Installing loc (lines of code counter)"

# Check if loc is already installed
if command -v loc >/dev/null 2>&1; then
    LOC_VERSION=$(loc --version 2>&1 | head -n 1 || echo "unknown")
    log_info "loc is already installed: $LOC_VERSION"
    log_info "Location: $(which loc)"
    
    # Ask user if they want to reinstall
    if [ -t 0 ]; then
        echo ""
        read -p "Do you want to reinstall? [y/N] " -n 1 -r
        echo ""
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            log_info "Skipping installation"
            exit 0
        fi
    else
        log_info "Non-interactive mode, skipping reinstallation"
        exit 0
    fi
fi

# Check if cargo is installed
if ! command -v cargo >/dev/null 2>&1; then
    log_error "cargo is not installed"
    log_error "Please install Rust from https://rustup.rs/"
    exit 1
fi

log_info "Installing loc using cargo..."

if [ "$INSTALL_MODE" = "global" ]; then
    log_info "Installing globally (requires sudo)..."
    sudo cargo install tokei --locked
else
    log_info "Installing to ~/.cargo/bin..."
    cargo install tokei --locked
fi

# Verify installation
if command -v loc >/dev/null 2>&1; then
    pass "loc installed successfully"
    LOC_VERSION=$(loc --version 2>&1 | head -n 1 || echo "unknown")
    log_info "Version: $LOC_VERSION"
    log_info "Location: $(which loc)"
elif command -v tokei >/dev/null 2>&1; then
    pass "tokei (loc) installed successfully"
    LOC_VERSION=$(tokei --version 2>&1 | head -n 1 || echo "unknown")
    log_info "Version: $LOC_VERSION"
    log_info "Location: $(which tokei)"
    log_info "Note: The tool is named 'tokei' but can count lines of code"
else
    fail "Installation failed"
    log_error "loc/tokei command not found after installation"
    exit 1
fi

log_info ""
log_info "You can now use 'make loc' to compute lines of code statistics"
