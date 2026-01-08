use crate::database::ConditionDatabase;
use crate::editor::TestCaseEditor;
use crate::fuzzy::TestCaseFuzzyFinder;
use crate::validation::SchemaValidator;
use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use indexmap::IndexMap;
use serde_yaml::Value;
use std::path::Path;

/// Interactive prompt utilities
pub struct Prompts;

impl Prompts {
    /// Prompt for a string input
    pub fn input(prompt: &str) -> Result<String> {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .interact_text()
            .context("Failed to read input")
    }

    /// Prompt for a string input with a recovered value as initial text (for recovery)
    pub fn input_with_recovered_default(prompt: &str, recovered: Option<&str>) -> Result<String> {
        if let Some(recovered_value) = recovered {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .with_initial_text(recovered_value)
                .interact_text()
                .context("Failed to read input")
        } else {
            Self::input(prompt)
        }
    }

    /// Prompt for an optional string input
    pub fn input_optional(prompt: &str) -> Result<Option<String>> {
        let input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .allow_empty(true)
            .interact_text()
            .context("Failed to read input")?;

        if input.trim().is_empty() {
            Ok(None)
        } else {
            Ok(Some(input))
        }
    }

    /// Prompt for a string with a default value
    pub fn input_with_default(prompt: &str, default: &str) -> Result<String> {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(default.to_string())
            .interact_text()
            .context("Failed to read input")
    }

    /// Prompt for an integer input
    pub fn input_integer(prompt: &str) -> Result<i64> {
        loop {
            let input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .interact_text()
                .context("Failed to read input")?;

            match input.trim().parse::<i64>() {
                Ok(value) => return Ok(value),
                Err(_) => println!("Please enter a valid integer"),
            }
        }
    }

    /// Prompt for an integer input with a recovered value as initial text (for recovery)
    pub fn input_integer_with_default(prompt: &str, recovered: Option<i64>) -> Result<i64> {
        if let Some(recovered_value) = recovered {
            loop {
                let input: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt(prompt)
                    .with_initial_text(recovered_value.to_string())
                    .interact_text()
                    .context("Failed to read input")?;

                match input.trim().parse::<i64>() {
                    Ok(value) => return Ok(value),
                    Err(_) => println!("Please enter a valid integer"),
                }
            }
        } else {
            Self::input_integer(prompt)
        }
    }

    /// Prompt for a confirmation (yes/no)
    pub fn confirm(prompt: &str) -> Result<bool> {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .interact()
            .context("Failed to read confirmation")
    }

    /// Prompt for a confirmation with a default value
    pub fn confirm_with_default(prompt: &str, default: bool) -> Result<bool> {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(default)
            .interact()
            .context("Failed to read confirmation")
    }

    /// Select from a list of items
    pub fn select<T: ToString>(prompt: &str, items: &[T]) -> Result<usize> {
        let item_strings: Vec<String> = items.iter().map(|i| i.to_string()).collect();

        Select::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .items(&item_strings)
            .interact()
            .context("Failed to read selection")
    }

    /// Multi-select from a list of items
    pub fn multi_select<T: ToString>(prompt: &str, items: &[T]) -> Result<Vec<usize>> {
        let item_strings: Vec<String> = items.iter().map(|i| i.to_string()).collect();

        MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .items(&item_strings)
            .interact()
            .context("Failed to read selection")
    }

    /// Prompt for tags (comma-separated)
    pub fn input_tags(prompt: &str) -> Result<Vec<String>> {
        let input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .allow_empty(true)
            .interact_text()
            .context("Failed to read input")?;

        if input.trim().is_empty() {
            Ok(Vec::new())
        } else {
            Ok(input
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect())
        }
    }

    /// Prompt for test case metadata fields
    pub fn prompt_metadata() -> Result<TestCaseMetadata> {
        println!("\n=== Test Case Metadata ===\n");

        let requirement = Self::input("Requirement")?;
        let item = Self::input_integer("Item")?;
        let tc = Self::input_integer("TC")?;
        let id = Self::input("ID")?;
        let description = Self::input("Description")?;

        let metadata = TestCaseMetadata {
            requirement,
            item,
            tc,
            id,
            description,
        };

        // metadata.validate_recursive(&validator, None).unwrap();

        Ok(metadata)
    }

