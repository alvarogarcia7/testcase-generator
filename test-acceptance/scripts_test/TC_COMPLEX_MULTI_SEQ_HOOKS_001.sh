#!/bin/bash
set -euo pipefail

# Execute script_start hook
set +e
if [ -f "test-acceptance/scripts/hooks/script_start_init.sh" ]; then
    source "test-acceptance/scripts/hooks/script_start_init.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/script_start_init.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Error: script_start hook failed with exit code $HOOK_EXIT_CODE" >&2
    exit $HOOK_EXIT_CODE
fi

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

# Test Case: TC_COMPLEX_MULTI_SEQ_HOOKS_001
# Description: Multi-sequence test with hooks, variables, and manual steps - comprehensive workflow with lifecycle management

JSON_LOG="TC_COMPLEX_MULTI_SEQ_HOOKS_001_execution_log.json"
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
# system: Bash shell version 3.2 or higher is available
# system: Test hooks infrastructure is properly configured
# system: File system operations are permitted
# environment: Environment variables are properly hydrated

# Initial Conditions
# filesystem: Temporary directory is writable
# filesystem: Test workspace can be created
# system: All hook scripts are executable and accessible

# Test Sequence 1: Automated Setup and Configuration
# Initialize test environment with variable capture and validation
TEST_SEQUENCE_ID=1
TEST_SEQUENCE_NAME='Automated Setup and Configuration'
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
# system: Hooks have initialized global context
# system: Workspace directory is available

# Step 1: Create test workspace with captured timestamp
TEST_STEP_NUMBER=1
TEST_STEP_DESCRIPTION='Create test workspace with captured timestamp'
export TEST_STEP_NUMBER TEST_STEP_DESCRIPTION
# Execute before_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/before_step_validate.sh" ]; then
    source "test-acceptance/scripts/hooks/before_step_validate.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/before_step_validate.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: before_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

LOG_FILE="TC_COMPLEX_MULTI_SEQ_HOOKS_001_sequence-1_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ date +%s; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_test_start_time=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " test_start_time "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES test_start_time"
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
GENERAL_VERIFY_PASS_verify_timestamp_captured=false
EXPR="[[ -n \"\$test_start_time\" ]]"
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
    GENERAL_VERIFY_PASS_verify_timestamp_captured=true
fi
GENERAL_VERIFY_PASS_verify_timestamp_numeric=false
EXPR="[[ \$test_start_time =~ ^[0-9]+\$ ]]"
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
    GENERAL_VERIFY_PASS_verify_timestamp_numeric=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_timestamp_captured" = true ] && [ "$GENERAL_VERIFY_PASS_verify_timestamp_numeric" = true ]; then
    echo "[PASS] Step 1: Create test workspace with captured timestamp"
else
    echo "[FAIL] Step 1: Create test workspace with captured timestamp"
    echo "  Command: date +%s"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_timestamp_captured: $GENERAL_VERIFY_PASS_verify_timestamp_captured"
    echo "  GENERAL_VERIFY_PASS_verify_timestamp_numeric: $GENERAL_VERIFY_PASS_verify_timestamp_numeric"
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
    echo '    "command": "date +%s",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

STEP_EXIT_CODE=$EXIT_CODE
export STEP_EXIT_CODE COMMAND_OUTPUT
# Execute after_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/after_step_metrics.sh" ]; then
    source "test-acceptance/scripts/hooks/after_step_metrics.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/after_step_metrics.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: after_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

# Step 2: Initialize workspace structure with environment variable
TEST_STEP_NUMBER=2
TEST_STEP_DESCRIPTION='Initialize workspace structure with environment variable'
export TEST_STEP_NUMBER TEST_STEP_DESCRIPTION
# Execute before_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/before_step_validate.sh" ]; then
    source "test-acceptance/scripts/hooks/before_step_validate.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/before_step_validate.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: before_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

