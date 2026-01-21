#!/usr/bin/env bash
#
# validate-files.sh - Generic file validation with caching and watch mode
#
# DESCRIPTION:
#   This script provides a generic framework for validating files matching a regex pattern
#   using a custom validator script. It implements a two-layer caching system (mtime and
#   content hash) to avoid redundant validations and improve performance. It also supports
#   watch mode for continuous monitoring and automatic validation of file changes.
#
# USAGE:
#   validate-files.sh --pattern PATTERN --validator SCRIPT [OPTIONS]
#
# REQUIRED ARGUMENTS:
#   --pattern PATTERN       Regex pattern to match files (POSIX extended regex)
#   --validator SCRIPT      Path to validation script that accepts a file path as argument
#
# OPTIONAL ARGUMENTS:
#   --cache-dir DIR        Cache directory for validation results (default: .validation-cache)
#   --verbose              Enable verbose output for debugging
#   --watch [DIR]          Enable watch mode to monitor directory for changes (default: testcases/)
#   -h, --help             Show help message and exit
#
# VALIDATOR SCRIPT REQUIREMENTS:
#   The validator script must:
#   - Accept a single argument: the file path to validate
#   - Exit with code 0 on successful validation
#   - Exit with non-zero code on validation failure
#   - Be executable (chmod +x)
#
# CACHING BEHAVIOR:
#   The script uses a two-layer caching strategy:
#   1. Layer 1 (fast): Check modification time (mtime) - if unchanged, use cached result
#   2. Layer 2 (thorough): Check SHA256 hash - if unchanged despite mtime change, use cached result
#   
#   Cache entries are stored as JSON files in the cache directory, containing:
#   - File path, mtime, content hash, validation result, and timestamp
#
# EXAMPLES:
#   # Validate all Rust files with a custom validator
#   validate-files.sh --pattern '\.rs$' --validator ./scripts/rust-validator.sh
#
#   # Validate JSON files with verbose output and custom cache directory
#   validate-files.sh --pattern '\.json$' --validator ./validate-json.sh \
#       --cache-dir /tmp/validation-cache --verbose
#
#   # Validate YAML files using a wrapper script that calls validate-yaml binary
#   validate-files.sh --pattern '\.ya?ml$' --validator ./scripts/validate-yaml-wrapper.sh
#
#   # Watch mode: continuously monitor testcases/ directory for YAML file changes
#   validate-files.sh --pattern '\.ya?ml$' --validator ./scripts/validate-yaml-wrapper.sh --watch
#
#   # Watch mode with custom directory and verbose output
#   validate-files.sh --pattern '\.ya?ml$' --validator ./scripts/validate-yaml-wrapper.sh \
#       --watch custom-dir/ --verbose
#
# WATCH MODE:
#   Watch mode continuously monitors a directory for file changes and automatically triggers
#   validation on modified files. It requires platform-specific tools:
#   - Linux: inotify-tools (install with: sudo apt-get install inotify-tools)
#   - macOS: fswatch (install with: brew install fswatch)
#   
#   Features:
#   - Runs initial full validation on startup
#   - Monitors directory recursively for file modifications, creations, deletions, and moves
#   - Instantly validates changed files matching the pattern
#   - Displays live validation results with color-coded output
#   - Maintains persistent cache across watch sessions for fast re-validation
#   - Cleans up cache entries for deleted files
#
# EXIT CODES:
#   0 - All validations passed (normal mode only)
#   1 - One or more validations failed or script error occurred
#
# INTEGRATION:
#   This script is designed to work with any validation tool. See scripts/validate-yaml-wrapper.sh
#   for an example of integrating with the validate-yaml binary that validates YAML files
#   against JSON schemas.
#

set -euo pipefail

PATTERN=""
VALIDATOR=""
CACHE_DIR=".validation-cache"
VERBOSE=0
WATCH_MODE=0
WATCH_DIR=""

