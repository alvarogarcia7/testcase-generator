#!/bin/bash

# Script to generate a sample HTML report for req-coverage
# This demonstrates the full workflow: analyze coverage and generate HTML report

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
OUTPUT_DIR="${SCRIPT_DIR}/sample_report"
REQUIREMENTS_FILE="${SCRIPT_DIR}/sample_requirements.yaml"
TEST_CASES_DIR="${PROJECT_ROOT}/crates/testcase-manager/tests/integration/req_coverage_testdata/test_cases"
VERIFICATION_RESULTS_DIR="${PROJECT_ROOT}/crates/testcase-manager/tests/integration/req_coverage_testdata/verification_results"

echo "======================================"
echo "req-coverage Sample Report Generator"
echo "======================================"
echo ""

# Create a sample requirements file
echo "Creating sample requirements file..."
cat > "${REQUIREMENTS_FILE}" << 'EOF'
requirements:
  - id: REQ-001
    text: "The system shall authenticate users and validate credentials"
    description: "Authentication requirement"
  - id: REQ-002
    text: "The system shall log all security events and audit trails"
    description: "Logging requirement"
  - id: REQ-003
    text: "The system shall validate input data and sanitize user input"
    description: "Input validation requirement"
  - id: REQ-004
    text: "The system shall handle errors gracefully and provide meaningful error messages"
    description: "Error handling requirement"
  - id: REQ-005
    text: "The system shall support multiple user roles with different permissions"
    description: "Authorization requirement"
  - id: REQ-006
    text: "The system shall encrypt sensitive data in transit and at rest"
    description: "Encryption requirement"
EOF

echo "Sample requirements file created at: ${REQUIREMENTS_FILE}"
echo ""

# Create output directory
mkdir -p "${OUTPUT_DIR}"

# Step 1: Generate coverage report JSON
echo "Step 1: Analyzing coverage and generating JSON report..."
COVERAGE_JSON="${OUTPUT_DIR}/coverage_report.json"

cargo run -p req-coverage -- verify \
    --test-cases-folder "${TEST_CASES_DIR}" \
    --test-results-folder "${VERIFICATION_RESULTS_DIR}" \
    --requirements-file "${REQUIREMENTS_FILE}" \
    --output "${COVERAGE_JSON}" \
    --log-level info

echo ""
echo "Coverage JSON report generated at: ${COVERAGE_JSON}"
echo ""

# Step 2: Generate HTML report from JSON
echo "Step 2: Generating HTML report from JSON..."
HTML_OUTPUT_DIR="${OUTPUT_DIR}/html"

cargo run -p req-coverage -- print \
    --format html \
    --input "${COVERAGE_JSON}" \
    --output "${HTML_OUTPUT_DIR}" \
    --log-level info

echo ""
echo "======================================"
echo "Sample Report Generation Complete"
echo "======================================"
echo ""
echo "Coverage JSON report: ${COVERAGE_JSON}"
echo "HTML report: ${HTML_OUTPUT_DIR}/index.html"
echo ""
echo "To view the HTML report, open:"
echo "  ${HTML_OUTPUT_DIR}/index.html"
echo ""
echo "Or run:"
echo "  open ${HTML_OUTPUT_DIR}/index.html"
echo ""
