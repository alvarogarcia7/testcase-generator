#!/usr/bin/env bash
#
# validate_envelope_schemas.sh - Validate TCMS envelope schemas for internal consistency
#
# This script validates:
# 1. All versioned schemas properly reference the envelope meta-schema
# 2. Sample files with envelope fields validate against their respective schemas
# 3. Schema files are syntactically valid JSON
# 4. Required envelope fields (type, schema) are present and properly constrained
# 5. Schema organization - schemas/tcms/ contains only schema files and approved subdirectories
#
# Usage:
#   ./scripts/validate_envelope_schemas.sh [--verbose]
#
# Exit codes:
#   0 - All validations passed
#   1 - One or more validations failed
#

set -euo pipefail

# Canonical paths:
# - Versioned schemas: schemas/tcms/*.schema.v1.json (production standard with envelope)
# - Envelope meta-schema: schemas/tcms-envelope.schema.json
# - Sample data: schemas/tcms/samples/ (canonical location)
# - Templates: schemas/templates/ (canonical location)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
SCHEMAS_DIR="${REPO_ROOT}/schemas"
TCMS_SCHEMAS_DIR="${SCHEMAS_DIR}/tcms"
TCMS_SAMPLES_DIR="${SCHEMAS_DIR}/tcms/samples"
TEMPLATES_DIR="${SCHEMAS_DIR}/templates"
ENVELOPE_SCHEMA="${SCHEMAS_DIR}/tcms-envelope.schema.json"

VERBOSE=0
FAILED=0
PASSED=0
WARNINGS=0

# Parse arguments
for arg in "$@"; do
    case "$arg" in
        --verbose)
            VERBOSE=1
            ;;
        *)
            echo "Unknown argument: $arg"
            echo "Usage: $0 [--verbose]"
            exit 1
            ;;
    esac
done

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $*"
}

log_success() {
    echo -e "${GREEN}[PASS]${NC} $*"
}

log_error() {
    echo -e "${RED}[FAIL]${NC} $*"
}

log_warning() {
    echo -e "${YELLOW}[WARN]${NC} $*"
    WARNINGS=$((WARNINGS + 1))
}

# Check if required tools are available
check_dependencies() {
    log_info "Checking dependencies..."
    
    # Check if uv is available (project uses uv for Python)
    if command -v uv > /dev/null 2>&1; then
        # Use uv to check for jsonschema
        if ! uv run python3.14 -c "import jsonschema" 2>/dev/null; then
            log_error "Python jsonschema module is required but not installed"
            log_info "Install with: uv pip install jsonschema"
            exit 1
        fi
        PYTHON_CMD="uv run python3.14"
    elif command -v python3 > /dev/null 2>&1; then
        # Fallback to system python3
        if ! python3 -c "import jsonschema" 2>/dev/null; then
            log_error "Python jsonschema module is required but not installed"
            log_info "Install with: pip install jsonschema"
            exit 1
        fi
        PYTHON_CMD="python3"
    else
        log_error "python3 is required but not found in PATH"
        exit 1
    fi
    
    log_success "All dependencies available"
}

# Validate JSON syntax
validate_json_syntax() {
    local file="$1"
    local filename
    filename="$(basename "$file")"
    
    if [ "$VERBOSE" -eq 1 ]; then
        log_info "Validating JSON syntax: $filename"
    fi
    
    if $PYTHON_CMD -c "import json; json.load(open('$file'))" 2>/dev/null; then
        PASSED=$((PASSED + 1))
        if [ "$VERBOSE" -eq 1 ]; then
            log_success "Valid JSON: $filename"
        fi
        return 0
    else
        FAILED=$((FAILED + 1))
        log_error "Invalid JSON syntax: $filename"
        $PYTHON_CMD -c "import json; json.load(open('$file'))" 2>&1 || true
        return 1
    fi
}

# Validate that a schema references the envelope meta-schema
validate_envelope_reference() {
    local schema_file="$1"
    local filename
    filename="$(basename "$schema_file")"
    
    if [ "$VERBOSE" -eq 1 ]; then
        log_info "Checking envelope reference: $filename"
    fi
    
    # Check if schema uses allOf to compose with envelope
    if grep -q '"allOf"' "$schema_file" && grep -q 'tcms-envelope.schema.json' "$schema_file"; then
        PASSED=$((PASSED + 1))
        if [ "$VERBOSE" -eq 1 ]; then
            log_success "References envelope meta-schema: $filename"
        fi
        return 0
    else
        FAILED=$((FAILED + 1))
        log_error "Missing envelope meta-schema reference: $filename"
        return 1
    fi
}

