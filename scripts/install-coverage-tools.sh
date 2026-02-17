#!/usr/bin/env bash
#
# install-coverage-tools.sh - Install code coverage tools for Rust projects
#
# This script installs coverage tools for:
# - Local development (cargo-llvm-cov)
# - GitHub Actions (cargo-llvm-cov)
# - GitLab CI (grcov)
#
# Usage:
#   ./scripts/install-coverage-tools.sh [OPTIONS]
#
# Options:
#   --local         Install tools for local development (default)
#   --github        Install tools for GitHub Actions
#   --gitlab        Install tools for GitLab CI
#   --all           Install all tools
#   --help          Show this help message
#
# Environment:
#   CI              Set to "true" in CI environments (auto-detected)
#   GITHUB_ACTIONS  Set to "true" in GitHub Actions (auto-detected)
#   GITLAB_CI       Set to "true" in GitLab CI (auto-detected)

set -e

# Get the script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source logger library
source "$SCRIPT_DIR/lib/logger.sh" || {
    echo "[ERROR] Failed to source logger library" >&2
    exit 1
}

# Show help message
show_help() {
    cat << EOF
Usage: $0 [OPTIONS]

Install code coverage tools for Rust projects across different environments.

OPTIONS:
    --local         Install tools for local development (cargo-llvm-cov)
    --github        Install tools for GitHub Actions (cargo-llvm-cov)
    --gitlab        Install tools for GitLab CI (grcov)
    --all           Install all tools
    --help          Show this help message

ENVIRONMENT DETECTION:
    The script automatically detects CI environments:
    - GitHub Actions (GITHUB_ACTIONS=true)
    - GitLab CI (GITLAB_CI=true)
    - Local development (default)

EXAMPLES:
    # Install for local development
    $0 --local

    # Install for GitHub Actions
    $0 --github

    # Install for GitLab CI
    $0 --gitlab

    # Install all tools
    $0 --all

    # Auto-detect environment and install
    $0

TOOLS INSTALLED:
    cargo-llvm-cov: Modern coverage tool using LLVM's native coverage
                    - Used in local development and GitHub Actions
                    - Generates LCOV and HTML reports
                    - Supports coverage thresholds

    grcov:          Mozilla's coverage aggregation tool
                    - Used in GitLab CI
                    - Processes raw coverage data from rustc
                    - Generates LCOV, Cobertura, and HTML reports

RUST COMPONENTS:
    llvm-tools-preview: Required for coverage instrumentation
                        - Installed automatically by this script
                        - Needed by both cargo-llvm-cov and grcov

EOF
}

# Check if Rust is installed
check_rust() {
    if ! command -v cargo >/dev/null 2>&1; then
        log_error "Rust/Cargo is not installed. Please install Rust first:"
        log_error "  https://rustup.rs/"
        exit 1
    fi
    log_info "Rust version: $(rustc --version)"
    log_info "Cargo version: $(cargo --version)"
}

# Install rustup components
install_rust_components() {
    log_info "Installing Rust components..."
    
    if rustup component add llvm-tools-preview 2>/dev/null; then
        pass "Installed llvm-tools-preview component"
    else
        log_warning "llvm-tools-preview already installed or failed to install"
    fi
}

# Install cargo-llvm-cov for local development and GitHub Actions
install_cargo_llvm_cov() {
    log_info "Installing cargo-llvm-cov..."
    
    if command -v cargo-llvm-cov >/dev/null 2>&1; then
        local current_version
        current_version=$(cargo-llvm-cov --version | head -n1 || echo "unknown")
        log_warning "cargo-llvm-cov is already installed: $current_version"
        
        # In CI, skip the interactive prompt
        if [ "${CI:-false}" = "true" ]; then
            log_info "Running in CI, skipping cargo-llvm-cov installation"
            return 0
        fi
        
        read -p "Do you want to reinstall/update? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            log_info "Skipping cargo-llvm-cov installation"
            return 0
        fi
    fi
    
    if cargo install cargo-llvm-cov; then
        pass "Installed cargo-llvm-cov"
        log_info "Version: $(cargo-llvm-cov --version)"
    else
        log_error "Failed to install cargo-llvm-cov"
        return 1
    fi
}

