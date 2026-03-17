#!/usr/bin/env bash
#
# validate_tpdg_integration.sh - Integration test for test-plan-doc-gen
#
# This script validates the integration between the verifier and test-plan-doc-gen by:
# 1. Running verifier on multiple test scenarios to generate container YAML
# 2. Invoking test-plan-documentation-generator on each container YAML
# 3. Validating generated AsciiDoc/Markdown/HTML reports exist and contain expected content markers
# 4. Checking exit codes for success/failure scenarios
# 5. Comparing report content quality with previous Python-generated reports for regressions
#
# Usage: ./scripts/validate_tpdg_integration.sh [OPTIONS]
#
# Options:
#   --scenarios-dir DIR        Directory containing test scenarios (default: testcases/verifier_scenarios)
#   --output-dir DIR           Output directory for reports (default: reports/tpdg_integration)
#   --test-plan-doc-gen DIR    Path to test-plan-doc-gen directory (default: ../test-plan-doc-gen)
#   --baseline-dir DIR         Directory with baseline reports for comparison (default: none)
#   --skip-build               Skip building binaries
#   --verbose                  Enable verbose output
#   --help                     Show this help message
#

set -e

# Get script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source required libraries
source "$SCRIPT_DIR/lib/logger.sh" || exit 1
source "$SCRIPT_DIR/lib/report_generator.sh" || exit 1

# Default configuration
SCENARIOS_DIR="$PROJECT_ROOT/testcases/verifier_scenarios"
OUTPUT_DIR="$PROJECT_ROOT/reports/tpdg_integration"
TEST_PLAN_DOC_GEN_DIR="../test-plan-doc-gen"
BASELINE_DIR=""
SKIP_BUILD=0

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --scenarios-dir)
            SCENARIOS_DIR="$2"
            shift 2
            ;;
        --output-dir)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --test-plan-doc-gen)
            TEST_PLAN_DOC_GEN_DIR="$2"
            shift 2
            ;;
        --baseline-dir)
            BASELINE_DIR="$2"
            shift 2
            ;;
        --skip-build)
            SKIP_BUILD=1
            shift
            ;;
        --verbose)
            export VERBOSE=1
            shift
            ;;
        --help)
            head -n 19 "$0" | tail -n +2 | sed 's/^# //'
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Display configuration
section "test-plan-doc-gen Integration Test"
log_info "Configuration:"
log_info "  Scenarios directory: $SCENARIOS_DIR"
log_info "  Output directory: $OUTPUT_DIR"
log_info "  test-plan-doc-gen: $TEST_PLAN_DOC_GEN_DIR"
if [[ -n "$BASELINE_DIR" ]]; then
    log_info "  Baseline directory: $BASELINE_DIR"
fi
echo ""

# Create output directory structure
mkdir -p "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR/verification"
mkdir -p "$OUTPUT_DIR/results"
mkdir -p "$OUTPUT_DIR/reports/asciidoc"
mkdir -p "$OUTPUT_DIR/reports/markdown"
mkdir -p "$OUTPUT_DIR/reports/html"

# Define file paths for reports
SUMMARY_REPORT="$OUTPUT_DIR/integration_test_summary.txt"
DETAILED_LOG="$OUTPUT_DIR/integration_test_detailed.log"

# Trap to always show summary and log files on exit
cleanup_and_show_reports() {
    local exit_code=$?
    
    echo ""
    section "Test Completion"
    
    if [[ -f "$SUMMARY_REPORT" ]]; then
        log_info "Summary report location: $SUMMARY_REPORT"
        echo ""
        echo "=== Summary Report ==="
        cat "$SUMMARY_REPORT"
        echo ""
    fi
    
    if [[ -f "$DETAILED_LOG" ]]; then
        log_info "Detailed log location: $DETAILED_LOG"
    fi
    
    exit $exit_code
}

trap cleanup_and_show_reports EXIT

# Test results tracking
declare -a PASSED_TESTS
declare -a FAILED_TESTS
declare -a CONTENT_CHECKS_PASSED
declare -a CONTENT_CHECKS_FAILED
declare -a REGRESSION_CHECKS

TOTAL_TESTS=0
PASSED_COUNT=0
FAILED_COUNT=0

# ============================================================================
# Step 1: Build required binaries
# ============================================================================

section "Step 1: Build Required Binaries"

