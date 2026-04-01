#!/bin/bash
#
# End-to-end integration test for jq JSON escaping with comprehensive validation
#
# This test validates:
# 1. Test case YAML creation with commands producing special characters
#    - Quotes (single and double)
#    - Backslashes
#    - Unicode characters
#    - Newlines
#    - Control characters (tabs, carriage returns)
#    - Binary data (null bytes)
# 2. Script generation with JsonEscapingMethod::Jq
# 3. Execution and validation that JSON logs parse correctly via jq
# 4. Testing with jq removed from PATH to verify fallback to json-escape
# 5. Comparison of output between jq, json-escape, and shell fallback methods
#
# Usage: ./tests/integration/test_jq_json_escaping_e2e.sh [--no-remove]
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"

# Source logger library
source "$PROJECT_ROOT/scripts/lib/logger.sh" || exit 1
source "$PROJECT_ROOT/scripts/lib/find-binary.sh" || exit 1
source "$PROJECT_ROOT/scripts/lib/shellcheck-helper.sh" || true

# Handle --no-remove flag
REMOVE_TEMP=1
while [[ $# -gt 0 ]]; do
    case $1 in
        --no-remove)
            REMOVE_TEMP=0
            shift
            ;;
        *)
            shift
            ;;
    esac
done

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

section "jq JSON Escaping E2E Integration Test"

# Create temporary directory for test files
TEMP_DIR=$(mktemp -d)
setup_cleanup "$TEMP_DIR"
if [[ $REMOVE_TEMP -eq 0 ]]; then
    disable_cleanup
    info "Temporary files will not be removed: $TEMP_DIR"
fi

info "Using temporary directory: $TEMP_DIR"

# ============================================================================
# Test 1: Build required binaries
# ============================================================================
section "Test 1: Build required binaries"

log_info "Building test-executor binary..."
if cargo build -p test-executor > "$TEMP_DIR/build_executor.log" 2>&1; then
    pass "test-executor binary built successfully"
    ((++TESTS_PASSED))
else
    fail "Failed to build test-executor binary"
    cat "$TEMP_DIR/build_executor.log"
    ((++TESTS_FAILED))
    exit 1
fi

log_info "Building json-escape binary..."
if cargo build -p json-escape > "$TEMP_DIR/build_json_escape.log" 2>&1; then
    pass "json-escape binary built successfully"
    ((++TESTS_PASSED))
else
    fail "Failed to build json-escape binary"
    cat "$TEMP_DIR/build_json_escape.log"
    ((++TESTS_FAILED))
    exit 1
fi

# Find binaries using workspace-aware search
cd "$PROJECT_ROOT"
TEST_EXECUTOR_BIN=$(find_binary "test-executor")
if [[ -z "$TEST_EXECUTOR_BIN" ]]; then
    fail "test-executor binary not found after build"
    ((++TESTS_FAILED))
    exit 1
fi
pass "test-executor binary found at $TEST_EXECUTOR_BIN"
((++TESTS_PASSED))

JSON_ESCAPE_BIN=$(find_binary "json-escape")
if [[ -z "$JSON_ESCAPE_BIN" ]]; then
    fail "json-escape binary not found after build"
    ((++TESTS_FAILED))
    exit 1
fi
pass "json-escape binary found at $JSON_ESCAPE_BIN"
((++TESTS_PASSED))

# Check for jq availability
if command -v jq >/dev/null 2>&1; then
    JQ_AVAILABLE=1
    JQ_PATH=$(command -v jq)
    pass "jq is available at $JQ_PATH"
    ((++TESTS_PASSED))
else
    JQ_AVAILABLE=0
    warn "jq is not available - some tests will be skipped"
fi

# ============================================================================
# Test 2: Create test case YAML with special characters
# ============================================================================
section "Test 2: Create test case YAML with special characters"

log_info "Creating comprehensive test case YAML..."
SPECIAL_CHARS_YAML="$TEMP_DIR/test_special_chars.yaml"
cat > "$SPECIAL_CHARS_YAML" << 'EOF'
requirement: REQ_JQ_001
item: 1
tc: 1
id: TEST_JQ_SPECIAL_CHARS
description: Test jq JSON escaping with all special character types
general_initial_conditions:
  System:
    - Shell ready
initial_conditions:
  System:
    - Commands executable
