#!/usr/bin/env bash
#
# coverage.sh - Wrapper for cargo llvm-cov with custom thresholds and output formats
#
# DESCRIPTION:
#   This script wraps cargo llvm-cov commands, providing flexible configuration of coverage
#   thresholds, output formats, and user-friendly error messages when coverage requirements
#   are not met. It supports multiple threshold types (lines, functions, regions) and various
#   output formats (text, html, lcov, json).
#
# USAGE:
#   coverage.sh [OPTIONS]
#
# OPTIONS:
#   --min-lines PERCENT       Minimum line coverage percentage (default: 70)
#   --min-functions PERCENT   Minimum function coverage percentage (default: none)
#   --min-regions PERCENT     Minimum region coverage percentage (default: none)
#   --format FORMAT           Output format: text, html, lcov, json (default: text)
#   --output PATH             Output file path (for lcov/json formats)
#   --open                    Open HTML report in browser (only with --format html)
#   --workspace               Run coverage for entire workspace (default: true)
#   --all-features            Enable all features during coverage (default: true)
#   --verbose                 Enable verbose output
#   -h, --help                Show this help message
#
# EXAMPLES:
#   # Run with default 70% line coverage threshold
#   coverage.sh
#
#   # Custom line coverage threshold
#   coverage.sh --min-lines 80
#
#   # Multiple thresholds
#   coverage.sh --min-lines 70 --min-functions 60 --min-regions 65
#
#   # Generate HTML report and open in browser
#   coverage.sh --format html --open
#
#   # Generate LCOV report
#   coverage.sh --format lcov --output target/llvm-cov/lcov.info
#
#   # Generate JSON report with custom thresholds
#   coverage.sh --format json --output coverage.json --min-lines 80
#
# EXIT CODES:
#   0 - All coverage thresholds met
#   1 - One or more coverage thresholds not met or script error
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source logger library
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

# Default values
MIN_LINES=""
MIN_FUNCTIONS=""
MIN_REGIONS=""
OUTPUT_FORMAT="text"
OUTPUT_PATH=""
OPEN_BROWSER=0
USE_WORKSPACE=1
USE_ALL_FEATURES=1
VERBOSE=0

usage() {
    cat << 'EOF'
Usage: coverage.sh [OPTIONS]

Run cargo llvm-cov with custom coverage thresholds and output formats.

OPTIONS:
    --min-lines PERCENT       Minimum line coverage percentage (default: 70)
    --min-functions PERCENT   Minimum function coverage percentage
    --min-regions PERCENT     Minimum region coverage percentage
    --format FORMAT           Output format: text, html, lcov, json (default: text)
    --output PATH             Output file path (for lcov/json formats)
    --open                    Open HTML report in browser (only with --format html)
    --workspace               Run coverage for entire workspace (default: true)
    --all-features            Enable all features during coverage (default: true)
    --no-workspace            Disable workspace mode
    --no-all-features         Disable all features
    --verbose                 Enable verbose output
    -h, --help                Show this help message

EXAMPLES:
    coverage.sh
    coverage.sh --min-lines 80
    coverage.sh --min-lines 70 --min-functions 60 --min-regions 65
    coverage.sh --format html --open
    coverage.sh --format lcov --output target/llvm-cov/lcov.info
    coverage.sh --format json --output coverage.json --min-lines 80

EOF
    exit 0
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --min-lines)
            MIN_LINES="$2"
            shift 2
            ;;
        --min-functions)
            MIN_FUNCTIONS="$2"
            shift 2
            ;;
        --min-regions)
            MIN_REGIONS="$2"
            shift 2
            ;;
        --format)
            OUTPUT_FORMAT="$2"
            shift 2
            ;;
        --output)
            OUTPUT_PATH="$2"
            shift 2
            ;;
        --open)
            OPEN_BROWSER=1
            shift
            ;;
        --workspace)
            USE_WORKSPACE=1
            shift
            ;;
        --no-workspace)
            USE_WORKSPACE=0
            shift
            ;;
        --all-features)
            USE_ALL_FEATURES=1
            shift
            ;;
        --no-all-features)
            USE_ALL_FEATURES=0
            shift
            ;;
        --verbose)
            VERBOSE=1
            shift
            ;;
        -h|--help)
            usage
            ;;
        *)
            log_error "Unknown option: $1"
            echo "Use --help for usage information" >&2
            exit 1
            ;;
    esac
