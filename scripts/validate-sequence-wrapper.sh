#!/usr/bin/env bash
#
# validate-sequence-wrapper.sh - Wrapper script for validate-sequence binary
#
# DESCRIPTION:
#   This script validates test sequence structure in YAML files. It ensures that
#   test sequences conform to the expected format, including proper step ordering,
#   required fields, and structural constraints.
#
# USAGE:
#   validate-sequence-wrapper.sh <sequence-file>
#
# ARGUMENTS:
#   sequence-file    Path to the test sequence YAML file to validate
#
# CONFIGURATION:
#   SEQUENCE_SCHEMA_FILE  Environment variable to specify the sequence schema file 
#                         (default: data/sequence-schema.json)
#   VALIDATE_SEQUENCE_BIN  Path to validate-sequence binary (default: auto-detected)
#
# EXIT CODES:
#   0 - Validation successful
#   1 - Validation failed or error occurred
#
# EXAMPLE USAGE WITH validate-files.sh:
#   # Validate all test sequence YAML files in a directory
#   export SEQUENCE_SCHEMA_FILE=data/sequence-schema.json
#   ./scripts/validate-files.sh --pattern 'sequence.*\.ya?ml$' --validator ./scripts/validate-sequence-wrapper.sh
#
#   # Validate with verbose output
#   ./scripts/validate-files.sh --pattern 'sequence.*\.ya?ml$' --validator ./scripts/validate-sequence-wrapper.sh --verbose
#
# STANDALONE USAGE:
#   # Validate a single test sequence file
#   export SEQUENCE_SCHEMA_FILE=data/sequence-schema.json
#   ./scripts/validate-sequence-wrapper.sh sequences/seq001.yml
#
#   # Use a different schema
#   SEQUENCE_SCHEMA_FILE=custom-sequence-schema.json ./scripts/validate-sequence-wrapper.sh my-sequence.yml
#

set -euo pipefail

# Configuration: Schema file to validate against
SEQUENCE_SCHEMA_FILE="${SEQUENCE_SCHEMA_FILE:-data/sequence-schema.json}"

# Auto-detect validate-sequence binary location
if [[ -n "${VALIDATE_SEQUENCE_BIN:-}" ]]; then
    VALIDATE_SEQUENCE="$VALIDATE_SEQUENCE_BIN"
elif [[ -x "target/release/validate-sequence" ]]; then
    VALIDATE_SEQUENCE="target/release/validate-sequence"
elif [[ -x "target/debug/validate-sequence" ]]; then
    VALIDATE_SEQUENCE="target/debug/validate-sequence"
elif command -v validate-sequence >/dev/null 2>&1; then
    VALIDATE_SEQUENCE="validate-sequence"
else
    echo "[ERROR] validate-sequence binary not found" >&2
    echo "[ERROR] Please build it with: cargo build --bin validate-sequence" >&2
    exit 1
fi

# Validate arguments
if [[ $# -eq 0 ]]; then
    echo "[ERROR] Missing required argument: sequence file path" >&2
    echo "Usage: $(basename "$0") <sequence-file>" >&2
    exit 1
fi

SEQUENCE_FILE="$1"

# Validate that sequence file exists
if [[ ! -f "$SEQUENCE_FILE" ]]; then
    echo "[ERROR] Sequence file not found: $SEQUENCE_FILE" >&2
    exit 1
fi

# Validate that schema file exists
if [[ ! -f "$SEQUENCE_SCHEMA_FILE" ]]; then
    echo "[ERROR] Sequence schema file not found: $SEQUENCE_SCHEMA_FILE" >&2
    echo "[ERROR] Set SEQUENCE_SCHEMA_FILE environment variable to specify the schema" >&2
    exit 1
fi

# Run validation
# The validate-sequence binary expects: validate-sequence <sequence-file> <schema-file>
"$VALIDATE_SEQUENCE" "$SEQUENCE_FILE" "$SEQUENCE_SCHEMA_FILE"
exit_code=$?

# Exit with the same code as validate-sequence
exit $exit_code