if [[ $SKIP_BUILD -eq 0 ]]; then
    log_info "Building verifier binary..."
    cd "$PROJECT_ROOT"
    if cargo build --release --bin verifier 2>&1 | while IFS= read -r line; do
        log_verbose "$line"
    done; then
        pass "Verifier binary built successfully"
    else
        fail "Failed to build verifier binary"
        exit 1
    fi
    
    VERIFIER_BIN="$PROJECT_ROOT/target/release/verifier"
else
    log_info "Skipping binary build (--skip-build)"
    VERIFIER_BIN="$PROJECT_ROOT/target/release/verifier"
    
    if [[ ! -f "$VERIFIER_BIN" ]]; then
        fail "Verifier binary not found: $VERIFIER_BIN"
        log_error "Run without --skip-build or build manually"
        exit 1
    fi
    pass "Using existing verifier binary"
fi

# Resolve test-plan-doc-gen directory path
if [[ ! "$TEST_PLAN_DOC_GEN_DIR" = /* ]]; then
    TEST_PLAN_DOC_GEN_DIR="$PROJECT_ROOT/$TEST_PLAN_DOC_GEN_DIR"
fi

# Check if test-plan-doc-gen directory exists
if [[ ! -d "$TEST_PLAN_DOC_GEN_DIR" ]]; then
    log_error "test-plan-doc-gen directory not found: $TEST_PLAN_DOC_GEN_DIR"
    log_error "Please clone test-plan-doc-gen as a sibling directory:"
    log_error "  cd $(dirname "$PROJECT_ROOT")"
    log_error "  git clone <test-plan-doc-gen-repo-url> test-plan-doc-gen"
    exit 1
fi

# Build or find test-plan-doc-gen binary
if [[ $SKIP_BUILD -eq 0 ]]; then
    if check_test_plan_doc_gen_available "$TEST_PLAN_DOC_GEN_DIR"; then
        pass "test-plan-doc-gen binary found"
    else
        log_info "Building test-plan-doc-gen..."
        if build_test_plan_doc_gen "$TEST_PLAN_DOC_GEN_DIR"; then
            pass "test-plan-doc-gen built successfully"
        else
            fail "Failed to build test-plan-doc-gen"
            exit 1
        fi
    fi
else
    if check_test_plan_doc_gen_available "$TEST_PLAN_DOC_GEN_DIR"; then
        pass "Using existing test-plan-doc-gen binary"
    else
        fail "test-plan-doc-gen binary not found"
        log_error "Run without --skip-build or build manually"
        exit 1
    fi
fi

export TEST_PLAN_DOC_GEN=$(find_test_plan_doc_gen "$TEST_PLAN_DOC_GEN_DIR")
log_verbose "test-plan-doc-gen binary: $TEST_PLAN_DOC_GEN"

# ============================================================================
# Step 2: Discover test scenarios
# ============================================================================

section "Step 2: Discover Test Scenarios"

if [[ ! -d "$SCENARIOS_DIR" ]]; then
    fail "Scenarios directory not found: $SCENARIOS_DIR"
    exit 1
fi

# Find all test case YAML files with execution logs
declare -a TEST_SCENARIOS

while IFS= read -r test_case_file; do
    test_case_name=$(basename "$test_case_file" .yml)
    test_case_name=$(basename "$test_case_name" .yaml)
    test_case_dir=$(dirname "$test_case_file")
    
    # Check for execution log
    execution_log="$test_case_dir/${test_case_name}_execution_log.json"
    
    if [[ -f "$execution_log" ]]; then
        TEST_SCENARIOS+=("$test_case_file")
        log_verbose "Found test scenario: $test_case_name"
    else
        log_verbose "Skipping $test_case_name (no execution log)"
    fi
done < <(find "$SCENARIOS_DIR" -type f \( -name "*.yml" -o -name "*.yaml" \) \
    ! -name "*_execution_log*" \
    ! -name "*_result*" \
    ! -name "*container*" 2>/dev/null)

SCENARIO_COUNT=${#TEST_SCENARIOS[@]}

if [[ $SCENARIO_COUNT -eq 0 ]]; then
    fail "No test scenarios found in: $SCENARIOS_DIR"
    exit 1
fi

pass "Found $SCENARIO_COUNT test scenario(s)"

# ============================================================================
# Step 3: Run verifier and generate container YAML for each scenario
# ============================================================================

section "Step 3: Run Verifier and Generate Container YAML"

declare -a CONTAINER_YAMLS

for test_case_file in "${TEST_SCENARIOS[@]}"; do
    test_case_name=$(basename "$test_case_file" .yml)
    test_case_name=$(basename "$test_case_name" .yaml)
    test_case_dir=$(dirname "$test_case_file")
    
    log_info "Processing: $test_case_name"
    
    # Paths for this test case
    execution_log="$test_case_dir/${test_case_name}_execution_log.json"
    verification_json="$OUTPUT_DIR/verification/${test_case_name}_verification.json"
    result_yaml="$OUTPUT_DIR/results/${test_case_name}_result.yaml"
    
    # Run verifier
    log_verbose "Running verifier for $test_case_name..."
    
    VERIFIER_EXIT=0
    if "$VERIFIER_BIN" \
        --log "$execution_log" \
        --test-case "$test_case_name" \
        --format json \
        --output "$verification_json" \
        --test-case-dir "$test_case_dir" 2>&1 | while IFS= read -r line; do
            log_verbose "$line"
        done; then
        log_verbose "Verifier succeeded for $test_case_name"
    else
        VERIFIER_EXIT=$?
        # Exit code 1 means test failures, which is expected for some scenarios
        if [[ $VERIFIER_EXIT -eq 1 ]]; then
            log_verbose "Verifier completed with test failures (expected for failure scenarios)"
        else
            log_warning "Verifier failed with exit code: $VERIFIER_EXIT"
        fi
    fi
    
    # Check verification JSON was generated
    if [[ ! -f "$verification_json" ]]; then
        fail "Verification JSON not generated for: $test_case_name"
        FAILED_TESTS+=("$test_case_name: verification generation failed")
        continue
    fi
    
    pass "Generated verification JSON: $(basename "$verification_json")"
    
    # Convert verification JSON to result YAML using Rust binary
    log_verbose "Converting to result YAML..."

    # Use the compiled Rust binary for better performance
    JSON_TO_YAML_BIN="$PROJECT_ROOT/target/release/json-to-yaml"

    # Fall back to debug build if release not available
    if [[ ! -f "$JSON_TO_YAML_BIN" ]]; then
        JSON_TO_YAML_BIN="$PROJECT_ROOT/target/debug/json-to-yaml"
    fi

    if [[ ! -f "$JSON_TO_YAML_BIN" ]]; then
        fail "json-to-yaml binary not found - ensure project is built with: cargo build --release"
        exit 1
    fi

    if "$JSON_TO_YAML_BIN" "$verification_json" -o "$OUTPUT_DIR/results" > /tmp/conversion_output.txt 2>&1; then
        cat /tmp/conversion_output.txt | while IFS= read -r line; do
            log_verbose "$line"
        done
        log_verbose "Conversion succeeded"
    else
        fail "Failed to convert verification JSON to result YAML for: $test_case_name"
        cat /tmp/conversion_output.txt
        FAILED_TESTS+=("$test_case_name: YAML conversion failed")
        continue
    fi
    
    # Check result YAML was generated
    if [[ ! -f "$result_yaml" ]]; then
        fail "Result YAML not generated for: $test_case_name"
        FAILED_TESTS+=("$test_case_name: result YAML not found")
        continue
    fi
    
    pass "Generated result YAML: $(basename "$result_yaml")"
    
    # Create container YAML for this test case
    container_yaml="$OUTPUT_DIR/results/${test_case_name}_container.yaml"
    
    log_verbose "Creating container YAML..."
    
    cat > "$container_yaml" << EOF
title: 'Test Execution Results - $test_case_name'
date: '$(date +%Y-%m-%d)'
product: 'Test Case Manager'
description: 'Integration test results for $test_case_name'
project: 'Test Case Manager - Integration Test'
test_date: '$(date +%Y-%m-%dT%H:%M:%S)'
test_results:
EOF
    
    # Append result content (without 'type: result' line)
    sed '/^type: result/d' "$result_yaml" | sed 's/^/  /' >> "$container_yaml"
    
    # Add metadata
    cat >> "$container_yaml" << EOF
metadata:
  environment: 'Integration Test Environment'
  platform: 'Test Case Manager Integration Tests'
  executor: 'validate_tpdg_integration.sh'
  total_test_cases: 1
EOF
    
    pass "Created container YAML: $(basename "$container_yaml")"
    
    CONTAINER_YAMLS+=("$container_yaml")
done

log_info "Generated ${#CONTAINER_YAMLS[@]} container YAML file(s)"

# ============================================================================
# Step 4: Generate reports using test-plan-doc-gen
# ============================================================================

section "Step 4: Generate Reports with test-plan-doc-gen"

for container_yaml in "${CONTAINER_YAMLS[@]}"; do
    container_name=$(basename "$container_yaml" _container.yaml)
    
    log_info "Generating reports for: $container_name"

    ((TOTAL_TESTS++)) || true

    # Generate AsciiDoc report
    asciidoc_output="$OUTPUT_DIR/reports/asciidoc/${container_name}.adoc"

    log_info "Generating AsciiDoc report for: $container_name"
    log_verbose "  Output: $asciidoc_output"
    log_verbose "  Container: $container_yaml"

    # Check if container YAML exists
    if [[ ! -f "$container_yaml" ]]; then
        log_error "Container YAML not found: $container_yaml"
        FAILED_TESTS+=("$container_name: container YAML missing")
        ((FAILED_COUNT++)) || true
        continue
    fi

    # Define paths to container schema, template, and verification methods
    container_schema="$TEST_PLAN_DOC_GEN_DIR/data/container/schema.json"
    container_template_asciidoc="$TEST_PLAN_DOC_GEN_DIR/data/container/template_asciidoc.adoc"
    verification_methods="$TEST_PLAN_DOC_GEN_DIR/data/verification_methods"

    # Get result YAML path for this container
    result_yaml="$OUTPUT_DIR/results/${container_name}_result.yaml"

    # Check required files exist
    if [[ ! -f "$container_schema" ]]; then
        log_warning "Container schema not found: $container_schema"
    fi
    if [[ ! -f "$container_template_asciidoc" ]]; then
        log_warning "AsciiDoc template not found: $container_template_asciidoc"
    fi
    if [[ ! -d "$verification_methods" ]]; then
        log_warning "Verification methods directory not found: $verification_methods"
    fi
    if [[ ! -f "$result_yaml" ]]; then
        log_error "Result YAML not found: $result_yaml"
        FAILED_TESTS+=("$container_name: result YAML missing")
        ((FAILED_COUNT++)) || true
        continue
    fi

    ASCIIDOC_EXIT=0
    if invoke_test_plan_doc_gen \
        --container "$container_schema" "$container_template_asciidoc" "$container_yaml" \
        --test-case "$verification_methods" "$result_yaml" \
        --output "$asciidoc_output" \
        --format asciidoc; then
        ASCIIDOC_EXIT=0
    else
        ASCIIDOC_EXIT=$?
    fi
    
    if [[ $ASCIIDOC_EXIT -eq 0 ]]; then
        # Assert that the output file was actually generated
        if [[ ! -f "$asciidoc_output" ]]; then
            fail "AsciiDoc output file not generated: $(basename "$asciidoc_output")"
            FAILED_TESTS+=("$container_name: AsciiDoc file not created")
            ((FAILED_COUNT++)) || true
            continue
        fi
        
        # Assert that the output file has content (not empty)
        if [[ ! -s "$asciidoc_output" ]]; then
            fail "AsciiDoc output file is empty: $(basename "$asciidoc_output")"
            FAILED_TESTS+=("$container_name: AsciiDoc file is empty")
            ((FAILED_COUNT++)) || true
            continue
        fi
        
        pass "Generated AsciiDoc: $(basename "$asciidoc_output")"
        log_verbose "File size: $(stat -f%z "$asciidoc_output" 2>/dev/null || stat -c%s "$asciidoc_output" 2>/dev/null) bytes"
    else
        fail "Failed to generate AsciiDoc report (exit code: $ASCIIDOC_EXIT)"
        FAILED_TESTS+=("$container_name: AsciiDoc generation failed (exit $ASCIIDOC_EXIT)")
        ((FAILED_COUNT++)) || true
        continue
    fi
    
    # Generate Markdown report
    markdown_output="$OUTPUT_DIR/reports/markdown/${container_name}.md"
    container_template_markdown="$TEST_PLAN_DOC_GEN_DIR/data/container/template.j2"

    log_info "Generating Markdown report for: $container_name"

    MARKDOWN_EXIT=0
    if invoke_test_plan_doc_gen \
        --container "$container_schema" "$container_template_markdown" "$container_yaml" \
        --test-case "$verification_methods" "$result_yaml" \
        --output "$markdown_output" \
        --format markdown; then
        MARKDOWN_EXIT=0
    else
        MARKDOWN_EXIT=$?
    fi

    if [[ $MARKDOWN_EXIT -eq 0 ]]; then
        # Assert that the output file was actually generated
        if [[ ! -f "$markdown_output" ]]; then
            fail "Markdown output file not generated: $(basename "$markdown_output")"
            FAILED_TESTS+=("$container_name: Markdown file not created")
            ((FAILED_COUNT++)) || true
            continue
        fi

        # Assert that the output file has content (not empty)
        if [[ ! -s "$markdown_output" ]]; then
            fail "Markdown output file is empty: $(basename "$markdown_output")"
            FAILED_TESTS+=("$container_name: Markdown file is empty")
            ((FAILED_COUNT++)) || true
            continue
        fi

        pass "Generated Markdown: $(basename "$markdown_output")"
        log_verbose "File size: $(stat -f%z "$markdown_output" 2>/dev/null || stat -c%s "$markdown_output" 2>/dev/null) bytes"
    else
        fail "Failed to generate Markdown report (exit code: $MARKDOWN_EXIT)"
        FAILED_TESTS+=("$container_name: Markdown generation failed (exit $MARKDOWN_EXIT)")
        ((FAILED_COUNT++)) || true
        continue
    fi
    
    # Try to generate HTML report (may not be supported)
    html_output="$OUTPUT_DIR/reports/html/${container_name}.html"

    log_verbose "Attempting to generate HTML report for: $container_name"

    HTML_EXIT=0
    if invoke_test_plan_doc_gen \
        --container "$container_schema" "$container_template_asciidoc" "$container_yaml" \
        --test-case "$verification_methods" "$result_yaml" \
        --output "$html_output" \
        --format asciidoc; then
        HTML_EXIT=0
    else
        HTML_EXIT=$?
    fi

    if [[ $HTML_EXIT -eq 0 ]]; then
        # Assert that the output file was actually generated
        if [[ ! -f "$html_output" ]]; then
            fail "HTML output file not generated: $(basename "$html_output")"
            log_verbose "HTML generation may not be supported"
        elif [[ ! -s "$html_output" ]]; then
            fail "HTML output file is empty: $(basename "$html_output")"
            log_verbose "HTML generation may not be supported"
        else
            pass "Generated HTML: $(basename "$html_output")"
        fi
    else
        log_verbose "HTML generation not supported or failed (exit code: $HTML_EXIT)"
    fi
    
    PASSED_TESTS+=("$container_name: reports generated successfully")
    ((PASSED_COUNT++)) || true
done

# ============================================================================
# Step 5: Validate report content
# ============================================================================

section "Step 5: Validate Report Content"

# Function to check if file contains expected content markers
check_content_markers() {
    local file="$1"
    local test_name="$2"
    local markers=("${@:3}")
    
    if [[ ! -f "$file" ]]; then
        log_warning "File not found: $file"
        return 1
    fi
    
    local missing_markers=0
    local found_markers=0
    
    for marker in "${markers[@]}"; do
        if grep -q "$marker" "$file" 2>/dev/null; then
            log_verbose "  ✓ Found marker: $marker"
            ((found_markers++)) || true
        else
            log_verbose "  ✗ Missing marker: $marker"
            ((missing_markers++)) || true
        fi
    done
    
    if [[ $missing_markers -eq 0 ]]; then
        pass "$test_name: all content markers found ($found_markers/${#markers[@]})"
        return 0
    else
        fail "$test_name: missing $missing_markers/${#markers[@]} content markers"
        return 1
    fi
}

# Validate content for each generated report
for container_yaml in "${CONTAINER_YAMLS[@]}"; do
    container_name=$(basename "$container_yaml" _container.yaml)
    
    log_info "Validating content for: $container_name"
    
    # Extract test case ID from container name
    test_case_id="$container_name"
    
    # Common content markers to check
    markers=(
        "$test_case_id"
        "Test"
        "Sequence"
        "Step"
    )
    
    # Check AsciiDoc content
    asciidoc_file="$OUTPUT_DIR/reports/asciidoc/${container_name}.adoc"
    if [[ -f "$asciidoc_file" ]]; then
        if check_content_markers "$asciidoc_file" "AsciiDoc" "${markers[@]}"; then
            CONTENT_CHECKS_PASSED+=("$container_name: AsciiDoc content valid")
        else
            CONTENT_CHECKS_FAILED+=("$container_name: AsciiDoc content incomplete")
        fi
    fi
    
    # Check Markdown content
    markdown_file="$OUTPUT_DIR/reports/markdown/${container_name}.md"
    if [[ -f "$markdown_file" ]]; then
        if check_content_markers "$markdown_file" "Markdown" "${markers[@]}"; then
            CONTENT_CHECKS_PASSED+=("$container_name: Markdown content valid")
        else
            CONTENT_CHECKS_FAILED+=("$container_name: Markdown content incomplete")
        fi
    fi
    
    # Check HTML content if it exists
    html_file="$OUTPUT_DIR/reports/html/${container_name}.html"
    if [[ -f "$html_file" ]]; then
        if check_content_markers "$html_file" "HTML" "${markers[@]}"; then
            CONTENT_CHECKS_PASSED+=("$container_name: HTML content valid")
        else
            CONTENT_CHECKS_FAILED+=("$container_name: HTML content incomplete")
        fi
    fi
done

# ============================================================================
# Step 6: Compare with baseline reports (if provided)
# ============================================================================

if [[ -n "$BASELINE_DIR" ]] && [[ -d "$BASELINE_DIR" ]]; then
    section "Step 6: Compare with Baseline Reports"
    
    log_info "Comparing against baseline reports in: $BASELINE_DIR"
    
    for container_yaml in "${CONTAINER_YAMLS[@]}"; do
        container_name=$(basename "$container_yaml" _container.yaml)
        
        log_info "Comparing reports for: $container_name"
        
        # Compare AsciiDoc reports
        baseline_asciidoc="$BASELINE_DIR/asciidoc/${container_name}.adoc"
        current_asciidoc="$OUTPUT_DIR/reports/asciidoc/${container_name}.adoc"
        
        if [[ -f "$baseline_asciidoc" ]] && [[ -f "$current_asciidoc" ]]; then
            baseline_size=$(stat -f%z "$baseline_asciidoc" 2>/dev/null || stat -c%s "$baseline_asciidoc" 2>/dev/null || echo "0")
            current_size=$(stat -f%z "$current_asciidoc" 2>/dev/null || stat -c%s "$current_asciidoc" 2>/dev/null || echo "0")
            
            # Check if current report is significantly smaller (potential regression)
            size_ratio=$(awk "BEGIN {if ($baseline_size > 0) print ($current_size / $baseline_size); else print 1}")
            
            if awk "BEGIN {exit !($size_ratio >= 0.8)}"; then
                pass "AsciiDoc size check: $current_size bytes (baseline: $baseline_size bytes)"
                REGRESSION_CHECKS+=("$container_name: AsciiDoc size OK (${size_ratio}x)")
            else
                fail "AsciiDoc size regression: $current_size bytes (baseline: $baseline_size bytes)"
                REGRESSION_CHECKS+=("$container_name: AsciiDoc size REGRESSION (${size_ratio}x)")
            fi
        else
            log_verbose "No baseline AsciiDoc report for comparison"
        fi
        
        # Compare Markdown reports
        baseline_markdown="$BASELINE_DIR/markdown/${container_name}.md"
        current_markdown="$OUTPUT_DIR/reports/markdown/${container_name}.md"
        
        if [[ -f "$baseline_markdown" ]] && [[ -f "$current_markdown" ]]; then
            baseline_size=$(stat -f%z "$baseline_markdown" 2>/dev/null || stat -c%s "$baseline_markdown" 2>/dev/null || echo "0")
            current_size=$(stat -f%z "$current_markdown" 2>/dev/null || stat -c%s "$current_markdown" 2>/dev/null || echo "0")
            
            size_ratio=$(awk "BEGIN {if ($baseline_size > 0) print ($current_size / $baseline_size); else print 1}")
            
            if awk "BEGIN {exit !($size_ratio >= 0.8)}"; then
                pass "Markdown size check: $current_size bytes (baseline: $baseline_size bytes)"
                REGRESSION_CHECKS+=("$container_name: Markdown size OK (${size_ratio}x)")
            else
                fail "Markdown size regression: $current_size bytes (baseline: $baseline_size bytes)"
                REGRESSION_CHECKS+=("$container_name: Markdown size REGRESSION (${size_ratio}x)")
            fi
        else
            log_verbose "No baseline Markdown report for comparison"
        fi
    done
else
    log_info "Skipping baseline comparison (no baseline directory specified)"
fi

# ============================================================================
# Step 7: Generate summary report
# ============================================================================

section "Step 7: Generate Summary Report"

cat > "$SUMMARY_REPORT" << EOF
test-plan-doc-gen Integration Test Summary
==========================================

Test Date: $(date +%Y-%m-%dT%H:%M:%S)
Scenarios Directory: $SCENARIOS_DIR
Output Directory: $OUTPUT_DIR

Test Execution Summary
----------------------
Total Test Scenarios: $SCENARIO_COUNT
Total Report Generation Tests: $TOTAL_TESTS
Passed: $PASSED_COUNT
Failed: $FAILED_COUNT
Success Rate: $(awk "BEGIN {if ($TOTAL_TESTS > 0) printf \"%.1f\", ($PASSED_COUNT / $TOTAL_TESTS) * 100; else print 0}")%

Content Validation Summary
--------------------------
Content Checks Passed: ${#CONTENT_CHECKS_PASSED[@]}
Content Checks Failed: ${#CONTENT_CHECKS_FAILED[@]}

EOF

if [[ ${#REGRESSION_CHECKS[@]} -gt 0 ]]; then
    cat >> "$SUMMARY_REPORT" << EOF
Regression Checks
-----------------
EOF
    for check in "${REGRESSION_CHECKS[@]}"; do
        echo "  • $check" >> "$SUMMARY_REPORT"
    done
    echo "" >> "$SUMMARY_REPORT"
fi

if [[ ${#PASSED_TESTS[@]} -gt 0 ]]; then
    cat >> "$SUMMARY_REPORT" << EOF
Passed Tests
------------
EOF
    for test in "${PASSED_TESTS[@]}"; do
        echo "  ✓ $test" >> "$SUMMARY_REPORT"
    done
    echo "" >> "$SUMMARY_REPORT"
fi

if [[ ${#FAILED_TESTS[@]} -gt 0 ]]; then
    cat >> "$SUMMARY_REPORT" << EOF
Failed Tests
------------
EOF
    for test in "${FAILED_TESTS[@]}"; do
        echo "  ✗ $test" >> "$SUMMARY_REPORT"
    done
    echo "" >> "$SUMMARY_REPORT"
fi

if [[ ${#CONTENT_CHECKS_FAILED[@]} -gt 0 ]]; then
    cat >> "$SUMMARY_REPORT" << EOF
Content Validation Failures
---------------------------
EOF
    for check in "${CONTENT_CHECKS_FAILED[@]}"; do
        echo "  ✗ $check" >> "$SUMMARY_REPORT"
    done
    echo "" >> "$SUMMARY_REPORT"
fi

cat >> "$SUMMARY_REPORT" << EOF
Generated Files
---------------
Verification JSON: $SCENARIO_COUNT
Result YAML: ${#CONTAINER_YAMLS[@]}
Container YAML: ${#CONTAINER_YAMLS[@]}
AsciiDoc Reports: $(find "$OUTPUT_DIR/reports/asciidoc" -name "*.adoc" 2>/dev/null | wc -l | tr -d ' ')
Markdown Reports: $(find "$OUTPUT_DIR/reports/markdown" -name "*.md" 2>/dev/null | wc -l | tr -d ' ')
HTML Reports: $(find "$OUTPUT_DIR/reports/html" -name "*.html" 2>/dev/null | wc -l | tr -d ' ')

Output Directory Structure
--------------------------
$OUTPUT_DIR/
  verification/          - Verification JSON files from verifier
  results/               - Result YAML and container YAML files
  reports/
    asciidoc/            - Generated AsciiDoc reports
    markdown/            - Generated Markdown reports
    html/                - Generated HTML reports (if supported)
  integration_test_summary.txt - This summary report

For detailed logs, run with --verbose flag.
EOF

pass "Summary report generated: $SUMMARY_REPORT"

# ============================================================================
# Exit with appropriate status
# ============================================================================

section "Complete"

# Create detailed log file
{
    echo "test-plan-doc-gen Integration Test - Detailed Log"
    echo "=================================================="
    echo "Test Date: $(date +%Y-%m-%dT%H:%M:%S)"
    echo "Test Plan Doc Gen Directory: $TEST_PLAN_DOC_GEN_DIR"
    echo ""
    echo "Binary Information:"
    echo "  Binary: $TEST_PLAN_DOC_GEN"
    if [[ -x "$TEST_PLAN_DOC_GEN" ]]; then
        echo "  Version: $($TEST_PLAN_DOC_GEN --version 2>&1 || echo 'unknown')"
    fi
    echo ""
    echo "Test Results:"
    echo "  Total Scenarios: $SCENARIO_COUNT"
    echo "  Total Tests: $TOTAL_TESTS"
    echo "  Passed: $PASSED_COUNT"
    echo "  Failed: $FAILED_COUNT"
    echo ""
    
    if [[ ${#FAILED_TESTS[@]} -gt 0 ]]; then
        echo "Failed Tests Details:"
        for test in "${FAILED_TESTS[@]}"; do
            echo "  - $test"
        done
        echo ""
    fi
    
    if [[ ${#PASSED_TESTS[@]} -gt 0 ]]; then
        echo "Passed Tests:"
        for test in "${PASSED_TESTS[@]}"; do
            echo "  - $test"
        done
        echo ""
    fi
    
    echo "File Structure:"
    echo "  Output Directory: $OUTPUT_DIR"
    echo "  Verification JSON files: $(find "$OUTPUT_DIR/verification" -name "*.json" 2>/dev/null | wc -l | tr -d ' ')"
    echo "  Result YAML files: $(find "$OUTPUT_DIR/results" -name "*_result.yaml" 2>/dev/null | wc -l | tr -d ' ')"
    echo "  Container YAML files: $(find "$OUTPUT_DIR/results" -name "*_container.yaml" 2>/dev/null | wc -l | tr -d ' ')"
    echo "  AsciiDoc reports: $(find "$OUTPUT_DIR/reports/asciidoc" -name "*.adoc" 2>/dev/null | wc -l | tr -d ' ')"
    echo "  Markdown reports: $(find "$OUTPUT_DIR/reports/markdown" -name "*.md" 2>/dev/null | wc -l | tr -d ' ')"
    echo "  HTML reports: $(find "$OUTPUT_DIR/reports/html" -name "*.html" 2>/dev/null | wc -l | tr -d ' ')"
    echo ""
    
    echo "Expected Files:"
    echo "  Container Schema: $TEST_PLAN_DOC_GEN_DIR/data/container/schema.json"
    echo "    Exists: $(if [[ -f "$TEST_PLAN_DOC_GEN_DIR/data/container/schema.json" ]]; then echo "yes"; else echo "no"; fi)"
    echo "  AsciiDoc Template: $TEST_PLAN_DOC_GEN_DIR/data/container/template_asciidoc.adoc"
    echo "    Exists: $(if [[ -f "$TEST_PLAN_DOC_GEN_DIR/data/container/template_asciidoc.adoc" ]]; then echo "yes"; else echo "no"; fi)"
    echo "  Markdown Template: $TEST_PLAN_DOC_GEN_DIR/data/container/template_markdown.md"
    echo "    Exists: $(if [[ -f "$TEST_PLAN_DOC_GEN_DIR/data/container/template_markdown.md" ]]; then echo "yes"; else echo "no"; fi)"
    echo "  Verification Methods: $TEST_PLAN_DOC_GEN_DIR/data/verification_methods"
    echo "    Exists: $(if [[ -d "$TEST_PLAN_DOC_GEN_DIR/data/verification_methods" ]]; then echo "yes"; else echo "no"; fi)"
    echo ""
    
    echo "Notes:"
    if [[ $FAILED_COUNT -gt 0 ]]; then
        echo "  - Some report generation tests failed"
        echo "  - This may be due to missing template files in test-plan-documentation-generator"
        echo "  - Check that all required files exist in the test-plan-doc-gen directory"
    fi
    if [[ ${#CONTENT_CHECKS_FAILED[@]} -gt 0 ]]; then
        echo "  - Some content validation checks failed"
        echo "  - Generated reports may be missing expected content"
    fi
    
} > "$DETAILED_LOG"

log_info "Detailed log saved to: $DETAILED_LOG"

if [[ $FAILED_COUNT -gt 0 ]] || [[ ${#CONTENT_CHECKS_FAILED[@]} -gt 0 ]]; then
    fail "Integration test completed with failures"
    log_error "Review the summary report for details: $SUMMARY_REPORT"
    log_error "Review the detailed log for more information: $DETAILED_LOG"
    exit 1
fi

pass "Integration test completed successfully!"
log_info "All reports generated and validated"
log_info "Summary report: $SUMMARY_REPORT"
log_info "Detailed log: $DETAILED_LOG"
exit 0
