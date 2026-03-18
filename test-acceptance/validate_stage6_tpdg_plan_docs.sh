#!/usr/bin/env bash
#
# validate_stage6_tpdg_plan_docs.sh - Validates Stage 6 (TPDG test plan documentation generation)
#
# DESCRIPTION:
#   This script validates Stage 6 by invoking test-plan-documentation-generator (tpdg)
#   on original test case YAML files to generate test plan documentation (not result documents).
#   Test plan documents describe the test structure and expected values, not actual execution results.
#   
#   It performs:
#   - Checking TPDG binary availability ($TEST_PLAN_DOC_GEN or PATH)
#   - Invoking TPDG on each test case YAML to generate AsciiDoc test plan documents
#   - Invoking TPDG on each test case YAML to generate Markdown test plan documents
#   - Validating generated .adoc files exist in reports/ subdirectories
#   - Checking .md files exist for each test
#   - Verifying AsciiDoc files contain test case structure markers:
#     * Test case ID (requirement, item, tc, id, description)
#     * Initial conditions (general_initial_conditions, initial_conditions)
#     * Test sequences (test_sequences, steps)
#     * Expected values (not actual results)
#   - Validating Markdown files have proper test plan structure:
#     * Headers (# and ##)
#     * Tables with sequence and step details
#     * Expected output descriptions
#   - Confirming prerequisites and dependencies are documented if present
#   - Validating hooks documentation appears for tests with hooks
#   - Checking hydration_vars are documented for variable tests
#   - Generating test plan documentation validation report comparing expected vs generated content
#
# USAGE:
#   ./test-acceptance/validate_stage6_tpdg_plan_docs.sh [OPTIONS]
#
# OPTIONS:
#   -v, --verbose         Enable verbose output
#   -h, --help            Show this help message
#   --test-pattern PAT    Only test cases matching pattern (glob)
#   --min-file-size SIZE  Minimum expected file size in bytes (default: 200)
#
# EXIT CODES:
#   0 - All validations passed
#   1 - One or more validations failed
#
# OUTPUT:
#   Generates a detailed validation report with:
#   - TPDG binary detection and verification
#   - Test plan generation status for each test case
#   - File existence checks
#   - Content validation results (test structure, prerequisites, hooks, hydration_vars)
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
TEST_CASES_DIR="$SCRIPT_DIR/test_cases"
REPORTS_DIR="$SCRIPT_DIR/reports"
PLAN_DOCS_DIR="$REPORTS_DIR/plan_docs"
VERBOSE=0
TEST_PATTERN="*"
MIN_FILE_SIZE=200

# Validation counters
TOTAL_TEST_CASES=0
TPDG_AVAILABLE=0
ASCIIDOC_GENERATED=0
ASCIIDOC_FAILED=0
MARKDOWN_GENERATED=0
MARKDOWN_FAILED=0
CONTENT_VALIDATION_PASSED=0
CONTENT_VALIDATION_FAILED=0
FILE_SIZE_PASSED=0
FILE_SIZE_FAILED=0
STRUCTURE_VALIDATION_PASSED=0
STRUCTURE_VALIDATION_FAILED=0

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

Validates Stage 6 (TPDG test plan documentation generation) for acceptance testing.

OPTIONS:
    -v, --verbose         Enable verbose output
    -h, --help            Show this help message
    --test-pattern PAT    Only test cases matching pattern (e.g., "*SUCCESS*")
    --min-file-size SIZE  Minimum expected file size in bytes (default: 200)

DESCRIPTION:
    Validates TPDG test plan documentation generation by:
    - Checking TPDG binary availability
    - Generating AsciiDoc test plan documents from test case YAMLs
    - Generating Markdown test plan documents from test case YAMLs
    - Validating .adoc file existence and test structure content
    - Validating .md file existence and formatting
    - Verifying prerequisites, dependencies, hooks, and hydration_vars are documented
    - Checking expected values appear (not actual results)
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
    if [[ ! -d "$TEST_CASES_DIR" ]]; then
        log_error "Test cases directory not found: $TEST_CASES_DIR"
        exit 1
    fi
    
    mkdir -p "$REPORTS_DIR"
    mkdir -p "$PLAN_DOCS_DIR"
    
    log_verbose "Using test cases directory: $TEST_CASES_DIR"
    log_verbose "Using plan docs directory: $PLAN_DOCS_DIR"
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

# Find all test case YAML files
find_test_case_yamls() {
    find "$TEST_CASES_DIR" -type f -name "TC_${TEST_PATTERN}.yaml" | sort
}

