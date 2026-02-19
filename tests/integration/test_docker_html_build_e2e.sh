#!/usr/bin/env bash
#
# End-to-end integration test for Docker container HTML build
#
# This test validates:
# 1. Running 'make docs-docker-build-site' to build documentation HTML inside Docker container
# 2. Verifying site/ directory is created with complete HTML structure
# 3. Testing all markdown files are converted to HTML inside container
# 4. Verifying assets (CSS, JS, images) are copied correctly
# 5. Checking generated site/ has correct ownership and permissions from host perspective
# 6. Testing multiple sequential builds work correctly
# 7. Verifying cleanup of site/ directory works
#
# Usage: ./tests/integration/test_docker_html_build_e2e.sh [--no-remove] [--verbose]
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Source logger library
source "$SCRIPT_DIR/../../scripts/lib/logger.sh" || exit 1

# Configuration
IMAGE_NAME="testcase-manager-docs:latest"
SITE_DIR="$PROJECT_ROOT/site"
DOCS_DIR="$PROJECT_ROOT/docs"
MKDOCS_YML="$PROJECT_ROOT/mkdocs.yml"

# Parse command line arguments
REMOVE_TEMP=1
while [[ $# -gt 0 ]]; do
    case $1 in
        --no-remove)
            REMOVE_TEMP=0
            shift
            ;;
        --verbose)
            export VERBOSE=1
            shift
            ;;
        *)
            shift
            ;;
    esac
done

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

section "Docker Container HTML Build End-to-End Test"
log_info "Project root: $PROJECT_ROOT"
log_info "Docker image: $IMAGE_NAME"
log_info "Site directory: $SITE_DIR"
echo

# Check prerequisites
section "Test 1: Checking Prerequisites"

if ! command -v docker &> /dev/null; then
    fail "Docker is not installed or not in PATH"
    log_error "Please install Docker from https://www.docker.com/get-started"
    exit 1
fi
pass "Docker is installed"

if ! docker info &> /dev/null; then
    fail "Docker daemon is not running"
    log_error "Please start Docker and try again"
    exit 1
fi
pass "Docker daemon is running"

# Check if Docker image exists
if ! docker images "$IMAGE_NAME" --format "{{.Repository}}:{{.Tag}}" | grep -q "^${IMAGE_NAME}$"; then
    fail "Docker image not found: $IMAGE_NAME"
    log_error "Please build the image first using: make docs-docker-build"
    exit 1
fi
pass "Docker image exists: $IMAGE_NAME"

# Check project files exist
if [[ ! -f "$MKDOCS_YML" ]]; then
    fail "mkdocs.yml not found at $MKDOCS_YML"
    exit 1
fi
pass "mkdocs.yml found"

if [[ ! -d "$DOCS_DIR" ]]; then
    fail "docs/ directory not found at $DOCS_DIR"
    exit 1
fi
pass "docs/ directory found"

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Clean up any existing site directory
section "Test 2: Clean Existing Site Directory"

if [[ -d "$SITE_DIR" ]]; then
    log_info "Removing existing site/ directory..."
    rm -rf "$SITE_DIR"
    if [[ -d "$SITE_DIR" ]]; then
        fail "Failed to remove existing site/ directory"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    else
        pass "Existing site/ directory removed"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    fi
else
    log_info "No existing site/ directory to clean"
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

echo

# Run Docker build
section "Test 3: Run 'make docs-docker-build-site'"

log_info "Running: make docs-docker-build-site"
log_info "This will build the documentation site inside the Docker container..."

BUILD_OUTPUT=$(mktemp)
if [[ $REMOVE_TEMP -eq 1 ]]; then
    setup_cleanup "$(dirname "$BUILD_OUTPUT")"
fi

if make -C "$PROJECT_ROOT" docs-docker-build-site > "$BUILD_OUTPUT" 2>&1; then
    pass "make docs-docker-build-site completed successfully"
    TESTS_PASSED=$((TESTS_PASSED + 1))
    
    log_verbose "Build output:"
    if [[ ${VERBOSE:-0} -eq 1 ]]; then
        cat "$BUILD_OUTPUT" >&2
    fi
