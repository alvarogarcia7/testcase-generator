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

# Test Case: TC_COMPLEX_SECURITY_AUTH_API_001
# Description: Security test with authentication token capture and API calls - demonstrates secure credential handling and API testing

JSON_LOG="TC_COMPLEX_SECURITY_AUTH_API_001_execution_log.json"
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
# environment: Secure credential storage is available
# environment: Network connectivity for API simulation
# security: Test credentials are isolated from production
# security: API endpoints are accessible for testing
# security: Authentication mechanism is functional

# Initial Conditions
# api: API simulation infrastructure is available
# security: Test user credentials are prepared
# security: Authentication tokens can be generated

# Test Sequence 1: Authentication and Token Generation
# Authenticate user and capture secure token
TEST_SEQUENCE_ID=1
TEST_SEQUENCE_NAME='Authentication and Token Generation'
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
# security: User credentials are available
# security: Authentication endpoint is accessible

# Step 1: Generate timestamp for authentication request
LOG_FILE="TC_COMPLEX_SECURITY_AUTH_API_001_sequence-1_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ date +%s; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_auth_timestamp=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " auth_timestamp "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES auth_timestamp"
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
GENERAL_VERIFY_PASS_verify_auth_timestamp=false
EXPR="[[ \$auth_timestamp =~ ^[0-9]+\$ ]]"
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
    GENERAL_VERIFY_PASS_verify_auth_timestamp=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_auth_timestamp" = true ]; then
    echo "[PASS] Step 1: Generate timestamp for authentication request"
else
    echo "[FAIL] Step 1: Generate timestamp for authentication request"
    echo "  Command: date +%s"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_auth_timestamp: $GENERAL_VERIFY_PASS_verify_auth_timestamp"
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

