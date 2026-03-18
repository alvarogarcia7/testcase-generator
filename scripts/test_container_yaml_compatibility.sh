#!/usr/bin/env bash
#
# Test container YAML compatibility with test-plan-doc-gen
#
# This script:
# 1. Runs the verifier on test scenarios to generate verification results
# 2. Converts verification results to container YAML format
# 3. Validates container YAML files using the compatibility checker
# 4. Generates a compatibility report
#

set -e

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source logger library
source "$SCRIPT_DIR/lib/logger.sh" || exit 1
source "$SCRIPT_DIR/lib/find-binary.sh" || exit 1

section "Container YAML Compatibility Test"

log_info "Project root: $PROJECT_ROOT"

# Build binaries
section "Building Binaries"

log_info "Building verifier..."
cargo build --release --bin verifier

log_info "Building test-plan-documentation-generator-compat..."
cargo build --release --bin test-plan-documentation-generator-compat

VERIFIER="$PROJECT_ROOT/target/release/verifier"
COMPAT_CHECKER="$PROJECT_ROOT/target/release/test-plan-documentation-generator-compat"

if [ ! -f "$VERIFIER" ]; then
    log_error "Verifier binary not found: $VERIFIER"
    exit 1
fi

if [ ! -f "$COMPAT_CHECKER" ]; then
    log_error "Compatibility checker binary not found: $COMPAT_CHECKER"
    exit 1
fi

pass "Binaries built successfully"

# Create output directory
OUTPUT_DIR="$PROJECT_ROOT/reports/container_yaml_compatibility"
mkdir -p "$OUTPUT_DIR"

log_info "Output directory: $OUTPUT_DIR"

# Run verifier on test scenarios
section "Running Verifier on Test Scenarios"

SCENARIOS_DIR="$PROJECT_ROOT/testcases/verifier_scenarios"

if [ ! -d "$SCENARIOS_DIR" ]; then
    log_error "Scenarios directory not found: $SCENARIOS_DIR"
    exit 1
fi

# Find all test case YAML files in verifier_scenarios
TEST_CASES=$(find "$SCENARIOS_DIR" -type f \( -name "*.yml" -o -name "*.yaml" \) \
    ! -name "*_execution_log*" \
    ! -name "*_result*" \
    ! -name "*container*")

TEST_CASE_COUNT=$(echo "$TEST_CASES" | wc -l | tr -d ' ')

log_info "Found $TEST_CASE_COUNT test case(s) in verifier scenarios"

# Process each test case
CONTAINER_YAMLS=()

for TEST_CASE_FILE in $TEST_CASES; do
    TEST_CASE_NAME=$(basename "$TEST_CASE_FILE" .yml)
    TEST_CASE_DIR=$(dirname "$TEST_CASE_FILE")
    
    log_info "Processing: $TEST_CASE_NAME"
    
    # Check for execution log
    EXECUTION_LOG="$TEST_CASE_DIR/${TEST_CASE_NAME}_execution_log.json"
    
    if [ ! -f "$EXECUTION_LOG" ]; then
        log_warning "Execution log not found: $EXECUTION_LOG"
        log_warning "Skipping $TEST_CASE_NAME"
        continue
    fi
    
    # Run verifier
    VERIFICATION_JSON="$OUTPUT_DIR/${TEST_CASE_NAME}_verification.json"
    
    log_verbose "Running verifier for $TEST_CASE_NAME..."
    
    if "$VERIFIER" \
        --log "$EXECUTION_LOG" \
        --test-case "$TEST_CASE_NAME" \
        --format json \
        --output "$VERIFICATION_JSON" \
        --test-case-dir "$SCENARIOS_DIR" > /dev/null 2>&1; then
        log_verbose "Verifier completed for $TEST_CASE_NAME"
    else
        log_warning "Verifier failed for $TEST_CASE_NAME (this may be expected for failure scenarios)"
    fi
    
    if [ -f "$VERIFICATION_JSON" ]; then
        pass "Generated verification JSON: $VERIFICATION_JSON"
        
        # Convert to result YAML
        RESULT_YAML="$OUTPUT_DIR/${TEST_CASE_NAME}_result.yaml"
        
        log_verbose "Converting to result YAML..."
        
        # Find Python interpreter
        PYTHON_CMD=$(find_python)
        if [[ -z "$PYTHON_CMD" ]]; then
            log_error "Python interpreter not found"
            exit 1
        fi
        
        if $PYTHON_CMD "$SCRIPT_DIR/convert_verification_to_result_yaml.py" \
            "$VERIFICATION_JSON" \
            -o "$OUTPUT_DIR" > /dev/null 2>&1; then
            log_verbose "Converted to result YAML"
        else
            log_warning "Failed to convert to result YAML"
        fi
        
        if [ -f "$RESULT_YAML" ]; then
            pass "Generated result YAML: $RESULT_YAML"
            CONTAINER_YAMLS+=("$RESULT_YAML")
        fi
    fi
