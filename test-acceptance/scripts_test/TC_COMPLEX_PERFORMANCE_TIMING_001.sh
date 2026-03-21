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

# Test Case: TC_COMPLEX_PERFORMANCE_TIMING_001
# Description: Performance test with timing capture and validation - demonstrates performance metrics collection and analysis

JSON_LOG="TC_COMPLEX_PERFORMANCE_TIMING_001_execution_log.json"
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

# General Initial Conditions
# environment: System is not under heavy load
# environment: Test environment is consistent for accurate measurements
# system: High-resolution timing is available
# system: Performance metrics can be captured

# Initial Conditions
# performance: Baseline performance can be established
# performance: Timing measurements are accurate to milliseconds
# filesystem: Temporary directory available for performance test data

# Test Sequence 1: Baseline Performance Measurement
# Establish baseline performance metrics
TEST_SEQUENCE_ID=1
TEST_SEQUENCE_NAME='Baseline Performance Measurement'
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
# performance: System is in idle state
# performance: No background tasks interfering

# Step 1: Capture baseline start timestamp
LOG_FILE="TC_COMPLEX_PERFORMANCE_TIMING_001_sequence-1_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ date +%s%N; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_baseline_start_ns=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " baseline_start_ns "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES baseline_start_ns"
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
GENERAL_VERIFY_PASS_verify_baseline_start_captured=false
EXPR="[[ -n \"\$baseline_start_ns\" ]]"
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
    GENERAL_VERIFY_PASS_verify_baseline_start_captured=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_baseline_start_captured" = true ]; then
    echo "[PASS] Step 1: Capture baseline start timestamp"
else
    echo "[FAIL] Step 1: Capture baseline start timestamp"
    echo "  Command: date +%s%N"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_baseline_start_captured: $GENERAL_VERIFY_PASS_verify_baseline_start_captured"
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
    echo '    "command": "date +%s%N",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 2: Execute baseline operation (simple echo)
LOG_FILE="TC_COMPLEX_PERFORMANCE_TIMING_001_sequence-1_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="for i in \$(seq 1 \${PERF_TEST_ITERATIONS}); do echo \"Iteration \$i\"; done | wc -l"
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
STEP_VAR_baseline_iteration_count=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " baseline_iteration_count "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES baseline_iteration_count"
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
EXPR="[[ \"\$COMMAND_OUTPUT\" -eq \${PERF_TEST_ITERATIONS} ]]"
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
    echo "[PASS] Step 2: Execute baseline operation (simple echo)"
else
    echo "[FAIL] Step 2: Execute baseline operation (simple echo)"
    echo "  Command: for i in $(seq 1 ${PERF_TEST_ITERATIONS}); do echo \"Iteration $i\"; done | wc -l"
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
    echo '    "command": "for i in $(seq 1 ${PERF_TEST_ITERATIONS}); do echo \"Iteration $i\"; done | wc -l",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 3: Capture baseline end timestamp
LOG_FILE="TC_COMPLEX_PERFORMANCE_TIMING_001_sequence-1_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ date +%s%N; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_baseline_end_ns=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " baseline_end_ns "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES baseline_end_ns"
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
    echo "[PASS] Step 3: Capture baseline end timestamp"
else
    echo "[FAIL] Step 3: Capture baseline end timestamp"
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
    echo '    "test_sequence": 1,'
    echo '    "step": 3,'
    echo '    "command": "date +%s%N",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 4: Calculate baseline duration in milliseconds
LOG_FILE="TC_COMPLEX_PERFORMANCE_TIMING_001_sequence-1_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo \$(( (\${baseline_end_ns} - \${baseline_start_ns}) / 1000000 ))"
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
STEP_VAR_baseline_duration_ms=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " baseline_duration_ms "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES baseline_duration_ms"
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
COND_EXPR="[[ \$baseline_duration_ms -ge 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        COND_EXPR=$(echo "$COND_EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$COND_EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
    echo 'Baseline duration calculated successfully'
