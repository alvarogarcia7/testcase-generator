#!/usr/bin/env bash
#
# E2E test for container YAML compatibility checker
#

set -e

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"

# Source shared libraries
source "$PROJECT_ROOT/scripts/lib/find-binary.sh" || exit 1
source "$PROJECT_ROOT/scripts/lib/logger.sh" || exit 1

section "Container YAML Compatibility E2E Test"

# Build the binary
log_info "Building test-plan-documentation-generator-compat binary..."
cargo build --bin test-plan-documentation-generator-compat

# Find binary using workspace-aware search
cd "$PROJECT_ROOT"
COMPAT_BIN=$(find_binary "test-plan-documentation-generator-compat")
if [[ -z "$COMPAT_BIN" ]]; then
    log_error "Binary not found after build"
    exit 1
fi

if [ ! -f "$COMPAT_BIN" ]; then
    log_error "Binary not found: $COMPAT_BIN"
    exit 1
fi

pass "Binary built successfully"

# Create test directory
TEST_DIR=$(mktemp -d)
setup_cleanup "$TEST_DIR"

log_info "Test directory: $TEST_DIR"

# Test 1: Validate the example container file
section "Test 1: Validate Example Container"

EXAMPLE_CONTAINER="$PROJECT_ROOT/testcases/expected_output_reports/container_data.yml"

if [ ! -f "$EXAMPLE_CONTAINER" ]; then
    log_error "Example container not found: $EXAMPLE_CONTAINER"
    exit 1
fi

log_info "Validating example container..."

if "$COMPAT_BIN" validate "$EXAMPLE_CONTAINER" --verbose > "$TEST_DIR/validate_output.txt" 2>&1; then
    pass "Example container is valid"
else
    fail "Example container validation failed"
    cat "$TEST_DIR/validate_output.txt"
    exit 1
fi

# Test 2: Create a minimal valid container and validate it
section "Test 2: Create and Validate Minimal Container"

cat > "$TEST_DIR/minimal_container.yaml" << 'EOF'
test_results:
  - test_case_id: 'TEST_MINIMAL_001'
    sequences:
      - sequence_id: 1
        name: "Minimal Test Sequence"
        step_results:
          - Pass:
              step: 1
              description: "Basic test step"
        all_steps_passed: true
    total_steps: 1
    passed_steps: 1
    failed_steps: 0
    not_executed_steps: 0
    overall_pass: true
EOF

log_info "Validating minimal container..."

if "$COMPAT_BIN" validate "$TEST_DIR/minimal_container.yaml" > "$TEST_DIR/minimal_output.txt" 2>&1; then
    pass "Minimal container is valid"
else
    fail "Minimal container validation failed"
    cat "$TEST_DIR/minimal_output.txt"
    exit 1
fi

# Test 3: Create an invalid container and verify it fails validation
section "Test 3: Validate Invalid Container (Should Fail)"

cat > "$TEST_DIR/invalid_container.yaml" << 'EOF'
test_results: []
EOF

log_info "Validating invalid container (expecting failure)..."

if "$COMPAT_BIN" validate "$TEST_DIR/invalid_container.yaml" > "$TEST_DIR/invalid_output.txt" 2>&1; then
    fail "Invalid container should have failed validation"
    exit 1
else
    pass "Invalid container correctly failed validation"
fi

# Verify error message
if grep -q "Missing or empty 'test_results' array" "$TEST_DIR/invalid_output.txt"; then
    pass "Error message is correct"
else
    fail "Expected error message not found"
    cat "$TEST_DIR/invalid_output.txt"
    exit 1
fi

# Test 4: Create a container with all variants
section "Test 4: Validate Container with All Step Variants"

