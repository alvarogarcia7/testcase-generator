# Schema Validation Implementation

## Summary

Added comprehensive schema validation to the TPDG conversion workflow to ensure data integrity before and after conversion.

## Changes Implemented

### Stage 0: Input Validation (Before Processing)

**Purpose**: Validate all input test case YAML files against the test case schema before any processing begins.

**Implementation**:
- Added new Stage 0 before script generation
- Uses `validate-yaml` binary with `schemas/test-case.schema.json`
- Validates only files with `type: test_case`
- Reports success/failed/skipped counts
- **Fails early**: Exits with error if any validation fails, preventing further processing of invalid inputs

**Benefits**:
- Catches schema violations early in the pipeline
- Prevents wasted effort on invalid test cases
- Ensures all test cases conform to the schema before execution

### Stage 4: Output Validation (After Conversion)

**Purpose**: Validate the generated TPDG container YAML against its schema after conversion completes.

**Implementation**:
- Added new Stage 4 after TPDG conversion (Stage 3)
- Uses `validate-yaml` binary with `data/testcase_results_container/schema.json`
- Validates the generated `acceptance_test_results_container.yaml`
- Reports validation success or failure
- Sets exit code to 1 if output validation fails

**Benefits**:
- Ensures generated output conforms to expected schema
- Catches conversion bugs that produce invalid output
- Provides confidence that the container can be consumed by downstream tools

## File Changes

**File**: `scripts/run_tpdg_conversion.sh`

### New Variables

```bash
VALIDATE_YAML=$(find_binary "validate-yaml")
TEST_CASE_SCHEMA="$PROJECT_ROOT/schemas/test-case.schema.json"
CONTAINER_SCHEMA="$PROJECT_ROOT/data/testcase_results_container/schema.json"
```

### Prerequisites Check Additions

```bash
# Validate-yaml binary
if [[ ! -x "$VALIDATE_YAML" ]]; then
    echo_error "validate-yaml binary not found at: $VALIDATE_YAML"
    echo_info "Run: cargo build --bin validate-yaml"
    exit 1
fi

# Schema files
if [[ ! -f "$TEST_CASE_SCHEMA" ]]; then
    echo_error "Test case schema not found at: $TEST_CASE_SCHEMA"
    exit 1
fi

if [[ ! -f "$CONTAINER_SCHEMA" ]]; then
    echo_error "Container schema not found at: $CONTAINER_SCHEMA"
    exit 1
fi
```

### Stage 0: Input Validation

```bash
echo_section "Stage 0: Validating Input Test Cases"

# Find all test case YAML files
YAML_FILES=()
while IFS= read -r -d $'\0' file; do
    YAML_FILES+=("$file")
done < <(find "$TEST_CASE_DIR" -type f \( -name "*.yaml" -o -name "*.yml" \) -print0 | sort -z)

VALIDATION_SUCCESS=0
VALIDATION_FAILED=0
VALIDATION_SKIPPED=0

# Validate each test case
for yaml_file in "${YAML_FILES[@]}"; do
    if ! grep -q "^type: test_case" "$yaml_file" 2>/dev/null; then
        ((VALIDATION_SKIPPED++))
        info "$basename (not a test_case, skipped)"
        continue
    fi
    
    if "$VALIDATE_YAML" --schema "$TEST_CASE_SCHEMA" "$yaml_file" >> "$LOG_FILE" 2>&1; then
        ((VALIDATION_SUCCESS++))
        pass "$basename"
    else
        ((VALIDATION_FAILED++))
        fail "$basename"
        echo "$yaml_file" >> "$ERROR_LOG"
    fi
done

# Fail early if validation errors
if [[ $VALIDATION_FAILED -gt 0 ]]; then
    echo_error "Input validation failed for $VALIDATION_FAILED test case(s)"
    echo_error "Fix validation errors before proceeding"
    exit 1
fi
```

### Stage 4: Output Validation

```bash
echo_section "Stage 4: Validating Output Container"

OUTPUT_VALIDATION_SUCCESS=0
OUTPUT_VALIDATION_FAILED=0

if [[ -f "$OUTPUT_FILE" ]]; then
    if "$VALIDATE_YAML" --schema "$CONTAINER_SCHEMA" "$OUTPUT_FILE" >> "$LOG_FILE" 2>&1; then
        ((OUTPUT_VALIDATION_SUCCESS++))
        pass "acceptance_test_results_container.yaml"
        echo_success "Output container schema validation passed!"
    else
        ((OUTPUT_VALIDATION_FAILED++))
        fail "acceptance_test_results_container.yaml"
        echo_error "Output container schema validation failed!"
        CONVERSION_EXIT_CODE=1
    fi
else
    echo_warning "Output file not found, skipping validation"
fi
```

