#!/bin/bash
# Audit Traceability Workflow Demo
# This script demonstrates the complete workflow of using audit traceability logs

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Audit Traceability Workflow Demo ===${NC}\n"

# Setup
DEMO_DIR="audit_demo_$$"
mkdir -p "$DEMO_DIR"
cd "$DEMO_DIR"

echo -e "${GREEN}Created demo directory: $DEMO_DIR${NC}\n"

# Step 1: Create a sample test case
echo -e "${BLUE}Step 1: Creating sample test case YAML${NC}"
cat > TC001.yaml << 'EOF'
id: TC001
description: Sample Test Case for Audit Demo
requirement: REQ-DEMO-001
item: 1
tc: 1
test_sequences:
  - id: 1
    name: Basic Validation
    steps:
      - step: 1
        description: Echo test message
        command: echo "Hello from test case"
        verification:
          result: "0"
          output: "Hello from test case"
      - step: 2
        description: Check system date
        command: date +%Y
        verification:
          result: "0"
          output: ".*"
EOF

echo -e "${GREEN}✓ Created TC001.yaml${NC}\n"

# Step 2: Create audit log
echo -e "${BLUE}Step 2: Creating audit traceability log${NC}"
cargo run --bin test-executor -- audit-log create \
  --output audit-traceability-log.json \
  --witness-key "demo-system-$(date +%Y%m%d)"

echo -e "${GREEN}✓ Audit log created${NC}\n"

# Step 3: Generate script with audit logging
echo -e "${BLUE}Step 3: Generating test script with audit logging${NC}"
cargo run --bin test-executor -- generate TC001.yaml \
  --output TC001.sh \
  --audit-log audit-traceability-log.json

echo -e "${GREEN}✓ Test script generated and added to audit log${NC}\n"

# Step 4: Display audit log
echo -e "${BLUE}Step 4: Displaying audit log contents${NC}"
echo -e "${YELLOW}--- audit-traceability-log.json ---${NC}"
cat audit-traceability-log.json | jq .
echo ""

# Step 5: Verify files
echo -e "${BLUE}Step 5: Verifying file integrity${NC}"
cargo run --bin test-executor -- audit-log verify \
  --log-file audit-traceability-log.json

echo ""

# Step 6: Simulate file modification
echo -e "${BLUE}Step 6: Simulating file modification${NC}"
echo "# Modified" >> TC001.sh
echo -e "${YELLOW}Modified TC001.sh by appending comment${NC}\n"

# Step 7: Verify after modification (should fail)
echo -e "${BLUE}Step 7: Verifying after modification (should detect change)${NC}"
if cargo run --bin test-executor -- audit-log verify \
  --log-file audit-traceability-log.json --test-case-id TC001 2>&1; then
  echo -e "${RED}✗ Verification should have failed but passed${NC}"
else
  echo -e "${GREEN}✓ Verification correctly detected file modification${NC}"
fi

echo ""

# Step 8: Re-generate to update audit log
echo -e "${BLUE}Step 8: Re-generating script to update audit log${NC}"
cargo run --bin test-executor -- generate TC001.yaml \
  --output TC001.sh \
  --audit-log audit-traceability-log.json

echo -e "${GREEN}✓ Script re-generated and audit log updated${NC}\n"

# Step 9: Verify again (should pass)
echo -e "${BLUE}Step 9: Verifying after re-generation (should pass)${NC}"
cargo run --bin test-executor -- audit-log verify \
  --log-file audit-traceability-log.json

echo ""

# Cleanup
echo -e "${BLUE}Cleaning up demo directory${NC}"
cd ..
rm -rf "$DEMO_DIR"

echo -e "${GREEN}=== Demo Complete ===${NC}"
echo -e "\nKey Takeaways:"
echo "  • Audit logs track file hashes (SHA-256) for integrity verification"
echo "  • Any modification to tracked files is detected during verification"
echo "  • Re-generating updates the audit log with new hashes"
echo "  • Suitable for CI/CD pipelines and compliance requirements"
