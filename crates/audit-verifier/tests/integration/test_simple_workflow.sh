#!/usr/bin/env bash
# Simple workflow demonstration test for audit-verifier
#
# This is a minimal test that demonstrates the complete workflow:
# 1. Create a test case YAML
# 2. Create an execution log with matching hash
# 3. Run audit-verifier to verify and sign
# 4. Verify the signature
#
# Usage: ./tests/integration/test_simple_workflow.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"

# Source shared libraries
source "$PROJECT_ROOT/scripts/lib/find-binary.sh" || exit 1
source "$PROJECT_ROOT/scripts/lib/logger.sh" || exit 1

# Find binaries
cd "$PROJECT_ROOT"
AUDIT_VERIFIER_BIN=$(find_binary "audit-verifier")
VERIFY_SIGNATURE_BIN=$(find_binary "verify-audit-signature")

if [[ -z "$AUDIT_VERIFIER_BIN" ]] || [[ -z "$VERIFY_SIGNATURE_BIN" ]]; then
    echo "[ERROR] Required binaries not found. Build with: cargo build -p audit-verifier" >&2
    exit 1
fi

echo "================================="
echo "Simple Workflow Demonstration"
echo "================================="
echo ""

# Create temp directory
TEMP_DIR=$(mktemp -d)
trap "rm -rf '$TEMP_DIR'" EXIT

# Step 1: Create test YAML
echo "Step 1: Creating test case YAML..."
TEST_YAML="$TEMP_DIR/test.yaml"
cat > "$TEST_YAML" << 'EOF'
type: test_case
id: SIMPLE_TEST_001
description: Simple test
test_sequences:
- id: 1
  name: Test
  steps:
  - step: 1
    command: echo "test"
    verification:
      result: '[[ $EXIT_CODE -eq 0 ]]'
EOF
echo "   ✓ Created: $TEST_YAML"

# Step 2: Compute hash and create log
echo "Step 2: Creating execution log with hash..."
YAML_HASH=$(shasum -a 256 "$TEST_YAML" | awk '{print $1}')
echo "   Computed hash: $YAML_HASH"

LOG_FILE="$TEMP_DIR/log.json"
cat > "$LOG_FILE" << EOF
[
  {
    "test_sequence": 1,
    "step": 1,
    "command": "echo \"test\"",
    "exit_code": 0,
    "output": "test",
    "timestamp": "2024-03-27T10:00:00Z",
    "result_verification_pass": true,
    "output_verification_pass": true,
    "source_yaml_sha256": "$YAML_HASH"
  }
]
EOF
echo "   ✓ Created: $LOG_FILE"

# Step 3: Verify and sign
echo "Step 3: Running audit-verifier..."
SIGNED_OUTPUT="$TEMP_DIR/signed.json"
KEY_FILE="$TEMP_DIR/key.pem"

if "$AUDIT_VERIFIER_BIN" \
    --yaml "$TEST_YAML" \
    --log "$LOG_FILE" \
    --save-key "$KEY_FILE" \
    --key-id "demo-key" \
    --output "$SIGNED_OUTPUT" \
    --log-level error 2>&1; then
    echo "   ✓ Verification and signing complete"
else
    echo "   ✗ Failed"
    exit 1
fi

# Step 4: Verify signature
echo "Step 4: Verifying signature..."
if "$VERIFY_SIGNATURE_BIN" --input "$SIGNED_OUTPUT" > /dev/null 2>&1; then
    echo "   ✓ Signature valid"
else
    echo "   ✗ Signature invalid"
    exit 1
fi

echo ""
echo "================================="
echo "✓ Workflow completed successfully"
echo "================================="
echo ""
echo "Generated files (temporary):"
echo "  - Test YAML: $TEST_YAML"
echo "  - Execution log: $LOG_FILE"
echo "  - Private key: $KEY_FILE"
echo "  - Signed output: $SIGNED_OUTPUT"
echo ""

exit 0
