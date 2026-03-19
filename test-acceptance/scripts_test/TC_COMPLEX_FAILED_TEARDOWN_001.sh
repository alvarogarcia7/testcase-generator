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

# Test Case: TC_COMPLEX_FAILED_TEARDOWN_001
# Description: Failed step scenario with teardown hooks ensuring proper cleanup - demonstrates error handling and cleanup guarantees

JSON_LOG="TC_COMPLEX_FAILED_TEARDOWN_001_execution_log.json"
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
# system: Test infrastructure supports cleanup hooks
# system: Teardown hooks will execute even on failure
# testing: This test intentionally includes a failing step
# testing: Cleanup must occur regardless of test outcome

# Initial Conditions
# hooks: Teardown hooks are configured with on_error: continue
# hooks: after_sequence hooks will execute for cleanup
# filesystem: Cleanup directory can be created

# Test Sequence 1: Resource Allocation Before Failure
# Allocate resources that must be cleaned up even if test fails
TEST_SEQUENCE_ID=1
TEST_SEQUENCE_NAME='Resource Allocation Before Failure'
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
# testing: Resources will be allocated for testing
# testing: Cleanup must be guaranteed

# Step 1: Create cleanup test directory structure
TEST_STEP_NUMBER=1
TEST_STEP_DESCRIPTION='Create cleanup test directory structure'
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

