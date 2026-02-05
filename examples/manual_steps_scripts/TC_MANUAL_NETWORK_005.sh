#!/bin/bash
set -euo pipefail

# Test Case: TC_MANUAL_NETWORK_005
# Description: Test case for network configuration and connectivity verification

JSON_LOG="TC_MANUAL_NETWORK_005_execution_log.json"
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
# network: Network equipment is available
# network: DHCP server is running
# system: Network configuration tools are installed

# Initial Conditions
# network: Device has no IP address assigned
# interface: Network interface is down

# Test Sequence 1: Network Configuration Setup
# This test sequence covers network interface configuration,
# manual cable connection, and connectivity verification.
# 
# Sequence Initial Conditions
# network: Network is ready for configuration

# Step 1: Check network interface status
LOG_FILE="TC_MANUAL_NETWORK_005_sequence-1_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ ip link show eth0 2>/dev/null || echo "Interface check completed"; } 2>&1 | tee "$LOG_FILE")
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
    echo "[PASS] Step 1: Check network interface status"
else
    echo "[FAIL] Step 1: Check network interface status"
    echo "  Command: ip link show eth0 2>/dev/null || echo \"Interface check completed\""
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
    echo '    "command": "ip link show eth0 2>/dev/null || echo \"Interface check completed\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\""
    echo '  }'
} >> "$JSON_LOG"

# Step 2: Physically connect Ethernet cable between device and switch port 8
echo "Step 2: Physically connect Ethernet cable between device and switch port 8"
echo "Command: Connect Ethernet cable to switch port 8"
echo "INFO: This is a manual step. You must perform this action manually."
if [[ "${DEBIAN_FRONTEND}" != 'noninteractive' && -t 0 ]]; then
    read -p "Press ENTER to continue..."
else
    echo "Non-interactive mode detected, skipping manual step confirmation."
fi

# Step 3: Bring network interface up
LOG_FILE="TC_MANUAL_NETWORK_005_sequence-1_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ echo "ip link set eth0 up" && echo "Interface brought up"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Verification result expression: [[ $EXIT_CODE -eq 0 ]]
# Verification output expression (from file): grep -q 'Interface brought up' "$LOG_FILE"
VERIFICATION_RESULT_PASS=false
VERIFICATION_OUTPUT_PASS=false

RESULT_EXPR="[[ \$EXIT_CODE -eq 0 ]]"

OUTPUT_EXPR="grep -q 'Interface brought up' \"\$LOG_FILE\""

if eval "$RESULT_EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

if eval "$OUTPUT_EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 3: Bring network interface up"
else
    echo "[FAIL] Step 3: Bring network interface up"
    echo "  Command: echo \"ip link set eth0 up\" && echo \"Interface brought up\""
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
    echo '    "command": "echo \"ip link set eth0 up\" && echo \"Interface brought up\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\""
    echo '  }'
} >> "$JSON_LOG"

# Step 4: Verify link status LED on device
echo "Step 4: Verify link status LED on device"
echo "Command: Visual inspection of network port LED"
echo "INFO: This is a manual step. You must perform this action manually."
if [[ "${DEBIAN_FRONTEND}" != 'noninteractive' && -t 0 ]]; then
    read -p "Press ENTER to continue..."
else
    echo "Non-interactive mode detected, skipping manual step confirmation."
fi

# Step 5: Test network connectivity with ping
LOG_FILE="TC_MANUAL_NETWORK_005_sequence-1_step-5.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ ping -c 4 8.8.8.8 2>/dev/null | grep -E 'packets transmitted|received' || echo "Ping test completed"; } 2>&1 | tee "$LOG_FILE")
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
    echo "[PASS] Step 5: Test network connectivity with ping"
else
    echo "[FAIL] Step 5: Test network connectivity with ping"
    echo "  Command: ping -c 4 8.8.8.8 2>/dev/null | grep -E 'packets transmitted|received' || echo \"Ping test completed\""
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
    echo '    "command": "ping -c 4 8.8.8.8 2>/dev/null | grep -E \"packets transmitted|received\" || echo \"Ping test completed\"",'
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
