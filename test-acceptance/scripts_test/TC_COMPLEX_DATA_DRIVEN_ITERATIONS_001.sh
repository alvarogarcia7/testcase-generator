#!/bin/bash
set -euo pipefail

# shellcheck shell=bash
# Bash helper functions for user prompts
# Prompts user for Y/n input with proper validation
# Returns: 0 for yes, 1 for no (standard bash convention)
# Supports both interactive and non-interactive modes with TTY detection
read_true_false() {
    local prompt="$1"
    local default="${2:-y}"

    # Check if running in non-interactive mode
    if [[ "${DEBIAN_FRONTEND:-}" == 'noninteractive' ]] || ! [ -t 0 ]; then
        # Non-interactive mode: return default
        if [[ "$default" =~ ^[Yy]$ ]]; then
            return 0
        else
            return 1
        fi
    fi

    # Interactive mode: prompt user
    while true; do
        if [[ "$default" =~ ^[Yy]$ ]]; then
            read -p "$prompt [Y/n]: " response
        else
            read -p "$prompt [y/N]: " response
        fi

        # Empty response uses default
        if [[ -z "$response" ]]; then
            response="$default"
        fi

        # Validate response
        case "$response" in
            [Yy]|[Yy][Ee][Ss])
                return 0
                ;;
            [Nn]|[Nn][Oo])
                return 1
                ;;
            *)
                echo "Invalid response. Please enter Y or n." >&2
                ;;
        esac
    done
}

# Prompts user for verification with Y/n input
# Returns: 0 for yes, 1 for no (standard bash convention)
# Supports both interactive and non-interactive modes with TTY detection
read_verification() {
    local prompt="$1"
    local default="${2:-y}"
    
    # Check if running in non-interactive mode
    if [[ "${DEBIAN_FRONTEND:-}" == 'noninteractive' ]] || ! [ -t 0 ]; then
        # Non-interactive mode: return default
        if [[ "$default" =~ ^[Yy]$ ]]; then
            return 0
        else
            return 1
        fi
    fi
    
    # Interactive mode: prompt user
    while true; do
        if [[ "$default" =~ ^[Yy]$ ]]; then
            read -p "$prompt [Y/n]: " response
        else
            read -p "$prompt [y/N]: " response
        fi
        
        # Empty response uses default
        if [[ -z "$response" ]]; then
            response="$default"
        fi
        
        # Validate response
        case "$response" in
            [Yy]|[Yy][Ee][Ss])
                return 0
                ;;
            [Nn]|[Nn][Oo])
                return 1
                ;;
            *)
                echo "Invalid response. Please enter Y or n." >&2
                ;;
        esac
    done
}

# Test Case: TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001
# Description: Data-driven test with multiple iterations and aggregated results - demonstrates iterative testing patterns

JSON_LOG="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_execution_log.json"
TIMESTAMP=$(date +"%Y-%m-%dT%H:%M:%S")

# Initialize variable storage for captured variables (bash 3.2+ compatible)
STEP_VAR_NAMES=""

# Trap to ensure JSON file is closed properly on exit
cleanup() {
    if [ -f "$JSON_LOG" ]; then
        # Check if JSON_LOG ends with '[' or ','
        LAST_CHAR=$(tail -c 2 "$JSON_LOG" | head -c 1)
        if [ "$LAST_CHAR" != "]" ]; then
            echo '' >> "$JSON_LOG"
            echo ']' >> "$JSON_LOG"
        fi
        # Validate JSON
        if command -v jq >/dev/null 2>&1; then
            if ! jq empty "$JSON_LOG" >/dev/null 2>&1; then
                echo "500 - Internal Script Error: Generated JSON is not valid" >&2
                exit 1
            fi
        fi
    fi
}
trap cleanup EXIT

echo '[' > "$JSON_LOG"
FIRST_ENTRY=true

# Execute setup_test hook
set +e
if [ -f "test-acceptance/scripts/hooks/setup_test_workspace.sh" ]; then
    source "test-acceptance/scripts/hooks/setup_test_workspace.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/setup_test_workspace.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Error: setup_test hook failed with exit code $HOOK_EXIT_CODE" >&2
    exit $HOOK_EXIT_CODE
fi

# General Initial Conditions
# testing: Multiple iterations can be executed
# testing: Results can be aggregated and analyzed
# system: Test iteration framework is available
# system: Data aggregation utilities are present

# Initial Conditions
# filesystem: Test data directory can be created
# testing: Iteration counter can be maintained
# testing: Results can be accumulated across iterations

# Test Sequence 1: Test Data Preparation
# Prepare test data sets for iterative testing
TEST_SEQUENCE_ID=1
TEST_SEQUENCE_NAME='Test Data Preparation'
export TEST_SEQUENCE_ID TEST_SEQUENCE_NAME
# Execute before_sequence hook
set +e
if [ -f "test-acceptance/scripts/hooks/before_sequence_log.sh" ]; then
    source "test-acceptance/scripts/hooks/before_sequence_log.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/before_sequence_log.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: before_sequence hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

# Sequence Initial Conditions
# testing: Test data directory is available