done

# Validate output format
case "$OUTPUT_FORMAT" in
    text|html|lcov|json)
        ;;
    *)
        log_error "Invalid output format: $OUTPUT_FORMAT"
        log_error "Supported formats: text, html, lcov, json"
        exit 1
        ;;
esac

# Validate --open is only used with html format
if [[ $OPEN_BROWSER -eq 1 && "$OUTPUT_FORMAT" != "html" ]]; then
    log_error "--open can only be used with --format html"
    exit 1
fi

# Validate --output is provided for lcov and json formats
if [[ "$OUTPUT_FORMAT" == "lcov" || "$OUTPUT_FORMAT" == "json" ]] && [[ -z "$OUTPUT_PATH" ]]; then
    log_error "--output is required for --format $OUTPUT_FORMAT"
    exit 1
fi

# Check if cargo-llvm-cov is installed
if ! command -v cargo-llvm-cov >/dev/null 2>&1; then
    log_error "cargo-llvm-cov is not installed"
    log_error "Install it with: cargo install cargo-llvm-cov"
    exit 1
fi

# Validate threshold values are numbers between 0 and 100
validate_threshold() {
    local name="$1"
    local value="$2"
    
    if [[ -z "$value" ]]; then
        return 0
    fi
    
    # Check if value is a number
    if ! [[ "$value" =~ ^[0-9]+(\.[0-9]+)?$ ]]; then
        log_error "$name must be a number, got: $value"
        exit 1
    fi
    
    # Check if value is between 0 and 100
    if (( $(echo "$value < 0" | bc -l 2>/dev/null || echo 0) )) || (( $(echo "$value > 100" | bc -l 2>/dev/null || echo 0) )); then
        log_error "$name must be between 0 and 100, got: $value"
        exit 1
    fi
}

# For BSD compatibility, use awk instead of bc if bc is not available
compare_numbers() {
    local num1="$1"
    local op="$2"
    local num2="$3"
    
    if command -v bc >/dev/null 2>&1; then
        case "$op" in
            "<")
                result=$(echo "$num1 < $num2" | bc -l)
                ;;
            "<=")
                result=$(echo "$num1 <= $num2" | bc -l)
                ;;
            ">")
                result=$(echo "$num1 > $num2" | bc -l)
                ;;
            ">=")
                result=$(echo "$num1 >= $num2" | bc -l)
                ;;
            *)
                log_error "Invalid operator: $op"
                exit 1
                ;;
        esac
        [[ "$result" -eq 1 ]]
    else
        awk -v n1="$num1" -v op="$op" -v n2="$num2" 'BEGIN {
            if (op == "<") exit !(n1 < n2);
            if (op == "<=") exit !(n1 <= n2);
            if (op == ">") exit !(n1 > n2);
            if (op == ">=") exit !(n1 >= n2);
            exit 1;
        }'
    fi
}

validate_threshold "--min-lines" "$MIN_LINES"
validate_threshold "--min-functions" "$MIN_FUNCTIONS"
validate_threshold "--min-regions" "$MIN_REGIONS"

# Build cargo-llvm-cov command
CARGO_CMD="cargo llvm-cov"

# Add workspace flag
if [[ $USE_WORKSPACE -eq 1 ]]; then
    CARGO_CMD="$CARGO_CMD --workspace"
fi

# Add all-features flag
if [[ $USE_ALL_FEATURES -eq 1 ]]; then
    CARGO_CMD="$CARGO_CMD --all-features"
fi

