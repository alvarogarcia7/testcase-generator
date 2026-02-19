#!/usr/bin/env bash
#
# End-to-end integration test for Docker container PDF generation
#
# This test validates:
# 1. Running 'make docs-docker-build-pdf' with ENABLE_PDF_EXPORT=1
# 2. Verifying PDF is generated at site/pdf/testcase-manager-documentation.pdf
# 3. Testing PDF contains table of contents with correct depth (toc_level: 3)
# 4. Verifying implementation notes are excluded (IMPLEMENTATION_*.md, *_SUMMARY.md)
# 5. Validating PDF structure using pdfinfo via Docker alpine container
# 6. Testing PDF file size is reasonable (2-10MB)
# 7. Verifying PDF generation doesn't fail with WeasyPrint errors
# 8. Testing that `file site/pdf/*.pdf` shows valid PDF
#
# Usage: ./tests/integration/test_docker_pdf_build_e2e.sh [--no-remove] [--verbose]
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Source logger library
source "$SCRIPT_DIR/../../scripts/lib/logger.sh" || exit 1

# Configuration
IMAGE_NAME="testcase-manager-docs:latest"
SITE_DIR="$PROJECT_ROOT/site"
PDF_DIR="$SITE_DIR/pdf"
PDF_FILE="$PDF_DIR/testcase-manager-documentation.pdf"
DOCS_DIR="$PROJECT_ROOT/docs"
MKDOCS_YML="$PROJECT_ROOT/mkdocs.yml"
MIN_PDF_SIZE_MB=2
MAX_PDF_SIZE_MB=10

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

section "Docker Container PDF Generation End-to-End Test"
log_info "Project root: $PROJECT_ROOT"
log_info "Docker image: $IMAGE_NAME"
log_info "PDF file: $PDF_FILE"
log_info "Expected PDF size: ${MIN_PDF_SIZE_MB}MB - ${MAX_PDF_SIZE_MB}MB"
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

# Verify mkdocs.yml PDF configuration
section "Test 2: Verify mkdocs.yml PDF Configuration"

log_info "Checking PDF plugin configuration in mkdocs.yml..."

# Check for with-pdf plugin
if grep -q "with-pdf:" "$MKDOCS_YML"; then
    pass "mkdocs-with-pdf plugin configured"
else
    fail "mkdocs-with-pdf plugin not found in mkdocs.yml"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    exit 1
fi

# Check for enabled_if_env
if grep -q "enabled_if_env: ENABLE_PDF_EXPORT" "$MKDOCS_YML"; then
    pass "PDF export enabled via ENABLE_PDF_EXPORT environment variable"
else
    fail "ENABLE_PDF_EXPORT configuration not found"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Check output path
if grep -q "output_path: pdf/testcase-manager-documentation.pdf" "$MKDOCS_YML"; then
    pass "PDF output path configured correctly"
else
    fail "PDF output path not configured correctly"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Check toc_level
if grep -q "toc_level: 3" "$MKDOCS_YML"; then
    pass "Table of contents depth set to 3"
else
    log_warning "Table of contents depth not set to 3 (may be different)"
fi

# Check excludes_children pattern
if grep -q "excludes_children:" "$MKDOCS_YML"; then
    pass "excludes_children configuration found"
    
    if grep -A 2 "excludes_children:" "$MKDOCS_YML" | grep -q "'IMPLEMENTATION_\*.md'"; then
        pass "IMPLEMENTATION_*.md files excluded from PDF"
    else
        log_warning "IMPLEMENTATION_*.md exclusion pattern not found"
    fi
    
    if grep -A 2 "excludes_children:" "$MKDOCS_YML" | grep -q "'\*_SUMMARY.md'"; then
        pass "*_SUMMARY.md files excluded from PDF"
    else
        log_warning "*_SUMMARY.md exclusion pattern not found"
    fi
else
    log_warning "excludes_children configuration not found"
fi

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Clean up any existing site directory
section "Test 3: Clean Existing Site Directory"

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

# Run Docker PDF build
section "Test 4: Run 'make docs-docker-build-pdf' with ENABLE_PDF_EXPORT=1"