else
    echo 'Baseline duration calculation error'
fi
echo "Baseline: ${baseline_duration_ms}ms"

# General verifications
GENERAL_VERIFY_PASS_verify_baseline_duration_numeric=false
EXPR="[[ \$baseline_duration_ms =~ ^[0-9]+\$ ]]"
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
    GENERAL_VERIFY_PASS_verify_baseline_duration_numeric=true
fi
GENERAL_VERIFY_PASS_verify_baseline_duration_valid=false
EXPR="[[ \$baseline_duration_ms -ge 0 ]]"
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
    GENERAL_VERIFY_PASS_verify_baseline_duration_valid=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_baseline_duration_numeric" = true ] && [ "$GENERAL_VERIFY_PASS_verify_baseline_duration_valid" = true ]; then
    echo "[PASS] Step 4: Calculate baseline duration in milliseconds"
else
    echo "[FAIL] Step 4: Calculate baseline duration in milliseconds"
    echo "  Command: echo $(( (${baseline_end_ns} - ${baseline_start_ns}) / 1000000 ))"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_baseline_duration_numeric: $GENERAL_VERIFY_PASS_verify_baseline_duration_numeric"
    echo "  GENERAL_VERIFY_PASS_verify_baseline_duration_valid: $GENERAL_VERIFY_PASS_verify_baseline_duration_valid"
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
    echo '    "command": "echo $(( (${baseline_end_ns} - ${baseline_start_ns}) / 1000000 ))",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 5: Record baseline metrics
