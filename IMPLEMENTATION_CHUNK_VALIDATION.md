# Implementation: Chunk-Based Validation

## Overview
Modified `validate_chunk` in `src/validation.rs` to validate only the provided chunk of input data without requiring all fields to be present. This allows validating partial test case structures, such as metadata-only, conditions-only, or any subset of fields.

## Changes Implemented

### 1. Modified `validate_chunk()` Method
**Purpose**: Validate a chunk of input data without requiring all schema fields

**Key Changes**:
- No longer validates against the complete schema requiring all fields
- Validates each property in the chunk individually against its schema definition
- Allows partial data validation (e.g., only metadata fields)
- Maintains type and structure validation for provided fields

**Implementation**:
```rust
pub fn validate_chunk(&self, yaml_content: &str) -> Result<()>
```

**Behavior**:
- Parses the YAML content into a JSON object
- Iterates through each property in the chunk
- For each property, creates a mini-schema containing just that property definition
- Validates the property value against its schema
- Collects all validation errors and reports them together
- Does NOT require missing schema fields to be present

**Use Cases**:
- Validating metadata-only chunks: `requirement`, `item`, `tc`, `id`, `description`
- Validating conditions-only chunks: `general_initial_conditions` or `initial_conditions`
- Validating any partial subset of the test case structure
- Progressive validation during interactive test case building

### 2. Added `validate_complete()` Method
**Purpose**: Validate a complete test case document with all required fields

**Key Changes**:
- New method that performs full schema validation
- Requires all mandatory fields defined in the schema
- Uses the original validation logic that checks the entire structure

**Implementation**:
```rust
pub fn validate_complete(&self, yaml_content: &str) -> Result<()>
```

**Behavior**:
- Validates that ALL required fields are present
- Validates the complete structure against the full schema
- Reports missing required fields as errors
- Use this for final validation of complete test cases

**Use Cases**:
- Validating complete test case files before saving
- Final validation after all parts have been assembled
- Ensuring test cases meet all schema requirements

### 3. Updated `validate_partial_chunk()` Method
**Purpose**: Convenience method for validating chunks that might be empty

**Behavior**:
- Allows empty objects to pass validation
- Delegates to `validate_chunk()` for non-empty objects
- Useful when chunks are optional

### 4. Updated Tests
Added comprehensive tests to verify chunk validation:

**New Tests**:
- `test_validate_chunk_metadata_only` - Validates just metadata fields
- `test_validate_chunk_wrong_type` - Ensures type checking works for chunks
- `test_validate_chunk_single_field` - Validates a single field chunk
- `test_validate_chunk_multiple_fields` - Validates multiple fields (not all)
- `test_validate_chunk_general_initial_conditions_only` - Validates conditions chunk
- `test_validate_chunk_initial_conditions_only` - Validates conditions chunk

**Updated Tests**:
- `test_validate_complete_valid_yaml` - Now uses `validate_complete()`
- `test_validate_invalid_yaml_missing_required` - Now uses `validate_complete()`
- `test_validate_invalid_yaml_wrong_type` - Now uses `validate_complete()`

## Key Differences

### Before (Old Behavior)
```rust
// validate_chunk required ALL schema fields
validator.validate_chunk(yaml_with_only_metadata) // ❌ Would fail - missing required fields

// Could only validate complete documents
validator.validate_chunk(yaml_complete_document)  // ✓ Would pass
```

### After (New Behavior)
```rust
// validate_chunk now validates only the provided chunk
validator.validate_chunk(yaml_with_only_metadata) // ✓ Now passes - validates just metadata

// For complete documents, use validate_complete
validator.validate_complete(yaml_complete_document) // ✓ Validates with all requirements
```

## Usage Examples

### Example 1: Validate Metadata Only (prompt_metadata use case)
```rust
let metadata_yaml = r#"
requirement: XXX100
item: 1
tc: 4
id: 'TC001'
description: 'Test description'
"#;

// This now works! Validates only the metadata fields
validator.validate_chunk(metadata_yaml)?; // ✓ Success
```

### Example 2: Validate Conditions Only
```rust
let conditions_yaml = r#"
general_initial_conditions:
  - eUICC:
      - "Profile is loaded"
"#;

// Validates just the conditions structure
validator.validate_chunk(conditions_yaml)?; // ✓ Success
```