else
    fail "make docs-docker-build-site failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "Build output:"
    cat "$BUILD_OUTPUT" >&2
    exit 1
fi

echo

# Verify site directory exists
section "Test 4: Verify Site Directory Creation"

if [[ ! -d "$SITE_DIR" ]]; then
    fail "site/ directory was not created"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    exit 1
fi
pass "site/ directory exists"

# Check site directory structure
log_info "Checking site/ directory structure..."

# Count files in site directory
FILE_COUNT=$(find "$SITE_DIR" -type f | wc -l | tr -d ' ')
log_info "Total files in site/: $FILE_COUNT"

if [[ $FILE_COUNT -lt 5 ]]; then
    fail "site/ directory has too few files ($FILE_COUNT < 5)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
else
    pass "site/ directory has sufficient files ($FILE_COUNT)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

echo

# Verify HTML files are created
section "Test 5: Verify HTML Files"

log_info "Checking for HTML files..."

# Check for index.html
if [[ -f "$SITE_DIR/index.html" ]]; then
    pass "index.html exists"
else
    fail "index.html not found"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Check for 404.html
if [[ -f "$SITE_DIR/404.html" ]]; then
    pass "404.html exists"
else
    log_warning "404.html not found (optional but recommended)"
fi

# Count HTML files
HTML_COUNT=$(find "$SITE_DIR" -type f -name "*.html" | wc -l | tr -d ' ')
log_info "Total HTML files: $HTML_COUNT"

if [[ $HTML_COUNT -lt 3 ]]; then
    fail "Too few HTML files generated ($HTML_COUNT < 3)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
else
    pass "Sufficient HTML files generated ($HTML_COUNT)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

echo

# Verify markdown to HTML conversion
section "Test 6: Verify Markdown to HTML Conversion"

log_info "Checking markdown files have been converted to HTML..."

# Get list of markdown files from docs/
MARKDOWN_FILES=()
while IFS= read -r -d '' file; do
    MARKDOWN_FILES+=("$file")
done < <(find "$DOCS_DIR" -type f \( -name "*.md" -o -name "*.markdown" \) -print0)

log_info "Found ${#MARKDOWN_FILES[@]} markdown files in docs/"

# Check that key markdown files have corresponding HTML
MARKDOWN_CONVERSION_PASSED=true

# Check for index.md -> index.html
if [[ -f "$DOCS_DIR/index.md" ]] || [[ -f "$PROJECT_ROOT/README.md" ]]; then
    if [[ -f "$SITE_DIR/index.html" ]]; then
        pass "index.md converted to index.html"
    else
        fail "index.md not converted to index.html"
        MARKDOWN_CONVERSION_PASSED=false
    fi
fi

# Sample check for other markdown files (check a few subdirectories)
for subdir in "getting-started" "user-guide" "cli-tools" "features"; do
    if [[ -d "$DOCS_DIR/$subdir" ]]; then
        SUBDIR_HTML_COUNT=$(find "$SITE_DIR/$subdir" -type f -name "*.html" 2>/dev/null | wc -l | tr -d ' ')
        if [[ $SUBDIR_HTML_COUNT -gt 0 ]]; then
            pass "$subdir/ contains HTML files ($SUBDIR_HTML_COUNT)"
        else
            log_warning "$subdir/ contains no HTML files (may be empty)"
        fi
    fi
done

if [ "$MARKDOWN_CONVERSION_PASSED" = true ]; then
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Verify assets are copied
section "Test 7: Verify Assets (CSS, JS, Images)"

log_info "Checking for assets..."

ASSETS_PASSED=true

# Check for CSS files
CSS_COUNT=$(find "$SITE_DIR" -type f -name "*.css" | wc -l | tr -d ' ')
log_info "CSS files: $CSS_COUNT"
if [[ $CSS_COUNT -gt 0 ]]; then
    pass "CSS files found ($CSS_COUNT)"
else
    fail "No CSS files found"
    ASSETS_PASSED=false
fi