    /// Prompt for test case metadata fields with recovery support
    pub fn prompt_metadata_with_recovery(
        recovered: Option<&TestCaseMetadata>,
    ) -> Result<TestCaseMetadata> {
        println!("\n=== Test Case Metadata ===\n");

        if recovered.is_some() {
            println!(
                "⚠ Recovered values shown as editable text (Enter confirms, you can edit/delete)\n"
            );
        }

        let requirement = Self::input_with_recovered_default(
            "Requirement",
            recovered.map(|m| m.requirement.as_str()),
        )?;
        let item = Self::input_integer_with_default("Item", recovered.map(|m| m.item))?;
        let tc = Self::input_integer_with_default("TC", recovered.map(|m| m.tc))?;
        let id = Self::input_with_recovered_default("ID", recovered.map(|m| m.id.as_str()))?;
        let description = Self::input_with_recovered_default(
            "Description",
            recovered.map(|m| m.description.as_str()),
        )?;

        Ok(TestCaseMetadata {
            requirement,
            item,
            tc,
            id,
            description,
        })
    }

    /// Prompt for general initial conditions with editor support
    pub fn prompt_general_initial_conditions(
        defaults: Option<&Value>,
        validator: &SchemaValidator,
    ) -> Result<Value> {
        println!("\n=== General Initial Conditions ===\n");

        if let Some(default_value) = defaults {
            let yaml_str =
                serde_yaml::to_string(default_value).context("Failed to serialize defaults")?;

            println!("Current defaults:");
            println!("{}", yaml_str);
            println!();

            let keep_defaults = Self::confirm_with_default("Keep these defaults?", true)?;

            if keep_defaults {
                return Ok(default_value.clone());
            }
        }

        loop {
            let template = r#"# General Initial Conditions
# Example:
# - eUICC:
#     - "Condition 1"
#     - "Condition 2"

- eUICC:
    - ""
"#;

            let edited_content =
                TestCaseEditor::edit_text(template).context("Failed to open editor")?;

            let parsed: Value =
                serde_yaml::from_str(&edited_content).context("Failed to parse YAML")?;

            let yaml_for_validation =
                serde_yaml::to_string(&parsed).context("Failed to serialize for validation")?;

            match validator.validate_partial_chunk(&yaml_for_validation) {
                Ok(_) => {
                    println!("✓ Valid structure");
                    return Ok(parsed);
                }
                Err(e) => {
                    println!("✗ Validation failed: {}", e);
                    let retry = Self::confirm("Try again?")?;
                    if !retry {
                        anyhow::bail!("Validation failed, user cancelled");
                    }
                }
            }
        }
    }

    /// Prompt for initial conditions with device selection and iterative condition collection
    pub fn prompt_initial_conditions(
        defaults: Option<&Value>,
        validator: &SchemaValidator,
    ) -> Result<Value> {
        println!("\n=== Initial Conditions ===\n");

        if let Some(default_value) = defaults {
            let yaml_str =
                serde_yaml::to_string(default_value).context("Failed to serialize defaults")?;

            println!("Current defaults:");
            println!("{}", yaml_str);
            println!();

            let keep_defaults = Self::confirm_with_default("Keep these defaults?", true)?;

            if keep_defaults {
                return Ok(default_value.clone());
            }
        }

        let device_name = Self::input("Device name (e.g., eUICC)")?;

        let mut conditions: Vec<String> = Vec::new();

        println!(
            "\nEnter conditions for '{}' (enter empty string to finish):",
            device_name
        );

        loop {
            let condition = Self::input_optional(&format!("Condition #{}", conditions.len() + 1))?;

            match condition {
                Some(cond) if !cond.trim().is_empty() => {
                    conditions.push(cond);
                }
                _ => {
                    if conditions.is_empty() {
                        println!("At least one condition is required.");
                        continue;
                    }
                    break;
                }
            }
        }

        let mut map = serde_yaml::Mapping::new();
        let conditions_array: Vec<Value> = conditions.into_iter().map(Value::String).collect();

        map.insert(
            Value::String(device_name.clone()),
            Value::Sequence(conditions_array),
        );

        let initial_conditions_value = Value::Mapping(map);

        validator
            .validate_initial_conditions(&initial_conditions_value)
            .context("Initial conditions validation failed")?;

        println!("✓ Valid structure");

        Ok(initial_conditions_value)
    }

