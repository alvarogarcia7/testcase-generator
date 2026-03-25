#!/usr/bin/env bash
#
# validate_stage2_scripts.sh - Validates Stage 2 (shell script generation)
#
# DESCRIPTION:
#   This script validates Stage 2 by generating bash scripts from test case YAML files
#   and performing comprehensive validation including:
#   - Running test-executor generate for each test case YAML
#   - Verifying bash script was generated with valid syntax using bash -n
#   - Checking script contains proper shebang (#!/usr/bin/env bash or #!/bin/bash)
#   - Verifying set -e is present for fail-fast behavior
#   - Confirming JSON logging code is present if --json-log flag used
#   - Validating hook execution code for test cases with hooks
#   - Checking variable substitution code for tests with variables
#   - Verifying manual step prompts for tests with manual steps
#   - Running shellcheck on generated scripts (warnings allowed, errors fail)
#   - Generating validation report showing which scripts passed/failed
#
# USAGE:
#   ./test-acceptance/validate_stage2_scripts.sh [OPTIONS]
#
# OPTIONS:
#   -v, --verbose         Enable verbose output
#   -h, --help            Show this help message
#   --skip-shellcheck     Skip shellcheck validation
#   --json-log            Test JSON logging generation
#
# EXIT CODES:
#   0 - All validations passed
#   1 - One or more validations failed
#
# OUTPUT:
#   Generates a detailed validation report with:
#   - Pass/fail counts for each validation category
#   - Specific error messages for each failed file
#   - Summary statistics
#

set -euo pipefail

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source logger library
source "$REPO_ROOT/scripts/lib/logger.sh" || exit 1
source "$REPO_ROOT/scripts/lib/find-binary.sh" || exit 1

# Configuration
TEST_CASES_DIR="$SCRIPT_DIR/test_cases"
SCRIPTS_DIR="$SCRIPT_DIR/scripts"
VERBOSE=0
SKIP_SHELLCHECK=0
TEST_JSON_LOG=0

# Validation counters
TOTAL_FILES=0
GENERATION_PASSED=0
GENERATION_FAILED=0
SYNTAX_PASSED=0
SYNTAX_FAILED=0
SHEBANG_PASSED=0
SHEBANG_FAILED=0
SET_E_PASSED=0
SET_E_FAILED=0
JSON_LOG_PASSED=0
JSON_LOG_FAILED=0
HOOKS_PASSED=0
HOOKS_FAILED=0
VARIABLES_PASSED=0
VARIABLES_FAILED=0
MANUAL_PASSED=0
MANUAL_FAILED=0
SHELLCHECK_PASSED=0
SHELLCHECK_FAILED=0
SHELLCHECK_SKIPPED=0

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

Validates Stage 2 (shell script generation) for acceptance testing.

OPTIONS:
    -v, --verbose         Enable verbose output
    -h, --help            Show this help message
    --skip-shellcheck     Skip shellcheck validation
    --json-log            Test JSON logging generation

DESCRIPTION:
    Validates shell script generation from test case YAML files by:
    - Running test-executor generate for each test case YAML
    - Verifying bash script was generated with valid syntax (bash -n)
    - Checking script contains proper shebang (#!/usr/bin/env bash or #!/bin/bash)
    - Verifying set -e is present for fail-fast behavior
    - Confirming JSON logging code if --json-log used
    - Validating hook execution code for test cases with hooks
    - Checking variable substitution code for tests with variables
    - Verifying manual step prompts for tests with manual steps
    - Running shellcheck on generated scripts (warnings allowed, errors fail)

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
            --skip-shellcheck)
                SKIP_SHELLCHECK=1
                shift
                ;;
            --json-log)
                TEST_JSON_LOG=1
                shift
                ;;
            *)
                log_error "Unknown option: $1"
                usage
                ;;
        esac
    done
}

# Find test-executor binary
find_test_executor() {
    local test_executor
    test_executor=$(find_binary "test-executor" "TEST_EXECUTOR_BIN")
    
    if [[ -z "$test_executor" ]]; then
        log_error "test-executor binary not found"
        log_error "Please build it with: cargo build --bin test-executor"
        exit 1
    fi
    
    echo "$test_executor"
}

# Validate test cases directory exists
validate_test_cases_dir() {
    if [[ ! -d "$TEST_CASES_DIR" ]]; then
        log_error "Test cases directory not found: $TEST_CASES_DIR"
        exit 1
    fi
    log_verbose "Using test cases directory: $TEST_CASES_DIR"
}

# Ensure scripts directory exists
ensure_scripts_dir() {
    if [[ ! -d "$SCRIPTS_DIR" ]]; then
        mkdir -p "$SCRIPTS_DIR"
        log_verbose "Created scripts directory: $SCRIPTS_DIR"
    fi
}

