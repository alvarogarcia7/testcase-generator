#!/usr/bin/env bash
# Fix executable permissions for integration test scripts

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "Fixing executable permissions for integration test scripts..."

# Make all .sh files in tests/integration executable
find "$PROJECT_ROOT/tests/integration" -type f -name "*.sh" -exec chmod +x {} \;

echo "✓ Permissions fixed"