    /// Prompt for general initial conditions from database with fuzzy search
    pub fn prompt_general_initial_conditions_from_database<P: AsRef<Path>>(
        database_path: P,
        validator: &SchemaValidator,
    ) -> Result<Value> {
        println!("\n=== General Initial Conditions (from database) ===\n");

        let db = ConditionDatabase::load_from_directory(database_path)
            .context("Failed to load condition database")?;

        let conditions = db.get_general_conditions();

        if conditions.is_empty() {
            println!("No general initial conditions found in database.");
            println!("Falling back to manual entry.\n");
            return Self::prompt_general_initial_conditions(None, validator);
        }

        println!(
            "Loaded {} unique general initial conditions from database\n",
            conditions.len()
        );

        let mut selected_conditions = Vec::new();

        loop {
            let selected = TestCaseFuzzyFinder::search_strings(
                conditions,
                "Select condition (ESC to finish): ",
            )?;

            match selected {
                Some(condition) => {
                    selected_conditions.push(condition.clone());
                    println!("✓ Added: {}\n", condition);

                    if !Self::confirm("Add another general initial condition?")? {
                        break;
                    }
                }
                None => {
                    if selected_conditions.is_empty() {
                        println!("No conditions selected.");
                        if Self::confirm("Use manual entry instead?")? {
                            return Self::prompt_general_initial_conditions(None, validator);
                        }
                    }
                    break;
                }
            }
        }

        if selected_conditions.is_empty() {
            anyhow::bail!("No general initial conditions selected");
        }

        let euicc_conditions: Vec<Value> =
            selected_conditions.into_iter().map(Value::String).collect();

        let mut general_cond_map = serde_yaml::Mapping::new();
        general_cond_map.insert(
            Value::String("eUICC".to_string()),
            Value::Sequence(euicc_conditions),
        );

        let general_conditions_array = vec![Value::Mapping(general_cond_map)];

        Ok(Value::Sequence(general_conditions_array))
    }

    /// Prompt for initial conditions from database with fuzzy search
    pub fn prompt_initial_conditions_from_database<P: AsRef<Path>>(
        database_path: P,
        validator: &SchemaValidator,
    ) -> Result<Value> {
        println!("\n=== Initial Conditions (from database) ===\n");

        let db = ConditionDatabase::load_from_directory(database_path)
            .context("Failed to load condition database")?;

        let conditions = db.get_initial_conditions();

        if conditions.is_empty() {
            println!("No initial conditions found in database.");
            println!("Falling back to manual entry.\n");
            return Self::prompt_initial_conditions(None, validator);
        }

        println!(
            "Loaded {} unique initial conditions from database\n",
            conditions.len()
        );

        let mut selected_conditions = Vec::new();

        loop {
            let selected = TestCaseFuzzyFinder::search_strings(
                conditions,
                "Select condition (ESC to finish): ",
            )?;

            match selected {
                Some(condition) => {
                    selected_conditions.push(condition.clone());
                    println!("✓ Added: {}\n", condition);

                    if !Self::confirm("Add another initial condition?")? {
                        break;
                    }
                }
                None => {
                    if selected_conditions.is_empty() {
                        println!("No conditions selected.");
                        if Self::confirm("Use manual entry instead?")? {
                            return Self::prompt_initial_conditions(None, validator);
                        }
                    }
                    break;
                }
            }
        }

        if selected_conditions.is_empty() {
            anyhow::bail!("No initial conditions selected");
        }

        let euicc_conditions: Vec<Value> =
            selected_conditions.into_iter().map(Value::String).collect();

        let mut initial_cond_map = serde_yaml::Mapping::new();
        initial_cond_map.insert(
            Value::String("eUICC".to_string()),
            Value::Sequence(euicc_conditions),
        );

        Ok(Value::Mapping(initial_cond_map))
    }
}

