# Validation Test Coverage

This document describes the comprehensive test coverage for the validation module fixes.

## Test Files

### 1. `src/validation.rs` - Unit Tests (30+ tests)

**Module-level tests** that verify the core validation logic:

#### Valid Payload Tests (Should Accept ✅)
- `test_schema_validator_creation` - Validator can be created
- `test_validate_complete_valid_yaml` - Complete valid YAML document
- `test_validate_chunk_metadata_only` - Metadata fields only
- `test_validate_chunk_single_field` - Single field validation
- `test_validate_chunk_multiple_fields` - Multiple fields (partial)
- `test_validate_chunk_general_initial_conditions_only` - General conditions only
- `test_validate_chunk_initial_conditions_only` - Initial conditions only
- `test_validate_chunk_test_sequences_with_three_sequences` - 3+ test sequences
- `test_validate_chunk_test_sequences_single_sequence_valid` - Single sequence
- `test_validate_chunk_manual_field_optional` - Steps without manual field
- `test_validate_chunk_success_field_optional_in_second_step` - Second step without success
- `test_validate_chunk_accepts_valid_complete_payload` - Valid complete payload
- `test_validate_chunk_accepts_additional_array_items` - 5+ items in array
- `test_validate_chunk_accepts_additional_steps` - 4+ steps in sequence
- `test_validate_chunk_initial_conditions_accepts_more_than_two_items` - 4+ eUICC items
- `test_validate_chunk_accepts_single_euicc_item` - Single eUICC condition
- `test_validate_chunk_empty_euicc_array` - Empty eUICC array
- `test_validate_chunk_general_initial_conditions_accepts_multiple_devices` - Multiple device types
- `test_validate_complete_empty_test_sequences` - Empty test_sequences array
- `test_validate_partial_chunk_empty` - Empty object
- `test_validate_initial_conditions_valid` - Valid initial conditions
- `test_validate_initial_conditions_multiple_devices` - Multiple devices
- `test_validate_initial_conditions_custom_device_types` - Custom device types
- `test_validate_initial_conditions_single_device` - Single device
- `test_validate_initial_conditions_empty_array_error` - Empty device array

#### Invalid Payload Tests (Should Reject ❌)
- `test_validate_invalid_yaml_missing_required` - Missing required fields
- `test_validate_invalid_yaml_wrong_type` - Wrong type (string for integer)
- `test_validate_chunk_wrong_type` - Chunk with wrong type
- `test_validate_chunk_test_sequences_missing_required_in_step` - Missing command
- `test_validate_chunk_test_sequences_missing_expected_fields` - Missing output
- `test_validate_chunk_general_initial_conditions_wrong_structure` - Non-array eUICC
- `test_validate_chunk_initial_conditions_eUICC_not_array` - eUICC not array
- `test_validate_chunk_initial_conditions_eUICC_items_not_strings` - Non-string items
- `test_validate_chunk_step_wrong_type_for_step_number` - String step number
- `test_validate_chunk_rejects_invalid_integer_type` - String for integer
- `test_validate_chunk_rejects_missing_required_sequence_fields` - Missing sequence fields
- `test_validate_initial_conditions_invalid_not_array` - Device value not array
- `test_validate_initial_conditions_invalid_not_strings` - Non-string conditions

### 2. `tests/validation_scenarios.rs` - Integration Tests (20 tests)

**End-to-end validation scenarios** that test real-world use cases:

#### Valid Scenarios (Should Accept ✅)
- `test_validator_accepts_valid_minimal_complete_document` - Minimal complete doc
- `test_validator_accepts_extra_test_sequences` - 4 test sequences
- `test_validator_accepts_extra_steps` - 5 steps
- `test_validator_accepts_extra_initial_conditions` - 5 conditions
- `test_validator_accepts_metadata_only` - Metadata chunk
- `test_validator_accepts_conditions_only` - Conditions chunk
- `test_validator_accepts_empty_arrays` - Empty arrays
- `test_validator_accepts_manual_optional` - Steps without manual
- `test_validator_accepts_success_optional_in_second_step` - Second step without success

#### Invalid Scenarios (Should Reject ❌)
- `test_validator_rejects_wrong_type_item` - String for item integer
- `test_validator_rejects_wrong_type_step` - String for step integer
- `test_validator_rejects_missing_required_command` - Missing command
- `test_validator_rejects_missing_expected_output` - Missing output
- `test_validator_rejects_non_array_euicc` - eUICC not array
- `test_validator_rejects_non_string_in_euicc_array` - Integer in eUICC array

## Coverage Summary

### By Feature

**Type Validation** (10 tests)
- String, integer, boolean, array, object types
- Type mismatches caught and reported
- Clear error messages for type violations

**Array Validation** (12 tests)
- Tuple validation with defined schemas
- Additional items allowed (Draft-04 behavior)
- Empty arrays accepted
- Array item type validation