# Add format-specific options
case "$OUTPUT_FORMAT" in
    text)
        # Text format uses fail-under-* flags
        if [[ -n "$MIN_LINES" ]]; then
            CARGO_CMD="$CARGO_CMD --fail-under-lines $MIN_LINES"
        fi
        if [[ -n "$MIN_FUNCTIONS" ]]; then
            CARGO_CMD="$CARGO_CMD --fail-under-functions $MIN_FUNCTIONS"
        fi
        if [[ -n "$MIN_REGIONS" ]]; then
            CARGO_CMD="$CARGO_CMD --fail-under-regions $MIN_REGIONS"
        fi
        ;;
    html)
        CARGO_CMD="$CARGO_CMD --html"
        if [[ $OPEN_BROWSER -eq 1 ]]; then
            CARGO_CMD="$CARGO_CMD --open"
        fi
        ;;
    lcov)
        CARGO_CMD="$CARGO_CMD --lcov --output-path $OUTPUT_PATH"
        ;;
    json)
        CARGO_CMD="$CARGO_CMD --json --output-path $OUTPUT_PATH"
        ;;
esac

# Log the command
log_verbose "Executing: $CARGO_CMD"

# Create a temporary file to capture the output
TEMP_OUTPUT=$(mktemp)
trap 'rm -f "$TEMP_OUTPUT"' EXIT

# Execute the command
EXIT_CODE=0
if [[ "$OUTPUT_FORMAT" == "text" ]]; then
    # For text format, capture output and handle thresholds directly via cargo
    eval "$CARGO_CMD" 2>&1 | tee "$TEMP_OUTPUT" || EXIT_CODE=$?
    
    if [[ $EXIT_CODE -ne 0 ]]; then
        echo ""
        log_error "Coverage thresholds not met!"
        echo ""
        
        # Parse the output to provide detailed failure messages
        if [[ -n "$MIN_LINES" ]]; then
            actual_lines=$(grep -E "^Lines:" "$TEMP_OUTPUT" | awk '{print $2}' | sed 's/%//' || echo "")
            if [[ -n "$actual_lines" ]] && compare_numbers "$actual_lines" "<" "$MIN_LINES"; then
                log_error "Line coverage: ${actual_lines}% (required: ${MIN_LINES}%)"
                log_error "  → ${actual_lines}% of lines are covered"
                log_error "  → Need $(awk -v req="$MIN_LINES" -v act="$actual_lines" 'BEGIN {printf "%.2f", req - act}')% more line coverage"
            fi
        fi
        
        if [[ -n "$MIN_FUNCTIONS" ]]; then
            actual_functions=$(grep -E "^Functions:" "$TEMP_OUTPUT" | awk '{print $2}' | sed 's/%//' || echo "")
            if [[ -n "$actual_functions" ]] && compare_numbers "$actual_functions" "<" "$MIN_FUNCTIONS"; then
                log_error "Function coverage: ${actual_functions}% (required: ${MIN_FUNCTIONS}%)"
                log_error "  → ${actual_functions}% of functions are covered"
                log_error "  → Need $(awk -v req="$MIN_FUNCTIONS" -v act="$actual_functions" 'BEGIN {printf "%.2f", req - act}')% more function coverage"
            fi
        fi
        
        if [[ -n "$MIN_REGIONS" ]]; then
            actual_regions=$(grep -E "^Regions:" "$TEMP_OUTPUT" | awk '{print $2}' | sed 's/%//' || echo "")
            if [[ -n "$actual_regions" ]] && compare_numbers "$actual_regions" "<" "$MIN_REGIONS"; then
                log_error "Region coverage: ${actual_regions}% (required: ${MIN_REGIONS}%)"
                log_error "  → ${actual_regions}% of regions are covered"
                log_error "  → Need $(awk -v req="$MIN_REGIONS" -v act="$actual_regions" 'BEGIN {printf "%.2f", req - act}')% more region coverage"
            fi
        fi
        
        echo ""
        log_error "To improve coverage:"
        log_error "  1. Run: coverage.sh --format html --open"
        log_error "  2. Review uncovered lines in the HTML report"
        log_error "  3. Add tests for uncovered code paths"
        echo ""
        
        exit 1
    fi
