#!/usr/bin/env bash
# Execute TPDG conversion and keep logs
# This script runs test cases to generate execution logs, then runs convert_verification_to_tpdg.py

set -euo pipefail

# Get script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source logger library
source "$SCRIPT_DIR/lib/logger.sh" || {
    echo "ERROR: Failed to source logger library" >&2
    exit 1
}

# Paths
TEST_CASE_DIR="$PROJECT_ROOT/test-acceptance/test_cases"
LOGS_DIR="$PROJECT_ROOT/test-acceptance/execution_logs"
SCRIPTS_DIR="$PROJECT_ROOT/test-acceptance/scripts"
OUTPUT_DIR="$PROJECT_ROOT/test-acceptance/results"
OUTPUT_FILE="$OUTPUT_DIR/acceptance_test_results_container.yaml"
CONVERSION_SCRIPT="$SCRIPT_DIR/convert_verification_to_tpdg.py"

# Find binaries
cd "$PROJECT_ROOT"
source "$SCRIPT_DIR/lib/find-binary.sh" || {
    log_error "Failed to source find-binary.sh"
    exit 1
}

TEST_EXECUTOR=$(find_binary "test-executor")

# Log file paths
LOG_DIR="$OUTPUT_DIR/logs"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
LOG_FILE="$LOG_DIR/conversion_${TIMESTAMP}.log"
ERROR_LOG="$LOG_DIR/conversion_${TIMESTAMP}.err"

# Enhanced logging functions that also write to log file
log_to_file() {
    echo "$*" >> "$LOG_FILE"
}

echo_info() {
    log_info "$*"
    log_to_file "[INFO] $*"
}

echo_success() {
    pass "$*"
    log_to_file "[SUCCESS] $*"
}

echo_error() {
    log_error "$*"
    log_to_file "[ERROR] $*"
}

echo_warning() {
    log_warning "$*"
    log_to_file "[WARNING] $*"
}

echo_section() {
    section "$*"
    log_to_file ""
    log_to_file "=== $* ==="
}

# Create log directory
mkdir -p "$LOG_DIR"
mkdir -p "$OUTPUT_DIR"
mkdir -p "$LOGS_DIR"
mkdir -p "$SCRIPTS_DIR"

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

echo_section "TPDG Conversion with Test Execution"
echo_info "Logs will be saved to: $LOG_DIR"
echo ""

# Verify prerequisites
echo_section "Verifying Prerequisites"

if [[ ! -f "$CONVERSION_SCRIPT" ]]; then
    echo_error "Conversion script not found: $CONVERSION_SCRIPT"
    exit 1
fi

if [[ ! -d "$TEST_CASE_DIR" ]]; then
    echo_error "Test case directory not found: $TEST_CASE_DIR"
    exit 1
fi

if [[ ! -x "$TEST_EXECUTOR" ]]; then
    echo_error "test-executor binary not found at: $TEST_EXECUTOR"
    echo_info "Run: cargo build --bin test-executor"
    exit 1
fi

# Check for Python
if ! command -v python3.14 &> /dev/null && ! command -v python3 &> /dev/null; then
    echo_error "Python 3 not found. Please install Python 3.14 or Python 3."
    exit 1
fi

PYTHON_CMD=$(command -v python3.14 2>/dev/null || command -v python3)
PYTHON_CMD="uv run python"
echo_info "Using Python: $PYTHON_CMD"

# Check for PyYAML
if ! $PYTHON_CMD -c "import yaml" 2>/dev/null; then
    echo_error "PyYAML not installed. Install with: pip3 install pyyaml"
    exit 1
fi

pass "Prerequisites verified"
echo ""

# Log configuration
{
    echo "Configuration:"
    echo "  Python: $PYTHON_CMD"
    echo "  Test Executor: $TEST_EXECUTOR"
    echo "  Conversion Script: $CONVERSION_SCRIPT"
    echo "  Test Case Directory: $TEST_CASE_DIR"
    echo "  Scripts Directory: $SCRIPTS_DIR"
    echo "  Logs Directory: $LOGS_DIR"
    echo "  Output File: $OUTPUT_FILE"
    echo "  Log File: $LOG_FILE"
    echo "  Error Log: $ERROR_LOG"
    echo ""
} >> "$LOG_FILE"

# Stage 1: Generate test scripts
echo_section "Stage 1: Generating Test Scripts"

# Find all test case YAML files
mapfile -d '' YAML_FILES < <(find "$TEST_CASE_DIR" -type f \( -name "*.yaml" -o -name "*.yml" \) -print0 | sort -z)