# Install grcov for GitLab CI
install_grcov() {
    log_info "Installing grcov..."
    
    # Check if running in GitLab CI
    if [ "${GITLAB_CI}" = "true" ]; then
        log_info "Detected GitLab CI environment"
        
        # Download pre-built binary for faster CI builds
        local grcov_version="v0.8.19"
        local grcov_url="https://github.com/mozilla/grcov/releases/download/${grcov_version}/grcov-x86_64-unknown-linux-gnu.tar.bz2"
        
        log_info "Downloading grcov ${grcov_version}..."
        if curl -L "${grcov_url}" | tar jxf -; then
            chmod +x grcov
            pass "Downloaded and extracted grcov binary"
            
            # Verify installation
            if ./grcov --version >/dev/null 2>&1; then
                pass "grcov is ready: $(./grcov --version)"
            else
                log_error "grcov binary is not working"
                return 1
            fi
        else
            log_error "Failed to download grcov"
            return 1
        fi
    else
        # For local installation, use cargo install
        if command -v grcov >/dev/null 2>&1; then
            local current_version
            current_version=$(grcov --version | head -n1 || echo "unknown")
            log_warning "grcov is already installed: $current_version"
            
            # In CI, skip the interactive prompt
            if [ "${CI:-false}" = "true" ]; then
                log_info "Running in CI, skipping grcov installation"
                return 0
            fi
            
            read -p "Do you want to reinstall/update? (y/N): " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                log_info "Skipping grcov installation"
                return 0
            fi
        fi
        
        if cargo install grcov; then
            pass "Installed grcov"
            log_info "Version: $(grcov --version)"
        else
            log_error "Failed to install grcov"
            return 1
        fi
    fi
}

# Verify installation
verify_installation() {
    local tool=$1
    log_info "Verifying ${tool} installation..."
    
    case $tool in
        cargo-llvm-cov)
            if command -v cargo-llvm-cov >/dev/null 2>&1; then
                pass "cargo-llvm-cov is installed: $(cargo-llvm-cov --version)"
                return 0
            else
                log_error "cargo-llvm-cov verification failed"
                return 1
            fi
            ;;
        grcov)
            if command -v grcov >/dev/null 2>&1 || [ -x ./grcov ]; then
                if [ -x ./grcov ]; then
                    pass "grcov is installed: $(./grcov --version)"
                else
                    pass "grcov is installed: $(grcov --version)"
                fi
                return 0
            else
                log_error "grcov verification failed"
                return 1
            fi
            ;;
        *)
            log_error "Unknown tool: $tool"
            return 1
            ;;
    esac
}

# Show post-installation instructions
show_instructions() {
    cat << EOF

Installation complete!

Usage Examples:

Local Development:
  # Run tests with coverage
  cargo llvm-cov --all-features --workspace

  # Generate HTML report and open in browser
  cargo llvm-cov --all-features --workspace --html --open

  # Generate LCOV report
  cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

  # Run with coverage threshold (fail if below 70%)
  cargo llvm-cov --all-features --workspace --fail-under-lines 70

  # Using Makefile targets
  make coverage          # Run with 70% threshold
  make coverage-html     # Generate HTML report
  make coverage-report   # Show coverage summary

GitLab CI:
  The .gitlab-ci.yml is already configured to use grcov.
  Coverage reports are automatically generated in the rust:build-test-lint job.

GitHub Actions:
  The .github/workflows/coverage.yml is already configured to use cargo-llvm-cov.
  Coverage reports are automatically uploaded to Codecov.

Documentation:
  cargo-llvm-cov: https://github.com/taiki-e/cargo-llvm-cov
  grcov:          https://github.com/mozilla/grcov

EOF
}

# Main installation logic
main() {
    local install_local=false
    local install_github=false
    local install_gitlab=false
    
    # Parse command line arguments
    if [ $# -eq 0 ]; then
        # Auto-detect environment
        if [ "${GITHUB_ACTIONS}" = "true" ]; then
            install_github=true
            log_info "Auto-detected GitHub Actions environment"
        elif [ "${GITLAB_CI}" = "true" ]; then
            install_gitlab=true
            log_info "Auto-detected GitLab CI environment"
        else
            install_local=true
            log_info "Auto-detected local development environment"
        fi
    else
        while [ $# -gt 0 ]; do
            case "$1" in
                --local)
                    install_local=true
                    ;;
                --github)
                    install_github=true
                    ;;
                --gitlab)
                    install_gitlab=true
                    ;;
                --all)
                    install_local=true
                    install_github=true
                    install_gitlab=true
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
    fi
    
    section "Starting coverage tools installation"
    
    # Check Rust installation
    check_rust
    echo
    
    # Install Rust components (needed by all tools)
    install_rust_components
    echo
    
    # Install tools based on selected options
    local success=true
    
    if [ "$install_local" = true ] || [ "$install_github" = true ]; then
        section "Installing tools for local/GitHub"
        if install_cargo_llvm_cov && verify_installation cargo-llvm-cov; then
            pass "Local/GitHub tools installed successfully"
        else
            log_error "Failed to install local/GitHub tools"
            success=false
        fi
        echo
    fi
    
    if [ "$install_gitlab" = true ]; then
        section "Installing tools for GitLab"
        if install_grcov && verify_installation grcov; then
            pass "GitLab tools installed successfully"
        else
            log_error "Failed to install GitLab tools"
            success=false
        fi
        echo
    fi
    
    # Show instructions if successful
    if [ "$success" = true ]; then
        show_instructions
        exit 0
    else
        log_error "Some tools failed to install"
        exit 1
    fi
}

# Run main function
main "$@"
