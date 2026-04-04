#!/usr/bin/env bash
#
# validate_stage5_tpdg_result_docs.sh - Validates Stage 5 (TPDG result documentation generation)
#
# DESCRIPTION:
#   This script validates Stage 5 by invoking test-plan-documentation-generator (tpdg)
#   on container YAML files to generate result documentation in multiple formats. It performs:
#   - Checking TPDG binary availability ($TEST_PLAN_DOC_GEN or PATH)
#   - Invoking TPDG on each container YAML to generate AsciiDoc result documents
#   - Invoking TPDG on each container YAML to generate Markdown result documents
#   - Validating generated .adoc files exist in 30_documentation_source/ subdirectories
#   - Checking .md files exist for each test
#   - Verifying AsciiDoc files contain expected content markers:
#     * Test case ID
#     * test_date field
#     * Pass/fail status
#     * Sequence information
#     * Step results
#   - Validating Markdown files have proper structure:
#     * Headers (# and ##)
#     * Tables
#     * Status indicators
#   - Checking HTML generation from AsciiDoc using asciidoctor (if available)
#   - Confirming report file sizes are non-zero and reasonable (>= 100 bytes)
#   - Generating TPDG result documentation validation report with:
#     * File counts (AsciiDoc, Markdown, HTML)
#     * Content quality checks
#     * Summary statistics
#
# USAGE:
#   ./test-acceptance/validate_stage5_tpdg_result_docs.sh [OPTIONS]
#
# OPTIONS:
#   -v, --verbose         Enable verbose output
#   -h, --help            Show this help message
#   --skip-html           Skip HTML generation validation
#   --test-pattern PAT    Only test containers matching pattern (glob)
#   --min-file-size SIZE  Minimum expected file size in bytes (default: 100)
#
# EXIT CODES:
#   0 - All validations passed
#   1 - One or more validations failed
#
# OUTPUT:
#   Generates a detailed validation report with:
#   - TPDG binary detection and verification
#   - Report generation status for each container
#   - File existence checks
#   - Content validation results
#   - HTML generation status
#   - Summary statistics
#

set -euo pipefail

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source logger and report generator libraries
source "$REPO_ROOT/scripts/lib/logger.sh" || exit 1
source "$REPO_ROOT/scripts/lib/report_generator.sh" || exit 1

# Configuration
VERIFICATION_RESULTS_DIR="$SCRIPT_DIR/20_verification"
REPORTS_DIR="$SCRIPT_DIR/30_documentation_source"
RESULT_DOCS_DIR="$REPORTS_DIR/result_docs"
VERBOSE=0
SKIP_HTML=0
TEST_PATTERN="*"
MIN_FILE_SIZE=100

# Validation counters
TOTAL_CONTAINERS=0
TPDG_AVAILABLE=0
ASCIIDOC_GENERATED=0
ASCIIDOC_FAILED=0
MARKDOWN_GENERATED=0
MARKDOWN_FAILED=0
HTML_GENERATED=0
HTML_FAILED=0
CONTENT_VALIDATION_PASSED=0
CONTENT_VALIDATION_FAILED=0
FILE_SIZE_PASSED=0
FILE_SIZE_FAILED=0

# Error tracking (bash 3.2 compatible)
ERRORS=""

# Append error helper
append_error() {
    if [[ -z "$ERRORS" ]]; then
        ERRORS="$1"
    else
        ERRORS="$ERRORS"$'\n'"$1"
    fi
}

# Usage function
usage() {
    cat << EOF
Usage: $(basename "$0") [OPTIONS]

Validates Stage 5 (TPDG result documentation generation) for acceptance testing.

OPTIONS:
    -v, --verbose         Enable verbose output
    -h, --help            Show this help message
    --skip-html           Skip HTML generation validation
    --test-pattern PAT    Only test containers matching pattern (e.g., "*SUCCESS*")
    --min-file-size SIZE  Minimum expected file size in bytes (default: 100)

DESCRIPTION:
    Validates TPDG result documentation generation by:
    - Checking TPDG binary availability
    - Generating AsciiDoc result documents from container YAMLs
    - Generating Markdown result documents from container YAMLs
    - Validating .adoc file existence and content
    - Validating .md file existence and structure
    - Generating HTML from AsciiDoc (if asciidoctor available)
    - Verifying file sizes are non-zero and reasonable
    - Generating comprehensive validation report

EXIT CODES:
    0 - All validations passed
    1 - One or more validations failed

EOF
    exit 0
}