# Validate envelope field constraints in versioned schemas
validate_envelope_constraints() {
    local schema_file="$1"
    local expected_type="$2"
    local expected_schema_path="$3"
    local filename
    filename="$(basename "$schema_file")"
    
    if [ "$VERBOSE" -eq 1 ]; then
        log_info "Validating envelope constraints: $filename"
    fi
    
    local has_type_const=0
    local has_schema_const=0
    
    # Check for type const constraint
    if $PYTHON_CMD -c "
import json
import sys
with open('$schema_file') as f:
    schema = json.load(f)
    # Check in allOf composition
    if 'allOf' in schema:
        for item in schema['allOf']:
            if 'properties' in item and 'type' in item['properties']:
                if 'const' in item['properties']['type'] and item['properties']['type']['const'] == '$expected_type':
                    sys.exit(0)
sys.exit(1)
" 2>/dev/null; then
        has_type_const=1
    fi
    
    # Check for schema const constraint
    if $PYTHON_CMD -c "
import json
import sys
with open('$schema_file') as f:
    schema = json.load(f)
    # Check in allOf composition
    if 'allOf' in schema:
        for item in schema['allOf']:
            if 'properties' in item and 'schema' in item['properties']:
                if 'const' in item['properties']['schema'] and item['properties']['schema']['const'] == '$expected_schema_path':
                    sys.exit(0)
sys.exit(1)
" 2>/dev/null; then
        has_schema_const=1
    fi
    
    if [ "$has_type_const" -eq 1 ] && [ "$has_schema_const" -eq 1 ]; then
        PASSED=$((PASSED + 1))
        if [ "$VERBOSE" -eq 1 ]; then
            log_success "Valid envelope constraints: $filename (type=$expected_type, schema=$expected_schema_path)"
        fi
        return 0
    else
        FAILED=$((FAILED + 1))
        log_error "Invalid envelope constraints: $filename"
        if [ "$has_type_const" -eq 0 ]; then
            log_error "  Missing or incorrect 'type' const: expected '$expected_type'"
        fi
        if [ "$has_schema_const" -eq 0 ]; then
            log_error "  Missing or incorrect 'schema' const: expected '$expected_schema_path'"
        fi
        return 1
    fi
}

# Create a sample document with envelope fields and validate it
validate_sample_with_envelope() {
    local schema_file="$1"
    local doc_type="$2"
    local schema_path="$3"
    local filename
    filename="$(basename "$schema_file")"
    
    if [ "$VERBOSE" -eq 1 ]; then
        log_info "Validating sample document: $filename"
    fi
    
    local temp_sample
    temp_sample="$(mktemp)"
    
    # Create minimal valid sample based on document type
    case "$doc_type" in
        test_case)
            cat > "$temp_sample" <<EOF
{
  "type": "$doc_type",
  "schema": "$schema_path",
  "requirement": "REQ-001",
  "item": 1,
  "tc": 1,
  "id": "test-case-001",
  "description": "Sample test case",
  "general_initial_conditions": {},
  "initial_conditions": {},
  "test_sequences": []
}
EOF
            ;;
        test_execution)
            cat > "$temp_sample" <<EOF
{
  "type": "$doc_type",
  "schema": "$schema_path",
  "test_sequence": 1,
  "step": 1,
  "command": "echo test",
  "exit_code": 0,
  "output": "test",
  "result_verification_pass": true,
  "output_verification_pass": true
}
EOF
            ;;
        test_verification|test_result)
            cat > "$temp_sample" <<EOF
{
  "type": "$doc_type",
  "schema": "$schema_path",
  "test_case_id": "test-001",
  "description": "Sample test",
  "sequences": [],
  "total_steps": 0,
  "passed_steps": 0,
  "failed_steps": 0,
  "not_executed_steps": 0,
  "overall_pass": true
}
EOF
            ;;
        container_config)
            cat > "$temp_sample" <<EOF
{
  "type": "$doc_type",
  "schema": "$schema_path",
  "title": "Test Report",
  "project": "Sample Project"
}
EOF
            ;;
        test_results_container)
            cat > "$temp_sample" <<EOF
{
  "type": "$doc_type",
  "schema": "$schema_path",
  "title": "Test Results",
  "project": "Sample Project",
  "test_date": "2024-01-01T00:00:00Z",
  "test_results": [],
  "metadata": {
    "execution_duration": 0,
    "total_test_cases": 0,
    "passed_test_cases": 0,
    "failed_test_cases": 0
  }
}
EOF
            ;;
        *)
            log_error "Unknown document type: $doc_type"
            rm -f "$temp_sample"
            return 1
            ;;
    esac
    
    # Validate sample against schema
    if $PYTHON_CMD -c "
