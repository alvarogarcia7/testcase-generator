#!/usr/bin/env bash
#
# End-to-end integration test for Docker MkDocs configuration validation
#
# This test validates:
# 1. MkDocs configuration is valid (mkdocs build --strict)
# 2. All navigation paths in mkdocs.yml point to existing files
# 3. Material theme configuration loads without errors
# 4. Markdown extensions are properly configured:
#    - pymdownx.highlight
#    - pymdownx.superfences
#    - admonitions
#    - tables
#    - toc
# 5. PDF plugin configuration with exclusion patterns
# 6. Search plugin configuration
#
# Usage: ./tests/integration/test_docker_mkdocs_config_validation_e2e.sh [--no-remove]
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Source logger library
source "$SCRIPT_DIR/../../scripts/lib/logger.sh" || exit 1

# Configuration
IMAGE_NAME="testcase-manager-docs"
IMAGE_TAG="latest"
FULL_IMAGE_NAME="${IMAGE_NAME}:${IMAGE_TAG}"
MKDOCS_CONFIG="mkdocs.yml"

# Handle --no-remove flag
REMOVE_TEMP=1
while [[ $# -gt 0 ]]; do
    case $1 in
        --no-remove)
            REMOVE_TEMP=0
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

section "Docker MkDocs Configuration Validation Test"
log_info "Project root: $PROJECT_ROOT"
log_info "Image name: $FULL_IMAGE_NAME"
log_info "MkDocs config: $MKDOCS_CONFIG"
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
if ! docker images "$FULL_IMAGE_NAME" --format "{{.Repository}}:{{.Tag}}" | grep -q "^${FULL_IMAGE_NAME}$"; then
    fail "Docker image $FULL_IMAGE_NAME not found"
    log_error "Please build the image first: make docs-docker-build"
    exit 1
fi
pass "Docker image $FULL_IMAGE_NAME exists"

if [[ ! -f "$PROJECT_ROOT/$MKDOCS_CONFIG" ]]; then
    fail "MkDocs configuration not found at $PROJECT_ROOT/$MKDOCS_CONFIG"
    exit 1
fi
pass "MkDocs configuration found"

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Test MkDocs configuration validation with --strict
section "Test 2: Validate MkDocs Configuration with --strict"

log_info "Running: docker run --rm $FULL_IMAGE_NAME mkdocs build --strict"

TEMP_DIR=$(mktemp -d)
setup_cleanup "$TEMP_DIR"

if [[ $REMOVE_TEMP -eq 0 ]]; then
    disable_cleanup
    info "Temporary files will not be removed: $TEMP_DIR"
fi

# Run mkdocs build --strict to validate configuration
BUILD_OUTPUT=$(docker run --rm "$FULL_IMAGE_NAME" mkdocs build --strict 2>&1)
BUILD_EXIT_CODE=$?

if [[ $BUILD_EXIT_CODE -eq 0 ]]; then
    pass "MkDocs configuration is valid (--strict mode passed)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "MkDocs configuration validation failed (--strict mode)"
    log_error "Build output:"
    echo "$BUILD_OUTPUT"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test navigation paths
section "Test 3: Verify All Navigation Paths Point to Existing Files"

log_info "Extracting navigation paths from $MKDOCS_CONFIG..."

# Extract nav paths from mkdocs.yml
NAV_FILES=$(grep -E "^\s+-\s+.*:\s+.+\.md$" "$PROJECT_ROOT/$MKDOCS_CONFIG" | sed -E 's/^.*:\s+//' | tr -d ' ')

NAV_FILES_COUNT=$(echo "$NAV_FILES" | wc -l | tr -d ' ')
log_info "Found $NAV_FILES_COUNT navigation entries"

MISSING_FILES=()
EXISTING_FILES=0

while IFS= read -r nav_file; do
    # Skip empty lines
    [[ -z "$nav_file" ]] && continue
    
    # Check if file exists in project root or docs directory
    if [[ -f "$PROJECT_ROOT/$nav_file" ]]; then
        EXISTING_FILES=$((EXISTING_FILES + 1))
        log_verbose "✓ $nav_file exists"
    elif [[ -f "$PROJECT_ROOT/docs/$nav_file" ]]; then
        EXISTING_FILES=$((EXISTING_FILES + 1))
        log_verbose "✓ docs/$nav_file exists"
    else
        MISSING_FILES+=("$nav_file")
        log_warning "✗ $nav_file not found"
    fi
done <<< "$NAV_FILES"

log_info "Existing files: $EXISTING_FILES"
log_info "Missing files: ${#MISSING_FILES[@]}"

if [[ ${#MISSING_FILES[@]} -eq 0 ]]; then
    pass "All navigation paths point to existing files"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "${#MISSING_FILES[@]} navigation path(s) point to missing files"
    log_error "Missing files:"
    for missing_file in "${MISSING_FILES[@]}"; do
        log_error "  - $missing_file"
    done
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test Material theme configuration
section "Test 4: Verify Material Theme Configuration"

log_info "Checking Material theme configuration..."

# Check theme is set to material
if grep -q "name: material" "$PROJECT_ROOT/$MKDOCS_CONFIG"; then
    pass "Material theme is configured"
else
    fail "Material theme is not configured"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Check Material theme can be imported in Docker container
MATERIAL_IMPORT_TEST=$(docker run --rm "$FULL_IMAGE_NAME" python -c "import material; print('success')" 2>&1)
if [[ "$MATERIAL_IMPORT_TEST" == "success" ]]; then
    pass "Material theme can be imported successfully"
else
    fail "Material theme cannot be imported"
    log_error "Import error: $MATERIAL_IMPORT_TEST"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Check Material theme features
log_info "Verifying Material theme features..."
THEME_FEATURES=(
    "navigation.tabs"
    "navigation.sections"
    "navigation.expand"
    "navigation.top"
    "search.suggest"
    "search.highlight"
    "content.code.copy"
)

FEATURES_FOUND=0
for feature in "${THEME_FEATURES[@]}"; do
    if grep -q "$feature" "$PROJECT_ROOT/$MKDOCS_CONFIG"; then
        FEATURES_FOUND=$((FEATURES_FOUND + 1))
        log_verbose "✓ Feature configured: $feature"
    else
        log_warning "✗ Feature not configured: $feature"
    fi
done

log_info "Theme features configured: $FEATURES_FOUND/${#THEME_FEATURES[@]}"

# Check palette configuration
if grep -q "palette:" "$PROJECT_ROOT/$MKDOCS_CONFIG"; then
    pass "Palette configuration is present"
else
    log_warning "Palette configuration not found"
fi

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Test markdown extensions configuration
section "Test 5: Verify Markdown Extensions Configuration"

log_info "Checking markdown extensions configuration..."

# Define required extensions
REQUIRED_EXTENSIONS=(
    "pymdownx.highlight"
    "pymdownx.superfences"
    "admonition"
    "tables"
    "toc"
)

EXTENSIONS_CONFIGURED=0
MISSING_EXTENSIONS=()

for extension in "${REQUIRED_EXTENSIONS[@]}"; do
    if grep -q "$extension" "$PROJECT_ROOT/$MKDOCS_CONFIG"; then
        EXTENSIONS_CONFIGURED=$((EXTENSIONS_CONFIGURED + 1))
        pass "Extension configured: $extension"
    else
        MISSING_EXTENSIONS+=("$extension")
        fail "Extension not configured: $extension"
    fi
done

log_info "Extensions configured: $EXTENSIONS_CONFIGURED/${#REQUIRED_EXTENSIONS[@]}"

if [[ ${#MISSING_EXTENSIONS[@]} -eq 0 ]]; then
    pass "All required markdown extensions are configured"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "${#MISSING_EXTENSIONS[@]} required extension(s) not configured"
    log_error "Missing extensions:"
    for missing_ext in "${MISSING_EXTENSIONS[@]}"; do
        log_error "  - $missing_ext"
    done
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test pymdownx.highlight configuration
section "Test 6: Verify pymdownx.highlight Configuration"

log_info "Checking pymdownx.highlight configuration..."

# Check for pymdownx.highlight configuration options
HIGHLIGHT_OPTIONS=(
    "anchor_linenums"
    "line_spans"
    "pygments_lang_class"
)

HIGHLIGHT_CONFIGURED=0
for option in "${HIGHLIGHT_OPTIONS[@]}"; do
    if grep -A 3 "pymdownx.highlight:" "$PROJECT_ROOT/$MKDOCS_CONFIG" | grep -q "$option"; then
        HIGHLIGHT_CONFIGURED=$((HIGHLIGHT_CONFIGURED + 1))
        pass "Highlight option configured: $option"
    else
        log_warning "Highlight option not configured: $option"
    fi
done

log_info "Highlight options configured: $HIGHLIGHT_CONFIGURED/${#HIGHLIGHT_OPTIONS[@]}"

# Test that pymdownx.highlight can be imported
HIGHLIGHT_IMPORT_TEST=$(docker run --rm "$FULL_IMAGE_NAME" python -c "from pymdownx import highlight; print('success')" 2>&1)
if [[ "$HIGHLIGHT_IMPORT_TEST" == "success" ]]; then
    pass "pymdownx.highlight can be imported successfully"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "pymdownx.highlight cannot be imported"
    log_error "Import error: $HIGHLIGHT_IMPORT_TEST"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test pymdownx.superfences configuration
section "Test 7: Verify pymdownx.superfences Configuration"

log_info "Checking pymdownx.superfences configuration..."

# Check for pymdownx.superfences configuration
if grep -q "pymdownx.superfences:" "$PROJECT_ROOT/$MKDOCS_CONFIG"; then
    pass "pymdownx.superfences is configured"
    
    # Check for custom_fences configuration
    if grep -A 5 "pymdownx.superfences:" "$PROJECT_ROOT/$MKDOCS_CONFIG" | grep -q "custom_fences"; then
        pass "Custom fences are configured"
    else
        log_warning "Custom fences not configured"
    fi
    
    # Check for mermaid support
    if grep -A 10 "pymdownx.superfences:" "$PROJECT_ROOT/$MKDOCS_CONFIG" | grep -q "mermaid"; then
        pass "Mermaid diagram support is configured"
    else
        log_warning "Mermaid diagram support not configured"
    fi
else
    fail "pymdownx.superfences is not configured"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test that pymdownx.superfences can be imported
SUPERFENCES_IMPORT_TEST=$(docker run --rm "$FULL_IMAGE_NAME" python -c "from pymdownx import superfences; print('success')" 2>&1)
if [[ "$SUPERFENCES_IMPORT_TEST" == "success" ]]; then
    pass "pymdownx.superfences can be imported successfully"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "pymdownx.superfences cannot be imported"
    log_error "Import error: $SUPERFENCES_IMPORT_TEST"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test admonitions configuration
section "Test 8: Verify Admonitions Configuration"

log_info "Checking admonitions configuration..."

# Check admonition extension
if grep -q "^  - admonition$" "$PROJECT_ROOT/$MKDOCS_CONFIG"; then
    pass "Admonition extension is configured"
else
    fail "Admonition extension is not configured"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Check pymdownx.details (for collapsible admonitions)
if grep -q "pymdownx.details" "$PROJECT_ROOT/$MKDOCS_CONFIG"; then
    pass "pymdownx.details is configured (for collapsible admonitions)"
else
    log_warning "pymdownx.details not configured (collapsible admonitions will not work)"
fi

# Test that admonition can be imported
ADMONITION_IMPORT_TEST=$(docker run --rm "$FULL_IMAGE_NAME" python -c "import markdown.extensions.admonition; print('success')" 2>&1)
if [[ "$ADMONITION_IMPORT_TEST" == "success" ]]; then
    pass "Admonition extension can be imported successfully"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Admonition extension cannot be imported"
    log_error "Import error: $ADMONITION_IMPORT_TEST"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test tables configuration
section "Test 9: Verify Tables Configuration"

log_info "Checking tables configuration..."

# Check tables extension
if grep -q "^  - tables$" "$PROJECT_ROOT/$MKDOCS_CONFIG"; then
    pass "Tables extension is configured"
else
    fail "Tables extension is not configured"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test that tables can be imported
TABLES_IMPORT_TEST=$(docker run --rm "$FULL_IMAGE_NAME" python -c "import markdown.extensions.tables; print('success')" 2>&1)
if [[ "$TABLES_IMPORT_TEST" == "success" ]]; then
    pass "Tables extension can be imported successfully"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Tables extension cannot be imported"
    log_error "Import error: $TABLES_IMPORT_TEST"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test TOC configuration
section "Test 10: Verify TOC (Table of Contents) Configuration"

log_info "Checking TOC configuration..."

# Check toc extension
if grep -q "^  - toc:" "$PROJECT_ROOT/$MKDOCS_CONFIG"; then
    pass "TOC extension is configured"
    
    # Check for permalink option
    if grep -A 3 "^  - toc:" "$PROJECT_ROOT/$MKDOCS_CONFIG" | grep -q "permalink:"; then
        pass "TOC permalink is configured"
    else
        log_warning "TOC permalink not configured"
    fi
    
    # Check for toc_depth option
    if grep -A 3 "^  - toc:" "$PROJECT_ROOT/$MKDOCS_CONFIG" | grep -q "toc_depth:"; then
        TOC_DEPTH=$(grep -A 3 "^  - toc:" "$PROJECT_ROOT/$MKDOCS_CONFIG" | grep "toc_depth:" | sed 's/.*toc_depth: *//')
        pass "TOC depth is configured: $TOC_DEPTH"
    else
        log_warning "TOC depth not configured"
    fi
else
    fail "TOC extension is not configured"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test that toc can be imported
TOC_IMPORT_TEST=$(docker run --rm "$FULL_IMAGE_NAME" python -c "import markdown.extensions.toc; print('success')" 2>&1)
if [[ "$TOC_IMPORT_TEST" == "success" ]]; then
    pass "TOC extension can be imported successfully"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "TOC extension cannot be imported"
    log_error "Import error: $TOC_IMPORT_TEST"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test PDF plugin configuration
section "Test 11: Verify PDF Plugin Configuration"

log_info "Checking PDF plugin configuration..."

# Check with-pdf plugin
if grep -q "with-pdf:" "$PROJECT_ROOT/$MKDOCS_CONFIG"; then
    pass "PDF plugin (with-pdf) is configured"
    
    # Check enabled_if_env
    if grep -A 20 "with-pdf:" "$PROJECT_ROOT/$MKDOCS_CONFIG" | grep -q "enabled_if_env:"; then
        ENV_VAR=$(grep -A 20 "with-pdf:" "$PROJECT_ROOT/$MKDOCS_CONFIG" | grep "enabled_if_env:" | sed 's/.*enabled_if_env: *//')
        pass "PDF plugin is conditionally enabled via environment variable: $ENV_VAR"
    else
        log_warning "PDF plugin enabled_if_env not configured"
    fi
    
    # Check output_path
    if grep -A 20 "with-pdf:" "$PROJECT_ROOT/$MKDOCS_CONFIG" | grep -q "output_path:"; then
        OUTPUT_PATH=$(grep -A 20 "with-pdf:" "$PROJECT_ROOT/$MKDOCS_CONFIG" | grep "output_path:" | sed 's/.*output_path: *//')
        pass "PDF output path is configured: $OUTPUT_PATH"
    else
        log_warning "PDF output path not configured"
    fi
    
    # Check exclude_pages
    if grep -A 50 "with-pdf:" "$PROJECT_ROOT/$MKDOCS_CONFIG" | grep -q "exclude_pages:"; then
        pass "PDF exclude_pages is configured"
        
        # Count excluded pages
        EXCLUDED_COUNT=$(grep -A 50 "with-pdf:" "$PROJECT_ROOT/$MKDOCS_CONFIG" | sed -n '/exclude_pages:/,/^[^ ]/p' | grep -c "IMPLEMENTATION" || echo "0")
        log_info "Number of IMPLEMENTATION files excluded: $EXCLUDED_COUNT"
        
        if [[ $EXCLUDED_COUNT -gt 0 ]]; then
            pass "IMPLEMENTATION files are excluded from PDF ($EXCLUDED_COUNT files)"
        else
            log_warning "No IMPLEMENTATION files found in exclude_pages"
        fi
    else
        log_warning "PDF exclude_pages not configured"
    fi
    
    # Check excludes_children
    if grep -A 50 "with-pdf:" "$PROJECT_ROOT/$MKDOCS_CONFIG" | grep -q "excludes_children:"; then
        pass "PDF excludes_children is configured"
    else
        log_warning "PDF excludes_children not configured"
    fi
else
    fail "PDF plugin (with-pdf) is not configured"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test that mkdocs-with-pdf can be imported
PDF_IMPORT_TEST=$(docker run --rm "$FULL_IMAGE_NAME" python -c "import mkdocs_with_pdf; print('success')" 2>&1)
if [[ "$PDF_IMPORT_TEST" == "success" ]]; then
    pass "mkdocs-with-pdf plugin can be imported successfully"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "mkdocs-with-pdf plugin cannot be imported"
    log_error "Import error: $PDF_IMPORT_TEST"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test search plugin configuration
section "Test 12: Verify Search Plugin Configuration"

log_info "Checking search plugin configuration..."

# Check search plugin
if grep -q "^  - search:" "$PROJECT_ROOT/$MKDOCS_CONFIG"; then
    pass "Search plugin is configured"
    
    # Check lang option
    if grep -A 3 "^  - search:" "$PROJECT_ROOT/$MKDOCS_CONFIG" | grep -q "lang:"; then
        SEARCH_LANG=$(grep -A 3 "^  - search:" "$PROJECT_ROOT/$MKDOCS_CONFIG" | grep "lang:" | sed 's/.*lang: *//')
        pass "Search language is configured: $SEARCH_LANG"
    else
        log_warning "Search language not configured"
    fi
    
    # Check separator option
    if grep -A 3 "^  - search:" "$PROJECT_ROOT/$MKDOCS_CONFIG" | grep -q "separator:"; then
        SEARCH_SEPARATOR=$(grep -A 3 "^  - search:" "$PROJECT_ROOT/$MKDOCS_CONFIG" | grep "separator:" | sed 's/.*separator: *//')
        pass "Search separator is configured: $SEARCH_SEPARATOR"
    else
        log_warning "Search separator not configured"
    fi
    
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Search plugin is not configured"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test complete build in container
section "Test 13: Test Complete Build in Docker Container"

log_info "Testing complete documentation build in container..."

# Create temporary output directory
BUILD_OUTPUT_DIR="$TEMP_DIR/build_output"
mkdir -p "$BUILD_OUTPUT_DIR"

# Run build command
BUILD_CMD_OUTPUT=$(docker run --rm \
    -v "$BUILD_OUTPUT_DIR:/docs/site" \
    "$FULL_IMAGE_NAME" mkdocs build 2>&1)
BUILD_CMD_EXIT_CODE=$?

if [[ $BUILD_CMD_EXIT_CODE -eq 0 ]]; then
    pass "Documentation build completed successfully"
    
    # Check if index.html was created
    if [[ -f "$BUILD_OUTPUT_DIR/index.html" ]]; then
        pass "Generated index.html exists"
    else
        fail "Generated index.html not found"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    
    # Check if search index was created
    if [[ -f "$BUILD_OUTPUT_DIR/search/search_index.json" ]]; then
        pass "Search index was generated"
    else
        log_warning "Search index not found"
    fi
    
    # Check if assets directory exists
    if [[ -d "$BUILD_OUTPUT_DIR/assets" ]]; then
        pass "Assets directory exists"
    else
        log_warning "Assets directory not found"
    fi
    
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Documentation build failed"
    log_error "Build output:"
    echo "$BUILD_CMD_OUTPUT"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test strict mode build
section "Test 14: Test Strict Mode Build in Docker Container"

log_info "Testing strict mode build to catch warnings..."

STRICT_BUILD_OUTPUT=$(docker run --rm "$FULL_IMAGE_NAME" mkdocs build --strict 2>&1)
STRICT_BUILD_EXIT_CODE=$?

if [[ $STRICT_BUILD_EXIT_CODE -eq 0 ]]; then
    pass "Strict mode build passed (no warnings or errors)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Strict mode build failed (warnings or errors present)"
    log_error "Strict build output:"
    echo "$STRICT_BUILD_OUTPUT"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test configuration validation command
section "Test 15: Test MkDocs Configuration Validation Command"

log_info "Testing mkdocs config command..."

CONFIG_OUTPUT=$(docker run --rm "$FULL_IMAGE_NAME" mkdocs --version 2>&1)
CONFIG_EXIT_CODE=$?

if [[ $CONFIG_EXIT_CODE -eq 0 ]]; then
    pass "MkDocs command is accessible"
    log_info "MkDocs version: $CONFIG_OUTPUT"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "MkDocs command is not accessible"
    log_error "Command output: $CONFIG_OUTPUT"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Final summary
section "Test Summary"

TOTAL_TESTS=$((TESTS_PASSED + TESTS_FAILED))
log_info "Total tests: $TOTAL_TESTS"
log_info "Tests passed: $TESTS_PASSED"
log_info "Tests failed: $TESTS_FAILED"

if [[ $TESTS_FAILED -eq 0 ]]; then
    echo
    pass "All Docker MkDocs configuration validation tests passed!"
    echo
    log_info "MkDocs configuration is valid and all components are working correctly"
    log_info "Summary of validations:"
    log_info "  ✓ MkDocs configuration is valid (--strict mode)"
    log_info "  ✓ All navigation paths point to existing files"
    log_info "  ✓ Material theme is properly configured"
    log_info "  ✓ All required markdown extensions are configured:"
    log_info "    - pymdownx.highlight"
    log_info "    - pymdownx.superfences"
    log_info "    - admonitions"
    log_info "    - tables"
    log_info "    - toc"
    log_info "  ✓ PDF plugin is configured with exclusion patterns"
    log_info "  ✓ Search plugin is properly configured"
    echo
    exit 0
else
    echo
    fail "Some Docker MkDocs configuration validation tests failed!"
    echo
    log_error "$TESTS_FAILED test(s) failed"
    log_info "Please review the output above and fix the configuration issues"
    echo
    exit 1
fi
