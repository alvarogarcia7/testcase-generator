use crate::models::{Priority, Status, TestType};
use crate::editor::TestCaseEditor;
use crate::validation::SchemaValidator;
use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use indexmap::IndexMap;
use serde_yaml::Value;

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

    /// Select a priority level
    pub fn select_priority() -> Result<Priority> {
        let priorities = vec!["Low", "Medium", "High", "Critical"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select priority")
            .items(&priorities)
            .default(1)
            .interact()
            .context("Failed to read selection")?;

        match selection {
            0 => Ok(Priority::Low),
            1 => Ok(Priority::Medium),
            2 => Ok(Priority::High),
            3 => Ok(Priority::Critical),
            _ => unreachable!(),
        }
    }

    /// Select a status
    pub fn select_status() -> Result<Status> {
        let statuses = vec!["Draft", "Active", "Deprecated", "Archived"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select status")
            .items(&statuses)
            .default(0)
            .interact()
            .context("Failed to read selection")?;

        match selection {
            0 => Ok(Status::Draft),
            1 => Ok(Status::Active),
            2 => Ok(Status::Deprecated),
            3 => Ok(Status::Archived),
            _ => unreachable!(),
        }
    }

    /// Select a test type
    pub fn select_test_type() -> Result<TestType> {
        let types = vec![
            "Functional",
            "Integration",
            "Regression",
            "Smoke",
            "Performance",
            "Security",
            "User Acceptance",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select test type")
            .items(&types)
            .default(0)
            .interact()
            .context("Failed to read selection")?;

        match selection {
            0 => Ok(TestType::Functional),
            1 => Ok(TestType::Integration),
            2 => Ok(TestType::Regression),
            3 => Ok(TestType::Smoke),
            4 => Ok(TestType::Performance),
            5 => Ok(TestType::Security),
            6 => Ok(TestType::UserAcceptance),
            _ => unreachable!(),
        }
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
            let yaml_str = serde_yaml::to_string(default_value)
                .context("Failed to serialize defaults")?;
            
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

            let edited_content = TestCaseEditor::edit_text(template)
                .context("Failed to open editor")?;

            let parsed: Value = serde_yaml::from_str(&edited_content)
                .context("Failed to parse YAML")?;

            let yaml_for_validation = serde_yaml::to_string(&parsed)
                .context("Failed to serialize for validation")?;

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
            let yaml_str = serde_yaml::to_string(default_value)
                .context("Failed to serialize defaults")?;
            
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
        
        println!("\nEnter conditions for '{}' (enter empty string to finish):", device_name);
        
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
        let conditions_array: Vec<Value> = conditions
            .into_iter()
            .map(Value::String)
            .collect();
        
        map.insert(
            Value::String(device_name.clone()),
            Value::Sequence(conditions_array)
        );

        let initial_conditions_value = Value::Mapping(map);

        validator.validate_initial_conditions(&initial_conditions_value)
            .context("Initial conditions validation failed")?;

        println!("✓ Valid structure");
        
        Ok(initial_conditions_value)
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
        map.insert("requirement".to_string(), Value::String(self.requirement.clone()));
        map.insert("item".to_string(), Value::Number(self.item.into()));
        map.insert("tc".to_string(), Value::Number(self.tc.into()));
        map.insert("id".to_string(), Value::String(self.id.clone()));
        map.insert("description".to_string(), Value::String(self.description.clone()));
        map
    }

    /// Validate metadata chunk
    pub fn validate(&self, validator: &SchemaValidator) -> Result<()> {
        let yaml_map = self.to_yaml();
        let yaml_value = Value::Mapping(serde_yaml::Mapping::from_iter(
            yaml_map.into_iter().map(|(k, v)| (Value::String(k), v))
        ));
        
        let yaml_str = serde_yaml::to_string(&yaml_value)
            .context("Failed to serialize metadata")?;
        
        validator.validate_partial_chunk(&yaml_str)
    }
}
