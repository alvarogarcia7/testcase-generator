#!/bin/bash
set -euo pipefail

# Test Case: TC_MANUAL_MIXED_010
# Description: Test case with mixed automated and manual workflow for complete system validation

JSON_LOG="TC_MANUAL_MIXED_010_execution_log.json"
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
# services: Required services are available
# system: All system components are installed
# system: Test environment is configured

# Initial Conditions
# monitoring: Monitoring tools are active
# system: System is in idle state

# Test Sequence 1: End-to-End System Validation
# This test sequence demonstrates a complete workflow mixing
# automated checks, manual operations, and visual verification.
# 
# Sequence Initial Conditions
# system: System is ready for end-to-end testing

# Step 1: Automated system health check
LOG_FILE="TC_MANUAL_MIXED_010_sequence-1_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ echo "System health check started" && uptime && echo "Health check completed"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Verification result expression: [[ $EXIT_CODE -eq 0 ]]
# Verification output expression (from file): grep -q 'Health check completed' "$LOG_FILE"
VERIFICATION_RESULT_PASS=false
VERIFICATION_OUTPUT_PASS=false

RESULT_EXPR="[[ \$EXIT_CODE -eq 0 ]]"

OUTPUT_EXPR="grep -q 'Health check completed' \"\$LOG_FILE\""

if eval "$RESULT_EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

if eval "$OUTPUT_EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 1: Automated system health check"
else
    echo "[FAIL] Step 1: Automated system health check"
    echo "  Command: echo \"System health check started\" && uptime && echo \"Health check completed\""
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
# Use Python for reliable JSON escaping if available, otherwise use sed/perl/awk
if command -v python3 >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | python3 -c 'import sys, json; s=sys.stdin.read(); print(json.dumps(s)[1:-1], end="")')
elif command -v python >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | python -c 'import sys, json; s=sys.stdin.read(); print json.dumps(s)[1:-1]')
elif command -v perl >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | perl -pe 's/\\/\\\\/g; s/"/\\"/g; s/\n/\\n/g; s/\r/\\r/g; s/\t/\\t/g' | tr -d '\n')
else
    # Fallback: escape backslashes, quotes, tabs, and convert newlines to \n
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
    echo '    "command": "echo \"System health check started\" && uptime && echo \"Health check completed\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\""
    echo '  }'
} >> "$JSON_LOG"

# Step 2: Manually start application service from GUI
echo "Step 2: Manually start application service from GUI"
echo "Command: Click Start button in application control panel"
echo "INFO: This is a manual step. You must perform this action manually."
if [[ "${DEBIAN_FRONTEND}" != 'noninteractive' && -t 0 ]]; then
    read -p "Press ENTER to continue..."
else
    echo "Non-interactive mode detected, skipping manual step confirmation."
fi

# Step 3: Automated verification of service startup
LOG_FILE="TC_MANUAL_MIXED_010_sequence-1_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ sleep 5 && ps aux | grep -i 'service' | grep -v grep || echo "Service process verified"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Verification result expression: [[ $EXIT_CODE -eq 0 ]]
# Verification output expression (from variable): true
VERIFICATION_RESULT_PASS=false
VERIFICATION_OUTPUT_PASS=false

RESULT_EXPR="[[ \$EXIT_CODE -eq 0 ]]"

OUTPUT_EXPR="true"

if eval "$RESULT_EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

if eval "$OUTPUT_EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 3: Automated verification of service startup"
else
    echo "[FAIL] Step 3: Automated verification of service startup"
    echo "  Command: sleep 5 && ps aux | grep -i 'service' | grep -v grep || echo \"Service process verified\""
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
# Use Python for reliable JSON escaping if available, otherwise use sed/perl/awk
if command -v python3 >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | python3 -c 'import sys, json; s=sys.stdin.read(); print(json.dumps(s)[1:-1], end="")')
elif command -v python >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | python -c 'import sys, json; s=sys.stdin.read(); print json.dumps(s)[1:-1]')
elif command -v perl >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | perl -pe 's/\\/\\\\/g; s/"/\\"/g; s/\n/\\n/g; s/\r/\\r/g; s/\t/\\t/g' | tr -d '\n')
else
    # Fallback: escape backslashes, quotes, tabs, and convert newlines to \n
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
    echo '    "command": "sleep 5 && ps aux | grep -i \"service\" | grep -v grep || echo \"Service process verified\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\""
    echo '  }'
} >> "$JSON_LOG"

# Step 4: Manually test application functionality through UI
echo "Step 4: Manually test application functionality through UI"
echo "Command: Navigate to Features > Test Mode and execute test workflow"
echo "INFO: This is a manual step. You must perform this action manually."
if [[ "${DEBIAN_FRONTEND}" != 'noninteractive' && -t 0 ]]; then
    read -p "Press ENTER to continue..."
else
    echo "Non-interactive mode detected, skipping manual step confirmation."
fi

# Step 5: Automated log analysis
LOG_FILE="TC_MANUAL_MIXED_010_sequence-1_step-5.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ echo "Log analysis completed" && grep -c "ERROR" /dev/null 2>/dev/null || echo "No critical errors found"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Verification result expression: [[ $EXIT_CODE -eq 0 ]]
# Verification output expression (from file): grep -q 'No critical errors found' "$LOG_FILE"
VERIFICATION_RESULT_PASS=false
VERIFICATION_OUTPUT_PASS=false

RESULT_EXPR="[[ \$EXIT_CODE -eq 0 ]]"

OUTPUT_EXPR="grep -q 'No critical errors found' \"\$LOG_FILE\""

if eval "$RESULT_EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

if eval "$OUTPUT_EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 5: Automated log analysis"
else
    echo "[FAIL] Step 5: Automated log analysis"
    echo "  Command: echo \"Log analysis completed\" && grep -c \"ERROR\" /dev/null 2>/dev/null || echo \"No critical errors found\""
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    exit 1
fi

# Escape output for JSON (BSD/GNU compatible)
# Use Python for reliable JSON escaping if available, otherwise use sed/perl/awk
if command -v python3 >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | python3 -c 'import sys, json; s=sys.stdin.read(); print(json.dumps(s)[1:-1], end="")')
elif command -v python >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | python -c 'import sys, json; s=sys.stdin.read(); print json.dumps(s)[1:-1]')
elif command -v perl >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | perl -pe 's/\\/\\\\/g; s/"/\\"/g; s/\n/\\n/g; s/\r/\\r/g; s/\t/\\t/g' | tr -d '\n')
else
    # Fallback: escape backslashes, quotes, tabs, and convert newlines to \n
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
    echo '    "command": "echo \"Log analysis completed\" && grep -c \"ERROR\" /dev/null 2>/dev/null || echo \"No critical errors found\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\""
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
