# Implementation: Recursive Attribute Validation

## Overview
Enhanced the `TestCaseMetadata` validation in `src/prompts.rs` to support recursive validation with a variable to indicate which attribute to validate at each step.

## Implementation Details

### New Methods Added to `TestCaseMetadata`

#### 1. `validate_recursive(&self, validator: &SchemaValidator, attribute: Option<&str>) -> Result<()>`
The core method that enables selective attribute validation.

**Parameters:**
- `validator`: The schema validator to use
- `attribute`: Optional attribute name to validate
  - `None`: Validates all attributes (default behavior)
  - `Some("attribute_name")`: Validates only the specified attribute

**Supported Attributes:**
- `"requirement"`
- `"item"`
- `"tc"`
- `"id"`
- `"description"`

**Usage Examples:**
```rust
// Validate all attributes
metadata.validate_recursive(validator, None)?;

// Validate only the 'requirement' attribute
metadata.validate_recursive(validator, Some("requirement"))?;

// Validate attributes recursively one by one
for attr in &["requirement", "item", "tc", "id", "description"] {
    metadata.validate_recursive(validator, Some(attr))?;
}
```

**Error Handling:**
- Returns error with context about which attribute failed
- Returns error for unknown attribute names

#### 2. `validate_all_attributes_recursively(&self, validator: &SchemaValidator) -> Result<()>`
Convenience method that validates all attributes one at a time in sequence.

**Purpose:**
- Useful for identifying exactly which attribute is causing validation failures
- Provides clear error messages with attribute context

**Usage:**
```rust
metadata.validate_all_attributes_recursively(validator)?;
```

#### 3. `get_validatable_attributes() -> Vec<&'static str>`
Static method that returns the list of all attributes that can be validated.

**Returns:**
```rust
vec!["requirement", "item", "tc", "id", "description"]
```

**Usage:**
```rust
let attributes = TestCaseMetadata::get_validatable_attributes();
for attr in attributes {
    println!("Validating attribute: {}", attr);
    metadata.validate_recursive(validator, Some(attr))?;
}
```

### Modified Method

#### `validate(&self, validator: &SchemaValidator) -> Result<()>`
The original `validate` method now delegates to `validate_recursive` with `None` as the attribute parameter, maintaining backward compatibility.

## Key Features

### 1. Recursive Validation
- Validates one attribute at a time when specified
- Allows stepping through attributes sequentially
- Maintains state between validations

### 2. Variable Injection
- The `attribute` parameter acts as the injected variable
- Controls which attribute is validated at each step
- Enables conditional validation flows

### 3. Granular Error Reporting
- Each attribute validation provides specific error context
- Easier to identify which attribute is causing issues
- Improves debugging experience

### 4. Backward Compatibility
- Original `validate()` method still works exactly as before
- No breaking changes to existing code
- New functionality is opt-in via `validate_recursive()`

## Use Cases

### Use Case 1: Progressive Validation
Validate attributes one at a time with user feedback:
```rust
for attribute in TestCaseMetadata::get_validatable_attributes() {
    println!("Validating {}...", attribute);
    
    match metadata.validate_recursive(validator, Some(attribute)) {
        Ok(_) => println!("✓ {} is valid", attribute),
        Err(e) => {
            println!("✗ {} validation failed: {}", attribute, e);
            // Optionally prompt user to fix this attribute
        }
    }
}
```

### Use Case 2: Selective Re-validation
After user edits a specific field, re-validate only that field:
```rust
// User edited the 'item' field
let new_item = prompt_for_item()?;
metadata.item = new_item;

// Validate only the changed attribute
metadata.validate_recursive(validator, Some("item"))?;
```

### Use Case 3: Conditional Validation
Skip validation of certain attributes based on conditions:
```rust
let attributes_to_validate = if include_optional {
    vec!["requirement", "item", "tc", "id", "description"]
} else {
    vec!["requirement", "item", "tc"]
};

for attr in attributes_to_validate {
    metadata.validate_recursive(validator, Some(attr))?;
}
```

### Use Case 4: Error Recovery Workflow
Continue validation even after errors, collecting all failures:
```rust
let mut errors = Vec::new();

for attribute in TestCaseMetadata::get_validatable_attributes() {
    if let Err(e) = metadata.validate_recursive(validator, Some(attribute)) {
        errors.push((attribute, e));
    }
}

if !errors.is_empty() {
    println!("Validation failed for {} attributes:", errors.len());
    for (attr, error) in errors {
        println!("  - {}: {}", attr, error);
    }
}
```

## Testing

Comprehensive tests added to verify the new functionality:

1. **`test_validate_recursive_single_attribute`**: Validates each attribute individually
2. **`test_validate_recursive_all_attributes`**: Validates all attributes at once (None parameter)
3. **`test_validate_recursive_unknown_attribute`**: Ensures error for unknown attributes
4. **`test_validate_all_attributes_recursively`**: Tests the convenience method
5. **`test_get_validatable_attributes`**: Verifies the attribute list is correct
6. **`test_validate_recursive_iterative`**: Tests iterating through all attributes

## Technical Implementation

### Attribute-Specific Validation
When an attribute is specified, the method creates a minimal YAML structure containing only that attribute:

```rust
let yaml_map = match attribute {
    Some("requirement") => {
        // Create map with only requirement field
        single_attr_map.insert("requirement", Value::String(self.requirement.clone()));
    }
    // ... other attributes
    None => {
        // Use full YAML structure
        self.to_yaml()
    }
};
```

### Context-Aware Error Messages
Errors include context about which attribute failed:

```rust
validator.validate_partial_chunk(&yaml_str)
    .context(match attribute {
        Some(attr) => format!("Validation failed for attribute '{}'", attr),
        None => "Validation failed for metadata".to_string(),
    })
```

## Benefits

1. **Improved Debugging**: Pinpoint exactly which attribute is invalid
2. **Better UX**: Can provide targeted feedback to users
3. **Flexible Workflows**: Support progressive and conditional validation
4. **Error Recovery**: Continue validation after individual failures
5. **Backward Compatible**: No changes required to existing code
6. **Well-Tested**: Comprehensive test coverage for all scenarios

## Files Modified

- **src/prompts.rs**: 
  - Enhanced `TestCaseMetadata` with 3 new methods
  - Modified `validate()` to delegate to `validate_recursive()`
  - Added 7 new test cases
  - Added ~150 lines of implementation and tests
