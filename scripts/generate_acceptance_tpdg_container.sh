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
LOG_FILE="$OUTPUT_DIR/generation.log"
CONVERSION_SCRIPT="$SCRIPT_DIR/convert_verification_to_tpdg.py"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo_info() {
    local msg="[INFO] $*"
    echo -e "${BLUE}${msg}${NC}"
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] ${msg}" >> "$LOG_FILE"
}

echo_success() {
    local msg="[SUCCESS] $*"
    echo -e "${GREEN}${msg}${NC}"
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] ${msg}" >> "$LOG_FILE"
}

echo_error() {
    local msg="[ERROR] $*"
    echo -e "${RED}${msg}${NC}"
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] ${msg}" >> "$LOG_FILE"
}

echo_warning() {
    local msg="[WARNING] $*"
    echo -e "${YELLOW}${msg}${NC}"
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] ${msg}" >> "$LOG_FILE"
}

# Create output directory first
mkdir -p "$OUTPUT_DIR"

# Initialize log file
echo "========================================" > "$LOG_FILE"
echo "TPDG Container Generation Log" >> "$LOG_FILE"
echo "Started: $(date)" >> "$LOG_FILE"
echo "========================================" >> "$LOG_FILE"
echo "" >> "$LOG_FILE"

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
echo_info "Using Python: $PYTHON_CMD"

# Check for PyYAML
if ! $PYTHON_CMD -c "import yaml" 2>/dev/null; then
    echo_error "PyYAML not installed. Install with: pip3 install pyyaml"
    exit 1
fi

echo_success "Prerequisites verified"
echo ""

# Run conversion script in dual-source mode
echo_info "Running conversion script in dual-source mode..."
echo_info "  Test cases: $TEST_CASE_DIR"
echo_info "  Logs: $LOGS_DIR"
echo_info "  Output: $OUTPUT_FILE"
echo_info "  Log file: $LOG_FILE"
echo ""

# Execute the conversion and capture all output to log file
CONVERSION_START=$(date +%s)

if $PYTHON_CMD "$CONVERSION_SCRIPT" \
    --test-case-dir "$TEST_CASE_DIR" \
    --logs-dir "$LOGS_DIR" \
    --recursive \
    --output "$OUTPUT_FILE" \
    --title "Acceptance Test Suite Results" \
    --project "Test Case Manager - Acceptance Test Suite" \
    --verbose 2>&1 | tee -a "$LOG_FILE"; then
    
    CONVERSION_END=$(date +%s)
    CONVERSION_DURATION=$((CONVERSION_END - CONVERSION_START))
    
    echo ""
    echo_success "TPDG container YAML generated successfully!"
    echo_info "Output file: $OUTPUT_FILE"
    echo_info "Conversion time: ${CONVERSION_DURATION}s"
else
    CONVERSION_END=$(date +%s)
    CONVERSION_DURATION=$((CONVERSION_END - CONVERSION_START))
    
    echo ""
    echo_error "Failed to generate TPDG container YAML"
    echo_info "Conversion time: ${CONVERSION_DURATION}s"
    echo_info "Check log file for details: $LOG_FILE"
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
if git -C "$PROJECT_ROOT" add "$OUTPUT_FILE" 2>&1 | tee -a "$LOG_FILE"; then
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

# Finalize log
echo "" >> "$LOG_FILE"
echo "========================================" >> "$LOG_FILE"
echo "Completed: $(date)" >> "$LOG_FILE"
echo "========================================" >> "$LOG_FILE"

echo ""
echo_success "Script completed successfully!"
echo_info "Full log saved to: $LOG_FILE"
