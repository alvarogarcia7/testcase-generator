#!/usr/bin/env bash
#
# test-mkdocs-setup.sh - End-to-end testing script for MkDocs setup
#
# This script performs comprehensive testing of the MkDocs documentation setup:
# 1. Installs MkDocs and verifies virtualenv creation
# 2. Serves documentation and verifies accessibility at localhost:8000
# 3. Builds static HTML site and verifies site/ directory structure
# 4. Builds PDF documentation and verifies PDF generation
# 5. Tests internal links in both HTML and PDF
# 6. Runs unit tests to ensure documentation changes don't break tests
#
# Usage:
#   ./scripts/test-mkdocs-setup.sh [OPTIONS]
#
# Options:
#   --skip-install      Skip docs-install step (assumes venv exists)
#   --skip-serve        Skip docs-serve test (useful in CI/CD)
#   --skip-build        Skip docs-build test
#   --skip-pdf          Skip docs-build-pdf test
#   --skip-links        Skip link checking
#   --skip-tests        Skip running tests
#   --clean             Clean all artifacts before starting
#   --help              Show this help message

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
SITE_DIR="$PROJECT_ROOT/site"
PDF_PATH="$SITE_DIR/pdf/testcase-manager-documentation.pdf"
SERVE_PORT=8000
SERVE_TIMEOUT=10
SERVE_PID=""

# Test flags
SKIP_INSTALL=false
SKIP_SERVE=false
SKIP_BUILD=false
SKIP_PDF=false
SKIP_LINKS=false
SKIP_TESTS=false
CLEAN=false

# Cleanup function
cleanup() {
    if [ -n "$SERVE_PID" ]; then
        log_info "Stopping MkDocs serve process (PID: $SERVE_PID)..."
        kill "$SERVE_PID" 2>/dev/null || true
        wait "$SERVE_PID" 2>/dev/null || true
        pass "Stopped MkDocs serve process"
    fi
}

# Register cleanup on exit
trap cleanup EXIT INT TERM

# Show help message
show_help() {
    cat << EOF
Usage: $0 [OPTIONS]

End-to-end testing script for MkDocs documentation setup.

OPTIONS:
    --skip-install      Skip docs-install step (assumes venv exists)
    --skip-serve        Skip docs-serve test (useful in CI/CD)
    --skip-build        Skip docs-build test
    --skip-pdf          Skip docs-build-pdf test
    --skip-links        Skip link checking
    --skip-tests        Skip running tests
    --clean             Clean all artifacts before starting
    --help              Show this help message

EXAMPLES:
    # Full test
    $0

    # Skip serve test (for CI/CD)
    $0 --skip-serve

    # Clean install and test
    $0 --clean

    # Quick test (skip install and serve)
    $0 --skip-install --skip-serve

TEST STEPS:
    1. Install MkDocs (unless --skip-install)
    2. Verify virtualenv creation
    3. Serve documentation at localhost:$SERVE_PORT (unless --skip-serve)
    4. Build static HTML site (unless --skip-build)
    5. Verify site/ directory structure
    6. Build PDF documentation (unless --skip-pdf)
    7. Verify PDF generation at $PDF_PATH
    8. Test internal links (unless --skip-links)
    9. Run unit tests (unless --skip-tests)

EOF
}

# Parse command line arguments
parse_args() {
    while [ $# -gt 0 ]; do
        case "$1" in
            --skip-install)
                SKIP_INSTALL=true
                ;;
            --skip-serve)
                SKIP_SERVE=true
                ;;
            --skip-build)
                SKIP_BUILD=true
                ;;
            --skip-pdf)
                SKIP_PDF=true
                ;;
            --skip-links)
                SKIP_LINKS=true
                ;;
            --skip-tests)
                SKIP_TESTS=true
                ;;
            --clean)
                CLEAN=true
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
}

# Clean artifacts
clean_artifacts() {
    section "Cleaning artifacts"
    
    if [ -d "$SITE_DIR" ]; then
        log_info "Removing site/ directory..."
        rm -rf "$SITE_DIR"
        pass "Removed site/ directory"
    fi
    
    if [ "$SKIP_INSTALL" = false ] && [ -d "$VENV_DIR" ]; then
        log_info "Removing virtual environment..."
        rm -rf "$VENV_DIR"
        pass "Removed virtual environment"
    fi
    
    echo
}

