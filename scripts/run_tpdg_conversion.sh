#!/usr/bin/env bash
# Execute TPDG conversion and keep logs
# This script runs convert_verification_to_tpdg.py and saves all output

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

# Log file paths
LOG_DIR="$OUTPUT_DIR/logs"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
LOG_FILE="$LOG_DIR/conversion_${TIMESTAMP}.log"
ERROR_LOG="$LOG_DIR/conversion_${TIMESTAMP}.err"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo_info() {
    echo -e "${BLUE}[INFO]${NC} $*" | tee -a "$LOG_FILE"
}

echo_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $*" | tee -a "$LOG_FILE"
}

echo_error() {
    echo -e "${RED}[ERROR]${NC} $*" | tee -a "$LOG_FILE" >&2
}

echo_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $*" | tee -a "$LOG_FILE"
}

# Create log directory
mkdir -p "$LOG_DIR"
mkdir -p "$OUTPUT_DIR"

# Create or clear log files
: > "$LOG_FILE"
: > "$ERROR_LOG"

# Log script start
{
    echo "========================================="
    echo "TPDG Conversion Execution Log"
    echo "========================================="
    echo "Started: $(date)"
    echo "Script: $0"
    echo "Working Directory: $(pwd)"
    echo "========================================="
    echo ""
} >> "$LOG_FILE"

echo_info "TPDG Conversion Starting..."
echo_info "Logs will be saved to: $LOG_DIR"
echo ""

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
    echo_info "Creating directory: $LOGS_DIR"
    mkdir -p "$LOGS_DIR"
fi

# Check for Python
if ! command -v python3.14 &> /dev/null && ! command -v python3 &> /dev/null; then
    echo_error "Python 3 not found. Please install Python 3.14 or Python 3."
    exit 1
fi

PYTHON_CMD=$(command -v python3.14 2>/dev/null || command -v python3)
echo_info "Using Python: $PYTHON_CMD"

# Check for PyYAML
if ! $PYTHON_CMD -c "import yaml" 2>/dev/null; then
    echo_error "PyYAML not installed. Install with: pip3 install pyyaml"
    exit 1
fi

echo_success "Prerequisites verified"
echo ""

# Log configuration
{
    echo "Configuration:"
    echo "  Python: $PYTHON_CMD"
    echo "  Conversion Script: $CONVERSION_SCRIPT"
    echo "  Test Case Directory: $TEST_CASE_DIR"
    echo "  Logs Directory: $LOGS_DIR"
    echo "  Output File: $OUTPUT_FILE"
    echo "  Log File: $LOG_FILE"
    echo "  Error Log: $ERROR_LOG"
    echo ""
} >> "$LOG_FILE"

# Run conversion script with all output captured
echo_info "Running TPDG conversion..."
echo_info "Command: python3 scripts/convert_verification_to_tpdg.py \\"
echo_info "    --test-case-dir test-acceptance/test_cases \\"
echo_info "    --logs-dir test-acceptance/execution_logs \\"
echo_info "    --recursive \\"
echo_info "    --output test-acceptance/results/acceptance_test_results_container.yaml \\"
echo_info "    --title \"Acceptance Test Suite Results\" \\"
echo_info "    --project \"Test Case Manager - Acceptance Test Suite\" \\"
echo_info "    --verbose"
echo ""

# Execute conversion with output capture
if $PYTHON_CMD "$CONVERSION_SCRIPT" \
    --test-case-dir "$TEST_CASE_DIR" \
    --logs-dir "$LOGS_DIR" \
    --recursive \
    --output "$OUTPUT_FILE" \
    --title "Acceptance Test Suite Results" \
    --project "Test Case Manager - Acceptance Test Suite" \
    --verbose \
    2>&1 | tee -a "$LOG_FILE"; then
    
    CONVERSION_EXIT_CODE=0
else
    CONVERSION_EXIT_CODE=$?
fi

echo ""

# Check conversion result
if [[ $CONVERSION_EXIT_CODE -eq 0 ]]; then
    echo_success "TPDG conversion completed successfully!"
    echo_info "Output file: $OUTPUT_FILE"
else
    echo_error "TPDG conversion failed with exit code: $CONVERSION_EXIT_CODE"
    {
        echo ""
        echo "ERROR: Conversion failed"
        echo "Exit code: $CONVERSION_EXIT_CODE"
    } >> "$ERROR_LOG"
fi

# Display file statistics if output exists
if [[ -f "$OUTPUT_FILE" ]]; then
    FILE_SIZE=$(du -h "$OUTPUT_FILE" | cut -f1)
    LINE_COUNT=$(wc -l < "$OUTPUT_FILE")
    echo ""
    echo_info "Generated file statistics:"
    echo_info "  Size: $FILE_SIZE"
    echo_info "  Lines: $LINE_COUNT"
    
    {
        echo ""
        echo "Output File Statistics:"
        echo "  Path: $OUTPUT_FILE"
        echo "  Size: $FILE_SIZE"
        echo "  Lines: $LINE_COUNT"
    } >> "$LOG_FILE"
fi

# Count test cases in logs directory
if [[ -d "$LOGS_DIR" ]]; then
    LOG_COUNT=$(find "$LOGS_DIR" -name "*_execution_log.json" -type f 2>/dev/null | wc -l)
    echo_info "Execution logs found: $LOG_COUNT"
    
    {
        echo "  Execution logs found: $LOG_COUNT"
    } >> "$LOG_FILE"
fi

# Log script end
{
    echo ""
    echo "========================================="
    echo "Completed: $(date)"
    echo "Exit Code: $CONVERSION_EXIT_CODE"
    echo "========================================="
} >> "$LOG_FILE"

echo ""
echo_info "Logs saved to:"
echo_info "  Standard output: $LOG_FILE"
if [[ -s "$ERROR_LOG" ]]; then
    echo_info "  Error output: $ERROR_LOG"
fi

# Create a symlink to latest log
LATEST_LOG_LINK="$LOG_DIR/latest.log"
ln -sf "$(basename "$LOG_FILE")" "$LATEST_LOG_LINK"
echo_info "  Latest log link: $LATEST_LOG_LINK"

echo ""
if [[ $CONVERSION_EXIT_CODE -eq 0 ]]; then
    echo_success "Script completed successfully!"
else
    echo_error "Script completed with errors!"
fi

exit $CONVERSION_EXIT_CODE
