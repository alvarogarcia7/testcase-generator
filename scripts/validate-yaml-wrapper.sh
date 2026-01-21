#!/usr/bin/env bash
#
# validate-yaml-wrapper.sh - Wrapper script for validate-yaml binary
#
# DESCRIPTION:
#   This script demonstrates how to integrate the validate-yaml binary with the
#   validate-files.sh framework. It validates YAML files against a JSON schema.
#
# USAGE:
#   validate-yaml-wrapper.sh <yaml-file>
#
# ARGUMENTS:
#   yaml-file    Path to the YAML file to validate
#
# CONFIGURATION:
#   SCHEMA_FILE  Environment variable to specify the schema file (default: data/schema.json)
#   VALIDATE_YAML_BIN  Path to validate-yaml binary (default: auto-detected)
#
# EXIT CODES:
#   0 - Validation successful
#   1 - Validation failed or error occurred
#
# EXAMPLE USAGE WITH validate-files.sh:
#   # Validate all YAML files in the repository
#   export SCHEMA_FILE=data/schema.json
#   ./scripts/validate-files.sh --pattern '\.ya?ml$' --validator ./scripts/validate-yaml-wrapper.sh
#
#   # Validate with verbose output
#   ./scripts/validate-files.sh --pattern '\.ya?ml$' --validator ./scripts/validate-yaml-wrapper.sh --verbose
#
# STANDALONE USAGE:
#   # Validate a single YAML file
#   export SCHEMA_FILE=data/schema.json
#   ./scripts/validate-yaml-wrapper.sh data/gsma_4.4.2.2_TC.yml
#
#   # Use a different schema
#   SCHEMA_FILE=my-schema.json ./scripts/validate-yaml-wrapper.sh my-file.yml
#

set -euo pipefail

# Get the script directory and source shared library
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib/find-binary.sh"

# Configuration: Schema file to validate against
SCHEMA_FILE="${SCHEMA_FILE:-data/schema.json}"

# Auto-detect validate-yaml binary location
VALIDATE_YAML=$(find_binary_or_exit "validate-yaml" "VALIDATE_YAML_BIN")

# Validate arguments
if [[ $# -eq 0 ]]; then
    echo "[ERROR] Missing required argument: YAML file path" >&2
    echo "Usage: $(basename "$0") <yaml-file>" >&2
    exit 1
fi

YAML_FILE="$1"

# Validate that YAML file exists
if [[ ! -f "$YAML_FILE" ]]; then
    echo "[ERROR] YAML file not found: $YAML_FILE" >&2
    exit 1
fi

# Validate that schema file exists
if [[ ! -f "$SCHEMA_FILE" ]]; then
    echo "[ERROR] Schema file not found: $SCHEMA_FILE" >&2
    echo "[ERROR] Set SCHEMA_FILE environment variable to specify the schema" >&2
    exit 1
fi

# Run validation
# The validate-yaml binary expects: validate-yaml <yaml-file> <schema-file>
"$VALIDATE_YAML" "$YAML_FILE" "$SCHEMA_FILE"
exit_code=$?

# Exit with the same code as validate-yaml
exit $exit_code
