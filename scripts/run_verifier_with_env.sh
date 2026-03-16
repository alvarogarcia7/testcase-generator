#!/usr/bin/env bash
#
# run_verifier_with_env.sh - Run verifier with environment-specific configuration
#
# This script demonstrates how to use environment-specific container configurations
# with the verifier binary. It automatically selects the appropriate config file
# based on the DEPLOY_ENV environment variable.
#
# Usage: 
#   DEPLOY_ENV=staging ./scripts/run_verifier_with_env.sh [VERIFIER_ARGS...]
#   DEPLOY_ENV=prod ./scripts/run_verifier_with_env.sh --folder logs/ --format yaml
#
# Environment Variables:
#   DEPLOY_ENV - Environment name (dev, staging, prod). Default: dev
#   BUILD_VARIANT - Cargo build variant (--debug or --release). Default: --release
#

set -e

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source logger library
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

# Determine environment
DEPLOY_ENV="${DEPLOY_ENV:-dev}"
BUILD_VARIANT="${BUILD_VARIANT:---release}"

section "Verifier with Environment-Specific Configuration"
log_info "Environment: $DEPLOY_ENV"

# Select config file based on environment
CONFIG_FILE="$PROJECT_ROOT/container_config.${DEPLOY_ENV}.yml"

# Fall back to default config if environment-specific config doesn't exist
if [[ ! -f "$CONFIG_FILE" ]]; then
    log_warning "Environment-specific config not found: $CONFIG_FILE"
    CONFIG_FILE="$PROJECT_ROOT/container_config.yml"
    
    if [[ -f "$CONFIG_FILE" ]]; then
        log_info "Using default config: $CONFIG_FILE"
    else
        log_warning "Default config not found either, using built-in defaults"
        CONFIG_FILE=""
    fi
else
    log_info "Using environment config: $CONFIG_FILE"
fi

# Build verifier binary if needed
VERIFIER_BIN="$PROJECT_ROOT/target/release/verifier"
if [[ "$BUILD_VARIANT" == "--debug" ]]; then
    VERIFIER_BIN="$PROJECT_ROOT/target/debug/verifier"
fi

if [[ ! -f "$VERIFIER_BIN" ]]; then
    log_info "Building verifier binary..."
    cd "$PROJECT_ROOT"
    cargo build ${BUILD_VARIANT} --bin verifier
    
    if [[ $? -ne 0 ]]; then
        fail "Failed to build verifier binary"
        exit 1
    fi
    pass "Verifier binary built successfully"
else
    pass "Verifier binary found"
fi

# Build command with config file
VERIFIER_CMD="\"$VERIFIER_BIN\""

# Add all passed arguments
for arg in "$@"; do
    VERIFIER_CMD="$VERIFIER_CMD \"$arg\""
done

# Add config file if available
if [[ -n "$CONFIG_FILE" ]]; then
    VERIFIER_CMD="$VERIFIER_CMD --config \"$CONFIG_FILE\""
fi

# Add environment-specific CLI overrides if desired
# Example: Override executor with dynamic information
if [[ -n "$BUILD_NUMBER" ]]; then
    VERIFIER_CMD="$VERIFIER_CMD --executor \"Build #$BUILD_NUMBER\""
fi

log_info "Executing verifier..."
log_verbose "Command: $VERIFIER_CMD"
echo ""

# Execute command
eval "$VERIFIER_CMD"
VERIFIER_EXIT=$?

echo ""
if [[ $VERIFIER_EXIT -eq 0 ]]; then
    pass "Verifier completed successfully"
elif [[ $VERIFIER_EXIT -eq 1 ]]; then
    log_warning "Verifier completed with test failures (exit code 1)"
else
    fail "Verifier failed with exit code: $VERIFIER_EXIT"
fi

exit $VERIFIER_EXIT