TOTAL_TEST_CASES=${#YAML_FILES[@]}
GENERATION_SUCCESS=0
GENERATION_FAILED=0
GENERATION_SKIPPED=0

echo_info "Found $TOTAL_TEST_CASES test case files"
echo ""

for yaml_file in "${YAML_FILES[@]}"; do
    basename=$(basename "$yaml_file" .yaml)
    basename=$(basename "$basename" .yml)
    script_file="$SCRIPTS_DIR/${basename}.sh"
    
    # Check if this is a hook script or other non-test-case file
    if ! grep -q "^type: test_case" "$yaml_file" 2>/dev/null; then
        ((GENERATION_SKIPPED++))
        info "$basename (not a test_case, skipped)"
        continue
    fi
    
    # Generate script with --json-log flag
    if "$TEST_EXECUTOR" generate --json-log --test-case-dir "$TEST_CASE_DIR" --output "$script_file" "$yaml_file" >> "$LOG_FILE" 2>&1; then
        ((GENERATION_SUCCESS++))
        pass "$basename.sh"
        chmod +x "$script_file"
    else
        ((GENERATION_FAILED++))
        fail "$basename.sh"
        echo "$yaml_file" >> "$ERROR_LOG"
    fi
done

echo ""
echo_info "Script Generation: $GENERATION_SUCCESS passed, $GENERATION_FAILED failed, $GENERATION_SKIPPED skipped"
echo ""

if [[ $GENERATION_FAILED -gt 0 ]]; then
    echo_warning "Some scripts failed to generate, but continuing with available scripts"
fi

# Stage 2: Execute test scripts
echo_section "Stage 2: Executing Test Scripts"

# Find all generated scripts
mapfile -d '' SCRIPT_FILES < <(find "$SCRIPTS_DIR" -type f -name "*.sh" -print0 | sort -z)

EXECUTION_SUCCESS=0
EXECUTION_FAILED=0
EXECUTION_SKIPPED=0

echo_info "Found ${#SCRIPT_FILES[@]} test scripts to execute"
echo ""

for script_file in "${SCRIPT_FILES[@]}"; do
    basename=$(basename "$script_file" .sh)
    
    # Skip if it's a hook script (no corresponding test case)
    yaml_file=""
    if [[ -f "$TEST_CASE_DIR/${basename}.yaml" ]]; then
        yaml_file="$TEST_CASE_DIR/${basename}.yaml"
    elif [[ -f "$TEST_CASE_DIR/${basename}.yml" ]]; then
        yaml_file="$TEST_CASE_DIR/${basename}.yml"
    else
        # Try to find in subdirectories
        yaml_file=$(find "$TEST_CASE_DIR" -type f -name "${basename}.yaml" -o -name "${basename}.yml" | head -1)
    fi
    
    if [[ -z "$yaml_file" ]]; then
        ((EXECUTION_SKIPPED++))
        info "$basename.sh (no test case found, skipped)"
        continue
    fi
    
    # Check if this is a manual test
    if grep -q "manual: true" "$yaml_file" 2>/dev/null; then
        ((EXECUTION_SKIPPED++))
        info "$basename.sh (manual test, skipped)"
        continue
    fi
    
    # Execute script (output redirected to avoid clutter)
    # The script will generate JSON log files in the scripts directory
    if bash "$script_file" < /dev/null >> "$LOG_FILE" 2>&1; then
        # Check if execution log was generated
        generated_log="$(dirname "$script_file")/${basename}_execution_log.json"
        target_log="$LOGS_DIR/${basename}_execution_log.json"
        
        if [[ -f "$generated_log" ]]; then
            cp "$generated_log" "$target_log"
            ((EXECUTION_SUCCESS++))
            pass "$basename.sh"
        else
            ((EXECUTION_FAILED++))
            fail "$basename.sh (no execution log generated)"
        fi
    else
        # Script failed, but check if log was still generated
        generated_log="$(dirname "$script_file")/${basename}_execution_log.json"
        target_log="$LOGS_DIR/${basename}_execution_log.json"
        
        if [[ -f "$generated_log" ]]; then
            cp "$generated_log" "$target_log"
            ((EXECUTION_SUCCESS++))
            pass "$basename.sh (failed but log captured)"
        else
            ((EXECUTION_FAILED++))
            fail "$basename.sh"
        fi
    fi
done

echo ""
echo_info "Test Execution: $EXECUTION_SUCCESS logs captured, $EXECUTION_FAILED failed, $EXECUTION_SKIPPED skipped"
echo ""

# Stage 3: Run TPDG conversion
echo_section "Stage 3: Running TPDG Conversion"

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

# Count execution logs
if [[ -d "$LOGS_DIR" ]]; then
    LOG_COUNT=$(find "$LOGS_DIR" -name "*_execution_log.json" -type f 2>/dev/null | wc -l)
    echo_info "Execution logs generated: $LOG_COUNT"
    
    {
        echo "  Execution logs generated: $LOG_COUNT"
    } >> "$LOG_FILE"
fi

# Log script end
{
    echo ""
    echo "========================================="
    echo "Summary:"
    echo "  Stage 1 - Script Generation:"
    echo "    Success: $GENERATION_SUCCESS"
    echo "    Failed: $GENERATION_FAILED"
    echo "    Skipped: $GENERATION_SKIPPED"
    echo "  Stage 2 - Test Execution:"
    echo "    Success: $EXECUTION_SUCCESS"
    echo "    Failed: $EXECUTION_FAILED"
    echo "    Skipped: $EXECUTION_SKIPPED"
    echo "  Stage 3 - TPDG Conversion:"
    echo "    Exit Code: $CONVERSION_EXIT_CODE"
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
echo_section "Execution Complete"
if [[ $CONVERSION_EXIT_CODE -eq 0 ]]; then
    echo_success "All stages completed successfully!"
else
    echo_error "Script completed with errors!"
fi

exit $CONVERSION_EXIT_CODE