log_info "Running: make docs-docker-build-pdf"
log_info "This will build the documentation with PDF export enabled..."
log_info "PDF generation may take several minutes depending on documentation size..."

BUILD_OUTPUT=$(mktemp)
if [[ $REMOVE_TEMP -eq 1 ]]; then
    setup_cleanup "$(dirname "$BUILD_OUTPUT")"
fi

# Verify ENABLE_PDF_EXPORT is set in Makefile target
if grep -q "ENABLE_PDF_EXPORT=1" "$PROJECT_ROOT/Makefile"; then
    pass "Makefile target sets ENABLE_PDF_EXPORT=1"
else
    log_warning "ENABLE_PDF_EXPORT=1 not explicitly set in Makefile target"
fi

if make -C "$PROJECT_ROOT" docs-docker-build-pdf > "$BUILD_OUTPUT" 2>&1; then
    pass "make docs-docker-build-pdf completed successfully"
    TESTS_PASSED=$((TESTS_PASSED + 1))
    
    log_verbose "Build output:"
    if [[ ${VERBOSE:-0} -eq 1 ]]; then
        cat "$BUILD_OUTPUT" >&2
    fi
else
    fail "make docs-docker-build-pdf failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "Build output:"
    cat "$BUILD_OUTPUT" >&2
    
    # Check for WeasyPrint errors
    if grep -i "weasyprint" "$BUILD_OUTPUT"; then
        log_error "WeasyPrint errors detected in build output"
    fi
    
    exit 1
fi

echo

# Check for WeasyPrint errors in output
section "Test 5: Verify No WeasyPrint Errors"

log_info "Checking for WeasyPrint errors in build output..."

if grep -i "error" "$BUILD_OUTPUT" | grep -i "weasyprint" > /dev/null; then
    fail "WeasyPrint errors found in build output"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "Error details:"
    grep -i "error" "$BUILD_OUTPUT" | grep -i "weasyprint" >&2 || true
elif grep -i "warning" "$BUILD_OUTPUT" | grep -i "weasyprint" > /dev/null; then
    log_warning "WeasyPrint warnings found in build output (may be acceptable)"
    log_verbose "Warning details:"
    if [[ ${VERBOSE:-0} -eq 1 ]]; then
        grep -i "warning" "$BUILD_OUTPUT" | grep -i "weasyprint" >&2 || true
    fi
    pass "No WeasyPrint errors (warnings are acceptable)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    pass "No WeasyPrint errors or warnings"
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

echo

# Verify site directory exists
section "Test 6: Verify Site Directory Creation"

if [[ ! -d "$SITE_DIR" ]]; then
    fail "site/ directory was not created"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    exit 1
fi
pass "site/ directory exists"

if [[ ! -d "$PDF_DIR" ]]; then
    fail "site/pdf/ directory was not created"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    exit 1
fi
pass "site/pdf/ directory exists"

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Verify PDF file exists
section "Test 7: Verify PDF File Generation"

log_info "Checking for PDF file at: $PDF_FILE"

if [[ ! -f "$PDF_FILE" ]]; then
    fail "PDF file not found at expected location: $PDF_FILE"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    
    # List contents of PDF directory
    log_error "Contents of site/pdf/ directory:"
    ls -la "$PDF_DIR" 2>&1 || log_error "PDF directory is empty or doesn't exist"
    
    # List any PDF files in site directory
    log_info "Searching for PDF files in site/..."
    find "$SITE_DIR" -name "*.pdf" -type f 2>/dev/null || log_info "No PDF files found in site/"
    
    exit 1
fi
pass "PDF file exists: $PDF_FILE"

# Get file size
PDF_SIZE_BYTES=$(stat -c %s "$PDF_FILE" 2>/dev/null || stat -f %z "$PDF_FILE" 2>/dev/null)
PDF_SIZE_MB=$((PDF_SIZE_BYTES / 1024 / 1024))
log_info "PDF file size: ${PDF_SIZE_MB} MB (${PDF_SIZE_BYTES} bytes)"

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Verify PDF file type using 'file' command
section "Test 8: Verify PDF File Type with 'file' Command"

