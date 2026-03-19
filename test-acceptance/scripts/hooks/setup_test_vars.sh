#!/usr/bin/env bash
# Hook: setup_test - Set up environment variables

# This hook executes once after script_start
# It can set environment variables for the test

set -e

# Export environment variable for test steps
export HOOK_WORKSPACE="/tmp/hook_workspace"

# Log that setup executed
echo "setup_test vars configured" >&2

exit 0
