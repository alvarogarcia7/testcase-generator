#!/usr/bin/env bash
#
# validate-output-schemas.sh - Validate expected output YAML files against their schemas
#
# DESCRIPTION:
#   This script validates all sample YAML files in testcases/examples/expected_test_results/
#   against their respective JSON schemas. It validates two types of files:
#   - test_case_result/*.yml - Individual test case results against test_case_result/schema.json
#   - container/*.yml - Container files with multiple results against container/container_schema.json
#
# USAGE:
#   validate-output-schemas.sh
#
# CONFIGURATION:
#   VERBOSE  Set to "1" for verbose output
#
# EXIT CODES:
#   0 - All validations successful
#   1 - One or more validations failed
#
# REQUIREMENTS:
#   - Python 3 with yaml and jsonschema modules
#   - JSON schemas in schemas/ directory
#

set -euo pipefail

# Get script directory and source logger
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

# Configuration
EXPECTED_RESULTS_DIR="testcases/examples/expected_test_results"
TEST_CASE_RESULT_DIR="$EXPECTED_RESULTS_DIR/test_case_result"
CONTAINER_DIR="$EXPECTED_RESULTS_DIR/container"
TEST_CASE_RESULT_SCHEMA="$TEST_CASE_RESULT_DIR/schema.json"
CONTAINER_SCHEMA="$CONTAINER_DIR/container_schema.json"

# Check for required Python modules
check_python_requirements() {
    log_info "Checking Python requirements..."

    if ! command -v uv >/dev/null 2>&1; then
        log_error "UV is required but not found"
        return 1
    fi

    if ! command -v python3 >/dev/null 2>&1; then
        log_error "Python 3 is required but not found"
        return 1
    fi

    # Check for yaml module
    if ! uv run python3 -c 'import yaml' 2>/dev/null; then
        log_error "Python yaml module is required but not found"
        log_info "Install with: pip3 install pyyaml"
        return 1
    fi

    # Check for jsonschema module
    if ! uv run python3 -c 'import jsonschema' 2>/dev/null; then
        log_error "Python jsonschema module is required but not found"
        log_info "Install with: pip3 install jsonschema"
        return 1
    fi

    log_info "Python requirements satisfied"
    return 0
}

# Validate YAML files using Python
validate_yaml_files() {
    local dir="$1"
    local schema="$2"

    uv run python3 << PYEOF
import json
import yaml
import sys
from pathlib import Path
from jsonschema import Draft7Validator

result_dir = Path('$dir')
schema_file = Path('$schema')

# Load schema
with open(schema_file) as f:
    schema = json.load(f)

failed = 0
total = 0

for yml_file in sorted(result_dir.glob('*.yml')):
    total += 1
    try:
        with open(yml_file) as f:
            data = yaml.safe_load(f)

        validator = Draft7Validator(schema)
        errors = list(validator.iter_errors(data))

        if errors:
            print(f'✗ {yml_file.name}: Validation failed')
            for error in errors[:3]:  # Show first 3 errors
                print(f'  - {error.message}')
            failed += 1
        else:
            print(f'✓ {yml_file.name}: Valid')
    except Exception as e:
        print(f'✗ {yml_file.name}: Error - {e}')
        failed += 1

sys.exit(0 if failed == 0 else 1)
PYEOF
}

# Main validation function
main() {
    section "Output Schema Validation"

    # Check Python requirements
    if ! check_python_requirements; then
        log_error "Python requirements not met"
        exit 1
    fi

    # Verify directories exist
    if [[ ! -d "$TEST_CASE_RESULT_DIR" ]]; then
        log_error "Test case result directory not found: $TEST_CASE_RESULT_DIR"
        exit 1
    fi

    # Verify schema files exist
    if [[ ! -f "$TEST_CASE_RESULT_SCHEMA" ]]; then
        log_error "Schema file not found: $TEST_CASE_RESULT_SCHEMA"
        exit 1
    fi

    # Validate test_case_result files
    section "Validating Test Case Results"
    log_info "Directory: $TEST_CASE_RESULT_DIR"
    log_info "Schema: $TEST_CASE_RESULT_SCHEMA"

    if validate_yaml_files "$TEST_CASE_RESULT_DIR" "$TEST_CASE_RESULT_SCHEMA"; then
        pass "Test case result files validated successfully"
    else
        fail "Test case result validation failed"
        exit 1
    fi
        

    # Validate container files
    if [[ -d "$CONTAINER_DIR" ]]; then
        section "Validating Container Files"
        log_info "Directory: $CONTAINER_DIR"
        log_info "Schema: $CONTAINER_SCHEMA"

        # Verify container schema exists
        if [[ ! -f "$CONTAINER_SCHEMA" ]]; then
            log_error "Schema file not found: $CONTAINER_SCHEMA"
            exit 1
        fi

        if validate_yaml_files "$CONTAINER_DIR" "$CONTAINER_SCHEMA"; then
            pass "Container files validated successfully"
        else
            fail "Container validation failed"
            exit 1
        fi

    fi

    # Summary
    section "Validation Summary"
    pass "All schema validations passed"
    exit 0
}

# Run main function
main "$@"
