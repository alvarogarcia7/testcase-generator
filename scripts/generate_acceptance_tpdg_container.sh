#!/usr/bin/env bash
# Generate TPDG container YAML from acceptance test cases and execution logs
# This script uses the dual-source mode of convert_verification_to_tpdg.py

set -euo pipefail

# Get script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Paths
TEST_CASE_DIR="$PROJECT_ROOT/test-acceptance/test_cases"
LOGS_DIR="$PROJECT_ROOT/test-acceptance/execution_logs"
OUTPUT_DIR="$PROJECT_ROOT/test-acceptance/results"
OUTPUT_FILE="$OUTPUT_DIR/acceptance_test_results_container.yaml"
CONVERSION_SCRIPT="$SCRIPT_DIR/convert_verification_to_tpdg.py"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo_info() {
    echo -e "${BLUE}[INFO]${NC} $*"
}

echo_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $*"
}

echo_error() {
    echo -e "${RED}[ERROR]${NC} $*"
}

echo_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $*"
}

# Verify prerequisites
echo_info "Verifying prerequisites..."

if [[ ! -f "$CONVERSION_SCRIPT" ]]; then
    echo_error "Conversion script not found: $CONVERSION_SCRIPT"
    exit 1
fi

if [[ ! -d "$TEST_CASE_DIR" ]]; then
    echo_error "Test case directory not found: $TEST_CASE_DIR"
    exit 1
fi

if [[ ! -d "$LOGS_DIR" ]]; then
    echo_warning "Execution logs directory not found: $LOGS_DIR"
    echo_info "This may be expected if tests haven't been executed yet."
    echo_info "The script will still run, but all test steps will be marked as NotExecuted."
fi

# Check for Python and PyYAML
if ! command -v python3.14 &> /dev/null && ! command -v python3 &> /dev/null; then
    echo_error "Python 3 not found. Please install Python 3.14 or Python 3."
    exit 1
fi

PYTHON_CMD=$(command -v python3.14 2>/dev/null || command -v python3)

# Check for PyYAML
if ! $PYTHON_CMD -c "import yaml" 2>/dev/null; then
    echo_error "PyYAML not installed. Install with: pip3 install pyyaml"
    exit 1
fi

echo_success "Prerequisites verified"
echo ""

# Create output directory
echo_info "Creating output directory: $OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

# Run conversion script in dual-source mode
echo_info "Running conversion script in dual-source mode..."
echo_info "  Test cases: $TEST_CASE_DIR"
echo_info "  Logs: $LOGS_DIR"
echo_info "  Output: $OUTPUT_FILE"
echo ""

if $PYTHON_CMD "$CONVERSION_SCRIPT" \
    --test-case-dir "$TEST_CASE_DIR" \
    --logs-dir "$LOGS_DIR" \
    --recursive \
    --output "$OUTPUT_FILE" \
    --title "Acceptance Test Suite Results" \
    --project "Test Case Manager - Acceptance Test Suite" \
    --verbose; then
    
    echo ""
    echo_success "TPDG container YAML generated successfully!"
    echo_info "Output file: $OUTPUT_FILE"
else
    echo ""
    echo_error "Failed to generate TPDG container YAML"
    exit 1
fi

# Display file size and line count
if [[ -f "$OUTPUT_FILE" ]]; then
    FILE_SIZE=$(du -h "$OUTPUT_FILE" | cut -f1)
    LINE_COUNT=$(wc -l < "$OUTPUT_FILE")
    echo ""
    echo_info "Generated file statistics:"
    echo_info "  Size: $FILE_SIZE"
    echo_info "  Lines: $LINE_COUNT"
fi

# Add to git staging
echo ""
echo_info "Staging file for git commit..."
if git -C "$PROJECT_ROOT" add "$OUTPUT_FILE"; then
    echo_success "File staged for commit: $OUTPUT_FILE"
    echo ""
    echo_info "To commit the changes, run:"
    echo "  git commit -m 'Add acceptance test results TPDG container'"
else
    echo_warning "Failed to stage file for git commit"
    echo_info "You may need to manually add and commit the file:"
    echo "  git add $OUTPUT_FILE"
    echo "  git commit -m 'Add acceptance test results TPDG container'"
fi

echo ""
echo_success "Script completed successfully!"