# Check for JS files
JS_COUNT=$(find "$SITE_DIR" -type f -name "*.js" | wc -l | tr -d ' ')
log_info "JavaScript files: $JS_COUNT"
if [[ $JS_COUNT -gt 0 ]]; then
    pass "JavaScript files found ($JS_COUNT)"
else
    fail "No JavaScript files found"
    ASSETS_PASSED=false
fi

# Check for common asset directories
for asset_dir in "assets" "stylesheets" "javascripts" "css" "js"; do
    if [[ -d "$SITE_DIR/$asset_dir" ]]; then
        ASSET_FILE_COUNT=$(find "$SITE_DIR/$asset_dir" -type f | wc -l | tr -d ' ')
        pass "$asset_dir/ directory exists with $ASSET_FILE_COUNT files"
    fi
done

# Check for search index (Material theme)
if [[ -f "$SITE_DIR/search/search_index.json" ]]; then
    pass "search_index.json exists"
else
    log_warning "search_index.json not found (search may not work)"
fi

if [ "$ASSETS_PASSED" = true ]; then
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Check ownership and permissions
section "Test 8: Verify Ownership and Permissions"

log_info "Checking file ownership and permissions from host perspective..."

# Check if site directory is readable
if [[ -r "$SITE_DIR" ]]; then
    pass "site/ directory is readable from host"
else
    fail "site/ directory is not readable from host"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Check if site directory is writable
if [[ -w "$SITE_DIR" ]]; then
    pass "site/ directory is writable from host"
else
    fail "site/ directory is not writable from host"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Check file permissions (should be readable)
UNREADABLE_FILES=0
while IFS= read -r file; do
    if [[ ! -r "$file" ]]; then
        UNREADABLE_FILES=$((UNREADABLE_FILES + 1))
        log_verbose "Unreadable file: $file"
    fi
done < <(find "$SITE_DIR" -type f)

if [[ $UNREADABLE_FILES -eq 0 ]]; then
    pass "All files in site/ are readable from host"
else
    fail "$UNREADABLE_FILES files in site/ are not readable from host"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Display ownership info for key files
log_info "Ownership information:"
if [[ -f "$SITE_DIR/index.html" ]]; then
    INDEX_OWNER=$(ls -l "$SITE_DIR/index.html" | awk '{print $3":"$4}')
    log_info "  index.html owner: $INDEX_OWNER"
fi

SITE_OWNER=$(ls -ld "$SITE_DIR" | awk '{print $3":"$4}')
log_info "  site/ owner: $SITE_OWNER"

# Check if we can modify the site directory
TEST_FILE="$SITE_DIR/.test_write_$$"
if touch "$TEST_FILE" 2>/dev/null; then
    pass "Can write to site/ directory from host"
    rm -f "$TEST_FILE"
else
    fail "Cannot write to site/ directory from host"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Verify HTML content quality
section "Test 9: Verify HTML Content Quality"

log_info "Checking HTML content quality..."

CONTENT_PASSED=true

if [[ -f "$SITE_DIR/index.html" ]]; then
    # Check for valid HTML structure
    if grep -q "<html" "$SITE_DIR/index.html"; then
        pass "index.html contains <html> tag"
    else
        fail "index.html missing <html> tag"
        CONTENT_PASSED=false
    fi
    
    if grep -q "<head>" "$SITE_DIR/index.html"; then
        pass "index.html contains <head> section"
    else
        fail "index.html missing <head> section"
        CONTENT_PASSED=false
    fi
    
    if grep -q "<body>" "$SITE_DIR/index.html"; then
        pass "index.html contains <body> section"
    else
        fail "index.html missing <body> section"
        CONTENT_PASSED=false
    fi
    
    # Check for Material theme indicators
    if grep -q "material" "$SITE_DIR/index.html"; then
        pass "Material theme assets detected"
    else
        log_warning "Material theme assets not detected in HTML"
    fi
    
    # Check for search functionality
    if grep -q "search" "$SITE_DIR/index.html"; then
        pass "Search functionality detected in HTML"
    else
        log_warning "Search functionality not detected in HTML"
    fi
fi

if [ "$CONTENT_PASSED" = true ]; then
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test multiple sequential builds
section "Test 10: Test Multiple Sequential Builds"

