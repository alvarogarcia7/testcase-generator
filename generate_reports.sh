#!/usr/bin/env bash
#
# Generate documentation reports for verifier scenarios
#
# This script has been migrated to use test-plan-documentation-generator
# instead of Python PDF generation.
#
# Usage: ./generate_reports.sh
#

set -e

# Get script directory and source logger
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "======================================================================="
echo "Test Verifier Report Generator"
echo "======================================================================="
echo ""
echo "This script has been migrated to use test-plan-documentation-generator."
echo "For report generation, please use:"
echo ""
echo "  make generate-docs              # Generate reports for verifier scenarios"
echo "  make generate-docs-all          # Generate reports for all test cases"
echo ""
echo "Or run the scripts directly:"
echo ""
echo "  ./scripts/run_verifier_and_generate_reports.sh"
echo "  ./scripts/generate_documentation_reports.sh"
echo ""
echo "======================================================================="
echo ""

# Redirect to the actual implementation
if [ -f "$SCRIPT_DIR/scripts/run_verifier_and_generate_reports.sh" ]; then
    echo "Running report generation via run_verifier_and_generate_reports.sh..."
    echo ""
    exec "$SCRIPT_DIR/scripts/run_verifier_and_generate_reports.sh"
else
    echo "Error: scripts/run_verifier_and_generate_reports.sh not found"
    exit 1
fi
