#!/usr/bin/env bash

set -euo pipefail

PATTERN=""
VALIDATOR=""
CACHE_DIR=".validation-cache"
VERBOSE=0

usage() {
    cat << EOF
Usage: $(basename "$0") [OPTIONS]

Validate files matching a regex pattern using a custom validator script.

OPTIONS:
    --pattern PATTERN       Regex pattern to match files (required)
    --validator SCRIPT      Path to validation script (required)
    --cache-dir DIR        Cache directory for validation results (default: .validation-cache)
    --verbose              Enable verbose output
    -h, --help             Show this help message

EXAMPLES:
    $(basename "$0") --pattern '\.rs$' --validator ./scripts/rust-validator.sh
    $(basename "$0") --pattern '\.json$' --validator ./validate.sh --cache-dir /tmp/cache --verbose

EOF
    exit 0
}

log_verbose() {
    if [[ $VERBOSE -eq 1 ]]; then
        echo "[VERBOSE] $*" >&2
    fi
}

log_info() {
    echo "[INFO] $*" >&2
}

log_error() {
    echo "[ERROR] $*" >&2
}

while [[ $# -gt 0 ]]; do
    case $1 in
        --pattern)
            PATTERN="$2"
            shift 2
            ;;
        --validator)
            VALIDATOR="$2"
            shift 2
            ;;
        --cache-dir)
            CACHE_DIR="$2"
            shift 2
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

if [[ -z "$PATTERN" ]]; then
    log_error "Missing required option: --pattern"
    echo "Use --help for usage information" >&2
    exit 1
fi

if [[ -z "$VALIDATOR" ]]; then
    log_error "Missing required option: --validator"
    echo "Use --help for usage information" >&2
    exit 1
fi

if [[ ! -f "$VALIDATOR" ]]; then
    log_error "Validator script not found: $VALIDATOR"
    exit 1
fi

if [[ ! -x "$VALIDATOR" ]]; then
    log_error "Validator script is not executable: $VALIDATOR"
    exit 1
fi

log_verbose "Pattern: $PATTERN"
log_verbose "Validator: $VALIDATOR"
log_verbose "Cache directory: $CACHE_DIR"

mkdir -p "$CACHE_DIR"
log_verbose "Cache directory created/verified: $CACHE_DIR"

log_info "Searching for files matching pattern: $PATTERN"

FILES=()
while IFS= read -r -d '' file; do
    FILES+=("$file")
done < <(find . -type f -regextype posix-extended -regex ".*${PATTERN}.*" -print0 2>/dev/null)

if [[ ${#FILES[@]} -eq 0 ]]; then
    log_info "No files found matching pattern: $PATTERN"
    exit 0
fi

log_info "Found ${#FILES[@]} file(s) matching pattern"

FAILED_FILES=()
PASSED_COUNT=0

for file in "${FILES[@]}"; do
    log_verbose "Validating: $file"
    
    if "$VALIDATOR" "$file"; then
        log_verbose "✓ Passed: $file"
        ((PASSED_COUNT++))
    else
        log_error "✗ Failed: $file"
        FAILED_FILES+=("$file")
    fi
done

echo ""
log_info "Validation complete: $PASSED_COUNT passed, ${#FAILED_FILES[@]} failed"

if [[ ${#FAILED_FILES[@]} -gt 0 ]]; then
    log_error "Failed files:"
    for file in "${FAILED_FILES[@]}"; do
        log_error "  - $file"
    done
    exit 1
fi

exit 0
