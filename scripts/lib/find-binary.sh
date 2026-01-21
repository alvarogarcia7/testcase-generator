#!/usr/bin/env bash
#
# find-binary.sh - Shared functions for finding binaries in target directory
#
# DESCRIPTION:
#   This library provides functions to locate Rust binaries in the target directory,
#   checking both release and debug builds, and falling back to system PATH.
#
# USAGE:
#   source scripts/lib/find-binary.sh
#   
#   # Find validate-yaml binary
#   VALIDATE_YAML=$(find_binary "validate-yaml")
#   if [[ -z "$VALIDATE_YAML" ]]; then
#       echo "Binary not found"
#       exit 1
#   fi
#
# FUNCTIONS:
#   find_binary <binary-name> [env-var-name]
#       Finds a binary by checking:
#       1. Environment variable (if provided)
#       2. target/release/<binary-name>
#       3. target/debug/<binary-name>
#       4. System PATH (command -v)
#       Returns the path to the binary or empty string if not found
#
#   find_binary_or_exit <binary-name> [env-var-name]
#       Same as find_binary but exits with error message if not found
#
#   ensure_binary_built <binary-name>
#       Checks if binary exists, builds it if missing
#       Returns 0 if binary exists or was built successfully, 1 otherwise
#

# Find a binary in target directory or system PATH
# Arguments:
#   $1 - binary name (required)
#   $2 - environment variable name to check first (optional)
# Returns:
#   Path to binary on stdout, or empty string if not found
find_binary() {
    local binary_name="$1"
    local env_var_name="${2:-}"
    
    # Check environment variable first if provided
    if [[ -n "$env_var_name" ]]; then
        local env_value="${!env_var_name:-}"
        if [[ -n "$env_value" ]]; then
            echo "$env_value"
            return 0
        fi
    fi
    
    # Check target/release
    if [[ -x "target/release/$binary_name" ]]; then
        echo "target/release/$binary_name"
        return 0
    fi
    
    # Check target/debug
    if [[ -x "target/debug/$binary_name" ]]; then
        echo "target/debug/$binary_name"
        return 0
    fi
    
    # Check system PATH
    if command -v "$binary_name" >/dev/null 2>&1; then
        echo "$binary_name"
        return 0
    fi
    
    # Not found
    echo ""
    return 1
}

# Find a binary or exit with error
# Arguments:
#   $1 - binary name (required)
#   $2 - environment variable name to check first (optional)
# Returns:
#   Path to binary on stdout
#   Exits with code 1 if not found
find_binary_or_exit() {
    local binary_name="$1"
    local env_var_name="${2:-}"
    
    local binary_path
    binary_path=$(find_binary "$binary_name" "$env_var_name")
    
    if [[ -z "$binary_path" ]]; then
        echo "[ERROR] $binary_name binary not found" >&2
        echo "[ERROR] Please build it with: cargo build --bin $binary_name" >&2
        exit 1
    fi
    
    echo "$binary_path"
    return 0
}

# Ensure a binary exists, building it if necessary
# Arguments:
#   $1 - binary name (required)
# Returns:
#   0 if binary exists or was built successfully
#   1 if build failed
ensure_binary_built() {
    local binary_name="$1"
    
    # Check if binary already exists
    local binary_path
    binary_path=$(find_binary "$binary_name")
    
    if [[ -n "$binary_path" ]]; then
        return 0
    fi
    
    # Binary not found, try to build it
    echo "[WARNING] $binary_name binary not found" >&2
    echo "[WARNING] Building the binary with: cargo build --bin $binary_name" >&2
    
    if cargo build --bin "$binary_name" 2>&1; then
        return 0
    else
        echo "[ERROR] Failed to build $binary_name" >&2
        return 1
    fi
}