### Example 3: Progressive Validation
```rust
// Step 1: Validate metadata
validator.validate_chunk(metadata_chunk)?;

// Step 2: Validate conditions
validator.validate_chunk(conditions_chunk)?;

// Step 3: Validate complete document
validator.validate_complete(complete_yaml)?;
```

### Example 4: Single Field Validation
```rust
let single_field = r#"
requirement: XXX100
"#;

// Validates just the requirement field
validator.validate_chunk(single_field)?; // ✓ Success
```

## Integration with prompt_metadata

When using `prompt_metadata`, the validation now works correctly:

```rust
// In TestCaseMetadata::validate()
pub fn validate(&self, validator: &SchemaValidator) -> Result<()> {
    self.validate_recursive(validator, None)
}

// In validate_recursive()
pub fn validate_recursive(&self, validator: &SchemaValidator, attribute: Option<&str>) -> Result<()> {
    let yaml_map = // ... build YAML from metadata fields
    let yaml_str = serde_yaml::to_string(&yaml_value)?;
    
    // This now validates ONLY the metadata fields without requiring
    // general_initial_conditions, initial_conditions, or test_sequences
    validator.validate_partial_chunk(&yaml_str) // ✓ Works!
}
```

## Technical Implementation Details

### Property-by-Property Validation

The new `validate_chunk` implementation:

1. **Extracts properties** from the chunk
2. **Accesses the schema's property definitions**
3. **For each property in the chunk**:
   - Creates a mini-schema: `{ "type": "object", "properties": { "field": schema }, "required": ["field"] }`
   - Compiles the mini-schema
   - Validates just that property value
   - Collects any errors
4. **Reports all errors together**

### Schema Access
```rust
// Access the schema's properties definition
if let Some(JsonValue::Object(schema_obj)) = self.schema.schema().get("properties") {
    // Iterate through chunk properties
    for (key, value) in obj.iter() {
        // Get the schema definition for this property
        if let Some(property_schema) = schema_obj.get(key) {
            // Validate this property
        }
    }
}
```

### Error Handling
- Maintains the same error format as before
- Provides clear path information for validation errors
- Contextual error messages help identify issues

## Benefits

1. **Flexible Validation**: Validate any subset of fields
2. **Progressive Building**: Validate as you build the test case
3. **Better UX**: Don't require all fields upfront
4. **Metadata-Only Validation**: Specifically addresses the prompt_metadata requirement
5. **Backward Compatible**: `validate_partial_chunk` still works as before
6. **Complete Validation Available**: `validate_complete` for final checks
7. **Type Safety**: Still validates types and structure of provided fields

## Migration Guide

### If You Were Using `validate_chunk` for Complete Documents
**Before:**
```rust
validator.validate_chunk(complete_yaml)?;
```

**After:**
```rust
validator.validate_complete(complete_yaml)?;
```

### If You Were Using `validate_chunk` for Partial Data
**Before:**
```rust
// This would fail with missing required fields
validator.validate_chunk(metadata_only)?; // ❌
```

**After:**
```rust
// This now works correctly
validator.validate_chunk(metadata_only)?; // ✓
```

### Using validate_partial_chunk (No Changes Needed)
```rust
// Still works the same way
validator.validate_partial_chunk(yaml_content)?;
```

## Testing

All tests pass with the new implementation:
- ✓ Complete document validation
- ✓ Chunk validation (metadata only)
- ✓ Chunk validation (single field)
- ✓ Chunk validation (multiple fields)
- ✓ Chunk validation (conditions only)
- ✓ Type error detection in chunks
- ✓ Partial chunk validation (empty objects)
- ✓ Initial conditions validation

## Summary

The implementation successfully addresses the requirements:
1. ✅ `validate_chunk` now validates only the provided chunk, not the whole input
2. ✅ When using `prompt_metadata`, only metadata attributes are validated
3. ✅ No required fields enforced during chunk validation
4. ✅ Complete validation still available via `validate_complete()`
5. ✅ Comprehensive test coverage added
6. ✅ Backward compatibility maintained via `validate_partial_chunk()`
