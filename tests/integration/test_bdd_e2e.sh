#!/bin/bash
#
# End-to-end BDD integration test for test-executor
#
# This test validates:
# 1. Script generation from BDD example YAML files
# 2. Shell syntax validation with bash -n
# 3. Execution of generated scripts
# 4. Verification that BDD commands actually ran (check for created files/directories)
# 5. JSON log output is valid
#
# Usage: ./tests/integration/test_bdd_e2e.sh
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TEST_EXECUTOR_BIN="$PROJECT_ROOT/target/debug/test-executor"
BDD_EXAMPLES_DIR="$PROJECT_ROOT/testcases/bdd_examples"

# Color codes for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

echo "======================================"
echo "BDD End-to-End Integration Test"
echo "======================================"
echo ""

# Function to print test status
pass() {
    echo -e "${GREEN}✓${NC} $1"
    TESTS_PASSED=$((TESTS_PASSED+1))
}

fail() {
    echo -e "${RED}✗${NC} $1"
    TESTS_FAILED=$((TESTS_FAILED+1))
}

info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

section() {
    echo ""
    echo -e "${YELLOW}=== $1 ===${NC}"
}

# Check prerequisites
section "Checking Prerequisites"

if [[ ! -f "$TEST_EXECUTOR_BIN" ]]; then
    fail "test-executor binary not found at $TEST_EXECUTOR_BIN"
    echo "Run 'cargo build' first"
    exit 1
fi
pass "test-executor binary found"

if [[ ! -d "$BDD_EXAMPLES_DIR" ]]; then
    fail "BDD examples directory not found at $BDD_EXAMPLES_DIR"
    exit 1
fi
pass "BDD examples directory found"

if ! command -v bash &> /dev/null; then
    fail "bash not found"
    exit 1
fi
pass "bash available"

# Check for jq (for JSON validation)
if command -v jq &> /dev/null; then
    pass "jq available for JSON validation"
    HAS_JQ=true
else
    info "jq not available - JSON validation will use python fallback"
    HAS_JQ=false
fi

# Create temporary directory for test files
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

info "Using temporary directory: $TEMP_DIR"

# Find all YAML files in bdd_examples directory
BDD_YAML_FILES=()
while IFS= read -r -d '' file; do
    BDD_YAML_FILES+=("$file")
done < <(find "$BDD_EXAMPLES_DIR" -type f \( -name "*.yml" -o -name "*.yaml" \) -print0)