# Parse command-line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -v|--verbose)
                VERBOSE=1
                shift
                ;;
            -h|--help)
                usage
                ;;
            --skip-html)
                SKIP_HTML=1
                shift
                ;;
            --test-pattern)
                TEST_PATTERN="$2"
                shift 2
                ;;
            --min-file-size)
                MIN_FILE_SIZE="$2"
                shift 2
                ;;
            *)
                log_error "Unknown option: $1"
                usage
                ;;
        esac
    done
}

# Ensure directories exist
ensure_directories() {
    if [[ ! -d "$VERIFICATION_RESULTS_DIR" ]]; then
        log_error "Verification results directory not found: $VERIFICATION_RESULTS_DIR"
        log_error "Please run validate_stage4_verification.sh first"
        exit 1
    fi
    
    mkdir -p "$REPORTS_DIR"
    mkdir -p "$RESULT_DOCS_DIR"
    
    log_verbose "Using verification results directory: $VERIFICATION_RESULTS_DIR"
    log_verbose "Using result docs directory: $RESULT_DOCS_DIR"
}

# Check TPDG binary availability
check_tpdg_availability() {
    log_info "Checking test-plan-documentation-generator availability..."
    
    if check_test_plan_doc_gen_available; then
        TPDG_AVAILABLE=1
        local binary_path
        binary_path=$(find_test_plan_doc_gen)
        pass "test-plan-documentation-generator found: $binary_path"
        
        # Verify binary works
        if verify_test_plan_doc_gen_binary "$binary_path"; then
            pass "Binary verification successful"
        else
            log_error "Binary found but verification failed"
            append_error "TPDG binary verification failed"
            return 1
        fi
        
        return 0
    else
        fail "test-plan-documentation-generator not found"
        log_error "TPDG is not available in \$TEST_PLAN_DOC_GEN or PATH"
        log_error "Please install it with: cargo install test-plan-documentation-generator"
        append_error "TPDG binary not found"
        return 1
    fi
}

# Find all container YAML files
find_container_yamls() {
    find "$VERIFICATION_RESULTS_DIR" -type f -name "${TEST_PATTERN}_container.yaml" | sort
}

# Get test case ID from container filename
get_test_case_id() {
    local container_file="$1"
    local basename
    basename=$(basename "$container_file" _container.yaml)
    echo "$basename"
}

# Generate AsciiDoc documentation
generate_asciidoc() {
    local container_file="$1"
    local test_id="$2"
    local output_file="$RESULT_DOCS_DIR/${test_id}_result.adoc"
    
    log_verbose "  Generating AsciiDoc: $output_file"
    
    # Invoke TPDG to generate AsciiDoc
    if invoke_test_plan_doc_gen \
        --container "$container_file" \
        --output "$output_file" \
        --format asciidoc >/dev/null 2>&1; then
        
        ((ASCIIDOC_GENERATED++))
        log_debug "    ✓ AsciiDoc generated"
        echo "$output_file"
        return 0
    else
        ((ASCIIDOC_FAILED++))
        append_error "$test_id: Failed to generate AsciiDoc"
        log_debug "    ✗ AsciiDoc generation failed"
        return 1
    fi
}

# Generate Markdown documentation
generate_markdown() {
    local container_file="$1"
    local test_id="$2"
    local output_file="$RESULT_DOCS_DIR/${test_id}_result.md"
    
    log_verbose "  Generating Markdown: $output_file"
    
    # Invoke TPDG to generate Markdown
    if invoke_test_plan_doc_gen \
        --container "$container_file" \
        --output "$output_file" \
        --format markdown >/dev/null 2>&1; then
        
        ((MARKDOWN_GENERATED++))
        log_debug "    ✓ Markdown generated"
        echo "$output_file"
        return 0
    else
        ((MARKDOWN_FAILED++))
        append_error "$test_id: Failed to generate Markdown"
        log_debug "    ✗ Markdown generation failed"
        return 1
    fi
}