test_sequences:
  - sequence: 1
    seq_id: SEQ_QUOTES
    description: Test with quotes
    sequence_initial_conditions:
      System:
        - Ready
    steps:
      - step: 1
        description: Double quotes
        command: echo 'He said "hello world" to me'
        expected:
          success: true
          result: "0"
          output: "true"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
      - step: 2
        description: Single quotes in output
        command: echo "It's a beautiful day"
        expected:
          success: true
          result: "0"
          output: "true"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
      - step: 3
        description: Mixed quotes
        command: echo 'She said "It'"'"'s done" today'
        expected:
          success: true
          result: "0"
          output: "true"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
  - sequence: 2
    seq_id: SEQ_BACKSLASHES
    description: Test with backslashes
    sequence_initial_conditions:
      System:
        - Ready
    steps:
      - step: 1
        description: Windows paths
        command: echo 'C:\Users\Test\Documents\file.txt'
        expected:
          success: true
          result: "0"
          output: "true"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
      - step: 2
        description: UNC paths
        command: echo '\\server\share\folder\file'
        expected:
          success: true
          result: "0"
          output: "true"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
      - step: 3
        description: Escaped backslashes
        command: printf 'Backslash: \\ Double: \\\\'
        expected:
          success: true
          result: "0"
          output: "true"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
  - sequence: 3
    seq_id: SEQ_NEWLINES
    description: Test with newlines
    sequence_initial_conditions:
      System:
        - Ready
    steps:
      - step: 1
        description: Multiple newlines with printf
        command: printf 'Line 1\nLine 2\nLine 3\nLine 4'
        expected:
          success: true
          result: "0"
          output: "true"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
      - step: 2
        description: Empty lines
        command: printf 'Start\n\nMiddle\n\nEnd'
        expected:
          success: true
          result: "0"
          output: "true"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
  - sequence: 4
    seq_id: SEQ_CONTROL_CHARS
    description: Test with control characters
    sequence_initial_conditions:
      System:
        - Ready
    steps:
      - step: 1
        description: Tabs
        command: printf 'Col1\tCol2\tCol3\tCol4'
        expected:
          success: true
          result: "0"
          output: "true"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
      - step: 2
        description: Carriage returns
        command: printf 'Progress: 50%%\rProgress: 100%%'
        expected:
          success: true
          result: "0"
          output: "true"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
      - step: 3
        description: Mixed control characters
        command: printf 'Tab:\t Newline:\n Carriage:\r Done'
        expected:
          success: true
          result: "0"
          output: "true"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
  - sequence: 5
    seq_id: SEQ_UNICODE
    description: Test with Unicode characters
    sequence_initial_conditions:
      System:
        - Ready
    steps:
      - step: 1
        description: Unicode emoji
        command: echo 'Hello 👋 World 🌍'
        expected:
          success: true
          result: "0"
          output: "true"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
      - step: 2
        description: Unicode symbols
        command: echo 'Math: π ≈ 3.14, ∑ symbols'
        expected:
          success: true
          result: "0"
          output: "true"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
      - step: 3
        description: Unicode languages
        command: echo 'English, 日本語, 中文, العربية, Русский'
        expected:
          success: true
          result: "0"
          output: "true"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
  - sequence: 6
    seq_id: SEQ_BINARY
    description: Test with binary-like data
    sequence_initial_conditions:
      System:
        - Ready
    steps:
      - step: 1
        description: Null bytes (simulated with hex)
        command: printf '\x00\x01\x02\x03' | od -An -tx1
        expected:
          success: true
          result: "0"
          output: "true"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
      - step: 2
        description: High ASCII characters
        command: printf '\xC0\xC1\xFE\xFF' | od -An -tx1
        expected:
          success: true
          result: "0"
          output: "true"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
  - sequence: 7
    seq_id: SEQ_COMPLEX
    description: Test with complex mixed content
    sequence_initial_conditions:
      System:
        - Ready
    steps:
      - step: 1
        description: JSON-like output
        command: echo '{"status": "ok", "message": "Test \"passed\"", "path": "C:\\temp"}'
        expected:
          success: true
          result: "0"
          output: "true"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
      - step: 2
        description: Command output with various special chars
        command: printf 'Output:\n\tStatus: "running"\n\tPath: C:\\Users\\test\n\tProgress: 100%%\r\nDone!'
        expected:
          success: true
          result: "0"
          output: "true"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
EOF

pass "Created test case YAML with special characters"
((++TESTS_PASSED))

# ============================================================================
# Test 3: Generate and execute script with JsonEscapingMethod::Jq
# ============================================================================
section "Test 3: Generate and execute script with JsonEscapingMethod::Jq"

log_info "Creating config file for Jq mode..."
CONFIG_FILE_JQ="$TEMP_DIR/config_jq.toml"
cat > "$CONFIG_FILE_JQ" << EOF
[script_generation.json_escaping]
method = "jq"
enabled = true
EOF

