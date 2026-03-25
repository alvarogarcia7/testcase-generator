#!/bin/bash
#
# End-to-end integration test for validate-yaml watch mode with transitive schema dependencies
#
# This test validates:
# 1. Watch mode discovers and monitors transitive schema dependencies
# 2. Changes to referenced schemas trigger re-validation
# 3. Multi-level schema references (schema A -> schema B -> schema C) are handled
# 4. Circular schema references don't cause infinite loops
# 5. Skip test on Windows (watch mode not supported)
#
# Usage: ./tests/integration/test_validate_yaml_transitive_schema_watch_e2e.sh
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
echo "validate-yaml Transitive Schema Watch E2E Test"
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

# Create test schema hierarchy
section "Creating Schema Hierarchy"

# Create base definitions schema (level 2)
BASE_SCHEMA="$TEMP_DIR/base-definitions.json"
cat > "$BASE_SCHEMA" << 'EOF'
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "definitions": {
    "name": {
      "type": "string",
      "minLength": 1,
      "maxLength": 100
    },
    "email": {
      "type": "string",
      "pattern": "^[^@]+@[^@]+\\.[^@]+$"
    }
  }
}
EOF

pass "Created base-definitions.json (level 2)"

# Create person schema that references base (level 1)
PERSON_SCHEMA="$TEMP_DIR/person-schema.json"
cat > "$PERSON_SCHEMA" << 'EOF'
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "definitions": {
    "person": {
      "type": "object",
      "properties": {
        "name": {
          "$ref": "base-definitions.json#/definitions/name"
        },
        "email": {
          "$ref": "base-definitions.json#/definitions/email"
        },
        "age": {
          "type": "integer",
          "minimum": 0,
          "maximum": 150
        }
      },
      "required": ["name", "age"]
    }
  }
}
EOF

pass "Created person-schema.json (level 1)"

# Create main schema that references person schema (level 0)
MAIN_SCHEMA="$TEMP_DIR/main-schema.json"
cat > "$MAIN_SCHEMA" << 'EOF'
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "type": "object",
  "properties": {
    "user": {
      "$ref": "person-schema.json#/definitions/person"
    },
    "timestamp": {
      "type": "string"
    }
  },
  "required": ["user"]
}
EOF

pass "Created main-schema.json (level 0)"

# Create test YAML files
section "Creating Test YAML Files"

YAML_FILE="$TEMP_DIR/test.yaml"
cat > "$YAML_FILE" << 'EOF'
user:
  name: Alice Johnson
  age: 28
  email: alice@example.com
timestamp: "2024-01-01T00:00:00Z"
EOF

pass "Created test.yaml"

# Start watch mode in background
section "Starting Watch Mode"

WATCH_LOG="$TEMP_DIR/watch_output.log"
"$VALIDATE_YAML_BIN" --schema "$MAIN_SCHEMA" --watch "$YAML_FILE" > "$WATCH_LOG" 2>&1 &
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

# Verify initial validation
if grep -q "Initial validation:" "$WATCH_LOG"; then
    pass "Initial validation completed"
else
    fail "Initial validation not found in log"
fi

# Verify all 3 schema files are being watched (main + person + base)
if grep -q "Monitoring 1 YAML file(s) and 3 schema file(s)" "$WATCH_LOG"; then
    pass "All 3 schema files (including transitive dependencies) are being watched"
else
    fail "Not all schema files are being watched"
    cat "$WATCH_LOG"
fi

# Test 1: Modify the deepest schema (base-definitions.json)
section "Test 1: Modify Base Definitions Schema (Level 2)"

# Change the email pattern in base schema
cat > "$BASE_SCHEMA" << 'EOF'
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "definitions": {
    "name": {
      "type": "string",
      "minLength": 1,
      "maxLength": 100
    },
    "email": {
      "type": "string",
      "pattern": "^.+@example\\.com$"
    }
  }
}
EOF

info "Modified base-definitions.json to require @example.com domain"

# Wait for watch to detect change
sleep 3

# Check if schema change was detected
if tail -n 100 "$WATCH_LOG" | grep -q "Schema file(s) modified"; then
    pass "Base schema modification detected"
else
    fail "Base schema modification not detected"
fi

# Verify re-validation occurred
if tail -n 100 "$WATCH_LOG" | grep -q "Schema changed - re-validating all YAML files"; then
    pass "Re-validation triggered by transitive schema change"