usage() {
    cat << EOF
Usage: $(basename "$0") [OPTIONS]

Validate files matching a regex pattern using a custom validator script.

OPTIONS:
    --pattern PATTERN       Regex pattern to match files (required)
    --validator SCRIPT      Path to validation script (required)
    --cache-dir DIR        Cache directory for validation results (default: .validation-cache)
    --verbose              Enable verbose output
    --watch [DIR]          Enable watch mode to monitor directory for changes (default: testcases/)
    -h, --help             Show this help message

EXAMPLES:
    $(basename "$0") --pattern '\.rs$' --validator ./scripts/rust-validator.sh
    $(basename "$0") --pattern '\.json$' --validator ./validate.sh --cache-dir /tmp/cache --verbose
    $(basename "$0") --pattern '\.ya?ml$' --validator ./scripts/validate-yaml-wrapper.sh --watch

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

# Get mtime of a file in seconds since epoch
get_mtime() {
    local file="$1"
    if [[ "$OSTYPE" == "darwin"* ]]; then
        stat -f %m "$file"
    else
        stat -c %Y "$file"
    fi
}

# Get SHA256 hash of a file
get_hash() {
    local file="$1"
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$file" | awk '{print $1}'
    elif command -v shasum >/dev/null 2>&1; then
        shasum -a 256 "$file" | awk '{print $1}'
    else
        log_error "No SHA256 utility found (sha256sum or shasum)"
        exit 1
    fi
}

# Get cache file path for a given file
get_cache_file() {
    local file="$1"
    local hash_input="${file}"
    local cache_hash
    
    if command -v sha256sum >/dev/null 2>&1; then
        cache_hash=$(echo -n "$hash_input" | sha256sum | awk '{print $1}')
    elif command -v shasum >/dev/null 2>&1; then
        cache_hash=$(echo -n "$hash_input" | shasum -a 256 | awk '{print $1}')
    else
        log_error "No SHA256 utility found (sha256sum or shasum)"
        exit 1
    fi
    
    echo "${CACHE_DIR}/${cache_hash}.json"
}

# Read cache entry for a file
read_cache() {
    local cache_file="$1"
    
    if [[ ! -f "$cache_file" ]]; then
        echo ""
        return
    fi
    
    cat "$cache_file"
}

# Parse JSON field from cache entry
parse_json_field() {
    local json="$1"
    local field="$2"
    
    # Simple JSON parsing for our specific format
    echo "$json" | grep -o "\"$field\":[^,}]*" | sed 's/^"[^"]*":\s*"\?\([^"]*\)"\?$/\1/' | sed 's/"$//' | cut -d: -f2 | sed 's/^"//' | sed 's/^ *//'
}

# Write cache entry for a file
write_cache() {
    local file="$1"
    local mtime="$2"
    local hash="$3"
    local valid="$4"
    local cache_file="$5"
    local timestamp
    timestamp=$(date +%s)
    
    local json_content
    json_content=$(cat <<EOF
{
  "path": "$file",
  "mtime": $mtime,
  "hash": "$hash",
  "valid": $valid,
  "timestamp": $timestamp
}
EOF
)
    
    echo "$json_content" > "$cache_file"
    log_verbose "Cache written: $cache_file"
}

# Check if file validation can be skipped based on cache
check_cache() {
    local file="$1"
    local cache_file
    cache_file=$(get_cache_file "$file")
    
    log_verbose "Checking cache for: $file"
    log_verbose "Cache file: $cache_file"
    
    local cache_content
    cache_content=$(read_cache "$cache_file")
    
    if [[ -z "$cache_content" ]]; then
        log_verbose "No cache entry found"
        echo "validate"
        return
    fi
    
    # Extract cached values
    local cached_mtime
    local cached_hash
    local cached_valid
    cached_mtime=$(parse_json_field "$cache_content" "mtime")
    cached_hash=$(parse_json_field "$cache_content" "hash")
    cached_valid=$(parse_json_field "$cache_content" "valid")
    
    log_verbose "Cached mtime: $cached_mtime"
    log_verbose "Cached hash: $cached_hash"
    log_verbose "Cached valid: $cached_valid"
    
    # Layer 1: Check mtime (fast path)
    local current_mtime
    current_mtime=$(get_mtime "$file")
    log_verbose "Current mtime: $current_mtime"
    
    if [[ "$current_mtime" == "$cached_mtime" ]]; then
        log_verbose "Mtime unchanged, using cached result"
        if [[ "$cached_valid" == "true" ]]; then
            echo "cached_valid"
        else
            echo "cached_invalid"
        fi
        return
    fi
    
    log_verbose "Mtime changed, checking hash"
    
    # Layer 2: Check hash (content-based)
    local current_hash
    current_hash=$(get_hash "$file")
    log_verbose "Current hash: $current_hash"
    
    if [[ "$current_hash" == "$cached_hash" ]]; then
        log_verbose "Hash unchanged, updating mtime in cache"
        # Content unchanged, update cache with new mtime
        write_cache "$file" "$current_mtime" "$current_hash" "$cached_valid" "$cache_file"
        if [[ "$cached_valid" == "true" ]]; then
            echo "cached_valid"
        else
            echo "cached_invalid"
        fi
        return
    fi
    
    log_verbose "Hash changed, validation required"
    echo "validate"
}

# Update cache after validation
update_cache() {
    local file="$1"
    local valid="$2"
    local cache_file
    cache_file=$(get_cache_file "$file")
    
    local mtime
    local hash
    mtime=$(get_mtime "$file")
    hash=$(get_hash "$file")
    
    write_cache "$file" "$mtime" "$hash" "$valid" "$cache_file"
}

# Validate a single file and return result
validate_single_file() {
    local file="$1"
    local show_output="${2:-1}"
    
    if [[ $show_output -eq 1 ]]; then
        log_verbose "Processing: $file"
    fi
    
    # Check if validation can be skipped based on cache
    cache_result=$(check_cache "$file")
    
    local result=""
    case "$cache_result" in
        cached_valid)
            if [[ $show_output -eq 1 ]]; then
                log_verbose "✓ Cached (valid): $file"
            fi
            result="passed"
            ;;
        cached_invalid)
            if [[ $show_output -eq 1 ]]; then
                log_verbose "✗ Cached (invalid): $file"
            fi
            result="failed"
            ;;
        validate)
            if [[ $show_output -eq 1 ]]; then
                log_verbose "Validating: $file"
            fi
            
            # Invoke validator script and capture exit code
            EXIT_CODE=0
            "$VALIDATOR" "$file" 2>&1 || EXIT_CODE=$?
            
            if [[ $EXIT_CODE -eq 0 ]]; then
                if [[ $show_output -eq 1 ]]; then
                    log_verbose "✓ Passed: $file"
                fi
                update_cache "$file" "true"
                result="passed"
            else
                if [[ $show_output -eq 1 ]]; then
                    log_error "✗ Failed: $file (exit code: $EXIT_CODE)"
                fi
                update_cache "$file" "false"
                result="failed"
            fi
            ;;
    esac
    
    echo "$result"
}

