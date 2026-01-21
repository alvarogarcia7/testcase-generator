#!/bin/bash
#
# E2E Integration Test for Test Run Manager (trm)
#
# This test verifies:
# - trm add creates test runs for multiple test cases
# - Folder structure and timestamped filenames exist
# - trm list displays correct output format
# - No file overwrites when adding multiple runs to same test case
#
# Usage: ./tests/integration/test_run_manager_e2e.sh
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TRM_BINARY="$PROJECT_ROOT/target/debug/trm"
TCM_BINARY="$PROJECT_ROOT/target/debug/tcm"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test state
TESTS_PASSED=0
TESTS_FAILED=0
TEST_DIR=""

# Cleanup function
cleanup() {
    if [[ -n "$TEST_DIR" ]] && [[ -d "$TEST_DIR" ]]; then
        rm -rf "$TEST_DIR"
    fi
}

trap cleanup EXIT

# Helper functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

assert_equals() {
    local expected="$1"
    local actual="$2"
    local message="$3"
    
    if [[ "$expected" == "$actual" ]]; then
        log_info "✓ $message"
        ((TESTS_PASSED++))
        return 0
    else
        log_error "✗ $message"
        log_error "  Expected: $expected"
        log_error "  Actual: $actual"
        ((TESTS_FAILED++))
        return 1
    fi
}

assert_true() {
    local condition="$1"
    local message="$2"
    
    if [[ "$condition" == "true" ]]; then
        log_info "✓ $message"
        ((TESTS_PASSED++))
        return 0
    else
        log_error "✗ $message"
        ((TESTS_FAILED++))
        return 1
    fi
}

assert_file_exists() {
    local file="$1"
    local message="$2"
    
    if [[ -f "$file" ]]; then
        log_info "✓ $message"
        ((TESTS_PASSED++))
        return 0
    else
        log_error "✗ $message: File not found: $file"
        ((TESTS_FAILED++))
        return 1
    fi
}

assert_dir_exists() {
    local dir="$1"
    local message="$2"
    
    if [[ -d "$dir" ]]; then
        log_info "✓ $message"
        ((TESTS_PASSED++))
        return 0
    else
        log_error "✗ $message: Directory not found: $dir"
        ((TESTS_FAILED++))
        return 1
    fi
}

assert_contains() {
    local haystack="$1"
    local needle="$2"
    local message="$3"
    
    if [[ "$haystack" == *"$needle"* ]]; then
        log_info "✓ $message"
        ((TESTS_PASSED++))
        return 0
    else
        log_error "✗ $message"
        log_error "  Expected to contain: $needle"
        log_error "  In: $haystack"
        ((TESTS_FAILED++))
        return 1
    fi
}

# Main test function
main() {
    echo "=========================================="
    echo "Test Run Manager E2E Integration Test"
    echo "=========================================="
    echo ""
    
    # Check binaries exist
    if [[ ! -f "$TRM_BINARY" ]]; then
        log_error "TRM binary not found at $TRM_BINARY"
        exit 1
    fi
    log_info "TRM binary found"
    
    if [[ ! -f "$TCM_BINARY" ]]; then
        log_error "TCM binary not found at $TCM_BINARY"
        exit 1
    fi
    log_info "TCM binary found"
    
    # Create temporary test directory
    TEST_DIR=$(mktemp -d)
    log_info "Created test directory: $TEST_DIR"
    
    # Setup test data
    setup_test_data
    
    # Run tests
    echo ""
    echo "=========================================="
    echo "Running Tests"
    echo "=========================================="
    echo ""
    
    test_create_test_runs
    test_folder_structure
    test_timestamped_filenames
    test_list_output_format
    test_no_overwrites
    
    # Print summary
    echo ""
    echo "=========================================="
    echo "Test Summary"
    echo "=========================================="
    echo "Passed: $TESTS_PASSED"
    echo "Failed: $TESTS_FAILED"
    echo ""
    
    if [[ $TESTS_FAILED -gt 0 ]]; then
        echo "Some tests FAILED ✗"
        exit 1
    else
        echo "All tests PASSED ✓"
        exit 0
    fi
}

# Setup test data - create sample test cases
setup_test_data() {
    log_info "Setting up test data..."
    
    # Create testcases directory
    mkdir -p "$TEST_DIR/testcases"
    
    # Create test case 1
    cat > "$TEST_DIR/testcases/TC001.yaml" << 'EOF'
requirement: REQ001
item: 1
tc: 1
id: TC001
description: First test case
initial_conditions: {}
test_sequences:
- id: 1
  name: Basic sequence
  initial_conditions: {}
  steps:
  - step: 1
    description: Test step 1
    command: echo test
    expected:
      result: success
      output: test
EOF
    
    # Create test case 2
    cat > "$TEST_DIR/testcases/TC002.yaml" << 'EOF'
requirement: REQ001
item: 1
tc: 2
id: TC002
description: Second test case
initial_conditions: {}
test_sequences:
- id: 1
  name: Basic sequence
  initial_conditions: {}
  steps:
  - step: 1
    description: Test step 1
    command: echo test
    expected:
      result: success
      output: test
EOF
    
    # Create test case 3
    cat > "$TEST_DIR/testcases/TC003.yaml" << 'EOF'
requirement: REQ001
item: 1
tc: 3
id: TC003
description: Third test case
initial_conditions: {}
test_sequences:
- id: 1
  name: Basic sequence
  initial_conditions: {}
  steps:
  - step: 1
    description: Test step 1
    command: echo test
    expected:
      result: success
      output: test
EOF
    
    log_info "Test data created successfully"
}