**Object Validation** (8 tests)
- Required field checking
- Nested object validation
- Property schema validation
- Multiple device types

**Chunk Validation** (10 tests)
- Metadata-only chunks
- Conditions-only chunks
- Partial field validation
- Progressive validation support

**Complete Validation** (5 tests)
- Full document validation
- All required fields present
- Comprehensive structure validation

### By Schema Section

**Metadata Fields** (5 tests)
- requirement, item, tc, id, description
- Type constraints
- Partial validation

**General Initial Conditions** (5 tests)
- Array structure
- Device objects
- Multiple items
- Multiple devices

**Initial Conditions** (8 tests)
- Object structure
- eUICC array
- Multiple devices
- Item count flexibility

**Test Sequences** (10 tests)
- Array validation
- Required fields
- Multiple sequences
- Tuple validation

**Steps** (12 tests)
- Array validation
- Required fields
- Multiple steps
- Expected object validation
- Optional fields (manual, success)

## Key Test Patterns

### 1. Tuple Validation Tests

Tests that verify tuple validation allows additional items:

```rust
// Should accept more than schema-defined items
test_validator_accepts_extra_test_sequences    // 4 sequences (schema defines 2)
test_validator_accepts_extra_steps             // 5 steps (schema defines 2)
test_validator_accepts_extra_initial_conditions // 5 conditions (schema defines 2)
```

### 2. Type Constraint Tests

Tests that verify type checking works correctly:

```rust
// Should reject wrong types
test_validator_rejects_wrong_type_item         // string instead of integer
test_validator_rejects_wrong_type_step         // string instead of integer
test_validator_rejects_non_array_euicc         // string instead of array
test_validator_rejects_non_string_in_euicc_array // integer instead of string
```

### 3. Required Field Tests

Tests that verify required field validation:

```rust
// Should reject missing required fields
test_validator_rejects_missing_required_command  // missing 'command'
test_validator_rejects_missing_expected_output   // missing 'output'
test_validate_invalid_yaml_missing_required      // missing root fields
```

### 4. Optional Field Tests

Tests that verify optional fields are accepted:

```rust
// Should accept missing optional fields
test_validator_accepts_manual_optional                      // 'manual' is optional
test_validator_accepts_success_optional_in_second_step      // 'success' optional in step 2
```

### 5. Chunk Validation Tests

Tests that verify partial validation works:

```rust
// Should validate partial chunks
test_validator_accepts_metadata_only           // only metadata fields
test_validator_accepts_conditions_only         // only condition fields
test_validate_chunk_single_field              // single field
```

## Error Message Tests

All rejection tests verify that error messages:
- Contain relevant field names
- Describe the validation failure
- Include path information
- Are human-readable

Example error assertions:
```rust
assert!(error.contains("Invalid type") || error.contains("integer"));
assert!(error.contains("command") || error.contains("required"));
assert!(error.contains("output") || error.contains("required"));
```

## Running Tests

### Run all validation tests
```bash
cargo test validation
```

### Run unit tests only
```bash
cargo test --lib validation
```

### Run integration tests only
```bash
cargo test --test validation_scenarios
```

### Run specific test
```bash
cargo test test_validator_accepts_extra_test_sequences
```

### Run with output
```bash
cargo test validation -- --nocapture
```

## Test Maintenance

When adding new schema constraints:

1. Add positive test (should accept valid data)
2. Add negative test (should reject invalid data)
3. Verify error messages are clear
4. Document the test in this file

When modifying schema:

1. Review existing tests
2. Update tests that depend on modified constraints
3. Add new tests for new constraints
4. Ensure backward compatibility where applicable

## Known Edge Cases

### Tuple Validation
- Schema defines 2 items, but allows any number
- This is correct Draft-04 behavior
- Tests verify this with 3+, 4+, 5+ item tests

### Optional Fields
- `manual` field is optional in all steps
- `success` field is optional in step position 1+ (required in position 0)
- Tests verify both positions

### Empty Arrays
- Empty arrays are valid
- Tests verify this doesn't cause errors

### Device Types
- Schema defines eUICC but supports any device name
- Tests verify multiple device types work
- LPA, SM_DP_PLUS, etc. are valid

## Validation Behavior Summary

| Scenario | Should Accept | Tests |
|----------|--------------|-------|
| Complete valid document | ✅ Yes | 3 |
| Metadata only | ✅ Yes | 3 |
| Conditions only | ✅ Yes | 2 |
| 3+ test sequences | ✅ Yes | 2 |
| 3+ steps | ✅ Yes | 2 |
| 3+ eUICC items | ✅ Yes | 2 |
| Empty arrays | ✅ Yes | 2 |
| Missing optional fields | ✅ Yes | 3 |
| Wrong type (str for int) | ❌ No | 4 |
| Missing required fields | ❌ No | 5 |
| Non-array eUICC | ❌ No | 2 |
| Non-string in array | ❌ No | 2 |

**Total Coverage: 50+ test cases**