else
    # For other formats, run the command and check thresholds manually if needed
    eval "$CARGO_CMD" 2>&1 | tee "$TEMP_OUTPUT" || EXIT_CODE=$?
    
    if [[ $EXIT_CODE -ne 0 ]]; then
        log_error "Coverage command failed with exit code: $EXIT_CODE"
        exit 1
    fi
    
    # For non-text formats, we need to parse the report output to check thresholds
    if [[ -n "$MIN_LINES" || -n "$MIN_FUNCTIONS" || -n "$MIN_REGIONS" ]]; then
        log_info "Checking coverage thresholds..."
        
        # Generate a text report to extract coverage percentages
        REPORT_OUTPUT=$(mktemp)
        trap 'rm -f "$TEMP_OUTPUT" "$REPORT_OUTPUT"' EXIT
        
        REPORT_CMD="cargo llvm-cov report"
        if [[ $USE_WORKSPACE -eq 1 ]]; then
            REPORT_CMD="$REPORT_CMD --workspace"
        fi
        if [[ $USE_ALL_FEATURES -eq 1 ]]; then
            REPORT_CMD="$REPORT_CMD --all-features"
        fi
        
        eval "$REPORT_CMD" 2>&1 > "$REPORT_OUTPUT" || true
        
        THRESHOLD_FAILED=0
        
        if [[ -n "$MIN_LINES" ]]; then
            actual_lines=$(grep -E "^TOTAL" "$REPORT_OUTPUT" | awk '{print $(NF-2)}' | sed 's/%//' || echo "")
            if [[ -n "$actual_lines" ]] && compare_numbers "$actual_lines" "<" "$MIN_LINES"; then
                log_error "Line coverage: ${actual_lines}% (required: ${MIN_LINES}%)"
                log_error "  → ${actual_lines}% of lines are covered"
                log_error "  → Need $(awk -v req="$MIN_LINES" -v act="$actual_lines" 'BEGIN {printf "%.2f", req - act}')% more line coverage"
                THRESHOLD_FAILED=1
            fi
        fi
        
        if [[ -n "$MIN_FUNCTIONS" ]]; then
            actual_functions=$(grep -E "^TOTAL" "$REPORT_OUTPUT" | awk '{print $(NF-1)}' | sed 's/%//' || echo "")
            if [[ -n "$actual_functions" ]] && compare_numbers "$actual_functions" "<" "$MIN_FUNCTIONS"; then
                log_error "Function coverage: ${actual_functions}% (required: ${MIN_FUNCTIONS}%)"
                log_error "  → ${actual_functions}% of functions are covered"
                log_error "  → Need $(awk -v req="$MIN_FUNCTIONS" -v act="$actual_functions" 'BEGIN {printf "%.2f", req - act}')% more function coverage"
                THRESHOLD_FAILED=1
            fi
        fi
        
        if [[ -n "$MIN_REGIONS" ]]; then
            actual_regions=$(grep -E "^TOTAL" "$REPORT_OUTPUT" | awk '{print $NF}' | sed 's/%//' || echo "")
            if [[ -n "$actual_regions" ]] && compare_numbers "$actual_regions" "<" "$MIN_REGIONS"; then
                log_error "Region coverage: ${actual_regions}% (required: ${MIN_REGIONS}%)"
                log_error "  → ${actual_regions}% of regions are covered"
                log_error "  → Need $(awk -v req="$MIN_REGIONS" -v act="$actual_regions" 'BEGIN {printf "%.2f", req - act}')% more region coverage"
                THRESHOLD_FAILED=1
            fi
        fi
        
        if [[ $THRESHOLD_FAILED -eq 1 ]]; then
            echo ""
            log_error "Coverage thresholds not met!"
            echo ""
            log_error "To improve coverage:"
            log_error "  1. Run: coverage.sh --format html --open"
            log_error "  2. Review uncovered lines in the HTML report"
            log_error "  3. Add tests for uncovered code paths"
            echo ""
            exit 1
        fi
        
        log_info "All coverage thresholds met!"
    fi
    
    # Report output location for non-text formats
    case "$OUTPUT_FORMAT" in
        html)
            echo ""
            if [[ $OPEN_BROWSER -eq 0 ]]; then
                log_info "HTML coverage report generated at: target/llvm-cov/html/index.html"
            fi
            ;;
        lcov)
            echo ""
            log_info "LCOV coverage report generated at: $OUTPUT_PATH"
            ;;
        json)
            echo ""
            log_info "JSON coverage report generated at: $OUTPUT_PATH"
            ;;
    esac
fi

echo ""
log_info "Coverage analysis complete!"
exit 0
