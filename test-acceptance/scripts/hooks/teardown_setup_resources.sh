#!/usr/bin/env bash
set -e

# Setup test hook for teardown test - creates resources that need cleanup
WORKSPACE_DIR="/tmp/teardown_workspace_$$"
mkdir -p "$WORKSPACE_DIR"
echo "Resources created for teardown test" > "$WORKSPACE_DIR/resource.txt"
