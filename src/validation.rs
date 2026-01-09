use crate::models::ValidationErrorDetail;
use anyhow::{Context, Result};
use jsonschema::JSONSchema;
use serde_json::Value as JsonValue;
use std::fs;
use std::path::Path;

pub struct SchemaValidator {
    schema: JSONSchema,
    schema_value: JsonValue,
}

impl SchemaValidator {
    pub fn new() -> Result<Self> {
        let schema_path = Path::new("data/schema.json");
        let schema_content =
            fs::read_to_string(schema_path).context(format!("Failed to read {:?}", schema_path))?;

        let schema_value: JsonValue =
            serde_json::from_str(&schema_content).context("Failed to parse schema file")?;

        let schema = JSONSchema::compile(&schema_value)
            .map_err(|e| anyhow::anyhow!("Failed to compile JSON schema: {}", e))?;

        Ok(Self {
            schema,
            schema_value,
        })
    }

    /// Validate a chunk of input data against the schema
    ///
    /// This method validates only the provided chunk without requiring all fields.
    /// It validates the provided properties against the schema's definitions
    /// with proper handling of arrays, nested objects, and type constraints.
    ///
    /// # Arguments
    /// * `yaml_content` - YAML string containing the chunk to validate
    ///
    /// # Returns
    /// * `Ok(())` if the chunk is valid according to the schema
    /// * `Err` with validation errors if the chunk is invalid
    pub fn validate_chunk(&self, yaml_content: &str) -> Result<()> {
        let yaml_value: serde_yaml::Value =
            serde_yaml::from_str(yaml_content).context("Failed to parse YAML content")?;

        let json_value: JsonValue =
            serde_json::to_value(&yaml_value).context("Failed to convert YAML to JSON")?;

        if let JsonValue::Object(obj) = &json_value {
            let mut errors = Vec::new();

            if let Some(JsonValue::Object(schema_obj)) = self.schema_value.get("properties") {
                for (key, value) in obj.iter() {
                    if let Some(property_schema) = schema_obj.get(key) {
                        if let Err(validation_errors) =
                            self.validate_value(value, property_schema, key)
                        {
                            errors.extend(validation_errors);
                        }
                    }
                }
            }

            if !errors.is_empty() {
                anyhow::bail!("Schema validation failed:\n{}", errors.join("\n"));
            }
        }

        Ok(())
    }

    /// Validate a single value against its schema definition
    fn validate_value(
        &self,
        value: &JsonValue,
        schema: &JsonValue,
        path: &str,
    ) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Check type constraint
        if let Some(expected_type) = schema.get("type") {
            if let Some(type_str) = expected_type.as_str() {
                let type_matches = match type_str {
                    "string" => value.is_string(),
                    "integer" => value.is_i64() || value.is_u64(),
                    "number" => value.is_f64() || value.is_i64() || value.is_u64(),
                    "boolean" => value.is_boolean(),
                    "array" => value.is_array(),
                    "object" => value.is_object(),
                    "null" => value.is_null(),
                    _ => true,
                };

                if !type_matches {
                    errors.push(format!(
                        "  - Path '{}': Invalid type (expected {}, got {})",
                        path,
                        type_str,
                        get_value_type(value)
                    ));
                    return Err(errors);
                }
            }
        }

        // Validate array items
        if let JsonValue::Array(arr) = value {
            if let Some(items_schema) = schema.get("items") {
                // Handle tuple validation (items as array) vs single schema
                if let JsonValue::Array(item_schemas) = items_schema {
                    // Tuple validation: each position has its own schema
                    for (idx, item) in arr.iter().enumerate() {
                        // In JSON Schema Draft-04, if items is an array, additional items are allowed
                        // We only validate against the schema if one is defined for this position
                        if let Some(item_schema) = item_schemas.get(idx) {
                            let item_path = format!("{}[{}]", path, idx);
                            if let Err(item_errors) =
                                self.validate_value(item, item_schema, &item_path)
                            {
                                errors.extend(item_errors);
                            }
                        }
                    }
                } else {
                    // Single schema for all items
                    for (idx, item) in arr.iter().enumerate() {
                        let item_path = format!("{}[{}]", path, idx);
                        if let Err(item_errors) =
                            self.validate_value(item, items_schema, &item_path)
                        {
                            errors.extend(item_errors);
                        }
                    }
                }
            }
        }

