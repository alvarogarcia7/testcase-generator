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

# Test Case: TC_COMPLEX_ALL_HOOKS_CAPTURE_001
# Description: Comprehensive test with all 8 hook types and extensive variable capture - complete lifecycle demonstration

JSON_LOG="TC_COMPLEX_ALL_HOOKS_CAPTURE_001_execution_log.json"
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
# system: All 8 hook scripts are available and executable
# system: Hook execution framework is operational
# hooks: script_start hook initializes global state
# hooks: setup_test hook prepares test infrastructure
# hooks: All hooks have proper error handling configured
# environment: Test execution environment supports lifecycle hooks

# Initial Conditions
# filesystem: Temporary directories can be created
# filesystem: Hook scripts have write permissions
# hooks: Hooks can access environment variables
# hooks: Hooks can access test context

# Test Sequence 1: Hook Lifecycle - Script Start and Setup
# Demonstrate script_start and setup_test hooks with variable initialization
TEST_SEQUENCE_ID=1
TEST_SEQUENCE_NAME='Hook Lifecycle - Script Start and Setup'
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
# variables: Hydration variables are available
# hooks: script_start hook has executed
# hooks: setup_test hook has prepared workspace

# Step 1: Capture script start timestamp (demonstrates before_step and after_step hooks)
TEST_STEP_NUMBER=1
TEST_STEP_DESCRIPTION='Capture script start timestamp (demonstrates before_step and after_step hooks)'
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

LOG_FILE="TC_COMPLEX_ALL_HOOKS_CAPTURE_001_sequence-1_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ date +%s; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_script_start_time=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " script_start_time "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES script_start_time"
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
GENERAL_VERIFY_PASS_verify_script_start_time=false
EXPR="[[ \$script_start_time =~ ^[0-9]+\$ ]]"
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
    GENERAL_VERIFY_PASS_verify_script_start_time=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_script_start_time" = true ]; then
    echo "[PASS] Step 1: Capture script start timestamp (demonstrates before_step and after_step hooks)"
else
    echo "[FAIL] Step 1: Capture script start timestamp (demonstrates before_step and after_step hooks)"
    echo "  Command: date +%s"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_script_start_time: $GENERAL_VERIFY_PASS_verify_script_start_time"
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

# Step 2: Create hook execution log directory
TEST_STEP_NUMBER=2
TEST_STEP_DESCRIPTION='Create hook execution log directory'
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

