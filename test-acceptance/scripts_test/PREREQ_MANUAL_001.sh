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

# Test Case: PREREQ_MANUAL_001
# Description: Test case with manual prerequisites requiring user confirmation

JSON_LOG="PREREQ_MANUAL_001_execution_log.json"
TIMESTAMP=$(date +"%Y-%m-%dT%H:%M:%S")

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

# Prerequisites
echo "Checking prerequisites..."

# Prerequisite 1: Ensure you have network connectivity to the internet
echo "[MANUAL PREREQUISITE 1] Ensure you have network connectivity to the internet"
if [[ "${DEBIAN_FRONTEND:-}" != 'noninteractive' && -t 0 ]]; then
    read -p "Press ENTER to confirm this prerequisite is satisfied..."
else
    echo "Non-interactive mode: assuming prerequisite is satisfied."
fi

# Prerequisite 2: Verify that you have appropriate permissions to create files in /tmp
echo "[MANUAL PREREQUISITE 2] Verify that you have appropriate permissions to create files in /tmp"
if [[ "${DEBIAN_FRONTEND:-}" != 'noninteractive' && -t 0 ]]; then
    read -p "Press ENTER to confirm this prerequisite is satisfied..."
else
    echo "Non-interactive mode: assuming prerequisite is satisfied."
fi

# Prerequisite 3: Confirm that no other test processes are currently running
echo "[MANUAL PREREQUISITE 3] Confirm that no other test processes are currently running"
if [[ "${DEBIAN_FRONTEND:-}" != 'noninteractive' && -t 0 ]]; then
    read -p "Press ENTER to confirm this prerequisite is satisfied..."
else
    echo "Non-interactive mode: assuming prerequisite is satisfied."
fi

echo "All prerequisites satisfied"
echo ""

# General Initial Conditions
# system: User has reviewed and confirmed all manual prerequisites
# system: Test environment is properly configured

# Initial Conditions
# system: All manual checks have been completed

# Test Sequence 1: Manual Prerequisite Test Sequence
# This test sequence executes after user has manually
# confirmed all prerequisites.
# 
# Sequence Initial Conditions
# system: User is ready to proceed with test execution

# Step 1: Create a test file in /tmp
LOG_FILE="PREREQ_MANUAL_001_sequence-1_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ echo "manual prereq test" > /tmp/manual_prereq_test.txt; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Verification result
VERIFICATION_RESULT_PASS=false
if [[ $EXIT_CODE -eq 0 ]]; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
if true; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 1: Create a test file in /tmp"
else
    echo "[FAIL] Step 1: Create a test file in /tmp"
    echo "  Command: echo \"manual prereq test\" > /tmp/manual_prereq_test.txt"
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
    echo '    "command": "echo \"manual prereq test\" > /tmp/manual_prereq_test.txt",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 2: Verify file creation
LOG_FILE="PREREQ_MANUAL_001_sequence-1_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ test -f /tmp/manual_prereq_test.txt && cat /tmp/manual_prereq_test.txt; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Verification result
VERIFICATION_RESULT_PASS=false
if [[ $EXIT_CODE -eq 0 ]]; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
if grep -q 'manual prereq test' <<< "$COMMAND_OUTPUT"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 2: Verify file creation"
else
    echo "[FAIL] Step 2: Verify file creation"
    echo "  Command: test -f /tmp/manual_prereq_test.txt && cat /tmp/manual_prereq_test.txt"
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
    echo '    "command": "test -f /tmp/manual_prereq_test.txt && cat /tmp/manual_prereq_test.txt",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 3: Remove test file
LOG_FILE="PREREQ_MANUAL_001_sequence-1_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ rm -f /tmp/manual_prereq_test.txt; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Verification result
VERIFICATION_RESULT_PASS=false
if [[ $EXIT_CODE -eq 0 ]]; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
if true; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 3: Remove test file"
else
    echo "[FAIL] Step 3: Remove test file"
    echo "  Command: rm -f /tmp/manual_prereq_test.txt"
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
    echo '    "command": "rm -f /tmp/manual_prereq_test.txt",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 4: Manually verify that the test file was removed
echo "Step 4: Manually verify that the test file was removed"
echo "Command: test ! -f /tmp/manual_prereq_test.txt && echo \"file removed\""
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
if Confirm that the test file no longer exists in /tmp; then
    USER_VERIFICATION_OUTPUT=true
fi

# Set USER_VERIFICATION based on verification results
if [ "$USER_VERIFICATION_RESULT" = true ] && [ "$USER_VERIFICATION_OUTPUT" = true ]; then
    USER_VERIFICATION=true
else
    USER_VERIFICATION=false
fi

if [ "$USER_VERIFICATION" = true ]; then
    echo "[PASS] Step 4: Manually verify that the test file was removed"
else
    echo "[FAIL] Step 4: Manually verify that the test file was removed"
    echo "  Result verification: $USER_VERIFICATION_RESULT"
    echo "  Output verification: $USER_VERIFICATION_OUTPUT"
    exit 1
fi

echo ']' >> "$JSON_LOG"

# Validate JSON against schema
if command -v jq >/dev/null 2>&1; then
    if ! jq empty "$JSON_LOG" >/dev/null 2>&1; then
        echo "500 - Internal Script Error: Generated JSON is not valid"
        exit 1
    fi
fi

echo "All test sequences completed successfully"
exit 0