        // Validate object properties
        if let JsonValue::Object(obj) = value {
            if let Some(JsonValue::Object(properties)) = schema.get("properties") {
                // Validate each property in the object
                for (key, val) in obj.iter() {
                    if let Some(prop_schema) = properties.get(key) {
                        let prop_path = format!("{}.{}", path, key);
                        if let Err(prop_errors) = self.validate_value(val, prop_schema, &prop_path)
                        {
                            errors.extend(prop_errors);
                        }
                    }
                }
            }

            // Validate additionalProperties if defined
            if let Some(additional_props_schema) = schema.get("additionalProperties") {
                let defined_properties = schema
                    .get("properties")
                    .and_then(|p| p.as_object())
                    .map(|obj| obj.keys().collect::<std::collections::HashSet<_>>())
                    .unwrap_or_default();

                for (key, val) in obj.iter() {
                    if !defined_properties.contains(key) {
                        let prop_path = format!("{}.{}", path, key);
                        if let Err(prop_errors) =
                            self.validate_value(val, additional_props_schema, &prop_path)
                        {
                            errors.extend(prop_errors);
                        }
                    }
                }
            }

            // Check required fields
            if let Some(JsonValue::Array(required)) = schema.get("required") {
                for req_field in required {
                    if let Some(field_name) = req_field.as_str() {
                        if !obj.contains_key(field_name) {
                            errors.push(format!(
                                "  - Path '{}': Missing required property '{}'",
                                path, field_name
                            ));
                        }
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Validate a complete test case document against the schema
    ///
    /// This method validates that all required fields are present and valid.
    /// Use this for complete test case documents.
    ///
    /// # Arguments
    /// * `yaml_content` - YAML string containing the complete document
    ///
    /// # Returns
    /// * `Ok(())` if the complete document is valid
    /// * `Err` with validation errors if the document is invalid or missing required fields
    pub fn validate_complete(&self, yaml_content: &str) -> Result<()> {
        let yaml_value: serde_yaml::Value =
            serde_yaml::from_str(yaml_content).context("Failed to parse YAML content")?;

        let json_value: JsonValue =
            serde_json::to_value(&yaml_value).context("Failed to convert YAML to JSON")?;

        let validation_result = self.schema.validate(&json_value);

        if let Err(errors) = validation_result {
            let error_messages: Vec<String> = errors
                .map(|e| {
                    let path = if e.instance_path.to_string().is_empty() {
                        "root".to_string()
                    } else {
                        e.instance_path.to_string()
                    };
                    format!("  - Path '{}': {}", path, format_validation_error(&e))
                })
                .collect();

            anyhow::bail!("Schema validation failed:\n{}", error_messages.join("\n"));
        }

        Ok(())
    }

    /// Validate and return detailed error information
    ///
    /// This method validates YAML content and returns structured error details
    /// that can be used for detailed error reporting.
    ///
    /// # Arguments
    /// * `yaml_content` - YAML string to validate
    ///
    /// # Returns
    /// * `Ok(())` if valid
    /// * `Err` with structured validation error details
    pub fn validate_with_details(&self, yaml_content: &str) -> Result<Vec<ValidationErrorDetail>> {
        let yaml_value: serde_yaml::Value =
            serde_yaml::from_str(yaml_content).context("Failed to parse YAML content")?;

        let json_value: JsonValue =
            serde_json::to_value(&yaml_value).context("Failed to convert YAML to JSON")?;

        let validation_result = self.schema.validate(&json_value);

        if let Err(errors) = validation_result {
            let mut error_details = Vec::new();

            for e in errors {
                let path = if e.instance_path.to_string().is_empty() {
                    "root".to_string()
                } else {
                    e.instance_path.to_string()
                };

                let error_str = e.to_string();
                let (constraint, expected_constraint) = extract_constraint_info(&error_str);

                let found_value = extract_instance_value(&json_value, &e.instance_path.to_string());

                error_details.push(ValidationErrorDetail {
                    path,
                    constraint,
                    found_value,
                    expected_constraint,
                });
            }

            return Ok(error_details);
        }

        Ok(Vec::new())
    }

    /// Validate a partial chunk, allowing empty objects
    ///
    /// This is a convenience method that allows empty objects to pass validation.
    /// For non-empty objects, it delegates to validate_chunk.
    ///
    /// # Arguments
    /// * `yaml_content` - YAML string containing the partial chunk
    ///
    /// # Returns
    /// * `Ok(())` if the chunk is valid or empty
    /// * `Err` with validation errors if the chunk is invalid
    pub fn validate_partial_chunk(&self, yaml_content: &str) -> Result<()> {
        let yaml_value: serde_yaml::Value =
            serde_yaml::from_str(yaml_content).context("Failed to parse YAML content")?;

        let json_value: JsonValue =
            serde_json::to_value(&yaml_value).context("Failed to convert YAML to JSON")?;

        if let JsonValue::Object(obj) = &json_value {
            if obj.is_empty() {
                return Ok(());
            }
        }

        self.validate_chunk(yaml_content)
    }

    pub fn validate_initial_conditions(
        &self,
        initial_conditions: &serde_yaml::Value,
    ) -> Result<()> {
        let json_value: JsonValue =
            serde_json::to_value(initial_conditions).context("Failed to convert to JSON")?;

        if let JsonValue::Object(obj) = &json_value {
            for (device_name, conditions) in obj.iter() {
                if !conditions.is_array() {
                    anyhow::bail!(
                        "Device '{}' must have an array of conditions, got: {:?}",
                        device_name,
                        conditions
                    );
                }

                if let JsonValue::Array(arr) = conditions {
                    for (idx, item) in arr.iter().enumerate() {
                        if !item.is_string() {
                            anyhow::bail!(
                                "Condition #{} for device '{}' must be a string, got: {:?}",
                                idx + 1,
                                device_name,
                                item
                            );
                        }
                    }
                }
            }
        } else {
            anyhow::bail!("initial_conditions must be an object with device names as keys");
        }

        Ok(())
    }
}

impl Default for SchemaValidator {
    fn default() -> Self {
        Self::new().expect("Failed to create default SchemaValidator")
    }
}

fn get_value_type(value: &JsonValue) -> &'static str {
    match value {
        JsonValue::String(_) => "string",
        JsonValue::Number(n) if n.is_f64() => "number",
        JsonValue::Number(_) => "integer",
        JsonValue::Bool(_) => "boolean",
        JsonValue::Array(_) => "array",
        JsonValue::Object(_) => "object",
        JsonValue::Null => "null",
    }
}

fn format_validation_error(error: &jsonschema::ValidationError) -> String {
    let error_str = error.to_string();

    if error_str.contains("is not of type") {
        return format!("Invalid type ({})", error_str);
    }

    if error_str.contains("is a required property") {
        if let Some(prop) = error_str.split('\'').nth(1) {
            return format!("Missing required property '{}'", prop);
        }
    }

    error_str
}

/// Extract constraint information from a validation error message
fn extract_constraint_info(error_str: &str) -> (String, String) {
    if error_str.contains("is not of type") {
        let parts: Vec<&str> = error_str.split("is not of type").collect();
        if parts.len() == 2 {
            let expected_type = parts[1].trim().trim_matches('\'').trim_matches('"');
            return (
                "type_mismatch".to_string(),
                format!("Expected type: {}", expected_type),
            );
        }
    }

    if error_str.contains("is a required property") {
        if let Some(prop) = error_str.split('\'').nth(1) {
            return (
                "missing_property".to_string(),
                format!("Required property '{}' is missing", prop),
            );
        }
    }

    if error_str.contains("is not valid under any of the given schemas") {
        return (
            "oneOf_validation".to_string(),
            "Value does not match any of the allowed schemas".to_string(),
        );
    }

    if error_str.contains("does not match pattern") {
        return ("pattern_mismatch".to_string(), error_str.to_string());
    }

    if error_str.contains("is less than the minimum") {
        return ("minimum_value".to_string(), error_str.to_string());
    }

    if error_str.contains("is greater than the maximum") {
        return ("maximum_value".to_string(), error_str.to_string());
    }

    ("validation_error".to_string(), error_str.to_string())
}

/// Extract the actual value at a given JSON path
fn extract_instance_value(json_value: &JsonValue, path: &str) -> String {
    if path.is_empty() || path == "/" {
        return format!("{}", json_value);
    }

    let path_parts: Vec<&str> = path.trim_start_matches('/').split('/').collect();
    let mut current = json_value;

    for part in path_parts {
        if let Ok(index) = part.parse::<usize>() {
            if let Some(arr) = current.as_array() {
                if let Some(value) = arr.get(index) {
                    current = value;
                } else {
                    return format!("<index {} out of bounds>", index);
                }
            } else {
                return format!("<not an array at {}>", part);
            }
        } else if let Some(obj) = current.as_object() {
            if let Some(value) = obj.get(part) {
                current = value;
            } else {
                return format!("<missing field '{}'>", part);
            }
        } else {
            return format!("<cannot access field '{}'>", part);
        }
    }

    match current {
        JsonValue::String(s) => format!("\"{}\"", s),
        JsonValue::Number(n) => format!("{}", n),
        JsonValue::Bool(b) => format!("{}", b),
        JsonValue::Null => "null".to_string(),
        JsonValue::Array(_) => "[array]".to_string(),
        JsonValue::Object(_) => "{object}".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_validator_creation() {
        let validator = SchemaValidator::new();
        assert!(validator.is_ok());
    }

    #[test]
    fn test_validate_complete_valid_yaml() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
requirement: XXX100
item: 1
tc: 4
id: '4.2.2.2.1 TC_eUICC_ES6.UpdateMetadata'
description: 'Test description'
general_initial_conditions:
  eUICC:
    - "Some condition"
initial_conditions:
  eUICC:
    - "Condition 1"
    - "Condition 2"
test_sequences:
  - id: 1
    name: "Test Sequence"
    description: "Test description"
    initial_conditions:
      eUICC:
        - "Condition"
    steps:
      - step: 1
        manual: true
        description: "Step description"
        command: "ssh"
        expected:
          success: true
          result: "SW=0x9000"
          output: "Success"
      - step: 2
        description: "Step 2 description"
        command: "ssh"
        expected:
          result: "SW=0x9000"
          output: "Success"
  - id: 2
    name: "Test Sequence 2"
    description: "Test description 2"
    initial_conditions:
      eUICC:
        - "Condition"
    steps:
      - step: 1
        manual: false
        description: "Step description"
        command: "ssh"
        expected:
          success: false
          result: "SW=0x9000"
          output: "Success"
      - step: 2
        description: "Step 2 description"
        command: "ssh"
        expected:
          result: "SW=0x9000"
          output: "Success"
"#;

        let result = validator.validate_complete(yaml_content);
        assert!(
            result.is_ok(),
            "Validation should succeed: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_validate_invalid_yaml_missing_required() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
requirement: XXX100
item: 1
"#;

        let result = validator.validate_complete(yaml_content);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Missing required property") || error_msg.contains("required"));
    }

    #[test]
    fn test_validate_chunk_metadata_only() {
        let validator = SchemaValidator::new().unwrap();

        // Validate only metadata fields as a chunk (not requiring all other fields)
        let yaml_content = r#"
requirement: XXX100
item: 1
tc: 4
id: 'TC001'
description: 'Test description'
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(
            result.is_ok(),
            "Chunk validation should succeed for metadata only: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_validate_invalid_yaml_wrong_type() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
requirement: XXX100
item: "not_an_integer"
tc: 4
id: '4.2.2.2.1'
description: 'Test'
general_initial_conditions:
  eUICC:
    - "Condition"
initial_conditions:
  eUICC:
    - "Condition 1"
    - "Condition 2"
test_sequences:
  - id: 1
    name: "Test"
    description: "Test"
    initial_conditions:
      eUICC:
        - "Condition"
    steps:
      - step: 1
        description: "Step"
        command: "ssh"
        expected:
          success: true
          result: "SW=0x9000"
          output: "Success"
      - step: 2
        description: "Step 2"
        command: "ssh"
        expected:
          result: "SW=0x9000"
          output: "Success"
  - id: 2
    name: "Test 2"
    description: "Test 2"
    initial_conditions:
      eUICC:
        - "Condition"
    steps:
      - step: 1
        description: "Step"
        command: "ssh"
        expected:
          success: true
          result: "SW=0x9000"
          output: "Success"
      - step: 2
        description: "Step 2"
        command: "ssh"
        expected:
          result: "SW=0x9000"
          output: "Success"
"#;

        let result = validator.validate_complete(yaml_content);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Invalid type") || error_msg.contains("type"));
    }

    #[test]
    fn test_validate_chunk_wrong_type() {
        let validator = SchemaValidator::new().unwrap();

        // Test that chunk validation catches type errors
        let yaml_content = r#"
requirement: XXX100
item: "not_an_integer"
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("Invalid type")
                || error_msg.contains("type")
                || error_msg.contains("item")
        );
    }

    #[test]
    fn test_validate_partial_chunk_empty() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = "{}";

        let result = validator.validate_partial_chunk(yaml_content);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_initial_conditions_valid() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
eUICC:
  - "Condition 1"
  - "Condition 2"
"#;
        let initial_conditions: serde_yaml::Value = serde_yaml::from_str(yaml_content).unwrap();

        let result = validator.validate_initial_conditions(&initial_conditions);
        assert!(
            result.is_ok(),
            "Validation should succeed: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_validate_initial_conditions_invalid_not_array() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
eUICC: "not an array"
"#;
        let initial_conditions: serde_yaml::Value = serde_yaml::from_str(yaml_content).unwrap();

        let result = validator.validate_initial_conditions(&initial_conditions);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("must have an array"));
    }

    #[test]
    fn test_validate_initial_conditions_invalid_not_strings() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
eUICC:
  - "Valid string"
  - 123
"#;
        let initial_conditions: serde_yaml::Value = serde_yaml::from_str(yaml_content).unwrap();

        let result = validator.validate_initial_conditions(&initial_conditions);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("must be a string"));
    }

    #[test]
    fn test_validate_initial_conditions_multiple_devices() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
eUICC:
  - "Condition 1"
  - "Condition 2"
LPA:
  - "LPA Condition 1"
"#;
        let initial_conditions: serde_yaml::Value = serde_yaml::from_str(yaml_content).unwrap();

        let result = validator.validate_initial_conditions(&initial_conditions);
        assert!(
            result.is_ok(),
            "Validation should succeed: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_validate_initial_conditions_custom_device_types() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
eUICC:
  - "eUICC Condition 1"
LPA:
  - "LPA Condition 1"
  - "LPA Condition 2"
SM_DP_PLUS:
  - "SM-DP+ Condition 1"
"#;
        let initial_conditions: serde_yaml::Value = serde_yaml::from_str(yaml_content).unwrap();

        let result = validator.validate_initial_conditions(&initial_conditions);
        assert!(
            result.is_ok(),
            "Validation should succeed with custom device types: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_validate_initial_conditions_single_device() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
LPA:
  - "LPA Condition 1"
"#;
        let initial_conditions: serde_yaml::Value = serde_yaml::from_str(yaml_content).unwrap();

        let result = validator.validate_initial_conditions(&initial_conditions);
        assert!(
            result.is_ok(),
            "Validation should succeed with single device: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_validate_initial_conditions_empty_array_error() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
eUICC: []
"#;
        let initial_conditions: serde_yaml::Value = serde_yaml::from_str(yaml_content).unwrap();

        let result = validator.validate_initial_conditions(&initial_conditions);
        assert!(result.is_ok(), "Empty array should be valid");
    }

    #[test]
    fn test_validate_chunk_single_field() {
        let validator = SchemaValidator::new().unwrap();

        // Test validating just one field
        let yaml_content = r#"
requirement: XXX100
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(
            result.is_ok(),
            "Should validate single field chunk: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_validate_chunk_multiple_fields() {
        let validator = SchemaValidator::new().unwrap();

        // Test validating multiple fields but not all
        let yaml_content = r#"
requirement: XXX100
item: 1
tc: 4
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(
            result.is_ok(),
            "Should validate multiple field chunk: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_validate_chunk_general_initial_conditions_only() {
        let validator = SchemaValidator::new().unwrap();

        // Test validating just general_initial_conditions
        let yaml_content = r#"
general_initial_conditions:
  eUICC:
    - "Condition 1"
    - "Condition 2"
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(
            result.is_ok(),
            "Should validate general_initial_conditions chunk: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_validate_chunk_initial_conditions_only() {
        let validator = SchemaValidator::new().unwrap();

        // Test validating just initial_conditions
        let yaml_content = r#"
initial_conditions:
  eUICC:
    - "Condition 1"
    - "Condition 2"
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(
            result.is_ok(),
            "Should validate initial_conditions chunk: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_validate_chunk_test_sequences_with_three_sequences() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
test_sequences:
  - id: 1
    name: "Sequence 1"
    description: "First sequence"
    initial_conditions:
      eUICC:
        - "Condition 1"
    steps:
      - step: 1
        manual: true
        description: "Step 1"
        command: "cmd1"
        expected:
          success: true
          result: "OK"
          output: "Success"
      - step: 2
        description: "Step 2"
        command: "cmd2"
        expected:
          result: "OK"
          output: "Success"
  - id: 2
    name: "Sequence 2"
    description: "Second sequence"
    initial_conditions:
      eUICC:
        - "Condition 2"
    steps:
      - step: 1
        manual: false
        description: "Step 1"
        command: "cmd1"
        expected:
          success: false
          result: "OK"
          output: "Success"
      - step: 2
        description: "Step 2"
        command: "cmd2"
        expected:
          result: "OK"
          output: "Success"
  - id: 3
    name: "Sequence 3"
    description: "Third sequence"
    initial_conditions:
      eUICC:
        - "Condition 3"
    steps:
      - step: 1
        description: "Step 1"
        command: "cmd1"
        expected:
          success: true
          result: "OK"
          output: "Success"
      - step: 2
        description: "Step 2"
        command: "cmd2"
        expected:
          result: "OK"
          output: "Success"
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(
            result.is_ok(),
            "Should accept 3+ test sequences: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_validate_chunk_test_sequences_missing_required_in_step() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
test_sequences:
  - id: 1
    name: "Test"
    description: "Test"
    initial_conditions:
      eUICC:
        - "Condition"
    steps:
      - step: 1
        description: "Step without command"
        expected:
          success: true
          result: "OK"
          output: "Success"
      - step: 2
        description: "Step 2"
        command: "cmd2"
        expected:
          result: "OK"
          output: "Success"
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(
            result.is_err(),
            "Should reject step missing required 'command'"
        );
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("command") || error_msg.contains("required"),
            "Error should mention missing command: {}",
            error_msg
        );
    }

    #[test]
    fn test_validate_chunk_test_sequences_missing_expected_fields() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
test_sequences:
  - id: 1
    name: "Test"
    description: "Test"
    initial_conditions:
      eUICC:
        - "Condition"
    steps:
      - step: 1
        manual: true
        description: "Step 1"
        command: "cmd1"
        expected:
          success: true
          result: "OK"
      - step: 2
        description: "Step 2"
        command: "cmd2"
        expected:
          result: "OK"
          output: "Success"
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(
            result.is_err(),
            "Should reject step with missing 'output' in first expected"
        );
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("output") || error_msg.contains("required"),
            "Error should mention missing output: {}",
            error_msg
        );
    }

    #[test]
    fn test_validate_chunk_general_initial_conditions_wrong_structure() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
general_initial_conditions:
  eUICC: "Should be array not string"
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(result.is_err(), "Should reject non-array eUICC value");
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("type") || error_msg.contains("Invalid"),
            "Error should mention type mismatch: {}",
            error_msg
        );
    }

    #[test]
    fn test_validate_chunk_initial_conditions_euicc_not_array() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
initial_conditions:
  eUICC: "Should be array"
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(result.is_err(), "Should reject non-array eUICC");
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("type") || error_msg.contains("Invalid"),
            "Error should mention type mismatch: {}",
            error_msg
        );
    }

    #[test]
    fn test_validate_chunk_initial_conditions_euicc_items_not_strings() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
initial_conditions:
  eUICC:
    - "Valid string"
    - 123
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(result.is_err(), "Should reject non-string array items");
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("type")
                || error_msg.contains("Invalid")
                || error_msg.contains("string"),
            "Error should mention type mismatch: {}",
            error_msg
        );
    }

    #[test]
    fn test_validate_chunk_step_wrong_type_for_step_number() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
