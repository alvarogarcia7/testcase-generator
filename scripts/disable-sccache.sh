#!/usr/bin/env bash
#
# disable-sccache.sh - Disable sccache for the current shell session
#
# This script helps unset RUSTC_WRAPPER=sccache for the current shell session.
# Use this if you encounter compilation issues with sccache.
#
# Usage:
#   source ./scripts/disable-sccache.sh
#   OR
#   . ./scripts/disable-sccache.sh

# NOTE: Do NOT use 'set -e' in a sourced script as it will exit the shell on errors

# Get the script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source logger library
if [ -f "$SCRIPT_DIR/lib/logger.sh" ]; then
    source "$SCRIPT_DIR/lib/logger.sh"
else
    # Fallback if logger is not available
    log_error() { echo "[ERROR] $*" >&2; }
    log_warning() { echo "[WARNING] $*" >&2; }
    log_info() { echo "[INFO] $*"; }
    pass() { echo "[✓] $*"; }
    section() { echo ""; echo "=== $* ==="; echo ""; }
fi

# Check if script is being sourced
if [ "${BASH_SOURCE[0]}" = "${0}" ]; then
    log_error "This script must be sourced, not executed"
    log_info "Run: source $0"
    log_info "Or: . $0"
    return 1
fi

section "Disabling sccache for Rust compilation"

# Check current state
if [ -z "$RUSTC_WRAPPER" ]; then
    log_info "RUSTC_WRAPPER is not set (sccache is already disabled)"
elif [ "$RUSTC_WRAPPER" = "sccache" ]; then
    unset RUSTC_WRAPPER
    pass "RUSTC_WRAPPER unset (sccache disabled for current session)"
else
    log_warning "RUSTC_WRAPPER was set to '$RUSTC_WRAPPER' (now unset)"
    unset RUSTC_WRAPPER
fi

echo
log_info "sccache is now disabled for this session"
log_info ""
log_info "To re-enable sccache:"
log_info "  source ./scripts/enable-sccache.sh"
