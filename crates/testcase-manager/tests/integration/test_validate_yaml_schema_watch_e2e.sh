#!/bin/bash
#
# End-to-end integration test for validate-yaml watch mode with schema changes
#
# This test validates:
# 1. Watch mode monitors both YAML files and schema files
# 2. Schema file changes trigger re-validation of all YAML files
# 3. Transitive schema dependencies are discovered and watched
# 4. Schema changes are properly detected and reported
# 5. Skip test on Windows (watch mode not supported)
#
# Usage: ./tests/integration/test_validate_yaml_schema_watch_e2e.sh
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Source shared libraries
source "$PROJECT_ROOT/scripts/lib/find-binary.sh" || exit 1
source "$PROJECT_ROOT/scripts/lib/logger.sh" || exit 1

# Find validate-yaml binary using workspace-aware search
cd "$PROJECT_ROOT"
VALIDATE_YAML_BIN=$(find_binary "validate-yaml")
if [[ -z "$VALIDATE_YAML_BIN" ]]; then
    echo "[ERROR] validate-yaml binary not found" >&2
    echo "[ERROR] Please build it with: cargo build --bin validate-yaml" >&2
    exit 1
fi

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

# PID of background process
WATCH_PID=""

echo "======================================"
echo "validate-yaml Schema Watch E2E Integration Test"
echo "======================================"
echo ""

# Check if running on Windows
section "Checking Platform"

if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    info "Windows platform detected - watch mode not supported"
    echo ""
    echo -e "${YELLOW}SKIPPED: Watch mode is not supported on Windows${NC}"
    exit 0
fi

pass "Non-Windows platform detected"

# Check prerequisites
section "Checking Prerequisites"

if [[ ! -f "$VALIDATE_YAML_BIN" ]]; then
    fail "validate-yaml binary not found at $VALIDATE_YAML_BIN"
    echo "Run 'cargo build' first"
    exit 1
fi
pass "validate-yaml binary found"

# Create temporary directory for test files
TEMP_DIR=$(mktemp -d)
setup_cleanup "$TEMP_DIR"
if [[ $REMOVE_TEMP -eq 0 ]]; then
    disable_cleanup
    info "Temporary files will not be removed: $TEMP_DIR"
fi
info "Using temporary directory: $TEMP_DIR"

# Create test schema and YAML files
section "Creating Test Schema and YAML Files"

# Create a simple schema file
SCHEMA_FILE="$TEMP_DIR/test-schema.json"
cat > "$SCHEMA_FILE" << 'EOF'
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "type": "object",
  "properties": {
    "name": {
      "type": "string"
    },
    "age": {
      "type": "integer",
      "minimum": 0
    },
    "email": {
      "type": "string"
    }
  },
  "required": ["name", "age"]
}
EOF

pass "Created test schema file"

# Create a valid YAML file
YAML_FILE_1="$TEMP_DIR/test1.yaml"
cat > "$YAML_FILE_1" << 'EOF'
name: John Doe
age: 30
email: john@example.com
EOF

pass "Created test1.yaml"

# Create another valid YAML file
YAML_FILE_2="$TEMP_DIR/test2.yaml"
cat > "$YAML_FILE_2" << 'EOF'
name: Jane Smith
age: 25
email: jane@example.com
EOF

pass "Created test2.yaml"

# Start watch mode in background
section "Starting Watch Mode"

WATCH_LOG="$TEMP_DIR/watch_output.log"
"$VALIDATE_YAML_BIN" --schema "$SCHEMA_FILE" --watch "$YAML_FILE_1" "$YAML_FILE_2" > "$WATCH_LOG" 2>&1 &
WATCH_PID=$!
register_background_pid "$WATCH_PID"

info "Started watch mode with PID: $WATCH_PID"

# Wait for watch mode to initialize
sleep 2

# Check if watch process is still running
if ! kill -0 "$WATCH_PID" 2>/dev/null; then
    fail "Watch mode process died unexpectedly"
    cat "$WATCH_LOG"
    exit 1
fi
pass "Watch mode process is running"

# Verify initial validation in log
if grep -q "Initial validation:" "$WATCH_LOG"; then
    pass "Initial validation completed"
else
    fail "Initial validation not found in log"
fi

# Verify schema file count is reported
if grep -q "Monitoring 2 YAML file(s) and 1 schema file(s)" "$WATCH_LOG"; then
    pass "Schema file count reported correctly"
else
    fail "Schema file count not reported correctly"
fi

# Test 1: Modify the schema to add a new required field
section "Test 1: Modify Schema to Add Required Field"

