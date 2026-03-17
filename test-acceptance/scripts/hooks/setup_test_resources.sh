#!/usr/bin/env bash
set -e

# Setup test hook - creates test resources and directories
WORKSPACE_DIR="/tmp/test_workspace_$$"
mkdir -p "$WORKSPACE_DIR"

# Create and initialize test database
sqlite3 "$WORKSPACE_DIR/test.db" "CREATE TABLE tests (id INTEGER, name TEXT);"