test_sequences:
  - id: 1
    name: "Test"
    description: "Test"
    initial_conditions:
      eUICC:
        - "Condition"
    steps:
      - step: "should be integer"
        description: "Step"
        command: "cmd"
        expected:
          success: true
          result: "OK"
          output: "Success"
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(result.is_err(), "Should reject string step number");
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("type")
                || error_msg.contains("Invalid")
                || error_msg.contains("integer"),
            "Error should mention type mismatch: {}",
            error_msg
        );
    }

    #[test]
    fn test_validate_chunk_test_sequences_single_sequence_valid() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
test_sequences:
  - id: 1
    name: "Single Sequence"
    description: "Only one sequence"
    initial_conditions:
      eUICC:
        - "Condition"
    steps:
      - step: 1
        manual: true
        description: "Step 1"
        command: "cmd1"
        expected:
          success: true
          result: "OK"
          output: "Success"
      - step: 2
        description: "Step 2"
        command: "cmd2"
        expected:
          result: "OK"
          output: "Success"
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(
            result.is_ok(),
            "Should accept single test sequence: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_validate_chunk_manual_field_optional() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
test_sequences:
  - id: 1
    name: "Test"
    description: "Test"
    initial_conditions:
      eUICC:
        - "Condition"
    steps:
      - step: 1
        description: "Step without manual field"
        command: "cmd1"
        expected:
          success: true
          result: "OK"
          output: "Success"
      - step: 2
        description: "Step 2"
        command: "cmd2"
        expected:
          result: "OK"
          output: "Success"
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(
            result.is_ok(),
            "Should accept steps without manual field: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_validate_complete_empty_test_sequences() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
requirement: XXX100
item: 1
tc: 4
id: 'TC001'
description: 'Test'
general_initial_conditions:
  eUICC:
    - "Condition"
initial_conditions:
  eUICC:
    - "Condition 1"
    - "Condition 2"
test_sequences: []
"#;

        let result = validator.validate_complete(yaml_content);
        assert!(
            result.is_ok(),
            "Should accept empty test_sequences array: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_validate_chunk_success_field_optional_in_second_step() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
test_sequences:
  - id: 1
    name: "Test"
    description: "Test"
    initial_conditions:
      eUICC:
        - "Condition"
    steps:
      - step: 1
        manual: true
        description: "Step with success"
        command: "cmd1"
        expected:
          success: true
          result: "OK"
          output: "Success"
      - step: 2
        description: "Step without success"
        command: "cmd2"
        expected:
          result: "OK"
          output: "Success"
  - id: 2
    name: "Test 2"
    description: "Test 2"
    initial_conditions:
      eUICC:
        - "Condition"
    steps:
      - step: 1
        description: "Step with success"
        command: "cmd1"
        expected:
          success: false
          result: "OK"
          output: "Success"
      - step: 2
        description: "Step without success"
        command: "cmd2"
        expected:
          result: "OK"
          output: "Success"
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(
            result.is_ok(),
            "Steps should follow tuple validation pattern: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_validate_chunk_accepts_valid_complete_payload() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
requirement: XXX100
item: 1
tc: 4
id: '4.2.2.2.1 TC_eUICC_ES6.UpdateMetadata'
description: 'Test description'
general_initial_conditions:
  eUICC:
    - "Some condition"
initial_conditions:
  eUICC:
    - "Condition 1"
    - "Condition 2"
test_sequences:
  - id: 1
    name: "Test Sequence"
    description: "Test description"
    initial_conditions:
      eUICC:
        - "Condition"
    steps:
      - step: 1
        manual: true
        description: "Step description"
        command: "ssh"
        expected:
          success: true
          result: "SW=0x9000"
          output: "Success"
      - step: 2
        description: "Step 2 description"
        command: "ssh"
        expected:
          result: "SW=0x9000"
          output: "Success"
  - id: 2
    name: "Test Sequence 2"
    description: "Test description 2"
    initial_conditions:
      eUICC:
        - "Condition"
    steps:
      - step: 1
        manual: false
        description: "Step description"
        command: "ssh"
        expected:
          success: false
          result: "SW=0x9000"
          output: "Success"
      - step: 2
        description: "Step 2 description"
        command: "ssh"
        expected:
          result: "SW=0x9000"
          output: "Success"
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(
            result.is_ok(),
            "Should accept valid complete payload: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_validate_chunk_rejects_invalid_integer_type() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
item: "not_an_integer"
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(
            result.is_err(),
            "Should reject string where integer expected"
        );
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("Invalid type") || error_msg.contains("integer"),
            "Error should mention type mismatch: {}",
            error_msg
        );
    }

    #[test]
    fn test_validate_chunk_accepts_additional_array_items() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
general_initial_conditions:
  eUICC:
     - "Condition 1"
     - "Condition 2"
     - "Condition 3"
     - "Condition 4"
     - "Condition 5"
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(
            result.is_ok(),
            "Should accept additional items in arrays (tuple validation allows extras): {:?}",
            result.err()
        );
    }

    #[test]
    fn test_validate_chunk_accepts_additional_steps() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
test_sequences:
  - id: 1
    name: "Test"
    description: "Test"
    initial_conditions:
      eUICC:
        - "Condition"
    steps:
      - step: 1
        description: "Step 1"
        command: "cmd1"
        expected:
          success: true
          result: "OK"
          output: "Success"
      - step: 2
        description: "Step 2"
        command: "cmd2"
        expected:
          result: "OK"
          output: "Success"
      - step: 3
        description: "Step 3"
        command: "cmd3"
        expected:
          result: "OK"
          output: "Success"
      - step: 4
        description: "Step 4"
        command: "cmd4"
        expected:
          result: "OK"
          output: "Success"
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(
            result.is_ok(),
            "Should accept more than 2 steps (tuple validation allows additional items): {:?}",
            result.err()
        );
    }

    #[test]
    fn test_validate_chunk_rejects_missing_required_sequence_fields() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
test_sequences:
  - id: 1
    description: "Missing name and other required fields"
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(
            result.is_err(),
            "Should reject sequence missing required fields"
        );
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("name") || error_msg.contains("required"),
            "Error should mention missing required field: {}",
            error_msg
        );
    }

    #[test]
    fn test_validate_chunk_initial_conditions_accepts_more_than_two_items() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