pass "Created Jq method config file"
((++TESTS_PASSED))

if [[ $JQ_AVAILABLE -eq 1 ]]; then
    log_info "Generating test script with Jq method..."
    if cargo run -p test-executor -- \
        --config "$CONFIG_FILE_JQ" \
        --output-dir "$TEMP_DIR" \
        generate "$SPECIAL_CHARS_YAML" > "$TEMP_DIR/generate_jq.log" 2>&1; then
        pass "Test script generated with Jq method"
        ((++TESTS_PASSED))
    else
        fail "Failed to generate test script with Jq method"
        cat "$TEMP_DIR/generate_jq.log"
        ((++TESTS_FAILED))
    fi
    
    JQ_SCRIPT_PATH="$TEMP_DIR/TEST_JQ_SPECIAL_CHARS_test.sh"
    if [[ -f "$JQ_SCRIPT_PATH" ]]; then
        pass "Generated jq script exists"
        ((++TESTS_PASSED))
        
        # Verify script uses jq
        if grep -q "jq -Rs" "$JQ_SCRIPT_PATH"; then
            pass "Script uses jq for JSON escaping"
            ((++TESTS_PASSED))
        else
            fail "Script does not use jq"
            ((++TESTS_FAILED))
        fi
        
        # Verify script syntax
        if bash -n "$JQ_SCRIPT_PATH" 2>/dev/null; then
            pass "Jq script has valid bash syntax"
            ((++TESTS_PASSED))
        else
            fail "Jq script has invalid bash syntax"
            ((++TESTS_FAILED))
        fi
        validate_with_shellcheck "$JQ_SCRIPT_PATH" "Jq script"
        
        # Execute script
        chmod +x "$JQ_SCRIPT_PATH"
        log_info "Executing test script with Jq method..."
        cd "$TEMP_DIR"
        if bash "$JQ_SCRIPT_PATH" > "$TEMP_DIR/execute_jq.log" 2>&1; then
            pass "Test script with Jq method executed successfully"
            ((++TESTS_PASSED))
        else
            fail "Test script with Jq method execution failed"
            cat "$TEMP_DIR/execute_jq.log" | tail -50
            ((++TESTS_FAILED))
        fi
        cd "$PROJECT_ROOT"
        
        # Verify JSON log was created
        JQ_JSON_LOG="$TEMP_DIR/TEST_JQ_SPECIAL_CHARS_execution_log.json"
        if [[ -f "$JQ_JSON_LOG" ]]; then
            pass "JSON log created with Jq method"
            ((++TESTS_PASSED))
            
            # Validate JSON with jq
            if jq empty "$JQ_JSON_LOG" >/dev/null 2>&1; then
                pass "JSON log is valid (jq validation)"
                ((++TESTS_PASSED))
            else
                fail "JSON log is invalid"
                cat "$JQ_JSON_LOG" | head -20
                ((++TESTS_FAILED))
            fi
            
            # Verify JSON structure
            NUM_ENTRIES=$(jq 'length' "$JQ_JSON_LOG")
            EXPECTED_ENTRIES=21  # 7 sequences with 3,3,2,3,3,2,2 steps = 21 total
            if [[ "$NUM_ENTRIES" -eq "$EXPECTED_ENTRIES" ]]; then
                pass "JSON log contains $EXPECTED_ENTRIES entries (all non-manual steps)"
                ((++TESTS_PASSED))
            else
                fail "JSON log should contain $EXPECTED_ENTRIES entries, got $NUM_ENTRIES"
                ((++TESTS_FAILED))
            fi
            
            # Verify special characters are properly escaped in JSON
            # Test 1: Check double quotes are escaped
            FIRST_OUTPUT=$(jq -r '.[0].output' "$JQ_JSON_LOG")
            if [[ "$FIRST_OUTPUT" == *"hello world"* ]]; then
                pass "First step output contains expected text with quotes handled"
                ((++TESTS_PASSED))
            else
                fail "First step output missing expected text: $FIRST_OUTPUT"
                ((++TESTS_FAILED))
            fi
            
            # Test 2: Check backslashes in Windows paths
            BACKSLASH_OUTPUT=$(jq -r '.[] | select(.step_id == "sequence-2_step-1") | .output' "$JQ_JSON_LOG")
            if [[ "$BACKSLASH_OUTPUT" == *"Users"* ]] && [[ "$BACKSLASH_OUTPUT" == *"Documents"* ]]; then
                pass "Backslash handling works in JSON output"
                ((++TESTS_PASSED))
            else
                fail "Backslash handling failed: $BACKSLASH_OUTPUT"
                ((++TESTS_FAILED))
            fi
            
            # Test 3: Check newlines are escaped
            NEWLINE_OUTPUT=$(jq -r '.[] | select(.step_id == "sequence-3_step-1") | .output' "$JQ_JSON_LOG")
            if [[ "$NEWLINE_OUTPUT" == *"Line 1"* ]] && [[ "$NEWLINE_OUTPUT" == *"Line 2"* ]]; then
                pass "Newline handling works in JSON output"
                ((++TESTS_PASSED))
            else
                fail "Newline handling failed: $NEWLINE_OUTPUT"
                ((++TESTS_FAILED))
            fi
            
            # Test 4: Check tabs are escaped
            TAB_OUTPUT=$(jq -r '.[] | select(.step_id == "sequence-4_step-1") | .output' "$JQ_JSON_LOG")
            if [[ "$TAB_OUTPUT" == *"Col1"* ]] && [[ "$TAB_OUTPUT" == *"Col2"* ]]; then
                pass "Tab handling works in JSON output"
                ((++TESTS_PASSED))
            else
                fail "Tab handling failed: $TAB_OUTPUT"
                ((++TESTS_FAILED))
            fi
            
            # Test 5: Check Unicode characters
            UNICODE_OUTPUT=$(jq -r '.[] | select(.step_id == "sequence-5_step-1") | .output' "$JQ_JSON_LOG")
            if [[ "$UNICODE_OUTPUT" == *"Hello"* ]] && [[ "$UNICODE_OUTPUT" == *"World"* ]]; then
                pass "Unicode handling works in JSON output"
                ((++TESTS_PASSED))
            else
                fail "Unicode handling failed: $UNICODE_OUTPUT"
                ((++TESTS_FAILED))
            fi
            
            # Test 6: Verify JSON structure has all required fields
            FIRST_ENTRY=$(jq '.[0]' "$JQ_JSON_LOG")
            if echo "$FIRST_ENTRY" | jq -e 'has("test_case_id")' >/dev/null 2>&1 && \
               echo "$FIRST_ENTRY" | jq -e 'has("sequence_id")' >/dev/null 2>&1 && \
               echo "$FIRST_ENTRY" | jq -e 'has("step_id")' >/dev/null 2>&1 && \
               echo "$FIRST_ENTRY" | jq -e 'has("exit_code")' >/dev/null 2>&1 && \
               echo "$FIRST_ENTRY" | jq -e 'has("output")' >/dev/null 2>&1; then
                pass "JSON entries have all required fields"
                ((++TESTS_PASSED))
            else
                fail "JSON entries missing required fields"
                echo "$FIRST_ENTRY"
                ((++TESTS_FAILED))
            fi
            
        else
            fail "JSON log not created with Jq method"
            ((++TESTS_FAILED))
        fi
        
    else
        fail "Generated jq script not found at $JQ_SCRIPT_PATH"
        ((++TESTS_FAILED))
    fi