# Step 2: Simulate authentication request and generate secure token
LOG_FILE="TC_COMPLEX_SECURITY_AUTH_API_001_sequence-1_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo 'AUTH_REQUEST: '\${TEST_USERNAME} && echo \${TEST_USERNAME}_\${auth_timestamp}_\${TEST_USER_ROLE} | md5sum 2>/dev/null | cut -d' ' -f1 || echo \${TEST_USERNAME}_\${auth_timestamp}_\${TEST_USER_ROLE} | md5 2>/dev/null | cut -d' ' -f1 || echo 'token_'\$RANDOM\$RANDOM"
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
STEP_VAR_auth_token=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([a-f0-9]+|token_[0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " auth_token "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES auth_token"
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
EXPR="[[ \"\$COMMAND_OUTPUT\" =~ [a-f0-9]+ ]] || [[ \"\$COMMAND_OUTPUT\" =~ token_[0-9]+ ]]"
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
GENERAL_VERIFY_PASS_verify_token_generated=false
EXPR="[[ -n \"\$auth_token\" ]]"
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
    GENERAL_VERIFY_PASS_verify_token_generated=true
fi
GENERAL_VERIFY_PASS_verify_token_format=false
EXPR="[[ \$auth_token =~ ^[a-f0-9]+\$ ]] || [[ \$auth_token =~ ^token_[0-9]+\$ ]]"
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
    GENERAL_VERIFY_PASS_verify_token_format=true
fi
GENERAL_VERIFY_PASS_verify_token_length=false
EXPR="[[ \${#auth_token} -ge 8 ]]"
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
    GENERAL_VERIFY_PASS_verify_token_length=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_token_generated" = true ] && [ "$GENERAL_VERIFY_PASS_verify_token_format" = true ] && [ "$GENERAL_VERIFY_PASS_verify_token_length" = true ]; then
    echo "[PASS] Step 2: Simulate authentication request and generate secure token"
else
    echo "[FAIL] Step 2: Simulate authentication request and generate secure token"
    echo "  Command: echo 'AUTH_REQUEST: '${TEST_USERNAME} && echo ${TEST_USERNAME}_${auth_timestamp}_${TEST_USER_ROLE} | md5sum 2>/dev/null | cut -d' ' -f1 || echo ${TEST_USERNAME}_${auth_timestamp}_${TEST_USER_ROLE} | md5 2>/dev/null | cut -d' ' -f1 || echo 'token_'$RANDOM$RANDOM"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_token_generated: $GENERAL_VERIFY_PASS_verify_token_generated"
    echo "  GENERAL_VERIFY_PASS_verify_token_format: $GENERAL_VERIFY_PASS_verify_token_format"
    echo "  GENERAL_VERIFY_PASS_verify_token_length: $GENERAL_VERIFY_PASS_verify_token_length"
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
    echo '    "command": "echo \"AUTH_REQUEST: \"${TEST_USERNAME} && echo ${TEST_USERNAME}_${auth_timestamp}_${TEST_USER_ROLE} | md5sum 2>/dev/null | cut -d\" \" -f1 || echo ${TEST_USERNAME}_${auth_timestamp}_${TEST_USER_ROLE} | md5 2>/dev/null | cut -d\" \" -f1 || echo \"token_\"$RANDOM$RANDOM",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 3: Calculate token expiry timestamp
LOG_FILE="TC_COMPLEX_SECURITY_AUTH_API_001_sequence-1_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo \$((\${auth_timestamp} + \${TOKEN_EXPIRY_SECONDS}))"
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
STEP_VAR_token_expiry=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " token_expiry "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES token_expiry"
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
GENERAL_VERIFY_PASS_verify_expiry_after_auth=false
EXPR="[[ \$token_expiry -gt \$auth_timestamp ]]"
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
    GENERAL_VERIFY_PASS_verify_expiry_after_auth=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_expiry_after_auth" = true ]; then
    echo "[PASS] Step 3: Calculate token expiry timestamp"
else
    echo "[FAIL] Step 3: Calculate token expiry timestamp"
    echo "  Command: echo $((${auth_timestamp} + ${TOKEN_EXPIRY_SECONDS}))"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_expiry_after_auth: $GENERAL_VERIFY_PASS_verify_expiry_after_auth"
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
    echo '    "command": "echo $((${auth_timestamp} + ${TOKEN_EXPIRY_SECONDS}))",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 4: Store authentication session securely
LOG_FILE="TC_COMPLEX_SECURITY_AUTH_API_001_sequence-1_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="mkdir -p /tmp/secure_session && cat > /tmp/secure_session/auth_\${auth_token:0:8}.sec << EOF # Secure Session Data - DO NOT COMMIT session_token=\${auth_token} username=\${TEST_USERNAME} role=\${TEST_USER_ROLE} authenticated_at=\${auth_timestamp} expires_at=\${token_expiry} api_base=\${API_BASE_URL} status=ACTIVE EOF chmod 600 /tmp/secure_session/auth_\${auth_token:0:8}.sec && cat /tmp/secure_session/auth_\${auth_token:0:8}.sec"
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
EXPR="grep -q 'session_token=\${auth_token}' <<< \"\$COMMAND_OUTPUT\" && grep -q 'status=ACTIVE' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 4: Store authentication session securely"
else
    echo "[FAIL] Step 4: Store authentication session securely"
    echo "  Command: mkdir -p /tmp/secure_session && cat > /tmp/secure_session/auth_${auth_token:0:8}.sec << EOF # Secure Session Data - DO NOT COMMIT session_token=${auth_token} username=${TEST_USERNAME} role=${TEST_USER_ROLE} authenticated_at=${auth_timestamp} expires_at=${token_expiry} api_base=${API_BASE_URL} status=ACTIVE EOF chmod 600 /tmp/secure_session/auth_${auth_token:0:8}.sec && cat /tmp/secure_session/auth_${auth_token:0:8}.sec"
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
    echo '    "command": "mkdir -p /tmp/secure_session && cat > /tmp/secure_session/auth_${auth_token:0:8}.sec << EOF # Secure Session Data - DO NOT COMMIT session_token=${auth_token} username=${TEST_USERNAME} role=${TEST_USER_ROLE} authenticated_at=${auth_timestamp} expires_at=${token_expiry} api_base=${API_BASE_URL} status=ACTIVE EOF chmod 600 /tmp/secure_session/auth_${auth_token:0:8}.sec && cat /tmp/secure_session/auth_${auth_token:0:8}.sec",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 5: Generate authorization header for API requests
LOG_FILE="TC_COMPLEX_SECURITY_AUTH_API_001_sequence-1_step-5.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo 'Authorization: Bearer '\${auth_token}"
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
STEP_VAR_auth_header=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\((Authorization: Bearer [a-f0-9]+|Authorization: Bearer token_[0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " auth_header "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES auth_header"
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
EXPR="grep -q 'Authorization: Bearer' <<< \"\$COMMAND_OUTPUT\""
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
GENERAL_VERIFY_PASS_verify_auth_header_format=false
EXPR="[[ -n \"\$auth_header\" ]]"
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
    GENERAL_VERIFY_PASS_verify_auth_header_format=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_auth_header_format" = true ]; then
    echo "[PASS] Step 5: Generate authorization header for API requests"
else
    echo "[FAIL] Step 5: Generate authorization header for API requests"
    echo "  Command: echo 'Authorization: Bearer '${auth_token}"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_auth_header_format: $GENERAL_VERIFY_PASS_verify_auth_header_format"
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
    echo '    "command": "echo \"Authorization: Bearer \"${auth_token}",'
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

# Test Sequence 2: API Resource Access with Token
# Use authenticated token to access API resources
TEST_SEQUENCE_ID=2
TEST_SEQUENCE_NAME='API Resource Access with Token'
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
# api: API endpoints are accessible
# security: Valid authentication token is available
# security: Token is not expired

# Step 1: Simulate GET request to user profile endpoint
LOG_FILE="TC_COMPLEX_SECURITY_AUTH_API_001_sequence-2_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo 'GET \${API_BASE_URL}/users/\${TEST_USERNAME}' && echo 'Headers: '\${auth_header} && cat << EOF { \"user_id\": \"\$(echo \${TEST_USERNAME} | md5sum 2>/dev/null | cut -c1-8 || echo \$RANDOM)\", \"username\": \"\${TEST_USERNAME}\", \"role\": \"\${TEST_USER_ROLE}\", \"authenticated\": true, \"token_valid\": true } EOF"
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
STEP_VAR_user_id=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\("user_id": "([a-f0-9]+)"\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " user_id "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES user_id"
fi
STEP_VAR_auth_status=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\("authenticated": (true|false)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " auth_status "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES auth_status"
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
EXPR="grep -q '\"username\": \"\${TEST_USERNAME}\"' <<< \"\$COMMAND_OUTPUT\" && grep -q '\"authenticated\": true' <<< \"\$COMMAND_OUTPUT\""
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
GENERAL_VERIFY_PASS_verify_user_id_captured=false
EXPR="[[ -n \"\$user_id\" ]]"
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
    GENERAL_VERIFY_PASS_verify_user_id_captured=true
fi
GENERAL_VERIFY_PASS_verify_authenticated=false
EXPR="[[ \"\$auth_status\" == \"true\" ]]"
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
    GENERAL_VERIFY_PASS_verify_authenticated=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_user_id_captured" = true ] && [ "$GENERAL_VERIFY_PASS_verify_authenticated" = true ]; then
    echo "[PASS] Step 1: Simulate GET request to user profile endpoint"
else
    echo "[FAIL] Step 1: Simulate GET request to user profile endpoint"
    echo "  Command: echo 'GET ${API_BASE_URL}/users/${TEST_USERNAME}' && echo 'Headers: '${auth_header} && cat << EOF { \"user_id\": \"$(echo ${TEST_USERNAME} | md5sum 2>/dev/null | cut -c1-8 || echo $RANDOM)\", \"username\": \"${TEST_USERNAME}\", \"role\": \"${TEST_USER_ROLE}\", \"authenticated\": true, \"token_valid\": true } EOF"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_user_id_captured: $GENERAL_VERIFY_PASS_verify_user_id_captured"
    echo "  GENERAL_VERIFY_PASS_verify_authenticated: $GENERAL_VERIFY_PASS_verify_authenticated"
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
    echo '    "command": "echo \"GET ${API_BASE_URL}/users/${TEST_USERNAME}\" && echo \"Headers: \"${auth_header} && cat << EOF { \"user_id\": \"$(echo ${TEST_USERNAME} | md5sum 2>/dev/null | cut -c1-8 || echo $RANDOM)\", \"username\": \"${TEST_USERNAME}\", \"role\": \"${TEST_USER_ROLE}\", \"authenticated\": true, \"token_valid\": true } EOF",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 2: Simulate GET request to resources based on role
LOG_FILE="TC_COMPLEX_SECURITY_AUTH_API_001_sequence-2_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo 'GET \${API_BASE_URL}/resources' && echo 'Headers: '\${auth_header} && case \"\${TEST_USER_ROLE}\" in admin) echo '{\"resources\": [\"all\", \"users\", \"settings\", \"logs\", \"reports\"], \"access_level\": \"full\"}' ;; user) echo '{\"resources\": [\"profile\", \"documents\", \"reports\"], \"access_level\": \"standard\"}' ;; readonly) echo '{\"resources\": [\"public\", \"help\"], \"access_level\": \"limited\"}' ;; *) echo '{\"resources\": [], \"access_level\": \"none\"}' ;; esac"
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
STEP_VAR_access_level=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\("access_level": "([a-z]+)"\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " access_level "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES access_level"
fi
STEP_VAR_resource_count=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(\\[([^]]+)\\]\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " resource_count "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES resource_count"
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
COND_EXPR="[[ \"\${TEST_USER_ROLE}\" == \"admin\" ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        COND_EXPR=$(echo "$COND_EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$COND_EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
    grep -q '"access_level": "full"' <<< "$COMMAND_OUTPUT"
else
    grep -q '"resources":' <<< "$COMMAND_OUTPUT"
fi
echo "Access level: ${access_level}"

# General verifications
GENERAL_VERIFY_PASS_verify_access_level_captured=false
EXPR="[[ -n \"\$access_level\" ]]"
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
    GENERAL_VERIFY_PASS_verify_access_level_captured=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_access_level_captured" = true ]; then
    echo "[PASS] Step 2: Simulate GET request to resources based on role"
else
    echo "[FAIL] Step 2: Simulate GET request to resources based on role"
    echo "  Command: echo 'GET ${API_BASE_URL}/resources' && echo 'Headers: '${auth_header} && case \"${TEST_USER_ROLE}\" in admin) echo '{\"resources\": [\"all\", \"users\", \"settings\", \"logs\", \"reports\"], \"access_level\": \"full\"}' ;; user) echo '{\"resources\": [\"profile\", \"documents\", \"reports\"], \"access_level\": \"standard\"}' ;; readonly) echo '{\"resources\": [\"public\", \"help\"], \"access_level\": \"limited\"}' ;; *) echo '{\"resources\": [], \"access_level\": \"none\"}' ;; esac"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_access_level_captured: $GENERAL_VERIFY_PASS_verify_access_level_captured"
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
    echo '    "command": "echo \"GET ${API_BASE_URL}/resources\" && echo \"Headers: \"${auth_header} && case \"${TEST_USER_ROLE}\" in admin) echo \"{\"resources\": [\"all\", \"users\", \"settings\", \"logs\", \"reports\"], \"access_level\": \"full\"}\" ;; user) echo \"{\"resources\": [\"profile\", \"documents\", \"reports\"], \"access_level\": \"standard\"}\" ;; readonly) echo \"{\"resources\": [\"public\", \"help\"], \"access_level\": \"limited\"}\" ;; *) echo \"{\"resources\": [], \"access_level\": \"none\"}\" ;; esac",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 3: Simulate POST request to create resource
LOG_FILE="TC_COMPLEX_SECURITY_AUTH_API_001_sequence-2_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo 'POST \${API_BASE_URL}/resources' && echo 'Headers: '\${auth_header} && echo 'Body: {\"name\": \"test_resource\", \"owner\": \"\${TEST_USERNAME}\"}' && date +%s"
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
STEP_VAR_request_timestamp=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)$\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " request_timestamp "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES request_timestamp"
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
EXPR="grep -q 'POST' <<< \"\$COMMAND_OUTPUT\" && grep -q '\${auth_header}' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 3: Simulate POST request to create resource"
else
    echo "[FAIL] Step 3: Simulate POST request to create resource"
    echo "  Command: echo 'POST ${API_BASE_URL}/resources' && echo 'Headers: '${auth_header} && echo 'Body: {\"name\": \"test_resource\", \"owner\": \"${TEST_USERNAME}\"}' && date +%s"
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
    echo '    "command": "echo \"POST ${API_BASE_URL}/resources\" && echo \"Headers: \"${auth_header} && echo \"Body: {\"name\": \"test_resource\", \"owner\": \"${TEST_USERNAME}\"}\" && date +%s",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 4: Simulate API response with resource ID
LOG_FILE="TC_COMPLEX_SECURITY_AUTH_API_001_sequence-2_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ echo 'resource_'$RANDOM$RANDOM | md5sum 2>/dev/null | cut -d' ' -f1 | cut -c1-16 || echo 'resource_'$RANDOM; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e

# Capture variables from output
STEP_VAR_resource_id=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([a-f0-9]+|resource_[0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " resource_id "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES resource_id"
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
EXPR="[[ -n \"\$COMMAND_OUTPUT\" ]]"
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
GENERAL_VERIFY_PASS_verify_resource_id_captured=false
EXPR="[[ -n \"\$resource_id\" ]]"
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
    GENERAL_VERIFY_PASS_verify_resource_id_captured=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_resource_id_captured" = true ]; then
    echo "[PASS] Step 4: Simulate API response with resource ID"
else
    echo "[FAIL] Step 4: Simulate API response with resource ID"
    echo "  Command: echo 'resource_'$RANDOM$RANDOM | md5sum 2>/dev/null | cut -d' ' -f1 | cut -c1-16 || echo 'resource_'$RANDOM"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_resource_id_captured: $GENERAL_VERIFY_PASS_verify_resource_id_captured"
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
    echo '    "command": "echo \"resource_\"$RANDOM$RANDOM | md5sum 2>/dev/null | cut -d\" \" -f1 | cut -c1-16 || echo \"resource_\"$RANDOM",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 5: Log API request audit trail
LOG_FILE="TC_COMPLEX_SECURITY_AUTH_API_001_sequence-2_step-5.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="cat >> /tmp/secure_session/api_audit.log << EOF [\${request_timestamp}] POST /resources - User: \${TEST_USERNAME} (\${user_id}) - Token: \${auth_token:0:8}... - Resource: \${resource_id} - Status: SUCCESS EOF tail -1 /tmp/secure_session/api_audit.log"
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
EXPR="grep -q 'Status: SUCCESS' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 5: Log API request audit trail"
else
    echo "[FAIL] Step 5: Log API request audit trail"
    echo "  Command: cat >> /tmp/secure_session/api_audit.log << EOF [${request_timestamp}] POST /resources - User: ${TEST_USERNAME} (${user_id}) - Token: ${auth_token:0:8}... - Resource: ${resource_id} - Status: SUCCESS EOF tail -1 /tmp/secure_session/api_audit.log"
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
    echo '    "command": "cat >> /tmp/secure_session/api_audit.log << EOF [${request_timestamp}] POST /resources - User: ${TEST_USERNAME} (${user_id}) - Token: ${auth_token:0:8}... - Resource: ${resource_id} - Status: SUCCESS EOF tail -1 /tmp/secure_session/api_audit.log",'
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

# Test Sequence 3: Token Validation and Security Checks
# Validate token integrity and perform security checks
TEST_SEQUENCE_ID=3
TEST_SEQUENCE_NAME='Token Validation and Security Checks'
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
# security: Authentication token is in use
# security: Multiple API requests have been made

# Step 1: Check token expiry status
LOG_FILE="TC_COMPLEX_SECURITY_AUTH_API_001_sequence-3_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="current_time=\$(date +%s) && if [[ \$current_time -lt \${token_expiry} ]]; then echo 'TOKEN_STATUS=VALID' echo 'TIME_REMAINING='\$((\${token_expiry} - \$current_time))'s' else echo 'TOKEN_STATUS=EXPIRED' echo 'TIME_REMAINING=0s' fi"
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
STEP_VAR_token_status=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(TOKEN_STATUS=([A-Z]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " token_status "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES token_status"
fi
STEP_VAR_time_remaining=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(TIME_REMAINING=([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " time_remaining "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES time_remaining"
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
COND_EXPR="[[ \"\$token_status\" == \"VALID\" ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        COND_EXPR=$(echo "$COND_EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$COND_EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
    echo 'Token is still valid'
else
    echo 'Token has expired'
fi
echo "Token status: ${token_status}, Remaining: ${time_remaining}s"

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 1: Check token expiry status"
else
    echo "[FAIL] Step 1: Check token expiry status"
    echo "  Command: current_time=$(date +%s) && if [[ $current_time -lt ${token_expiry} ]]; then echo 'TOKEN_STATUS=VALID' echo 'TIME_REMAINING='$((${token_expiry} - $current_time))'s' else echo 'TOKEN_STATUS=EXPIRED' echo 'TIME_REMAINING=0s' fi"
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
    echo '    "command": "current_time=$(date +%s) && if [[ $current_time -lt ${token_expiry} ]]; then echo \"TOKEN_STATUS=VALID\" echo \"TIME_REMAINING=\"$((${token_expiry} - $current_time))\"s\" else echo \"TOKEN_STATUS=EXPIRED\" echo \"TIME_REMAINING=0s\" fi",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 2: Validate token format and length
LOG_FILE="TC_COMPLEX_SECURITY_AUTH_API_001_sequence-3_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo 'Token: '\${auth_token} && echo 'Length: '\${#auth_token} && if [[ \${#auth_token} -ge 8 ]]; then echo 'FORMAT_CHECK=PASS' else echo 'FORMAT_CHECK=FAIL' fi"
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
STEP_VAR_format_check=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(FORMAT_CHECK=([A-Z]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " format_check "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES format_check"
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
EXPR="grep -q 'FORMAT_CHECK=PASS' <<< \"\$COMMAND_OUTPUT\""
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
GENERAL_VERIFY_PASS_verify_token_format_pass=false
EXPR="[[ \"\$format_check\" == \"PASS\" ]]"
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
    GENERAL_VERIFY_PASS_verify_token_format_pass=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_token_format_pass" = true ]; then
    echo "[PASS] Step 2: Validate token format and length"
else
    echo "[FAIL] Step 2: Validate token format and length"
    echo "  Command: echo 'Token: '${auth_token} && echo 'Length: '${#auth_token} && if [[ ${#auth_token} -ge 8 ]]; then echo 'FORMAT_CHECK=PASS' else echo 'FORMAT_CHECK=FAIL' fi"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_token_format_pass: $GENERAL_VERIFY_PASS_verify_token_format_pass"
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
    echo '    "command": "echo \"Token: \"${auth_token} && echo \"Length: \"${#auth_token} && if [[ ${#auth_token} -ge 8 ]]; then echo \"FORMAT_CHECK=PASS\" else echo \"FORMAT_CHECK=FAIL\" fi",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 3: Count total API requests made with this token
LOG_FILE="TC_COMPLEX_SECURITY_AUTH_API_001_sequence-3_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="grep -c '\${auth_token:0:8}' /tmp/secure_session/api_audit.log 2>/dev/null || echo 0"
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
STEP_VAR_request_count=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " request_count "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES request_count"
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
GENERAL_VERIFY_PASS_verify_request_count_numeric=false
EXPR="[[ \$request_count =~ ^[0-9]+\$ ]]"
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
    GENERAL_VERIFY_PASS_verify_request_count_numeric=true
fi
GENERAL_VERIFY_PASS_verify_requests_made=false
EXPR="[[ \$request_count -ge 1 ]]"
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
    GENERAL_VERIFY_PASS_verify_requests_made=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_request_count_numeric" = true ] && [ "$GENERAL_VERIFY_PASS_verify_requests_made" = true ]; then
    echo "[PASS] Step 3: Count total API requests made with this token"
else
    echo "[FAIL] Step 3: Count total API requests made with this token"
    echo "  Command: grep -c '${auth_token:0:8}' /tmp/secure_session/api_audit.log 2>/dev/null || echo 0"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_request_count_numeric: $GENERAL_VERIFY_PASS_verify_request_count_numeric"
    echo "  GENERAL_VERIFY_PASS_verify_requests_made: $GENERAL_VERIFY_PASS_verify_requests_made"
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
    echo '    "command": "grep -c \"${auth_token:0:8}\" /tmp/secure_session/api_audit.log 2>/dev/null || echo 0",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 4: Verify session file security permissions
LOG_FILE="TC_COMPLEX_SECURITY_AUTH_API_001_sequence-3_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="ls -l /tmp/secure_session/auth_\${auth_token:0:8}.sec | awk '{print \$1}'"
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
STEP_VAR_file_permissions=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([rwx-]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " file_permissions "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES file_permissions"
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
COND_EXPR="[[ \"\$file_permissions\" =~ ^-rw------- ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        COND_EXPR=$(echo "$COND_EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$COND_EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
    echo 'Session file has correct restrictive permissions (600)'
else
    echo 'WARNING: Session file permissions may be too permissive'
fi
echo "Permissions: ${file_permissions}"

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 4: Verify session file security permissions"
else
    echo "[FAIL] Step 4: Verify session file security permissions"
    echo "  Command: ls -l /tmp/secure_session/auth_${auth_token:0:8}.sec | awk '{print $1}'"
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
    echo '    "command": "ls -l /tmp/secure_session/auth_${auth_token:0:8}.sec | awk \"{print $1}\"",'
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

# Test Sequence 4: Session Termination and Cleanup
# Properly terminate authenticated session and cleanup sensitive data
TEST_SEQUENCE_ID=4
TEST_SEQUENCE_NAME='Session Termination and Cleanup'
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
# security: Authenticated session is active
# security: Sensitive data files exist

# Step 1: Simulate logout API request
LOG_FILE="TC_COMPLEX_SECURITY_AUTH_API_001_sequence-4_step-1.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo 'POST \${API_BASE_URL}/auth/logout' && echo 'Headers: '\${auth_header} && date +%s"
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
STEP_VAR_logout_timestamp=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)$\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " logout_timestamp "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES logout_timestamp"
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
EXPR="grep -q 'logout' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 1: Simulate logout API request"
else
    echo "[FAIL] Step 1: Simulate logout API request"
    echo "  Command: echo 'POST ${API_BASE_URL}/auth/logout' && echo 'Headers: '${auth_header} && date +%s"
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
    echo '    "command": "echo \"POST ${API_BASE_URL}/auth/logout\" && echo \"Headers: \"${auth_header} && date +%s",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 2: Calculate session duration
LOG_FILE="TC_COMPLEX_SECURITY_AUTH_API_001_sequence-4_step-2.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="echo \$((\${logout_timestamp} - \${auth_timestamp}))"
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
STEP_VAR_session_duration=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*\(([0-9]+)\).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " session_duration "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES session_duration"
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
COND_EXPR="[[ \$session_duration -ge 0 ]]"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\]/\\&/g')
        COND_EXPR=$(echo "$COND_EXPR" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi
if eval "$COND_EXPR"; then
    VERIFICATION_OUTPUT_PASS=true
    echo 'Session duration calculated successfully'
else
    echo 'Session duration calculation error'
fi
echo "Session duration: ${session_duration}s"

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ]; then
    echo "[PASS] Step 2: Calculate session duration"
else
    echo "[FAIL] Step 2: Calculate session duration"
    echo "  Command: echo $((${logout_timestamp} - ${auth_timestamp}))"
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
    echo '    "command": "echo $((${logout_timestamp} - ${auth_timestamp}))",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 3: Generate security audit report
LOG_FILE="TC_COMPLEX_SECURITY_AUTH_API_001_sequence-4_step-3.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="cat > /tmp/secure_session/security_report.txt << EOF Security Test Audit Report ==========================
Authentication Details: - User: \${TEST_USERNAME} - User ID: \${user_id} - Role: \${TEST_USER_ROLE} - Access Level: \${access_level}
Session Information: - Token: \${auth_token:0:8}...[REDACTED] - Authenticated: \${auth_timestamp} - Logout: \${logout_timestamp} - Duration: \${session_duration}s - Expiry: \${token_expiry} - Status at logout: \${token_status}
API Activity: - Total Requests: \${request_count} - Resources Created: \${resource_id}
Security Checks: - Token Format: \${format_check} - File Permissions: \${file_permissions} - Token Status: \${token_status}
Conclusion: - Authentication: SUCCESS - API Access: SUCCESS - Security Validation: SUCCESS - Session Cleanup: PENDING EOF cat /tmp/secure_session/security_report.txt"
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
EXPR="grep -q 'Security Test Audit Report' <<< \"\$COMMAND_OUTPUT\" && grep -q 'Session Cleanup: PENDING' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 3: Generate security audit report"
else
    echo "[FAIL] Step 3: Generate security audit report"
    echo "  Command: cat > /tmp/secure_session/security_report.txt << EOF Security Test Audit Report ==========================
Authentication Details: - User: ${TEST_USERNAME} - User ID: ${user_id} - Role: ${TEST_USER_ROLE} - Access Level: ${access_level}
Session Information: - Token: ${auth_token:0:8}...[REDACTED] - Authenticated: ${auth_timestamp} - Logout: ${logout_timestamp} - Duration: ${session_duration}s - Expiry: ${token_expiry} - Status at logout: ${token_status}
API Activity: - Total Requests: ${request_count} - Resources Created: ${resource_id}
Security Checks: - Token Format: ${format_check} - File Permissions: ${file_permissions} - Token Status: ${token_status}
Conclusion: - Authentication: SUCCESS - API Access: SUCCESS - Security Validation: SUCCESS - Session Cleanup: PENDING EOF cat /tmp/secure_session/security_report.txt"
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
    echo '    "command": "cat > /tmp/secure_session/security_report.txt << EOF Security Test Audit Report ==========================\nAuthentication Details: - User: ${TEST_USERNAME} - User ID: ${user_id} - Role: ${TEST_USER_ROLE} - Access Level: ${access_level}\nSession Information: - Token: ${auth_token:0:8}...[REDACTED] - Authenticated: ${auth_timestamp} - Logout: ${logout_timestamp} - Duration: ${session_duration}s - Expiry: ${token_expiry} - Status at logout: ${token_status}\nAPI Activity: - Total Requests: ${request_count} - Resources Created: ${resource_id}\nSecurity Checks: - Token Format: ${format_check} - File Permissions: ${file_permissions} - Token Status: ${token_status}\nConclusion: - Authentication: SUCCESS - API Access: SUCCESS - Security Validation: SUCCESS - Session Cleanup: PENDING EOF cat /tmp/secure_session/security_report.txt",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 4: Securely wipe sensitive session data
LOG_FILE="TC_COMPLEX_SECURITY_AUTH_API_001_sequence-4_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="if [ -f /tmp/secure_session/auth_\${auth_token:0:8}.sec ]; then cat /dev/null > /tmp/secure_session/auth_\${auth_token:0:8}.sec rm -f /tmp/secure_session/auth_\${auth_token:0:8}.sec echo 'Session file wiped and removed' else echo 'Session file not found' fi"
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
EXPR="grep -q 'wiped' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 4: Securely wipe sensitive session data"
else
    echo "[FAIL] Step 4: Securely wipe sensitive session data"
    echo "  Command: if [ -f /tmp/secure_session/auth_${auth_token:0:8}.sec ]; then cat /dev/null > /tmp/secure_session/auth_${auth_token:0:8}.sec rm -f /tmp/secure_session/auth_${auth_token:0:8}.sec echo 'Session file wiped and removed' else echo 'Session file not found' fi"
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
    echo '    "command": "if [ -f /tmp/secure_session/auth_${auth_token:0:8}.sec ]; then cat /dev/null > /tmp/secure_session/auth_${auth_token:0:8}.sec rm -f /tmp/secure_session/auth_${auth_token:0:8}.sec echo \"Session file wiped and removed\" else echo \"Session file not found\" fi",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 5: Archive audit logs and remove sensitive data directory
LOG_FILE="TC_COMPLEX_SECURITY_AUTH_API_001_sequence-4_step-5.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="if [ -d /tmp/secure_session ]; then tar -czf /tmp/security_audit_\${auth_timestamp}.tar.gz -C /tmp secure_session/api_audit.log secure_session/security_report.txt 2>&1 | head -1 || echo 'Audit archived' rm -rf /tmp/secure_session echo 'Security cleanup complete' else echo 'Session directory not found' fi"
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

# General verifications
GENERAL_VERIFY_PASS_verify_cleanup_complete=false
EXPR="[[ ! -d /tmp/secure_session ]]"
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
    GENERAL_VERIFY_PASS_verify_cleanup_complete=true
fi

if [ "$VERIFICATION_RESULT_PASS" = true ] && [ "$VERIFICATION_OUTPUT_PASS" = true ] && [ "$GENERAL_VERIFY_PASS_verify_cleanup_complete" = true ]; then
    echo "[PASS] Step 5: Archive audit logs and remove sensitive data directory"
else
    echo "[FAIL] Step 5: Archive audit logs and remove sensitive data directory"
    echo "  Command: if [ -d /tmp/secure_session ]; then tar -czf /tmp/security_audit_${auth_timestamp}.tar.gz -C /tmp secure_session/api_audit.log secure_session/security_report.txt 2>&1 | head -1 || echo 'Audit archived' rm -rf /tmp/secure_session echo 'Security cleanup complete' else echo 'Session directory not found' fi"
    echo "  Exit code: $EXIT_CODE"
    echo "  Output: $COMMAND_OUTPUT"
    echo "  Result verification: $VERIFICATION_RESULT_PASS"
    echo "  Output verification: $VERIFICATION_OUTPUT_PASS"
    echo "  GENERAL_VERIFY_PASS_verify_cleanup_complete: $GENERAL_VERIFY_PASS_verify_cleanup_complete"
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
    echo '    "command": "if [ -d /tmp/secure_session ]; then tar -czf /tmp/security_audit_${auth_timestamp}.tar.gz -C /tmp secure_session/api_audit.log secure_session/security_report.txt 2>&1 | head -1 || echo \"Audit archived\" rm -rf /tmp/secure_session echo \"Security cleanup complete\" else echo \"Session directory not found\" fi",'
    echo "    \"exit_code\": $EXIT_CODE,"
    echo "    \"output\": \"$OUTPUT_ESCAPED\","
    echo "    \"timestamp\": \"$TIMESTAMP\","
    echo "    \"result_verification_pass\": $VERIFICATION_RESULT_PASS,"
    echo "    \"output_verification_pass\": $VERIFICATION_OUTPUT_PASS"
    echo '  }'
} >> "$JSON_LOG"

# Step 6: Verify all sensitive data has been removed
LOG_FILE="TC_COMPLEX_SECURITY_AUTH_API_001_sequence-4_step-6.actual.log"
COMMAND_OUTPUT=""
set +e
ORIGINAL_COMMAND="if [ ! -d /tmp/secure_session ]; then echo 'CLEANUP_STATUS=SUCCESS' echo 'Sensitive session data removed' else echo 'CLEANUP_STATUS=INCOMPLETE' echo 'WARNING: Session directory still exists' fi && echo 'Audit archive: /tmp/security_audit_'\${auth_timestamp}'.tar.gz'"
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
EXPR="grep -q 'CLEANUP_STATUS=SUCCESS' <<< \"\$COMMAND_OUTPUT\" && grep -q 'Sensitive session data removed' <<< \"\$COMMAND_OUTPUT\""
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
    echo "[PASS] Step 6: Verify all sensitive data has been removed"
else
    echo "[FAIL] Step 6: Verify all sensitive data has been removed"
    echo "  Command: if [ ! -d /tmp/secure_session ]; then echo 'CLEANUP_STATUS=SUCCESS' echo 'Sensitive session data removed' else echo 'CLEANUP_STATUS=INCOMPLETE' echo 'WARNING: Session directory still exists' fi && echo 'Audit archive: /tmp/security_audit_'${auth_timestamp}'.tar.gz'"
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
    echo '    "command": "if [ ! -d /tmp/secure_session ]; then echo \"CLEANUP_STATUS=SUCCESS\" echo \"Sensitive session data removed\" else echo \"CLEANUP_STATUS=INCOMPLETE\" echo \"WARNING: Session directory still exists\" fi && echo \"Audit archive: /tmp/security_audit_\"${auth_timestamp}\".tar.gz\"",'
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
exit 0
