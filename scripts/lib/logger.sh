#!/bin/bash
# Centralized logger library for bash scripts
# Provides standard logging functions and temporary file cleanup management

# Color codes for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Cleanup management
_CLEANUP_ENABLED=1
_TEMP_DIRS=()
_BACKGROUND_PIDS=()

# Standard logging functions
log_info() {
    echo "[INFO] $*" >&2
}

log_warning() {
    echo "[WARNING] $*" >&2
}

log_error() {
    echo "[ERROR] $*" >&2
}

log_debug() {
    if [[ ${VERBOSE:-0} -eq 1 ]]; then
        echo "[DEBUG] $*" >&2
    fi
}

log_verbose() {
    if [[ ${VERBOSE:-0} -eq 1 ]]; then
        echo "[VERBOSE] $*" >&2
    fi
}

# Color-coded test helpers
pass() {
    echo -e "${GREEN}✓${NC} $1" >&2
}

fail() {
    echo -e "${RED}✗${NC} $1" >&2
}

info() {
    echo -e "${BLUE}ℹ${NC} $1" >&2
}

section() {
    echo "" >&2
    echo -e "${YELLOW}=== $1 ===${NC}" >&2
}

# Cleanup management functions
setup_cleanup() {
    local temp_dir="$1"

    if [[ -z "$temp_dir" ]]; then
        log_error "setup_cleanup: temp_dir argument required"
        return 1
    fi

    _TEMP_DIRS+=("$temp_dir")

    # Register cleanup trap only once
    if [[ ${#_TEMP_DIRS[@]} -eq 1 ]]; then
        trap '_perform_cleanup' EXIT
    fi
}

register_background_pid() {
    local pid="$1"

    if [[ -z "$pid" ]]; then
        log_error "register_background_pid: pid argument required"
        return 1
    fi

    _BACKGROUND_PIDS+=("$pid")
}

disable_cleanup() {
    _CLEANUP_ENABLED=0
}

enable_cleanup() {
    _CLEANUP_ENABLED=1
}

# Internal cleanup function (called by trap)
_perform_cleanup() {
    if [[ $_CLEANUP_ENABLED -eq 0 ]]; then
        return 0
    fi

    # Clean up background processes
    for pid in "${_BACKGROUND_PIDS[@]}"; do
        if kill -0 "$pid" 2>/dev/null; then
            kill "$pid" 2>/dev/null || true
            wait "$pid" 2>/dev/null || true
        fi
    done

    # Clean up temporary directories
    for temp_dir in "${_TEMP_DIRS[@]}"; do
        if [[ -n "$temp_dir" ]] && [[ -d "$temp_dir" ]]; then
            rm -rf "$temp_dir"
        fi
    done
}