# Find all YAML files in test cases directory
find_yaml_files() {
    find "$TEST_CASES_DIR" -type f \( -name "*.yaml" -o -name "*.yml" \) | sort
}

# Check if YAML has hooks
yaml_has_hooks() {
    local yaml_file="$1"
    
    local python_cmd
    python_cmd=$(find_python)
    
    $python_cmd -c "
import yaml
import sys
try:
    with open('$yaml_file', 'r') as f:
        data = yaml.safe_load(f)
        if 'hooks' in data:
            sys.exit(0)
        else:
            sys.exit(1)
except Exception:
    sys.exit(1)
" 2>/dev/null
}

# Check if YAML has variables (capture_vars or hydration_vars)
yaml_has_variables() {
    local yaml_file="$1"
    
    local python_cmd
    python_cmd=$(find_python)
    
    $python_cmd -c "
import yaml
import sys
try:
    with open('$yaml_file', 'r') as f:
        data = yaml.safe_load(f)
        
        # Check for hydration_vars
        if 'hydration_vars' in data:
            sys.exit(0)
        
        # Check for capture_vars in any step
        sequences = data.get('test_sequences', [])
        for seq in sequences:
            steps = seq.get('steps', [])
            for step in steps:
                if 'capture_vars' in step or 'capture' in step:
                    sys.exit(0)
        
        sys.exit(1)
except Exception:
    sys.exit(1)
" 2>/dev/null
}

# Check if YAML has manual steps
yaml_has_manual_steps() {
    local yaml_file="$1"
    
    local python_cmd
    python_cmd=$(find_python)
    
    $python_cmd -c "
import yaml
import sys
try:
    with open('$yaml_file', 'r') as f:
        data = yaml.safe_load(f)
        sequences = data.get('test_sequences', [])
        for seq in sequences:
            steps = seq.get('steps', [])
            for step in steps:
                if step.get('manual') == True:
                    sys.exit(0)
        sys.exit(1)
except Exception:
    sys.exit(1)
" 2>/dev/null
}

# Generate script from YAML
generate_script() {
    local yaml_file="$1"
    local output_file="$2"
    local test_executor="$3"
    local json_log_flag="${4:-}"
    
    local cmd="$test_executor generate"
    if [[ -n "$json_log_flag" ]]; then
        cmd="$cmd --json-log"
    fi
    cmd="$cmd --output '$output_file' '$yaml_file'"
    
    if eval "$cmd" >/dev/null 2>&1; then
        return 0
    else
        return 1
    fi
}

# Validate bash syntax
validate_bash_syntax() {
    local script_file="$1"
    
    if bash -n "$script_file" >/dev/null 2>&1; then
        return 0
    else
        return 1
    fi
}

