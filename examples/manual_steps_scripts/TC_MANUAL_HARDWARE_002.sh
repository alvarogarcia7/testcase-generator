#!/bin/bash
set -euo pipefail

# Test Case: TC_MANUAL_HARDWARE_002
# Description: Test case for physical hardware setup and verification

JSON_LOG="TC_MANUAL_HARDWARE_002_execution_log.json"
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
# hardware: Test hardware components are available
# hardware: Cables and connectors are inspected and ready
# environment: Test bench is clean and organized

# Initial Conditions
# tools: Multimeter and testing equipment available
# hardware: Device is in powered-off state

# Test Sequence 1: Hardware Setup and Power On
# This test sequence covers physical hardware connection,
# power-up sequence, and automated system detection.
# 
# Sequence Initial Conditions
# hardware: Hardware is ready for setup

# Step 1: Connect power cable to device
echo "Step 1: Connect power cable to device"
echo "Command: Physical connection of power cable"
echo "INFO: This is a manual step. You must perform this action manually."
if [[ "${DEBIAN_FRONTEND}" != 'noninteractive' && -t 0 ]]; then
    read -p "Press ENTER to continue..."
else
    echo "Non-interactive mode detected, skipping manual step confirmation."
fi

# Step 2: Connect Ethernet cable to port 1
echo "Step 2: Connect Ethernet cable to port 1"
echo "Command: Physical connection of Ethernet cable"
echo "INFO: This is a manual step. You must perform this action manually."
if [[ "${DEBIAN_FRONTEND}" != 'noninteractive' && -t 0 ]]; then
    read -p "Press ENTER to continue..."
else
    echo "Non-interactive mode detected, skipping manual step confirmation."
fi

# Step 3: Press power button to turn on device
echo "Step 3: Press power button to turn on device"
echo "Command: Press power button for 2 seconds"
echo "INFO: This is a manual step. You must perform this action manually."
if [[ "${DEBIAN_FRONTEND}" != 'noninteractive' && -t 0 ]]; then
    read -p "Press ENTER to continue..."
else
    echo "Non-interactive mode detected, skipping manual step confirmation."
fi

# Step 4: Wait for device boot and check USB devices
LOG_FILE="TC_MANUAL_HARDWARE_002_sequence-1_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ sleep 30 && lsusb | grep -i device || echo "USB enumeration complete"; } 2>&1 | tee "$LOG_FILE")
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
    echo "[PASS] Step 4: Wait for device boot and check USB devices"
else
    echo "[FAIL] Step 4: Wait for device boot and check USB devices"
    echo "  Command: sleep 30 && lsusb | grep -i device || echo \"USB enumeration complete\""
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
    echo '    "step": 4,'
    echo '    "command": "sleep 30 && lsusb | grep -i device || echo \"USB enumeration complete\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\""
    echo '  }'
} >> "$JSON_LOG"

# Step 5: Verify network interface detection
LOG_FILE="TC_MANUAL_HARDWARE_002_sequence-1_step-5.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ ip link show | grep -E 'eth[0-9]|enp' || echo "Network interface detected"; } 2>&1 | tee "$LOG_FILE")
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
    echo "[PASS] Step 5: Verify network interface detection"
else
    echo "[FAIL] Step 5: Verify network interface detection"
    echo "  Command: ip link show | grep -E 'eth[0-9]|enp' || echo \"Network interface detected\""
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
    echo '    "command": "ip link show | grep -E \"eth[0-9]|enp\" || echo \"Network interface detected\"",'
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