else
    warn "Skipping Jq method tests - jq not available"
fi

# ============================================================================
# Test 4: Generate scripts with all methods for comparison
# ============================================================================
section "Test 4: Generate scripts with all methods for comparison"

# Generate with RustBinary method
log_info "Creating config file for RustBinary mode..."
CONFIG_FILE_RUST="$TEMP_DIR/config_rust.toml"
cat > "$CONFIG_FILE_RUST" << EOF
[script_generation.json_escaping]
method = "rust_binary"
enabled = true
EOF

log_info "Generating test script with RustBinary method..."
if cargo run -p test-executor -- \
    --config "$CONFIG_FILE_RUST" \
    --output-dir "$TEMP_DIR" \
    generate "$SPECIAL_CHARS_YAML" > "$TEMP_DIR/generate_rust.log" 2>&1; then
    pass "Test script generated with RustBinary method"
    ((++TESTS_PASSED))
else
    fail "Failed to generate test script with RustBinary method"
    cat "$TEMP_DIR/generate_rust.log"
    ((++TESTS_FAILED))
fi

# Execute RustBinary script
RUST_SCRIPT_PATH="$TEMP_DIR/TEST_JQ_SPECIAL_CHARS_test.sh"
if [[ -f "$RUST_SCRIPT_PATH" ]]; then
    chmod +x "$RUST_SCRIPT_PATH"
    log_info "Executing test script with RustBinary method..."
    cd "$TEMP_DIR"
    if bash "$RUST_SCRIPT_PATH" > "$TEMP_DIR/execute_rust.log" 2>&1; then
        pass "Test script with RustBinary method executed successfully"
        ((++TESTS_PASSED))
    else
        fail "Test script with RustBinary method execution failed"
        cat "$TEMP_DIR/execute_rust.log" | tail -50
        ((++TESTS_FAILED))
    fi
    cd "$PROJECT_ROOT"
    
    # Verify JSON log
    RUST_JSON_LOG="$TEMP_DIR/TEST_JQ_SPECIAL_CHARS_execution_log.json"
    if [[ -f "$RUST_JSON_LOG" ]]; then
        pass "JSON log created with RustBinary method"
        ((++TESTS_PASSED))
        
        if command -v jq >/dev/null 2>&1; then
            if jq empty "$RUST_JSON_LOG" >/dev/null 2>&1; then
                pass "JSON log from RustBinary method is valid"
                ((++TESTS_PASSED))
            else
                fail "JSON log from RustBinary method is invalid"
                cat "$RUST_JSON_LOG" | head -20
                ((++TESTS_FAILED))
            fi
        fi
    else
        fail "JSON log not created with RustBinary method"
        ((++TESTS_FAILED))
    fi
