#!/usr/bin/env bash
#
# validate-verification-wrapper.sh - Wrapper script for validating verification JSON files
#
# DESCRIPTION:
#   This script validates verification JSON files (e.g., test-output/*_verification.json)
#   against the verification schema. It is designed to integrate with the validate-files.sh
#   framework for batch validation.
#
# USAGE:
#   validate-verification-wrapper.sh <verification-json-file>
#
# ARGUMENTS:
#   verification-json-file    Path to the verification JSON file to validate
#
# CONFIGURATION:
#   VERIFICATION_SCHEMA_FILE  Environment variable to specify the schema file
#                             (default: data/verification-schema.json)
#   VALIDATE_JSON_BIN         Path to validate-json binary (default: auto-detected)
#
# EXIT CODES:
#   0 - Validation successful
#   1 - Validation failed or error occurred
#
# EXAMPLE USAGE WITH validate-files.sh:
#   # Validate all verification JSON files
#   ./scripts/validate-files.sh --pattern '_verification\.json$' \
#       --validator ./scripts/validate-verification-wrapper.sh
#
#   # Validate with verbose output
#   ./scripts/validate-files.sh --pattern '_verification\.json$' \
#       --validator ./scripts/validate-verification-wrapper.sh --verbose
#
# STANDALONE USAGE:
#   # Validate a single verification file
#   ./scripts/validate-verification-wrapper.sh \
#       test-output/SELF_VALIDATED_EXAMPLE_001_verification.json
#
#   # Use a different schema
#   VERIFICATION_SCHEMA_FILE=my-verification-schema.json \
#       ./scripts/validate-verification-wrapper.sh my-verification.json
#

set -euo pipefail

# Source logger library
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

# Configuration: Schema file to validate against
VERIFICATION_SCHEMA_FILE="${VERIFICATION_SCHEMA_FILE:-data/verification-schema.json}"

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
    log_error Missing required argument: verification JSON file path" >&2
    echo "Usage: $(basename "$0") <verification-json-file>" >&2
    exit 1
fi

JSON_FILE="$1"

# Validate that JSON file exists
if [[ ! -f "$JSON_FILE" ]]; then
    log_error Verification JSON file not found: $JSON_FILE" >&2
    exit 1
fi

# Validate that schema file exists
if [[ ! -f "$VERIFICATION_SCHEMA_FILE" ]]; then
    log_error Verification schema file not found: $VERIFICATION_SCHEMA_FILE" >&2
    log_error Set VERIFICATION_SCHEMA_FILE environment variable to specify the schema" >&2
    exit 1
fi

# Run validation
# The validate-json binary expects: validate-json <json-file> <schema-file>
"$VALIDATE_JSON" "$JSON_FILE" "$VERIFICATION_SCHEMA_FILE"
exit_code=$?

# Exit with the same code as validate-json
exit $exit_code