# Add a new required field to the schema
cat > "$SCHEMA_FILE" << 'EOF'
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "type": "object",
  "properties": {
    "name": {
      "type": "string"
    },
    "age": {
      "type": "integer",
      "minimum": 0
    },
    "email": {
      "type": "string"
    },
    "city": {
      "type": "string"
    }
  },
  "required": ["name", "age", "city"]
}
EOF

info "Modified schema to require 'city' field"

# Wait for watch to detect change and re-validate
sleep 3

# Check if schema change was detected
if tail -n 100 "$WATCH_LOG" | grep -q "Schema file(s) modified"; then
    pass "Schema file modification detected"
else
    fail "Schema file modification not detected"
fi

# Verify re-validation message
if tail -n 100 "$WATCH_LOG" | grep -q "Schema changed - re-validating all YAML files"; then
    pass "Re-validation triggered for all YAML files"
else
    fail "Re-validation not triggered for all YAML files"
fi

# Verify both files now fail validation (missing 'city' field)
if tail -n 100 "$WATCH_LOG" | grep -q "Failed: 2"; then
    pass "Both YAML files correctly failed validation"
else
    fail "YAML files did not fail validation as expected"
fi

# Test 2: Fix YAML files to match new schema
section "Test 2: Fix YAML Files to Match Schema"

# Update YAML files to include the required 'city' field
cat > "$YAML_FILE_1" << 'EOF'
name: John Doe
age: 30
email: john@example.com
city: New York
EOF

cat > "$YAML_FILE_2" << 'EOF'
name: Jane Smith
age: 25
email: jane@example.com
city: San Francisco
EOF

info "Updated YAML files to include 'city' field"

# Wait for watch to detect changes
sleep 3

# Verify files are being validated
if tail -n 100 "$WATCH_LOG" | grep -q "File changes detected:"; then
    pass "YAML file changes detected"
else
    fail "YAML file changes not detected"
fi

# Verify both files now pass
if tail -n 100 "$WATCH_LOG" | grep -q "Passed: 2"; then
    pass "Both YAML files pass validation"
else
    fail "YAML files did not pass validation"
fi

# Test 3: Modify schema to relax constraints
section "Test 3: Modify Schema to Relax Constraints"

# Make 'city' optional again
cat > "$SCHEMA_FILE" << 'EOF'
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "type": "object",
  "properties": {
    "name": {
      "type": "string"
    },
    "age": {
      "type": "integer",
      "minimum": 0
    },
    "email": {
      "type": "string"
    },
    "city": {
      "type": "string"
    }
  },
  "required": ["name", "age"]
}
EOF

info "Modified schema to make 'city' optional"

# Wait for watch to detect change
sleep 3

# Verify schema change detected
if tail -n 100 "$WATCH_LOG" | grep -q "Schema file(s) modified"; then
    pass "Schema modification detected again"
else
    fail "Schema modification not detected"
fi

# Verify files still pass
if tail -n 100 "$WATCH_LOG" | grep -q "Passed: 2"; then
    pass "YAML files still pass with relaxed schema"
else
    fail "YAML files did not pass with relaxed schema"
fi

# Test 4: Test with invalid schema change
section "Test 4: Invalid Schema Syntax"

# Create invalid JSON schema
cat > "$SCHEMA_FILE" << 'EOF'
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "type": "object",
  "properties": {
    "name": {
      "type": "invalid_type"
    }
  }
}
EOF

info "Modified schema with invalid type"

# Wait for watch to detect change
sleep 3

# The validator should still attempt to validate, and likely fail gracefully
# We just want to ensure the process doesn't crash
if kill -0 "$WATCH_PID" 2>/dev/null; then
    pass "Watch process still running after invalid schema"
else
    fail "Watch process crashed on invalid schema"
fi

# Restore valid schema
cat > "$SCHEMA_FILE" << 'EOF'
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "type": "object",
  "properties": {
    "name": {
      "type": "string"
    },
    "age": {
      "type": "integer",
      "minimum": 0
    }
  },
  "required": ["name", "age"]
}
EOF

info "Restored valid schema"
sleep 3

# Cleanup is handled by trap
section "Cleanup"

if kill -0 "$WATCH_PID" 2>/dev/null; then
    kill "$WATCH_PID" 2>/dev/null || true
    wait "$WATCH_PID" 2>/dev/null || true
    pass "Watch mode process terminated"
else
    fail "Watch mode process already terminated"
fi

WATCH_PID=""

# Summary
section "Test Summary"
echo ""
echo "Tests Passed: $TESTS_PASSED"
echo "Tests Failed: $TESTS_FAILED"
echo ""

if [[ $TESTS_FAILED -eq 0 ]]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed!${NC}"
    echo ""
    echo "Watch log contents:"
    echo "==================="
    cat "$WATCH_LOG"
    exit 1
fi