LOG_FILE="TC_COMPLEX_FAILED_TEARDOWN_001_sequence-1_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="mkdir -p \${CLEANUP_DIR}/{tmp,data,logs,locks} && echo 'Directories created' && ls -d \${CLEANUP_DIR}/*"
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
EXPR="grep -q 'tmp' <<< \"\$COMMAND_OUTPUT\" && grep -q 'data' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 1: Create cleanup test directory structure"
else
    echo "[FAIL] Step 1: Create cleanup test directory structure"
    echo "  Command: mkdir -p ${CLEANUP_DIR}/{tmp,data,logs,locks} && echo 'Directories created' && ls -d ${CLEANUP_DIR}/*"
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
    echo '    "step": 1,'
    echo '    "command": "mkdir -p ${CLEANUP_DIR}/{tmp,data,logs,locks} && echo \"Directories created\" && ls -d ${CLEANUP_DIR}/*",'
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

# Step 2: Create test resources that need cleanup
TEST_STEP_NUMBER=2
TEST_STEP_DESCRIPTION='Create test resources that need cleanup'
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

LOG_FILE="TC_COMPLEX_FAILED_TEARDOWN_001_sequence-1_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ date +%s; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_resource_timestamp=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " resource_timestamp "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES resource_timestamp"
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
EXPR="[[ -n \"\$resource_timestamp\" ]]"
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

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_timestamp_captured" = true ]; then
    echo "[PASS] Step 2: Create test resources that need cleanup"
else
    echo "[FAIL] Step 2: Create test resources that need cleanup"
    echo "  Command: date +%s"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_timestamp_captured: $GENERAL_VERIFY_PASS_verify_timestamp_captured"
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

# Step 3: Allocate multiple test files
TEST_STEP_NUMBER=3
TEST_STEP_DESCRIPTION='Allocate multiple test files'
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

LOG_FILE="TC_COMPLEX_FAILED_TEARDOWN_001_sequence-1_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="for i in 1 2 3 4 5; do echo \"Test data \$i\" > \${CLEANUP_DIR}/data/test_file_\$i.txt; done && ls \${CLEANUP_DIR}/data/ | wc -l"
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
STEP_VAR_allocated_files=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " allocated_files "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES allocated_files"
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
EXPR="[[ \"\$COMMAND_OUTPUT\" -eq 5 ]]"
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
GENERAL_VERIFY_PASS_verify_files_allocated=false
EXPR="[[ \$allocated_files -eq 5 ]]"
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
    GENERAL_VERIFY_PASS_verify_files_allocated=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_files_allocated" = true ]; then
    echo "[PASS] Step 3: Allocate multiple test files"
else
    echo "[FAIL] Step 3: Allocate multiple test files"
    echo "  Command: for i in 1 2 3 4 5; do echo \"Test data $i\" > ${CLEANUP_DIR}/data/test_file_$i.txt; done && ls ${CLEANUP_DIR}/data/ | wc -l"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_files_allocated: $GENERAL_VERIFY_PASS_verify_files_allocated"
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
    echo '    "command": "for i in 1 2 3 4 5; do echo \"Test data $i\" > ${CLEANUP_DIR}/data/test_file_$i.txt; done && ls ${CLEANUP_DIR}/data/ | wc -l",'
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

# Step 4: Create lock file that must be cleaned up
TEST_STEP_NUMBER=4
TEST_STEP_DESCRIPTION='Create lock file that must be cleaned up'
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

LOG_FILE="TC_COMPLEX_FAILED_TEARDOWN_001_sequence-1_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo \$\$ > \${CLEANUP_DIR}/locks/test.lock && cat \${CLEANUP_DIR}/locks/test.lock"
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
STEP_VAR_lock_pid=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " lock_pid "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES lock_pid"
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
    echo "[PASS] Step 4: Create lock file that must be cleaned up"
else
    echo "[FAIL] Step 4: Create lock file that must be cleaned up"
    echo "  Command: echo $$ > ${CLEANUP_DIR}/locks/test.lock && cat ${CLEANUP_DIR}/locks/test.lock"
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
    echo '    "command": "echo $$ > ${CLEANUP_DIR}/locks/test.lock && cat ${CLEANUP_DIR}/locks/test.lock",'
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

# Step 5: Create cleanup manifest
TEST_STEP_NUMBER=5
TEST_STEP_DESCRIPTION='Create cleanup manifest'
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

LOG_FILE="TC_COMPLEX_FAILED_TEARDOWN_001_sequence-1_step-5.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="cat > \${CLEANUP_DIR}/cleanup_manifest.txt << EOF # Cleanup Manifest - Created at \${resource_timestamp} # Resources that must be cleaned up: DIRECTORIES: tmp data logs locks FILES: \${allocated_files} files in data/ LOCKS: test.lock (PID: \${lock_pid}) STATUS: Resources allocated, cleanup required EOF cat \${CLEANUP_DIR}/cleanup_manifest.txt"
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
EXPR="grep -q 'cleanup required' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 5: Create cleanup manifest"
else
    echo "[FAIL] Step 5: Create cleanup manifest"
    echo "  Command: cat > ${CLEANUP_DIR}/cleanup_manifest.txt << EOF # Cleanup Manifest - Created at ${resource_timestamp} # Resources that must be cleaned up: DIRECTORIES: tmp data logs locks FILES: ${allocated_files} files in data/ LOCKS: test.lock (PID: ${lock_pid}) STATUS: Resources allocated, cleanup required EOF cat ${CLEANUP_DIR}/cleanup_manifest.txt"
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
    echo '    "step": 5,'
    echo '    "command": "cat > ${CLEANUP_DIR}/cleanup_manifest.txt << EOF # Cleanup Manifest - Created at ${resource_timestamp} # Resources that must be cleaned up: DIRECTORIES: tmp data logs locks FILES: ${allocated_files} files in data/ LOCKS: test.lock (PID: ${lock_pid}) STATUS: Resources allocated, cleanup required EOF cat ${CLEANUP_DIR}/cleanup_manifest.txt",'
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

# Test Sequence 2: Intentional Failure Sequence
# Sequence that will fail to test cleanup behavior
TEST_SEQUENCE_ID=2
TEST_SEQUENCE_NAME='Intentional Failure Sequence'
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
# testing: Resources from sequence 1 are allocated
# testing: Step ${FAIL_AT_STEP} will intentionally fail
# testing: Cleanup hooks must still execute

# Step 1: Pre-failure operation - should succeed
TEST_STEP_NUMBER=1
TEST_STEP_DESCRIPTION='Pre-failure operation - should succeed'
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

LOG_FILE="TC_COMPLEX_FAILED_TEARDOWN_001_sequence-2_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo 'Pre-failure step executing' && echo 'Resource timestamp: '\${resource_timestamp}"
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
EXPR="grep -q 'Pre-failure step executing' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 1: Pre-failure operation - should succeed"
else
    echo "[FAIL] Step 1: Pre-failure operation - should succeed"
    echo "  Command: echo 'Pre-failure step executing' && echo 'Resource timestamp: '${resource_timestamp}"
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
    echo '    "command": "echo \"Pre-failure step executing\" && echo \"Resource timestamp: \"${resource_timestamp}",'
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

# Step 2: Create temporary processing files
TEST_STEP_NUMBER=2
TEST_STEP_DESCRIPTION='Create temporary processing files'
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

LOG_FILE="TC_COMPLEX_FAILED_TEARDOWN_001_sequence-2_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="for i in 1 2 3; do echo 'Processing \$i' > \${CLEANUP_DIR}/tmp/process_\$i.tmp; done && ls \${CLEANUP_DIR}/tmp/ | wc -l"
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
STEP_VAR_temp_files_created=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " temp_files_created "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES temp_files_created"
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
EXPR="[[ \"\$COMMAND_OUTPUT\" -eq 3 ]]"
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
    echo "[PASS] Step 2: Create temporary processing files"
else
    echo "[FAIL] Step 2: Create temporary processing files"
    echo "  Command: for i in 1 2 3; do echo 'Processing $i' > ${CLEANUP_DIR}/tmp/process_$i.tmp; done && ls ${CLEANUP_DIR}/tmp/ | wc -l"
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
    echo '    "command": "for i in 1 2 3; do echo \"Processing $i\" > ${CLEANUP_DIR}/tmp/process_$i.tmp; done && ls ${CLEANUP_DIR}/tmp/ | wc -l",'
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

# Step 3: THIS STEP INTENTIONALLY FAILS - Simulating error condition
TEST_STEP_NUMBER=3
TEST_STEP_DESCRIPTION='THIS STEP INTENTIONALLY FAILS - Simulating error condition'
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

LOG_FILE="TC_COMPLEX_FAILED_TEARDOWN_001_sequence-2_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ echo 'Attempting risky operation...' && echo 'SIMULATED FAILURE' && exit 1; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Verification result
VERIFICATION_RESULT_PASS=false
EXPR="[[ \$EXIT_CODE -eq 1 ]]"
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
EXPR="grep -q 'SIMULATED FAILURE' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 3: THIS STEP INTENTIONALLY FAILS - Simulating error condition"
else
    echo "[FAIL] Step 3: THIS STEP INTENTIONALLY FAILS - Simulating error condition"
    echo "  Command: echo 'Attempting risky operation...' && echo 'SIMULATED FAILURE' && exit 1"
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
    echo '    "command": "echo \"Attempting risky operation...\" && echo \"SIMULATED FAILURE\" && exit 1",'
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

# Step 4: Post-failure step - should not execute
TEST_STEP_NUMBER=4
TEST_STEP_DESCRIPTION='Post-failure step - should not execute'
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

LOG_FILE="TC_COMPLEX_FAILED_TEARDOWN_001_sequence-2_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ echo 'This step should not execute'; } 2>&1 | tee "$LOG_FILE")
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
EXPR="grep -q 'should not execute' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 4: Post-failure step - should not execute"
else
    echo "[FAIL] Step 4: Post-failure step - should not execute"
    echo "  Command: echo 'This step should not execute'"
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
    echo '    "command": "echo \"This step should not execute\"",'
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

# Test Sequence 3: Cleanup Verification Sequence
# This sequence verifies cleanup hooks executed properly (may not execute if test stops on failure)
TEST_SEQUENCE_ID=3
TEST_SEQUENCE_NAME='Cleanup Verification Sequence'
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
# testing: This sequence may not execute if test fails
# testing: Cleanup hooks should have executed regardless

# Step 1: Verify cleanup directory still exists for inspection
TEST_STEP_NUMBER=1
TEST_STEP_DESCRIPTION='Verify cleanup directory still exists for inspection'
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

LOG_FILE="TC_COMPLEX_FAILED_TEARDOWN_001_sequence-3_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="test -d \${CLEANUP_DIR} && echo 'Cleanup directory exists' || echo 'Cleanup directory removed'"
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
    echo "[PASS] Step 1: Verify cleanup directory still exists for inspection"
else
    echo "[FAIL] Step 1: Verify cleanup directory still exists for inspection"
    echo "  Command: test -d ${CLEANUP_DIR} && echo 'Cleanup directory exists' || echo 'Cleanup directory removed'"
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
    echo '    "command": "test -d ${CLEANUP_DIR} && echo \"Cleanup directory exists\" || echo \"Cleanup directory removed\"",'
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

# Step 2: Check if cleanup manifest is still present
TEST_STEP_NUMBER=2
TEST_STEP_DESCRIPTION='Check if cleanup manifest is still present'
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

LOG_FILE="TC_COMPLEX_FAILED_TEARDOWN_001_sequence-3_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="test -f \${CLEANUP_DIR}/cleanup_manifest.txt && cat \${CLEANUP_DIR}/cleanup_manifest.txt || echo 'Manifest not found'"
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
    echo "[PASS] Step 2: Check if cleanup manifest is still present"
else
    echo "[FAIL] Step 2: Check if cleanup manifest is still present"
    echo "  Command: test -f ${CLEANUP_DIR}/cleanup_manifest.txt && cat ${CLEANUP_DIR}/cleanup_manifest.txt || echo 'Manifest not found'"
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
    echo '    "command": "test -f ${CLEANUP_DIR}/cleanup_manifest.txt && cat ${CLEANUP_DIR}/cleanup_manifest.txt || echo \"Manifest not found\"",'
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

# Step 3: Log cleanup verification results
TEST_STEP_NUMBER=3
TEST_STEP_DESCRIPTION='Log cleanup verification results'
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

LOG_FILE="TC_COMPLEX_FAILED_TEARDOWN_001_sequence-3_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ echo 'Cleanup verification completed' && echo 'Teardown hooks should execute regardless of this sequence'; } 2>&1 | tee "$LOG_FILE")
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
EXPR="grep -q 'Cleanup verification completed' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 3: Log cleanup verification results"
else
    echo "[FAIL] Step 3: Log cleanup verification results"
    echo "  Command: echo 'Cleanup verification completed' && echo 'Teardown hooks should execute regardless of this sequence'"
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
    echo '    "command": "echo \"Cleanup verification completed\" && echo \"Teardown hooks should execute regardless of this sequence\"",'
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
