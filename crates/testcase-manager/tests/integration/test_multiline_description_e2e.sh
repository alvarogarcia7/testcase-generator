#!/bin/bash
#
# End-to-end integration test for TCMS-33: Multi-line test case descriptions
#
# This test validates:
# 1. Test case YAML with multi-line descriptions validates correctly
# 2. Shell script generation properly comments all description lines
# 3. First line includes the "Description:" label
# 4. Subsequent lines are properly commented with "# "
#
# Usage: ./tests/integration/test_multiline_description_e2e.sh
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"

# Source shared libraries
source "$PROJECT_ROOT/scripts/lib/find-binary.sh" || exit 1
source "$PROJECT_ROOT/scripts/lib/logger.sh" || exit 1

# Find binaries
cd "$PROJECT_ROOT"
TEST_EXECUTOR_BIN=$(find_binary "test-executor")
if [[ -z "$TEST_EXECUTOR_BIN" ]]; then
    fail "test-executor binary not found"
    exit 1
fi

# Create temporary directory
TEMP_DIR=$(mktemp -d)
setup_cleanup "$TEMP_DIR"

echo "======================================"
echo "TCMS-33: Multi-line Description E2E Test"
echo "======================================"
echo ""

# Create test case YAML with multi-line description
section "Creating Test Case with Multi-line Description"

YAML_FILE="$TEMP_DIR/test_multiline.yaml"
cat > "$YAML_FILE" << 'YAML_EOF'
type: test_case
schema: tcms/test-case.schema.v1.json
requirement: TCMS33
item: 1
tc: 1
id: TC_MULTILINE_DESC
title: Test Case with Multi-line Description
description: |
  This is the first line of the test case description.
  This is the second line of the test case description.
  This is the third line providing additional context.
general_initial_conditions:
  system:
    - Bash shell is available
initial_conditions:
  environment:
    - System is ready for testing
test_sequences:
  - id: 1
    name: Basic Sequence
    description: Simple test sequence
    initial_conditions:
      system:
        - Ready
    steps:
      - step: 1
        description: Simple echo step
        command: echo 'Testing multi-line description feature'
        expected:
          success: true
          result: "0"
          output: Testing
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "[[ \"$COMMAND_OUTPUT\" == *Testing* ]]"
YAML_EOF

pass "Created test YAML file with multi-line description"

# Generate shell script
section "Generating Shell Script from YAML"

SCRIPT_FILE="$TEMP_DIR/test_multiline.sh"
if "$TEST_EXECUTOR_BIN" generate "$YAML_FILE" > "$SCRIPT_FILE" 2>&1; then
    pass "Shell script generated successfully"
else
    fail "Failed to generate shell script"
    exit 1
fi

# Verify script syntax
section "Verifying Script Syntax"

if bash -n "$SCRIPT_FILE" 2>&1; then
    pass "Generated script has valid bash syntax"
else
    fail "Generated script has invalid bash syntax"
    exit 1
fi

# Verify multi-line description comments
section "Verifying Multi-line Description Comments"

EXPECTED_LINES=(
    "# Description: This is the first line of the test case description."
    "# This is the second line of the test case description."
    "# This is the third line providing additional context."
)

ALL_FOUND=true
for expected_line in "${EXPECTED_LINES[@]}"; do
    if grep -Fxq "$expected_line" "$SCRIPT_FILE"; then
        pass "Found: ${expected_line:0:65}..."
    else
        fail "Missing: $expected_line"
        ALL_FOUND=false
    fi
done

if [[ "$ALL_FOUND" == "true" ]]; then
    pass "All ${#EXPECTED_LINES[@]} description lines are properly commented"
else
    fail "Not all description lines were properly commented"
    exit 1
fi

# Test single-line description for backward compatibility
section "Testing Backward Compatibility with Single-line Descriptions"

YAML_SINGLE="$TEMP_DIR/test_single.yaml"
cat > "$YAML_SINGLE" << 'YAML_EOF'
type: test_case
schema: tcms/test-case.schema.v1.json
requirement: TCMS33
item: 2
tc: 2
id: TC_SINGLELINE_DESC
title: Test Case with Single-line Description
description: This is a simple single-line description
general_initial_conditions:
  system:
    - Bash shell is available
initial_conditions:
  environment:
    - System is ready
test_sequences:
  - id: 1
    name: Basic Sequence
    description: Simple test
    initial_conditions:
      system:
        - Ready
    steps:
      - step: 1
        description: Echo test
        command: echo 'hello'
        expected:
          success: true
          result: "0"
          output: hello
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "[[ \"$COMMAND_OUTPUT\" == *hello* ]]"
YAML_EOF

SCRIPT_SINGLE="$TEMP_DIR/test_single.sh"
if "$TEST_EXECUTOR_BIN" generate "$YAML_SINGLE" > "$SCRIPT_SINGLE" 2>&1; then
    if grep -Fxq "# Description: This is a simple single-line description" "$SCRIPT_SINGLE"; then
        pass "Single-line description format is correct"
    else
        fail "Single-line description format is incorrect"
        exit 1
    fi
else
    fail "Failed to generate single-line test script"
    exit 1
fi

# Final verification
section "Final Verification"

if [[ -f "$SCRIPT_FILE" ]] && [[ -f "$SCRIPT_SINGLE" ]]; then
    pass "Both multi-line and single-line scripts generated successfully"
    echo ""
    pass "TCMS-33: Multi-line Test Case Description Implementation VERIFIED"
    echo ""
    exit 0
else
    fail "One or more test scripts are missing"
    exit 1
fi