# Validate AsciiDoc content
validate_asciidoc_content() {
    local adoc_file="$1"
    local test_id="$2"
    
    log_verbose "  Validating AsciiDoc content..."
    
    # Check if file exists
    if [[ ! -f "$adoc_file" ]]; then
        log_debug "    ✗ AsciiDoc file not found"
        append_error "$test_id: AsciiDoc file not found"
        return 1
    fi
    
    local validation_errors=0
    
    # Check for test case ID
    if ! grep -q "$test_id" "$adoc_file" 2>/dev/null; then
        log_debug "    ✗ Test case ID not found in AsciiDoc"
        append_error "$test_id: AsciiDoc missing test case ID"
        ((validation_errors++))
    fi
    
    # Check for test_date field
    if ! grep -q "test_date\|Test Date\|Date:" "$adoc_file" 2>/dev/null; then
        log_debug "    ✗ test_date not found in AsciiDoc"
        append_error "$test_id: AsciiDoc missing test_date"
        ((validation_errors++))
    fi
    
    # Check for pass/fail status
    if ! grep -qi "pass\|fail\|status" "$adoc_file" 2>/dev/null; then
        log_debug "    ✗ Pass/fail status not found in AsciiDoc"
        append_error "$test_id: AsciiDoc missing pass/fail status"
        ((validation_errors++))
    fi
    
    # Check for sequence information
    if ! grep -qi "sequence\|seq" "$adoc_file" 2>/dev/null; then
        log_debug "    ✗ Sequence information not found in AsciiDoc"
        append_error "$test_id: AsciiDoc missing sequence information"
        ((validation_errors++))
    fi
    
    # Check for step results
    if ! grep -qi "step\|result" "$adoc_file" 2>/dev/null; then
        log_debug "    ✗ Step results not found in AsciiDoc"
        append_error "$test_id: AsciiDoc missing step results"
        ((validation_errors++))
    fi
    
    if [[ $validation_errors -eq 0 ]]; then
        log_debug "    ✓ AsciiDoc content validation passed"
        return 0
    else
        log_debug "    ✗ AsciiDoc content validation failed ($validation_errors errors)"
        return 1
    fi
}

# Validate Markdown structure
validate_markdown_structure() {
    local md_file="$1"
    local test_id="$2"
    
    log_verbose "  Validating Markdown structure..."
    
    # Check if file exists
    if [[ ! -f "$md_file" ]]; then
        log_debug "    ✗ Markdown file not found"
        append_error "$test_id: Markdown file not found"
        return 1
    fi
    
    local validation_errors=0
    
    # Check for headers (# and ##)
    if ! grep -q "^#" "$md_file" 2>/dev/null; then
        log_debug "    ✗ No headers found in Markdown"
        append_error "$test_id: Markdown missing headers"
        ((validation_errors++))
    fi
    
    # Check for tables (| symbols)
    if ! grep -q "|" "$md_file" 2>/dev/null; then
        log_debug "    ✗ No tables found in Markdown"
        append_error "$test_id: Markdown missing tables"
        ((validation_errors++))
    fi
    
    # Check for status indicators
    if ! grep -qi "pass\|fail\|success\|✓\|✗" "$md_file" 2>/dev/null; then
        log_debug "    ✗ No status indicators found in Markdown"
        append_error "$test_id: Markdown missing status indicators"
        ((validation_errors++))
    fi
    
    # Check for test case ID
    if ! grep -q "$test_id" "$md_file" 2>/dev/null; then
        log_debug "    ✗ Test case ID not found in Markdown"
        append_error "$test_id: Markdown missing test case ID"
        ((validation_errors++))
    fi
    
    if [[ $validation_errors -eq 0 ]]; then
        log_debug "    ✓ Markdown structure validation passed"
        return 0
    else
        log_debug "    ✗ Markdown structure validation failed ($validation_errors errors)"
        return 1
    fi
}

