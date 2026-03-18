#!/usr/bin/env bash
set -e

# Get script directory and source logger
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

# Parse command line arguments
OUTPUT_FILE=""
VERBOSE=0
FORMAT="text"

while [[ $# -gt 0 ]]; do
    case $1 in
        --output)
            OUTPUT_FILE="$2"
            shift 2
            ;;
        --verbose)
            VERBOSE=1
            shift
            ;;
        --format)
            FORMAT="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo ""
            echo "Compute lines of code statistics for the project"
            echo ""
            echo "Options:"
            echo "  --output FILE   Save output to file"
            echo "  --verbose       Show verbose output"
            echo "  --format FORMAT Output format (text, json, yaml)"
            echo "  --help          Show this help message"
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

section "Lines of Code Statistics"

# Check if tokei is installed (tokei is the actual name of the loc tool)
LOC_CMD=""
if command -v tokei >/dev/null 2>&1; then
    LOC_CMD="tokei"
elif command -v loc >/dev/null 2>&1; then
    LOC_CMD="loc"
else
    log_error "tokei/loc is not installed"
    log_error "Run 'make install-loc' to install it"
    exit 1
fi

# Get repo root (go up one directory from scripts)
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

log_info "Repository: $REPO_ROOT"
log_info "Tool: $LOC_CMD ($($LOC_CMD --version 2>&1 | head -n 1))"
echo ""

# Prepare output options
OUTPUT_ARGS=()
if [ "$FORMAT" = "json" ]; then
    OUTPUT_ARGS+=("--output" "json")
elif [ "$FORMAT" = "yaml" ]; then
    OUTPUT_ARGS+=("--output" "yaml")
fi

# Run tokei with appropriate options
if [ "$VERBOSE" -eq 1 ]; then
    OUTPUT_ARGS+=("--verbose")
fi

# Create temporary file for output if needed
TEMP_OUTPUT=""
if [ -n "$OUTPUT_FILE" ]; then
    TEMP_OUTPUT=$(mktemp)
fi

# Compute overall statistics
section "Overall Project Statistics"
if [ -n "$TEMP_OUTPUT" ]; then
    $LOC_CMD "${OUTPUT_ARGS[@]}" "$REPO_ROOT" | tee "$TEMP_OUTPUT"
else
    $LOC_CMD "${OUTPUT_ARGS[@]}" "$REPO_ROOT"
fi
echo ""

# Compute Rust statistics
section "Rust Code Statistics"
if [ "$FORMAT" = "text" ]; then
    $LOC_CMD "${OUTPUT_ARGS[@]}" --types rust "$REPO_ROOT" || log_warning "No Rust files found"
    echo ""
fi

# Compute Python statistics
section "Python Code Statistics"
if [ "$FORMAT" = "text" ]; then
    $LOC_CMD "${OUTPUT_ARGS[@]}" --types python "$REPO_ROOT" || log_warning "No Python files found"
    echo ""
fi

# Compute Shell script statistics
section "Shell Script Statistics"
if [ "$FORMAT" = "text" ]; then
    $LOC_CMD "${OUTPUT_ARGS[@]}" --types sh "$REPO_ROOT" || log_warning "No Shell scripts found"
    echo ""
fi

# Compute Documentation statistics (Markdown)
section "Documentation Statistics (Markdown)"
if [ "$FORMAT" = "text" ]; then
    $LOC_CMD "${OUTPUT_ARGS[@]}" --types markdown "$REPO_ROOT" || log_warning "No Markdown files found"
    echo ""
fi

# Compute YAML statistics
section "YAML Files Statistics"
if [ "$FORMAT" = "text" ]; then
    $LOC_CMD "${OUTPUT_ARGS[@]}" --types yaml "$REPO_ROOT" || log_warning "No YAML files found"
    echo ""
fi

# Summary by language category
section "Summary by Language Category"
if [ "$FORMAT" = "text" ]; then
    echo ""
    log_info "Languages included:"
    echo "  • Rust (*.rs)"
    echo "  • Python (*.py)"
    echo "  • Shell Scripts (*.sh)"
    echo "  • Markdown Documentation (*.md)"
    echo "  • YAML Configuration (*.yml, *.yaml)"
    echo ""
    
    # Create a summary using tokei's sort feature
    log_info "Top languages by lines of code:"
    $LOC_CMD --sort code --types rust,python,sh,markdown,yaml "$REPO_ROOT" || true
fi

# Save output if requested
if [ -n "$OUTPUT_FILE" ] && [ -n "$TEMP_OUTPUT" ]; then
    mv "$TEMP_OUTPUT" "$OUTPUT_FILE"
    pass "Statistics saved to: $OUTPUT_FILE"
fi

# Cleanup temp file if it still exists
if [ -n "$TEMP_OUTPUT" ] && [ -f "$TEMP_OUTPUT" ]; then
    rm -f "$TEMP_OUTPUT"
fi

pass "Lines of code statistics computed successfully"