else
    fail "Re-validation not triggered"
fi

# File should still pass (email is alice@example.com)
if tail -n 100 "$WATCH_LOG" | grep -q "Passed: 1"; then
    pass "YAML file still passes with modified base schema"
else
    fail "YAML file did not pass validation"
fi

# Test 2: Modify middle schema (person-schema.json)
section "Test 2: Modify Person Schema (Level 1)"

# Add minimum age requirement
cat > "$PERSON_SCHEMA" << 'EOF'
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "definitions": {
    "person": {
      "type": "object",
      "properties": {
        "name": {
          "$ref": "base-definitions.json#/definitions/name"
        },
        "email": {
          "$ref": "base-definitions.json#/definitions/email"
        },
        "age": {
          "type": "integer",
          "minimum": 18,
          "maximum": 150
        }
      },
      "required": ["name", "age"]
    }
  }
}
EOF

info "Modified person-schema.json to require minimum age of 18"

# Wait for watch to detect change
sleep 3

# Verify detection
if tail -n 100 "$WATCH_LOG" | grep -q "Schema file(s) modified"; then
    pass "Person schema modification detected"
else
    fail "Person schema modification not detected"
fi

# File should still pass (age is 28)
if tail -n 100 "$WATCH_LOG" | grep -q "Passed: 1"; then
    pass "YAML file passes with age requirement"
else
    fail "YAML file did not pass"
fi

# Test 3: Create YAML that violates the constraint
section "Test 3: Update YAML to Violate Age Constraint"

cat > "$YAML_FILE" << 'EOF'
user:
  name: Bob Young
  age: 16
  email: bob@example.com
timestamp: "2024-01-02T00:00:00Z"
EOF

info "Modified test.yaml with age below minimum"

# Wait for watch to detect change
sleep 3

# Verify YAML change detected
if tail -n 100 "$WATCH_LOG" | grep -q "File changes detected:"; then
    pass "YAML file change detected"
else
    fail "YAML file change not detected"
fi

# Should fail validation
if tail -n 100 "$WATCH_LOG" | grep -q "Failed: 1"; then
    pass "YAML file correctly failed validation"
else
    fail "YAML file did not fail as expected"
fi

# Test 4: Relax constraint and verify YAML passes
section "Test 4: Relax Age Constraint"

cat > "$PERSON_SCHEMA" << 'EOF'
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "definitions": {
    "person": {
      "type": "object",
      "properties": {
        "name": {
          "$ref": "base-definitions.json#/definitions/name"
        },
        "email": {
          "$ref": "base-definitions.json#/definitions/email"
        },
        "age": {
          "type": "integer",
          "minimum": 0,
          "maximum": 150
        }
      },
      "required": ["name", "age"]
    }
  }
}
EOF

info "Relaxed age constraint to minimum 0"

# Wait for watch to detect change
sleep 3

# Verify schema change
if tail -n 100 "$WATCH_LOG" | grep -q "Schema file(s) modified"; then
    pass "Schema relaxation detected"
else
    fail "Schema relaxation not detected"
fi

# Should pass now
if tail -n 100 "$WATCH_LOG" | grep -q "Passed: 1"; then
    pass "YAML file passes after relaxing constraint"
else
    fail "YAML file did not pass"
fi

# Test 5: Test circular reference handling
section "Test 5: Create Circular Schema Reference"

# Create a circular reference: main -> person -> main (should not cause issues)
cat > "$PERSON_SCHEMA" << 'EOF'
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "definitions": {
    "person": {
      "type": "object",
      "properties": {
        "name": {
          "$ref": "base-definitions.json#/definitions/name"
        },
        "email": {
          "$ref": "base-definitions.json#/definitions/email"
        },
        "age": {
          "type": "integer",
          "minimum": 0,
          "maximum": 150
        },
        "manager": {
          "$ref": "main-schema.json#/properties/user"
        }
      },
      "required": ["name", "age"]
    }
  }
}
EOF

info "Created circular reference between schemas"

# Wait for watch to detect change
sleep 3

# Process should still be running (no infinite loop)
if kill -0 "$WATCH_PID" 2>/dev/null; then
    pass "Watch process handles circular references without crashing"
else
    fail "Watch process crashed on circular reference"
fi

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