# Validate file size
validate_file_size() {
    local file_path="$1"
    local test_id="$2"
    local file_type="$3"
    
    if [[ ! -f "$file_path" ]]; then
        log_debug "    ✗ $file_type file not found for size check"
        return 1
    fi
    
    # Get file size (portable across BSD and GNU)
    local file_size
    file_size=$(stat -f%z "$file_path" 2>/dev/null || stat -c%s "$file_path" 2>/dev/null || echo "0")
    
    if [[ "$file_size" -eq 0 ]]; then
        log_debug "    ✗ $file_type file is empty"
        append_error "$test_id: $file_type file is empty (0 bytes)"
        return 1
    elif [[ "$file_size" -lt "$MIN_FILE_SIZE" ]]; then
        log_debug "    ✗ $file_type file is too small ($file_size bytes, minimum: $MIN_FILE_SIZE bytes)"
        append_error "$test_id: $file_type file is too small ($file_size bytes)"
        return 1
    else
        log_debug "    ✓ $file_type file size OK ($file_size bytes)"
        return 0
    fi
}

# Generate HTML from AsciiDoc
generate_html_from_asciidoc() {
    local adoc_file="$1"
    local test_id="$2"
    local html_file="${adoc_file%.adoc}.html"
    
    # Check if asciidoctor is available
    if ! command -v asciidoctor >/dev/null 2>&1; then
        log_debug "  asciidoctor not available, skipping HTML generation"
        return 2  # Special return code for "skipped"
    fi
    
    log_verbose "  Generating HTML: $html_file"
    
    # Generate HTML using asciidoctor
    if asciidoctor "$adoc_file" -o "$html_file" >/dev/null 2>&1; then
        ((HTML_GENERATED++))
        log_debug "    ✓ HTML generated"
        
        # Validate HTML file
        if validate_file_size "$html_file" "$test_id" "HTML"; then
            log_debug "    ✓ HTML file size validation passed"
        else
            log_debug "    ✗ HTML file size validation failed"
        fi
        
        return 0
    else
        ((HTML_FAILED++))
        append_error "$test_id: Failed to generate HTML from AsciiDoc"
        log_debug "    ✗ HTML generation failed"
        return 1
    fi
}

# Validate a single container
validate_container() {
    local container_file="$1"
    local test_id
    test_id=$(get_test_case_id "$container_file")
    local container_basename
    container_basename=$(basename "$container_file")
    
    log_verbose "Validating container: $container_basename"
    
    # Generate AsciiDoc
    local adoc_file
    adoc_file=$(generate_asciidoc "$container_file" "$test_id")
    local adoc_result=$?
    
    # Generate Markdown
    local md_file
    md_file=$(generate_markdown "$container_file" "$test_id")
    local md_result=$?
    
    # Validate AsciiDoc content if generated
    local adoc_content_ok=0
    if [[ $adoc_result -eq 0 ]] && [[ -n "$adoc_file" ]]; then
        if validate_asciidoc_content "$adoc_file" "$test_id"; then
            adoc_content_ok=1
            ((CONTENT_VALIDATION_PASSED++))
        else
            ((CONTENT_VALIDATION_FAILED++))
        fi
        
        # Validate AsciiDoc file size
        if validate_file_size "$adoc_file" "$test_id" "AsciiDoc"; then
            ((FILE_SIZE_PASSED++))
        else
            ((FILE_SIZE_FAILED++))
        fi
        
        # Generate HTML from AsciiDoc (if not skipped)
        if [[ $SKIP_HTML -eq 0 ]]; then
            generate_html_from_asciidoc "$adoc_file" "$test_id"
        fi
    fi
    
    # Validate Markdown structure if generated
    local md_structure_ok=0
    if [[ $md_result -eq 0 ]] && [[ -n "$md_file" ]]; then
        if validate_markdown_structure "$md_file" "$test_id"; then
            md_structure_ok=1
            ((CONTENT_VALIDATION_PASSED++))
        else
            ((CONTENT_VALIDATION_FAILED++))
        fi
        
        # Validate Markdown file size
        if validate_file_size "$md_file" "$test_id" "Markdown"; then
            ((FILE_SIZE_PASSED++))
        else
            ((FILE_SIZE_FAILED++))
        fi
    fi
    
    # Overall validation status
    if [[ $adoc_result -eq 0 ]] && [[ $md_result -eq 0 ]] && [[ $adoc_content_ok -eq 1 ]] && [[ $md_structure_ok -eq 1 ]]; then
        log_debug "  ✓ Container validation passed: $test_id"
    else
        log_debug "  ✗ Container validation had issues: $test_id"
    fi
}