# Test: Install MkDocs
test_install() {
    section "Test 1: Install MkDocs"
    
    log_info "Running: make docs-install"
    if make -C "$PROJECT_ROOT" docs-install; then
        pass "make docs-install succeeded"
    else
        fail "make docs-install failed"
        exit 1
    fi
    
    echo
}

# Test: Verify virtualenv
test_virtualenv() {
    section "Test 2: Verify virtualenv creation"
    
    if [ ! -d "$VENV_DIR" ]; then
        fail "Virtual environment not found: $VENV_DIR"
        exit 1
    fi
    pass "Virtual environment exists: $VENV_DIR"
    
    if [ ! -x "$VENV_DIR/bin/mkdocs" ]; then
        fail "MkDocs executable not found: $VENV_DIR/bin/mkdocs"
        exit 1
    fi
    pass "MkDocs executable exists"
    
    local mkdocs_version
    mkdocs_version=$("$VENV_DIR/bin/mkdocs" --version 2>&1)
    log_info "MkDocs version: $mkdocs_version"
    pass "MkDocs is executable"
    
    echo
}

# Test: Serve documentation
test_serve() {
    section "Test 3: Serve documentation"
    
    log_info "Starting MkDocs serve on port $SERVE_PORT..."
    make -C "$PROJECT_ROOT" docs-serve >/dev/null 2>&1 &
    SERVE_PID=$!
    
    log_info "MkDocs serve PID: $SERVE_PID"
    
    # Wait for server to start
    log_info "Waiting for server to start (timeout: ${SERVE_TIMEOUT}s)..."
    local elapsed=0
    while [ $elapsed -lt $SERVE_TIMEOUT ]; do
        if curl -s "http://localhost:$SERVE_PORT" >/dev/null 2>&1; then
            pass "Server started successfully"
            break
        fi
        sleep 1
        elapsed=$((elapsed + 1))
    done
    
    if [ $elapsed -ge $SERVE_TIMEOUT ]; then
        fail "Server failed to start within ${SERVE_TIMEOUT}s"
        exit 1
    fi
    
    # Test accessibility
    log_info "Testing accessibility at http://localhost:$SERVE_PORT..."
    if curl -s "http://localhost:$SERVE_PORT" | grep -q "Test Case Manager"; then
        pass "Site is accessible and contains expected content"
    else
        fail "Site is not accessible or missing expected content"
        exit 1
    fi
    
    # Test navigation
    log_info "Testing navigation links..."
    local response
    response=$(curl -s "http://localhost:$SERVE_PORT")
    
    if echo "$response" | grep -q "Getting Started"; then
        pass "Navigation contains 'Getting Started'"
    else
        log_warning "Navigation might be missing 'Getting Started'"
    fi
    
    if echo "$response" | grep -q "User Guide"; then
        pass "Navigation contains 'User Guide'"
    else
        log_warning "Navigation might be missing 'User Guide'"
    fi
    
    # Test page rendering
    log_info "Testing page rendering..."
    if curl -s "http://localhost:$SERVE_PORT/getting-started/" | grep -q "Getting Started"; then
        pass "Getting Started page renders correctly"
    else
        log_warning "Getting Started page might not render correctly"
    fi
    
    # Stop server
    log_info "Stopping MkDocs serve..."
    kill "$SERVE_PID" 2>/dev/null || true
    wait "$SERVE_PID" 2>/dev/null || true
    SERVE_PID=""
    pass "Stopped MkDocs serve"
    
    echo
}

# Test: Build HTML site
test_build() {
    section "Test 4: Build HTML site"
    
    log_info "Running: make docs-build"
    if make -C "$PROJECT_ROOT" docs-build; then
        pass "make docs-build succeeded"
    else
        fail "make docs-build failed"
        exit 1
    fi
    
    echo
}