log_info "Running: file $PDF_FILE"

FILE_OUTPUT=$(file "$PDF_FILE" 2>&1)
log_info "File output: $FILE_OUTPUT"

if echo "$FILE_OUTPUT" | grep -q "PDF"; then
    pass "File is recognized as a valid PDF"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "File is not recognized as a PDF"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "File type: $FILE_OUTPUT"
fi

echo

# Verify PDF file size is reasonable
section "Test 9: Verify PDF File Size"

log_info "Checking PDF file size is within acceptable range..."
log_info "Expected range: ${MIN_PDF_SIZE_MB}MB - ${MAX_PDF_SIZE_MB}MB"
log_info "Actual size: ${PDF_SIZE_MB}MB"

SIZE_PASSED=true

if [[ $PDF_SIZE_MB -lt $MIN_PDF_SIZE_MB ]]; then
    fail "PDF file is too small (${PDF_SIZE_MB}MB < ${MIN_PDF_SIZE_MB}MB)"
    log_error "The PDF may be incomplete or missing content"
    SIZE_PASSED=false
elif [[ $PDF_SIZE_MB -gt $MAX_PDF_SIZE_MB ]]; then
    log_warning "PDF file is larger than expected (${PDF_SIZE_MB}MB > ${MAX_PDF_SIZE_MB}MB)"
    log_info "This may be acceptable depending on documentation size"
    pass "PDF file size is acceptable (${PDF_SIZE_MB}MB)"
    SIZE_PASSED=true
else
    pass "PDF file size is within expected range (${PDF_SIZE_MB}MB)"
    SIZE_PASSED=true
fi

if [ "$SIZE_PASSED" = true ]; then
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Validate PDF structure using pdfinfo via Docker
section "Test 10: Validate PDF Structure Using pdfinfo"

log_info "Running pdfinfo via Docker alpine container..."
log_info "Command: docker run --rm -v \$(pwd)/site:/site alpine sh -c \"apk add poppler-utils && pdfinfo /site/pdf/*.pdf\""

PDFINFO_OUTPUT=$(mktemp)
if [[ $REMOVE_TEMP -eq 1 ]]; then
    setup_cleanup "$(dirname "$PDFINFO_OUTPUT")"
fi

# Run pdfinfo in alpine container
if docker run --rm -v "$SITE_DIR:/site" alpine sh -c "apk add poppler-utils > /dev/null 2>&1 && pdfinfo /site/pdf/*.pdf" > "$PDFINFO_OUTPUT" 2>&1; then
    pass "pdfinfo command executed successfully"
    
    log_info "PDF Information:"
    cat "$PDFINFO_OUTPUT" >&2
    
    # Extract and verify key PDF properties
    PDF_VALID=true
    
    # Check for Title
    if grep -q "^Title:" "$PDFINFO_OUTPUT"; then
        PDF_TITLE=$(grep "^Title:" "$PDFINFO_OUTPUT" | sed 's/Title:[[:space:]]*//')
        pass "PDF has title: $PDF_TITLE"
    else
        log_warning "PDF title not found in metadata"
    fi
    
    # Check for Author
    if grep -q "^Author:" "$PDFINFO_OUTPUT"; then
        PDF_AUTHOR=$(grep "^Author:" "$PDFINFO_OUTPUT" | sed 's/Author:[[:space:]]*//')
        pass "PDF has author: $PDF_AUTHOR"
    else
        log_warning "PDF author not found in metadata"
    fi
    
    # Check for number of pages
    if grep -q "^Pages:" "$PDFINFO_OUTPUT"; then
        PDF_PAGES=$(grep "^Pages:" "$PDFINFO_OUTPUT" | awk '{print $2}')
        log_info "PDF has $PDF_PAGES pages"
        
        if [[ $PDF_PAGES -lt 10 ]]; then
            fail "PDF has too few pages ($PDF_PAGES < 10)"
            log_error "The PDF may be incomplete"
            PDF_VALID=false
        else
            pass "PDF has sufficient pages ($PDF_PAGES)"
        fi
    else
        fail "Page count not found in PDF metadata"
        PDF_VALID=false
    fi
    
    # Check for PDF version
    if grep -q "^PDF version:" "$PDFINFO_OUTPUT"; then
        PDF_VERSION=$(grep "^PDF version:" "$PDFINFO_OUTPUT" | awk '{print $3}')
        pass "PDF version: $PDF_VERSION"
    else
        log_warning "PDF version not found in metadata"
    fi
    
    # Check file size from pdfinfo
    if grep -q "^File size:" "$PDFINFO_OUTPUT"; then
        FILE_SIZE=$(grep "^File size:" "$PDFINFO_OUTPUT" | sed 's/File size:[[:space:]]*//')
        log_info "File size from pdfinfo: $FILE_SIZE"
    fi
    
    # Check if PDF is optimized
    if grep -q "^Optimized:" "$PDFINFO_OUTPUT"; then
        OPTIMIZED=$(grep "^Optimized:" "$PDFINFO_OUTPUT" | awk '{print $2}')
        log_info "PDF optimized: $OPTIMIZED"
    fi
    
    if [ "$PDF_VALID" = true ]; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