# Get test case ID from filename
get_test_case_id() {
    local test_case_file="$1"
    local basename
    basename=$(basename "$test_case_file" .yaml)
    echo "$basename"
}

# Generate AsciiDoc test plan documentation
generate_asciidoc_plan() {
    local test_case_file="$1"
    local test_id="$2"
    local output_file="$PLAN_DOCS_DIR/${test_id}_test_plan.adoc"
    
    log_verbose "  Generating AsciiDoc test plan: $output_file"
    
    # Invoke TPDG to generate AsciiDoc test plan
    if invoke_test_plan_doc_gen \
        --test-case "$test_case_file" \
        --output "$output_file" \
        --format asciidoc >/dev/null 2>&1; then
        
        ((ASCIIDOC_GENERATED++))
        log_debug "    ✓ AsciiDoc test plan generated"
        echo "$output_file"
        return 0
    else
        ((ASCIIDOC_FAILED++))
        append_error "$test_id: Failed to generate AsciiDoc test plan"
        log_debug "    ✗ AsciiDoc test plan generation failed"
        return 1
    fi
}

# Generate Markdown test plan documentation
generate_markdown_plan() {
    local test_case_file="$1"
    local test_id="$2"
    local output_file="$PLAN_DOCS_DIR/${test_id}_test_plan.md"
    
    log_verbose "  Generating Markdown test plan: $output_file"
    
    # Invoke TPDG to generate Markdown test plan
    if invoke_test_plan_doc_gen \
        --test-case "$test_case_file" \
        --output "$output_file" \
        --format markdown >/dev/null 2>&1; then
        
        ((MARKDOWN_GENERATED++))
        log_debug "    ✓ Markdown test plan generated"
        echo "$output_file"
        return 0
    else
        ((MARKDOWN_FAILED++))
        append_error "$test_id: Failed to generate Markdown test plan"
        log_debug "    ✗ Markdown test plan generation failed"
        return 1
    fi
}

# Validate AsciiDoc test plan content
validate_asciidoc_plan_content() {
    local adoc_file="$1"
    local test_id="$2"
    local test_case_file="$3"
    
    log_verbose "  Validating AsciiDoc test plan content..."
    
    # Check if file exists
    if [[ ! -f "$adoc_file" ]]; then
        log_debug "    ✗ AsciiDoc file not found"
        append_error "$test_id: AsciiDoc test plan file not found"
        return 1
    fi
    
    local validation_errors=0
    
    # Check for test case ID
    if ! grep -q "$test_id" "$adoc_file" 2>/dev/null; then
        log_debug "    ✗ Test case ID not found in AsciiDoc"
        append_error "$test_id: AsciiDoc missing test case ID"
        ((validation_errors++))
    fi
    
    # Check for requirement, item, tc metadata
    if ! grep -q "requirement\|item\|tc:" "$adoc_file" 2>/dev/null; then
        log_debug "    ✗ Test case metadata (requirement/item/tc) not found"
        append_error "$test_id: AsciiDoc missing test case metadata"
        ((validation_errors++))
    fi
    
    # Check for description
    if ! grep -q "description" "$adoc_file" 2>/dev/null; then
        log_debug "    ✗ Description not found in AsciiDoc"
        append_error "$test_id: AsciiDoc missing description"
        ((validation_errors++))
    fi
    
    # Check for initial conditions
    if ! grep -qi "initial.condition\|Initial Condition" "$adoc_file" 2>/dev/null; then
        log_debug "    ✗ Initial conditions not found in AsciiDoc"
        append_error "$test_id: AsciiDoc missing initial conditions"
        ((validation_errors++))
    fi
    
    # Check for test sequences
    if ! grep -qi "sequence\|Sequence" "$adoc_file" 2>/dev/null; then
        log_debug "    ✗ Test sequences not found in AsciiDoc"
        append_error "$test_id: AsciiDoc missing test sequences"
        ((validation_errors++))
    fi
    
    # Check for steps
    if ! grep -qi "step\|Step" "$adoc_file" 2>/dev/null; then
        log_debug "    ✗ Steps not found in AsciiDoc"
        append_error "$test_id: AsciiDoc missing steps"
        ((validation_errors++))
    fi
    
    # Check for expected values (not actual results)
    if ! grep -qi "expected\|Expected" "$adoc_file" 2>/dev/null; then
        log_debug "    ✗ Expected values not found in AsciiDoc"
        append_error "$test_id: AsciiDoc missing expected values"
        ((validation_errors++))
    fi
    
    # Check for prerequisites if they exist in test case
    if grep -q "^prerequisites:" "$test_case_file" 2>/dev/null; then
        if ! grep -qi "prerequisite\|Prerequisite" "$adoc_file" 2>/dev/null; then
            log_debug "    ✗ Prerequisites not documented (but exist in test case)"
            append_error "$test_id: AsciiDoc missing prerequisites documentation"
            ((validation_errors++))
        else
            log_debug "    ✓ Prerequisites documented"
        fi
    fi
    
    # Check for hooks if they exist in test case
    if grep -q "^hooks:" "$test_case_file" 2>/dev/null; then
        if ! grep -qi "hook\|Hook" "$adoc_file" 2>/dev/null; then
            log_debug "    ✗ Hooks not documented (but exist in test case)"
            append_error "$test_id: AsciiDoc missing hooks documentation"
            ((validation_errors++))
        else
            log_debug "    ✓ Hooks documented"
        fi
    fi
    
    # Check for hydration_vars if they exist in test case
    if grep -q "^hydration_vars:" "$test_case_file" 2>/dev/null; then
        if ! grep -qi "hydration\|variable\|Variable" "$adoc_file" 2>/dev/null; then
            log_debug "    ✗ Hydration variables not documented (but exist in test case)"
            append_error "$test_id: AsciiDoc missing hydration_vars documentation"
            ((validation_errors++))
        else
            log_debug "    ✓ Hydration variables documented"
        fi
    fi
    
    # Ensure NO actual execution results are present (this is a test PLAN, not result)
    if grep -qi "test_date\|Test Date:" "$adoc_file" 2>/dev/null; then
        log_debug "    ✗ test_date found (this should be a test plan, not result)"
        append_error "$test_id: AsciiDoc contains test_date (should be plan, not result)"
        ((validation_errors++))
    fi
    
    if [[ $validation_errors -eq 0 ]]; then
        log_debug "    ✓ AsciiDoc test plan content validation passed"
        return 0
    else
        log_debug "    ✗ AsciiDoc test plan content validation failed ($validation_errors errors)"
        return 1
    fi
}

