use anyhow::{Context, Result};
use jsonschema::JSONSchema;
use serde_json::Value as JsonValue;
use std::fs;
use std::path::Path;

pub struct SchemaValidator {
    schema: JSONSchema,
}

impl SchemaValidator {
    pub fn new() -> Result<Self> {
        let schema_path = Path::new("data/schema.json");
        let schema_content =
            fs::read_to_string(schema_path).context("Failed to read data/schema.json")?;

        let schema_value: JsonValue =
            serde_json::from_str(&schema_content).context("Failed to parse schema.json")?;

        let schema = JSONSchema::compile(&schema_value)
            .map_err(|e| anyhow::anyhow!("Failed to compile JSON schema: {}", e))?;

        Ok(Self { schema })
    }

    pub fn validate_chunk(&self, yaml_content: &str) -> Result<()> {
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
  - eUICC:
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
      - eUICC:
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
      - eUICC:
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

        let result = validator.validate_chunk(yaml_content);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Missing required property") || error_msg.contains("required"));
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
  - eUICC:
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
      - eUICC:
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
      - eUICC:
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

        let result = validator.validate_chunk(yaml_content);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Invalid type") || error_msg.contains("type"));
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
}