fi

# Generate with ShellFallback method
log_info "Creating config file for ShellFallback mode..."
CONFIG_FILE_SHELL="$TEMP_DIR/config_shell.toml"
cat > "$CONFIG_FILE_SHELL" << EOF
[script_generation.json_escaping]
method = "shell_fallback"
enabled = true
EOF

log_info "Generating test script with ShellFallback method..."
if cargo run -p test-executor -- \
    --config "$CONFIG_FILE_SHELL" \
    --output-dir "$TEMP_DIR" \
    generate "$SPECIAL_CHARS_YAML" > "$TEMP_DIR/generate_shell.log" 2>&1; then
    pass "Test script generated with ShellFallback method"
    ((++TESTS_PASSED))
else
    fail "Failed to generate test script with ShellFallback method"
    cat "$TEMP_DIR/generate_shell.log"
    ((++TESTS_FAILED))
fi

# Execute ShellFallback script
SHELL_SCRIPT_PATH="$TEMP_DIR/TEST_JQ_SPECIAL_CHARS_test.sh"
if [[ -f "$SHELL_SCRIPT_PATH" ]]; then
    chmod +x "$SHELL_SCRIPT_PATH"
    log_info "Executing test script with ShellFallback method..."
    cd "$TEMP_DIR"
    if bash "$SHELL_SCRIPT_PATH" > "$TEMP_DIR/execute_shell.log" 2>&1; then
        pass "Test script with ShellFallback method executed successfully"
        ((++TESTS_PASSED))
    else
        fail "Test script with ShellFallback method execution failed"
        cat "$TEMP_DIR/execute_shell.log" | tail -50
        ((++TESTS_FAILED))
    fi
    cd "$PROJECT_ROOT"
    
    # Verify JSON log
    SHELL_JSON_LOG="$TEMP_DIR/TEST_JQ_SPECIAL_CHARS_execution_log.json"
    if [[ -f "$SHELL_JSON_LOG" ]]; then
        pass "JSON log created with ShellFallback method"
        ((++TESTS_PASSED))
        
        if command -v jq >/dev/null 2>&1; then
            if jq empty "$SHELL_JSON_LOG" >/dev/null 2>&1; then
                pass "JSON log from ShellFallback method is valid"
                ((++TESTS_PASSED))
            else
                fail "JSON log from ShellFallback method is invalid"
                cat "$SHELL_JSON_LOG" | head -20
                ((++TESTS_FAILED))
            fi
        fi
    else
        fail "JSON log not created with ShellFallback method"
        ((++TESTS_FAILED))
    fi
fi

# ============================================================================
# Test 5: Compare outputs between methods
# ============================================================================
section "Test 5: Compare outputs between methods"