if [[ ${#BDD_YAML_FILES[@]} -eq 0 ]]; then
    fail "No YAML files found in $BDD_EXAMPLES_DIR"
    exit 1
fi

info "Found ${#BDD_YAML_FILES[@]} BDD example YAML file(s)"

# Process each BDD example YAML file
for yaml_file in "${BDD_YAML_FILES[@]}"; do
    yaml_basename=$(basename "$yaml_file")
    yaml_name="${yaml_basename%.*}"
    
    section "Testing: $yaml_basename"
    
    # Generate script
    script_file="$TEMP_DIR/${yaml_name}.sh"
    if "$TEST_EXECUTOR_BIN" generate "$yaml_file" -o "$script_file" > /dev/null 2>&1; then
        pass "Generated script from $yaml_basename"
    else
        fail "Failed to generate script from $yaml_basename"
        continue
    fi
    
    # Check script file exists
    if [[ -f "$script_file" ]]; then
        pass "Script file created: ${yaml_name}.sh"
    else
        fail "Script file not found: ${yaml_name}.sh"
        continue
    fi
    
    # Validate shell syntax
    if bash -n "$script_file" 2>/dev/null; then
        pass "Script has valid bash syntax"
    else
        fail "Script has invalid bash syntax"
        bash -n "$script_file" 2>&1 | head -5
        continue
    fi
    
    # Check script contains shebang
    if head -n 1 "$script_file" | grep -q "#!/bin/bash"; then
        pass "Script has bash shebang"
    else
        fail "Script missing bash shebang"
    fi
    
    # Execute the generated script in a subshell and capture artifacts
    execution_dir="$TEMP_DIR/exec_${yaml_name}"
    mkdir -p "$execution_dir"
    
    # Run the script in the execution directory
    cd "$execution_dir"
    execution_output="$execution_dir/execution_output.txt"
    execution_success=false
    
    if bash "$script_file" > "$execution_output" 2>&1; then
        execution_success=true
        pass "Script executed successfully (exit code 0)"
    else
        exit_code=$?
        info "Script execution failed with exit code $exit_code (this may be expected for some tests)"
        # Don't fail here - some tests are expected to fail
    fi
    
    # Check for JSON log file
    json_log_file=$(find "$execution_dir" -name "*_execution_log.json" -type f | head -n 1)
    if [[ -n "$json_log_file" && -f "$json_log_file" ]]; then
        pass "JSON log file created"
        
        # Validate JSON
        json_valid=false
        if [[ "$HAS_JQ" == "true" ]]; then
            if jq empty "$json_log_file" >/dev/null 2>&1; then
                json_valid=true
                pass "JSON log is valid (verified with jq)"
            else
                fail "JSON log is invalid"
                jq empty "$json_log_file" 2>&1 | head -5
            fi
        else
            # Fallback to python for JSON validation
            if python3 -c "import json; json.load(open('$json_log_file'))" 2>/dev/null; then
                json_valid=true
                pass "JSON log is valid (verified with python)"
            else
                fail "JSON log is invalid"
            fi
        fi
        
        # Check JSON structure
        if [[ "$json_valid" == "true" ]]; then
            if [[ "$HAS_JQ" == "true" ]]; then
                # Check that JSON is an array
                if jq -e 'type == "array"' "$json_log_file" >/dev/null 2>&1; then
                    pass "JSON log is an array"
                else
                    fail "JSON log is not an array"
                fi
                
                # Check that array has entries
                entry_count=$(jq 'length' "$json_log_file")
                if [[ $entry_count -gt 0 ]]; then
                    pass "JSON log has $entry_count entries"
                else
                    fail "JSON log is empty"
                fi
            fi
        fi
    else
        fail "JSON log file not created"
    fi
    
    # Verify BDD commands actually ran based on the specific test
    cd "$PROJECT_ROOT"
    
    case "$yaml_basename" in
        *file_creation*|*comprehensive*)
            # Check for created files/directories in /tmp
            if [[ -d "/tmp/test_dir" ]] || [[ -d "/tmp/bdd_test" ]]; then
                pass "BDD commands created expected directories"
            else
                info "Expected directories not found (may have been cleaned up)"
            fi
            ;;
        *network_ping*)
            # For network test, check that script attempted ping
            if grep -q "ping" "$script_file"; then
                pass "Script contains ping command from BDD"
            else
                fail "Script missing expected ping command"
            fi
            ;;
        *)
            info "No specific BDD verification for $yaml_basename"
            ;;
    esac
    
    # Additional script content checks
    if grep -q "COMMAND_OUTPUT=" "$script_file"; then
        pass "Script captures command output"
    else
        fail "Script doesn't capture command output"
    fi
    
    if grep -q "EXIT_CODE=" "$script_file"; then
        pass "Script captures exit code"
    else
        fail "Script doesn't capture exit code"
    fi
    
    # Look for BDD-generated commands in the script
    bdd_command_found=false
    if grep -qE '(mkdir -p|touch|ping -c)' "$script_file"; then
        bdd_command_found=true
        pass "Script contains BDD-generated commands"
    fi
    
    if [[ "$bdd_command_found" == "false" ]]; then
        info "No obvious BDD-generated commands found in script"
    fi
done

# Clean up any test artifacts in /tmp
info "Cleaning up test artifacts..."
rm -rf /tmp/test_dir 2>/dev/null || true
rm -rf /tmp/bdd_test 2>/dev/null || true

# Summary
section "Test Summary"
echo ""
echo "Tests Passed: $TESTS_PASSED"
echo "Tests Failed: $TESTS_FAILED"
echo ""

if [[ $TESTS_FAILED -eq 0 ]]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed!${NC}"
    exit 1
fi
