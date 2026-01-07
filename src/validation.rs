use anyhow::{Context, Result};
use jsonschema::JSONSchema;
use serde_json::Value as JsonValue;

/// JSON Schema for validating test case YAML files
pub const TEST_CASE_SCHEMA: &str = r##"{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "TestCase",
  "type": "object",
  "required": ["id", "title", "priority", "status", "type", "sequences"],
  "properties": {
    "id": {
      "type": "string",
      "pattern": "^[a-zA-Z0-9_-]+$"
    },
    "title": {
      "type": "string",
      "minLength": 1
    },
    "description": {
      "type": "string"
    },
    "priority": {
      "type": "string",
      "enum": ["low", "medium", "high", "critical"]
    },
    "status": {
      "type": "string",
      "enum": ["draft", "active", "deprecated", "archived"]
    },
    "type": {
      "type": "string",
      "enum": ["functional", "integration", "regression", "smoke", "performance", "security", "user-acceptance"]
    },
    "tags": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "author": {
      "type": "string"
    },
    "created_at": {
      "type": "string",
      "format": "date-time"
    },
    "updated_at": {
      "type": "string",
      "format": "date-time"
    },
    "sequences": {
      "type": "array",
      "minItems": 1,
      "items": {
        "$ref": "#/definitions/TestSequence"
      }
    },
    "preconditions": {
      "type": "array",
      "items": {
        "$ref": "#/definitions/Precondition"
      }
    },
    "cleanup": {
      "type": "array",
      "items": {
        "$ref": "#/definitions/Cleanup"
      }
    },
    "environments": {
      "type": "array",
      "items": {
        "$ref": "#/definitions/Environment"
      }
    },
    "related_tests": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "metadata": {
      "type": "object"
    }
  },
  "definitions": {
    "TestSequence": {
      "type": "object",
      "required": ["id", "name", "steps"],
      "properties": {
        "id": {
          "type": "string",
          "pattern": "^[a-zA-Z0-9_-]+$"
        },
        "name": {
          "type": "string",
          "minLength": 1
        },
        "description": {
          "type": "string"
        },
        "steps": {
          "type": "array",
          "minItems": 1,
          "items": {
            "$ref": "#/definitions/Step"
          }
        },
        "tags": {
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "metadata": {
          "type": "object"
        }
      }
    },
    "Step": {
      "type": "object",
      "required": ["id", "description", "action"],
      "properties": {
        "id": {
          "type": "string",
          "pattern": "^[a-zA-Z0-9_-]+$"
        },
        "description": {
          "type": "string",
          "minLength": 1
        },
        "action": {
          "type": "string",
          "minLength": 1
        },
        "target": {
          "type": "string"
        },
        "value": {
          "type": "string"
        },
        "expected": {
          "type": "string"
        },
        "metadata": {
          "type": "object"
        }
      }
    },
    "Precondition": {
      "type": "object",
      "required": ["description"],
      "properties": {
        "description": {
          "type": "string",
          "minLength": 1
        },
        "setup_steps": {
          "type": "array",
          "items": {
            "type": "string"
          }
        }
      }
    },
    "Cleanup": {
      "type": "object",
      "required": ["description"],
      "properties": {
        "description": {
          "type": "string",
          "minLength": 1
        },
        "cleanup_steps": {
          "type": "array",
          "items": {
            "type": "string"
          }
        }
      }
    },
    "Environment": {
      "type": "object",
      "required": ["name"],
      "properties": {
        "name": {
          "type": "string",
          "minLength": 1
        },
        "url": {
          "type": "string",
          "format": "uri"
        },
        "variables": {
          "type": "object",
          "additionalProperties": {
            "type": "string"
          }
        }
      }
    }
  }
}"##;

/// Validator for test case data
pub struct TestCaseValidator {
    schema: JSONSchema,
}

impl TestCaseValidator {
    /// Create a new validator
    pub fn new() -> Result<Self> {
        let schema_value: JsonValue =
            serde_json::from_str(TEST_CASE_SCHEMA).context("Failed to parse test case schema")?;

        let schema = JSONSchema::compile(&schema_value)
            .map_err(|e| anyhow::anyhow!("Failed to compile schema: {}", e))?;

        Ok(Self { schema })
    }

    /// Validate a test case YAML string
    pub fn validate_yaml(&self, yaml_content: &str) -> Result<()> {
        let yaml_value: serde_yaml::Value =
            serde_yaml::from_str(yaml_content).context("Failed to parse YAML")?;

        let json_value: JsonValue =
            serde_json::to_value(&yaml_value).context("Failed to convert YAML to JSON")?;

        let validation_result = self.schema.validate(&json_value);

        if let Err(errors) = validation_result {
            let error_messages: Vec<String> = errors
                .map(|e| format!("  - {}: {}", e.instance_path, e))
                .collect();

            anyhow::bail!("Validation failed:\n{}", error_messages.join("\n"));
        }

        Ok(())
    }

    /// Validate a test case object
    pub fn validate_test_case(&self, test_case: &crate::models::TestCase) -> Result<()> {
        let yaml_content =
            serde_yaml::to_string(test_case).context("Failed to serialize test case to YAML")?;

        self.validate_yaml(&yaml_content)
    }
}

impl Default for TestCaseValidator {
    fn default() -> Self {
        Self::new().expect("Failed to create default validator")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;

    #[test]
    fn test_valid_test_case() {
        let validator = TestCaseValidator::new().unwrap();

        let mut test_case = TestCase::new("TC001".to_string(), "Test Case 1".to_string());
        test_case.sequences.push(TestSequence {
            id: "SEQ001".to_string(),
            name: "Main Flow".to_string(),
            description: None,
            steps: vec![Step::new(
                "STEP001".to_string(),
                "Open app".to_string(),
                "click".to_string(),
            )],
            tags: Vec::new(),
            metadata: std::collections::HashMap::new(),
        });

        assert!(validator.validate_test_case(&test_case).is_ok());
    }

    #[test]
    fn test_invalid_test_case_no_sequences() {
        let validator = TestCaseValidator::new().unwrap();

        let test_case = TestCase::new("TC001".to_string(), "Test Case 1".to_string());

        assert!(validator.validate_test_case(&test_case).is_err());
    }
}
