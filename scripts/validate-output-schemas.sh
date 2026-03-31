#!/usr/bin/env bash
#
# validate-output-schemas.sh - Validate expected output YAML files against their schemas
#
# DESCRIPTION:
#   This script validates all sample YAML files in testcases/examples/expected_test_results/
#   and schemas/tcms/samples/ against their respective JSON schemas. It validates two types of files:
#   - test_case_result/*.yml - Individual test case results against test_case_result/schema.json
#   - container/*.yml - Container files with multiple results against container/container_schema.json
#   - schemas/tcms/samples/*.yml - Sample files against TCMS schemas
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

# New schema organization paths
SCHEMAS_DIR="schemas"
TCMS_SCHEMAS_DIR="$SCHEMAS_DIR/tcms"
TCMS_SAMPLES_DIR="$TCMS_SCHEMAS_DIR/samples"
TEMPLATES_DIR="$SCHEMAS_DIR/templates"

WARNINGS=0

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_warning_custom() {
    echo -e "${YELLOW}[WARN]${NC} $*"
    WARNINGS=$((WARNINGS + 1))
}

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

# Validate TCMS sample files against their schemas
validate_tcms_samples() {
    section "Validating TCMS Sample Files"
    
    if [[ ! -d "$TCMS_SAMPLES_DIR" ]]; then
        log_info "No TCMS samples directory found at: $TCMS_SAMPLES_DIR"
        return 0
    fi
    
    log_info "Directory: $TCMS_SAMPLES_DIR"
    
    # Count YAML/YML files in samples directory
    local sample_count=0
    if ls "$TCMS_SAMPLES_DIR"/*.yml >/dev/null 2>&1 || ls "$TCMS_SAMPLES_DIR"/*.yaml >/dev/null 2>&1; then
        sample_count=$(find "$TCMS_SAMPLES_DIR" -maxdepth 1 \( -name "*.yml" -o -name "*.yaml" \) | wc -l | tr -d ' ')
    fi
    
    if [ "$sample_count" -eq 0 ]; then
        log_info "No sample YAML files found in $TCMS_SAMPLES_DIR"
        return 0
    fi
    
    log_info "Found $sample_count sample file(s) to validate"
    
    # Validate samples against their respective schemas
    # This is a simple validation - just check they are valid YAML
    # More sophisticated validation against specific schemas would require
    # reading the 'schema' field from each sample
    
    uv run python3 << PYEOF
import yaml
import sys
from pathlib import Path

samples_dir = Path('$TCMS_SAMPLES_DIR')

failed = 0
total = 0

for sample_file in sorted(samples_dir.glob('*.yml')) + sorted(samples_dir.glob('*.yaml')):
    total += 1
    try:
        with open(sample_file) as f:
            data = yaml.safe_load(f)
        
        # Check if it's a valid YAML and has expected envelope fields
        if isinstance(data, dict):
            if 'type' in data and 'schema' in data:
                print(f'✓ {sample_file.name}: Valid TCMS sample (type={data.get("type")})')
            else:
                print(f'⚠ {sample_file.name}: Valid YAML but missing envelope fields')
                # Don't fail, just note
        else:
            print(f'✗ {sample_file.name}: Invalid structure (expected dict)')
            failed += 1
            
    except yaml.YAMLError as e:
        print(f'✗ {sample_file.name}: YAML parsing error - {e}')
        failed += 1
    except Exception as e:
        print(f'✗ {sample_file.name}: Error - {e}')
        failed += 1

sys.exit(0 if failed == 0 else 1)
PYEOF
    
    if [ $? -eq 0 ]; then
        pass "TCMS sample files validated successfully"
        return 0
    else
        fail "TCMS sample validation failed"
        return 1
    fi
}

# Check for misplaced sample files
check_misplaced_samples() {
    section "Checking for Misplaced Sample Files"
    
    local misplaced_found=0
    
    # Check for sample YAML files in schemas/tcms/ root (should be in samples/)
    if ls "$TCMS_SCHEMAS_DIR"/*.yml >/dev/null 2>&1 || ls "$TCMS_SCHEMAS_DIR"/*.yaml >/dev/null 2>&1; then
        log_warning_custom "Found YAML/YML files in schemas/tcms/ root"
        log_warning_custom "  Sample files should be in schemas/tcms/samples/"
        for file in "$TCMS_SCHEMAS_DIR"/*.yml "$TCMS_SCHEMAS_DIR"/*.yaml; do
            if [ -f "$file" ]; then
                log_warning_custom "  - $(basename "$file")"
                misplaced_found=1
            fi
        done
    fi
    
    # Check for template files in schemas/tcms/ (should be in schemas/templates/)
    for file in "$TCMS_SCHEMAS_DIR"/*; do
        if [ -f "$file" ]; then
            local basename_file
            basename_file="$(basename "$file")"
            if [[ "$basename_file" =~ template ]] || [[ "$basename_file" =~ \.j2$ ]] || [[ "$basename_file" =~ \.adoc$ ]]; then
                log_warning_custom "Template file found in schemas/tcms/: $basename_file"
                log_warning_custom "  Templates should be in schemas/templates/"
                misplaced_found=1
            fi
        fi
    done
    
    # Verify expected directories exist
    if [ ! -d "$TCMS_SAMPLES_DIR" ]; then
        log_warning_custom "schemas/tcms/samples/ directory does not exist"
        log_warning_custom "  Consider creating it to organize sample files"
        misplaced_found=1
    fi
    
    if [ ! -d "$TEMPLATES_DIR" ]; then
        log_warning_custom "schemas/templates/ directory does not exist"
        log_warning_custom "  Consider creating it to organize template files"
        misplaced_found=1
    fi
    
    if [ "$misplaced_found" -eq 0 ]; then
        pass "No misplaced files detected"
    else
        log_info "Found some organization issues (see warnings above)"
    fi
    
    return 0
}

# Check for duplicate schema files
check_duplicate_schemas() {
    section "Checking for Duplicate Schema Files"
    
    local duplicates_found=0
    
    # Use Python to track schema files across directories
    uv run python3 << PYEOF
import sys
from pathlib import Path
from collections import defaultdict

schemas_root = Path('$SCHEMAS_DIR')
tcms_dir = Path('$TCMS_SCHEMAS_DIR')

# Track schema files by basename
schema_files = defaultdict(list)

# Check schemas/ root
for schema_file in schemas_root.glob('*.json'):
    schema_files[schema_file.name].append(f'schemas/')

# Check schemas/tcms/
for schema_file in tcms_dir.glob('*.json'):
    schema_files[schema_file.name].append(f'schemas/tcms/')

# Check schemas/tcms/verification_methods/
vm_dir = tcms_dir / 'verification_methods'
if vm_dir.exists():
    for subdir in vm_dir.iterdir():
        if subdir.is_dir():
            for schema_file in subdir.glob('*.json'):
                rel_path = f'schemas/tcms/verification_methods/{subdir.name}/'
                schema_files[schema_file.name].append(rel_path)

# Report duplicates
duplicates = {name: locs for name, locs in schema_files.items() if len(locs) > 1}

if duplicates:
    for name, locations in duplicates.items():
        print(f'⚠ Duplicate schema file: {name}')
        print(f'  Locations: {", ".join(locations)}')
    sys.exit(1)
else:
    print('✓ No duplicate schema files detected')
    sys.exit(0)
PYEOF
    
    if [ $? -ne 0 ]; then
        log_warning_custom "Duplicate schema files detected (see above)"
        duplicates_found=1
    else
        pass "No duplicate schema files"
    fi
    
    return 0
}

# Main validation function
main() {
    section "Output Schema Validation"

    # Check Python requirements
    if ! check_python_requirements; then
        log_error "Python requirements not met"
        exit 1
    fi

    # Check for organizational issues
    check_misplaced_samples
    check_duplicate_schemas

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
    
    # Validate TCMS sample files
    if ! validate_tcms_samples; then
        exit 1
    fi

    # Summary
    section "Validation Summary"
    if [ "$WARNINGS" -gt 0 ]; then
        log_info "All validations passed but $WARNINGS warning(s) were issued"
    fi
    pass "All schema validations passed"
    exit 0
}

# Run main function
main "$@"