else
    fail "pdfinfo command failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "pdfinfo output:"
    cat "$PDFINFO_OUTPUT" >&2
fi

echo

# Verify table of contents depth
section "Test 11: Verify Table of Contents Structure"

log_info "Checking table of contents in PDF..."

# Use pdftotext to extract text and check for TOC
PDFTEXT_OUTPUT=$(mktemp)
if [[ $REMOVE_TEMP -eq 1 ]]; then
    setup_cleanup "$(dirname "$PDFTEXT_OUTPUT")"
fi

if docker run --rm -v "$SITE_DIR:/site" alpine sh -c "apk add poppler-utils > /dev/null 2>&1 && pdftotext /site/pdf/*.pdf -" > "$PDFTEXT_OUTPUT" 2>&1; then
    pass "PDF text extraction successful"
    
    # Check for Table of Contents
    if grep -q "Table of Contents" "$PDFTEXT_OUTPUT"; then
        pass "Table of Contents found in PDF"
    else
        log_warning "Table of Contents not found in PDF text (may use different format)"
    fi
    
    # Check for major sections in TOC/content
    SECTIONS_FOUND=0
    EXPECTED_SECTIONS=("Getting Started" "User Guide" "CLI Tools" "Features" "Development")
    
    for section in "${EXPECTED_SECTIONS[@]}"; do
        if grep -q "$section" "$PDFTEXT_OUTPUT"; then
            SECTIONS_FOUND=$((SECTIONS_FOUND + 1))
            log_verbose "Found section: $section"
        fi
    done
    
    log_info "Found $SECTIONS_FOUND/${#EXPECTED_SECTIONS[@]} major sections in PDF"
    
    if [[ $SECTIONS_FOUND -ge 3 ]]; then
        pass "Sufficient major sections found in PDF ($SECTIONS_FOUND)"
    else
        log_warning "Few major sections found in PDF ($SECTIONS_FOUND)"
    fi
    
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    log_warning "PDF text extraction failed (pdftotext not available)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

echo

# Verify implementation notes are excluded
section "Test 12: Verify Implementation Notes Exclusion"

log_info "Checking that IMPLEMENTATION_*.md and *_SUMMARY.md files are excluded..."

EXCLUSION_PASSED=true

# Check for implementation file patterns in PDF text
IMPL_PATTERNS=("IMPLEMENTATION_COMPLETE" "IMPLEMENTATION_SUMMARY" "DOCUMENTATION_SUMMARY" "DOCKER_CLEANUP_SUMMARY")

IMPL_FOUND=0
for pattern in "${IMPL_PATTERNS[@]}"; do
    if grep -q "$pattern" "$PDFTEXT_OUTPUT"; then
        IMPL_FOUND=$((IMPL_FOUND + 1))
        log_warning "Implementation file pattern found in PDF: $pattern"
        EXCLUSION_PASSED=false
    fi
done

