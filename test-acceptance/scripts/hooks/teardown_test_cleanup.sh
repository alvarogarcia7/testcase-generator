#!/usr/bin/env bash

# Teardown test hook - performs final cleanup (on_error: fail)
# This hook cleans up resources created by setup_test

# Remove workspace created by setup_test
WORKSPACE_DIR="/tmp/test_workspace_$$"
if [ -d "$WORKSPACE_DIR" ]; then
    rm -rf "$WORKSPACE_DIR"
fi

# Remove marker file
rm -f /tmp/teardown_marker_$$.txt

echo "Teardown completed" > /tmp/teardown_complete_$$.txt