# Test: Create test runs using trm add
test_create_test_runs() {
    echo ""
    echo "--- Test: Create Test Runs ---"
    
    # Add test run for TC001 (Pass)
    log_info "Adding Pass run for TC001..."
    output=$(echo -e "TC001\nPass\n100\nTest execution log\n" | "$TRM_BINARY" --path "$TEST_DIR/testcases" add 2>&1)
    assert_contains "$output" "Test run saved" "TC001 Pass run created"
    
    # Add test run for TC002 (Fail)
    log_info "Adding Fail run for TC002..."
    output=$(echo -e "TC002\nFail\n200\nTest execution log\nError occurred\n" | "$TRM_BINARY" --path "$TEST_DIR/testcases" add 2>&1)
    assert_contains "$output" "Test run saved" "TC002 Fail run created"
    
    # Add test run for TC003 (Skip)
    log_info "Adding Skip run for TC003..."
    output=$(echo -e "TC003\nSkip\n0\nTest was skipped\n" | "$TRM_BINARY" --path "$TEST_DIR/testcases" add 2>&1)
    assert_contains "$output" "Test run saved" "TC003 Skip run created"
}

# Test: Verify folder structure
test_folder_structure() {
    echo ""
    echo "--- Test: Folder Structure ---"
    
    # Check test-runs directory exists
    assert_dir_exists "$TEST_DIR/testcases/test-runs" "test-runs directory exists"
    
    # Check TC001 directory structure
    assert_dir_exists "$TEST_DIR/testcases/test-runs/TC001" "TC001 directory exists"
    assert_dir_exists "$TEST_DIR/testcases/test-runs/TC001/runs" "TC001/runs directory exists"
    
    # Check TC002 directory structure
    assert_dir_exists "$TEST_DIR/testcases/test-runs/TC002" "TC002 directory exists"
    assert_dir_exists "$TEST_DIR/testcases/test-runs/TC002/runs" "TC002/runs directory exists"
    
    # Check TC003 directory structure
    assert_dir_exists "$TEST_DIR/testcases/test-runs/TC003" "TC003 directory exists"
    assert_dir_exists "$TEST_DIR/testcases/test-runs/TC003/runs" "TC003/runs directory exists"
}

# Test: Verify timestamped filenames
test_timestamped_filenames() {
    echo ""
    echo "--- Test: Timestamped Filenames ---"
    
    # Check TC001 has timestamped YAML file
    local tc001_files=$(ls "$TEST_DIR/testcases/test-runs/TC001/runs/"*.yaml 2>/dev/null | wc -l)
    assert_equals "1" "$tc001_files" "TC001 has exactly 1 timestamped YAML file"
    
    # Verify filename format (RFC3339 timestamp)
    local tc001_file=$(ls "$TEST_DIR/testcases/test-runs/TC001/runs/"*.yaml 2>/dev/null | head -n1)
    if [[ -n "$tc001_file" ]]; then
        local filename=$(basename "$tc001_file")
        # RFC3339 format: YYYY-MM-DDTHH:MM:SS+00:00.yaml or similar
        if [[ "$filename" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2} ]]; then
            log_info "✓ Filename follows RFC3339 timestamp format: $filename"
            ((TESTS_PASSED++))
        else
            log_error "✗ Filename does not follow RFC3339 timestamp format: $filename"
            ((TESTS_FAILED++))
        fi
    fi
    
    # Check TC002 has timestamped YAML file
    local tc002_files=$(ls "$TEST_DIR/testcases/test-runs/TC002/runs/"*.yaml 2>/dev/null | wc -l)
    assert_equals "1" "$tc002_files" "TC002 has exactly 1 timestamped YAML file"
    
    # Check TC003 has timestamped YAML file
    local tc003_files=$(ls "$TEST_DIR/testcases/test-runs/TC003/runs/"*.yaml 2>/dev/null | wc -l)
    assert_equals "1" "$tc003_files" "TC003 has exactly 1 timestamped YAML file"
}

