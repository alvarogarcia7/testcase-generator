#!/usr/bin/env bash
#
# validate-testrun-wrapper.sh - Wrapper script for validate-testrun binary
#
# DESCRIPTION:
#   This script validates test run YAML files against the TestRun schema. It ensures
#   that test run configurations conform to the expected structure and constraints.
#
# USAGE:
#   validate-testrun-wrapper.sh <testrun-file>
#
# ARGUMENTS:
#   testrun-file    Path to the test run YAML file to validate
#
# CONFIGURATION:
#   TESTRUN_SCHEMA_FILE  Environment variable to specify the TestRun schema file 
#                        (default: schemas/testrun-schema.json)
#   VALIDATE_TESTRUN_BIN  Path to validate-testrun binary (default: auto-detected)
#
# EXIT CODES:
#   0 - Validation successful
#   1 - Validation failed or error occurred
#
# EXAMPLE USAGE WITH validate-files.sh:
#   # Validate all test run YAML files in a directory
#   export TESTRUN_SCHEMA_FILE=schemas/testrun-schema.json
#   ./scripts/validate-files.sh --pattern 'testrun.*\.ya?ml$' --validator ./scripts/validate-testrun-wrapper.sh
#
#   # Validate with verbose output
#   ./scripts/validate-files.sh --pattern 'testrun.*\.ya?ml$' --validator ./scripts/validate-testrun-wrapper.sh --verbose
#
# STANDALONE USAGE:
#   # Validate a single test run file
#   export TESTRUN_SCHEMA_FILE=schemas/testrun-schema.json
#   ./scripts/validate-testrun-wrapper.sh testruns/run001.yml
#
#   # Use a different schema
#   TESTRUN_SCHEMA_FILE=custom-testrun-schema.json ./scripts/validate-testrun-wrapper.sh my-testrun.yml
#

set -euo pipefail

# Source logger library
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

# Configuration: Schema file to validate against
TESTRUN_SCHEMA_FILE="${TESTRUN_SCHEMA_FILE:-schemas/testrun-schema.json}"

# Auto-detect validate-testrun binary location
if [[ -n "${VALIDATE_TESTRUN_BIN:-}" ]]; then
    VALIDATE_TESTRUN="$VALIDATE_TESTRUN_BIN"
elif [[ -x "target/release/validate-testrun" ]]; then
    VALIDATE_TESTRUN="target/release/validate-testrun"
elif [[ -x "target/debug/validate-testrun" ]]; then
    VALIDATE_TESTRUN="target/debug/validate-testrun"
elif command -v validate-testrun >/dev/null 2>&1; then
    VALIDATE_TESTRUN="validate-testrun"
else
    log_error validate-testrun binary not found" >&2
    log_error Please build it with: cargo build --bin validate-testrun" >&2
    exit 1
fi

# Validate arguments
if [[ $# -eq 0 ]]; then
    log_error Missing required argument: test run file path" >&2
    echo "Usage: $(basename "$0") <testrun-file>" >&2
    exit 1
fi

TESTRUN_FILE="$1"

# Validate that test run file exists
if [[ ! -f "$TESTRUN_FILE" ]]; then
    log_error Test run file not found: $TESTRUN_FILE" >&2
    exit 1
fi

# Validate that schema file exists
if [[ ! -f "$TESTRUN_SCHEMA_FILE" ]]; then
    log_error TestRun schema file not found: $TESTRUN_SCHEMA_FILE" >&2
    log_error Set TESTRUN_SCHEMA_FILE environment variable to specify the schema" >&2
    exit 1
fi

# Run validation
# The validate-testrun binary expects: validate-testrun <testrun-file> <schema-file>
"$VALIDATE_TESTRUN" "$TESTRUN_FILE" "$TESTRUN_SCHEMA_FILE"
exit_code=$?

# Exit with the same code as validate-testrun
exit $exit_code
