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

# Test Case: PREREQ_PARTIAL_FAIL_001
# Description: Test case where some automatic prerequisites pass but at least one fails

JSON_LOG="PREREQ_PARTIAL_FAIL_001_execution_log.json"
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

# Prerequisite 1: Check that echo command is available (should pass)
echo "[AUTOMATIC PREREQUISITE 1] Verifying: Check that echo command is available (should pass)"
set +e
PREREQ_OUTPUT=$({ which echo > /dev/null 2>&1; } 2>&1)
PREREQ_EXIT_CODE=$?
set -e
if [ $PREREQ_EXIT_CODE -ne 0 ]; then
    echo "ERROR: Prerequisite 1 failed: Check that echo command is available (should pass)"
    echo "Verification command: which echo > /dev/null 2>&1"
    echo "Exit code: $PREREQ_EXIT_CODE"
    echo "Output: $PREREQ_OUTPUT"
    exit 1
fi
echo "[PASS] Prerequisite 1 verified"

# Prerequisite 2: Verify /tmp directory exists (should pass)
echo "[AUTOMATIC PREREQUISITE 2] Verifying: Verify /tmp directory exists (should pass)"
set +e
PREREQ_OUTPUT=$({ test -d /tmp; } 2>&1)
PREREQ_EXIT_CODE=$?
set -e
if [ $PREREQ_EXIT_CODE -ne 0 ]; then
    echo "ERROR: Prerequisite 2 failed: Verify /tmp directory exists (should pass)"
    echo "Verification command: test -d /tmp"
    echo "Exit code: $PREREQ_EXIT_CODE"
    echo "Output: $PREREQ_OUTPUT"
    exit 1
fi
echo "[PASS] Prerequisite 2 verified"

# Prerequisite 3: Check for non-existent utility (should fail)
echo "[AUTOMATIC PREREQUISITE 3] Verifying: Check for non-existent utility (should fail)"
set +e
PREREQ_OUTPUT=$({ which nonexistent_utility_xyz > /dev/null 2>&1; } 2>&1)
PREREQ_EXIT_CODE=$?
set -e
if [ $PREREQ_EXIT_CODE -ne 0 ]; then
    echo "ERROR: Prerequisite 3 failed: Check for non-existent utility (should fail)"
    echo "Verification command: which nonexistent_utility_xyz > /dev/null 2>&1"
    echo "Exit code: $PREREQ_EXIT_CODE"
    echo "Output: $PREREQ_OUTPUT"
    exit 1
fi
echo "[PASS] Prerequisite 3 verified"

# Prerequisite 4: This check should not be reached
echo "[AUTOMATIC PREREQUISITE 4] Verifying: This check should not be reached"
set +e
PREREQ_OUTPUT=$({ which bash > /dev/null 2>&1; } 2>&1)
PREREQ_EXIT_CODE=$?
set -e
if [ $PREREQ_EXIT_CODE -ne 0 ]; then
    echo "ERROR: Prerequisite 4 failed: This check should not be reached"
    echo "Verification command: which bash > /dev/null 2>&1"
    echo "Exit code: $PREREQ_EXIT_CODE"
    echo "Output: $PREREQ_OUTPUT"
    exit 1
fi
echo "[PASS] Prerequisite 4 verified"

echo "All prerequisites satisfied"
echo ""

# General Initial Conditions
# system: Shell environment is available

# Initial Conditions
# system: Some prerequisites will pass, but at least one will fail

# Test Sequence 1: Test Sequence That Should Not Execute
# This test sequence should not execute because one of the
# automatic prerequisites will fail, even though others pass.
# 
# Sequence Initial Conditions
# system: This should never be reached

# Step 1: This step should never execute
LOG_FILE="PREREQ_PARTIAL_FAIL_001_sequence-1_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ echo "Test failed to stop at failed prerequisite"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Verification result
VERIFICATION_RESULT_PASS=false
if [[ $EXIT_CODE -eq 0 ]]; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
if grep -q 'Test failed' <<< "$COMMAND_OUTPUT"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 1: This step should never execute"
else
    echo "[FAIL] Step 1: This step should never execute"
    echo "  Command: echo \"Test failed to stop at failed prerequisite\""
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
    echo '    "command": "echo \"Test failed to stop at failed prerequisite\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

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
