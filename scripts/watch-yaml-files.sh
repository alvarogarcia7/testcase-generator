#!/usr/bin/env bash
#
# watch-yaml-files.sh - Example script for watching YAML files
#
# This script demonstrates how to use watch mode to continuously monitor
# the testcases/ directory for YAML file changes with automatic validation.
#

set -euo pipefail

# Get the script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source shared library for finding binaries
source "$SCRIPT_DIR/lib/find-binary.sh"

# Change to project root
cd "$PROJECT_ROOT"

# Set the schema file for validation
export SCHEMA_FILE="${SCHEMA_FILE:-schemas/schema.json}"

# Check if schema file exists
if [[ ! -f "$SCHEMA_FILE" ]]; then
    echo "[ERROR] Schema file not found: $SCHEMA_FILE" >&2
    echo "[ERROR] Please ensure the schema file exists or set SCHEMA_FILE environment variable" >&2
    exit 1
fi

# Ensure validate-yaml binary exists (build if necessary)
ensure_binary_built "validate-yaml" || exit 1

# Run watch mode
echo "Starting watch mode for YAML files in testcases/ directory..."
echo "Schema: $SCHEMA_FILE"
echo ""

./scripts/validate-files.sh \
    --pattern '\.ya?ml$' \
    --validator ./scripts/validate-yaml-wrapper.sh \
    --watch testcases/