LOG_FILE="TC_COMPLEX_PERFORMANCE_TIMING_001_sequence-1_step-5.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="mkdir -p /tmp/perf_test_metrics && cat > /tmp/perf_test_metrics/baseline.txt << EOF Baseline Performance Metrics ============================ Start: \${baseline_start_ns} End: \${baseline_end_ns} Duration: \${baseline_duration_ms}ms Iterations: \${baseline_iteration_count} Avg per iteration: \$(echo \"scale=2; \${baseline_duration_ms} / \${baseline_iteration_count}\" | bc 2>/dev/null || echo 'N/A')ms EOF cat /tmp/perf_test_metrics/baseline.txt"
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
EXPR="grep -q 'Duration: \${baseline_duration_ms}ms' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 5: Record baseline metrics"
else
    echo "[FAIL] Step 5: Record baseline metrics"
    echo "  Command: mkdir -p /tmp/perf_test_metrics && cat > /tmp/perf_test_metrics/baseline.txt << EOF Baseline Performance Metrics ============================ Start: ${baseline_start_ns} End: ${baseline_end_ns} Duration: ${baseline_duration_ms}ms Iterations: ${baseline_iteration_count} Avg per iteration: $(echo \"scale=2; ${baseline_duration_ms} / ${baseline_iteration_count}\" | bc 2>/dev/null || echo 'N/A')ms EOF cat /tmp/perf_test_metrics/baseline.txt"
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
    echo '    "command": "mkdir -p /tmp/perf_test_metrics && cat > /tmp/perf_test_metrics/baseline.txt << EOF Baseline Performance Metrics ============================ Start: ${baseline_start_ns} End: ${baseline_end_ns} Duration: ${baseline_duration_ms}ms Iterations: ${baseline_iteration_count} Avg per iteration: $(echo \"scale=2; ${baseline_duration_ms} / ${baseline_iteration_count}\" | bc 2>/dev/null || echo \"N/A\")ms EOF cat /tmp/perf_test_metrics/baseline.txt",'
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

# Test Sequence 2: File I/O Performance Test
# Measure file creation and read performance
TEST_SEQUENCE_ID=2
TEST_SEQUENCE_NAME='File I/O Performance Test'
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
# filesystem: Performance test directory is available
# performance: Baseline has been established

# Step 1: Capture file I/O test start timestamp
LOG_FILE="TC_COMPLEX_PERFORMANCE_TIMING_001_sequence-2_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ date +%s%N; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_fileio_start_ns=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " fileio_start_ns "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES fileio_start_ns"
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
    echo "[PASS] Step 1: Capture file I/O test start timestamp"
else
    echo "[FAIL] Step 1: Capture file I/O test start timestamp"
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
    echo '    "step": 1,'
    echo '    "command": "date +%s%N",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 2: Create test files with measured workload
LOG_FILE="TC_COMPLEX_PERFORMANCE_TIMING_001_sequence-2_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="for i in \$(seq 1 \${PERF_WORKLOAD_SIZE}); do echo \"Test data line \$i\" >> /tmp/perf_test_metrics/test_data.txt; done && wc -l /tmp/perf_test_metrics/test_data.txt | awk '{print \$1}'"
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
STEP_VAR_lines_written=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " lines_written "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES lines_written"
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
EXPR="[[ \"\$COMMAND_OUTPUT\" -eq \${PERF_WORKLOAD_SIZE} ]]"
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
    echo "[PASS] Step 2: Create test files with measured workload"
else
    echo "[FAIL] Step 2: Create test files with measured workload"
    echo "  Command: for i in $(seq 1 ${PERF_WORKLOAD_SIZE}); do echo \"Test data line $i\" >> /tmp/perf_test_metrics/test_data.txt; done && wc -l /tmp/perf_test_metrics/test_data.txt | awk '{print $1}'"
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
    echo '    "command": "for i in $(seq 1 ${PERF_WORKLOAD_SIZE}); do echo \"Test data line $i\" >> /tmp/perf_test_metrics/test_data.txt; done && wc -l /tmp/perf_test_metrics/test_data.txt | awk \"{print $1}\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 3: Capture file I/O test end timestamp
LOG_FILE="TC_COMPLEX_PERFORMANCE_TIMING_001_sequence-2_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ date +%s%N; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_fileio_end_ns=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " fileio_end_ns "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES fileio_end_ns"
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
    echo "[PASS] Step 3: Capture file I/O test end timestamp"
else
    echo "[FAIL] Step 3: Capture file I/O test end timestamp"
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
    echo '    "step": 3,'
    echo '    "command": "date +%s%N",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 4: Calculate file I/O duration
LOG_FILE="TC_COMPLEX_PERFORMANCE_TIMING_001_sequence-2_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo \$(( (\${fileio_end_ns} - \${fileio_start_ns}) / 1000000 ))"
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
STEP_VAR_fileio_duration_ms=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " fileio_duration_ms "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES fileio_duration_ms"
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
COND_EXPR="[[ \$fileio_duration_ms -lt \${PERF_THRESHOLD_MS} ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        COND_EXPR=$(echo "$COND_EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$COND_EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
    echo 'File I/O performance within threshold'
else
    echo 'WARNING: File I/O performance exceeded threshold'
fi
echo "File I/O: ${fileio_duration_ms}ms (threshold: ${PERF_THRESHOLD_MS}ms)"

# General verifications
GENERAL_VERIFY_PASS_verify_fileio_duration_numeric=false
EXPR="[[ \$fileio_duration_ms =~ ^[0-9]+\$ ]]"
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
    GENERAL_VERIFY_PASS_verify_fileio_duration_numeric=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_fileio_duration_numeric" = true ]; then
    echo "[PASS] Step 4: Calculate file I/O duration"
else
    echo "[FAIL] Step 4: Calculate file I/O duration"
    echo "  Command: echo $(( (${fileio_end_ns} - ${fileio_start_ns}) / 1000000 ))"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_fileio_duration_numeric: $GENERAL_VERIFY_PASS_verify_fileio_duration_numeric"
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
    echo '    "command": "echo $(( (${fileio_end_ns} - ${fileio_start_ns}) / 1000000 ))",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 5: Record file I/O metrics
LOG_FILE="TC_COMPLEX_PERFORMANCE_TIMING_001_sequence-2_step-5.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="cat > /tmp/perf_test_metrics/fileio.txt << EOF File I/O Performance Metrics ============================ Start: \${fileio_start_ns} End: \${fileio_end_ns} Duration: \${fileio_duration_ms}ms Lines Written: \${lines_written} Workload Size: \${PERF_WORKLOAD_SIZE} Throughput: \$(echo \"scale=2; \${lines_written} * 1000 / \${fileio_duration_ms}\" | bc 2>/dev/null || echo 'N/A') lines/sec Threshold: \${PERF_THRESHOLD_MS}ms Status: \$([ \${fileio_duration_ms} -lt \${PERF_THRESHOLD_MS} ] && echo 'PASS' || echo 'WARN') EOF cat /tmp/perf_test_metrics/fileio.txt"
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
EXPR="grep -q 'Duration: \${fileio_duration_ms}ms' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 5: Record file I/O metrics"
else
    echo "[FAIL] Step 5: Record file I/O metrics"
    echo "  Command: cat > /tmp/perf_test_metrics/fileio.txt << EOF File I/O Performance Metrics ============================ Start: ${fileio_start_ns} End: ${fileio_end_ns} Duration: ${fileio_duration_ms}ms Lines Written: ${lines_written} Workload Size: ${PERF_WORKLOAD_SIZE} Throughput: $(echo \"scale=2; ${lines_written} * 1000 / ${fileio_duration_ms}\" | bc 2>/dev/null || echo 'N/A') lines/sec Threshold: ${PERF_THRESHOLD_MS}ms Status: $([ ${fileio_duration_ms} -lt ${PERF_THRESHOLD_MS} ] && echo 'PASS' || echo 'WARN') EOF cat /tmp/perf_test_metrics/fileio.txt"
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
    echo '    "step": 5,'
    echo '    "command": "cat > /tmp/perf_test_metrics/fileio.txt << EOF File I/O Performance Metrics ============================ Start: ${fileio_start_ns} End: ${fileio_end_ns} Duration: ${fileio_duration_ms}ms Lines Written: ${lines_written} Workload Size: ${PERF_WORKLOAD_SIZE} Throughput: $(echo \"scale=2; ${lines_written} * 1000 / ${fileio_duration_ms}\" | bc 2>/dev/null || echo \"N/A\") lines/sec Threshold: ${PERF_THRESHOLD_MS}ms Status: $([ ${fileio_duration_ms} -lt ${PERF_THRESHOLD_MS} ] && echo \"PASS\" || echo \"WARN\") EOF cat /tmp/perf_test_metrics/fileio.txt",'
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

# Test Sequence 3: Processing Performance Test
# Measure data processing and computation performance
TEST_SEQUENCE_ID=3
TEST_SEQUENCE_NAME='Processing Performance Test'
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
# performance: File I/O test completed
# performance: Test data file is available

# Step 1: Capture processing test start timestamp
LOG_FILE="TC_COMPLEX_PERFORMANCE_TIMING_001_sequence-3_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ date +%s%N; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_processing_start_ns=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " processing_start_ns "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES processing_start_ns"
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
    echo "[PASS] Step 1: Capture processing test start timestamp"
else
    echo "[FAIL] Step 1: Capture processing test start timestamp"
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
    echo '    "step": 1,'
    echo '    "command": "date +%s%N",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 2: Process test data (grep, sort, count operations)
LOG_FILE="TC_COMPLEX_PERFORMANCE_TIMING_001_sequence-3_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ grep 'Test data' /tmp/perf_test_metrics/test_data.txt | sort | uniq | wc -l; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_unique_lines=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " unique_lines "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES unique_lines"
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
    echo "[PASS] Step 2: Process test data (grep, sort, count operations)"
else
    echo "[FAIL] Step 2: Process test data (grep, sort, count operations)"
    echo "  Command: grep 'Test data' /tmp/perf_test_metrics/test_data.txt | sort | uniq | wc -l"
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
    echo '    "command": "grep \"Test data\" /tmp/perf_test_metrics/test_data.txt | sort | uniq | wc -l",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 3: Capture processing test end timestamp
LOG_FILE="TC_COMPLEX_PERFORMANCE_TIMING_001_sequence-3_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ date +%s%N; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_processing_end_ns=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " processing_end_ns "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES processing_end_ns"
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
    echo "[PASS] Step 3: Capture processing test end timestamp"
else
    echo "[FAIL] Step 3: Capture processing test end timestamp"
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
    echo '    "step": 3,'
    echo '    "command": "date +%s%N",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 4: Calculate processing duration
LOG_FILE="TC_COMPLEX_PERFORMANCE_TIMING_001_sequence-3_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo \$(( (\${processing_end_ns} - \${processing_start_ns}) / 1000000 ))"
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
STEP_VAR_processing_duration_ms=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " processing_duration_ms "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES processing_duration_ms"
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
COND_EXPR="[[ \$processing_duration_ms -lt \${PERF_THRESHOLD_MS} ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        COND_EXPR=$(echo "$COND_EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$COND_EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
    echo 'Processing performance within threshold'
else
    echo 'WARNING: Processing performance exceeded threshold'
fi
echo "Processing: ${processing_duration_ms}ms"

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 4: Calculate processing duration"
else
    echo "[FAIL] Step 4: Calculate processing duration"
    echo "  Command: echo $(( (${processing_end_ns} - ${processing_start_ns}) / 1000000 ))"
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
    echo '    "command": "echo $(( (${processing_end_ns} - ${processing_start_ns}) / 1000000 ))",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 5: Record processing metrics
LOG_FILE="TC_COMPLEX_PERFORMANCE_TIMING_001_sequence-3_step-5.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="cat > /tmp/perf_test_metrics/processing.txt << EOF Processing Performance Metrics ============================== Start: \${processing_start_ns} End: \${processing_end_ns} Duration: \${processing_duration_ms}ms Unique Lines: \${unique_lines} Operations: grep, sort, uniq, wc Threshold: \${PERF_THRESHOLD_MS}ms Status: \$([ \${processing_duration_ms} -lt \${PERF_THRESHOLD_MS} ] && echo 'PASS' || echo 'WARN') EOF cat /tmp/perf_test_metrics/processing.txt"
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
EXPR="grep -q 'Duration: \${processing_duration_ms}ms' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 5: Record processing metrics"
else
    echo "[FAIL] Step 5: Record processing metrics"
    echo "  Command: cat > /tmp/perf_test_metrics/processing.txt << EOF Processing Performance Metrics ============================== Start: ${processing_start_ns} End: ${processing_end_ns} Duration: ${processing_duration_ms}ms Unique Lines: ${unique_lines} Operations: grep, sort, uniq, wc Threshold: ${PERF_THRESHOLD_MS}ms Status: $([ ${processing_duration_ms} -lt ${PERF_THRESHOLD_MS} ] && echo 'PASS' || echo 'WARN') EOF cat /tmp/perf_test_metrics/processing.txt"
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
    echo '    "command": "cat > /tmp/perf_test_metrics/processing.txt << EOF Processing Performance Metrics ============================== Start: ${processing_start_ns} End: ${processing_end_ns} Duration: ${processing_duration_ms}ms Unique Lines: ${unique_lines} Operations: grep, sort, uniq, wc Threshold: ${PERF_THRESHOLD_MS}ms Status: $([ ${processing_duration_ms} -lt ${PERF_THRESHOLD_MS} ] && echo \"PASS\" || echo \"WARN\") EOF cat /tmp/perf_test_metrics/processing.txt",'
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

# Test Sequence 4: Performance Analysis and Reporting
# Analyze collected metrics and generate performance report
TEST_SEQUENCE_ID=4
TEST_SEQUENCE_NAME='Performance Analysis and Reporting'
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
# performance: All performance tests have completed
# performance: All metrics files are available

# Step 1: Calculate total test execution time
LOG_FILE="TC_COMPLEX_PERFORMANCE_TIMING_001_sequence-4_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo \$(( (\${processing_end_ns} - \${baseline_start_ns}) / 1000000 ))"
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
STEP_VAR_total_duration_ms=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " total_duration_ms "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES total_duration_ms"
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
COND_EXPR="[[ \$total_duration_ms -gt 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        COND_EXPR=$(echo "$COND_EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$COND_EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
    echo 'Total duration is valid'
else
    echo 'Total duration calculation error'
fi
echo "Total: ${total_duration_ms}ms"

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 1: Calculate total test execution time"
else
    echo "[FAIL] Step 1: Calculate total test execution time"
    echo "  Command: echo $(( (${processing_end_ns} - ${baseline_start_ns}) / 1000000 ))"
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
    echo '    "command": "echo $(( (${processing_end_ns} - ${baseline_start_ns}) / 1000000 ))",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 2: Determine performance test status
LOG_FILE="TC_COMPLEX_PERFORMANCE_TIMING_001_sequence-4_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="if [[ \$fileio_duration_ms -lt \${PERF_THRESHOLD_MS} && \$processing_duration_ms -lt \${PERF_THRESHOLD_MS} ]]; then echo 'OVERALL_STATUS=PASS' else echo 'OVERALL_STATUS=WARN' fi"
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
STEP_VAR_overall_status=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(OVERALL_STATUS=([A-Z]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " overall_status "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES overall_status"
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
EXPR="grep -q 'OVERALL_STATUS=' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 2: Determine performance test status"
else
    echo "[FAIL] Step 2: Determine performance test status"
    echo "  Command: if [[ $fileio_duration_ms -lt ${PERF_THRESHOLD_MS} && $processing_duration_ms -lt ${PERF_THRESHOLD_MS} ]]; then echo 'OVERALL_STATUS=PASS' else echo 'OVERALL_STATUS=WARN' fi"
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
    echo '    "command": "if [[ $fileio_duration_ms -lt ${PERF_THRESHOLD_MS} && $processing_duration_ms -lt ${PERF_THRESHOLD_MS} ]]; then echo \"OVERALL_STATUS=PASS\" else echo \"OVERALL_STATUS=WARN\" fi",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 3: Generate comprehensive performance report
LOG_FILE="TC_COMPLEX_PERFORMANCE_TIMING_001_sequence-4_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="cat > /tmp/perf_test_metrics/performance_report.txt << EOF Performance Test Summary Report ================================
Test Configuration: - Iterations: \${PERF_TEST_ITERATIONS} - Workload Size: \${PERF_WORKLOAD_SIZE} - Threshold: \${PERF_THRESHOLD_MS}ms
Performance Results: ------------------- 1. Baseline Test: Duration: \${baseline_duration_ms}ms Iterations: \${baseline_iteration_count}
2. File I/O Test: Duration: \${fileio_duration_ms}ms Lines Written: \${lines_written} Status: \$([ \${fileio_duration_ms} -lt \${PERF_THRESHOLD_MS} ] && echo 'PASS' || echo 'WARN')
3. Processing Test: Duration: \${processing_duration_ms}ms Unique Lines: \${unique_lines} Status: \$([ \${processing_duration_ms} -lt \${PERF_THRESHOLD_MS} ] && echo 'PASS' || echo 'WARN')
Overall Summary: --------------- Total Execution Time: \${total_duration_ms}ms Overall Status: \${overall_status}
Performance Breakdown: - Baseline: \${baseline_duration_ms}ms (\$(echo \"scale=1; \${baseline_duration_ms} * 100 / \${total_duration_ms}\" | bc 2>/dev/null || echo 'N/A')%) - File I/O: \${fileio_duration_ms}ms (\$(echo \"scale=1; \${fileio_duration_ms} * 100 / \${total_duration_ms}\" | bc 2>/dev/null || echo 'N/A')%) - Processing: \${processing_duration_ms}ms (\$(echo \"scale=1; \${processing_duration_ms} * 100 / \${total_duration_ms}\" | bc 2>/dev/null || echo 'N/A')%)
Conclusion: \$([ \"\${overall_status}\" == \"PASS\" ] && echo 'All performance tests passed within threshold limits.' || echo 'Some performance tests exceeded threshold limits.') EOF cat /tmp/perf_test_metrics/performance_report.txt"
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
EXPR="grep -q 'Performance Test Summary Report' <<< \"\$COMMAND_OUTPUT\" && grep -q 'Overall Status: \${overall_status}' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 3: Generate comprehensive performance report"
else
    echo "[FAIL] Step 3: Generate comprehensive performance report"
    echo "  Command: cat > /tmp/perf_test_metrics/performance_report.txt << EOF Performance Test Summary Report ================================
Test Configuration: - Iterations: ${PERF_TEST_ITERATIONS} - Workload Size: ${PERF_WORKLOAD_SIZE} - Threshold: ${PERF_THRESHOLD_MS}ms
Performance Results: ------------------- 1. Baseline Test: Duration: ${baseline_duration_ms}ms Iterations: ${baseline_iteration_count}
2. File I/O Test: Duration: ${fileio_duration_ms}ms Lines Written: ${lines_written} Status: $([ ${fileio_duration_ms} -lt ${PERF_THRESHOLD_MS} ] && echo 'PASS' || echo 'WARN')
3. Processing Test: Duration: ${processing_duration_ms}ms Unique Lines: ${unique_lines} Status: $([ ${processing_duration_ms} -lt ${PERF_THRESHOLD_MS} ] && echo 'PASS' || echo 'WARN')
Overall Summary: --------------- Total Execution Time: ${total_duration_ms}ms Overall Status: ${overall_status}
Performance Breakdown: - Baseline: ${baseline_duration_ms}ms ($(echo \"scale=1; ${baseline_duration_ms} * 100 / ${total_duration_ms}\" | bc 2>/dev/null || echo 'N/A')%) - File I/O: ${fileio_duration_ms}ms ($(echo \"scale=1; ${fileio_duration_ms} * 100 / ${total_duration_ms}\" | bc 2>/dev/null || echo 'N/A')%) - Processing: ${processing_duration_ms}ms ($(echo \"scale=1; ${processing_duration_ms} * 100 / ${total_duration_ms}\" | bc 2>/dev/null || echo 'N/A')%)
Conclusion: $([ \"${overall_status}\" == \"PASS\" ] && echo 'All performance tests passed within threshold limits.' || echo 'Some performance tests exceeded threshold limits.') EOF cat /tmp/perf_test_metrics/performance_report.txt"
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
    echo '    "command": "cat > /tmp/perf_test_metrics/performance_report.txt << EOF Performance Test Summary Report ================================\nTest Configuration: - Iterations: ${PERF_TEST_ITERATIONS} - Workload Size: ${PERF_WORKLOAD_SIZE} - Threshold: ${PERF_THRESHOLD_MS}ms\nPerformance Results: ------------------- 1. Baseline Test: Duration: ${baseline_duration_ms}ms Iterations: ${baseline_iteration_count}\n2. File I/O Test: Duration: ${fileio_duration_ms}ms Lines Written: ${lines_written} Status: $([ ${fileio_duration_ms} -lt ${PERF_THRESHOLD_MS} ] && echo \"PASS\" || echo \"WARN\")\n3. Processing Test: Duration: ${processing_duration_ms}ms Unique Lines: ${unique_lines} Status: $([ ${processing_duration_ms} -lt ${PERF_THRESHOLD_MS} ] && echo \"PASS\" || echo \"WARN\")\nOverall Summary: --------------- Total Execution Time: ${total_duration_ms}ms Overall Status: ${overall_status}\nPerformance Breakdown: - Baseline: ${baseline_duration_ms}ms ($(echo \"scale=1; ${baseline_duration_ms} * 100 / ${total_duration_ms}\" | bc 2>/dev/null || echo \"N/A\")%) - File I/O: ${fileio_duration_ms}ms ($(echo \"scale=1; ${fileio_duration_ms} * 100 / ${total_duration_ms}\" | bc 2>/dev/null || echo \"N/A\")%) - Processing: ${processing_duration_ms}ms ($(echo \"scale=1; ${processing_duration_ms} * 100 / ${total_duration_ms}\" | bc 2>/dev/null || echo \"N/A\")%)\nConclusion: $([ \"${overall_status}\" == \"PASS\" ] && echo \"All performance tests passed within threshold limits.\" || echo \"Some performance tests exceeded threshold limits.\") EOF cat /tmp/perf_test_metrics/performance_report.txt",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 4: Validate performance metrics quality
LOG_FILE="TC_COMPLEX_PERFORMANCE_TIMING_001_sequence-4_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="cat << EOF Baseline: \${baseline_duration_ms}ms File I/O: \${fileio_duration_ms}ms Processing: \${processing_duration_ms}ms Total: \${total_duration_ms}ms EOF"
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

# General verifications
GENERAL_VERIFY_PASS_verify_all_metrics_numeric=false
EXPR="[[ \$baseline_duration_ms =~ ^[0-9]+\$ && \$fileio_duration_ms =~ ^[0-9]+\$ && \$processing_duration_ms =~ ^[0-9]+\$ && \$total_duration_ms =~ ^[0-9]+\$ ]]"
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
    GENERAL_VERIFY_PASS_verify_all_metrics_numeric=true
fi
GENERAL_VERIFY_PASS_verify_metrics_valid=false
EXPR="[[ \$baseline_duration_ms -ge 0 && \$fileio_duration_ms -ge 0 && \$processing_duration_ms -ge 0 && \$total_duration_ms -gt 0 ]]"
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
    GENERAL_VERIFY_PASS_verify_metrics_valid=true
fi
GENERAL_VERIFY_PASS_verify_total_duration_consistent=false
EXPR="[[ \$total_duration_ms -ge \$baseline_duration_ms ]]"
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
    GENERAL_VERIFY_PASS_verify_total_duration_consistent=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_all_metrics_numeric" = true ] && [ "$GENERAL_VERIFY_PASS_verify_metrics_valid" = true ] && [ "$GENERAL_VERIFY_PASS_verify_total_duration_consistent" = true ]; then
    echo "[PASS] Step 4: Validate performance metrics quality"
else
    echo "[FAIL] Step 4: Validate performance metrics quality"
    echo "  Command: cat << EOF Baseline: ${baseline_duration_ms}ms File I/O: ${fileio_duration_ms}ms Processing: ${processing_duration_ms}ms Total: ${total_duration_ms}ms EOF"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_all_metrics_numeric: $GENERAL_VERIFY_PASS_verify_all_metrics_numeric"
    echo "  GENERAL_VERIFY_PASS_verify_metrics_valid: $GENERAL_VERIFY_PASS_verify_metrics_valid"
    echo "  GENERAL_VERIFY_PASS_verify_total_duration_consistent: $GENERAL_VERIFY_PASS_verify_total_duration_consistent"
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
    echo '    "command": "cat << EOF Baseline: ${baseline_duration_ms}ms File I/O: ${fileio_duration_ms}ms Processing: ${processing_duration_ms}ms Total: ${total_duration_ms}ms EOF",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 5: Cleanup performance test artifacts
LOG_FILE="TC_COMPLEX_PERFORMANCE_TIMING_001_sequence-4_step-5.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ rm -rf /tmp/perf_test_metrics && echo 'Performance test cleanup complete'; } 2>&1 | tee "$LOG_FILE")
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
EXPR="grep -q 'cleanup complete' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 5: Cleanup performance test artifacts"
else
    echo "[FAIL] Step 5: Cleanup performance test artifacts"
    echo "  Command: rm -rf /tmp/perf_test_metrics && echo 'Performance test cleanup complete'"
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
    echo '    "command": "rm -rf /tmp/perf_test_metrics && echo \"Performance test cleanup complete\"",'
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