# Run validation on all matching files
run_validation() {
    local show_summary="${1:-1}"
    
    if [[ $show_summary -eq 1 ]]; then
        log_info "Searching for files matching pattern: $PATTERN"
    fi
    
    FILES=()
    while IFS= read -r -d '' file; do
        FILES+=("$file")
    done < <(find -E . -type f -regex ".*${PATTERN}.*" -print0 2>/dev/null)
    
    if [[ ${#FILES[@]} -eq 0 ]]; then
        if [[ $show_summary -eq 1 ]]; then
            log_info "No files found matching pattern: $PATTERN"
        fi
        return 0
    fi
    
    if [[ $show_summary -eq 1 ]]; then
        log_info "Found ${#FILES[@]} file(s) matching pattern"
    fi
    
    # Initialize statistics tracking
    TOTAL_FILES=${#FILES[@]}
    FAILED_FILES=()
    PASSED_COUNT=0
    FAILED_COUNT=0
    
    # Process each file
    for file in "${FILES[@]}"; do
        result=$(validate_single_file "$file" "$VERBOSE")
        
        if [[ "$result" == "passed" ]]; then
            ((PASSED_COUNT++))
        else
            FAILED_FILES+=("$file")
            ((FAILED_COUNT++))
        fi
    done
    
    # Report statistics if requested
    if [[ $show_summary -eq 1 ]]; then
        echo ""
        log_info "=== Validation Summary ==="
        log_info "Total files:     $TOTAL_FILES"
        log_info "Passed:          $PASSED_COUNT"
        log_info "Failed:          $FAILED_COUNT"
        echo ""
        
        if [[ ${#FAILED_FILES[@]} -gt 0 ]]; then
            log_error "Failed files:"
            for file in "${FAILED_FILES[@]}"; do
                log_error "  - $file"
            done
            return 1
        fi
        
        log_info "All validations passed!"
    fi
    
    return 0
}

# Watch mode implementation
run_watch_mode() {
    log_info "Starting watch mode on directory: $WATCH_DIR"
    log_info "Watching for file changes matching pattern: $PATTERN"
    log_info "Press Ctrl+C to stop"
    echo ""
    
    # Verify watch directory exists
    if [[ ! -d "$WATCH_DIR" ]]; then
        log_error "Watch directory does not exist: $WATCH_DIR"
        exit 1
    fi
    
    # Determine OS and check for file watcher availability
    local watcher_cmd=""
    local watcher_available=0
    
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux: use inotifywait
        if command -v inotifywait >/dev/null 2>&1; then
            watcher_cmd="inotifywait"
            watcher_available=1
            log_info "Using inotifywait for file monitoring (Linux)"
        else
            log_error "inotifywait not found. Install with: sudo apt-get install inotify-tools"
            exit 1
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS: use fswatch
        if command -v fswatch >/dev/null 2>&1; then
            watcher_cmd="fswatch"
            watcher_available=1
            log_info "Using fswatch for file monitoring (macOS)"
        else
            log_error "fswatch not found. Install with: brew install fswatch"
            exit 1
        fi
    else
        log_error "Watch mode not supported on this operating system: $OSTYPE"
        exit 1
    fi
    
    # Run initial validation
    log_info "Running initial validation..."
    echo ""
    run_validation 1
    initial_status=$?
    
    echo ""
    log_info "Initial validation complete. Now monitoring for changes..."
    echo ""
    
    # Start watching for changes
    if [[ "$watcher_cmd" == "inotifywait" ]]; then
        # inotifywait (Linux)
        inotifywait -m -r -e modify,create,delete,move --format '%w%f' "$WATCH_DIR" 2>/dev/null | while read -r changed_file; do
            # Check if file matches pattern
            if echo "$changed_file" | grep -E "$PATTERN" >/dev/null 2>&1; then
                # Small delay to ensure file is fully written
                sleep 0.1
                
                # Check if file still exists (it might have been deleted)
                if [[ -f "$changed_file" ]]; then
                    echo ""
                    log_info "File changed: $changed_file"
                    log_info "Validating..."
                    
                    result=$(validate_single_file "$changed_file" 0)
                    
                    if [[ "$result" == "passed" ]]; then
                        echo -e "\033[32m✓ PASSED\033[0m: $changed_file"
                    else
                        echo -e "\033[31m✗ FAILED\033[0m: $changed_file"
                        # Run validator again to show error output
                        "$VALIDATOR" "$changed_file" 2>&1 || true
                    fi
                    echo ""
                else
                    log_info "File deleted: $changed_file"
                    # Remove from cache
                    cache_file=$(get_cache_file "$changed_file")
                    if [[ -f "$cache_file" ]]; then
                        rm -f "$cache_file"
                        log_verbose "Cache entry removed: $cache_file"
                    fi
                fi
            fi
        done
    elif [[ "$watcher_cmd" == "fswatch" ]]; then
        # fswatch (macOS)
        fswatch -0 -r -e ".*" -i "$PATTERN" "$WATCH_DIR" | while read -d "" changed_file; do
            # Small delay to ensure file is fully written
            sleep 0.1
            
            # Check if file still exists
            if [[ -f "$changed_file" ]]; then
                echo ""
                log_info "File changed: $changed_file"
                log_info "Validating..."
                
                result=$(validate_single_file "$changed_file" 0)
                
                if [[ "$result" == "passed" ]]; then
                    echo -e "\033[32m✓ PASSED\033[0m: $changed_file"
                else
                    echo -e "\033[31m✗ FAILED\033[0m: $changed_file"
                    # Run validator again to show error output
                    "$VALIDATOR" "$changed_file" 2>&1 || true
                fi
                echo ""
            else
                log_info "File deleted: $changed_file"
                # Remove from cache
                cache_file=$(get_cache_file "$changed_file")
                if [[ -f "$cache_file" ]]; then
                    rm -f "$cache_file"
                    log_verbose "Cache entry removed: $cache_file"
                fi
            fi
        done
    fi
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
        --watch)
            WATCH_MODE=1
            if [[ $# -gt 1 && ! "$2" =~ ^-- ]]; then
                WATCH_DIR="$2"
                shift 2
            else
                WATCH_DIR="testcases/"
                shift
            fi
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

if command "$VALIDATOR"; then
  log_info "Using command '$VALIDATOR' as validator"
elif [[ -f "$VALIDATOR" ]]; then
  log_info "Using file '$VALIDATOR' as validator"
  if [[ ! -x "$VALIDATOR" ]]; then
      log_error "Validator script is not executable: $VALIDATOR"
      exit 1
  fi
else
    log_error "Validator script not found: $VALIDATOR"
    exit 1
fi


log_verbose "Pattern: $PATTERN"
log_verbose "Validator: $VALIDATOR"
log_verbose "Cache directory: $CACHE_DIR"
log_verbose "Watch mode: $WATCH_MODE"
if [[ $WATCH_MODE -eq 1 ]]; then
    log_verbose "Watch directory: $WATCH_DIR"
fi

mkdir -p "$CACHE_DIR"
log_verbose "Cache directory created/verified: $CACHE_DIR"

# Main execution: run watch mode or normal validation
if [[ $WATCH_MODE -eq 1 ]]; then
    run_watch_mode
else
    run_validation 1
    exit $?
fi