# Test: Verify trm list output format
test_list_output_format() {
    echo ""
    echo "--- Test: List Output Format ---"
    
    # Run trm list
    local list_output=$("$TRM_BINARY" --path "$TEST_DIR/testcases" list 2>&1)
    
    # Check header is present
    assert_contains "$list_output" "Test Case ID" "List output contains 'Test Case ID' header"
    assert_contains "$list_output" "Run Count" "List output contains 'Run Count' header"
    assert_contains "$list_output" "Latest Run" "List output contains 'Latest Run' header"
    assert_contains "$list_output" "Status Summary" "List output contains 'Status Summary' header"
    
    # Check TC001 is listed
    assert_contains "$list_output" "TC001" "List output contains TC001"
    
    # Check TC002 is listed
    assert_contains "$list_output" "TC002" "List output contains TC002"
    
    # Check TC003 is listed
    assert_contains "$list_output" "TC003" "List output contains TC003"
    
    # Check status summary format (P:X F:Y S:Z)
    assert_contains "$list_output" "P:" "List output contains Pass count indicator"
    assert_contains "$list_output" "F:" "List output contains Fail count indicator"
    assert_contains "$list_output" "S:" "List output contains Skip count indicator"
    
    # Verify run counts are "1" for each test case
    if echo "$list_output" | grep -q "TC001.*1.*P:1 F:0 S:0"; then
        log_info "✓ TC001 shows correct run count (1) and status (P:1 F:0 S:0)"
        ((TESTS_PASSED++))
    else
        log_error "✗ TC001 does not show correct run count or status"
        ((TESTS_FAILED++))
    fi
    
    if echo "$list_output" | grep -q "TC002.*1.*P:0 F:1 S:0"; then
        log_info "✓ TC002 shows correct run count (1) and status (P:0 F:1 S:0)"
        ((TESTS_PASSED++))
    else
        log_error "✗ TC002 does not show correct run count or status"
        ((TESTS_FAILED++))
    fi
    
    if echo "$list_output" | grep -q "TC003.*1.*P:0 F:0 S:1"; then
        log_info "✓ TC003 shows correct run count (1) and status (P:0 F:0 S:1)"
        ((TESTS_PASSED++))
    else
        log_error "✗ TC003 does not show correct run count or status"
        ((TESTS_FAILED++))
    fi
}

# Test: Verify no overwrites when adding multiple runs
test_no_overwrites() {
    echo ""
    echo "--- Test: No Overwrites with Multiple Runs ---"
    
    # Add second run for TC001
    log_info "Adding second Pass run for TC001..."
    sleep 1  # Ensure different timestamp
    output=$(echo -e "TC001\nPass\n150\nSecond test execution\n" | "$TRM_BINARY" --path "$TEST_DIR/testcases" add 2>&1)
    assert_contains "$output" "Test run saved" "TC001 second run created"
    
    # Add third run for TC001
    log_info "Adding third Fail run for TC001..."
    sleep 1  # Ensure different timestamp
    output=$(echo -e "TC001\nFail\n200\nThird test execution\nSome error\n" | "$TRM_BINARY" --path "$TEST_DIR/testcases" add 2>&1)
    assert_contains "$output" "Test run saved" "TC001 third run created"
    
    # Verify TC001 now has 3 files
    local tc001_files=$(ls "$TEST_DIR/testcases/test-runs/TC001/runs/"*.yaml 2>/dev/null | wc -l)
    assert_equals "3" "$tc001_files" "TC001 has exactly 3 timestamped YAML files (no overwrites)"
    
    # Verify all files have different names (timestamps)
    local unique_files=$(ls "$TEST_DIR/testcases/test-runs/TC001/runs/"*.yaml 2>/dev/null | sort -u | wc -l)
    assert_equals "3" "$unique_files" "All 3 TC001 files have unique timestamps"
    
    # Verify trm list shows updated counts
    local list_output=$("$TRM_BINARY" --path "$TEST_DIR/testcases" list 2>&1)
    
    if echo "$list_output" | grep -q "TC001.*3"; then
        log_info "✓ TC001 shows correct updated run count (3)"
        ((TESTS_PASSED++))
    else
        log_error "✗ TC001 does not show correct updated run count"
        ((TESTS_FAILED++))
    fi
    
    # Verify status summary reflects all 3 runs (2 Pass, 1 Fail, 0 Skip)
    if echo "$list_output" | grep -q "TC001.*3.*P:2 F:1 S:0"; then
        log_info "✓ TC001 shows correct status summary (P:2 F:1 S:0)"
        ((TESTS_PASSED++))
    else
        log_error "✗ TC001 does not show correct status summary"
        ((TESTS_FAILED++))
    fi
    
    # Add second run for TC002
    log_info "Adding second Pass run for TC002..."
    sleep 1  # Ensure different timestamp
    output=$(echo -e "TC002\nPass\n100\nSecond test execution\n" | "$TRM_BINARY" --path "$TEST_DIR/testcases" add 2>&1)
    assert_contains "$output" "Test run saved" "TC002 second run created"
    
    # Verify TC002 now has 2 files
    local tc002_files=$(ls "$TEST_DIR/testcases/test-runs/TC002/runs/"*.yaml 2>/dev/null | wc -l)
    assert_equals "2" "$tc002_files" "TC002 has exactly 2 timestamped YAML files (no overwrites)"
    
    # Verify final list output
    list_output=$("$TRM_BINARY" --path "$TEST_DIR/testcases" list 2>&1)
    
    if echo "$list_output" | grep -q "TC002.*2.*P:1 F:1 S:0"; then
        log_info "✓ TC002 shows correct final status (2 runs: P:1 F:1 S:0)"
        ((TESTS_PASSED++))
    else
        log_error "✗ TC002 does not show correct final status"
        ((TESTS_FAILED++))
    fi
}

# Run main
main
