#!/usr/bin/env bash
# Setup sccache environment variables for cargo builds
#
# This script sets RUSTC_WRAPPER and SCCACHE_DIR based on environment variables:
#   - USE_SCCACHE: Set to 1 to enable sccache, 0 or unset to disable
#   - DOCKER_BUILD: Set to 1 for Docker builds, 0 or unset for local builds
#
# Usage:
#   source scripts/setup-sccache-env.sh
#   cargo build

# Enable sccache if USE_SCCACHE is set to 1
if [ "${USE_SCCACHE:-0}" = "1" ]; then
    export RUSTC_WRAPPER="sccache"
    
    # Set SCCACHE_DIR based on DOCKER_BUILD
    if [ "${DOCKER_BUILD:-0}" = "1" ]; then
        export SCCACHE_DIR=".sccache/docker"
    else
        export SCCACHE_DIR=".sccache/host"
    fi
    
    echo "sccache enabled: RUSTC_WRAPPER=$RUSTC_WRAPPER, SCCACHE_DIR=$SCCACHE_DIR"
else
    # Unset RUSTC_WRAPPER to disable sccache
    unset RUSTC_WRAPPER
    unset SCCACHE_DIR
    echo "sccache disabled"
fi