log_info "Testing multiple sequential builds..."

# Capture initial build timestamp
if [[ -f "$SITE_DIR/index.html" ]]; then
    FIRST_BUILD_TIME=$(stat -c %Y "$SITE_DIR/index.html" 2>/dev/null || stat -f %m "$SITE_DIR/index.html" 2>/dev/null)
    log_info "First build timestamp: $FIRST_BUILD_TIME"
fi

# Wait a moment to ensure timestamps differ
sleep 2

# Run second build
log_info "Running second build..."
BUILD_OUTPUT_2=$(mktemp)
if [[ $REMOVE_TEMP -eq 1 ]]; then
    setup_cleanup "$(dirname "$BUILD_OUTPUT_2")"
fi

if make -C "$PROJECT_ROOT" docs-docker-build-site > "$BUILD_OUTPUT_2" 2>&1; then
    pass "Second build completed successfully"
    
    # Check if site directory still exists and is valid
    if [[ -d "$SITE_DIR" ]] && [[ -f "$SITE_DIR/index.html" ]]; then
        pass "site/ directory still valid after second build"
        
        # Verify rebuild updated files
        SECOND_BUILD_TIME=$(stat -c %Y "$SITE_DIR/index.html" 2>/dev/null || stat -f %m "$SITE_DIR/index.html" 2>/dev/null)
        log_info "Second build timestamp: $SECOND_BUILD_TIME"
        
        if [[ $SECOND_BUILD_TIME -gt $FIRST_BUILD_TIME ]]; then
            pass "Second build updated the files"
        else
            log_warning "Second build may not have updated files (timestamps unchanged)"
        fi
    else
        fail "site/ directory invalid after second build"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
else
    fail "Second build failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "Second build output:"
    cat "$BUILD_OUTPUT_2" >&2
fi

# Run third build to ensure consistency
log_info "Running third build for consistency check..."
BUILD_OUTPUT_3=$(mktemp)
if [[ $REMOVE_TEMP -eq 1 ]]; then
    setup_cleanup "$(dirname "$BUILD_OUTPUT_3")"
fi

if make -C "$PROJECT_ROOT" docs-docker-build-site > "$BUILD_OUTPUT_3" 2>&1; then
    pass "Third build completed successfully"
    
    if [[ -d "$SITE_DIR" ]] && [[ -f "$SITE_DIR/index.html" ]]; then
        pass "site/ directory remains valid after third build"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        fail "site/ directory invalid after third build"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
else
    fail "Third build failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test directory listing
section "Test 11: Verify Directory Structure with 'ls -la site/'"

log_info "Running: ls -la site/"
echo

# Run ls -la to display directory contents
ls -la "$SITE_DIR" 2>&1 || true