LOG_FILE="TC_COMPLEX_MULTI_SEQ_HOOKS_001_sequence-1_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="mkdir -p \${TEST_WORKSPACE}/run_\${test_start_time} && echo \${TEST_WORKSPACE}/run_\${test_start_time}"
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
STEP_VAR_run_directory=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\((/tmp/[^[[:space:]]]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " run_directory "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES run_directory"
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
EXPR="grep -q '/tmp/' <<< \"\$COMMAND_OUTPUT\""
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
GENERAL_VERIFY_PASS_verify_run_directory_captured=false
EXPR="[[ -n \"\$run_directory\" ]]"
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
    GENERAL_VERIFY_PASS_verify_run_directory_captured=true
fi
GENERAL_VERIFY_PASS_verify_directory_exists=false
EXPR="[[ -d \"\$run_directory\" ]]"
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
    GENERAL_VERIFY_PASS_verify_directory_exists=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_run_directory_captured" = true ] && [ "$GENERAL_VERIFY_PASS_verify_directory_exists" = true ]; then
    echo "[PASS] Step 2: Initialize workspace structure with environment variable"
else
    echo "[FAIL] Step 2: Initialize workspace structure with environment variable"
    echo "  Command: mkdir -p ${TEST_WORKSPACE}/run_${test_start_time} && echo ${TEST_WORKSPACE}/run_${test_start_time}"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_run_directory_captured: $GENERAL_VERIFY_PASS_verify_run_directory_captured"
    echo "  GENERAL_VERIFY_PASS_verify_directory_exists: $GENERAL_VERIFY_PASS_verify_directory_exists"
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
    echo '    "command": "mkdir -p ${TEST_WORKSPACE}/run_${test_start_time} && echo ${TEST_WORKSPACE}/run_${test_start_time}",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

STEP_EXIT_CODE=$EXIT_CODE
export STEP_EXIT_CODE COMMAND_OUTPUT
# Execute after_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/after_step_metrics.sh" ]; then
    source "test-acceptance/scripts/hooks/after_step_metrics.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/after_step_metrics.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: after_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

# Step 3: Generate test configuration file
TEST_STEP_NUMBER=3
TEST_STEP_DESCRIPTION='Generate test configuration file'
export TEST_STEP_NUMBER TEST_STEP_DESCRIPTION
# Execute before_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/before_step_validate.sh" ]; then
    source "test-acceptance/scripts/hooks/before_step_validate.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/before_step_validate.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: before_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

LOG_FILE="TC_COMPLEX_MULTI_SEQ_HOOKS_001_sequence-1_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="cat > \${run_directory}/config.txt << EOF environment=\${TEST_ENVIRONMENT} workspace=\${TEST_WORKSPACE} start_time=\${test_start_time} max_retries=\${MAX_RETRIES} EOF cat \${run_directory}/config.txt"
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
EXPR="grep -q 'environment=' <<< \"\$COMMAND_OUTPUT\" && grep -q 'workspace=' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 3: Generate test configuration file"
else
    echo "[FAIL] Step 3: Generate test configuration file"
    echo "  Command: cat > ${run_directory}/config.txt << EOF environment=${TEST_ENVIRONMENT} workspace=${TEST_WORKSPACE} start_time=${test_start_time} max_retries=${MAX_RETRIES} EOF cat ${run_directory}/config.txt"
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
    echo '    "command": "cat > ${run_directory}/config.txt << EOF environment=${TEST_ENVIRONMENT} workspace=${TEST_WORKSPACE} start_time=${test_start_time} max_retries=${MAX_RETRIES} EOF cat ${run_directory}/config.txt",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

STEP_EXIT_CODE=$EXIT_CODE
export STEP_EXIT_CODE COMMAND_OUTPUT
# Execute after_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/after_step_metrics.sh" ]; then
    source "test-acceptance/scripts/hooks/after_step_metrics.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/after_step_metrics.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: after_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

# Step 4: Extract and verify configuration values
TEST_STEP_NUMBER=4
TEST_STEP_DESCRIPTION='Extract and verify configuration values'
export TEST_STEP_NUMBER TEST_STEP_DESCRIPTION
# Execute before_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/before_step_validate.sh" ]; then
    source "test-acceptance/scripts/hooks/before_step_validate.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/before_step_validate.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: before_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

LOG_FILE="TC_COMPLEX_MULTI_SEQ_HOOKS_001_sequence-1_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="grep 'max_retries=' \${run_directory}/config.txt | cut -d= -f2"
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
STEP_VAR_configured_retries=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " configured_retries "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES configured_retries"
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
GENERAL_VERIFY_PASS_verify_retries_configured=false
EXPR="[[ \$configured_retries -gt 0 ]]"
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
    GENERAL_VERIFY_PASS_verify_retries_configured=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_retries_configured" = true ]; then
    echo "[PASS] Step 4: Extract and verify configuration values"
else
    echo "[FAIL] Step 4: Extract and verify configuration values"
    echo "  Command: grep 'max_retries=' ${run_directory}/config.txt | cut -d= -f2"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_retries_configured: $GENERAL_VERIFY_PASS_verify_retries_configured"
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
    echo '    "command": "grep \"max_retries=\" ${run_directory}/config.txt | cut -d= -f2",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

STEP_EXIT_CODE=$EXIT_CODE
export STEP_EXIT_CODE COMMAND_OUTPUT
# Execute after_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/after_step_metrics.sh" ]; then
    source "test-acceptance/scripts/hooks/after_step_metrics.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/after_step_metrics.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: after_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

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

# Test Sequence 2: Manual Verification and Data Processing
# Manual steps interspersed with automated processing and variable usage
TEST_SEQUENCE_ID=2
TEST_SEQUENCE_NAME='Manual Verification and Data Processing'
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
# filesystem: Run directory and config file exist from sequence 1
# system: Captured variables are available for use

# Step 1: Verify test workspace was created correctly
TEST_STEP_NUMBER=1
TEST_STEP_DESCRIPTION='Verify test workspace was created correctly'
export TEST_STEP_NUMBER TEST_STEP_DESCRIPTION
# Execute before_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/before_step_validate.sh" ]; then
    source "test-acceptance/scripts/hooks/before_step_validate.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/before_step_validate.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: before_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

echo "Step 1: Verify test workspace was created correctly"
echo "Command: Navigate to the test workspace directory and verify config.txt exists"
echo "INFO: This is a manual step. You must perform this action manually."
# Prompt user to confirm manual action completion
if read_true_false "Have you completed the manual action?"; then
    # User confirmed (read_true_false returns 0 for yes)
    :
else
    echo "Manual action not completed. Exiting." >&2
    exit 1
fi

# Manual step verification
USER_VERIFICATION_RESULT=false
USER_VERIFICATION_OUTPUT=false
USER_VERIFICATION_RESULT=true
EXPR="Can you confirm config.txt exists in the workspace? (Y/n)"
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
    USER_VERIFICATION_OUTPUT=true
fi

# Set USER_VERIFICATION based on verification results
if [ "$USER_VERIFICATION_RESULT" = true ] && [ "$USER_VERIFICATION_OUTPUT" = true ]; then
    USER_VERIFICATION=true
else
    USER_VERIFICATION=false
fi

if [ "$USER_VERIFICATION" = true ]; then
    echo "[PASS] Step 1: Verify test workspace was created correctly"
else
    echo "[FAIL] Step 1: Verify test workspace was created correctly"
    echo "  Result verification: $USER_VERIFICATION_RESULT"
    echo "  Output verification: $USER_VERIFICATION_OUTPUT"
    exit 1
fi

STEP_EXIT_CODE=0
COMMAND_OUTPUT=""
export STEP_EXIT_CODE COMMAND_OUTPUT
# Execute after_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/after_step_metrics.sh" ]; then
    source "test-acceptance/scripts/hooks/after_step_metrics.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/after_step_metrics.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: after_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

# Step 2: Create test data file with sequence information
TEST_STEP_NUMBER=2
TEST_STEP_DESCRIPTION='Create test data file with sequence information'
export TEST_STEP_NUMBER TEST_STEP_DESCRIPTION
# Execute before_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/before_step_validate.sh" ]; then
    source "test-acceptance/scripts/hooks/before_step_validate.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/before_step_validate.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: before_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

LOG_FILE="TC_COMPLEX_MULTI_SEQ_HOOKS_001_sequence-2_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo 'Sequence: 2' > \${run_directory}/sequence_2_data.txt && echo 'Timestamp: '\${test_start_time} >> \${run_directory}/sequence_2_data.txt && cat \${run_directory}/sequence_2_data.txt"
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
EXPR="grep -q 'Sequence: 2' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 2: Create test data file with sequence information"
else
    echo "[FAIL] Step 2: Create test data file with sequence information"
    echo "  Command: echo 'Sequence: 2' > ${run_directory}/sequence_2_data.txt && echo 'Timestamp: '${test_start_time} >> ${run_directory}/sequence_2_data.txt && cat ${run_directory}/sequence_2_data.txt"
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
    echo '    "command": "echo \"Sequence: 2\" > ${run_directory}/sequence_2_data.txt && echo \"Timestamp: \"${test_start_time} >> ${run_directory}/sequence_2_data.txt && cat ${run_directory}/sequence_2_data.txt",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

STEP_EXIT_CODE=$EXIT_CODE
export STEP_EXIT_CODE COMMAND_OUTPUT
# Execute after_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/after_step_metrics.sh" ]; then
    source "test-acceptance/scripts/hooks/after_step_metrics.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/after_step_metrics.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: after_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

# Step 3: Process data and capture results
TEST_STEP_NUMBER=3
TEST_STEP_DESCRIPTION='Process data and capture results'
export TEST_STEP_NUMBER TEST_STEP_DESCRIPTION
# Execute before_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/before_step_validate.sh" ]; then
    source "test-acceptance/scripts/hooks/before_step_validate.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/before_step_validate.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: before_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

LOG_FILE="TC_COMPLEX_MULTI_SEQ_HOOKS_001_sequence-2_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="wc -l \${run_directory}/*.txt | tail -1 | awk '{print \$1}'"
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
STEP_VAR_total_lines=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " total_lines "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES total_lines"
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
GENERAL_VERIFY_PASS_verify_lines_counted=false
EXPR="[[ \$total_lines -gt 0 ]]"
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
    GENERAL_VERIFY_PASS_verify_lines_counted=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_lines_counted" = true ]; then
    echo "[PASS] Step 3: Process data and capture results"
else
    echo "[FAIL] Step 3: Process data and capture results"
    echo "  Command: wc -l ${run_directory}/*.txt | tail -1 | awk '{print $1}'"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_lines_counted: $GENERAL_VERIFY_PASS_verify_lines_counted"
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
    echo '    "command": "wc -l ${run_directory}/*.txt | tail -1 | awk \"{print $1}\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

STEP_EXIT_CODE=$EXIT_CODE
export STEP_EXIT_CODE COMMAND_OUTPUT
# Execute after_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/after_step_metrics.sh" ]; then
    source "test-acceptance/scripts/hooks/after_step_metrics.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/after_step_metrics.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: after_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

# Step 4: Verify file count and content manually
TEST_STEP_NUMBER=4
TEST_STEP_DESCRIPTION='Verify file count and content manually'
export TEST_STEP_NUMBER TEST_STEP_DESCRIPTION
# Execute before_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/before_step_validate.sh" ]; then
    source "test-acceptance/scripts/hooks/before_step_validate.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/before_step_validate.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: before_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

echo "Step 4: Verify file count and content manually"
echo "Command: List files in run directory and verify their contents"
echo "INFO: This is a manual step. You must perform this action manually."
# Prompt user to confirm manual action completion
if read_true_false "Have you completed the manual action?"; then
    # User confirmed (read_true_false returns 0 for yes)
    :
else
    echo "Manual action not completed. Exiting." >&2
    exit 1
fi

# Manual step verification
USER_VERIFICATION_RESULT=false
USER_VERIFICATION_OUTPUT=false
USER_VERIFICATION_RESULT=true
EXPR="Are both files present with readable content? (Y/n)"
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
    USER_VERIFICATION_OUTPUT=true
fi

# Set USER_VERIFICATION based on verification results
if [ "$USER_VERIFICATION_RESULT" = true ] && [ "$USER_VERIFICATION_OUTPUT" = true ]; then
    USER_VERIFICATION=true
else
    USER_VERIFICATION=false
fi

if [ "$USER_VERIFICATION" = true ]; then
    echo "[PASS] Step 4: Verify file count and content manually"
else
    echo "[FAIL] Step 4: Verify file count and content manually"
    echo "  Result verification: $USER_VERIFICATION_RESULT"
    echo "  Output verification: $USER_VERIFICATION_OUTPUT"
    exit 1
fi

STEP_EXIT_CODE=0
COMMAND_OUTPUT=""
export STEP_EXIT_CODE COMMAND_OUTPUT
# Execute after_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/after_step_metrics.sh" ]; then
    source "test-acceptance/scripts/hooks/after_step_metrics.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/after_step_metrics.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: after_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

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

# Test Sequence 3: Conditional Verification and Cleanup
# Complex conditional logic with captured variables and final cleanup
TEST_SEQUENCE_ID=3
TEST_SEQUENCE_NAME='Conditional Verification and Cleanup'
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
# filesystem: All data files from previous sequences exist
# variables: All captured variables are in scope

# Step 1: Calculate test duration
TEST_STEP_NUMBER=1
TEST_STEP_DESCRIPTION='Calculate test duration'
export TEST_STEP_NUMBER TEST_STEP_DESCRIPTION
# Execute before_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/before_step_validate.sh" ]; then
    source "test-acceptance/scripts/hooks/before_step_validate.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/before_step_validate.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: before_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

LOG_FILE="TC_COMPLEX_MULTI_SEQ_HOOKS_001_sequence-3_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ date +%s; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_test_end_time=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " test_end_time "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES test_end_time"
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
    echo "[PASS] Step 1: Calculate test duration"
else
    echo "[FAIL] Step 1: Calculate test duration"
    echo "  Command: date +%s"
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
    echo '    "command": "date +%s",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

STEP_EXIT_CODE=$EXIT_CODE
export STEP_EXIT_CODE COMMAND_OUTPUT
# Execute after_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/after_step_metrics.sh" ]; then
    source "test-acceptance/scripts/hooks/after_step_metrics.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/after_step_metrics.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: after_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

# Step 2: Verify test execution duration with conditional logic
TEST_STEP_NUMBER=2
TEST_STEP_DESCRIPTION='Verify test execution duration with conditional logic'
export TEST_STEP_NUMBER TEST_STEP_DESCRIPTION
# Execute before_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/before_step_validate.sh" ]; then
    source "test-acceptance/scripts/hooks/before_step_validate.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/before_step_validate.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: before_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

LOG_FILE="TC_COMPLEX_MULTI_SEQ_HOOKS_001_sequence-3_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo \$((\${test_end_time} - \${test_start_time}))"
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
STEP_VAR_test_duration=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " test_duration "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES test_duration"
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
COND_EXPR="[[ \$test_duration -lt 300 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        COND_EXPR=$(echo "$COND_EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$COND_EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
    echo 'Test completed within acceptable time (<5 minutes)'
else
    echo 'Warning: Test took longer than expected'
fi
echo "Total duration: ${test_duration} seconds"

# General verifications
GENERAL_VERIFY_PASS_verify_duration_numeric=false
EXPR="[[ \$test_duration =~ ^[0-9]+\$ ]]"
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
    GENERAL_VERIFY_PASS_verify_duration_numeric=true
fi
GENERAL_VERIFY_PASS_verify_duration_positive=false
EXPR="[[ \$test_duration -ge 0 ]]"
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
    GENERAL_VERIFY_PASS_verify_duration_positive=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_duration_numeric" = true ] && [ "$GENERAL_VERIFY_PASS_verify_duration_positive" = true ]; then
    echo "[PASS] Step 2: Verify test execution duration with conditional logic"
else
    echo "[FAIL] Step 2: Verify test execution duration with conditional logic"
    echo "  Command: echo $((${test_end_time} - ${test_start_time}))"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_duration_numeric: $GENERAL_VERIFY_PASS_verify_duration_numeric"
    echo "  GENERAL_VERIFY_PASS_verify_duration_positive: $GENERAL_VERIFY_PASS_verify_duration_positive"
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
    echo '    "command": "echo $((${test_end_time} - ${test_start_time}))",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

STEP_EXIT_CODE=$EXIT_CODE
export STEP_EXIT_CODE COMMAND_OUTPUT
# Execute after_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/after_step_metrics.sh" ]; then
    source "test-acceptance/scripts/hooks/after_step_metrics.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/after_step_metrics.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: after_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

# Step 3: Create test summary report
TEST_STEP_NUMBER=3
TEST_STEP_DESCRIPTION='Create test summary report'
export TEST_STEP_NUMBER TEST_STEP_DESCRIPTION
# Execute before_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/before_step_validate.sh" ]; then
    source "test-acceptance/scripts/hooks/before_step_validate.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/before_step_validate.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: before_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

LOG_FILE="TC_COMPLEX_MULTI_SEQ_HOOKS_001_sequence-3_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="cat > \${run_directory}/summary.txt << EOF Test Execution Summary ====================== Start Time: \${test_start_time} End Time: \${test_end_time} Duration: \${test_duration} seconds Total Lines: \${total_lines} Environment: \${TEST_ENVIRONMENT} Retries Configured: \${configured_retries} Status: COMPLETE EOF cat \${run_directory}/summary.txt"
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
EXPR="grep -q 'Test Execution Summary' <<< \"\$COMMAND_OUTPUT\" && grep -q 'Status: COMPLETE' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 3: Create test summary report"
else
    echo "[FAIL] Step 3: Create test summary report"
    echo "  Command: cat > ${run_directory}/summary.txt << EOF Test Execution Summary ====================== Start Time: ${test_start_time} End Time: ${test_end_time} Duration: ${test_duration} seconds Total Lines: ${total_lines} Environment: ${TEST_ENVIRONMENT} Retries Configured: ${configured_retries} Status: COMPLETE EOF cat ${run_directory}/summary.txt"
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
    echo '    "command": "cat > ${run_directory}/summary.txt << EOF Test Execution Summary ====================== Start Time: ${test_start_time} End Time: ${test_end_time} Duration: ${test_duration} seconds Total Lines: ${total_lines} Environment: ${TEST_ENVIRONMENT} Retries Configured: ${configured_retries} Status: COMPLETE EOF cat ${run_directory}/summary.txt",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

STEP_EXIT_CODE=$EXIT_CODE
export STEP_EXIT_CODE COMMAND_OUTPUT
# Execute after_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/after_step_metrics.sh" ]; then
    source "test-acceptance/scripts/hooks/after_step_metrics.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/after_step_metrics.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: after_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

# Step 4: Review and verify test summary
TEST_STEP_NUMBER=4
TEST_STEP_DESCRIPTION='Review and verify test summary'
export TEST_STEP_NUMBER TEST_STEP_DESCRIPTION
# Execute before_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/before_step_validate.sh" ]; then
    source "test-acceptance/scripts/hooks/before_step_validate.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/before_step_validate.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: before_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

echo "Step 4: Review and verify test summary"
echo "Command: Review the summary.txt file for accuracy and completeness"
echo "INFO: This is a manual step. You must perform this action manually."
# Prompt user to confirm manual action completion
if read_true_false "Have you completed the manual action?"; then
    # User confirmed (read_true_false returns 0 for yes)
    :
else
    echo "Manual action not completed. Exiting." >&2
    exit 1
fi

# Manual step verification
USER_VERIFICATION_RESULT=false
USER_VERIFICATION_OUTPUT=false
USER_VERIFICATION_RESULT=true
EXPR="Does the summary show complete and accurate test information? (Y/n)"
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
    USER_VERIFICATION_OUTPUT=true
fi

# Set USER_VERIFICATION based on verification results
if [ "$USER_VERIFICATION_RESULT" = true ] && [ "$USER_VERIFICATION_OUTPUT" = true ]; then
    USER_VERIFICATION=true
else
    USER_VERIFICATION=false
fi

if [ "$USER_VERIFICATION" = true ]; then
    echo "[PASS] Step 4: Review and verify test summary"
else
    echo "[FAIL] Step 4: Review and verify test summary"
    echo "  Result verification: $USER_VERIFICATION_RESULT"
    echo "  Output verification: $USER_VERIFICATION_OUTPUT"
    exit 1
fi

STEP_EXIT_CODE=0
COMMAND_OUTPUT=""
export STEP_EXIT_CODE COMMAND_OUTPUT
# Execute after_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/after_step_metrics.sh" ]; then
    source "test-acceptance/scripts/hooks/after_step_metrics.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/after_step_metrics.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: after_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

# Step 5: Archive test results
TEST_STEP_NUMBER=5
TEST_STEP_DESCRIPTION='Archive test results'
export TEST_STEP_NUMBER TEST_STEP_DESCRIPTION
# Execute before_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/before_step_validate.sh" ]; then
    source "test-acceptance/scripts/hooks/before_step_validate.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/before_step_validate.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: before_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

LOG_FILE="TC_COMPLEX_MULTI_SEQ_HOOKS_001_sequence-3_step-5.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="tar -czf \${TEST_WORKSPACE}/test_results_\${test_start_time}.tar.gz -C \${TEST_WORKSPACE} run_\${test_start_time} 2>&1 | head -1 || echo 'Archive created'"
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
EXPR="true"
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
    echo "[PASS] Step 5: Archive test results"
else
    echo "[FAIL] Step 5: Archive test results"
    echo "  Command: tar -czf ${TEST_WORKSPACE}/test_results_${test_start_time}.tar.gz -C ${TEST_WORKSPACE} run_${test_start_time} 2>&1 | head -1 || echo 'Archive created'"
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
    echo '    "command": "tar -czf ${TEST_WORKSPACE}/test_results_${test_start_time}.tar.gz -C ${TEST_WORKSPACE} run_${test_start_time} 2>&1 | head -1 || echo \"Archive created\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

STEP_EXIT_CODE=$EXIT_CODE
export STEP_EXIT_CODE COMMAND_OUTPUT
# Execute after_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/after_step_metrics.sh" ]; then
    source "test-acceptance/scripts/hooks/after_step_metrics.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/after_step_metrics.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: after_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

# Step 6: Verify archive and cleanup workspace
TEST_STEP_NUMBER=6
TEST_STEP_DESCRIPTION='Verify archive and cleanup workspace'
export TEST_STEP_NUMBER TEST_STEP_DESCRIPTION
# Execute before_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/before_step_validate.sh" ]; then
    source "test-acceptance/scripts/hooks/before_step_validate.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/before_step_validate.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: before_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

LOG_FILE="TC_COMPLEX_MULTI_SEQ_HOOKS_001_sequence-3_step-6.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="test -f \${TEST_WORKSPACE}/test_results_\${test_start_time}.tar.gz && echo 'Archive verified' && rm -rf \${run_directory} && echo 'Cleanup complete'"
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
EXPR="grep -q 'Archive verified' <<< \"\$COMMAND_OUTPUT\" && grep -q 'Cleanup complete' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 6: Verify archive and cleanup workspace"
else
    echo "[FAIL] Step 6: Verify archive and cleanup workspace"
    echo "  Command: test -f ${TEST_WORKSPACE}/test_results_${test_start_time}.tar.gz && echo 'Archive verified' && rm -rf ${run_directory} && echo 'Cleanup complete'"
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
    echo '    "step": 6,'
    echo '    "command": "test -f ${TEST_WORKSPACE}/test_results_${test_start_time}.tar.gz && echo \"Archive verified\" && rm -rf ${run_directory} && echo \"Cleanup complete\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

STEP_EXIT_CODE=$EXIT_CODE
export STEP_EXIT_CODE COMMAND_OUTPUT
# Execute after_step hook
set +e
if [ -f "test-acceptance/scripts/hooks/after_step_metrics.sh" ]; then
    source "test-acceptance/scripts/hooks/after_step_metrics.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/after_step_metrics.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: after_step hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

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
# Execute script_end hook
set +e
if [ -f "test-acceptance/scripts/hooks/script_end_summary.sh" ]; then
    source "test-acceptance/scripts/hooks/script_end_summary.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'test-acceptance/scripts/hooks/script_end_summary.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
set -e
if [ $HOOK_EXIT_CODE -ne 0 ]; then
    echo "Warning: script_end hook failed with exit code $HOOK_EXIT_CODE (continuing)" >&2
fi

exit 0
