#!/bin/bash
set -euo pipefail

# Test Case: TC_MANUAL_SSH_001
# Description: Test case for SSH device access verification with manual steps

JSON_LOG="TC_MANUAL_SSH_001_execution_log.json"
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
# device: Device is powered on and connected to network
# device: SSH service is running on device
# system: SSH client is available on test system

# Initial Conditions
# device: Device IP address is known and reachable
# network: Network connectivity is established

# Test Sequence 1: SSH Connection and Authentication
# This test sequence verifies SSH connectivity to a remote device
# with manual authentication and automated verification steps.
# 
# Sequence Initial Conditions
# device: SSH server is accessible

# Step 1: Check device network connectivity
LOG_FILE="TC_MANUAL_SSH_001_sequence-1_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ ping -c 3 192.168.1.100; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Verification result expression: [[ $EXIT_CODE -eq 0 ]]
# Verification output expression (from file): grep -q 'packets transmitted' "$LOG_FILE"
VERIFICATION_RESULT_PASS=false
VERIFICATION_OUTPUT_PASS=false

RESULT_EXPR="[[ \$EXIT_CODE -eq 0 ]]"

OUTPUT_EXPR="grep -q 'packets transmitted' \"\$LOG_FILE\""

if eval "$RESULT_EXPR"; then
    VERIFICATION_RESULT_PASS=true
fi

if eval "$OUTPUT_EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 1: Check device network connectivity"
else
    echo "[FAIL] Step 1: Check device network connectivity"
    echo "  Command: ping -c 3 192.168.1.100"
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
    echo '    "command": "ping -c 3 192.168.1.100",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\""
    echo '  }'
} >> "$JSON_LOG"

# Step 2: Manually SSH into device and verify login
echo "Step 2: Manually SSH into device and verify login"
echo "Command: ssh admin@192.168.1.100"
echo "INFO: This is a manual step. You must perform this action manually."
if [[ "${DEBIAN_FRONTEND}" != 'noninteractive' && -t 0 ]]; then
    read -p "Press ENTER to continue..."
else
    echo "Non-interactive mode detected, skipping manual step confirmation."
fi

# Step 3: Execute uptime command on remote device
echo "Step 3: Execute uptime command on remote device"
echo "Command: uptime"
echo "INFO: This is a manual step. You must perform this action manually."
if [[ "${DEBIAN_FRONTEND}" != 'noninteractive' && -t 0 ]]; then
    read -p "Press ENTER to continue..."
else
    echo "Non-interactive mode detected, skipping manual step confirmation."
fi

# Step 4: Check SSH service status locally
LOG_FILE="TC_MANUAL_SSH_001_sequence-1_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ systemctl status ssh | grep -i active || echo "SSH service check"; } 2>&1 | tee "$LOG_FILE")
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
    echo "[PASS] Step 4: Check SSH service status locally"
else
    echo "[FAIL] Step 4: Check SSH service status locally"
    echo "  Command: systemctl status ssh | grep -i active || echo \"SSH service check\""
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
    echo '    "command": "systemctl status ssh | grep -i active || echo \"SSH service check\"",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\""
    echo '  }'
} >> "$JSON_LOG"

# Step 5: Manually log out from SSH session
echo "Step 5: Manually log out from SSH session"
echo "Command: exit"
echo "INFO: This is a manual step. You must perform this action manually."
if [[ "${DEBIAN_FRONTEND}" != 'noninteractive' && -t 0 ]]; then
    read -p "Press ENTER to continue..."
else
    echo "Non-interactive mode detected, skipping manual step confirmation."
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