import json
import sys
from jsonschema import validate, ValidationError, RefResolver
import os

schema_dir = os.path.dirname('$schema_file')
schema_uri = 'file://' + os.path.abspath('$schema_file')

# Load envelope meta-schema
envelope_schema_path = os.path.join(schema_dir, '..', 'tcms-envelope.schema.json')
envelope_schema_path = os.path.abspath(envelope_schema_path)
with open(envelope_schema_path) as ef:
    envelope_schema = json.load(ef)
    envelope_id = envelope_schema.get('\\\$id', 'https://tcms.example.com/schemas/tcms-envelope.schema.json')

with open('$schema_file') as sf:
    schema = json.load(sf)
    
with open('$temp_sample') as df:
    doc = json.load(df)

# Create resolver for handling \\\$ref with envelope schema in store
store = {
    envelope_id: envelope_schema,
    'file://' + envelope_schema_path: envelope_schema,
}
resolver = RefResolver(
    base_uri=schema_uri,
    referrer=schema,
    store=store,
)

try:
    validate(instance=doc, schema=schema, resolver=resolver)
    sys.exit(0)
except ValidationError as e:
    print(f'Validation error: {e.message}', file=sys.stderr)
    sys.exit(1)
" 2>&1; then
        PASSED=$((PASSED + 1))
        if [ "$VERBOSE" -eq 1 ]; then
            log_success "Sample validation passed: $filename"
        fi
        rm -f "$temp_sample"
        return 0
    else
        FAILED=$((FAILED + 1))
        log_error "Sample validation failed: $filename"
        rm -f "$temp_sample"
        return 1
    fi
}