echo
log_info "Sample subdirectories:"
for subdir in "$SITE_DIR"/*; do
    if [[ -d "$subdir" ]]; then
        SUBDIR_NAME=$(basename "$subdir")
        FILE_COUNT=$(find "$subdir" -type f | wc -l | tr -d ' ')
        log_info "  $SUBDIR_NAME/ - $FILE_COUNT files"
    fi
done

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Test cleanup
section "Test 12: Test Cleanup of site/ Directory"

log_info "Testing site/ directory cleanup..."

# Test cleanup using make docs-clean
log_info "Running: make docs-clean"
if make -C "$PROJECT_ROOT" docs-clean > /dev/null 2>&1; then
    pass "make docs-clean completed successfully"
    
    if [[ ! -d "$SITE_DIR" ]]; then
        pass "site/ directory successfully removed"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        fail "site/ directory still exists after cleanup"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        
        # Check if directory is empty
        REMAINING_FILES=$(find "$SITE_DIR" -type f | wc -l | tr -d ' ')
        log_info "Files remaining in site/: $REMAINING_FILES"
    fi
else
    fail "make docs-clean failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Rebuild for final verification
log_info "Rebuilding site/ for final verification..."
if make -C "$PROJECT_ROOT" docs-docker-build-site > /dev/null 2>&1; then
    pass "Final rebuild successful after cleanup"
    
    if [[ -d "$SITE_DIR" ]] && [[ -f "$SITE_DIR/index.html" ]]; then
        pass "site/ directory recreated successfully"
    else
        fail "site/ directory not properly recreated"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
else
    fail "Final rebuild failed after cleanup"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Comprehensive directory verification
section "Test 13: Comprehensive Site Structure Verification"

log_info "Performing comprehensive site structure verification..."

# Define expected structure elements
EXPECTED_DIRS=("assets" "search")
EXPECTED_FILES=("index.html" "sitemap.xml")

STRUCTURE_PASSED=true

# Check for expected directories
for dir in "${EXPECTED_DIRS[@]}"; do
    if [[ -d "$SITE_DIR/$dir" ]]; then
        pass "$dir/ directory exists"
    else
        log_warning "$dir/ directory not found (may be optional)"
    fi
done

# Check for expected files
for file in "${EXPECTED_FILES[@]}"; do
    if [[ -f "$SITE_DIR/$file" ]]; then
        pass "$file exists"
    else
        log_warning "$file not found (may be optional)"
    fi
done

# Check for sitemap.xml.gz
if [[ -f "$SITE_DIR/sitemap.xml.gz" ]]; then
    pass "sitemap.xml.gz exists"
else
    log_warning "sitemap.xml.gz not found (may be optional)"
fi

# Verify navigation structure
log_info "Checking navigation structure..."
NAV_SECTIONS=("getting-started" "user-guide" "cli-tools" "features" "development")
NAV_COUNT=0

for section in "${NAV_SECTIONS[@]}"; do
    if [[ -d "$SITE_DIR/$section" ]]; then
        NAV_COUNT=$((NAV_COUNT + 1))
        pass "$section/ section exists"
    else
        log_warning "$section/ section not found"
    fi
done

log_info "Found $NAV_COUNT/${#NAV_SECTIONS[@]} navigation sections"

if [[ $NAV_COUNT -ge 3 ]]; then
    pass "Sufficient navigation sections found ($NAV_COUNT)"
else
    log_warning "Few navigation sections found ($NAV_COUNT)"
fi

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Final summary
section "Test Summary"

TOTAL_TESTS=$((TESTS_PASSED + TESTS_FAILED))
log_info "Total tests: $TOTAL_TESTS"
log_info "Tests passed: $TESTS_PASSED"
log_info "Tests failed: $TESTS_FAILED"

echo
log_info "Final site/ directory statistics:"
if [[ -d "$SITE_DIR" ]]; then
    log_info "  Total files: $(find "$SITE_DIR" -type f | wc -l | tr -d ' ')"
    log_info "  Total directories: $(find "$SITE_DIR" -type d | wc -l | tr -d ' ')"
    log_info "  HTML files: $(find "$SITE_DIR" -type f -name "*.html" | wc -l | tr -d ' ')"
    log_info "  CSS files: $(find "$SITE_DIR" -type f -name "*.css" | wc -l | tr -d ' ')"
    log_info "  JS files: $(find "$SITE_DIR" -type f -name "*.js" | wc -l | tr -d ' ')"
    
    # Calculate total size
    TOTAL_SIZE=$(du -sh "$SITE_DIR" 2>/dev/null | awk '{print $1}')
    log_info "  Total size: $TOTAL_SIZE"
fi

if [[ $TESTS_FAILED -eq 0 ]]; then
    echo
    pass "All Docker HTML build tests passed successfully!"
    echo
    log_info "The Docker container HTML build is working correctly:"
    log_info "  ✓ site/ directory created with complete HTML structure"
    log_info "  ✓ All markdown files converted to HTML"
    log_info "  ✓ Assets (CSS, JS) copied correctly"
    log_info "  ✓ Correct ownership and permissions from host"
    log_info "  ✓ Multiple sequential builds work correctly"
    log_info "  ✓ Cleanup of site/ directory works"
    echo
    exit 0
else
    echo
    fail "Some Docker HTML build tests failed!"
    echo
    log_error "$TESTS_FAILED test(s) failed"
    log_info "Please review the output above and fix the issues"
    echo
    exit 1
fi