cat > "$TEST_DIR/all_variants_container.yaml" << 'EOF'
title: 'All Variants Test'
project: 'Container Compatibility Test'
test_date: '2024-03-15T14:30:00Z'
test_results:
  - test_case_id: 'TEST_ALL_VARIANTS_001'
    description: 'Test with Pass, Fail, and NotExecuted variants'
    requirement: 'REQ_TEST'
    item: 1
    tc: 1
    sequences:
      - sequence_id: 1
        name: "All Variants Sequence"
        step_results:
          - Pass:
              step: 1
              description: "Passed step"
          - Fail:
              step: 2
              description: "Failed step"
              expected:
                success: true
                result: "0"
                output: "Expected output"
              actual_result: "1"
              actual_output: "Actual output"
              reason: "Exit code mismatch"
          - NotExecuted:
              step: 3
              description: "Not executed step"
        all_steps_passed: false
    total_steps: 3
    passed_steps: 1
    failed_steps: 1
    not_executed_steps: 1
    overall_pass: false
metadata:
  environment: 'Test Environment'
  platform: 'Test Platform'
  executor: 'Test Executor'
  execution_duration: 10.5
  total_test_cases: 1
  passed_test_cases: 0
  failed_test_cases: 1
EOF

log_info "Validating all variants container..."

if "$COMPAT_BIN" validate "$TEST_DIR/all_variants_container.yaml" --verbose > "$TEST_DIR/all_variants_output.txt" 2>&1; then
    pass "All variants container is valid"
else
    fail "All variants container validation failed"
    cat "$TEST_DIR/all_variants_output.txt"
    exit 1
fi

# Test 5: Batch validation
section "Test 5: Batch Validation"

log_info "Running batch validation on test directory..."

if "$COMPAT_BIN" batch "$TEST_DIR" --continue-on-error > "$TEST_DIR/batch_output.txt" 2>&1; then
    pass "Batch validation completed"
else
    # Batch validation may fail if invalid files are present, which is expected
    log_info "Batch validation completed with some failures (expected)"
fi

# Verify batch output mentions multiple files
if grep -q "Found.*container YAML files" "$TEST_DIR/batch_output.txt"; then
    pass "Batch validation found multiple files"
else
    fail "Batch validation did not find files"
    cat "$TEST_DIR/batch_output.txt"
    exit 1
fi

# Test 6: Generate compatibility report
section "Test 6: Generate Compatibility Report"

log_info "Generating compatibility report..."

if "$COMPAT_BIN" report "$TEST_DIR" \
    --output "$TEST_DIR/compatibility_report.md" \
    --format markdown > "$TEST_DIR/report_output.txt" 2>&1; then
    pass "Compatibility report generated"
else
    log_error "Failed to generate compatibility report"
    cat "$TEST_DIR/report_output.txt"
    exit 1
fi

# Verify report was created
if [ -f "$TEST_DIR/compatibility_report.md" ]; then
    pass "Report file exists"
    
    # Verify report contains expected sections
    if grep -q "# Test Plan Documentation Generator Compatibility Report" "$TEST_DIR/compatibility_report.md"; then
        pass "Report has correct title"
    else
        fail "Report missing title"
    fi
    
    if grep -q "## Summary" "$TEST_DIR/compatibility_report.md"; then
        pass "Report has summary section"
    else
        fail "Report missing summary section"
    fi
else
    fail "Report file not created"
    exit 1
fi

# Test 7: JSON output format
section "Test 7: JSON Output Format"

log_info "Testing JSON output format..."

if "$COMPAT_BIN" validate "$TEST_DIR/minimal_container.yaml" --json > "$TEST_DIR/json_output.json" 2>&1; then
    pass "JSON output generated"
    
    # Verify it's valid JSON
    if command -v jq > /dev/null 2>&1; then
        if jq . "$TEST_DIR/json_output.json" > /dev/null 2>&1; then
            pass "JSON output is valid"
        else
            fail "JSON output is invalid"
            cat "$TEST_DIR/json_output.json"
            exit 1
        fi
    else
        log_info "jq not installed, skipping JSON validation"
    fi
else
    fail "JSON output generation failed"
    exit 1
fi

section "Summary"

pass "All E2E tests passed successfully"

info "Test artifacts in: $TEST_DIR"
log_info "  • Validation outputs"
log_info "  • Test containers"
log_info "  • Compatibility report"

section "Complete"

exit 0