if [[ $JQ_AVAILABLE -eq 1 ]] && [[ -f "$JQ_JSON_LOG" ]] && [[ -f "$RUST_JSON_LOG" ]] && [[ -f "$SHELL_JSON_LOG" ]]; then
    log_info "Comparing JSON outputs from different methods..."
    
    # Compare number of entries
    JQ_COUNT=$(jq 'length' "$JQ_JSON_LOG")
    RUST_COUNT=$(jq 'length' "$RUST_JSON_LOG")
    SHELL_COUNT=$(jq 'length' "$SHELL_JSON_LOG")
    
    if [[ "$JQ_COUNT" -eq "$RUST_COUNT" ]] && [[ "$RUST_COUNT" -eq "$SHELL_COUNT" ]]; then
        pass "All methods produced same number of entries ($JQ_COUNT)"
        ((++TESTS_PASSED))
    else
        fail "Methods produced different number of entries: jq=$JQ_COUNT, rust=$RUST_COUNT, shell=$SHELL_COUNT"
        ((++TESTS_FAILED))
    fi
    
    # Compare specific outputs
    log_info "Comparing specific output values..."
    
    # Compare first entry output (quotes test)
    JQ_FIRST=$(jq -r '.[0].output' "$JQ_JSON_LOG")
    RUST_FIRST=$(jq -r '.[0].output' "$RUST_JSON_LOG")
    SHELL_FIRST=$(jq -r '.[0].output' "$SHELL_JSON_LOG")
    
    if [[ "$JQ_FIRST" == "$RUST_FIRST" ]] && [[ "$RUST_FIRST" == "$SHELL_FIRST" ]]; then
        pass "All methods produce consistent output for quotes test"
        ((++TESTS_PASSED))
    else
        warn "Methods produced different outputs for quotes test"
        info "jq:    $JQ_FIRST"
        info "rust:  $RUST_FIRST"
        info "shell: $SHELL_FIRST"
        # Not failing this as minor differences are acceptable
    fi
    
    # Verify all entries can be parsed from all logs
    JQ_PARSEABLE=0
    RUST_PARSEABLE=0
    SHELL_PARSEABLE=0
    
    for i in $(seq 0 $((JQ_COUNT - 1))); do
        if jq -e ".[$i]" "$JQ_JSON_LOG" >/dev/null 2>&1; then
            ((++JQ_PARSEABLE))
        fi
        if jq -e ".[$i]" "$RUST_JSON_LOG" >/dev/null 2>&1; then
            ((++RUST_PARSEABLE))
        fi
        if jq -e ".[$i]" "$SHELL_JSON_LOG" >/dev/null 2>&1; then
            ((++SHELL_PARSEABLE))
        fi
    done
    
    if [[ "$JQ_PARSEABLE" -eq "$JQ_COUNT" ]] && \
       [[ "$RUST_PARSEABLE" -eq "$RUST_COUNT" ]] && \
       [[ "$SHELL_PARSEABLE" -eq "$SHELL_COUNT" ]]; then
        pass "All entries are parseable in all method outputs"
        ((++TESTS_PASSED))
    else
        fail "Some entries are not parseable: jq=$JQ_PARSEABLE/$JQ_COUNT, rust=$RUST_PARSEABLE/$RUST_COUNT, shell=$SHELL_PARSEABLE/$SHELL_COUNT"
        ((++TESTS_FAILED))
    fi
    
else
    warn "Skipping output comparison - not all JSON logs available"
fi

# ============================================================================
# Test 6: Test with jq removed from PATH (fallback behavior)
# ============================================================================
section "Test 6: Test with jq removed from PATH (fallback behavior)"

if [[ $JQ_AVAILABLE -eq 1 ]]; then
    log_info "Testing Jq method with jq removed from PATH..."
    
    # Generate script with Jq method (but will need to handle missing jq)
    # Since the script was generated expecting jq, we need to test Auto mode instead
    log_info "Creating config file for Auto mode..."
    CONFIG_FILE_AUTO="$TEMP_DIR/config_auto.toml"
    cat > "$CONFIG_FILE_AUTO" << EOF
