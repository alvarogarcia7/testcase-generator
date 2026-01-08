use crate::database::ConditionDatabase;
use crate::editor::EditorFlow;
use crate::fuzzy::TestCaseFuzzyFinder;
use crate::oracle::{Oracle, TtyCliOracle};
use crate::sample::SampleData;
use crate::validation::SchemaValidator;
use crate::{config::EditorConfig, TestCaseEditor};
use anyhow::{Context, Result};
use indexmap::IndexMap;
use serde_yaml::Value;
use std::path::Path;

/// Interactive prompt utilities
pub struct Prompts<'a> {
    db: Option<&'a ConditionDatabase>,
    sample: Option<&'a SampleData>,
}

impl<'a> Prompts<'a> {
    /// Create a new Prompts instance without a database
    pub fn new() -> Prompts<'static> {
        Prompts {
            db: None,
            sample: None,
        }
    }

    /// Create a new Prompts instance with a database
    pub fn new_with_database(db: &'a ConditionDatabase) -> Self {
        Self {
            db: Some(db),
            sample: None,
        }
    }

    /// Create a new Prompts instance with sample data
    pub fn new_with_sample(sample: &'a SampleData) -> Self {
        Self {
            db: None,
            sample: Some(sample),
        }
    }

    /// Create a new Prompts instance with both database and sample data
    pub fn new_with_database_and_sample(db: &'a ConditionDatabase, sample: &'a SampleData) -> Self {
        Self {
            db: Some(db),
            sample: Some(sample),
        }
    }

    /// Prompt for a string input (backward compatibility - uses default TtyCliOracle)
    pub fn input(prompt: &str) -> Result<String> {
        Self::input_with_oracle(prompt, &TtyCliOracle::new())
    }

    /// Prompt for a string input with custom oracle
    pub fn input_with_oracle(prompt: &str, oracle: &impl Oracle) -> Result<String> {
        oracle.input(prompt)
    }

    /// Prompt for a string input with sample default
    pub fn input_with_sample(&self, prompt: &str, sample_value: &str) -> Result<String> {
        self.input_with_sample_oracle(prompt, sample_value, &TtyCliOracle::new())
    }

    /// Prompt for a string input with sample default and custom oracle
    pub fn input_with_sample_oracle(
        &self,
        prompt: &str,
        sample_value: &str,
        oracle: &impl Oracle,
    ) -> Result<String> {
        oracle.input_with_initial_text(prompt, sample_value)
    }

    /// Prompt for a string input with a recovered value as initial text (for recovery)
    pub fn input_with_recovered_default(prompt: &str, recovered: Option<&str>) -> Result<String> {
        Self::input_with_recovered_default_oracle(prompt, recovered, &TtyCliOracle::new())
    }

    /// Prompt for a string input with a recovered value as initial text and custom oracle
    pub fn input_with_recovered_default_oracle(
        prompt: &str,
        recovered: Option<&str>,
        oracle: &impl Oracle,
    ) -> Result<String> {
        if let Some(recovered_value) = recovered {
            oracle.input_with_initial_text(prompt, recovered_value)
        } else {
            Self::input_with_oracle(prompt, oracle)
        }
    }

    /// Prompt for an optional string input
    pub fn input_optional(prompt: &str) -> Result<Option<String>> {
        Self::input_optional_with_oracle(prompt, &TtyCliOracle::new())
    }

    /// Prompt for an optional string input with custom oracle
    pub fn input_optional_with_oracle(
        prompt: &str,
        oracle: &impl Oracle,
    ) -> Result<Option<String>> {
        oracle.input_optional(prompt)
    }

    /// Prompt for an optional string input with sample default
    pub fn input_optional_with_sample(
        &self,
        prompt: &str,
        sample_value: &str,
    ) -> Result<Option<String>> {
        self.input_optional_with_sample_oracle(prompt, sample_value, &TtyCliOracle::new())
    }

    /// Prompt for an optional string input with sample default and custom oracle
    pub fn input_optional_with_sample_oracle(
        &self,
        prompt: &str,
        sample_value: &str,
        oracle: &impl Oracle,
    ) -> Result<Option<String>> {
        oracle.input_optional_with_initial_text(prompt, sample_value)
    }

    /// Prompt for a string with a default value
    pub fn input_with_default(prompt: &str, default: &str) -> Result<String> {
        Self::input_with_default_oracle(prompt, default, &TtyCliOracle::new())
    }

    /// Prompt for a string with a default value and custom oracle
    pub fn input_with_default_oracle(
        prompt: &str,
        default: &str,
        oracle: &impl Oracle,
    ) -> Result<String> {
        oracle.input_with_default(prompt, default)
    }

    /// Prompt for an integer input
    pub fn input_integer(prompt: &str) -> Result<i64> {
        Self::input_integer_with_oracle(prompt, &TtyCliOracle::new())
    }

    /// Prompt for an integer input with custom oracle
    pub fn input_integer_with_oracle(prompt: &str, oracle: &impl Oracle) -> Result<i64> {
        oracle.input_integer(prompt)
    }

    /// Prompt for an integer input with sample default
    pub fn input_integer_with_sample(&self, prompt: &str, sample_value: i64) -> Result<i64> {
        self.input_integer_with_sample_oracle(prompt, sample_value, &TtyCliOracle::new())
    }

    /// Prompt for an integer input with sample default and custom oracle
    pub fn input_integer_with_sample_oracle(
        &self,
        prompt: &str,
        sample_value: i64,
        oracle: &impl Oracle,
    ) -> Result<i64> {
        oracle.input_integer_with_initial_text(prompt, &sample_value.to_string())
    }

    /// Prompt for an integer input with a recovered value as initial text (for recovery)
    pub fn input_integer_with_default(prompt: &str, recovered: Option<i64>) -> Result<i64> {
        Self::input_integer_with_default_oracle(prompt, recovered, &TtyCliOracle::new())
    }

    /// Prompt for an integer input with a recovered value as initial text and custom oracle
    pub fn input_integer_with_default_oracle(
        prompt: &str,
        recovered: Option<i64>,
        oracle: &impl Oracle,
    ) -> Result<i64> {
        if let Some(recovered_value) = recovered {
            oracle.input_integer_with_initial_text(prompt, &recovered_value.to_string())
        } else {
            Self::input_integer_with_oracle(prompt, oracle)
        }
    }

    /// Prompt for a confirmation (yes/no)
    pub fn confirm(prompt: &str) -> Result<bool> {
        Self::confirm_with_oracle(prompt, &TtyCliOracle::new())
    }

    /// Prompt for a confirmation with custom oracle
    pub fn confirm_with_oracle(prompt: &str, oracle: &impl Oracle) -> Result<bool> {
        oracle.confirm(prompt)
    }

    /// Prompt for a confirmation with a default value
    pub fn confirm_with_default(prompt: &str, default: bool) -> Result<bool> {
        Self::confirm_with_default_oracle(prompt, default, &TtyCliOracle::new())
    }

    /// Prompt for a confirmation with a default value and custom oracle
    pub fn confirm_with_default_oracle(
        prompt: &str,
        default: bool,
        oracle: &impl Oracle,
    ) -> Result<bool> {
        oracle.confirm_with_default(prompt, default)
    }

    /// Prompt for a confirmation with sample default
    pub fn confirm_with_sample(&self, prompt: &str, sample_value: bool) -> Result<bool> {
        self.confirm_with_sample_oracle(prompt, sample_value, &TtyCliOracle::new())
    }

    /// Prompt for a confirmation with sample default and custom oracle
    pub fn confirm_with_sample_oracle(
        &self,
        prompt: &str,
        sample_value: bool,
        oracle: &impl Oracle,
    ) -> Result<bool> {
        oracle.confirm_with_default(prompt, sample_value)
    }

    /// Select from a list of items
    pub fn select<T: ToString>(prompt: &str, items: &[T]) -> Result<usize> {
        Self::select_with_oracle(prompt, items, &TtyCliOracle::new())
    }

    /// Select from a list of items with custom oracle
    pub fn select_with_oracle<T: ToString>(
        prompt: &str,
        items: &[T],
        oracle: &impl Oracle,
    ) -> Result<usize> {
        oracle.select(prompt, items)
    }

    /// Multi-select from a list of items
    pub fn multi_select<T: ToString>(prompt: &str, items: &[T]) -> Result<Vec<usize>> {
        Self::multi_select_with_oracle(prompt, items, &TtyCliOracle::new())
    }

    /// Multi-select from a list of items with custom oracle
    pub fn multi_select_with_oracle<T: ToString>(
        prompt: &str,
        items: &[T],
        oracle: &impl Oracle,
    ) -> Result<Vec<usize>> {
        oracle.multi_select(prompt, items)
    }

    /// Prompt for tags (comma-separated)
    pub fn input_tags(prompt: &str) -> Result<Vec<String>> {
        Self::input_tags_with_oracle(prompt, &TtyCliOracle::new())
    }

    /// Prompt for tags with custom oracle
    pub fn input_tags_with_oracle(prompt: &str, oracle: &impl Oracle) -> Result<Vec<String>> {
        let input = oracle.input(prompt)?;

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
        Self::prompt_metadata_with_oracle(&TtyCliOracle::new())
    }

    /// Prompt for test case metadata fields with custom oracle
    pub fn prompt_metadata_with_oracle(oracle: &impl Oracle) -> Result<TestCaseMetadata> {
        println!("\n=== Test Case Metadata ===\n");

        let requirement = Self::input_with_oracle("Requirement", oracle)?;
        let item = Self::input_integer_with_oracle("Item", oracle)?;
        let tc = Self::input_integer_with_oracle("TC", oracle)?;
        let id = Self::input_with_oracle("ID", oracle)?;
        let description = Self::input_with_oracle("Description", oracle)?;

        let metadata = TestCaseMetadata {
            requirement,
            item,
            tc,
            id,
            description,
        };
        log::debug!(
            "Metadata collected: requirement={}, item={}, tc={}",
            metadata.requirement,
            metadata.item,
            metadata.tc
        );
        Ok(metadata)
    }

    /// Prompt for test case metadata fields with optional sample data
    pub fn prompt_metadata_with_sample(&self) -> Result<TestCaseMetadata> {
        self.prompt_metadata_with_sample_oracle(&TtyCliOracle::new())
    }

    /// Prompt for test case metadata fields with optional sample data and custom oracle
    pub fn prompt_metadata_with_sample_oracle(
        &self,
        oracle: &impl Oracle,
    ) -> Result<TestCaseMetadata> {
        println!("\n=== Test Case Metadata ===\n");

        if self.sample.is_some() {
            log::info!(
                "⚠ Sample values shown as editable text (Enter confirms, you can edit/delete)\n"
            );
        }

        let (requirement, item, tc, id, description) = if let Some(sample) = self.sample {
            (
                self.input_with_sample_oracle(
                    "Requirement",
                    &sample.metadata_requirement(),
                    oracle,
                )?,
                self.input_integer_with_sample_oracle("Item", sample.metadata_item(), oracle)?,
                self.input_integer_with_sample_oracle("TC", sample.metadata_tc(), oracle)?,
                self.input_with_sample_oracle("ID", &sample.metadata_id(), oracle)?,
                self.input_with_sample_oracle(
                    "Description",
                    &sample.metadata_description(),
                    oracle,
                )?,
            )
        } else {
            (
                Self::input_with_oracle("Requirement", oracle)?,
                Self::input_integer_with_oracle("Item", oracle)?,
                Self::input_integer_with_oracle("TC", oracle)?,
                Self::input_with_oracle("ID", oracle)?,
                Self::input_with_oracle("Description", oracle)?,
            )
        };

        Ok(TestCaseMetadata {
            requirement,
            item,
            tc,
            id,
            description,
        })
    }

    /// Prompt for test case metadata fields with recovery support
    pub fn prompt_metadata_with_recovery(
        recovered: Option<&TestCaseMetadata>,
    ) -> Result<TestCaseMetadata> {
        Self::prompt_metadata_with_recovery_oracle(recovered, &TtyCliOracle::new())
    }

    /// Prompt for test case metadata fields with recovery support and custom oracle
    pub fn prompt_metadata_with_recovery_oracle(
        recovered: Option<&TestCaseMetadata>,
        oracle: &impl Oracle,
    ) -> Result<TestCaseMetadata> {
        log::info!("\n=== Test Case Metadata ===\n");

        if recovered.is_some() {
            log::info!(
                "⚠ Recovered values shown as editable text (Enter confirms, you can edit/delete)\n"
            );
        }

        let requirement = Self::input_with_recovered_default_oracle(
            "Requirement",
            recovered.map(|m| m.requirement.as_str()),
            oracle,
        )?;
        let item =
            Self::input_integer_with_default_oracle("Item", recovered.map(|m| m.item), oracle)?;
        let tc = Self::input_integer_with_default_oracle("TC", recovered.map(|m| m.tc), oracle)?;
        let id = Self::input_with_recovered_default_oracle(
            "ID",
            recovered.map(|m| m.id.as_str()),
            oracle,
        )?;
        let description = Self::input_with_recovered_default_oracle(
            "Description",
            recovered.map(|m| m.description.as_str()),
            oracle,
        )?;

        Ok(TestCaseMetadata {
            requirement,
            item,
            tc,
            id,
            description,
        })
    }

    /// Prompt for general initial conditions with fuzzy search and editor support
    pub fn prompt_general_initial_conditions(
        defaults: Option<&Value>,
        validator: &SchemaValidator,
        editor_config: &EditorConfig,
    ) -> Result<Value> {
        Self::prompt_general_initial_conditions_with_oracle(
            defaults,
            validator,
            editor_config,
            &TtyCliOracle::new(),
        )
    }

    /// Prompt for general initial conditions with fuzzy search and editor support with custom oracle
    pub fn prompt_general_initial_conditions_with_oracle(
        defaults: Option<&Value>,
        validator: &SchemaValidator,
        editor_config: &EditorConfig,
        oracle: &impl Oracle,
    ) -> Result<Value> {
        log::info!("\n=== General Initial Conditions ===\n");

        if let Some(default_value) = defaults {
            let yaml_str =
                serde_yaml::to_string(default_value).context("Failed to serialize defaults")?;

            println!("Current defaults:");
            println!("{}", yaml_str);
            println!();

            let keep_defaults =
                Self::confirm_with_default_oracle("Keep these defaults?", true, oracle)?;

            if keep_defaults {
                return Ok(default_value.clone());
            }
        }

        let template = r#"# General Initial Conditions
# Example:
# - eUICC:
#     - "Condition 1"
#     - "Condition 2"

- eUICC:
    - ""
"#;

        let editor_flow = EditorFlow::new(editor_config.clone());
        let parsed = editor_flow.edit_with_validation_loop(template, |value: &Value| {
            let yaml_for_validation =
                serde_yaml::to_string(value).context("Failed to serialize for validation")?;
            validator.validate_partial_chunk(&yaml_for_validation)
        })?;

        log::info!("✓ Valid structure");
        Ok(parsed)
    }

    /// Prompt for general initial conditions with fuzzy search from database
    pub fn prompt_general_initial_conditions_with_search(
        defaults: Option<&Value>,
        validator: &SchemaValidator,
        storage: &crate::storage::TestCaseStorage,
        editor_config: &EditorConfig,
    ) -> Result<Value> {
        Self::prompt_general_initial_conditions_with_search_oracle(
            defaults,
            validator,
            storage,
            editor_config,
            &TtyCliOracle::new(),
        )
    }

    /// Prompt for general initial conditions with fuzzy search from database with custom oracle
    pub fn prompt_general_initial_conditions_with_search_oracle(
        defaults: Option<&Value>,
        validator: &SchemaValidator,
        storage: &crate::storage::TestCaseStorage,
        editor_config: &EditorConfig,
        oracle: &impl Oracle,
    ) -> Result<Value> {
        use crate::fuzzy::TestCaseFuzzyFinder;

        println!("\n=== General Initial Conditions ===\n");

        if let Some(default_value) = defaults {
            let yaml_str =
                serde_yaml::to_string(default_value).context("Failed to serialize defaults")?;

            println!("Current defaults:");
            println!("{}", yaml_str);
            println!();

            let keep_defaults =
                Self::confirm_with_default_oracle("Keep these defaults?", true, oracle)?;

            if keep_defaults {
                return Ok(default_value.clone());
            }
        }

        // Get existing general initial conditions from database
        let existing_conditions = storage
            .get_all_general_initial_conditions()
            .unwrap_or_else(|_| Vec::new());

        if !existing_conditions.is_empty() {
            println!(
                "Found {} existing general initial condition(s) in database.",
                existing_conditions.len()
            );

            if Self::confirm_with_oracle("Search existing general initial conditions?", oracle)? {
                if let Some(selected_yaml) = TestCaseFuzzyFinder::search_strings(
                    &existing_conditions,
                    "Select general initial condition: ",
                )? {
                    // Parse the selected YAML
                    let parsed: Value = serde_yaml::from_str(&selected_yaml)
                        .context("Failed to parse selected general initial conditions")?;

                    println!("\n✓ Selected existing general initial conditions:");
                    println!("{}", selected_yaml);

                    if Self::confirm_with_oracle("Use this as-is?", oracle)? {
                        return Ok(parsed);
                    } else if Self::confirm_with_oracle("Edit this condition?", oracle)? {
                        // Let user edit the selected condition
                        loop {
                            let edited_content =
                                TestCaseEditor::edit_text(&selected_yaml, editor_config)
                                    .context("Failed to open editor")?;

                            let parsed_edited: Value = serde_yaml::from_str(&edited_content)
                                .context("Failed to parse YAML")?;

                            let yaml_for_validation = serde_yaml::to_string(&parsed_edited)
                                .context("Failed to serialize for validation")?;

                            match validator.validate_partial_chunk(&yaml_for_validation) {
                                Ok(_) => {
                                    println!("✓ Valid structure");
                                    return Ok(parsed_edited);
                                }
                                Err(e) => {
                                    println!("✗ Validation failed: {}", e);
                                    let retry = Self::confirm_with_oracle("Try again?", oracle)?;
                                    if !retry {
                                        anyhow::bail!("Validation failed, user cancelled");
                                    }
                                }
                            }
                        }
                    }
                    // If user doesn't want to use or edit, fall through to create new
                }
            }
        }

        // Create new general initial condition
        println!("\nCreating new general initial condition...");
        loop {
            let template = r#"# General Initial Conditions
# Example:
# - eUICC:
#     - "Condition 1"
#     - "Condition 2"

- eUICC:
    - ""
"#;

            let edited_content = TestCaseEditor::edit_text(template, editor_config)
                .context("Failed to open editor")?;

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
                    let retry = Self::confirm_with_oracle("Try again?", oracle)?;
                    if !retry {
                        anyhow::bail!("Validation failed, user cancelled");
                    }
                }
            }
        }
    }

    /// Prompt for initial conditions with device selection and iterative condition collection
    pub fn prompt_initial_conditions(
        &self,
        defaults: Option<&Value>,
        validator: &SchemaValidator,
    ) -> Result<Value> {
        self.prompt_initial_conditions_with_oracle(defaults, validator, &TtyCliOracle::new())
    }

    /// Prompt for initial conditions with device selection and iterative condition collection with custom oracle
    pub fn prompt_initial_conditions_with_oracle(
        &self,
        defaults: Option<&Value>,
        validator: &SchemaValidator,
        oracle: &impl Oracle,
    ) -> Result<Value> {
        log::info!("\n=== Initial Conditions ===\n");

        if let Some(default_value) = defaults {
            let yaml_str =
                serde_yaml::to_string(default_value).context("Failed to serialize defaults")?;

            println!("Current defaults:");
            println!("{}", yaml_str);
            println!();

            let keep_defaults = if let Some(sample) = self.sample {
                self.confirm_with_sample_oracle(
                    "Keep these defaults?",
                    sample.confirm_keep_defaults(),
                    oracle,
                )?
            } else {
                Self::confirm_with_default_oracle("Keep these defaults?", true, oracle)?
            };

            if keep_defaults {
                return Ok(default_value.clone());
            }
        }

        // Get device names from database or use defaults
        let default_device_names = if let Some(db) = self.db {
            let db_names = db.get_device_names();
            if !db_names.is_empty() {
                db_names.to_vec()
            } else {
                vec!["eUICC".to_string(), "LPA".to_string(), "SM-DP+".to_string()]
            }
        } else {
            vec!["eUICC".to_string(), "LPA".to_string(), "SM-DP+".to_string()]
        };

        let device_name = if let Some(sample) = self.sample {
            self.input_with_sample_oracle(
                "Device name (e.g., eUICC)",
                &sample.initial_condition_device_name(),
                oracle,
            )?
        } else {
            match TestCaseFuzzyFinder::search_strings(
                &default_device_names,
                "Select device name (fuzzy search, or ESC to enter manually): ",
            )? {
                Some(name) => name,
                None => {
                    println!("No selection made, entering manually.");
                    Self::input_with_oracle("Device name (e.g., eUICC)", oracle)?
                }
            }
        };

        let mut conditions: Vec<String> = Vec::new();

        println!(
            "\nEnter conditions for '{}' (enter empty string to finish):",
            device_name
        );

        loop {
            let condition = if let Some(sample) = self.sample {
                let cond_value = sample.initial_condition(conditions.len());
                if sample.input_optional_stop(conditions.len()) {
                    None
                } else {
                    self.input_optional_with_sample_oracle(
                        &format!("Condition #{}", conditions.len() + 1),
                        &cond_value,
                        oracle,
                    )?
                }
            } else {
                Self::input_optional_with_oracle(
                    &format!("Condition #{}", conditions.len() + 1),
                    oracle,
                )?
            };

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

        log::info!("✓ Valid structure");

        Ok(initial_conditions_value)
    }

    /// Prompt for general initial conditions from database with fuzzy search
    pub fn prompt_general_initial_conditions_from_database<P: AsRef<Path>>(
        database_path: P,
        validator: &SchemaValidator,
        editor_config: &EditorConfig,
    ) -> Result<Value> {
        Self::prompt_general_initial_conditions_from_database_oracle(
            database_path,
            validator,
            editor_config,
            &TtyCliOracle::new(),
        )
    }

    /// Prompt for general initial conditions from database with fuzzy search with custom oracle
    pub fn prompt_general_initial_conditions_from_database_oracle<P: AsRef<Path>>(
        database_path: P,
        validator: &SchemaValidator,
        editor_config: &EditorConfig,
        oracle: &impl Oracle,
    ) -> Result<Value> {
        println!("\n=== General Initial Conditions (from database) ===\n");

        let db = ConditionDatabase::load_from_directory(database_path)
            .context("Failed to load condition database")?;

        let conditions = db.get_general_conditions();

        if conditions.is_empty() {
            println!("No general initial conditions found in database.");
            println!("Falling back to manual entry.\n");
            return Self::prompt_general_initial_conditions_with_oracle(
                None,
                validator,
                editor_config,
                oracle,
            );
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

                    if !Self::confirm_with_oracle("Add another general initial condition?", oracle)?
                    {
                        break;
                    }
                }
                None => {
                    if selected_conditions.is_empty() {
                        println!("No conditions selected.");
                        if Self::confirm_with_oracle("Use manual entry instead?", oracle)? {
                            return Self::prompt_general_initial_conditions_with_oracle(
                                None,
                                validator,
                                editor_config,
                                oracle,
                            );
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
        &self,
        database_path: P,
        validator: &SchemaValidator,
    ) -> Result<Value> {
        self.prompt_initial_conditions_from_database_oracle(
            database_path,
            validator,
            &TtyCliOracle::new(),
        )
    }

    /// Prompt for initial conditions from database with fuzzy search with custom oracle
    pub fn prompt_initial_conditions_from_database_oracle<P: AsRef<Path>>(
        &self,
        database_path: P,
        validator: &SchemaValidator,
        oracle: &impl Oracle,
    ) -> Result<Value> {
        println!("\n=== Initial Conditions (from database) ===\n");

        let db = ConditionDatabase::load_from_directory(database_path)
            .context("Failed to load condition database")?;

        let conditions = db.get_initial_conditions();

        if conditions.is_empty() {
            println!("No initial conditions found in database.");
            println!("Falling back to manual entry.\n");
            return self.prompt_initial_conditions_with_oracle(None, validator, oracle);
        }

        println!(
            "Loaded {} unique initial conditions from database\n",
            conditions.len()
        );

        // Fuzzy search for device name from database
        let device_names = db.get_device_names();
        let device_name = if !device_names.is_empty() {
            println!("\n=== Select Device Name ===");
            println!(
                "Available device names from database: {}\n",
                device_names.len()
            );

            match TestCaseFuzzyFinder::search_strings(
                device_names,
                "Select device name (or ESC to enter manually): ",
            )? {
                Some(name) => name,
                None => {
                    println!("No device name selected, entering manually.");
                    Self::input_with_oracle("Device name (e.g., eUICC)", oracle)?
                }
            }
        } else {
            println!("No device names found in database, using manual entry.");
            Self::input_with_oracle("Device name (e.g., eUICC)", oracle)?
        };

        println!("\nSelected device: {}\n", device_name);

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

                    if !Self::confirm_with_oracle("Add another initial condition?", oracle)? {
                        break;
                    }
                }
                None => {
                    if selected_conditions.is_empty() {
                        println!("No conditions selected.");
                        if Self::confirm_with_oracle("Use manual entry instead?", oracle)? {
                            return self
                                .prompt_initial_conditions_with_oracle(None, validator, oracle);
                        }
                    }
                    break;
                }
            }
        }

        if selected_conditions.is_empty() {
            anyhow::bail!("No initial conditions selected");
        }

        let conditions_values: Vec<Value> =
            selected_conditions.into_iter().map(Value::String).collect();

        let mut initial_cond_map = serde_yaml::Mapping::new();
        initial_cond_map.insert(
            Value::String(device_name),
            Value::Sequence(conditions_values),
        );

        Ok(Value::Mapping(initial_cond_map))
    }
}

impl Default for Prompts<'_> {
    fn default() -> Self {
        Self::new()
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