# Generate validation report
generate_report() {
    local report_file="$REPORTS_DIR/stage5_tpdg_result_docs_validation_report.txt"
    
    {
        echo "========================================="
        echo "Stage 5 TPDG Result Documentation Validation Report"
        echo "========================================="
        echo ""
        echo "Generated: $(date)"
        echo ""
        
        echo "Summary"
        echo "-------"
        echo "Total containers validated: $TOTAL_CONTAINERS"
        echo ""
        
        echo "TPDG Availability:"
        if [[ $TPDG_AVAILABLE -eq 1 ]]; then
            echo "  Status: Available"
            local binary_path
            binary_path=$(find_test_plan_doc_gen)
            echo "  Path: $binary_path"
        else
            echo "  Status: Not Available"
        fi
        echo ""
        
        echo "AsciiDoc Generation:"
        echo "  Generated: $ASCIIDOC_GENERATED"
        echo "  Failed: $ASCIIDOC_FAILED"
        echo ""
        
        echo "Markdown Generation:"
        echo "  Generated: $MARKDOWN_GENERATED"
        echo "  Failed: $MARKDOWN_FAILED"
        echo ""
        
        if [[ $SKIP_HTML -eq 0 ]]; then
            echo "HTML Generation:"
            echo "  Generated: $HTML_GENERATED"
            echo "  Failed: $HTML_FAILED"
            if ! command -v asciidoctor >/dev/null 2>&1; then
                echo "  Note: asciidoctor not available"
            fi
            echo ""
        fi
        
        echo "Content Validation:"
        echo "  Passed: $CONTENT_VALIDATION_PASSED"
        echo "  Failed: $CONTENT_VALIDATION_FAILED"
        echo ""
        
        echo "File Size Validation:"
        echo "  Passed: $FILE_SIZE_PASSED"
        echo "  Failed: $FILE_SIZE_FAILED"
        echo "  Minimum size: $MIN_FILE_SIZE bytes"
        echo ""
        
        # File counts
        echo "Generated Files:"
        local adoc_count
        adoc_count=$(find "$RESULT_DOCS_DIR" -type f -name "*.adoc" 2>/dev/null | wc -l | tr -d ' ')
        echo "  AsciiDoc files: $adoc_count"
        
        local md_count
        md_count=$(find "$RESULT_DOCS_DIR" -type f -name "*.md" 2>/dev/null | wc -l | tr -d ' ')
        echo "  Markdown files: $md_count"
        
        if [[ $SKIP_HTML -eq 0 ]]; then
            local html_count
            html_count=$(find "$RESULT_DOCS_DIR" -type f -name "*.html" 2>/dev/null | wc -l | tr -d ' ')
            echo "  HTML files: $html_count"
        fi
        echo ""
        
        # Errors section
        if [[ -n "$ERRORS" ]]; then
            echo "Validation Errors"
            echo "-----------------"
            
            # Print errors line by line (bash 3.2 compatible)
            local IFS=$'\n'
            local error_line
            for error_line in $ERRORS; do
                echo "  $error_line"
            done
            echo ""
        fi
        
        # Overall result
        local total_failed=$((ASCIIDOC_FAILED + MARKDOWN_FAILED + HTML_FAILED + CONTENT_VALIDATION_FAILED + FILE_SIZE_FAILED))
        
        echo "Overall Result"
        echo "--------------"
        if [[ $TPDG_AVAILABLE -eq 0 ]]; then
            echo "✗ TPDG not available - cannot validate"
        elif [[ $total_failed -eq 0 ]]; then
            echo "✓ All validations passed!"
        else
            echo "✗ Total failures: $total_failed"
        fi
        echo ""
        
        # Quality metrics
        echo "Quality Metrics"
        echo "---------------"
        if [[ $TOTAL_CONTAINERS -gt 0 ]]; then
            local adoc_success_rate=$((ASCIIDOC_GENERATED * 100 / TOTAL_CONTAINERS))
            local md_success_rate=$((MARKDOWN_GENERATED * 100 / TOTAL_CONTAINERS))
            echo "  AsciiDoc generation rate: $adoc_success_rate%"
            echo "  Markdown generation rate: $md_success_rate%"
            
            if [[ $((ASCIIDOC_GENERATED + MARKDOWN_GENERATED)) -gt 0 ]]; then
                local content_success_rate=$((CONTENT_VALIDATION_PASSED * 100 / (ASCIIDOC_GENERATED + MARKDOWN_GENERATED)))
                echo "  Content validation rate: $content_success_rate%"
            fi
        else
            echo "  No containers processed"
        fi
        echo ""
        
    } > "$report_file"
    
    log_info "Report saved to: $report_file"
}