LOG_FILE="TC_COMPLEX_ALL_HOOKS_CAPTURE_001_sequence-1_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="mkdir -p /tmp/hook_logs_\${TEST_ID} && echo '/tmp/hook_logs_'\${TEST_ID}"
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
STEP_VAR_hook_log_dir=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\((/tmp/hook_logs_[A-Z_0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " hook_log_dir "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES hook_log_dir"
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
EXPR="grep -q '/tmp/hook_logs_' <<< \"\$COMMAND_OUTPUT\""
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
GENERAL_VERIFY_PASS_verify_hook_log_dir_captured=false
EXPR="[[ -n \"\$hook_log_dir\" ]]"
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
    GENERAL_VERIFY_PASS_verify_hook_log_dir_captured=true
fi
GENERAL_VERIFY_PASS_verify_hook_log_dir_exists=false
EXPR="[[ -d \"\$hook_log_dir\" ]]"
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
    GENERAL_VERIFY_PASS_verify_hook_log_dir_exists=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_hook_log_dir_captured" = true ] && [ "$GENERAL_VERIFY_PASS_verify_hook_log_dir_exists" = true ]; then
    echo "[PASS] Step 2: Create hook execution log directory"
else
    echo "[FAIL] Step 2: Create hook execution log directory"
    echo "  Command: mkdir -p /tmp/hook_logs_${TEST_ID} && echo '/tmp/hook_logs_'${TEST_ID}"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_hook_log_dir_captured: $GENERAL_VERIFY_PASS_verify_hook_log_dir_captured"
    echo "  GENERAL_VERIFY_PASS_verify_hook_log_dir_exists: $GENERAL_VERIFY_PASS_verify_hook_log_dir_exists"
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
    echo '    "command": "mkdir -p /tmp/hook_logs_${TEST_ID} && echo \"/tmp/hook_logs_\"${TEST_ID}",'
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

# Step 3: Initialize hook tracking file
TEST_STEP_NUMBER=3
TEST_STEP_DESCRIPTION='Initialize hook tracking file'
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

LOG_FILE="TC_COMPLEX_ALL_HOOKS_CAPTURE_001_sequence-1_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="cat > \${hook_log_dir}/hooks_executed.log << EOF # Hook Execution Log for \${TEST_ID} # Started at: \${script_start_time} # Log Level: \${LOG_LEVEL} # Metrics: \${METRICS_ENABLED}
HOOK_EXECUTION_START=\${script_start_time} EOF cat \${hook_log_dir}/hooks_executed.log"
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
EXPR="grep -q 'Hook Execution Log' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 3: Initialize hook tracking file"
else
    echo "[FAIL] Step 3: Initialize hook tracking file"
    echo "  Command: cat > ${hook_log_dir}/hooks_executed.log << EOF # Hook Execution Log for ${TEST_ID} # Started at: ${script_start_time} # Log Level: ${LOG_LEVEL} # Metrics: ${METRICS_ENABLED}
HOOK_EXECUTION_START=${script_start_time} EOF cat ${hook_log_dir}/hooks_executed.log"
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
    echo '    "command": "cat > ${hook_log_dir}/hooks_executed.log << EOF # Hook Execution Log for ${TEST_ID} # Started at: ${script_start_time} # Log Level: ${LOG_LEVEL} # Metrics: ${METRICS_ENABLED}\nHOOK_EXECUTION_START=${script_start_time} EOF cat ${hook_log_dir}/hooks_executed.log",'
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

# Step 4: Record that before_sequence hook should have executed
TEST_STEP_NUMBER=4
TEST_STEP_DESCRIPTION='Record that before_sequence hook should have executed'
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

LOG_FILE="TC_COMPLEX_ALL_HOOKS_CAPTURE_001_sequence-1_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo 'before_sequence: Sequence 1 executed' >> \${hook_log_dir}/hooks_executed.log && tail -1 \${hook_log_dir}/hooks_executed.log"
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
EXPR="grep -q 'before_sequence: Sequence 1 executed' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 4: Record that before_sequence hook should have executed"
else
    echo "[FAIL] Step 4: Record that before_sequence hook should have executed"
    echo "  Command: echo 'before_sequence: Sequence 1 executed' >> ${hook_log_dir}/hooks_executed.log && tail -1 ${hook_log_dir}/hooks_executed.log"
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
    echo '    "command": "echo \"before_sequence: Sequence 1 executed\" >> ${hook_log_dir}/hooks_executed.log && tail -1 ${hook_log_dir}/hooks_executed.log",'
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

# Test Sequence 2: Hook Lifecycle - Sequence Transitions
# Demonstrate before_sequence and after_sequence hooks between sequences
TEST_SEQUENCE_ID=2
TEST_SEQUENCE_NAME='Hook Lifecycle - Sequence Transitions'
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
# variables: Variables from sequence 1 are available
# hooks: after_sequence hook from sequence 1 has executed
# hooks: before_sequence hook for sequence 2 is about to execute

# Step 1: Capture sequence 2 start time
TEST_STEP_NUMBER=1
TEST_STEP_DESCRIPTION='Capture sequence 2 start time'
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

LOG_FILE="TC_COMPLEX_ALL_HOOKS_CAPTURE_001_sequence-2_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ date +%s; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_seq2_start_time=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " seq2_start_time "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES seq2_start_time"
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
    echo "[PASS] Step 1: Capture sequence 2 start time"
else
    echo "[FAIL] Step 1: Capture sequence 2 start time"
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
    echo '    "test_sequence": 2,'
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

# Step 2: Record sequence transition in hook log
TEST_STEP_NUMBER=2
TEST_STEP_DESCRIPTION='Record sequence transition in hook log'
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

LOG_FILE="TC_COMPLEX_ALL_HOOKS_CAPTURE_001_sequence-2_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo 'SEQUENCE_TRANSITION: 1 -> 2 at \${seq2_start_time}' >> \${hook_log_dir}/hooks_executed.log && echo 'after_sequence: Sequence 1 completed' >> \${hook_log_dir}/hooks_executed.log && echo 'before_sequence: Sequence 2 started' >> \${hook_log_dir}/hooks_executed.log && tail -3 \${hook_log_dir}/hooks_executed.log"
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
EXPR="grep -q 'SEQUENCE_TRANSITION' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 2: Record sequence transition in hook log"
else
    echo "[FAIL] Step 2: Record sequence transition in hook log"
    echo "  Command: echo 'SEQUENCE_TRANSITION: 1 -> 2 at ${seq2_start_time}' >> ${hook_log_dir}/hooks_executed.log && echo 'after_sequence: Sequence 1 completed' >> ${hook_log_dir}/hooks_executed.log && echo 'before_sequence: Sequence 2 started' >> ${hook_log_dir}/hooks_executed.log && tail -3 ${hook_log_dir}/hooks_executed.log"
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
    echo '    "command": "echo \"SEQUENCE_TRANSITION: 1 -> 2 at ${seq2_start_time}\" >> ${hook_log_dir}/hooks_executed.log && echo \"after_sequence: Sequence 1 completed\" >> ${hook_log_dir}/hooks_executed.log && echo \"before_sequence: Sequence 2 started\" >> ${hook_log_dir}/hooks_executed.log && tail -3 ${hook_log_dir}/hooks_executed.log",'
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

# Step 3: Count before_step and after_step hook executions for sequence 1
TEST_STEP_NUMBER=3
TEST_STEP_DESCRIPTION='Count before_step and after_step hook executions for sequence 1'
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

LOG_FILE="TC_COMPLEX_ALL_HOOKS_CAPTURE_001_sequence-2_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ echo 4; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_seq1_step_count=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " seq1_step_count "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES seq1_step_count"
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
GENERAL_VERIFY_PASS_verify_seq1_steps=false
EXPR="[[ \$seq1_step_count -eq 4 ]]"
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
    GENERAL_VERIFY_PASS_verify_seq1_steps=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_seq1_steps" = true ]; then
    echo "[PASS] Step 3: Count before_step and after_step hook executions for sequence 1"
else
    echo "[FAIL] Step 3: Count before_step and after_step hook executions for sequence 1"
    echo "  Command: echo 4"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_seq1_steps: $GENERAL_VERIFY_PASS_verify_seq1_steps"
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
    echo '    "command": "echo 4",'
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

# Step 4: Calculate time elapsed since script start
TEST_STEP_NUMBER=4
TEST_STEP_DESCRIPTION='Calculate time elapsed since script start'
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

LOG_FILE="TC_COMPLEX_ALL_HOOKS_CAPTURE_001_sequence-2_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo \$((\${seq2_start_time} - \${script_start_time}))"
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
STEP_VAR_elapsed_time=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " elapsed_time "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES elapsed_time"
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
COND_EXPR="[[ \$elapsed_time -ge 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        COND_EXPR=$(echo "$COND_EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$COND_EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
    echo 'Elapsed time is valid'
else
    echo 'Elapsed time calculation error'
fi
echo "Time since start: ${elapsed_time}s"

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 4: Calculate time elapsed since script start"
else
    echo "[FAIL] Step 4: Calculate time elapsed since script start"
    echo "  Command: echo $((${seq2_start_time} - ${script_start_time}))"
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
    echo '    "command": "echo $((${seq2_start_time} - ${script_start_time}))",'
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

# Test Sequence 3: Hook Lifecycle - Step-Level Hooks
# Intensive step execution demonstrating before_step and after_step hooks
TEST_SEQUENCE_ID=3
TEST_SEQUENCE_NAME='Hook Lifecycle - Step-Level Hooks'
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
# hooks: before_step hook executes before each step
# hooks: after_step hook executes after each step
# metrics: Metrics collection is active if enabled

# Step 1: Generate random operation ID (after_step will record this)
TEST_STEP_NUMBER=1
TEST_STEP_DESCRIPTION='Generate random operation ID (after_step will record this)'
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

LOG_FILE="TC_COMPLEX_ALL_HOOKS_CAPTURE_001_sequence-3_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ echo 'OP_'$RANDOM; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_operation_id=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\((OP_[0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " operation_id "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES operation_id"
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
EXPR="[[ \"\$COMMAND_OUTPUT\" =~ ^OP_[0-9]+\$ ]]"
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
    echo "[PASS] Step 1: Generate random operation ID (after_step will record this)"
else
    echo "[FAIL] Step 1: Generate random operation ID (after_step will record this)"
    echo "  Command: echo 'OP_'$RANDOM"
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
    echo '    "command": "echo \"OP_\"$RANDOM",'
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

# Step 2: Simulate operation with metrics collection
TEST_STEP_NUMBER=2
TEST_STEP_DESCRIPTION='Simulate operation with metrics collection'
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

LOG_FILE="TC_COMPLEX_ALL_HOOKS_CAPTURE_001_sequence-3_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo 'Operation \${operation_id} processing' && sleep 0.1 && echo 'Operation \${operation_id} complete'"
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
STEP_VAR_operation_status=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(Operation [^ ]+ ([a-z]+)$\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " operation_status "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES operation_status"
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

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 2: Simulate operation with metrics collection"
else
    echo "[FAIL] Step 2: Simulate operation with metrics collection"
    echo "  Command: echo 'Operation ${operation_id} processing' && sleep 0.1 && echo 'Operation ${operation_id} complete'"
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
    echo '    "command": "echo \"Operation ${operation_id} processing\" && sleep 0.1 && echo \"Operation ${operation_id} complete\"",'
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

# Step 3: Record operation in metrics log
TEST_STEP_NUMBER=3
TEST_STEP_DESCRIPTION='Record operation in metrics log'
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

LOG_FILE="TC_COMPLEX_ALL_HOOKS_CAPTURE_001_sequence-3_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="if [[ \"\${METRICS_ENABLED}\" == \"true\" ]]; then echo 'METRIC: \${operation_id} - Status: \${operation_status}' >> \${hook_log_dir}/metrics.log echo 'Metrics recorded' else echo 'Metrics disabled' fi"
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
EXPR="grep -q 'Metrics' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 3: Record operation in metrics log"
else
    echo "[FAIL] Step 3: Record operation in metrics log"
    echo "  Command: if [[ \"${METRICS_ENABLED}\" == \"true\" ]]; then echo 'METRIC: ${operation_id} - Status: ${operation_status}' >> ${hook_log_dir}/metrics.log echo 'Metrics recorded' else echo 'Metrics disabled' fi"
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
    echo '    "command": "if [[ \"${METRICS_ENABLED}\" == \"true\" ]]; then echo \"METRIC: ${operation_id} - Status: ${operation_status}\" >> ${hook_log_dir}/metrics.log echo \"Metrics recorded\" else echo \"Metrics disabled\" fi",'
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

# Step 4: Count total steps executed so far
TEST_STEP_NUMBER=4
TEST_STEP_DESCRIPTION='Count total steps executed so far'
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

LOG_FILE="TC_COMPLEX_ALL_HOOKS_CAPTURE_001_sequence-3_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo \$((\${seq1_step_count} + 4 + 3))"
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
STEP_VAR_total_steps_so_far=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " total_steps_so_far "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES total_steps_so_far"
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
GENERAL_VERIFY_PASS_verify_step_count=false
EXPR="[[ \$total_steps_so_far -ge 11 ]]"
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
    GENERAL_VERIFY_PASS_verify_step_count=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_step_count" = true ]; then
    echo "[PASS] Step 4: Count total steps executed so far"
else
    echo "[FAIL] Step 4: Count total steps executed so far"
    echo "  Command: echo $((${seq1_step_count} + 4 + 3))"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_step_count: $GENERAL_VERIFY_PASS_verify_step_count"
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
    echo '    "command": "echo $((${seq1_step_count} + 4 + 3))",'
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

# Step 5: Record step hook executions (before_step and after_step for each)
TEST_STEP_NUMBER=5
TEST_STEP_DESCRIPTION='Record step hook executions (before_step and after_step for each)'
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

LOG_FILE="TC_COMPLEX_ALL_HOOKS_CAPTURE_001_sequence-3_step-5.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo 'before_step executions: '\${total_steps_so_far} && echo 'after_step executions: '\${total_steps_so_far}"
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
STEP_VAR_hook_execution_count=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(executions: ([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " hook_execution_count "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES hook_execution_count"
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
EXPR="grep -q 'before_step executions' <<< \"\$COMMAND_OUTPUT\" && grep -q 'after_step executions' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 5: Record step hook executions (before_step and after_step for each)"
else
    echo "[FAIL] Step 5: Record step hook executions (before_step and after_step for each)"
    echo "  Command: echo 'before_step executions: '${total_steps_so_far} && echo 'after_step executions: '${total_steps_so_far}"
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
    echo '    "command": "echo \"before_step executions: \"${total_steps_so_far} && echo \"after_step executions: \"${total_steps_so_far}",'
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

# Test Sequence 4: Hook Lifecycle - Teardown and Completion
# Demonstrate teardown_test and script_end hooks with final reporting
TEST_SEQUENCE_ID=4
TEST_SEQUENCE_NAME='Hook Lifecycle - Teardown and Completion'
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
# variables: All captured variables from previous sequences are available
# hooks: All step and sequence hooks have executed multiple times
# hooks: teardown_test hook will execute after this sequence
# hooks: script_end hook will execute at the very end

# Step 1: Capture final timestamp before teardown
TEST_STEP_NUMBER=1
TEST_STEP_DESCRIPTION='Capture final timestamp before teardown'
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

LOG_FILE="TC_COMPLEX_ALL_HOOKS_CAPTURE_001_sequence-4_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ date +%s; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_teardown_start_time=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " teardown_start_time "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES teardown_start_time"
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
    echo "[PASS] Step 1: Capture final timestamp before teardown"
else
    echo "[FAIL] Step 1: Capture final timestamp before teardown"
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
    echo '    "test_sequence": 4,'
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

# Step 2: Calculate total test execution time
TEST_STEP_NUMBER=2
TEST_STEP_DESCRIPTION='Calculate total test execution time'
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

LOG_FILE="TC_COMPLEX_ALL_HOOKS_CAPTURE_001_sequence-4_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo \$((\${teardown_start_time} - \${script_start_time}))"
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
STEP_VAR_total_execution_time=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " total_execution_time "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES total_execution_time"
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
COND_EXPR="[[ \$total_execution_time -ge 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        COND_EXPR=$(echo "$COND_EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$COND_EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
    echo 'Total execution time is valid'
else
    echo 'Execution time error'
fi
echo "Total time: ${total_execution_time}s"

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 2: Calculate total test execution time"
else
    echo "[FAIL] Step 2: Calculate total test execution time"
    echo "  Command: echo $((${teardown_start_time} - ${script_start_time}))"
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
    echo '    "command": "echo $((${teardown_start_time} - ${script_start_time}))",'
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

# Step 3: Generate comprehensive hook execution report
TEST_STEP_NUMBER=3
TEST_STEP_DESCRIPTION='Generate comprehensive hook execution report'
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

LOG_FILE="TC_COMPLEX_ALL_HOOKS_CAPTURE_001_sequence-4_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="cat > \${hook_log_dir}/hook_summary.txt << EOF Complete Hook Execution Report ==============================
Test ID: \${TEST_ID} Log Level: \${LOG_LEVEL} Metrics Enabled: \${METRICS_ENABLED}
Execution Timeline: ------------------- Script Start: \${script_start_time} Sequence 2 Start: \${seq2_start_time} Teardown Start: \${teardown_start_time} Total Duration: \${total_execution_time}s
Hook Execution Summary: ---------------------- 1. script_start: Executed once at beginning 2. setup_test: Executed once after script_start 3. before_sequence: Executed 4 times (once per sequence) 4. after_sequence: Will execute 4 times (once per sequence) 5. before_step: Executed \${total_steps_so_far}+ times (once per step) 6. after_step: Executed \${total_steps_so_far}+ times (once per step) 7. teardown_test: Will execute once after this sequence 8. script_end: Will execute once at very end
Variables Captured: ------------------ - script_start_time: \${script_start_time} - hook_log_dir: \${hook_log_dir} - seq2_start_time: \${seq2_start_time} - seq1_step_count: \${seq1_step_count} - elapsed_time: \${elapsed_time}s - operation_id: \${operation_id} - operation_status: \${operation_status} - total_steps_so_far: \${total_steps_so_far} - hook_execution_count: \${hook_execution_count} - teardown_start_time: \${teardown_start_time} - total_execution_time: \${total_execution_time}s
Test Status: COMPLETE All 8 hook types demonstrated successfully EOF cat \${hook_log_dir}/hook_summary.txt"
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
EXPR="grep -q 'Complete Hook Execution Report' <<< \"\$COMMAND_OUTPUT\" && grep -q 'Test Status: COMPLETE' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 3: Generate comprehensive hook execution report"
else
    echo "[FAIL] Step 3: Generate comprehensive hook execution report"
    echo "  Command: cat > ${hook_log_dir}/hook_summary.txt << EOF Complete Hook Execution Report ==============================
Test ID: ${TEST_ID} Log Level: ${LOG_LEVEL} Metrics Enabled: ${METRICS_ENABLED}
Execution Timeline: ------------------- Script Start: ${script_start_time} Sequence 2 Start: ${seq2_start_time} Teardown Start: ${teardown_start_time} Total Duration: ${total_execution_time}s
Hook Execution Summary: ---------------------- 1. script_start: Executed once at beginning 2. setup_test: Executed once after script_start 3. before_sequence: Executed 4 times (once per sequence) 4. after_sequence: Will execute 4 times (once per sequence) 5. before_step: Executed ${total_steps_so_far}+ times (once per step) 6. after_step: Executed ${total_steps_so_far}+ times (once per step) 7. teardown_test: Will execute once after this sequence 8. script_end: Will execute once at very end
Variables Captured: ------------------ - script_start_time: ${script_start_time} - hook_log_dir: ${hook_log_dir} - seq2_start_time: ${seq2_start_time} - seq1_step_count: ${seq1_step_count} - elapsed_time: ${elapsed_time}s - operation_id: ${operation_id} - operation_status: ${operation_status} - total_steps_so_far: ${total_steps_so_far} - hook_execution_count: ${hook_execution_count} - teardown_start_time: ${teardown_start_time} - total_execution_time: ${total_execution_time}s
Test Status: COMPLETE All 8 hook types demonstrated successfully EOF cat ${hook_log_dir}/hook_summary.txt"
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
    echo '    "command": "cat > ${hook_log_dir}/hook_summary.txt << EOF Complete Hook Execution Report ==============================\nTest ID: ${TEST_ID} Log Level: ${LOG_LEVEL} Metrics Enabled: ${METRICS_ENABLED}\nExecution Timeline: ------------------- Script Start: ${script_start_time} Sequence 2 Start: ${seq2_start_time} Teardown Start: ${teardown_start_time} Total Duration: ${total_execution_time}s\nHook Execution Summary: ---------------------- 1. script_start: Executed once at beginning 2. setup_test: Executed once after script_start 3. before_sequence: Executed 4 times (once per sequence) 4. after_sequence: Will execute 4 times (once per sequence) 5. before_step: Executed ${total_steps_so_far}+ times (once per step) 6. after_step: Executed ${total_steps_so_far}+ times (once per step) 7. teardown_test: Will execute once after this sequence 8. script_end: Will execute once at very end\nVariables Captured: ------------------ - script_start_time: ${script_start_time} - hook_log_dir: ${hook_log_dir} - seq2_start_time: ${seq2_start_time} - seq1_step_count: ${seq1_step_count} - elapsed_time: ${elapsed_time}s - operation_id: ${operation_id} - operation_status: ${operation_status} - total_steps_so_far: ${total_steps_so_far} - hook_execution_count: ${hook_execution_count} - teardown_start_time: ${teardown_start_time} - total_execution_time: ${total_execution_time}s\nTest Status: COMPLETE All 8 hook types demonstrated successfully EOF cat ${hook_log_dir}/hook_summary.txt",'
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

# Step 4: Record hook execution counts for verification
TEST_STEP_NUMBER=4
TEST_STEP_DESCRIPTION='Record hook execution counts for verification'
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

LOG_FILE="TC_COMPLEX_ALL_HOOKS_CAPTURE_001_sequence-4_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="cat >> \${hook_log_dir}/hooks_executed.log << EOF HOOK_EXECUTION_END=\${teardown_start_time} TOTAL_SEQUENCES=4 TOTAL_STEPS=\${total_steps_so_far} BEFORE_SEQUENCE_COUNT=4 AFTER_SEQUENCE_COUNT=4 BEFORE_STEP_COUNT=\${total_steps_so_far} AFTER_STEP_COUNT=\${total_steps_so_far} EOF tail -8 \${hook_log_dir}/hooks_executed.log"
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
EXPR="grep -q 'TOTAL_SEQUENCES=4' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 4: Record hook execution counts for verification"
else
    echo "[FAIL] Step 4: Record hook execution counts for verification"
    echo "  Command: cat >> ${hook_log_dir}/hooks_executed.log << EOF HOOK_EXECUTION_END=${teardown_start_time} TOTAL_SEQUENCES=4 TOTAL_STEPS=${total_steps_so_far} BEFORE_SEQUENCE_COUNT=4 AFTER_SEQUENCE_COUNT=4 BEFORE_STEP_COUNT=${total_steps_so_far} AFTER_STEP_COUNT=${total_steps_so_far} EOF tail -8 ${hook_log_dir}/hooks_executed.log"
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
    echo '    "command": "cat >> ${hook_log_dir}/hooks_executed.log << EOF HOOK_EXECUTION_END=${teardown_start_time} TOTAL_SEQUENCES=4 TOTAL_STEPS=${total_steps_so_far} BEFORE_SEQUENCE_COUNT=4 AFTER_SEQUENCE_COUNT=4 BEFORE_STEP_COUNT=${total_steps_so_far} AFTER_STEP_COUNT=${total_steps_so_far} EOF tail -8 ${hook_log_dir}/hooks_executed.log",'
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

# Step 5: Verify metrics log exists if enabled
TEST_STEP_NUMBER=5
TEST_STEP_DESCRIPTION='Verify metrics log exists if enabled'
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

LOG_FILE="TC_COMPLEX_ALL_HOOKS_CAPTURE_001_sequence-4_step-5.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="if [[ \"\${METRICS_ENABLED}\" == \"true\" ]]; then test -f \${hook_log_dir}/metrics.log && echo 'Metrics log verified' || echo 'Metrics log missing' else echo 'Metrics disabled, no log expected' fi"
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
    echo "[PASS] Step 5: Verify metrics log exists if enabled"
else
    echo "[FAIL] Step 5: Verify metrics log exists if enabled"
    echo "  Command: if [[ \"${METRICS_ENABLED}\" == \"true\" ]]; then test -f ${hook_log_dir}/metrics.log && echo 'Metrics log verified' || echo 'Metrics log missing' else echo 'Metrics disabled, no log expected' fi"
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
    echo '    "command": "if [[ \"${METRICS_ENABLED}\" == \"true\" ]]; then test -f ${hook_log_dir}/metrics.log && echo \"Metrics log verified\" || echo \"Metrics log missing\" else echo \"Metrics disabled, no log expected\" fi",'
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

# Step 6: Count total files generated during test
TEST_STEP_NUMBER=6
TEST_STEP_DESCRIPTION='Count total files generated during test'
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

LOG_FILE="TC_COMPLEX_ALL_HOOKS_CAPTURE_001_sequence-4_step-6.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="find \${hook_log_dir} -type f | wc -l"
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
STEP_VAR_total_files_generated=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " total_files_generated "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES total_files_generated"
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
COND_EXPR="[[ \$total_files_generated -ge 2 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        COND_EXPR=$(echo "$COND_EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$COND_EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
    echo 'Expected files generated'
else
    echo 'Fewer files than expected'
fi
echo "Files generated: ${total_files_generated}"

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 6: Count total files generated during test"
else
    echo "[FAIL] Step 6: Count total files generated during test"
    echo "  Command: find ${hook_log_dir} -type f | wc -l"
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
    echo '    "step": 6,'
    echo '    "command": "find ${hook_log_dir} -type f | wc -l",'
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

# Step 7: Final step - prepare for teardown_test and script_end hooks
TEST_STEP_NUMBER=7
TEST_STEP_DESCRIPTION='Final step - prepare for teardown_test and script_end hooks'
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

LOG_FILE="TC_COMPLEX_ALL_HOOKS_CAPTURE_001_sequence-4_step-7.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ echo 'All sequences complete' && echo 'teardown_test hook will execute next' && echo 'script_end hook will execute last' && echo 'Test execution finishing'; } 2>&1 | tee "$LOG_FILE")
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
EXPR="grep -q 'Test execution finishing' <<< \"\$COMMAND_OUTPUT\""
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
GENERAL_VERIFY_PASS_verify_all_hooks_demonstrated=false
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
    GENERAL_VERIFY_PASS_verify_all_hooks_demonstrated=true
fi
GENERAL_VERIFY_PASS_verify_all_variables_captured=false
EXPR="[[ -n \"\$script_start_time\" && -n \"\$teardown_start_time\" && -n \"\$total_execution_time\" ]]"
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
    GENERAL_VERIFY_PASS_verify_all_variables_captured=true
fi
GENERAL_VERIFY_PASS_verify_hook_logs_exist=false
EXPR="[[ -f \"\${hook_log_dir}/hook_summary.txt\" && -f \"\${hook_log_dir}/hooks_executed.log\" ]]"
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
    GENERAL_VERIFY_PASS_verify_hook_logs_exist=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_all_hooks_demonstrated" = true ] && [ "$GENERAL_VERIFY_PASS_verify_all_variables_captured" = true ] && [ "$GENERAL_VERIFY_PASS_verify_hook_logs_exist" = true ]; then
    echo "[PASS] Step 7: Final step - prepare for teardown_test and script_end hooks"
else
    echo "[FAIL] Step 7: Final step - prepare for teardown_test and script_end hooks"
    echo "  Command: echo 'All sequences complete' && echo 'teardown_test hook will execute next' && echo 'script_end hook will execute last' && echo 'Test execution finishing'"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_all_hooks_demonstrated: $GENERAL_VERIFY_PASS_verify_all_hooks_demonstrated"
    echo "  GENERAL_VERIFY_PASS_verify_all_variables_captured: $GENERAL_VERIFY_PASS_verify_all_variables_captured"
    echo "  GENERAL_VERIFY_PASS_verify_hook_logs_exist: $GENERAL_VERIFY_PASS_verify_hook_logs_exist"
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
    echo '    "step": 7,'
    echo '    "command": "echo \"All sequences complete\" && echo \"teardown_test hook will execute next\" && echo \"script_end hook will execute last\" && echo \"Test execution finishing\"",'
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