# Step 1: Create test data directory structure
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-1_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="mkdir -p \${TEST_DATA_DIR}/{input,output,results} && echo 'Created: '\${TEST_DATA_DIR}"
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
COMMAND_OUTPUT=$({ eval "$SUBSTITUTED_COMMAND"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_data_dir=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(Created: (/tmp/[^[[:space:]]]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " data_dir "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES data_dir"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="grep -q 'Created:' <<< \"\$COMMAND_OUTPUT\""
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

# General verifications
GENERAL_VERIFY_PASS_verify_data_dir_captured=false
EXPR="[[ -n \"\$data_dir\" ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    GENERAL_VERIFY_PASS_verify_data_dir_captured=true
fi
GENERAL_VERIFY_PASS_verify_data_dir_exists=false
EXPR="[[ -d \"\$data_dir\" ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    GENERAL_VERIFY_PASS_verify_data_dir_exists=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_data_dir_captured" = true ] && [ "$GENERAL_VERIFY_PASS_verify_data_dir_exists" = true ]; then
    echo "[PASS] Step 1: Create test data directory structure"
else
    echo "[FAIL] Step 1: Create test data directory structure"
    echo "  Command: mkdir -p ${TEST_DATA_DIR}/{input,output,results} && echo 'Created: '${TEST_DATA_DIR}"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_data_dir_captured: $GENERAL_VERIFY_PASS_verify_data_dir_captured"
    echo "  GENERAL_VERIFY_PASS_verify_data_dir_exists: $GENERAL_VERIFY_PASS_verify_data_dir_exists"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 1,'
    echo '    "step": 1,'
    echo '    "command": "mkdir -p ${TEST_DATA_DIR}/{input,output,results} && echo \"Created: \"${TEST_DATA_DIR}",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 2: Generate test run identifier
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-1_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ date +%Y%m%d_%H%M%S; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_test_run_id=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9_]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " test_run_id "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES test_run_id"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="[[ \"\$COMMAND_OUTPUT\" =~ ^[0-9_]+\$ ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 2: Generate test run identifier"
else
    echo "[FAIL] Step 2: Generate test run identifier"
    echo "  Command: date +%Y%m%d_%H%M%S"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 1,'
    echo '    "step": 2,'
    echo '    "command": "date +%Y%m%d_%H%M%S",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 3: Initialize iteration counter and results file
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-1_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo 0 > \${data_dir}/results/iteration_counter.txt && cat > \${data_dir}/results/test_results_\${test_run_id}.csv << EOF iteration,data_size,processing_time_ms,success,error_count EOF cat \${data_dir}/results/test_results_\${test_run_id}.csv"
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
COMMAND_OUTPUT=$({ eval "$SUBSTITUTED_COMMAND"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="grep -q 'iteration' <<< \"\$COMMAND_OUTPUT\""
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 3: Initialize iteration counter and results file"
else
    echo "[FAIL] Step 3: Initialize iteration counter and results file"
    echo "  Command: echo 0 > ${data_dir}/results/iteration_counter.txt && cat > ${data_dir}/results/test_results_${test_run_id}.csv << EOF iteration,data_size,processing_time_ms,success,error_count EOF cat ${data_dir}/results/test_results_${test_run_id}.csv"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 1,'
    echo '    "step": 3,'
    echo '    "command": "echo 0 > ${data_dir}/results/iteration_counter.txt && cat > ${data_dir}/results/test_results_${test_run_id}.csv << EOF iteration,data_size,processing_time_ms,success,error_count EOF cat ${data_dir}/results/test_results_${test_run_id}.csv",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 4: Create test data manifest
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-1_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="cat > \${data_dir}/test_manifest.txt << EOF Test Run Manifest ================= Run ID: \${test_run_id} Iterations: \${ITERATION_COUNT} Data Set Size: \${DATA_SET_SIZE} Failure Threshold: \${FAILURE_THRESHOLD}% Data Directory: \${data_dir} Status: INITIALIZED EOF cat \${data_dir}/test_manifest.txt"
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
COMMAND_OUTPUT=$({ eval "$SUBSTITUTED_COMMAND"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="grep -q 'Run ID: \${test_run_id}' <<< \"\$COMMAND_OUTPUT\""
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 4: Create test data manifest"
else
    echo "[FAIL] Step 4: Create test data manifest"
    echo "  Command: cat > ${data_dir}/test_manifest.txt << EOF Test Run Manifest ================= Run ID: ${test_run_id} Iterations: ${ITERATION_COUNT} Data Set Size: ${DATA_SET_SIZE} Failure Threshold: ${FAILURE_THRESHOLD}% Data Directory: ${data_dir} Status: INITIALIZED EOF cat ${data_dir}/test_manifest.txt"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 1,'
    echo '    "step": 4,'
    echo '    "command": "cat > ${data_dir}/test_manifest.txt << EOF Test Run Manifest ================= Run ID: ${test_run_id} Iterations: ${ITERATION_COUNT} Data Set Size: ${DATA_SET_SIZE} Failure Threshold: ${FAILURE_THRESHOLD}% Data Directory: ${data_dir} Status: INITIALIZED EOF cat ${data_dir}/test_manifest.txt",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Execute after_sequence hook
set +e
if [ -f "test-acceptance/scripts/hooks/after_sequence_cleanup.sh" ]; then
    source "test-acceptance/scripts/hooks/after_sequence_cleanup.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/after_sequence_cleanup.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: after_sequence hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

# Test Sequence 2: Iteration 1 - Data Processing
# First iteration of data-driven test
TEST_SEQUENCE_ID=2
TEST_SEQUENCE_NAME='Iteration 1 - Data Processing'
export TEST_SEQUENCE_ID TEST_SEQUENCE_NAME
# Execute before_sequence hook
set +e
if [ -f "test-acceptance/scripts/hooks/before_sequence_log.sh" ]; then
    source "test-acceptance/scripts/hooks/before_sequence_log.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/before_sequence_log.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: before_sequence hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

# Sequence Initial Conditions
# testing: Test data infrastructure is ready
# testing: Results file is initialized

# Step 1: Generate input data for iteration 1
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-2_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="for i in \$(seq 1 \${DATA_SET_SIZE}); do echo \"DATA_ITEM_1_\$i\" >> \${data_dir}/input/iteration_1.txt; done && wc -l \${data_dir}/input/iteration_1.txt | awk '{print \$1}'"
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
COMMAND_OUTPUT=$({ eval "$SUBSTITUTED_COMMAND"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_iter1_input_count=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " iter1_input_count "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES iter1_input_count"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="[[ \"\$COMMAND_OUTPUT\" -eq \${DATA_SET_SIZE} ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 1: Generate input data for iteration 1"
else
    echo "[FAIL] Step 1: Generate input data for iteration 1"
    echo "  Command: for i in $(seq 1 ${DATA_SET_SIZE}); do echo \"DATA_ITEM_1_$i\" >> ${data_dir}/input/iteration_1.txt; done && wc -l ${data_dir}/input/iteration_1.txt | awk '{print $1}'"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 2,'
    echo '    "step": 1,'
    echo '    "command": "for i in $(seq 1 ${DATA_SET_SIZE}); do echo \"DATA_ITEM_1_$i\" >> ${data_dir}/input/iteration_1.txt; done && wc -l ${data_dir}/input/iteration_1.txt | awk \"{print $1}\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 2: Capture iteration 1 start time
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-2_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ date +%s%N; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_iter1_start_ns=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " iter1_start_ns "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES iter1_start_ns"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="[[ \"\$COMMAND_OUTPUT\" =~ ^[0-9]+\$ ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 2: Capture iteration 1 start time"
else
    echo "[FAIL] Step 2: Capture iteration 1 start time"
    echo "  Command: date +%s%N"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 2,'
    echo '    "step": 2,'
    echo '    "command": "date +%s%N",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 3: Process iteration 1 data
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-2_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="grep 'DATA_ITEM' \${data_dir}/input/iteration_1.txt | sort | uniq > \${data_dir}/output/iteration_1_processed.txt && wc -l \${data_dir}/output/iteration_1_processed.txt | awk '{print \$1}'"
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
COMMAND_OUTPUT=$({ eval "$SUBSTITUTED_COMMAND"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_iter1_output_count=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " iter1_output_count "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES iter1_output_count"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="[[ \"\$COMMAND_OUTPUT\" -eq \${DATA_SET_SIZE} ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 3: Process iteration 1 data"
else
    echo "[FAIL] Step 3: Process iteration 1 data"
    echo "  Command: grep 'DATA_ITEM' ${data_dir}/input/iteration_1.txt | sort | uniq > ${data_dir}/output/iteration_1_processed.txt && wc -l ${data_dir}/output/iteration_1_processed.txt | awk '{print $1}'"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 2,'
    echo '    "step": 3,'
    echo '    "command": "grep \"DATA_ITEM\" ${data_dir}/input/iteration_1.txt | sort | uniq > ${data_dir}/output/iteration_1_processed.txt && wc -l ${data_dir}/output/iteration_1_processed.txt | awk \"{print $1}\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 4: Capture iteration 1 end time and calculate duration
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-2_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ date +%s%N; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_iter1_end_ns=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " iter1_end_ns "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES iter1_end_ns"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="[[ \"\$COMMAND_OUTPUT\" =~ ^[0-9]+\$ ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 4: Capture iteration 1 end time and calculate duration"
else
    echo "[FAIL] Step 4: Capture iteration 1 end time and calculate duration"
    echo "  Command: date +%s%N"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 2,'
    echo '    "step": 4,'
    echo '    "command": "date +%s%N",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 5: Calculate iteration 1 processing time and record results
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-2_step-5.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="iter1_duration_ms=\$(( (\${iter1_end_ns} - \${iter1_start_ns}) / 1000000 )) && echo \"1,\${iter1_output_count},\${iter1_duration_ms},true,0\" >> \${data_dir}/results/test_results_\${test_run_id}.csv && echo \$iter1_duration_ms"
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
COMMAND_OUTPUT=$({ eval "$SUBSTITUTED_COMMAND"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_iter1_duration_ms=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " iter1_duration_ms "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES iter1_duration_ms"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="[[ \"\$COMMAND_OUTPUT\" =~ ^[0-9]+\$ ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

# General verifications
GENERAL_VERIFY_PASS_verify_iter1_duration=false
EXPR="[[ \$iter1_duration_ms -ge 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    GENERAL_VERIFY_PASS_verify_iter1_duration=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_iter1_duration" = true ]; then
    echo "[PASS] Step 5: Calculate iteration 1 processing time and record results"
else
    echo "[FAIL] Step 5: Calculate iteration 1 processing time and record results"
    echo "  Command: iter1_duration_ms=$(( (${iter1_end_ns} - ${iter1_start_ns}) / 1000000 )) && echo \"1,${iter1_output_count},${iter1_duration_ms},true,0\" >> ${data_dir}/results/test_results_${test_run_id}.csv && echo $iter1_duration_ms"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_iter1_duration: $GENERAL_VERIFY_PASS_verify_iter1_duration"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 2,'
    echo '    "step": 5,'
    echo '    "command": "iter1_duration_ms=$(( (${iter1_end_ns} - ${iter1_start_ns}) / 1000000 )) && echo \"1,${iter1_output_count},${iter1_duration_ms},true,0\" >> ${data_dir}/results/test_results_${test_run_id}.csv && echo $iter1_duration_ms",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Execute after_sequence hook
set +e
if [ -f "test-acceptance/scripts/hooks/after_sequence_cleanup.sh" ]; then
    source "test-acceptance/scripts/hooks/after_sequence_cleanup.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/after_sequence_cleanup.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: after_sequence hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

# Test Sequence 3: Iteration 2 - Data Processing
# Second iteration of data-driven test
TEST_SEQUENCE_ID=3
TEST_SEQUENCE_NAME='Iteration 2 - Data Processing'
export TEST_SEQUENCE_ID TEST_SEQUENCE_NAME
# Execute before_sequence hook
set +e
if [ -f "test-acceptance/scripts/hooks/before_sequence_log.sh" ]; then
    source "test-acceptance/scripts/hooks/before_sequence_log.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/before_sequence_log.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: before_sequence hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

# Sequence Initial Conditions
# testing: Iteration 1 completed successfully

# Step 1: Generate input data for iteration 2
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-3_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="for i in \$(seq 1 \${DATA_SET_SIZE}); do echo \"DATA_ITEM_2_\$i\" >> \${data_dir}/input/iteration_2.txt; done && wc -l \${data_dir}/input/iteration_2.txt | awk '{print \$1}'"
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
COMMAND_OUTPUT=$({ eval "$SUBSTITUTED_COMMAND"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_iter2_input_count=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " iter2_input_count "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES iter2_input_count"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="[[ \"\$COMMAND_OUTPUT\" -eq \${DATA_SET_SIZE} ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 1: Generate input data for iteration 2"
else
    echo "[FAIL] Step 1: Generate input data for iteration 2"
    echo "  Command: for i in $(seq 1 ${DATA_SET_SIZE}); do echo \"DATA_ITEM_2_$i\" >> ${data_dir}/input/iteration_2.txt; done && wc -l ${data_dir}/input/iteration_2.txt | awk '{print $1}'"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 3,'
    echo '    "step": 1,'
    echo '    "command": "for i in $(seq 1 ${DATA_SET_SIZE}); do echo \"DATA_ITEM_2_$i\" >> ${data_dir}/input/iteration_2.txt; done && wc -l ${data_dir}/input/iteration_2.txt | awk \"{print $1}\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 2: Capture iteration 2 start time
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-3_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ date +%s%N; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_iter2_start_ns=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " iter2_start_ns "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES iter2_start_ns"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="[[ \"\$COMMAND_OUTPUT\" =~ ^[0-9]+\$ ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 2: Capture iteration 2 start time"
else
    echo "[FAIL] Step 2: Capture iteration 2 start time"
    echo "  Command: date +%s%N"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 3,'
    echo '    "step": 2,'
    echo '    "command": "date +%s%N",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 3: Process iteration 2 data
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-3_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="grep 'DATA_ITEM' \${data_dir}/input/iteration_2.txt | sort | uniq > \${data_dir}/output/iteration_2_processed.txt && wc -l \${data_dir}/output/iteration_2_processed.txt | awk '{print \$1}'"
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
COMMAND_OUTPUT=$({ eval "$SUBSTITUTED_COMMAND"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_iter2_output_count=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " iter2_output_count "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES iter2_output_count"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="[[ \"\$COMMAND_OUTPUT\" -eq \${DATA_SET_SIZE} ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 3: Process iteration 2 data"
else
    echo "[FAIL] Step 3: Process iteration 2 data"
    echo "  Command: grep 'DATA_ITEM' ${data_dir}/input/iteration_2.txt | sort | uniq > ${data_dir}/output/iteration_2_processed.txt && wc -l ${data_dir}/output/iteration_2_processed.txt | awk '{print $1}'"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 3,'
    echo '    "step": 3,'
    echo '    "command": "grep \"DATA_ITEM\" ${data_dir}/input/iteration_2.txt | sort | uniq > ${data_dir}/output/iteration_2_processed.txt && wc -l ${data_dir}/output/iteration_2_processed.txt | awk \"{print $1}\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 4: Capture iteration 2 end time and calculate duration
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-3_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ date +%s%N; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_iter2_end_ns=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " iter2_end_ns "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES iter2_end_ns"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="[[ \"\$COMMAND_OUTPUT\" =~ ^[0-9]+\$ ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 4: Capture iteration 2 end time and calculate duration"
else
    echo "[FAIL] Step 4: Capture iteration 2 end time and calculate duration"
    echo "  Command: date +%s%N"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 3,'
    echo '    "step": 4,'
    echo '    "command": "date +%s%N",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 5: Calculate iteration 2 processing time and record results
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-3_step-5.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="iter2_duration_ms=\$(( (\${iter2_end_ns} - \${iter2_start_ns}) / 1000000 )) && echo \"2,\${iter2_output_count},\${iter2_duration_ms},true,0\" >> \${data_dir}/results/test_results_\${test_run_id}.csv && echo \$iter2_duration_ms"
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
COMMAND_OUTPUT=$({ eval "$SUBSTITUTED_COMMAND"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_iter2_duration_ms=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " iter2_duration_ms "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES iter2_duration_ms"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="[[ \"\$COMMAND_OUTPUT\" =~ ^[0-9]+\$ ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 5: Calculate iteration 2 processing time and record results"
else
    echo "[FAIL] Step 5: Calculate iteration 2 processing time and record results"
    echo "  Command: iter2_duration_ms=$(( (${iter2_end_ns} - ${iter2_start_ns}) / 1000000 )) && echo \"2,${iter2_output_count},${iter2_duration_ms},true,0\" >> ${data_dir}/results/test_results_${test_run_id}.csv && echo $iter2_duration_ms"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 3,'
    echo '    "step": 5,'
    echo '    "command": "iter2_duration_ms=$(( (${iter2_end_ns} - ${iter2_start_ns}) / 1000000 )) && echo \"2,${iter2_output_count},${iter2_duration_ms},true,0\" >> ${data_dir}/results/test_results_${test_run_id}.csv && echo $iter2_duration_ms",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Execute after_sequence hook
set +e
if [ -f "test-acceptance/scripts/hooks/after_sequence_cleanup.sh" ]; then
    source "test-acceptance/scripts/hooks/after_sequence_cleanup.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/after_sequence_cleanup.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: after_sequence hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

# Test Sequence 4: Iteration 3 - Data Processing
# Third iteration of data-driven test
TEST_SEQUENCE_ID=4
TEST_SEQUENCE_NAME='Iteration 3 - Data Processing'
export TEST_SEQUENCE_ID TEST_SEQUENCE_NAME
# Execute before_sequence hook
set +e
if [ -f "test-acceptance/scripts/hooks/before_sequence_log.sh" ]; then
    source "test-acceptance/scripts/hooks/before_sequence_log.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/before_sequence_log.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: before_sequence hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

# Sequence Initial Conditions
# testing: Iteration 2 completed successfully

# Step 1: Generate input data for iteration 3
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-4_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="for i in \$(seq 1 \${DATA_SET_SIZE}); do echo \"DATA_ITEM_3_\$i\" >> \${data_dir}/input/iteration_3.txt; done && wc -l \${data_dir}/input/iteration_3.txt | awk '{print \$1}'"
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
COMMAND_OUTPUT=$({ eval "$SUBSTITUTED_COMMAND"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_iter3_input_count=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " iter3_input_count "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES iter3_input_count"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="[[ \"\$COMMAND_OUTPUT\" -eq \${DATA_SET_SIZE} ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 1: Generate input data for iteration 3"
else
    echo "[FAIL] Step 1: Generate input data for iteration 3"
    echo "  Command: for i in $(seq 1 ${DATA_SET_SIZE}); do echo \"DATA_ITEM_3_$i\" >> ${data_dir}/input/iteration_3.txt; done && wc -l ${data_dir}/input/iteration_3.txt | awk '{print $1}'"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 4,'
    echo '    "step": 1,'
    echo '    "command": "for i in $(seq 1 ${DATA_SET_SIZE}); do echo \"DATA_ITEM_3_$i\" >> ${data_dir}/input/iteration_3.txt; done && wc -l ${data_dir}/input/iteration_3.txt | awk \"{print $1}\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 2: Capture iteration 3 start time
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-4_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ date +%s%N; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_iter3_start_ns=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " iter3_start_ns "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES iter3_start_ns"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="[[ \"\$COMMAND_OUTPUT\" =~ ^[0-9]+\$ ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 2: Capture iteration 3 start time"
else
    echo "[FAIL] Step 2: Capture iteration 3 start time"
    echo "  Command: date +%s%N"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 4,'
    echo '    "step": 2,'
    echo '    "command": "date +%s%N",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 3: Process iteration 3 data
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-4_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="grep 'DATA_ITEM' \${data_dir}/input/iteration_3.txt | sort | uniq > \${data_dir}/output/iteration_3_processed.txt && wc -l \${data_dir}/output/iteration_3_processed.txt | awk '{print \$1}'"
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
COMMAND_OUTPUT=$({ eval "$SUBSTITUTED_COMMAND"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_iter3_output_count=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " iter3_output_count "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES iter3_output_count"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="[[ \"\$COMMAND_OUTPUT\" -eq \${DATA_SET_SIZE} ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 3: Process iteration 3 data"
else
    echo "[FAIL] Step 3: Process iteration 3 data"
    echo "  Command: grep 'DATA_ITEM' ${data_dir}/input/iteration_3.txt | sort | uniq > ${data_dir}/output/iteration_3_processed.txt && wc -l ${data_dir}/output/iteration_3_processed.txt | awk '{print $1}'"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 4,'
    echo '    "step": 3,'
    echo '    "command": "grep \"DATA_ITEM\" ${data_dir}/input/iteration_3.txt | sort | uniq > ${data_dir}/output/iteration_3_processed.txt && wc -l ${data_dir}/output/iteration_3_processed.txt | awk \"{print $1}\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 4: Capture iteration 3 end time and calculate duration
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-4_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ date +%s%N; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_iter3_end_ns=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " iter3_end_ns "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES iter3_end_ns"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="[[ \"\$COMMAND_OUTPUT\" =~ ^[0-9]+\$ ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 4: Capture iteration 3 end time and calculate duration"
else
    echo "[FAIL] Step 4: Capture iteration 3 end time and calculate duration"
    echo "  Command: date +%s%N"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 4,'
    echo '    "step": 4,'
    echo '    "command": "date +%s%N",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 5: Calculate iteration 3 processing time and record results
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-4_step-5.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="iter3_duration_ms=\$(( (\${iter3_end_ns} - \${iter3_start_ns}) / 1000000 )) && echo \"3,\${iter3_output_count},\${iter3_duration_ms},true,0\" >> \${data_dir}/results/test_results_\${test_run_id}.csv && echo \$iter3_duration_ms"
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
COMMAND_OUTPUT=$({ eval "$SUBSTITUTED_COMMAND"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_iter3_duration_ms=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " iter3_duration_ms "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES iter3_duration_ms"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="[[ \"\$COMMAND_OUTPUT\" =~ ^[0-9]+\$ ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 5: Calculate iteration 3 processing time and record results"
else
    echo "[FAIL] Step 5: Calculate iteration 3 processing time and record results"
    echo "  Command: iter3_duration_ms=$(( (${iter3_end_ns} - ${iter3_start_ns}) / 1000000 )) && echo \"3,${iter3_output_count},${iter3_duration_ms},true,0\" >> ${data_dir}/results/test_results_${test_run_id}.csv && echo $iter3_duration_ms"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 4,'
    echo '    "step": 5,'
    echo '    "command": "iter3_duration_ms=$(( (${iter3_end_ns} - ${iter3_start_ns}) / 1000000 )) && echo \"3,${iter3_output_count},${iter3_duration_ms},true,0\" >> ${data_dir}/results/test_results_${test_run_id}.csv && echo $iter3_duration_ms",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Execute after_sequence hook
set +e
if [ -f "test-acceptance/scripts/hooks/after_sequence_cleanup.sh" ]; then
    source "test-acceptance/scripts/hooks/after_sequence_cleanup.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/after_sequence_cleanup.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: after_sequence hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

# Test Sequence 5: Results Aggregation and Analysis
# Aggregate results from all iterations and generate analysis report
TEST_SEQUENCE_ID=5
TEST_SEQUENCE_NAME='Results Aggregation and Analysis'
export TEST_SEQUENCE_ID TEST_SEQUENCE_NAME
# Execute before_sequence hook
set +e
if [ -f "test-acceptance/scripts/hooks/before_sequence_log.sh" ]; then
    source "test-acceptance/scripts/hooks/before_sequence_log.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/before_sequence_log.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: before_sequence hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

# Sequence Initial Conditions
# testing: All iterations have completed
# testing: Results CSV file contains all iteration data

# Step 1: Count total iterations completed
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-5_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="tail -n +2 \${data_dir}/results/test_results_\${test_run_id}.csv | wc -l"
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
COMMAND_OUTPUT=$({ eval "$SUBSTITUTED_COMMAND"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_total_iterations=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " total_iterations "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES total_iterations"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
COND_EXPR="[[ \$total_iterations -ge 3 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        COND_EXPR=$(echo "$COND_EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$COND_EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
    echo 'Required iterations completed'
else
    echo 'Fewer iterations than expected'
fi
echo "Iterations: ${total_iterations}"

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 1: Count total iterations completed"
else
    echo "[FAIL] Step 1: Count total iterations completed"
    echo "  Command: tail -n +2 ${data_dir}/results/test_results_${test_run_id}.csv | wc -l"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 5,'
    echo '    "step": 1,'
    echo '    "command": "tail -n +2 ${data_dir}/results/test_results_${test_run_id}.csv | wc -l",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 2: Calculate average processing time across iterations
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-5_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="tail -n +2 \${data_dir}/results/test_results_\${test_run_id}.csv | awk -F',' '{sum+=\$3; count++} END {if(count>0) print int(sum/count); else print 0}'"
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
COMMAND_OUTPUT=$({ eval "$SUBSTITUTED_COMMAND"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_avg_processing_time_ms=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " avg_processing_time_ms "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES avg_processing_time_ms"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="[[ \"\$COMMAND_OUTPUT\" =~ ^[0-9]+\$ ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

# General verifications
GENERAL_VERIFY_PASS_verify_avg_time_calculated=false
EXPR="[[ \$avg_processing_time_ms =~ ^[0-9]+\$ ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    GENERAL_VERIFY_PASS_verify_avg_time_calculated=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_avg_time_calculated" = true ]; then
    echo "[PASS] Step 2: Calculate average processing time across iterations"
else
    echo "[FAIL] Step 2: Calculate average processing time across iterations"
    echo "  Command: tail -n +2 ${data_dir}/results/test_results_${test_run_id}.csv | awk -F',' '{sum+=$3; count++} END {if(count>0) print int(sum/count); else print 0}'"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_avg_time_calculated: $GENERAL_VERIFY_PASS_verify_avg_time_calculated"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 5,'
    echo '    "step": 2,'
    echo '    "command": "tail -n +2 ${data_dir}/results/test_results_${test_run_id}.csv | awk -F\",\" \"{sum+=$3; count++} END {if(count>0) print int(sum/count); else print 0}\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 3: Calculate minimum processing time
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-5_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="tail -n +2 \${data_dir}/results/test_results_\${test_run_id}.csv | awk -F',' 'NR==1 || \$3<min {min=\$3} END {print int(min)}'"
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
COMMAND_OUTPUT=$({ eval "$SUBSTITUTED_COMMAND"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_min_processing_time_ms=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " min_processing_time_ms "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES min_processing_time_ms"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="[[ \"\$COMMAND_OUTPUT\" =~ ^[0-9]+\$ ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 3: Calculate minimum processing time"
else
    echo "[FAIL] Step 3: Calculate minimum processing time"
    echo "  Command: tail -n +2 ${data_dir}/results/test_results_${test_run_id}.csv | awk -F',' 'NR==1 || $3<min {min=$3} END {print int(min)}'"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 5,'
    echo '    "step": 3,'
    echo '    "command": "tail -n +2 ${data_dir}/results/test_results_${test_run_id}.csv | awk -F\",\" \"NR==1 || $3<min {min=$3} END {print int(min)}\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 4: Calculate maximum processing time
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-5_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="tail -n +2 \${data_dir}/results/test_results_\${test_run_id}.csv | awk -F',' 'NR==1 || \$3>max {max=\$3} END {print int(max)}'"
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
COMMAND_OUTPUT=$({ eval "$SUBSTITUTED_COMMAND"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_max_processing_time_ms=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " max_processing_time_ms "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES max_processing_time_ms"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="[[ \"\$COMMAND_OUTPUT\" =~ ^[0-9]+\$ ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 4: Calculate maximum processing time"
else
    echo "[FAIL] Step 4: Calculate maximum processing time"
    echo "  Command: tail -n +2 ${data_dir}/results/test_results_${test_run_id}.csv | awk -F',' 'NR==1 || $3>max {max=$3} END {print int(max)}'"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 5,'
    echo '    "step": 4,'
    echo '    "command": "tail -n +2 ${data_dir}/results/test_results_${test_run_id}.csv | awk -F\",\" \"NR==1 || $3>max {max=$3} END {print int(max)}\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 5: Count successful iterations
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-5_step-5.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="tail -n +2 \${data_dir}/results/test_results_\${test_run_id}.csv | grep -c 'true'"
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
COMMAND_OUTPUT=$({ eval "$SUBSTITUTED_COMMAND"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_success_count=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " success_count "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES success_count"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="[[ \"\$COMMAND_OUTPUT\" =~ ^[0-9]+\$ ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 5: Count successful iterations"
else
    echo "[FAIL] Step 5: Count successful iterations"
    echo "  Command: tail -n +2 ${data_dir}/results/test_results_${test_run_id}.csv | grep -c 'true'"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 5,'
    echo '    "step": 5,'
    echo '    "command": "tail -n +2 ${data_dir}/results/test_results_${test_run_id}.csv | grep -c \"true\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 6: Calculate success rate percentage
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-5_step-6.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo \"scale=2; (\${success_count} * 100) / \${total_iterations}\" | bc 2>/dev/null || echo 100"
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
COMMAND_OUTPUT=$({ eval "$SUBSTITUTED_COMMAND"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_success_rate=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+\\.?[0-9]*)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " success_rate "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES success_rate"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
COND_EXPR="[[ \$(echo \"\${success_rate} >= 90\" | bc 2>/dev/null || echo 1) -eq 1 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        COND_EXPR=$(echo "$COND_EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$COND_EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
    echo 'Success rate meets quality threshold (>=90%)'
else
    echo 'WARNING: Success rate below quality threshold'
fi
echo "Success rate: ${success_rate}%"

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 6: Calculate success rate percentage"
else
    echo "[FAIL] Step 6: Calculate success rate percentage"
    echo "  Command: echo \"scale=2; (${success_count} * 100) / ${total_iterations}\" | bc 2>/dev/null || echo 100"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 5,'
    echo '    "step": 6,'
    echo '    "command": "echo \"scale=2; (${success_count} * 100) / ${total_iterations}\" | bc 2>/dev/null || echo 100",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 7: Generate comprehensive analysis report
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-5_step-7.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="cat > \${data_dir}/results/analysis_report_\${test_run_id}.txt << EOF Data-Driven Test Analysis Report =================================
Test Configuration: - Run ID: \${test_run_id} - Iterations Configured: \${ITERATION_COUNT} - Iterations Completed: \${total_iterations} - Data Set Size: \${DATA_SET_SIZE} items per iteration - Failure Threshold: \${FAILURE_THRESHOLD}%
Performance Metrics: ------------------- - Average Processing Time: \${avg_processing_time_ms}ms - Minimum Processing Time: \${min_processing_time_ms}ms - Maximum Processing Time: \${max_processing_time_ms}ms - Processing Time Range: \$((\${max_processing_time_ms} - \${min_processing_time_ms}))ms
Quality Metrics: --------------- - Successful Iterations: \${success_count} / \${total_iterations} - Success Rate: \${success_rate}% - Quality Threshold: 90% - Status: \$([ \$(echo \"\${success_rate} >= 90\" | bc 2>/dev/null || echo 1) -eq 1 ] && echo 'PASS' || echo 'FAIL')
Individual Iteration Results: ---------------------------- Iteration 1: \${iter1_duration_ms}ms (\${iter1_output_count} items) Iteration 2: \${iter2_duration_ms}ms (\${iter2_output_count} items) Iteration 3: \${iter3_duration_ms}ms (\${iter3_output_count} items)
Data Verification: ----------------- - Input Files: \$(ls \${data_dir}/input/ | wc -l) - Output Files: \$(ls \${data_dir}/output/ | wc -l) - Results Files: \$(ls \${data_dir}/results/ | wc -l)
Conclusion: ---------- \$([ \$(echo \"\${success_rate} >= 90\" | bc 2>/dev/null || echo 1) -eq 1 ] && echo 'All iterations completed successfully with acceptable performance.' || echo 'Some iterations did not meet quality thresholds.')
Test data and results are available in: \${data_dir} EOF cat \${data_dir}/results/analysis_report_\${test_run_id}.txt"
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
COMMAND_OUTPUT=$({ eval "$SUBSTITUTED_COMMAND"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="grep -q 'Data-Driven Test Analysis Report' <<< \"\$COMMAND_OUTPUT\" && grep -q 'Success Rate: \${success_rate}%' <<< \"\$COMMAND_OUTPUT\""
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 7: Generate comprehensive analysis report"
else
    echo "[FAIL] Step 7: Generate comprehensive analysis report"
    echo "  Command: cat > ${data_dir}/results/analysis_report_${test_run_id}.txt << EOF Data-Driven Test Analysis Report =================================
Test Configuration: - Run ID: ${test_run_id} - Iterations Configured: ${ITERATION_COUNT} - Iterations Completed: ${total_iterations} - Data Set Size: ${DATA_SET_SIZE} items per iteration - Failure Threshold: ${FAILURE_THRESHOLD}%
Performance Metrics: ------------------- - Average Processing Time: ${avg_processing_time_ms}ms - Minimum Processing Time: ${min_processing_time_ms}ms - Maximum Processing Time: ${max_processing_time_ms}ms - Processing Time Range: $((${max_processing_time_ms} - ${min_processing_time_ms}))ms
Quality Metrics: --------------- - Successful Iterations: ${success_count} / ${total_iterations} - Success Rate: ${success_rate}% - Quality Threshold: 90% - Status: $([ $(echo \"${success_rate} >= 90\" | bc 2>/dev/null || echo 1) -eq 1 ] && echo 'PASS' || echo 'FAIL')
Individual Iteration Results: ---------------------------- Iteration 1: ${iter1_duration_ms}ms (${iter1_output_count} items) Iteration 2: ${iter2_duration_ms}ms (${iter2_output_count} items) Iteration 3: ${iter3_duration_ms}ms (${iter3_output_count} items)
Data Verification: ----------------- - Input Files: $(ls ${data_dir}/input/ | wc -l) - Output Files: $(ls ${data_dir}/output/ | wc -l) - Results Files: $(ls ${data_dir}/results/ | wc -l)
Conclusion: ---------- $([ $(echo \"${success_rate} >= 90\" | bc 2>/dev/null || echo 1) -eq 1 ] && echo 'All iterations completed successfully with acceptable performance.' || echo 'Some iterations did not meet quality thresholds.')
Test data and results are available in: ${data_dir} EOF cat ${data_dir}/results/analysis_report_${test_run_id}.txt"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 5,'
    echo '    "step": 7,'
    echo '    "command": "cat > ${data_dir}/results/analysis_report_${test_run_id}.txt << EOF Data-Driven Test Analysis Report =================================\nTest Configuration: - Run ID: ${test_run_id} - Iterations Configured: ${ITERATION_COUNT} - Iterations Completed: ${total_iterations} - Data Set Size: ${DATA_SET_SIZE} items per iteration - Failure Threshold: ${FAILURE_THRESHOLD}%\nPerformance Metrics: ------------------- - Average Processing Time: ${avg_processing_time_ms}ms - Minimum Processing Time: ${min_processing_time_ms}ms - Maximum Processing Time: ${max_processing_time_ms}ms - Processing Time Range: $((${max_processing_time_ms} - ${min_processing_time_ms}))ms\nQuality Metrics: --------------- - Successful Iterations: ${success_count} / ${total_iterations} - Success Rate: ${success_rate}% - Quality Threshold: 90% - Status: $([ $(echo \"${success_rate} >= 90\" | bc 2>/dev/null || echo 1) -eq 1 ] && echo \"PASS\" || echo \"FAIL\")\nIndividual Iteration Results: ---------------------------- Iteration 1: ${iter1_duration_ms}ms (${iter1_output_count} items) Iteration 2: ${iter2_duration_ms}ms (${iter2_output_count} items) Iteration 3: ${iter3_duration_ms}ms (${iter3_output_count} items)\nData Verification: ----------------- - Input Files: $(ls ${data_dir}/input/ | wc -l) - Output Files: $(ls ${data_dir}/output/ | wc -l) - Results Files: $(ls ${data_dir}/results/ | wc -l)\nConclusion: ---------- $([ $(echo \"${success_rate} >= 90\" | bc 2>/dev/null || echo 1) -eq 1 ] && echo \"All iterations completed successfully with acceptable performance.\" || echo \"Some iterations did not meet quality thresholds.\")\nTest data and results are available in: ${data_dir} EOF cat ${data_dir}/results/analysis_report_${test_run_id}.txt",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 8: Verify analysis report completeness
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-5_step-8.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="grep -E '(Performance Metrics|Quality Metrics|Individual Iteration|Conclusion)' \${data_dir}/results/analysis_report_\${test_run_id}.txt | wc -l"
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
COMMAND_OUTPUT=$({ eval "$SUBSTITUTED_COMMAND"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_report_sections=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " report_sections "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES report_sections"
fi

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
COND_EXPR="[[ \$report_sections -ge 4 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        COND_EXPR=$(echo "$COND_EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$COND_EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
    echo 'Report contains all required sections'
else
    echo 'Report may be incomplete'
fi
echo "Report sections: ${report_sections}"

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 8: Verify analysis report completeness"
else
    echo "[FAIL] Step 8: Verify analysis report completeness"
    echo "  Command: grep -E '(Performance Metrics|Quality Metrics|Individual Iteration|Conclusion)' ${data_dir}/results/analysis_report_${test_run_id}.txt | wc -l"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 5,'
    echo '    "step": 8,'
    echo '    "command": "grep -E \"(Performance Metrics|Quality Metrics|Individual Iteration|Conclusion)\" ${data_dir}/results/analysis_report_${test_run_id}.txt | wc -l",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 9: Archive test results and cleanup
LOG_FILE="TC_COMPLEX_DATA_DRIVEN_ITERATIONS_001_sequence-5_step-9.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="cd \${data_dir} && tar -czf /tmp/data_driven_results_\${test_run_id}.tar.gz results/ 2>&1 | head -1 || echo 'Results archived' && rm -rf \${data_dir} && echo 'Test cleanup complete'"
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
COMMAND_OUTPUT=$({ eval "$SUBSTITUTED_COMMAND"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
EXPR="grep -q 'complete' <<< \"\$COMMAND_OUTPUT\""
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

# General verifications
GENERAL_VERIFY_PASS_verify_cleanup_complete=false
EXPR="[[ ! -d \"\${data_dir}\" ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        # Replace ${var_name} pattern
        EXPR=$(echo "$EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$EXPR"; then
    GENERAL_VERIFY_PASS_verify_cleanup_complete=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_cleanup_complete" = true ]; then
    echo "[PASS] Step 9: Archive test results and cleanup"
else
    echo "[FAIL] Step 9: Archive test results and cleanup"
    echo "  Command: cd ${data_dir} && tar -czf /tmp/data_driven_results_${test_run_id}.tar.gz results/ 2>&1 | head -1 || echo 'Results archived' && rm -rf ${data_dir} && echo 'Test cleanup complete'"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_cleanup_complete: $GENERAL_VERIFY_PASS_verify_cleanup_complete"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

if [ "$FIRST_ENTRY" = false ]; then
    echo ',' >> "$JSON_LOG"
fi
FIRST_ENTRY=false

# Write JSON entry
{
    echo '  {'
    echo '    "test_sequence": 5,'
    echo '    "step": 9,'
    echo '    "command": "cd ${data_dir} && tar -czf /tmp/data_driven_results_${test_run_id}.tar.gz results/ 2>&1 | head -1 || echo \"Results archived\" && rm -rf ${data_dir} && echo \"Test cleanup complete\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Execute after_sequence hook
set +e
if [ -f "test-acceptance/scripts/hooks/after_sequence_cleanup.sh" ]; then
    source "test-acceptance/scripts/hooks/after_sequence_cleanup.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/after_sequence_cleanup.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: after_sequence hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

echo ']' >> "$JSON_LOG"

# Validate JSON against schema
if command -v jq >/dev/null 2>&1; then
    if ! jq empty "$JSON_LOG" >/dev/null 2>&1; then
        echo "500 - Internal Script Error: Generated JSON is not valid"
        exit 1
    fi
fi

# Execute teardown_test hook
set +e
if [ -f "test-acceptance/scripts/hooks/teardown_test_final.sh" ]; then
    source "test-acceptance/scripts/hooks/teardown_test_final.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/teardown_test_final.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: teardown_test hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

echo "All test sequences completed successfully"
exit 0
