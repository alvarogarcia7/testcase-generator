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

# Validate a YAML file against a JSON schema
validate_yaml_file() {
    local yaml_file="$1"
    local schema_file="$2"
    local file_name
    file_name=$(basename "$yaml_file")
    
    log_info "Validating: $file_name"
    
    # Create temporary JSON file
    local temp_json
    temp_json=$(mktemp)
    
    # Convert YAML to JSON and validate
    uv run python3 -c "
import sys
import json
import yaml
from jsonschema import validate, ValidationError, Draft7Validator

try:
    # Load YAML file
    with open('$yaml_file', 'r') as f:
        data = yaml.safe_load(f)
    
    # Load schema
    with open('$schema_file', 'r') as f:
        schema = json.load(f)
    
    # Create validator
    validator = Draft7Validator(schema)
    
    # Validate
    errors = list(validator.iter_errors(data))
    
    if errors:
        print('Validation failed with the following errors:', file=sys.stderr)
        for i, error in enumerate(errors, 1):
            path = ' > '.join(str(p) for p in error.absolute_path) if error.absolute_path else 'root'
            print(f'  Error {i}: At {path}: {error.message}', file=sys.stderr)
        sys.exit(1)
    else:
        sys.exit(0)
        
except FileNotFoundError as e:
    print(f'Error: File not found: {e.filename}', file=sys.stderr)
    sys.exit(1)
except yaml.YAMLError as e:
    print(f'Error: Invalid YAML: {e}', file=sys.stderr)
    sys.exit(1)
except json.JSONDecodeError as e:
    print(f'Error: Invalid JSON in schema: {e}', file=sys.stderr)
    sys.exit(1)
except Exception as e:
    print(f'Error: {e}', file=sys.stderr)
    sys.exit(1)
" && {
        pass "$file_name conforms to schema"
        rm -f "$temp_json"
        return 0
    } || {
        fail "$file_name does not conform to schema"
        rm -f "$temp_json"
        return 1
    }
}

# Main validation function
main() {
    local failed=0
    local total=0
    
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
    
    for yaml_file in "$TEST_CASE_RESULT_DIR"/*.yml; do
        if [[ ! -f "$yaml_file" ]]; then
            log_warning "No YAML files found in $TEST_CASE_RESULT_DIR"
            continue
        fi
        
        ((total++))
        if ! validate_yaml_file "$yaml_file" "$TEST_CASE_RESULT_SCHEMA"; then
            ((failed++))
        fi
    done
    
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
        
        for yaml_file in "$CONTAINER_DIR"/*.yml; do
            if [[ ! -f "$yaml_file" ]]; then
                log_warning "No YAML files found in $CONTAINER_DIR"
                continue
            fi
            
            ((total++))
            if ! validate_yaml_file "$yaml_file" "$CONTAINER_SCHEMA"; then
                ((failed++))
            fi
        done
    fi
    
    # Summary
    section "Validation Summary"
    log_info "Total files validated: $total"
    
    if [[ $failed -eq 0 ]]; then
        pass "All validations passed"
        exit 0
    else
        fail "$failed of $total validations failed"
        exit 1
    fi
}

# Run main function
main "$@"