# Check for shebang
check_shebang() {
    local script_file="$1"
    
    local first_line
    first_line=$(head -n 1 "$script_file")
    
    if [[ "$first_line" =~ ^#!/usr/bin/env\ bash$ ]] || [[ "$first_line" =~ ^#!/bin/bash$ ]]; then
        return 0
    else
        return 1
    fi
}

# Check for set -e
check_set_e() {
    local script_file="$1"
    
    # Check for set -e, set -euo, set -eu, etc.
    if grep -q '^set -[eou]*e[eou]*' "$script_file"; then
        return 0
    else
        return 1
    fi
}

# Check for JSON logging code
check_json_log_code() {
    local script_file="$1"
    
    # Look for JSON_LOG variable and JSON output
    if grep -q 'JSON_LOG=' "$script_file" && grep -q 'execution_log\.json' "$script_file"; then
        return 0
    else
        return 1
    fi
}

# Check for hook execution code
check_hook_code() {
    local script_file="$1"
    
    # Look for hook-related comments or execution
    if grep -q -i 'hook' "$script_file"; then
        return 0
    else
        return 1
    fi
}

# Check for variable substitution code
check_variable_code() {
    local script_file="$1"
    
    # Look for variable capture or substitution patterns
    if grep -q 'CAPTURED_VAR_NAMES' "$script_file" || grep -q 'capture_vars' "$script_file" || grep -q 'SUBSTITUTED_COMMAND' "$script_file"; then
        return 0
    else
        return 1
    fi
}

# Check for manual step prompts
check_manual_prompts() {
    local script_file="$1"
    
    # Look for read_true_false or read_verification functions
    if grep -q 'read_true_false\|read_verification' "$script_file"; then
        return 0
    else
        return 1
    fi
}

# Run shellcheck on script
run_shellcheck() {
    local script_file="$1"
    
    # Check if shellcheck is available
    if ! command -v shellcheck >/dev/null 2>&1; then
        return 2  # Skip if not available
    fi
    
    # Run shellcheck, only fail on errors (not warnings)
    local output
    output=$(shellcheck -S error "$script_file" 2>&1)
    local result=$?
    
    if [[ $result -eq 0 ]]; then
        return 0
    else
        return 1
    fi
}

# Validate a single YAML file
validate_yaml_file() {
    local yaml_file="$1"
    local test_executor="$2"
    local file_basename
    file_basename=$(basename "$yaml_file")
    local file_id="${file_basename%.yaml}"
    local file_id="${file_id%.yml}"
    
    log_verbose "Validating: $file_basename"
    
    # Determine output script filename
    local script_file="$SCRIPTS_DIR/${file_id}.sh"
    
    # Determine if we should test JSON logging
    local json_log_flag=""
    if [[ $TEST_JSON_LOG -eq 1 ]]; then
        json_log_flag="--json-log"
    fi
    
    # 1. Generate script
    if generate_script "$yaml_file" "$script_file" "$test_executor" "$json_log_flag"; then
        ((GENERATION_PASSED++))
        log_debug "  ✓ Script generation passed"
    else
        ((GENERATION_FAILED++))
        append_error "$file_basename: Script generation failed"
        log_debug "  ✗ Script generation failed"
        return  # Skip remaining validations if generation failed
    fi
    
    # Verify the script file was created
    if [[ ! -f "$script_file" ]]; then
        ((GENERATION_FAILED++))
        append_error "$file_basename: Script file not created at $script_file"
        log_debug "  ✗ Script file not created"
        return
    fi
    
    # 2. Validate bash syntax
    if validate_bash_syntax "$script_file"; then
        ((SYNTAX_PASSED++))
        log_debug "  ✓ Bash syntax validation passed"
    else
        ((SYNTAX_FAILED++))
        append_error "$file_basename: Bash syntax validation failed"
        log_debug "  ✗ Bash syntax validation failed"
    fi
    
    # 3. Check shebang
    if check_shebang "$script_file"; then
        ((SHEBANG_PASSED++))
        log_debug "  ✓ Shebang check passed"
    else
        ((SHEBANG_FAILED++))
        append_error "$file_basename: Missing or invalid shebang"
        log_debug "  ✗ Shebang check failed"
    fi
    
    # 4. Check set -e
    if check_set_e "$script_file"; then
        ((SET_E_PASSED++))
        log_debug "  ✓ set -e check passed"
    else
        ((SET_E_FAILED++))
        append_error "$file_basename: Missing set -e for fail-fast behavior"
        log_debug "  ✗ set -e check failed"
    fi
    
    # 5. Check JSON logging (if --json-log was used)
    if [[ $TEST_JSON_LOG -eq 1 ]]; then
        if check_json_log_code "$script_file"; then
            ((JSON_LOG_PASSED++))
            log_debug "  ✓ JSON logging code check passed"
        else
            ((JSON_LOG_FAILED++))
            append_error "$file_basename: Missing JSON logging code"
            log_debug "  ✗ JSON logging code check failed"
        fi
    fi
    
    # 6. Check hooks (if test case has hooks)
    if yaml_has_hooks "$yaml_file"; then
        if check_hook_code "$script_file"; then
            ((HOOKS_PASSED++))
            log_debug "  ✓ Hook execution code check passed"
        else
            ((HOOKS_FAILED++))
            append_error "$file_basename: Missing hook execution code"
            log_debug "  ✗ Hook execution code check failed"
        fi
    fi
    
    # 7. Check variables (if test case has variables)
    if yaml_has_variables "$yaml_file"; then
        if check_variable_code "$script_file"; then
            ((VARIABLES_PASSED++))
            log_debug "  ✓ Variable substitution code check passed"
        else
            ((VARIABLES_FAILED++))
            append_error "$file_basename: Missing variable substitution code"
            log_debug "  ✗ Variable substitution code check failed"
        fi
    fi
    
    # 8. Check manual steps (if test case has manual steps)
    if yaml_has_manual_steps "$yaml_file"; then
        if check_manual_prompts "$script_file"; then
            ((MANUAL_PASSED++))
            log_debug "  ✓ Manual step prompts check passed"
        else
            ((MANUAL_FAILED++))
            append_error "$file_basename: Missing manual step prompts"
            log_debug "  ✗ Manual step prompts check failed"
        fi
    fi
    
    # 9. Run shellcheck (if not skipped)
    if [[ $SKIP_SHELLCHECK -eq 0 ]]; then
        local shellcheck_result
        run_shellcheck "$script_file"
        shellcheck_result=$?
        
        if [[ $shellcheck_result -eq 0 ]]; then
            ((SHELLCHECK_PASSED++))
            log_debug "  ✓ Shellcheck validation passed"
        elif [[ $shellcheck_result -eq 2 ]]; then
            ((SHELLCHECK_SKIPPED++))
            log_debug "  ⊘ Shellcheck skipped (not available)"
        else
            ((SHELLCHECK_FAILED++))
            append_error "$file_basename: Shellcheck validation failed"
            log_debug "  ✗ Shellcheck validation failed"
        fi
    else
        ((SHELLCHECK_SKIPPED++))
        log_debug "  ⊘ Shellcheck skipped (--skip-shellcheck)"
    fi
}

# Print validation report
print_report() {
    section "Stage 2 Script Generation Validation Report"
    
    echo ""
    log_info "Total files validated: $TOTAL_FILES"
    echo ""
    
    log_info "Script Generation:"
    log_info "  Passed: $GENERATION_PASSED"
    log_info "  Failed: $GENERATION_FAILED"
    echo ""
    
    log_info "Bash Syntax Validation:"
    log_info "  Passed: $SYNTAX_PASSED"
    log_info "  Failed: $SYNTAX_FAILED"
    echo ""
    
    log_info "Shebang Check:"
    log_info "  Passed: $SHEBANG_PASSED"
    log_info "  Failed: $SHEBANG_FAILED"
    echo ""
    
    log_info "set -e Check:"
    log_info "  Passed: $SET_E_PASSED"
    log_info "  Failed: $SET_E_FAILED"
    echo ""
    
    if [[ $TEST_JSON_LOG -eq 1 ]]; then
        log_info "JSON Logging Code:"
        log_info "  Passed: $JSON_LOG_PASSED"
        log_info "  Failed: $JSON_LOG_FAILED"
        echo ""
    fi
    
    if [[ $((HOOKS_PASSED + HOOKS_FAILED)) -gt 0 ]]; then
        log_info "Hook Execution Code:"
        log_info "  Passed: $HOOKS_PASSED"
        log_info "  Failed: $HOOKS_FAILED"
        echo ""
    fi
    
    if [[ $((VARIABLES_PASSED + VARIABLES_FAILED)) -gt 0 ]]; then
        log_info "Variable Substitution Code:"
        log_info "  Passed: $VARIABLES_PASSED"
        log_info "  Failed: $VARIABLES_FAILED"
        echo ""
    fi
    
    if [[ $((MANUAL_PASSED + MANUAL_FAILED)) -gt 0 ]]; then
        log_info "Manual Step Prompts:"
        log_info "  Passed: $MANUAL_PASSED"
        log_info "  Failed: $MANUAL_FAILED"
        echo ""
    fi
    
    if [[ $SKIP_SHELLCHECK -eq 0 ]]; then
        log_info "Shellcheck Validation:"
        log_info "  Passed: $SHELLCHECK_PASSED"
        log_info "  Failed: $SHELLCHECK_FAILED"
        log_info "  Skipped: $SHELLCHECK_SKIPPED"
        echo ""
    fi
    
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
    
    local total_failed=$((GENERATION_FAILED + SYNTAX_FAILED + SHEBANG_FAILED + SET_E_FAILED + JSON_LOG_FAILED + HOOKS_FAILED + VARIABLES_FAILED + MANUAL_FAILED + SHELLCHECK_FAILED))
    
    if [[ $total_failed -eq 0 ]]; then
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
    
    section "Stage 2 Script Generation Validation"
    
    log_info "Validating shell script generation from test case YAML files..."
    echo ""
    
    # Validate prerequisites
    validate_test_cases_dir
    ensure_scripts_dir
    
    # Find test-executor binary
    local test_executor
    test_executor=$(find_test_executor)
    log_verbose "Using test-executor: $test_executor"
    
    # Find all YAML files (bash 3.2 compatible)
    local yaml_files=()
    while IFS= read -r file; do
        yaml_files+=("$file")
    done < <(find_yaml_files)
    
    if [[ ${#yaml_files[@]} -eq 0 ]]; then
        log_warning "No YAML files found in $TEST_CASES_DIR"
        exit 0
    fi
    
    TOTAL_FILES=${#yaml_files[@]}
    log_info "Found $TOTAL_FILES YAML file(s) to validate"
    echo ""
    
    # Validate each file
    for yaml_file in "${yaml_files[@]}"; do
        validate_yaml_file "$yaml_file" "$test_executor"
    done
    
    echo ""
    
    # Print report and exit with appropriate code
    if print_report; then
        exit 0
    else
        exit 1
    fi
}

# Run main function
main "$@"