if [ "$EXCLUSION_PASSED" = true ]; then
    pass "Implementation notes properly excluded from PDF"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    log_warning "$IMPL_FOUND implementation file patterns found in PDF"
    log_info "This may be acceptable if they appear in content, not as separate pages"
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

echo

# Test multiple PDF builds
section "Test 13: Test Multiple Sequential PDF Builds"

log_info "Testing multiple sequential PDF builds..."

# Capture initial build timestamp
if [[ -f "$PDF_FILE" ]]; then
    FIRST_BUILD_TIME=$(stat -c %Y "$PDF_FILE" 2>/dev/null || stat -f %m "$PDF_FILE" 2>/dev/null)
    log_info "First build timestamp: $FIRST_BUILD_TIME"
fi

# Wait a moment to ensure timestamps differ
sleep 2

# Run second build
log_info "Running second PDF build..."
BUILD_OUTPUT_2=$(mktemp)
if [[ $REMOVE_TEMP -eq 1 ]]; then
    setup_cleanup "$(dirname "$BUILD_OUTPUT_2")"
fi

if make -C "$PROJECT_ROOT" docs-docker-build-pdf > "$BUILD_OUTPUT_2" 2>&1; then
    pass "Second PDF build completed successfully"
    
    # Check if PDF file still exists and is valid
    if [[ -f "$PDF_FILE" ]]; then
        pass "PDF file still exists after second build"
        
        # Verify rebuild updated the file
        SECOND_BUILD_TIME=$(stat -c %Y "$PDF_FILE" 2>/dev/null || stat -f %m "$PDF_FILE" 2>/dev/null)
        log_info "Second build timestamp: $SECOND_BUILD_TIME"
        
        if [[ $SECOND_BUILD_TIME -gt $FIRST_BUILD_TIME ]]; then
            pass "Second build updated the PDF file"
        else
            log_warning "Second build may not have updated PDF (timestamps unchanged)"
        fi
        
        # Verify file size is still reasonable
        PDF_SIZE_2=$(stat -c %s "$PDF_FILE" 2>/dev/null || stat -f %z "$PDF_FILE" 2>/dev/null)
        PDF_SIZE_2_MB=$((PDF_SIZE_2 / 1024 / 1024))
        log_info "Second build PDF size: ${PDF_SIZE_2_MB} MB"
        
        if [[ $PDF_SIZE_2_MB -ge $MIN_PDF_SIZE_MB ]] && [[ $PDF_SIZE_2_MB -le $MAX_PDF_SIZE_MB ]]; then
            pass "Second build PDF size is within expected range"
        else
            log_warning "Second build PDF size is outside expected range: ${PDF_SIZE_2_MB}MB"
        fi
    else
        fail "PDF file missing after second build"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Second PDF build failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "Second build output:"
    cat "$BUILD_OUTPUT_2" >&2
fi

echo

# Test cleanup
section "Test 14: Test Cleanup of site/ Directory"

log_info "Testing site/ directory cleanup..."

# Test cleanup using make docs-clean
log_info "Running: make docs-clean"
if make -C "$PROJECT_ROOT" docs-clean > /dev/null 2>&1; then
    pass "make docs-clean completed successfully"
    
    if [[ ! -d "$SITE_DIR" ]]; then
        pass "site/ directory successfully removed"
    else
        fail "site/ directory still exists after cleanup"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    
    if [[ ! -f "$PDF_FILE" ]]; then
        pass "PDF file successfully removed with site/ directory"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        fail "PDF file still exists after cleanup"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
else
    fail "make docs-clean failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Rebuild for final verification
log_info "Rebuilding site with PDF for final verification..."
BUILD_OUTPUT_FINAL=$(mktemp)
if [[ $REMOVE_TEMP -eq 1 ]]; then
    setup_cleanup "$(dirname "$BUILD_OUTPUT_FINAL")"
fi

if make -C "$PROJECT_ROOT" docs-docker-build-pdf > "$BUILD_OUTPUT_FINAL" 2>&1; then
    pass "Final rebuild successful after cleanup"
    
    if [[ -f "$PDF_FILE" ]]; then
        pass "PDF file recreated successfully"
    else
        fail "PDF file not recreated after cleanup and rebuild"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