### Updated Summary

```bash
echo "Summary:"
echo "  Stage 0 - Input Validation:"
echo "    Success: $VALIDATION_SUCCESS"
echo "    Failed: $VALIDATION_FAILED"
echo "    Skipped: $VALIDATION_SKIPPED"
echo "  Stage 1 - Script Generation:"
echo "    Success: $GENERATION_SUCCESS"
echo "    Failed: $GENERATION_FAILED"
echo "    Skipped: $GENERATION_SKIPPED"
echo "  Stage 2 - Test Execution:"
echo "    Success: $EXECUTION_SUCCESS"
echo "    Failed: $EXECUTION_FAILED"
echo "    Skipped: $EXECUTION_SKIPPED"
echo "  Stage 3 - TPDG Conversion:"
echo "    Exit Code: $CONVERSION_EXIT_CODE"
echo "  Stage 4 - Output Validation:"
echo "    Success: $OUTPUT_VALIDATION_SUCCESS"
echo "    Failed: $OUTPUT_VALIDATION_FAILED"
```

## Workflow

The complete five-stage workflow is now:

1. **Stage 0: Input Validation** - Validate test case YAMLs against schema
2. **Stage 1: Script Generation** - Generate bash scripts from test cases
3. **Stage 2: Test Execution** - Execute scripts and capture logs
4. **Stage 3: TPDG Conversion** - Convert logs to TPDG container
5. **Stage 4: Output Validation** - Validate output against container schema

## Error Handling

### Stage 0 Failures
- **Action**: Exit immediately with error code 1
- **Rationale**: No point processing invalid inputs
- **User Action**: Fix schema violations in test case YAML files

### Stage 4 Failures
- **Action**: Set exit code to 1, log error
- **Rationale**: Conversion produced invalid output
- **User Action**: Check conversion script or schema compatibility

## Example Output

```
=== Stage 0: Validating Input Test Cases ===
[INFO] Found 89 YAML files to validate

✓ TC_SUCCESS_SIMPLE_001
✓ TC_SUCCESS_MULTI_SEQ_001
✓ TC_FAILURE_EXIT_CODE_MISMATCH_001
ℹ hooks_common (not a test_case, skipped)
...

[INFO] Input Validation: 76 passed, 0 failed, 13 skipped

=== Stage 1: Generating Test Scripts ===
...

=== Stage 2: Executing Test Scripts ===
...

=== Stage 3: Running TPDG Conversion ===
...

=== Stage 4: Validating Output Container ===
[INFO] Validating: test-acceptance/results/acceptance_test_results_container.yaml
✓ acceptance_test_results_container.yaml
✓ Output container schema validation passed!

=== Summary ===
  Stage 0 - Input Validation:
    Success: 76
    Failed: 0
    Skipped: 13
  Stage 1 - Script Generation:
    Success: 76
    Failed: 0
    Skipped: 13
  Stage 2 - Test Execution:
    Success: 67
    Failed: 0
    Skipped: 9
  Stage 3 - TPDG Conversion:
    Exit Code: 0
  Stage 4 - Output Validation:
    Success: 1
    Failed: 0
```

## Benefits

1. **Data Quality**: Ensures both inputs and outputs conform to schemas
2. **Early Detection**: Catches validation errors before expensive processing
3. **Pipeline Integrity**: Validates each stage of the conversion pipeline
4. **Debugging**: Clear indication when schema violations occur
5. **Confidence**: Provides assurance that data is structurally correct
6. **Documentation**: Schema serves as contract for data format

## Integration

This validation integrates seamlessly with:
- **CI/CD Pipelines**: Fails fast on schema violations
- **Development Workflow**: Catches errors during local testing
- **Quality Gates**: Ensures only valid data proceeds through pipeline
- **Downstream Tools**: Guarantees compatible output format

## Future Enhancements

Potential improvements:
- Verbose mode to show detailed validation errors
- Option to skip validation for faster development iterations
- Validation reports in JSON or HTML format
- Schema version compatibility checks
- Automatic schema update suggestions

## Conclusion

The addition of schema validation provides robust quality control for the TPDG conversion pipeline, ensuring data integrity from input test cases through to final output containers.