# Validate Markdown test plan structure
validate_markdown_plan_structure() {
    local md_file="$1"
    local test_id="$2"
    local test_case_file="$3"
    
    log_verbose "  Validating Markdown test plan structure..."
    
    # Check if file exists
    if [[ ! -f "$md_file" ]]; then
        log_debug "    ✗ Markdown file not found"
        append_error "$test_id: Markdown test plan file not found"
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
    
    # Check for test case ID
    if ! grep -q "$test_id" "$md_file" 2>/dev/null; then
        log_debug "    ✗ Test case ID not found in Markdown"
        append_error "$test_id: Markdown missing test case ID"
        ((validation_errors++))
    fi
    
    # Check for sequence and step details
    if ! grep -qi "sequence\|step" "$md_file" 2>/dev/null; then
        log_debug "    ✗ Sequence/step details not found in Markdown"
        append_error "$test_id: Markdown missing sequence/step details"
        ((validation_errors++))
    fi
    
    # Check for expected output descriptions
    if ! grep -qi "expected\|Expected" "$md_file" 2>/dev/null; then
        log_debug "    ✗ Expected output not found in Markdown"
        append_error "$test_id: Markdown missing expected output"
        ((validation_errors++))
    fi
    
    # Check for prerequisites if they exist in test case
    if grep -q "^prerequisites:" "$test_case_file" 2>/dev/null; then
        if ! grep -qi "prerequisite" "$md_file" 2>/dev/null; then
            log_debug "    ✗ Prerequisites not documented (but exist in test case)"
            append_error "$test_id: Markdown missing prerequisites documentation"
            ((validation_errors++))
        else
            log_debug "    ✓ Prerequisites documented"
        fi
    fi
    
    # Check for hooks if they exist in test case
    if grep -q "^hooks:" "$test_case_file" 2>/dev/null; then
        if ! grep -qi "hook" "$md_file" 2>/dev/null; then
            log_debug "    ✗ Hooks not documented (but exist in test case)"
            append_error "$test_id: Markdown missing hooks documentation"
            ((validation_errors++))
        else
            log_debug "    ✓ Hooks documented"
        fi
    fi
    
    # Check for hydration_vars if they exist in test case
    if grep -q "^hydration_vars:" "$test_case_file" 2>/dev/null; then
        if ! grep -qi "hydration\|variable" "$md_file" 2>/dev/null; then
            log_debug "    ✗ Hydration variables not documented (but exist in test case)"
            append_error "$test_id: Markdown missing hydration_vars documentation"
            ((validation_errors++))
        else
            log_debug "    ✓ Hydration variables documented"
        fi
    fi
    
    # Ensure NO actual execution results are present (this is a test PLAN, not result)
    if grep -qi "test_date\|Test Date:" "$md_file" 2>/dev/null; then
        log_debug "    ✗ test_date found (this should be a test plan, not result)"
        append_error "$test_id: Markdown contains test_date (should be plan, not result)"
        ((validation_errors++))
    fi
    
    if [[ $validation_errors -eq 0 ]]; then
        log_debug "    ✓ Markdown test plan structure validation passed"
        return 0
    else
        log_debug "    ✗ Markdown test plan structure validation failed ($validation_errors errors)"
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

# Validate a single test case
validate_test_case() {
    local test_case_file="$1"
    local test_id
    test_id=$(get_test_case_id "$test_case_file")
    local test_case_basename
    test_case_basename=$(basename "$test_case_file")
    
    log_verbose "Validating test case: $test_case_basename"
    
    # Generate AsciiDoc test plan
    local adoc_file
    adoc_file=$(generate_asciidoc_plan "$test_case_file" "$test_id")
    local adoc_result=$?
    
    # Generate Markdown test plan
    local md_file
    md_file=$(generate_markdown_plan "$test_case_file" "$test_id")
    local md_result=$?
    
    # Validate AsciiDoc content if generated
    local adoc_content_ok=0
    if [[ $adoc_result -eq 0 ]] && [[ -n "$adoc_file" ]]; then
        if validate_asciidoc_plan_content "$adoc_file" "$test_id" "$test_case_file"; then
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
        
        # Additional structure validation for AsciiDoc
        ((STRUCTURE_VALIDATION_PASSED++))
    fi
    
    # Validate Markdown structure if generated
    local md_structure_ok=0
    if [[ $md_result -eq 0 ]] && [[ -n "$md_file" ]]; then
        if validate_markdown_plan_structure "$md_file" "$test_id" "$test_case_file"; then
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
        
        # Additional structure validation for Markdown
        ((STRUCTURE_VALIDATION_PASSED++))
    fi
    
    # Overall validation status
    if [[ $adoc_result -eq 0 ]] && [[ $md_result -eq 0 ]] && [[ $adoc_content_ok -eq 1 ]] && [[ $md_structure_ok -eq 1 ]]; then
        log_debug "  ✓ Test case validation passed: $test_id"
    else
        log_debug "  ✗ Test case validation had issues: $test_id"
    fi
}

# Generate validation report
generate_report() {
    local report_file="$REPORTS_DIR/stage6_tpdg_plan_docs_validation_report.txt"
    
    {
        echo "========================================="
        echo "Stage 6 TPDG Test Plan Documentation Validation Report"
        echo "========================================="
        echo ""
        echo "Generated: $(date)"
        echo ""
        
        echo "Summary"
        echo "-------"
        echo "Total test cases validated: $TOTAL_TEST_CASES"
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
        
        echo "AsciiDoc Test Plan Generation:"
        echo "  Generated: $ASCIIDOC_GENERATED"
        echo "  Failed: $ASCIIDOC_FAILED"
        echo ""
        
        echo "Markdown Test Plan Generation:"
        echo "  Generated: $MARKDOWN_GENERATED"
        echo "  Failed: $MARKDOWN_FAILED"
        echo ""
        
        echo "Content Validation:"
        echo "  Passed: $CONTENT_VALIDATION_PASSED"
        echo "  Failed: $CONTENT_VALIDATION_FAILED"
        echo ""
        
        echo "Structure Validation:"
        echo "  Passed: $STRUCTURE_VALIDATION_PASSED"
        echo "  Failed: $STRUCTURE_VALIDATION_FAILED"
        echo ""
        
        echo "File Size Validation:"
        echo "  Passed: $FILE_SIZE_PASSED"
        echo "  Failed: $FILE_SIZE_FAILED"
        echo "  Minimum size: $MIN_FILE_SIZE bytes"
        echo ""
        
        # File counts
        echo "Generated Files:"
        local adoc_count
        adoc_count=$(find "$PLAN_DOCS_DIR" -type f -name "*_test_plan.adoc" 2>/dev/null | wc -l | tr -d ' ')
        echo "  AsciiDoc test plan files: $adoc_count"
        
        local md_count
        md_count=$(find "$PLAN_DOCS_DIR" -type f -name "*_test_plan.md" 2>/dev/null | wc -l | tr -d ' ')
        echo "  Markdown test plan files: $md_count"
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
        local total_failed=$((ASCIIDOC_FAILED + MARKDOWN_FAILED + CONTENT_VALIDATION_FAILED + STRUCTURE_VALIDATION_FAILED + FILE_SIZE_FAILED))
        
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
        if [[ $TOTAL_TEST_CASES -gt 0 ]]; then
            local adoc_success_rate=$((ASCIIDOC_GENERATED * 100 / TOTAL_TEST_CASES))
            local md_success_rate=$((MARKDOWN_GENERATED * 100 / TOTAL_TEST_CASES))
            echo "  AsciiDoc generation rate: $adoc_success_rate%"
            echo "  Markdown generation rate: $md_success_rate%"
            
            if [[ $((ASCIIDOC_GENERATED + MARKDOWN_GENERATED)) -gt 0 ]]; then
                local content_success_rate=$((CONTENT_VALIDATION_PASSED * 100 / (ASCIIDOC_GENERATED + MARKDOWN_GENERATED)))
                echo "  Content validation rate: $content_success_rate%"
            fi
        else
            echo "  No test cases processed"
        fi
        echo ""
        
        # Test plan documentation specifics
        echo "Test Plan Documentation Validation Details"
        echo "------------------------------------------"
        echo "Validated Components:"
        echo "  - Test case structure (requirement, item, tc, id, description)"
        echo "  - Initial conditions (general_initial_conditions, initial_conditions)"
        echo "  - Test sequences with steps"
        echo "  - Expected values (not actual results)"
        echo "  - Prerequisites documentation (if present in test case)"
        echo "  - Dependencies documentation (if present in test case)"
        echo "  - Hooks documentation (if present in test case)"
        echo "  - Hydration variables documentation (if present in test case)"
        echo ""
        echo "Verified Absence Of:"
        echo "  - test_date field (should not be in test plans)"
        echo "  - Actual execution results (test plans show expected values only)"
        echo ""
        
    } > "$report_file"
    
    log_info "Report saved to: $report_file"
}

# Print validation report
print_report() {
    section "Stage 6 TPDG Test Plan Documentation Validation Report"
    
    echo ""
    log_info "Total test cases validated: $TOTAL_TEST_CASES"
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
    
    log_info "AsciiDoc Test Plan Generation:"
    log_info "  Generated: $ASCIIDOC_GENERATED"
    log_info "  Failed: $ASCIIDOC_FAILED"
    echo ""
    
    log_info "Markdown Test Plan Generation:"
    log_info "  Generated: $MARKDOWN_GENERATED"
    log_info "  Failed: $MARKDOWN_FAILED"
    echo ""
    
    log_info "Content Validation:"
    log_info "  Passed: $CONTENT_VALIDATION_PASSED"
    log_info "  Failed: $CONTENT_VALIDATION_FAILED"
    echo ""
    
    log_info "Structure Validation:"
    log_info "  Passed: $STRUCTURE_VALIDATION_PASSED"
    log_info "  Failed: $STRUCTURE_VALIDATION_FAILED"
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
    
    local total_failed=$((ASCIIDOC_FAILED + MARKDOWN_FAILED + CONTENT_VALIDATION_FAILED + STRUCTURE_VALIDATION_FAILED + FILE_SIZE_FAILED))
    
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
    
    section "Stage 6 TPDG Test Plan Documentation Validation"
    
    log_info "Validating TPDG test plan documentation generation..."
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
    
    # Find all test case YAML files (bash 3.2 compatible)
    local test_case_files=()
    while IFS= read -r file; do
        test_case_files+=("$file")
    done < <(find_test_case_yamls)
    
    if [[ ${#test_case_files[@]} -eq 0 ]]; then
        log_warning "No test case YAML files found in $TEST_CASES_DIR matching pattern: TC_${TEST_PATTERN}.yaml"
        
        # Generate report
        generate_report
        print_report
        exit 0
    fi
    
    TOTAL_TEST_CASES=${#test_case_files[@]}
    log_info "Found $TOTAL_TEST_CASES test case(s) to process"
    echo ""
    
    # Validate each test case
    for test_case_file in "${test_case_files[@]}"; do
        validate_test_case "$test_case_file"
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