# Print validation report
print_report() {
    section "Stage 5 TPDG Result Documentation Validation Report"
    
    echo ""
    log_info "Total containers validated: $TOTAL_CONTAINERS"
    echo ""
    
    log_info "TPDG Availability:"
    if [[ $TPDG_AVAILABLE -eq 1 ]]; then
        pass "  Available"
        local binary_path
        binary_path=$(find_test_plan_doc_gen)
        log_info "  Path: $binary_path"
    else
        fail "  Not Available"
    fi
    echo ""
    
    log_info "AsciiDoc Generation:"
    log_info "  Generated: $ASCIIDOC_GENERATED"
    log_info "  Failed: $ASCIIDOC_FAILED"
    echo ""
    
    log_info "Markdown Generation:"
    log_info "  Generated: $MARKDOWN_GENERATED"
    log_info "  Failed: $MARKDOWN_FAILED"
    echo ""
    
    if [[ $SKIP_HTML -eq 0 ]]; then
        log_info "HTML Generation:"
        log_info "  Generated: $HTML_GENERATED"
        log_info "  Failed: $HTML_FAILED"
        if ! command -v asciidoctor >/dev/null 2>&1; then
            log_info "  Note: asciidoctor not available"
        fi
        echo ""
    fi
    
    log_info "Content Validation:"
    log_info "  Passed: $CONTENT_VALIDATION_PASSED"
    log_info "  Failed: $CONTENT_VALIDATION_FAILED"
    echo ""
    
    log_info "File Size Validation:"
    log_info "  Passed: $FILE_SIZE_PASSED"
    log_info "  Failed: $FILE_SIZE_FAILED"
    echo ""
    
    if [[ -n "$ERRORS" ]]; then
        section "Validation Errors"
        echo ""
        # Print errors line by line (bash 3.2 compatible)
        local IFS=$'\n'
        local error_line
        for error_line in $ERRORS; do
            fail "$error_line"
        done
        echo ""
    fi
    
    local total_failed=$((ASCIIDOC_FAILED + MARKDOWN_FAILED + HTML_FAILED + CONTENT_VALIDATION_FAILED + FILE_SIZE_FAILED))
    
    if [[ $TPDG_AVAILABLE -eq 0 ]]; then
        section "Validation Result"
        fail "TPDG not available - cannot validate"
        return 1
    elif [[ $total_failed -eq 0 ]]; then
        section "Validation Result"
        pass "All validations passed!"
        return 0
    else
        section "Validation Result"
        fail "Total failures: $total_failed"
        return 1
    fi
}

# Main function
main() {
    parse_args "$@"
    
    section "Stage 5 TPDG Result Documentation Validation"
    
    log_info "Validating TPDG result documentation generation..."
    echo ""
    
    # Ensure directories exist
    ensure_directories
    
    # Check TPDG availability
    if ! check_tpdg_availability; then
        log_error "TPDG is not available, cannot proceed with validation"
        
        # Generate report even if TPDG is not available
        generate_report
        print_report
        exit 1
    fi
    
    echo ""
    
    # Find all container YAML files (bash 3.2 compatible)
    local container_files=()
    while IFS= read -r file; do
        container_files+=("$file")
    done < <(find_container_yamls)
    
    if [[ ${#container_files[@]} -eq 0 ]]; then
        log_warning "No container YAML files found in $VERIFICATION_RESULTS_DIR matching pattern: ${TEST_PATTERN}_container.yaml"
        
        # Generate report
        generate_report
        print_report
        exit 0
    fi
    
    TOTAL_CONTAINERS=${#container_files[@]}
    log_info "Found $TOTAL_CONTAINERS container(s) to process"
    echo ""
    
    # Validate each container
    for container_file in "${container_files[@]}"; do
        validate_container "$container_file"
    done
    
    echo ""
    
    # Generate report file
    generate_report
    
    # Print report and exit with appropriate code
    if print_report; then
        exit 0
    else
        exit 1
    fi
}

# Run main function
main "$@"