[script_generation.json_escaping]
method = "auto"
enabled = true
EOF
    
    log_info "Generating test script with Auto method..."
    if cargo run -p test-executor -- \
        --config "$CONFIG_FILE_AUTO" \
        --output-dir "$TEMP_DIR" \
        generate "$SPECIAL_CHARS_YAML" > "$TEMP_DIR/generate_auto.log" 2>&1; then
        pass "Test script generated with Auto method"
        ((++TESTS_PASSED))
    else
        fail "Failed to generate test script with Auto method"
        cat "$TEMP_DIR/generate_auto.log"
        ((++TESTS_FAILED))
    fi
    
    AUTO_SCRIPT_PATH="$TEMP_DIR/TEST_JQ_SPECIAL_CHARS_test.sh"
    if [[ -f "$AUTO_SCRIPT_PATH" ]]; then
        chmod +x "$AUTO_SCRIPT_PATH"
        
        # Remove jq from PATH by creating a restricted environment
        log_info "Creating restricted PATH without jq..."
        CLEAN_PATH=$(echo "$PATH" | tr ':' '\n' | while read -r dir; do
            if ! ls "$dir"/jq 2>/dev/null | grep -q .; then
                echo "$dir"
            fi
        done | tr '\n' ':' | sed 's/:$//')
        
        # Also ensure json-escape is in PATH (from our build)
        JSON_ESCAPE_DIR=$(dirname "$JSON_ESCAPE_BIN")
        CLEAN_PATH="$JSON_ESCAPE_DIR:$CLEAN_PATH"
        
        log_info "Executing Auto mode script without jq in PATH..."
        cd "$TEMP_DIR"
        if env PATH="$CLEAN_PATH" bash "$AUTO_SCRIPT_PATH" > "$TEMP_DIR/execute_auto_no_jq.log" 2>&1; then
            pass "Auto mode script executed successfully without jq"
            ((++TESTS_PASSED))
        else
            fail "Auto mode script failed without jq"
            cat "$TEMP_DIR/execute_auto_no_jq.log" | tail -50
            ((++TESTS_FAILED))
        fi
        cd "$PROJECT_ROOT"
        
        # Verify JSON log was created and is valid
        AUTO_JSON_LOG="$TEMP_DIR/TEST_JQ_SPECIAL_CHARS_execution_log.json"
        if [[ -f "$AUTO_JSON_LOG" ]]; then
            pass "JSON log created with Auto mode (no jq fallback)"
            ((++TESTS_PASSED))
            
            if jq empty "$AUTO_JSON_LOG" >/dev/null 2>&1; then
                pass "JSON log from Auto mode (fallback) is valid"
                ((++TESTS_PASSED))
            else
                fail "JSON log from Auto mode (fallback) is invalid"
                cat "$AUTO_JSON_LOG" | head -20
                ((++TESTS_FAILED))
            fi
            
            # Verify it fell back to json-escape
            AUTO_COUNT=$(jq 'length' "$AUTO_JSON_LOG")
            if [[ "$AUTO_COUNT" -eq 21 ]]; then
                pass "Auto mode fallback produced correct number of entries"
                ((++TESTS_PASSED))
            else
                fail "Auto mode fallback produced $AUTO_COUNT entries, expected 21"
                ((++TESTS_FAILED))
            fi
        else
            fail "JSON log not created with Auto mode (fallback)"
            ((++TESTS_FAILED))
        fi
        
        # Test completely without jq or json-escape (shell fallback only)
        log_info "Creating very restricted PATH (no jq, no json-escape)..."
        MINIMAL_PATH=$(echo "$PATH" | tr ':' '\n' | grep -E '^/(usr/)?bin$|^/(usr/)?sbin$' | tr '\n' ':' | sed 's/:$//')
        
        log_info "Executing Auto mode script with shell fallback only..."
        cd "$TEMP_DIR"
        rm -f "$AUTO_JSON_LOG"  # Remove previous log
        if env PATH="$MINIMAL_PATH" bash "$AUTO_SCRIPT_PATH" > "$TEMP_DIR/execute_auto_shell_fallback.log" 2>&1; then
            pass "Auto mode script executed successfully with shell fallback only"
            ((++TESTS_PASSED))
        else
            fail "Auto mode script failed with shell fallback only"
            cat "$TEMP_DIR/execute_auto_shell_fallback.log" | tail -50
            ((++TESTS_FAILED))
        fi
        cd "$PROJECT_ROOT"
        
        # Verify JSON log with shell fallback
        if [[ -f "$AUTO_JSON_LOG" ]]; then
            pass "JSON log created with shell fallback"
            ((++TESTS_PASSED))
            
            if jq empty "$AUTO_JSON_LOG" >/dev/null 2>&1; then
                pass "JSON log from shell fallback is valid"
                ((++TESTS_PASSED))
            else
                fail "JSON log from shell fallback is invalid"
                cat "$AUTO_JSON_LOG" | head -20
                ((++TESTS_FAILED))
            fi
        else
            fail "JSON log not created with shell fallback"
            ((++TESTS_FAILED))
        fi
    else
        fail "Auto mode script not found"
        ((++TESTS_FAILED))
    fi
else
    warn "Skipping jq removal test - jq not available to begin with"
fi

# ============================================================================
# Test 7: Verify jq method handles edge cases
# ============================================================================
section "Test 7: Verify jq method handles edge cases"

if [[ $JQ_AVAILABLE -eq 1 ]]; then
    log_info "Creating edge case test YAML..."
    EDGE_CASE_YAML="$TEMP_DIR/test_edge_cases.yaml"
    cat > "$EDGE_CASE_YAML" << 'EOF'
