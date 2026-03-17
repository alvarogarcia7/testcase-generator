#!/usr/bin/env bash
set -e

# Get script directory and source logger
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

# Script to validate test case result container YAML/JSON files against the schema
# Usage: ./scripts/validate-container-output.sh <file.yml|file.json>

# Check if file argument is provided
if [ $# -lt 1 ]; then
    log_error "Usage: $0 <file.yml|file.json>"
    log_info "Example: $0 data/testcase_results_container/data_sample.yml"
    exit 1
fi

INPUT_FILE="$1"
SCHEMA_FILE="data/testcase_results_container/schema.json"

# Check if input file exists
if [ ! -f "$INPUT_FILE" ]; then
    log_error "Input file not found: $INPUT_FILE"
    exit 1
fi

# Check if schema file exists
if [ ! -f "$SCHEMA_FILE" ]; then
    log_error "Schema file not found: $SCHEMA_FILE"
    exit 1
fi

section "Validating Container Output"
log_info "Input file: $INPUT_FILE"
log_info "Schema file: $SCHEMA_FILE"

# Determine file format
FILE_EXT="${INPUT_FILE##*.}"
case "$FILE_EXT" in
    yml|yaml)
        FORMAT="yaml"
        ;;
    json)
        FORMAT="json"
        ;;
    *)
        log_error "Unsupported file format: $FILE_EXT"
        log_info "Supported formats: yml, yaml, json"
        exit 1
        ;;
esac

log_info "Format: $FORMAT"

# Create temporary JSON file for validation
TEMP_JSON=$(mktemp)
setup_cleanup "$TEMP_JSON"

# Convert YAML to JSON if needed
if [ "$FORMAT" = "yaml" ]; then
    log_info "Converting YAML to JSON for validation..."
    
    # Use Python to convert YAML to JSON
    if command -v python3 >/dev/null 2>&1; then
        python3 -c "
import sys
import json
import yaml

try:
    with open('$INPUT_FILE', 'r') as f:
        data = yaml.safe_load(f)
    with open('$TEMP_JSON', 'w') as f:
        json.dump(data, f, indent=2)
except Exception as e:
    print(f'Error: {e}', file=sys.stderr)
    sys.exit(1)
" || {
            log_error "Failed to convert YAML to JSON"
            exit 1
        }
    else
        log_error "Python3 is required for YAML validation"
        exit 1
    fi
    
    VALIDATION_FILE="$TEMP_JSON"
else
    # For JSON, validate directly
    VALIDATION_FILE="$INPUT_FILE"
fi

# Validate using Python jsonschema
log_info "Validating against schema..."

python3 -c "
import sys
import json
from jsonschema import validate, ValidationError, Draft7Validator

try:
    # Load schema
    with open('$SCHEMA_FILE', 'r') as f:
        schema = json.load(f)
    
    # Load data
    with open('$VALIDATION_FILE', 'r') as f:
        data = json.load(f)
    
    # Create validator
    validator = Draft7Validator(schema)
    
    # Validate
    errors = list(validator.iter_errors(data))
    
    if errors:
        print('Validation failed with the following errors:', file=sys.stderr)
        for i, error in enumerate(errors, 1):
            path = ' > '.join(str(p) for p in error.absolute_path) if error.absolute_path else 'root'
            print(f'  {i}. At {path}: {error.message}', file=sys.stderr)
        sys.exit(1)
    else:
        print('✓ Validation successful: document conforms to schema')
        sys.exit(0)
        
except FileNotFoundError as e:
    print(f'Error: File not found: {e.filename}', file=sys.stderr)
    sys.exit(1)
except json.JSONDecodeError as e:
    print(f'Error: Invalid JSON: {e}', file=sys.stderr)
    sys.exit(1)
except Exception as e:
    print(f'Error: {e}', file=sys.stderr)
    sys.exit(1)
" && {
    pass "Validation passed: $INPUT_FILE conforms to schema"
    exit 0
} || {
    fail "Validation failed: $INPUT_FILE does not conform to schema"
    exit 1
}