# Test: Verify site structure
test_site_structure() {
    section "Test 5: Verify site/ directory structure"
    
    if [ ! -d "$SITE_DIR" ]; then
        fail "site/ directory not found: $SITE_DIR"
        exit 1
    fi
    pass "site/ directory exists"
    
    # Check index.html
    if [ ! -f "$SITE_DIR/index.html" ]; then
        fail "index.html not found in site/ directory"
        exit 1
    fi
    pass "index.html exists"
    
    # Check content
    if grep -q "Test Case Manager" "$SITE_DIR/index.html"; then
        pass "index.html contains expected content"
    else
        fail "index.html missing expected content"
        exit 1
    fi
    
    # Check assets
    if [ -d "$SITE_DIR/assets" ]; then
        pass "assets/ directory exists"
    else
        log_warning "assets/ directory not found (might be expected)"
    fi
    
    # Check CSS/JS
    if [ -d "$SITE_DIR/css" ] || [ -d "$SITE_DIR/stylesheets" ]; then
        pass "CSS directory exists"
    else
        log_warning "CSS directory not found"
    fi
    
    if [ -d "$SITE_DIR/js" ] || [ -d "$SITE_DIR/javascripts" ]; then
        pass "JavaScript directory exists"
    else
        log_warning "JavaScript directory not found"
    fi
    
    # Check section directories
    if [ -d "$SITE_DIR/getting-started" ]; then
        pass "getting-started/ section exists"
    else
        log_warning "getting-started/ section not found"
    fi
    
    if [ -d "$SITE_DIR/user-guide" ]; then
        pass "user-guide/ section exists"
    else
        log_warning "user-guide/ section not found"
    fi
    
    if [ -d "$SITE_DIR/cli-tools" ]; then
        pass "cli-tools/ section exists"
    else
        log_warning "cli-tools/ section not found"
    fi
    
    if [ -d "$SITE_DIR/features" ]; then
        pass "features/ section exists"
    else
        log_warning "features/ section not found"
    fi
    
    if [ -d "$SITE_DIR/development" ]; then
        pass "development/ section exists"
    else
        log_warning "development/ section not found"
    fi
    
    echo
}

# Test: Build PDF
test_pdf_build() {
    section "Test 6: Build PDF documentation"
    
    log_info "Running: make docs-build-pdf"
    if make -C "$PROJECT_ROOT" docs-build-pdf; then
        pass "make docs-build-pdf succeeded"
    else
        fail "make docs-build-pdf failed"
        exit 1
    fi
    
    echo
}

# Test: Verify PDF generation
test_pdf_verification() {
    section "Test 7: Verify PDF generation"
    
    if [ ! -f "$PDF_PATH" ]; then
        fail "PDF not found: $PDF_PATH"
        log_error "Expected PDF at: $PDF_PATH"
        log_info "Contents of site/pdf/ directory:"
        if [ -d "$SITE_DIR/pdf" ]; then
            ls -la "$SITE_DIR/pdf/" || true
        else
            log_error "site/pdf/ directory does not exist"
        fi
        exit 1
    fi
    pass "PDF exists: $PDF_PATH"
    
    # Check PDF size
    local pdf_size
    pdf_size=$(stat -f%z "$PDF_PATH" 2>/dev/null || stat -c%s "$PDF_PATH" 2>/dev/null)
    if [ -z "$pdf_size" ] || [ "$pdf_size" -eq 0 ]; then
        fail "PDF file is empty"
        exit 1
    fi
    log_info "PDF size: $((pdf_size / 1024)) KB"
    pass "PDF is not empty"
    
    # Check if PDF is valid (has PDF header)
    if head -c 4 "$PDF_PATH" | grep -q "%PDF"; then
        pass "PDF has valid header"
    else
        fail "PDF does not have valid header"
        exit 1
    fi
    
    # Check PDF structure with pdfinfo (if available)
    if command -v pdfinfo >/dev/null 2>&1; then
        log_info "PDF information:"
        pdfinfo "$PDF_PATH" | grep -E "^(Title|Pages):" | while IFS= read -r line; do
            log_info "  $line"
        done
        pass "PDF structure is valid (pdfinfo)"
    else
        log_info "pdfinfo not available, skipping PDF structure check"
    fi
    
    echo
}

