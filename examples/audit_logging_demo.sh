#!/bin/bash
set -e

echo "=== Audit Logging and Digital Signature Demo ==="
echo ""

# Setup
DEMO_DIR=$(mktemp -d)
echo "Demo directory: $DEMO_DIR"
cd "$DEMO_DIR"

# Create a sample test case YAML
cat > test_example.yml << 'EOF'
id: AUDIT-DEMO-001
description: Sample test case for audit logging demonstration
test_sequences:
  - name: Setup
    steps:
      - description: Create test file
        command: echo "Hello, World!" > test.txt
        verification:
          result: exit_status == 0
          output: ""
  - name: Verify
    steps:
      - description: Check file contents
        command: cat test.txt
        verification:
          result: exit_status == 0
          output: grep "Hello"
EOF

echo "✓ Created sample test case: test_example.yml"
echo ""

# Step 1: Generate script with audit logging
echo "Step 1: Generate test script with audit logging"
export AUDIT_LOG_FILE="$DEMO_DIR/operations.audit.json"
cargo run --bin test-executor -- generate test_example.yml --output test_example.sh
echo "✓ Generated test_example.sh"
echo ""

# Step 2: Execute the test with audit logging
echo "Step 2: Execute test with audit logging"
cargo run --bin test-executor -- execute test_example.yml
echo "✓ Executed test"
echo ""

# Step 3: Show the audit log
echo "Step 3: View audit log contents"
echo "----------------------------------------"
cat operations.audit.json | jq '.'
echo "----------------------------------------"
echo ""

# Step 4: Sign the audit log
echo "Step 4: Sign the audit log with a digital signature"
cargo run --bin sign-audit-log -- \
  --log operations.audit.json \
  --output signed_operations.json \
  --save-key signing_key.pem \
  --key-id "demo-signing-key"
echo "✓ Audit log signed and saved to signed_operations.json"
echo "✓ Private key saved to signing_key.pem"
echo ""

# Step 5: Verify the signed audit log
echo "Step 5: Verify the signed audit log"
cargo run --bin verify-audit-log -- signed_operations.json --detailed
echo ""

# Step 6: Demonstrate tamper detection
echo "Step 6: Demonstrate tamper detection"
echo "Modifying the audit log to simulate tampering..."

# Make a copy and tamper with it
cp signed_operations.json tampered_operations.json
cat tampered_operations.json | jq '.audit_log.entries += [{"timestamp": "2024-01-01T00:00:00Z", "operation": "other", "status": "success"}]' > tmp.json
mv tmp.json tampered_operations.json

echo "✓ Tampered with audit log"
echo ""

echo "Verifying tampered audit log (should fail):"
if cargo run --bin verify-audit-log -- tampered_operations.json 2>&1; then
    echo "ERROR: Verification should have failed!"
    exit 1
else
    echo "✓ Tamper detection successful - verification failed as expected"
fi
echo ""

# Step 7: Show verification report
echo "Step 7: Generate verification report"
cargo run --bin verify-audit-log -- \
  signed_operations.json \
  --output verification_report.json

echo "✓ Verification report saved to verification_report.json"
echo ""
echo "Verification report contents:"
echo "----------------------------------------"
cat verification_report.json | jq '.'
echo "----------------------------------------"
echo ""

# Cleanup information
echo "=== Demo Complete ==="
echo ""
echo "Demo files created in: $DEMO_DIR"
echo ""
echo "Files:"
echo "  - test_example.yml          : Sample test case"
echo "  - test_example.sh           : Generated test script"
echo "  - operations.audit.json     : Audit log of operations"
echo "  - signed_operations.json    : Digitally signed audit log"
echo "  - signing_key.pem           : Private signing key"
echo "  - verification_report.json  : Signature verification report"
echo "  - tampered_operations.json  : Tampered audit log (for demo)"
echo ""
echo "To clean up: rm -rf $DEMO_DIR"
echo ""
echo "Key takeaways:"
echo "  ✓ All operations are automatically logged to the audit log"
echo "  ✓ Audit logs can be digitally signed for integrity"
echo "  ✓ Signatures can be verified to detect tampering"
echo "  ✓ File hashes ensure input/output file integrity"
echo "  ✓ Metadata tracks user, host, duration, and more"