done

# Also check for existing container YAML files
section "Finding Existing Container YAML Files"

EXISTING_CONTAINERS=$(find "$SCENARIOS_DIR" -type f \( -name "*container*.yml" -o -name "*container*.yaml" \) 2>/dev/null || true)

if [ -n "$EXISTING_CONTAINERS" ]; then
    EXISTING_COUNT=$(echo "$EXISTING_CONTAINERS" | wc -l | tr -d ' ')
    log_info "Found $EXISTING_COUNT existing container YAML file(s)"
    
    for CONTAINER in $EXISTING_CONTAINERS; do
        CONTAINER_YAMLS+=("$CONTAINER")
    done
else
    log_info "No existing container YAML files found"
fi

# Check the template container file
TEMPLATE_CONTAINER="$PROJECT_ROOT/testcases/expected_output_reports/container_data.yml"
if [ -f "$TEMPLATE_CONTAINER" ]; then
    log_info "Including template container: $TEMPLATE_CONTAINER"
    CONTAINER_YAMLS+=("$TEMPLATE_CONTAINER")
fi

# Validate all container YAML files
section "Validating Container YAML Files"

TOTAL_CONTAINERS=${#CONTAINER_YAMLS[@]}

if [ "$TOTAL_CONTAINERS" -eq 0 ]; then
    log_error "No container YAML files to validate"
    exit 1
fi

log_info "Validating $TOTAL_CONTAINERS container YAML file(s)..."

VALID_COUNT=0
INVALID_COUNT=0

for CONTAINER_YAML in "${CONTAINER_YAMLS[@]}"; do
    log_info "Validating: $(basename "$CONTAINER_YAML")"
    
    if "$COMPAT_CHECKER" "$CONTAINER_YAML" > /dev/null 2>&1; then
        pass "Valid: $(basename "$CONTAINER_YAML")"
        VALID_COUNT=$((VALID_COUNT + 1))
    else
        fail "Invalid: $(basename "$CONTAINER_YAML")"
        INVALID_COUNT=$((INVALID_COUNT + 1))
        
        # Show detailed errors
        "$COMPAT_CHECKER" "$CONTAINER_YAML" || true
    fi
done

# Generate compatibility report
section "Generating Compatibility Report"

REPORT_MD="$OUTPUT_DIR/compatibility_report.md"
REPORT_TXT="$OUTPUT_DIR/compatibility_report.txt"

log_info "Generating Markdown report..."
if "$COMPAT_CHECKER" report "$OUTPUT_DIR" --output "$REPORT_MD" --format markdown; then
    pass "Markdown report: $REPORT_MD"
else
    log_warning "Failed to generate Markdown report"
fi

log_info "Generating text report..."
if "$COMPAT_CHECKER" report "$OUTPUT_DIR" --output "$REPORT_TXT" --format text; then
    pass "Text report: $REPORT_TXT"
else
    log_warning "Failed to generate text report"
fi

# Summary
section "Summary"

info "Total Containers: $TOTAL_CONTAINERS"
pass "Valid: $VALID_COUNT"

if [ "$INVALID_COUNT" -gt 0 ]; then
    fail "Invalid: $INVALID_COUNT"
else
    pass "Invalid: $INVALID_COUNT"
fi

SUCCESS_RATE=$(awk "BEGIN {printf \"%.1f\", ($VALID_COUNT / $TOTAL_CONTAINERS) * 100}")
info "Success Rate: $SUCCESS_RATE%"

log_info ""
log_info "Reports generated in: $OUTPUT_DIR"
log_info "  • Markdown report: $REPORT_MD"
log_info "  • Text report: $REPORT_TXT"

# Display report summary
if [ -f "$REPORT_MD" ]; then
    section "Compatibility Report Summary"
    
    # Extract summary section from markdown report
    sed -n '/## Summary/,/## Compatibility Issues Summary/p' "$REPORT_MD" | head -n -1
fi

section "Complete"

if [ "$INVALID_COUNT" -gt 0 ]; then
    log_warning "Some container YAML files have compatibility issues"
    log_info "Review the compatibility report for details"
    exit 1
fi

pass "All container YAML files are compatible with test-plan-doc-gen"
exit 0