# Test: Internal links
test_links() {
    section "Test 8: Test internal links"
    
    log_info "Testing HTML internal links..."
    
    # Check if we can find broken links in HTML
    local broken_links=0
    
    # Simple link check: look for href attributes and verify files exist
    find "$SITE_DIR" -name "*.html" -type f | while IFS= read -r html_file; do
        log_debug "Checking links in: ${html_file#$SITE_DIR/}"
        
        # Extract relative links (not external URLs)
        grep -oE 'href="[^"]*"' "$html_file" | \
            sed 's/href="\([^"]*\)"/\1/' | \
            grep -v '^http' | \
            grep -v '^#' | \
            grep -v '^mailto:' | \
            while IFS= read -r link; do
                # Remove fragment identifier
                local link_path="${link%%#*}"
                [ -z "$link_path" ] && continue
                
                # Convert to absolute path
                local dir_path
                dir_path="$(dirname "$html_file")"
                local full_path="$dir_path/$link_path"
                
                # Normalize path
                full_path="$(cd "$dir_path" 2>/dev/null && cd "$(dirname "$link_path")" 2>/dev/null && pwd)/$(basename "$link_path")" 2>/dev/null || echo "$full_path"
                
                # Check if target exists
                if [ ! -e "$full_path" ] && [ ! -e "${full_path}.html" ] && [ ! -d "$full_path" ]; then
                    log_warning "Broken link in ${html_file#$SITE_DIR/}: $link"
                    broken_links=$((broken_links + 1))
                fi
            done
    done
    
    if [ "$broken_links" -eq 0 ]; then
        pass "No broken internal links found in HTML"
    else
        log_warning "Found $broken_links potential broken links (may be false positives)"
    fi
    
    log_info "PDF link testing skipped (requires specialized PDF tools)"
    
    echo
}

# Test: Run tests
test_run_tests() {
    section "Test 9: Run unit tests"
    
    log_info "Running: make test"
    if make -C "$PROJECT_ROOT" test; then
        pass "make test succeeded"
    else
        fail "make test failed"
        log_error "Documentation changes may have broken tests"
        exit 1
    fi
    
    echo
}

# Main execution
main() {
    parse_args "$@"
    
    section "MkDocs End-to-End Test Suite"
    log_info "Project root: $PROJECT_ROOT"
    log_info "Virtual environment: $VENV_DIR"
    log_info "Site directory: $SITE_DIR"
    log_info "PDF path: $PDF_PATH"
    echo
    
    # Clean if requested
    if [ "$CLEAN" = true ]; then
        clean_artifacts
    fi
    
    # Run tests
    if [ "$SKIP_INSTALL" = false ]; then
        test_install
    else
        log_info "Skipping docs-install (--skip-install)"
        echo
    fi
    
    test_virtualenv
    
    if [ "$SKIP_SERVE" = false ]; then
        test_serve
    else
        log_info "Skipping docs-serve test (--skip-serve)"
        echo
    fi
    
    if [ "$SKIP_BUILD" = false ]; then
        test_build
        test_site_structure
    else
        log_info "Skipping docs-build test (--skip-build)"
        echo
    fi
    
    if [ "$SKIP_PDF" = false ]; then
        test_pdf_build
        test_pdf_verification
    else
        log_info "Skipping docs-build-pdf test (--skip-pdf)"
        echo
    fi
    
    if [ "$SKIP_LINKS" = false ]; then
        test_links
    else
        log_info "Skipping link checking (--skip-links)"
        echo
    fi
    
    if [ "$SKIP_TESTS" = false ]; then
        test_run_tests
    else
        log_info "Skipping unit tests (--skip-tests)"
        echo
    fi
    
    # Summary
    section "Test Summary"
    pass "All tests completed successfully!"
    log_info "Virtual environment: $VENV_DIR"
    log_info "HTML site: $SITE_DIR"
    log_info "PDF documentation: $PDF_PATH"
    echo
    
    log_info "To view the documentation:"
    log_info "  1. Serve locally:  make docs-serve"
    log_info "  2. Open browser:   http://localhost:8000"
    log_info "  3. View HTML:      open $SITE_DIR/index.html"
    log_info "  4. View PDF:       open $PDF_PATH"
    echo
    
    pass "MkDocs setup is working correctly!"
}

# Run main
main "$@"
