#!/usr/bin/env bash
#
# validate-json-wrapper.sh - Wrapper script for validate-json binary
#
# DESCRIPTION:
#   This script validates JSON files against a JSON schema. It is designed to
#   integrate with the validate-files.sh framework for batch validation.
#
# USAGE:
#   validate-json-wrapper.sh <json-file>
#
# ARGUMENTS:
#   json-file    Path to the JSON file to validate
#
# CONFIGURATION:
#   SCHEMA_FILE  Environment variable to specify the schema file (default: data/schema.json)
#   VALIDATE_JSON_BIN  Path to validate-json binary (default: auto-detected)
#
# EXIT CODES:
#   0 - Validation successful
#   1 - Validation failed or error occurred
#
# EXAMPLE USAGE WITH validate-files.sh:
#   # Validate all JSON files in the repository
#   export SCHEMA_FILE=data/schema.json
#   ./scripts/validate-files.sh --pattern '\.json$' --validator ./scripts/validate-json-wrapper.sh
#
#   # Validate with verbose output
#   ./scripts/validate-files.sh --pattern '\.json$' --validator ./scripts/validate-json-wrapper.sh --verbose
#
# STANDALONE USAGE:
#   # Validate a single JSON file
#   export SCHEMA_FILE=data/schema.json
#   ./scripts/validate-json-wrapper.sh data/testcase.json
#
#   # Use a different schema
#   SCHEMA_FILE=my-schema.json ./scripts/validate-json-wrapper.sh my-file.json
#

set -euo pipefail

# Source logger library
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

# Configuration: Schema file to validate against
SCHEMA_FILE="${SCHEMA_FILE:-data/schema.json}"

# Auto-detect validate-json binary location
if [[ -n "${VALIDATE_JSON_BIN:-}" ]]; then
    VALIDATE_JSON="$VALIDATE_JSON_BIN"
elif [[ -x "target/release/validate-json" ]]; then
    VALIDATE_JSON="target/release/validate-json"
elif [[ -x "target/debug/validate-json" ]]; then
    VALIDATE_JSON="target/debug/validate-json"
elif command -v validate-json >/dev/null 2>&1; then
    VALIDATE_JSON="validate-json"
else
    log_error validate-json binary not found" >&2
    log_error Please build it with: cargo build --bin validate-json" >&2
    exit 1
fi

# Validate arguments
if [[ $# -eq 0 ]]; then
    log_error Missing required argument: JSON file path" >&2
    echo "Usage: $(basename "$0") <json-file>" >&2
    exit 1
fi

JSON_FILE="$1"

# Validate that JSON file exists
if [[ ! -f "$JSON_FILE" ]]; then
    log_error JSON file not found: $JSON_FILE" >&2
    exit 1
fi

# Validate that schema file exists
if [[ ! -f "$SCHEMA_FILE" ]]; then
    log_error Schema file not found: $SCHEMA_FILE" >&2
    log_error Set SCHEMA_FILE environment variable to specify the schema" >&2
    exit 1
fi

# Run validation
# The validate-json binary expects: validate-json <json-file> <schema-file>
"$VALIDATE_JSON" "$JSON_FILE" "$SCHEMA_FILE"
exit_code=$?

# Exit with the same code as validate-json
exit $exit_code
