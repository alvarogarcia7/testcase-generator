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

# Test Case: TC_DEPENDENCY_NESTED_001
# Description: Nested dependency test - demonstrates transitive dependency chain (A depends on B, B depends on C)

JSON_LOG="TC_DEPENDENCY_NESTED_001_execution_log.json"
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

# General Initial Conditions
# system: Bash shell is available
# system: Dependency chain: TC_DEPENDENCY_NESTED_001 -> TC_DEPENDENCY_SIMPLE_001 -> TC_SUCCESS_SIMPLE_001
# system: Bash shell is available
# system: Basic Unix utilities are installed
# system: Depends on TC_SUCCESS_SIMPLE_001

# Initial Conditions
# system: Transitive dependencies through TC_DEPENDENCY_SIMPLE_001
# filesystem: Temporary directory /tmp is writable

# Test Sequence 1: First Level Dependency
# Direct dependency on another dependent test
# Sequence Initial Conditions
# system: First level dependencies established

# Step 1: Verify nested dependency chain
LOG_FILE="TC_DEPENDENCY_NESTED_001_sequence-1_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ echo 'Nested dependency: TC_DEPENDENCY_NESTED_001 -> TC_DEPENDENCY_SIMPLE_001 -> TC_SUCCESS_SIMPLE_001'; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Verification result
VERIFICATION_RESULT_PASS=false
if [[ $EXIT_CODE -eq 0 ]]; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
if grep -q 'Nested dependency' <<< "$COMMAND_OUTPUT"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 1: Verify nested dependency chain"
else
    echo "[FAIL] Step 1: Verify nested dependency chain"
    echo "  Command: echo 'Nested dependency: TC_DEPENDENCY_NESTED_001 -> TC_DEPENDENCY_SIMPLE_001 -> TC_SUCCESS_SIMPLE_001'"
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
    echo '    "command": "echo \"Nested dependency: TC_DEPENDENCY_NESTED_001 -> TC_DEPENDENCY_SIMPLE_001 -> TC_SUCCESS_SIMPLE_001\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 2: Execute with nested dependencies
LOG_FILE="TC_DEPENDENCY_NESTED_001_sequence-1_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ echo 'All nested dependencies resolved' && echo 'Depth: 3 levels'; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Verification result
VERIFICATION_RESULT_PASS=false
if [[ $EXIT_CODE -eq 0 ]]; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
if grep -q 'All nested dependencies resolved' <<< "$COMMAND_OUTPUT" && grep -q 'Depth: 3 levels' <<< "$COMMAND_OUTPUT"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 2: Execute with nested dependencies"
else
    echo "[FAIL] Step 2: Execute with nested dependencies"
    echo "  Command: echo 'All nested dependencies resolved' && echo 'Depth: 3 levels'"
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
    echo '    "command": "echo \"All nested dependencies resolved\" && echo \"Depth: 3 levels\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Test Sequence 2: Second Level Dependency
# Sequence with additional nested dependencies
# Sequence Initial Conditions
# filesystem: Dependency on TC_SUCCESS_MULTI_SEQ_001 sequence 1
# system: Dependency on TC_DEPENDENCY_SEQUENCE_001 sequence 2
# system: Second level dependencies established
# system: Transitive closure includes TC_SUCCESS_MULTI_SEQ_001
# system: Dependencies from TC_SUCCESS_MULTI_SEQ_001 sequence 1 must be satisfied

# Step 1: Verify multi-level dependency
LOG_FILE="TC_DEPENDENCY_NESTED_001_sequence-2_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ echo 'Multi-level dependency graph established'; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Verification result
VERIFICATION_RESULT_PASS=false
if [[ $EXIT_CODE -eq 0 ]]; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
if grep -q 'Multi-level dependency graph established' <<< "$COMMAND_OUTPUT"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 1: Verify multi-level dependency"
else
    echo "[FAIL] Step 1: Verify multi-level dependency"
    echo "  Command: echo 'Multi-level dependency graph established'"
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
    echo '    "command": "echo \"Multi-level dependency graph established\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 2: Complete nested test execution
LOG_FILE="TC_DEPENDENCY_NESTED_001_sequence-2_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ echo 'Nested test execution complete'; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Verification result
VERIFICATION_RESULT_PASS=false
if [[ $EXIT_CODE -eq 0 ]]; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
if grep -q 'Nested test execution complete' <<< "$COMMAND_OUTPUT"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 2: Complete nested test execution"
else
    echo "[FAIL] Step 2: Complete nested test execution"
    echo "  Command: echo 'Nested test execution complete'"
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
    echo '    "command": "echo \"Nested test execution complete\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Test Sequence 3: Complex Nested Dependencies
# Sequence with complex nested dependency graph
# Sequence Initial Conditions
# system: Multiple dependencies at sequence level
# system: Complex dependency graph with transitive dependencies
# system: Bash shell is available
# system: Basic Unix utilities are installed
# system: Depends on TC_SUCCESS_SIMPLE_001
# system: No dependencies required
# system: Bash shell is available
# system: Basic utilities available

# Step 1: Execute with complex nested dependencies
LOG_FILE="TC_DEPENDENCY_NESTED_001_sequence-3_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ echo 'Complex nested dependencies: 6+ transitive dependencies'; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Verification result
VERIFICATION_RESULT_PASS=false
if [[ $EXIT_CODE -eq 0 ]]; then
    VERIFICATION_RESULT_PASS=true
fi

# Verification output
VERIFICATION_OUTPUT_PASS=false
if grep -q 'Complex nested dependencies' <<< "$COMMAND_OUTPUT"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 1: Execute with complex nested dependencies"
else
    echo "[FAIL] Step 1: Execute with complex nested dependencies"
    echo "  Command: echo 'Complex nested dependencies: 6+ transitive dependencies'"
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
    echo '    "command": "echo \"Complex nested dependencies: 6+ transitive dependencies\"",'
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