# Validate schema organization in schemas/tcms/
validate_tcms_organization() {
    log_info "Validating schemas/tcms/ organization..."
    
    local org_failed=0
    
    # Check for files in schemas/tcms/ root
    for file in "$TCMS_SCHEMAS_DIR"/*; do
        if [ ! -e "$file" ]; then
            continue
        fi
        
        local basename_file
        basename_file="$(basename "$file")"
        
        # Check if it's a directory
        if [ -d "$file" ]; then
            # Allow only specific subdirectories
            case "$basename_file" in
                samples|verification_methods)
                    if [ "$VERBOSE" -eq 1 ]; then
                        log_success "Approved subdirectory: $basename_file"
                    fi
                    ;;
                *)
                    log_warning "Unexpected subdirectory in schemas/tcms/: $basename_file"
                    log_warning "  Only 'samples/' and 'verification_methods/' are expected"
                    org_failed=1
                    ;;
            esac
        elif [ -f "$file" ]; then
            # Check if it's a valid schema file
            if [[ "$basename_file" =~ ^[a-z-]+\.schema\.v[0-9]+\.json$ ]]; then
                if [ "$VERBOSE" -eq 1 ]; then
                    log_success "Valid schema file: $basename_file"
                fi
            elif [[ "$basename_file" =~ \.md$ ]] || [[ "$basename_file" =~ \.csv$ ]]; then
                # Documentation files are acceptable
                if [ "$VERBOSE" -eq 1 ]; then
                    log_success "Documentation file: $basename_file"
                fi
            else
                log_warning "Misplaced file in schemas/tcms/: $basename_file"
                log_warning "  Expected pattern: *.schema.v*.json or documentation files"
                org_failed=1
            fi
        fi
    done
    
    if [ "$org_failed" -eq 0 ]; then
        PASSED=$((PASSED + 1))
        log_success "schemas/tcms/ organization is valid"
    else
        FAILED=$((FAILED + 1))
        log_error "schemas/tcms/ organization has issues"
    fi
    
    return "$org_failed"
}

# Detect duplicate schema files across directories
detect_duplicate_schemas() {
    log_info "Checking for duplicate schema files..."
    
    # Use Python to detect duplicates for better compatibility
    local duplicates_output
    duplicates_output=$($PYTHON_CMD << 'PYEOF'
import os
from collections import defaultdict

schemas_dir = os.environ.get('SCHEMAS_DIR', 'schemas')
tcms_dir = os.path.join(schemas_dir, 'tcms')

# Track schema files by basename
schema_files = defaultdict(list)

# Check schemas/ root
if os.path.isdir(schemas_dir):
    for file in os.listdir(schemas_dir):
        if file.endswith('.json') and os.path.isfile(os.path.join(schemas_dir, file)):
            schema_files[file].append('schemas/')

# Check schemas/tcms/
if os.path.isdir(tcms_dir):
    for file in os.listdir(tcms_dir):
        if file.endswith('.json') and os.path.isfile(os.path.join(tcms_dir, file)):
            schema_files[file].append('schemas/tcms/')

# Check schemas/tcms/verification_methods/
vm_dir = os.path.join(tcms_dir, 'verification_methods')
if os.path.isdir(vm_dir):
    for subdir in os.listdir(vm_dir):
        subdir_path = os.path.join(vm_dir, subdir)
        if os.path.isdir(subdir_path):
            for file in os.listdir(subdir_path):
                if file.endswith('.json') and os.path.isfile(os.path.join(subdir_path, file)):
                    schema_files[file].append(f'schemas/tcms/verification_methods/{subdir}/')

# Report duplicates
duplicates = {name: locs for name, locs in schema_files.items() if len(locs) > 1}

if duplicates:
    for name, locations in duplicates.items():
        print(f'DUPLICATE:{name}:{",".join(locations)}')
else:
    print('NO_DUPLICATES')
PYEOF
)
    
    if echo "$duplicates_output" | grep -q "^DUPLICATE:"; then
        # Parse and display duplicates
        echo "$duplicates_output" | while IFS=':' read -r status name locations; do
            if [ "$status" = "DUPLICATE" ]; then
                log_warning "Duplicate schema file found: $name"
                log_warning "  Locations: $locations"
            fi
        done
        log_warning "Duplicate schema files detected (see warnings above)"
    else
        PASSED=$((PASSED + 1))
        log_success "No duplicate schema files detected"
    fi
    
    return 0
}

# Validate that templates are in schemas/templates/
validate_templates_location() {
    log_info "Validating template file locations..."
    
    local misplaced_found=0
    
    # Check for template files in schemas/tcms/ (should be in schemas/templates/)
    for file in "$TCMS_SCHEMAS_DIR"/*; do
        if [ -f "$file" ]; then
            local basename_file
            basename_file="$(basename "$file")"
            if [[ "$basename_file" =~ template ]] || [[ "$basename_file" =~ \.j2$ ]] || [[ "$basename_file" =~ \.adoc$ ]]; then
                log_warning "Template file found in schemas/tcms/: $basename_file"
                log_warning "  Templates should be in schemas/templates/"
                misplaced_found=1
            fi
        fi
    done
    
    # Check for template files in verification_methods subdirectories
    if [ -d "$TCMS_SCHEMAS_DIR/verification_methods" ]; then
        for subdir in "$TCMS_SCHEMAS_DIR/verification_methods"/*; do
            if [ -d "$subdir" ]; then
                for file in "$subdir"/*; do
                    if [ -f "$file" ]; then
                        local basename_file
                        basename_file="$(basename "$file")"
                        if [[ "$basename_file" =~ template ]] || [[ "$basename_file" =~ \.j2$ ]] || [[ "$basename_file" =~ \.adoc$ ]]; then
                            log_warning "Template file found in schemas/tcms/verification_methods/: $file"
                            log_warning "  Templates should be in schemas/templates/verification_methods/"
                            misplaced_found=1
                        fi
                    fi
                done
            fi
        done
    fi
    
    # Verify templates directory exists
    if [ ! -d "$TEMPLATES_DIR" ]; then
        log_warning "schemas/templates/ directory not found"
        log_warning "  Template files should be stored in schemas/templates/"
        misplaced_found=1
    fi
    
    if [ "$misplaced_found" -eq 0 ]; then
        PASSED=$((PASSED + 1))
        log_success "All template files are properly located"
    else
        # Don't fail, just warn
        log_warning "Some template files may be misplaced (see warnings above)"
    fi
    
    return 0
}

# Validate that samples are in schemas/tcms/samples/
validate_samples_location() {
    log_info "Validating sample file locations..."
    
    local misplaced_found=0
    
    # Check for sample files in schemas/tcms/ root (should be in schemas/tcms/samples/)
    for file in "$TCMS_SCHEMAS_DIR"/*; do
        if [ -f "$file" ]; then
            local basename_file
            basename_file="$(basename "$file")"
            if [[ "$basename_file" =~ sample ]] || [[ "$basename_file" =~ \.yml$ ]] || [[ "$basename_file" =~ \.yaml$ ]]; then
                # Exclude documentation files
                if [[ ! "$basename_file" =~ \.md$ ]]; then
                    log_warning "Sample file found in schemas/tcms/: $basename_file"
                    log_warning "  Sample files should be in schemas/tcms/samples/"
                    misplaced_found=1
                fi
            fi
        fi
    done
    
    # Verify samples directory exists
    if [ ! -d "$TCMS_SAMPLES_DIR" ]; then
        log_warning "schemas/tcms/samples/ directory not found"
        log_warning "  Sample files should be stored in schemas/tcms/samples/"
        misplaced_found=1
    fi
    
    if [ "$misplaced_found" -eq 0 ]; then
        PASSED=$((PASSED + 1))
        log_success "All sample files are properly located"
    else
        # Don't fail, just warn
        log_warning "Some sample files may be misplaced (see warnings above)"
    fi
    
    return 0
}

# Main validation workflow
main() {
    log_info "Starting TCMS envelope schema validation"
    echo ""
    
    check_dependencies
    echo ""
    
    # Step 1: Validate schema organization
    log_info "Step 1: Validating schema organization"
    validate_tcms_organization
    echo ""
    
    # Step 2: Check for duplicate schemas
    log_info "Step 2: Checking for duplicate schemas"
    detect_duplicate_schemas
    echo ""
    
    # Step 3: Validate template locations
    log_info "Step 3: Validating template locations"
    validate_templates_location
    echo ""
    
    # Step 4: Validate sample locations
    log_info "Step 4: Validating sample locations"
    validate_samples_location
    echo ""
    
    # Step 5: Validate envelope meta-schema JSON syntax
    log_info "Step 5: Validating envelope meta-schema"
    validate_json_syntax "$ENVELOPE_SCHEMA"
    echo ""
    
    # Step 6: Validate all versioned schemas
    log_info "Step 6: Validating versioned schemas"
    
    # Define schema validation rules (using case statement for bash 3.2 compatibility)
    get_schema_validation_rule() {
        local filename="$1"
        case "$filename" in
            "test-case.schema.v1.json")
                echo "test_case:tcms/test-case.schema.v1.json"
                ;;
            "test-execution.schema.v1.json")
                echo "test_execution:tcms/test-execution.schema.v1.json"
                ;;
            "test-verification.schema.v1.json")
                echo "test_verification:tcms/test-verification.schema.v1.json"
                ;;
            "test-result.schema.v1.json")
                echo "test_result:tcms/test-result.schema.v1.json"
                ;;
            "container-config.schema.v1.json")
                echo "container_config:tcms/container-config.schema.v1.json"
                ;;
            "test-results-container.schema.v1.json")
                echo "test_results_container:tcms/test-results-container.schema.v1.json"
                ;;
            *)
                echo ""
                ;;
        esac
    }
    
    for schema_file in "$TCMS_SCHEMAS_DIR"/*.json; do
        if [ ! -f "$schema_file" ]; then
            continue
        fi
        
        filename="$(basename "$schema_file")"
        
        # Validate JSON syntax
        validate_json_syntax "$schema_file"
        
        # Validate envelope reference
        validate_envelope_reference "$schema_file"
        
        # Validate envelope constraints
        validation_rule="$(get_schema_validation_rule "$filename")"
        if [ -n "$validation_rule" ]; then
            IFS=':' read -r doc_type schema_path <<< "$validation_rule"
            validate_envelope_constraints "$schema_file" "$doc_type" "$schema_path"
            validate_sample_with_envelope "$schema_file" "$doc_type" "$schema_path"
        else
            log_warning "No validation rules defined for: $filename"
        fi
    done
    
    echo ""
    log_info "Step 7: Summary"
    echo ""
    
    # Print summary
    TOTAL=$((PASSED + FAILED))
    echo "=========================================="
    echo "Validation Summary"
    echo "=========================================="
    echo "Total checks:  $TOTAL"
    echo -e "Passed:        ${GREEN}$PASSED${NC}"
    echo -e "Failed:        ${RED}$FAILED${NC}"
    echo -e "Warnings:      ${YELLOW}$WARNINGS${NC}"
    echo "=========================================="
    echo ""
    
    if [ "$FAILED" -eq 0 ]; then
        if [ "$WARNINGS" -gt 0 ]; then
            log_warning "All validations passed but $WARNINGS warning(s) were issued"
        else
            log_success "All envelope schema validations passed!"
        fi
        return 0
    else
        log_error "$FAILED validation(s) failed"
        return 1
    fi
}

# Run main function
main "$@"
