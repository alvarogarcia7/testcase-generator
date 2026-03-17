#!/usr/bin/env bash
set -e

# Setup test hook - prepares test infrastructure
# This hook executes once after script_start, before any test sequences run

SETUP_TIME=$(date +%s)
echo "setup_test: Test workspace preparation at $(date)" >> /tmp/setup_test_$$.log
echo "setup_test: Creating test workspace directories" >> /tmp/setup_test_$$.log

# Create common test workspace if it doesn't exist
if [ ! -d /tmp/test_workspace_$$ ]; then
    mkdir -p /tmp/test_workspace_$$
    echo "setup_test: Created /tmp/test_workspace_$$" >> /tmp/setup_test_$$.log
fi