initial_conditions:
  eUICC:
    - "Condition 1"
    - "Condition 2"
    - "Condition 3"
    - "Condition 4"
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(
            result.is_ok(),
            "Should accept more than 2 items in eUICC array (tuple validation allows extras): {:?}",
            result.err()
        );
    }

    #[test]
    fn test_validate_chunk_accepts_single_euicc_item() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
initial_conditions:
  eUICC:
    - "Single condition"
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(
            result.is_ok(),
            "Should accept single item in eUICC array: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_validate_chunk_empty_euicc_array() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
initial_conditions:
  eUICC: []
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(
            result.is_ok(),
            "Should accept empty eUICC array: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_validate_chunk_general_initial_conditions_accepts_multiple_devices() {
        let validator = SchemaValidator::new().unwrap();

        let yaml_content = r#"
general_initial_conditions:
  eUICC:
    - "eUICC condition 1"
    - "eUICC condition 2"
  LPA:
    - "LPA condition 1"
  SM_DP_PLUS:
    - "SM-DP+ condition 1"
    - "SM-DP+ condition 2"
    - "SM-DP+ condition 3"
"#;

        let result = validator.validate_chunk(yaml_content);
        assert!(
            result.is_ok(),
            "Should accept multiple device types in general_initial_conditions: {:?}",
            result.err()
        );
    }
}