/// Metadata fields for a test case
#[derive(Debug, Clone)]
pub struct TestCaseMetadata {
    pub requirement: String,
    pub item: i64,
    pub tc: i64,
    pub id: String,
    pub description: String,
}

impl TestCaseMetadata {
    /// Convert to YAML structure
    pub fn to_yaml(&self) -> IndexMap<String, Value> {
        let mut map = IndexMap::new();
        map.insert(
            "requirement".to_string(),
            Value::String(self.requirement.clone()),
        );
        map.insert("item".to_string(), Value::Number(self.item.into()));
        map.insert("tc".to_string(), Value::Number(self.tc.into()));
        map.insert("id".to_string(), Value::String(self.id.clone()));
        map.insert(
            "description".to_string(),
            Value::String(self.description.clone()),
        );
        map
    }

    /// Extract from YAML structure
    pub fn from_structure(structure: &IndexMap<String, Value>) -> Option<Self> {
        let requirement = structure
            .get("requirement")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())?;

        let item = structure.get("item").and_then(|v| v.as_i64())?;

        let tc = structure.get("tc").and_then(|v| v.as_i64())?;

        let id = structure
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())?;

        let description = structure
            .get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())?;

        Some(TestCaseMetadata {
            requirement,
            item,
            tc,
            id,
            description,
        })
    }

    /// Validate metadata chunk
    pub fn validate(&self, validator: &SchemaValidator) -> Result<()> {
        self.validate_recursive(validator, None)
    }

    /// Validate metadata chunk with optional attribute specification for recursive validation
    ///
    /// # Arguments
    /// * `validator` - The schema validator to use
    /// * `attribute` - Optional attribute name to validate. If None, validates all attributes.
    ///   Supports nested validation by validating one attribute at a time.
    ///
    /// # Example
    /// ```ignore
    /// // Validate all attributes
    /// metadata.validate_recursive(validator, None)?;
    ///
    /// // Validate only the 'requirement' attribute
    /// metadata.validate_recursive(validator, Some("requirement"))?;
    ///
    /// // Validate attributes recursively one by one
    /// for attr in &["requirement", "item", "tc", "id", "description"] {
    ///     metadata.validate_recursive(validator, Some(attr))?;
    /// }
    /// ```
    pub fn validate_recursive(
        &self,
        validator: &SchemaValidator,
        attribute: Option<&str>,
    ) -> Result<()> {
        let yaml_map = match attribute {
            Some(attr_name) => {
                // Validate only the specified attribute
                let mut single_attr_map = IndexMap::new();

                match attr_name {
                    "requirement" => {
                        single_attr_map.insert(
                            "requirement".to_string(),
                            Value::String(self.requirement.clone()),
                        );
                    }
                    "item" => {
                        single_attr_map.insert("item".to_string(), Value::Number(self.item.into()));
                    }
                    "tc" => {
                        single_attr_map.insert("tc".to_string(), Value::Number(self.tc.into()));
                    }
                    "id" => {
                        single_attr_map.insert("id".to_string(), Value::String(self.id.clone()));
                    }
                    "description" => {
                        single_attr_map.insert(
                            "description".to_string(),
                            Value::String(self.description.clone()),
                        );
                    }
                    _ => {
                        anyhow::bail!("Unknown attribute '{}' for validation", attr_name);
                    }
                }

                single_attr_map
            }
            None => {
                // Validate all attributes
                self.to_yaml()
            }
        };

        let yaml_value = Value::Mapping(serde_yaml::Mapping::from_iter(
            yaml_map.into_iter().map(|(k, v)| (Value::String(k), v)),
        ));

        let yaml_str =
            serde_yaml::to_string(&yaml_value).context("Failed to serialize metadata")?;

        validator
            .validate_partial_chunk(&yaml_str)
            .context(match attribute {
                Some(attr) => format!("Validation failed for attribute '{}'", attr),
                None => "OK".to_string(),
            })
    }

    /// Validate all metadata attributes recursively, one at a time
    ///
    /// This method validates each attribute individually in sequence, which is useful
    /// for identifying exactly which attribute is causing validation failures.
    ///
    /// # Arguments
    /// * `validator` - The schema validator to use
    ///
    /// # Returns
    /// * `Ok(())` if all attributes are valid
    /// * `Err` with context about which attribute failed
    pub fn validate_all_attributes_recursively(&self, validator: &SchemaValidator) -> Result<()> {
        let attributes = ["requirement", "item", "tc", "id", "description"];

        for attribute in &attributes {
            self.validate_recursive(validator, Some(attribute))
                .context(format!("Failed at attribute '{}'", attribute))?;
        }

        Ok(())
    }

    /// Get list of all attribute names that can be validated
    pub fn get_validatable_attributes() -> Vec<&'static str> {
        vec!["requirement", "item", "tc", "id", "description"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_to_yaml() {
        let metadata = TestCaseMetadata {
            requirement: "REQ001".to_string(),
            item: 1,
            tc: 2,
            id: "TC001".to_string(),
            description: "Test description".to_string(),
        };

        let yaml_map = metadata.to_yaml();
        assert_eq!(yaml_map.len(), 5);
        assert_eq!(
            yaml_map.get("requirement"),
            Some(&Value::String("REQ001".to_string()))
        );
        assert_eq!(yaml_map.get("item"), Some(&Value::Number(1.into())));
        assert_eq!(yaml_map.get("tc"), Some(&Value::Number(2.into())));
        assert_eq!(
            yaml_map.get("id"),
            Some(&Value::String("TC001".to_string()))
        );
        assert_eq!(
            yaml_map.get("description"),
            Some(&Value::String("Test description".to_string()))
        );
    }

    #[test]
    fn test_metadata_from_structure() {
        let mut structure = IndexMap::new();
        structure.insert(
            "requirement".to_string(),
            Value::String("REQ001".to_string()),
        );
        structure.insert("item".to_string(), Value::Number(1.into()));
        structure.insert("tc".to_string(), Value::Number(2.into()));
        structure.insert("id".to_string(), Value::String("TC001".to_string()));
        structure.insert(
            "description".to_string(),
            Value::String("Test description".to_string()),
        );

        let metadata = TestCaseMetadata::from_structure(&structure).unwrap();
        assert_eq!(metadata.requirement, "REQ001");
        assert_eq!(metadata.item, 1);
        assert_eq!(metadata.tc, 2);
        assert_eq!(metadata.id, "TC001");
        assert_eq!(metadata.description, "Test description");
    }

    #[test]
    fn test_metadata_from_structure_missing_field() {
        let mut structure = IndexMap::new();
        structure.insert(
            "requirement".to_string(),
            Value::String("REQ001".to_string()),
        );
        structure.insert("item".to_string(), Value::Number(1.into()));

        let metadata = TestCaseMetadata::from_structure(&structure);
        assert!(metadata.is_none());
    }

    #[test]
    fn test_metadata_from_structure_invalid_type() {
        let mut structure = IndexMap::new();
        structure.insert(
            "requirement".to_string(),
            Value::String("REQ001".to_string()),
        );
        structure.insert(
            "item".to_string(),
            Value::String("not_a_number".to_string()),
        );
        structure.insert("tc".to_string(), Value::Number(2.into()));
        structure.insert("id".to_string(), Value::String("TC001".to_string()));
        structure.insert(
            "description".to_string(),
            Value::String("Test description".to_string()),
        );

        let metadata = TestCaseMetadata::from_structure(&structure);
        assert!(metadata.is_none());
    }

    #[test]
    fn test_metadata_to_yaml_structure() {
        let metadata = TestCaseMetadata {
            requirement: "REQ001".to_string(),
            item: 1,
            tc: 2,
            id: "TC001".to_string(),
            description: "Test description".to_string(),
        };

        let yaml_map = metadata.to_yaml();

        // Verify the YAML structure is correct
        assert_eq!(yaml_map.len(), 5);
        assert!(yaml_map.contains_key("requirement"));
        assert!(yaml_map.contains_key("item"));
        assert!(yaml_map.contains_key("tc"));
        assert!(yaml_map.contains_key("id"));
        assert!(yaml_map.contains_key("description"));
    }

    #[test]
    fn test_metadata_roundtrip() {
        let original = TestCaseMetadata {
            requirement: "REQ001".to_string(),
            item: 1,
            tc: 2,
            id: "TC001".to_string(),
            description: "Test description".to_string(),
        };

        let yaml_map = original.to_yaml();
        let recovered = TestCaseMetadata::from_structure(&yaml_map).unwrap();

        assert_eq!(original.requirement, recovered.requirement);
        assert_eq!(original.item, recovered.item);
        assert_eq!(original.tc, recovered.tc);
        assert_eq!(original.id, recovered.id);
        assert_eq!(original.description, recovered.description);
    }

    #[test]
    fn test_validate_recursive_single_attribute() {
        let metadata = TestCaseMetadata {
            requirement: "XXX100".to_string(),
            item: 1,
            tc: 4,
            id: "TC001".to_string(),
            description: "Test description".to_string(),
        };

        let validator = crate::validation::SchemaValidator::new().unwrap();

        // Should be able to validate individual attributes
        assert!(metadata
            .validate_recursive(&validator, Some("requirement"))
            .is_ok());
        assert!(metadata
            .validate_recursive(&validator, Some("item"))
            .is_ok());
        assert!(metadata.validate_recursive(&validator, Some("tc")).is_ok());
        assert!(metadata.validate_recursive(&validator, Some("id")).is_ok());
        assert!(metadata
            .validate_recursive(&validator, Some("description"))
            .is_ok());
    }

    #[test]
    fn test_validate_recursive_all_attributes() {
        let metadata = TestCaseMetadata {
            requirement: "XXX100".to_string(),
            item: 1,
            tc: 4,
            id: "TC001".to_string(),
            description: "Test description".to_string(),
        };

        let validator = crate::validation::SchemaValidator::new().unwrap();

        // Should validate all attributes when None is passed
        assert!(metadata.validate_recursive(&validator, None).is_ok());
    }

    #[test]
    fn test_validate_recursive_unknown_attribute() {
        let metadata = TestCaseMetadata {
            requirement: "XXX100".to_string(),
            item: 1,
            tc: 4,
            id: "TC001".to_string(),
            description: "Test description".to_string(),
        };

        let validator = crate::validation::SchemaValidator::new().unwrap();

        // Should fail for unknown attribute
        let result = metadata.validate_recursive(&validator, Some("unknown_field"));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Unknown attribute"));
    }

    #[test]
    fn test_validate_all_attributes_recursively() {
        let metadata = TestCaseMetadata {
            requirement: "XXX100".to_string(),
            item: 1,
            tc: 4,
            id: "TC001".to_string(),
            description: "Test description".to_string(),
        };

        let validator = crate::validation::SchemaValidator::new().unwrap();

        // Should validate all attributes one by one
        assert!(metadata
            .validate_all_attributes_recursively(&validator)
            .is_ok());
    }

    #[test]
    fn test_get_validatable_attributes() {
        let attributes = TestCaseMetadata::get_validatable_attributes();

        assert_eq!(attributes.len(), 5);
        assert!(attributes.contains(&"requirement"));
        assert!(attributes.contains(&"item"));
        assert!(attributes.contains(&"tc"));
        assert!(attributes.contains(&"id"));
        assert!(attributes.contains(&"description"));
    }

    #[test]
    fn test_validate_recursive_iterative() {
        let metadata = TestCaseMetadata {
            requirement: "XXX100".to_string(),
            item: 1,
            tc: 4,
            id: "TC001".to_string(),
            description: "Test description".to_string(),
        };

        let validator = crate::validation::SchemaValidator::new().unwrap();

        // Validate each attribute recursively one at a time
        for attribute in &["requirement", "item", "tc", "id", "description"] {
            let result = metadata.validate_recursive(&validator, Some(attribute));
            assert!(
                result.is_ok(),
                "Validation failed for attribute '{}': {:?}",
                attribute,
                result.err()
            );
        }
    }
}