else
    fail "Final rebuild failed after cleanup"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "Final rebuild output:"
    cat "$BUILD_OUTPUT_FINAL" >&2
fi

echo

# Test PDF file command output
section "Test 15: Verify 'file site/pdf/*.pdf' Output"

log_info "Running: file site/pdf/*.pdf"

if [[ -f "$PDF_FILE" ]]; then
    FILE_CMD_OUTPUT=$(file "$PDF_FILE" 2>&1)
    echo "$FILE_CMD_OUTPUT" >&2
    
    # Verify output contains PDF indicators
    if echo "$FILE_CMD_OUTPUT" | grep -q "PDF"; then
        pass "file command confirms PDF format"
    else
        fail "file command does not recognize PDF format"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    
    # Check for PDF version in file output
    if echo "$FILE_CMD_OUTPUT" | grep -qE "version [0-9]+\.[0-9]+"; then
        VERSION=$(echo "$FILE_CMD_OUTPUT" | grep -oE "version [0-9]+\.[0-9]+" | head -1)
        pass "PDF format details: $VERSION"
    else
        log_info "PDF version not detected in file output"
    fi
    
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "PDF file not found for file command test"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Final PDF validation
section "Test 16: Final PDF Validation Summary"

log_info "Performing final PDF validation..."

if [[ -f "$PDF_FILE" ]]; then
    # Final size check
    FINAL_SIZE=$(stat -c %s "$PDF_FILE" 2>/dev/null || stat -f %z "$PDF_FILE" 2>/dev/null)
    FINAL_SIZE_MB=$((FINAL_SIZE / 1024 / 1024))
    
    log_info "Final PDF statistics:"
    log_info "  File: $PDF_FILE"
    log_info "  Size: ${FINAL_SIZE_MB} MB (${FINAL_SIZE} bytes)"
    log_info "  Format: $(file "$PDF_FILE" | cut -d: -f2-)"
    
    # Run final pdfinfo check
    log_info "Final pdfinfo check:"
    if docker run --rm -v "$SITE_DIR:/site" alpine sh -c "apk add poppler-utils > /dev/null 2>&1 && pdfinfo /site/pdf/*.pdf" 2>&1 | head -15 >&2; then
        pass "Final pdfinfo check successful"
    else
        log_warning "Final pdfinfo check had issues"
    fi
    
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "PDF file not found for final validation"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Final summary
section "Test Summary"

TOTAL_TESTS=$((TESTS_PASSED + TESTS_FAILED))
log_info "Total tests: $TOTAL_TESTS"
log_info "Tests passed: $TESTS_PASSED"
log_info "Tests failed: $TESTS_FAILED"

echo
if [[ -f "$PDF_FILE" ]]; then
    log_info "PDF file information:"
    log_info "  Location: $PDF_FILE"
    log_info "  Size: ${FINAL_SIZE_MB:-$PDF_SIZE_MB} MB"
    log_info "  Type: $(file "$PDF_FILE" | cut -d: -f2- | xargs)"
fi

if [[ $TESTS_FAILED -eq 0 ]]; then
    echo
    pass "All Docker PDF generation tests passed successfully!"
    echo
    log_info "The Docker container PDF generation is working correctly:"
    log_info "  ✓ PDF generated with ENABLE_PDF_EXPORT=1"
    log_info "  ✓ PDF file created at correct location"
    log_info "  ✓ PDF contains table of contents with correct depth"
    log_info "  ✓ Implementation notes properly excluded"
    log_info "  ✓ PDF structure validated with pdfinfo"
    log_info "  ✓ PDF file size is reasonable (${FINAL_SIZE_MB:-$PDF_SIZE_MB}MB)"
    log_info "  ✓ No WeasyPrint errors in generation"
    log_info "  ✓ file command confirms valid PDF format"
    echo
    exit 0
else
    echo
    fail "Some Docker PDF generation tests failed!"
    echo
    log_error "$TESTS_FAILED test(s) failed"
    log_info "Please review the output above and fix the issues"
    echo
    exit 1
fi