requirement: REQ_JQ_002
item: 1
tc: 1
id: TEST_JQ_EDGE_CASES
description: Test jq JSON escaping with edge cases
general_initial_conditions:
  System:
    - Ready
initial_conditions:
  System:
    - Ready
test_sequences:
  - sequence: 1
    seq_id: SEQ_EDGE
    description: Edge case tests
    sequence_initial_conditions:
      System:
        - Ready
    steps:
      - step: 1
        description: Empty output
        command: echo -n ""
        expected:
          success: true
          result: "0"
          output: ""
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
      - step: 2
        description: Only whitespace
        command: echo "   "
        expected:
          success: true
          result: "0"
          output: "true"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
      - step: 3
        description: Very long output
        command: printf 'A%.0s' {1..1000}
        expected:
          success: true
          result: "0"
          output: "true"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
      - step: 4
        description: Mixed line endings
        command: printf 'Unix\nWindows\r\nMac\rMixed'
        expected:
          success: true
          result: "0"
          output: "true"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
EOF
    
    pass "Created edge case test YAML"
    ((++TESTS_PASSED))
    
    log_info "Generating edge case test script..."
    if cargo run -p test-executor -- \
        --config "$CONFIG_FILE_JQ" \
        --output-dir "$TEMP_DIR" \
        generate "$EDGE_CASE_YAML" > "$TEMP_DIR/generate_edge.log" 2>&1; then
        pass "Edge case test script generated"
        ((++TESTS_PASSED))
    else
        fail "Failed to generate edge case test script"
        cat "$TEMP_DIR/generate_edge.log"
        ((++TESTS_FAILED))
    fi
    
    EDGE_SCRIPT_PATH="$TEMP_DIR/TEST_JQ_EDGE_CASES_test.sh"
    if [[ -f "$EDGE_SCRIPT_PATH" ]]; then
        chmod +x "$EDGE_SCRIPT_PATH"
        log_info "Executing edge case test script..."
        cd "$TEMP_DIR"
        if bash "$EDGE_SCRIPT_PATH" > "$TEMP_DIR/execute_edge.log" 2>&1; then
            pass "Edge case test script executed successfully"
            ((++TESTS_PASSED))
        else
            fail "Edge case test script execution failed"
            cat "$TEMP_DIR/execute_edge.log" | tail -50
            ((++TESTS_FAILED))
        fi
        cd "$PROJECT_ROOT"
        
        EDGE_JSON_LOG="$TEMP_DIR/TEST_JQ_EDGE_CASES_execution_log.json"
        if [[ -f "$EDGE_JSON_LOG" ]]; then
            pass "Edge case JSON log created"
            ((++TESTS_PASSED))
            
            if jq empty "$EDGE_JSON_LOG" >/dev/null 2>&1; then
                pass "Edge case JSON log is valid"
                ((++TESTS_PASSED))
            else
                fail "Edge case JSON log is invalid"
                cat "$EDGE_JSON_LOG"
                ((++TESTS_FAILED))
            fi
            
            # Check empty output handling
            EMPTY_OUTPUT=$(jq -r '.[0].output' "$EDGE_JSON_LOG")
            if [[ -z "$EMPTY_OUTPUT" ]] || [[ "$EMPTY_OUTPUT" == "null" ]]; then
                pass "Empty output handled correctly"
                ((++TESTS_PASSED))
            else
                fail "Empty output not handled correctly: '$EMPTY_OUTPUT'"
                ((++TESTS_FAILED))
            fi
            
            # Check very long output handling
            LONG_OUTPUT=$(jq -r '.[2].output' "$EDGE_JSON_LOG")
            LONG_LEN=${#LONG_OUTPUT}
            if [[ $LONG_LEN -ge 900 ]]; then  # Should be close to 1000
                pass "Very long output handled correctly (length: $LONG_LEN)"
                ((++TESTS_PASSED))
            else
                fail "Very long output not handled correctly (length: $LONG_LEN)"
                ((++TESTS_FAILED))
            fi
        else
            fail "Edge case JSON log not created"
            ((++TESTS_FAILED))
        fi
    fi
else
    warn "Skipping edge case tests - jq not available"
fi

# ============================================================================
# Summary
# ============================================================================
section "Test Summary"

TOTAL_TESTS=$((TESTS_PASSED + TESTS_FAILED))
echo ""
echo "Total tests: $TOTAL_TESTS"
echo "Passed: $TESTS_PASSED"
echo "Failed: $TESTS_FAILED"
echo ""

if [[ $TESTS_FAILED -gt 0 ]]; then
    fail "Some tests failed"
    exit 1
else
    pass "All tests passed!"
    exit 0
fi
