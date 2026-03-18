#!/usr/bin/env bash

# Teardown test hook - performs final cleanup (on_error: continue)
WORKSPACE_DIR="/tmp/teardown_workspace_$$"

# Remove workspace if it exists
if [ -d "$WORKSPACE_DIR" ]; then
    rm -rf "$WORKSPACE_DIR"
fi

# Remove marker file
rm -f /tmp/teardown_marker_$$.txt

echo "Teardown completed" > /tmp/teardown_complete_$$.txt
