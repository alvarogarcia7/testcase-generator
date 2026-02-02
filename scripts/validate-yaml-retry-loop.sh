#!/usr/bin/env bash
#
# validate-yaml-retry-loop.sh - Retry loop for YAML validation
#
# DESCRIPTION:
#   This script validates YAML files with automatic retry logic. It uses nested loops:
#   - Outer loop: Iterates through YAML files from ls command output
#   - Inner loop: Retries validation until successful, with 5-second delays
#
# USAGE:
#   validate-yaml-retry-loop.sh <yaml-file-pattern>...
#
# ARGUMENTS:
#   yaml-file-pattern    Pattern(s) to pass to ls command (e.g., "*.yml", "data/*.yaml")
#                        If no arguments provided, defaults to "*.yml"
#
# REQUIREMENTS:
#   - validate-yaml-wrapper.sh must be in the same directory or in PATH
#   - SCHEMA_FILE environment variable should be set (or uses validate-yaml-wrapper.sh default)
#
# EXAMPLES:
#   # Validate all YAML files in current directory
#   ./scripts/validate-yaml-retry-loop.sh "*.yml"
#
#   # Validate YAML files in data directory
#   export SCHEMA_FILE=data/schema.json
#   ./scripts/validate-yaml-retry-loop.sh "data/*.yml" "data/*.yaml"
#
#   # Use default pattern (*.yml)
#   ./scripts/validate-yaml-retry-loop.sh
#
# RETRY BEHAVIOR:
#   For each YAML file found:
#   - Attempts validation using validate-yaml-wrapper.sh
#   - On failure, waits 5 seconds and retries indefinitely
#   - On success, moves to the next file
#   - Displays informative messages for each attempt
#
# EXIT CODES:
#   0 - All files validated successfully
#   1 - Error occurred (missing wrapper script, no files found, etc.)
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WRAPPER_SCRIPT="$SCRIPT_DIR/validate-yaml-wrapper.sh"
RETRY_DELAY=5

log_info() {
    echo "[INFO] $*"
}

log_error() {
    echo "[ERROR] $*" >&2
}

log_retry() {
    echo "[RETRY] $*"
}

log_success() {
    echo "[SUCCESS] $*"
}

if [[ ! -f "$WRAPPER_SCRIPT" ]]; then
    log_error "validate-yaml-wrapper.sh not found at: $WRAPPER_SCRIPT"
    exit 1
fi

if [[ ! -x "$WRAPPER_SCRIPT" ]]; then
    log_error "validate-yaml-wrapper.sh is not executable: $WRAPPER_SCRIPT"
    exit 1
fi

if [[ $# -eq 0 ]]; then
    set -- "*.yml"
    log_info "No arguments provided, using default pattern: *.yml"
fi

log_info "Starting YAML validation with retry logic"
log_info "Wrapper script: $WRAPPER_SCRIPT"
log_info "Retry delay: ${RETRY_DELAY} seconds"
echo ""

YAML_FILES=()
for pattern in "$@"; do
    log_info "Searching for files matching pattern: $pattern"
    
    while IFS= read -r file; do
        if [[ -f "$file" ]]; then
            YAML_FILES+=("$file")
        fi
    done < <(ls $pattern 2>/dev/null || true)
done

if [[ ${#YAML_FILES[@]} -eq 0 ]]; then
    log_error "No YAML files found matching the provided patterns"
    exit 1
fi

log_info "Found ${#YAML_FILES[@]} YAML file(s) to validate"
echo ""

TOTAL_FILES=${#YAML_FILES[@]}
CURRENT_FILE_NUM=0

for yaml_file in "${YAML_FILES[@]}"; do
    ((CURRENT_FILE_NUM++))
    
    log_info "[$CURRENT_FILE_NUM/$TOTAL_FILES] Processing: $yaml_file"
    
    ATTEMPT=0
    while true; do
        ((ATTEMPT++))
        
        if [[ $ATTEMPT -eq 1 ]]; then
            log_info "Validating (attempt $ATTEMPT): $yaml_file"
        else
            log_retry "Validating (attempt $ATTEMPT): $yaml_file"
        fi
        
        EXIT_CODE=0
        "$WRAPPER_SCRIPT" "$yaml_file" 2>&1 || EXIT_CODE=$?
        
        if [[ $EXIT_CODE -eq 0 ]]; then
            log_success "Validation passed: $yaml_file"
            echo ""
            break
        else
            log_error "Validation failed (attempt $ATTEMPT) for: $yaml_file (exit code: $EXIT_CODE)"
            log_info "Waiting ${RETRY_DELAY} seconds before retry..."
            sleep "$RETRY_DELAY"
        fi
    done
done

log_info "=== Validation Complete ==="
log_info "Total files validated: $TOTAL_FILES"
log_success "All files validated successfully!"
