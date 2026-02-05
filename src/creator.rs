use crate::complex_structure_editor::ComplexStructureEditor;
use crate::config::EditorConfig;
use crate::database::ConditionDatabase;
use crate::git::GitManager;
use crate::models::{Expected, Prerequisite, PrerequisiteType, Step, TestSequence};
use crate::oracle::Oracle;
use crate::prompts::Prompts;
use crate::sample::SampleData;
use crate::validation::SchemaValidator;
use crate::TestCaseMetadata;
use anyhow::{Context, Result};
use indexmap::IndexMap;
use serde_yaml::Value;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Encapsulates all test case creation logic with injected dependencies
pub struct TestCaseCreator {
    base_path: PathBuf,
    oracle: Arc<dyn Oracle>,
    editor_config: EditorConfig,
    database: Option<ConditionDatabase>,
    validator: SchemaValidator,
}

impl TestCaseCreator {
    /// Create a new TestCaseCreator with injected dependencies
    pub fn new<P: AsRef<Path>>(
        base_path: P,
        oracle: Arc<dyn Oracle>,
        editor_config: EditorConfig,
        database: Option<ConditionDatabase>,
    ) -> Result<Self> {
        let validator = SchemaValidator::new().context("Failed to create schema validator")?;

        Ok(Self {
            base_path: base_path.as_ref().to_path_buf(),
            oracle,
            editor_config,
            database,
            validator,
        })
    }

    /// Get a reference to the validator
    pub fn validator(&self) -> &SchemaValidator {
        &self.validator
    }

    /// Get a reference to the oracle
    pub fn oracle(&self) -> &Arc<dyn Oracle> {
        &self.oracle
    }

    /// Get a reference to the editor config
    pub fn editor_config(&self) -> &EditorConfig {
        &self.editor_config
    }

    /// Get a reference to the database (if available)
    pub fn database(&self) -> Option<&ConditionDatabase> {
        self.database.as_ref()
    }

    pub fn append_metadata(&self, metadata: TestCaseMetadata) -> Result<IndexMap<String, Value>> {
        log::info!("\n=== Validating Metadata ===");
        metadata
            .validate(&self.validator)
            .context("Metadata validation failed")?;
        log::info!("✓ Metadata is valid\n");

        let mut structure = IndexMap::new();

        let yaml_map = metadata.to_yaml();
        for (key, value) in yaml_map {
            structure.insert(key, value);
        }

        Ok(structure)
    }

    /// Prompt for and add metadata to the structure
    pub fn add_metadata(&self, structure: &mut IndexMap<String, Value>) -> Result<()> {
        let metadata = Prompts::prompt_metadata_with_oracle(&self.oracle)
            .context("Failed to prompt for metadata")?;

        log::info!("\n=== Validating Metadata ===");
        metadata
            .validate(&self.validator)
            .context("Metadata validation failed")?;
        log::info!("✓ Metadata is valid\n");

        let yaml_map = metadata.to_yaml();
        for (key, value) in yaml_map {
            structure.insert(key, value);
        }

        Ok(())
    }

    /// Prompt for and add prerequisites to the structure
    ///
    /// Interactively prompts the user to add prerequisites to the test case.
    /// Prerequisites can be either manual (requiring human verification) or automatic
    /// (verified programmatically with a command).
    ///
    /// # Flow
    /// 1. Asks if prerequisites should be added
    /// 2. For each prerequisite:
    ///    - Prompts for type selection (manual/automatic)
    ///    - Prompts for description
    ///    - If automatic, prompts for verification command
    /// 3. Asks if more prerequisites should be added
    ///
    /// # Example
    /// ```ignore
    /// let mut structure = IndexMap::new();
    /// creator.add_prerequisites(&mut structure)?;
    /// ```
    pub fn add_prerequisites(&self, structure: &mut IndexMap<String, Value>) -> Result<()> {
        log::info!("\n=== Prerequisites ===\n");

        if !Prompts::confirm_with_oracle("Add prerequisites?", &self.oracle)? {
            return Ok(());
        }

        let mut prerequisites = Vec::new();

        loop {
            log::info!("\n--- Adding Prerequisite #{} ---", prerequisites.len() + 1);

            let prerequisite = self.prompt_single_prerequisite()?;
            prerequisites.push(prerequisite);

            if !Prompts::confirm_with_oracle("\nAdd another prerequisite?", &self.oracle)? {
                break;
            }
        }

        if !prerequisites.is_empty() {
            let prerequisites_value = serde_yaml::to_value(&prerequisites)
                .context("Failed to convert prerequisites to YAML value")?;
            structure.insert("prerequisites".to_string(), prerequisites_value);
            log::info!("\n✓ Added {} prerequisite(s)\n", prerequisites.len());
        }

        Ok(())
    }

    /// Prompt for a single prerequisite
    ///
    /// Interactively collects information for one prerequisite, including:
    /// - Type (manual or automatic)
    /// - Description
    /// - Verification command (for automatic type only)
    fn prompt_single_prerequisite(&self) -> Result<Prerequisite> {
        let prerequisite_type = self.prompt_prerequisite_type()?;

        let description = Prompts::input_with_oracle("Prerequisite description", &self.oracle)?;

        let verification_command = match prerequisite_type {
            PrerequisiteType::Automatic => {
                let command = Prompts::input_with_oracle("Verification command", &self.oracle)?;
                Some(command)
            }
            PrerequisiteType::Manual => None,
        };

        Ok(Prerequisite {
            prerequisite_type,
            description,
            verification_command,
        })
    }

    /// Prompt for prerequisite type selection
    ///
    /// Presents a menu to select between:
    /// - `manual`: Requires human verification
    /// - `automatic`: Can be verified programmatically with a command
    fn prompt_prerequisite_type(&self) -> Result<PrerequisiteType> {
        let items = vec!["manual".to_string(), "automatic".to_string()];
        let selected = Prompts::select_with_oracle("Prerequisite type", items, &self.oracle)?;

        match selected.as_str() {
            "manual" => Ok(PrerequisiteType::Manual),
            "automatic" => Ok(PrerequisiteType::Automatic),
            _ => anyhow::bail!("Invalid prerequisite type: {}", selected),
        }
    }

    /// Add general initial conditions with interactive prompts
    pub fn add_general_initial_conditions(
        &self,
        structure: &mut IndexMap<String, Value>,
        defaults: Option<&Value>,
    ) -> Result<()> {
        let conditions = Prompts::prompt_general_initial_conditions_with_oracle(
            defaults,
            &self.validator,
            &self.editor_config,
            &self.oracle,
        )
        .context("Failed to prompt for general initial conditions")?;

        structure.insert("general_initial_conditions".to_string(), conditions);

        Ok(())
    }

    /// Add general initial conditions with fuzzy search from database
    pub fn add_general_initial_conditions_with_search(
        &self,
        structure: &mut IndexMap<String, Value>,
        defaults: Option<&Value>,
        storage: &crate::storage::TestCaseStorage,
    ) -> Result<()> {
        let conditions = Prompts::prompt_general_initial_conditions_with_search_oracle(
            defaults,
            &self.validator,
            storage,
            &self.editor_config,
            &self.oracle,
        )
        .context("Failed to prompt for general initial conditions")?;

        structure.insert("general_initial_conditions".to_string(), conditions);

        Ok(())
    }

    /// Add initial conditions with interactive prompts
    pub fn add_initial_conditions(
        &self,
        structure: &mut IndexMap<String, Value>,
        defaults: Option<&Value>,
    ) -> Result<()> {
        let prompts = if let Some(ref db) = self.database {
            Prompts::new_with_database(db)
        } else {
            Prompts::new()
        };
        let conditions = prompts
            .prompt_initial_conditions_with_oracle(defaults, &self.validator, &self.oracle)
            .context("Failed to prompt for initial conditions")?;

        structure.insert("initial_conditions".to_string(), conditions);

        Ok(())
    }

    /// Add general initial conditions from database with fuzzy search
    pub fn add_general_initial_conditions_from_database(
        &self,
        structure: &mut IndexMap<String, Value>,
        database: &ConditionDatabase,
    ) -> Result<()> {
        use crate::fuzzy::TestCaseFuzzyFinder;

        let conditions = database.get_general_conditions();

        if conditions.is_empty() {
            log::info!("No general initial conditions found in database.");
            return Ok(());
        }

        log::info!(
            "Loaded {} unique general initial conditions from database\n",
            conditions.len()
        );

        let mut selected_conditions = Vec::new();

        loop {
            log::info!("\n=== Select General Initial Condition ===");

            let selected = TestCaseFuzzyFinder::search_strings(
                conditions,
                "Select condition (ESC to finish): ",
            )?;

            match selected {
                Some(condition) => {
                    selected_conditions.push(condition.clone());
                    log::info!("✓ Added: {}\n", condition);

                    if !Prompts::confirm_with_oracle(
                        "Add another general initial condition?",
                        &self.oracle,
                    )? {
                        break;
                    }
                }
                None => {
                    if selected_conditions.is_empty() {
                        log::info!("No conditions selected.");
                        if !Prompts::confirm_with_oracle(
                            "Continue without general initial conditions?",
                            &self.oracle,
                        )? {
                            continue;
                        }
                    }
                    break;
                }
            }
        }

        if !selected_conditions.is_empty() {
            let euicc_conditions: Vec<Value> =
                selected_conditions.into_iter().map(Value::String).collect();

            let mut general_cond_map = serde_yaml::Mapping::new();
            general_cond_map.insert(
                Value::String("eUICC".to_string()),
                Value::Sequence(euicc_conditions),
            );

            let general_conditions_array = vec![Value::Mapping(general_cond_map)];

            structure.insert(
                "general_initial_conditions".to_string(),
                Value::Sequence(general_conditions_array),
            );

            log::info!("\n✓ General initial conditions added to test case");
        }

        Ok(())
    }

    /// Add initial conditions from database with fuzzy search
    pub fn add_initial_conditions_from_database(
        &self,
        structure: &mut IndexMap<String, Value>,
        database: &ConditionDatabase,
    ) -> Result<()> {
        use crate::fuzzy::TestCaseFuzzyFinder;

        let conditions = database.get_initial_conditions();

        if conditions.is_empty() {
            log::info!("No initial conditions found in database.");
            return Ok(());
        }

        log::info!(
            "Loaded {} unique initial conditions from database\n",
            conditions.len()
        );

        let mut selected_conditions = Vec::new();

        loop {
            log::info!("\n=== Select Initial Condition ===");

            let selected = TestCaseFuzzyFinder::search_strings(
                conditions,
                "Select condition (ESC to finish): ",
            )?;

            match selected {
                Some(condition) => {
                    selected_conditions.push(condition.clone());
                    log::info!("✓ Added: {}\n", condition);

                    if !Prompts::confirm_with_oracle(
                        "Add another initial condition?",
                        &self.oracle,
                    )? {
                        break;
                    }
                }
                None => {
                    if selected_conditions.is_empty() {
                        log::info!("No conditions selected.");
                        if !Prompts::confirm_with_oracle(
                            "Continue without initial conditions?",
                            &self.oracle,
                        )? {
                            continue;
                        }
                    }
                    break;
                }
            }
        }

        if !selected_conditions.is_empty() {
            let euicc_conditions: Vec<Value> =
                selected_conditions.into_iter().map(Value::String).collect();

            let mut initial_cond_map = serde_yaml::Mapping::new();
            initial_cond_map.insert(
                Value::String("eUICC".to_string()),
                Value::Sequence(euicc_conditions),
            );

            structure.insert(
                "initial_conditions".to_string(),
                Value::Mapping(initial_cond_map),
            );

            log::info!("\n✓ Initial conditions added to test case");
        }

        Ok(())
    }

    /// Get the next test sequence ID
    pub fn get_next_sequence_id(&self, structure: &IndexMap<String, Value>) -> i64 {
        if let Some(Value::Sequence(sequences)) = structure.get("test_sequences") {
            let max_id = sequences
                .iter()
                .filter_map(|seq| {
                    if let Value::Mapping(map) = seq {
                        map.get(Value::String("id".to_string()))
                            .and_then(|v| match v {
                                Value::Number(n) => n.as_i64(),
                                _ => None,
                            })
                    } else {
                        None
                    }
                })
                .max()
                .unwrap_or(0);
            max_id + 1
        } else {
            1
        }
    }

    /// Add a test sequence with interactive prompts
    pub fn add_test_sequence_interactive(
        &self,
        structure: &mut IndexMap<String, Value>,
        sample: Option<&SampleData>,
    ) -> Result<()> {
        log::info!("\n=== Add Test Sequence ===\n");

        let sequence_id = self.get_next_sequence_id(structure);
        log::debug!("Sequence ID: {}", sequence_id);

        let db = ConditionDatabase::load_from_directory(&self.base_path)
            .context("Failed to load condition database")?;

        let sequence_items = db.get_all_sequences();

        // Create a template for a new sequence
        let template = format!(
            r#"id: {}
name: ""
description: ""
initial_conditions: {{}}
steps: []
"#,
            sequence_id
        );

        // Use ComplexStructureEditor to allow fuzzy search and editing
        let mut edited_sequence = if !sequence_items.is_empty() {
            match ComplexStructureEditor::<TestSequence>::edit_with_fuzzy_search(
                &sequence_items,
                "Select sequence template (ESC to create new): ",
                self.oracle.as_ref(),
                &self.editor_config,
                &self.validator,
                &template,
            ) {
                Ok(mut seq) => {
                    // Update the ID to the next available ID
                    seq.id = sequence_id;
                    seq
                }
                Err(e) => {
                    log::warn!("Fuzzy search failed or cancelled: {}", e);
                    log::info!("Falling back to manual entry");
                    return self.add_test_sequence_interactive_fallback(structure, sample);
                }
            }
        } else {
            log::info!("No sequences in database, using manual entry.");
            return self.add_test_sequence_interactive_fallback(structure, sample);
        };

        // Clear steps from template since we're creating a new sequence
        edited_sequence.steps.clear();

        // Convert to YAML Value
        let sequence_value = serde_yaml::to_value(&edited_sequence)
            .context("Failed to convert TestSequence to YAML value")?;

        log::info!("\n=== Validating Test Sequence ===");
        Self::validate_and_append_sequence(structure, sequence_value)?;
        log::info!("✓ Test sequence validated and added\n");

        Ok(())
    }

    /// Fallback method for adding test sequence when database is unavailable or fuzzy search is cancelled
    fn add_test_sequence_interactive_fallback(
        &self,
        structure: &mut IndexMap<String, Value>,
        sample: Option<&SampleData>,
    ) -> Result<()> {
        use crate::editor::TestCaseEditor;
        use crate::fuzzy::TestCaseFuzzyFinder;

        let sequence_id = self.get_next_sequence_id(structure);

        let existing_sequences = Self::get_existing_sequence_names(structure);
        let sequence_name = if let Some(sample) = sample {
            let prompts = Prompts::new_with_sample(sample);
            prompts.input_with_sample_oracle(
                "Sequence name",
                &sample.sequence_name(),
                &self.oracle,
            )?
        } else if !existing_sequences.is_empty() {
            log::info!("\nYou can select from existing sequence names or type a new one.");

            if Prompts::confirm_with_oracle(
                "Use fuzzy search to select from existing names?",
                &self.oracle,
            )? {
                match TestCaseFuzzyFinder::search_strings(
                    &existing_sequences,
                    "Select sequence name: ",
                )? {
                    Some(name) => name,
                    None => {
                        log::info!("No selection made, entering new name.");
                        Prompts::input_with_oracle("Sequence name", &self.oracle)?
                    }
                }
            } else {
                Prompts::input_with_oracle("Sequence name", &self.oracle)?
            }
        } else {
            Prompts::input_with_oracle("Sequence name", &self.oracle)?
        };

        let description = if let Some(sample) = sample {
            let prompts = Prompts::new_with_sample(sample);
            if let Some(desc) = sample.sequence_description() {
                prompts.input_optional_with_sample_oracle("Description", &desc, &self.oracle)?
            } else {
                None
            }
        } else if Prompts::confirm_with_oracle("\nEdit description in editor?", &self.oracle)? {
            let template = format!(
                "# Description for: {}\n# Enter the sequence description below:\n\n",
                sequence_name
            );
            let edited = TestCaseEditor::edit_text(&template, &self.editor_config)?;

            let cleaned: String = edited
                .lines()
                .filter(|line| !line.trim().starts_with('#'))
                .collect::<Vec<&str>>()
                .join("\n")
                .trim()
                .to_string();

            if cleaned.is_empty() {
                None
            } else {
                Some(cleaned)
            }
        } else {
            Prompts::input_optional_with_oracle("Description", &self.oracle)?
        };

        let add_initial_conditions = if let Some(sample) = sample {
            let prompts = Prompts::new_with_sample(sample);
            prompts.confirm_with_sample_oracle(
                "\nAdd sequence-specific initial conditions?",
                sample.confirm_add_sequence_initial_conditions(),
                &self.oracle,
            )?
        } else {
            Prompts::confirm_with_oracle(
                "\nAdd sequence-specific initial conditions?",
                &self.oracle,
            )?
        };

        let db = ConditionDatabase::load_from_directory(&self.base_path)
            .context("Failed to load condition database")?;

        let prompts = if let Some(sample) = sample {
            Prompts::new_with_database_and_sample(&db, sample)
        } else {
            Prompts::new_with_database(&db)
        };

        let initial_conditions = if add_initial_conditions {
            let use_db = if let Some(sample) = sample {
                let sample_prompts = Prompts::new_with_sample(sample);
                sample_prompts.confirm_with_sample_oracle(
                    "Use database for initial conditions?",
                    sample.confirm_use_database(),
                    &self.oracle,
                )?
            } else {
                Prompts::confirm_with_oracle("Use database for initial conditions?", &self.oracle)?
            };

            if use_db {
                let conditions = db.get_initial_conditions();

                if !conditions.is_empty() {
                    let mut selected_conditions = Vec::new();

                    loop {
                        let selected = TestCaseFuzzyFinder::search_strings(
                            conditions,
                            "Select condition (ESC to finish): ",
                        )?;

                        match selected {
                            Some(condition) => {
                                selected_conditions.push(condition.clone());
                                log::info!("✓ Added: {}", condition);

                                if !Prompts::confirm_with_oracle(
                                    "Add another condition?",
                                    &self.oracle,
                                )? {
                                    break;
                                }
                            }
                            None => break,
                        }
                    }

                    if !selected_conditions.is_empty() {
                        let euicc_conditions: Vec<Value> =
                            selected_conditions.into_iter().map(Value::String).collect();

                        let mut initial_cond_map = serde_yaml::Mapping::new();
                        initial_cond_map.insert(
                            Value::String("eUICC".to_string()),
                            Value::Sequence(euicc_conditions),
                        );

                        Some(Value::Mapping(initial_cond_map))
                    } else {
                        None
                    }
                } else {
                    log::info!("No conditions in database, using manual entry.");
                    Some(prompts.prompt_initial_conditions_with_oracle(
                        None,
                        &self.validator,
                        &self.oracle,
                    )?)
                }
            } else {
                Some(prompts.prompt_initial_conditions_with_oracle(
                    None,
                    &self.validator,
                    &self.oracle,
                )?)
            }
        } else {
            None
        };

        let mut sequence_map = serde_yaml::Mapping::new();
        sequence_map.insert(
            Value::String("id".to_string()),
            Value::Number(sequence_id.into()),
        );
        sequence_map.insert(
            Value::String("name".to_string()),
            Value::String(sequence_name.clone()),
        );

        if let Some(desc) = description {
            sequence_map.insert(
                Value::String("description".to_string()),
                Value::String(desc),
            );
        }

        if let Some(ic) = initial_conditions {
            let ic_array = vec![ic];
            sequence_map.insert(
                Value::String("initial_conditions".to_string()),
                Value::Sequence(ic_array),
            );
        }

        sequence_map.insert(
            Value::String("steps".to_string()),
            Value::Sequence(Vec::new()),
        );

        let sequence_value = Value::Mapping(sequence_map);

        log::info!("\n=== Validating Test Sequence ===");
        Self::validate_and_append_sequence(structure, sequence_value)?;
        log::info!("✓ Test sequence validated and added\n");

        Ok(())
    }

    /// Get existing sequence names from the structure
    pub fn get_existing_sequence_names(structure: &IndexMap<String, Value>) -> Vec<String> {
        let mut names = Vec::new();

        if let Some(Value::Sequence(sequences)) = structure.get("test_sequences") {
            for seq in sequences {
                if let Value::Mapping(map) = seq {
                    if let Some(Value::String(name)) = map.get(Value::String("name".to_string())) {
                        names.push(name.clone());
                    }
                }
            }
        }

        names
    }

    /// Validate a test sequence structure and append it to test_sequences
    pub fn validate_and_append_sequence(
        structure: &mut IndexMap<String, Value>,
        sequence: Value,
    ) -> Result<()> {
        if let Value::Mapping(seq_map) = &sequence {
            if !seq_map.contains_key(Value::String("id".to_string())) {
                anyhow::bail!("Sequence must have an 'id' field");
            }
            if !seq_map.contains_key(Value::String("name".to_string())) {
                anyhow::bail!("Sequence must have a 'name' field");
            }
            if !seq_map.contains_key(Value::String("steps".to_string())) {
                anyhow::bail!("Sequence must have a 'steps' field");
            }
        } else {
            anyhow::bail!("Sequence must be a mapping");
        }

        let sequences = structure
            .entry("test_sequences".to_string())
            .or_insert_with(|| Value::Sequence(Vec::new()));

        if let Value::Sequence(seq_vec) = sequences {
            seq_vec.push(sequence);
        } else {
            anyhow::bail!("test_sequences must be a sequence");
        }

        Ok(())
    }

    /// Helper method to prompt for step fields individually
    pub fn prompt_for_step_fields(&self, step_number: i64) -> Result<Step> {
        let description = Prompts::input_with_oracle("Step description", &self.oracle)?;

        let manual = if Prompts::confirm_with_oracle("Is this a manual step?", &self.oracle)? {
            Some(true)
        } else {
            None
        };

        let command = Prompts::input_with_oracle("Command", &self.oracle)?;

        let expected_value = self.prompt_for_expected()?;
        let expected: Expected = serde_yaml::from_value(expected_value)
            .context("Failed to convert expected value to Expected struct")?;

        // Use template-based verification prompts
        let verification = Prompts::prompt_verification_with_templates(&self.oracle)?;

        Ok(Step {
            step: step_number,
            manual,
            description,
            command,
            capture_vars: None,
            expected,
            verification,
        })
    }

    /// Get sequence ID by index
    pub fn get_sequence_id_by_index(
        &self,
        structure: &IndexMap<String, Value>,
        index: usize,
    ) -> Result<i64> {
        if let Some(Value::Sequence(sequences)) = structure.get("test_sequences") {
            if let Some(Value::Mapping(seq_map)) = sequences.get(index) {
                if let Some(Value::Number(id)) = seq_map.get(Value::String("id".to_string())) {
                    return id
                        .as_i64()
                        .ok_or_else(|| anyhow::anyhow!("Invalid sequence ID"));
                }
            }
        }
        anyhow::bail!("Sequence not found at index {}", index)
    }

    /// Get sequence name by index
    pub fn get_sequence_name_by_index(
        &self,
        structure: &IndexMap<String, Value>,
        index: usize,
    ) -> Result<String> {
        if let Some(Value::Sequence(sequences)) = structure.get("test_sequences") {
            if let Some(Value::Mapping(seq_map)) = sequences.get(index) {
                if let Some(Value::String(name)) = seq_map.get(Value::String("name".to_string())) {
                    return Ok(name.clone());
                }
            }
        }
        anyhow::bail!("Sequence not found at index {}", index)
    }

    /// Get all existing steps
    pub fn get_all_existing_steps(&self, structure: &IndexMap<String, Value>) -> Vec<String> {
        let mut descriptions = Vec::new();

        if let Some(Value::Sequence(sequences)) = structure.get("test_sequences") {
            for seq in sequences {
                if let Value::Mapping(seq_map) = seq {
                    if let Some(Value::Sequence(steps)) =
                        seq_map.get(Value::String("steps".to_string()))
                    {
                        for step in steps {
                            if let Value::Mapping(step_map) = step {
                                if let Some(Value::String(desc)) =
                                    step_map.get(Value::String("description".to_string()))
                                {
                                    if !descriptions.contains(desc) {
                                        descriptions.push(desc.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        descriptions
    }

    /// Prompt for expected result
    pub fn prompt_for_expected(&self) -> Result<Value> {
        if let Some(ref db) = self.database {
            let expected_items = db.get_all_expected();

            if !expected_items.is_empty() {
                let template = r#"# success: true  # Optional field
result: ""
output: ""
"#;

                match ComplexStructureEditor::<Expected>::edit_with_fuzzy_search(
                    &expected_items,
                    "Select expected result (ESC to skip): ",
                    self.oracle.as_ref(),
                    &self.editor_config,
                    &self.validator,
                    template,
                ) {
                    Ok(expected) => {
                        let yaml_value = serde_yaml::to_value(&expected)
                            .context("Failed to convert Expected to YAML value")?;
                        return Ok(yaml_value);
                    }
                    Err(e) => {
                        log::warn!("Fuzzy search failed or cancelled: {}", e);
                        log::info!("Falling back to field-by-field prompts");
                    }
                }
            }
        }

        let include_success =
            Prompts::confirm_with_oracle("Include 'success' field?", &self.oracle)?;
        let result = Prompts::input_with_oracle("Expected result", &self.oracle)?;
        let output = Prompts::input_with_oracle("Expected output", &self.oracle)?;
        let success_value = if include_success {
            Prompts::confirm_with_oracle("Success value (true/false)?", &self.oracle)?
        } else {
            false
        };

        let mut expected_map = serde_yaml::Mapping::new();

        if include_success {
            expected_map.insert(
                Value::String("success".to_string()),
                Value::Bool(success_value),
            );
        }

        expected_map.insert(Value::String("result".to_string()), Value::String(result));
        expected_map.insert(Value::String("output".to_string()), Value::String(output));

        Ok(Value::Mapping(expected_map))
    }

    /// Get the next step number
    pub fn get_next_step_number(
        &self,
        structure: &IndexMap<String, Value>,
        sequence_index: usize,
    ) -> Result<i64> {
        if let Some(Value::Sequence(sequences)) = structure.get("test_sequences") {
            if let Some(Value::Mapping(seq_map)) = sequences.get(sequence_index) {
                if let Some(Value::Sequence(steps)) =
                    seq_map.get(Value::String("steps".to_string()))
                {
                    let max_step = steps
                        .iter()
                        .filter_map(|step| {
                            if let Value::Mapping(step_map) = step {
                                step_map.get(Value::String("step".to_string())).and_then(
                                    |v| match v {
                                        Value::Number(n) => n.as_i64(),
                                        _ => None,
                                    },
                                )
                            } else {
                                None
                            }
                        })
                        .max()
                        .unwrap_or(0);
                    return Ok(max_step + 1);
                }
            }
        }
        anyhow::bail!("Sequence not found at index {}", sequence_index)
    }

    /// Create a step value structure
    pub fn create_step_value(
        &self,
        step_number: i64,
        manual: Option<bool>,
        description: String,
        command: String,
        expected: Value,
    ) -> Result<Value> {
        let mut step_map = serde_yaml::Mapping::new();

        step_map.insert(
            Value::String("step".to_string()),
            Value::Number(step_number.into()),
        );

        if let Some(is_manual) = manual {
            step_map.insert(Value::String("manual".to_string()), Value::Bool(is_manual));
        }

        step_map.insert(
            Value::String("description".to_string()),
            Value::String(description),
        );

        step_map.insert(Value::String("command".to_string()), Value::String(command));

        step_map.insert(Value::String("expected".to_string()), expected);

        Ok(Value::Mapping(step_map))
    }

    /// Validate a step structure and append it to the sequence
    pub fn validate_and_append_step(
        &self,
        structure: &mut IndexMap<String, Value>,
        sequence_index: usize,
        step: Value,
    ) -> Result<()> {
        if let Value::Mapping(step_map) = &step {
            if !step_map.contains_key(Value::String("step".to_string())) {
                anyhow::bail!("Step must have a 'step' field");
            }
            if !step_map.contains_key(Value::String("description".to_string())) {
                anyhow::bail!("Step must have a 'description' field");
            }
            if !step_map.contains_key(Value::String("command".to_string())) {
                anyhow::bail!("Step must have a 'command' field");
            }
            if !step_map.contains_key(Value::String("expected".to_string())) {
                anyhow::bail!("Step must have an 'expected' field");
            }

            if let Some(Value::Mapping(expected_map)) =
                step_map.get(Value::String("expected".to_string()))
            {
                if !expected_map.contains_key(Value::String("result".to_string())) {
                    anyhow::bail!("Expected must have a 'result' field");
                }
                if !expected_map.contains_key(Value::String("output".to_string())) {
                    anyhow::bail!("Expected must have an 'output' field");
                }
            } else {
                anyhow::bail!("Expected must be a mapping");
            }
        } else {
            anyhow::bail!("Step must be a mapping");
        }

        let sequences = structure
            .get_mut("test_sequences")
            .ok_or_else(|| anyhow::anyhow!("test_sequences not found"))?;

        if let Value::Sequence(seq_vec) = sequences {
            if let Some(Value::Mapping(seq_map)) = seq_vec.get_mut(sequence_index) {
                if let Some(Value::Sequence(steps)) =
                    seq_map.get_mut(Value::String("steps".to_string()))
                {
                    steps.push(step);
                } else {
                    anyhow::bail!("steps field is not a sequence");
                }
            } else {
                anyhow::bail!("Sequence at index {} is not a mapping", sequence_index);
            }
        } else {
            anyhow::bail!("test_sequences is not a sequence");
        }

        Ok(())
    }

    /// Save a structure to a YAML file with git operations decorator
    pub fn save_with_git(
        &self,
        structure: &IndexMap<String, Value>,
        git_manager: Option<&GitManager>,
        message: &str,
    ) -> Result<PathBuf> {
        let yaml_content = Self::structure_to_yaml_string(structure)?;
        let file_name = Self::get_file_name(structure)?;
        let file_path = self.base_path.join(&file_name);

        std::fs::write(&file_path, yaml_content).context("Failed to write YAML file")?;

        // Apply git operations as decorator
        if let Some(git) = git_manager {
            let author_name = std::env::var("GIT_AUTHOR_NAME")
                .unwrap_or_else(|_| "Test Case Manager".to_string());
            let author_email = std::env::var("GIT_AUTHOR_EMAIL")
                .unwrap_or_else(|_| "testcase@example.com".to_string());

            git.commit_progress(&file_name, message, &author_name, &author_email)
                .context("Failed to commit to git")?;

            log::info!("✓ Committed: {}", message);
        }

        Ok(file_path)
    }

    /// Convert structure to YAML string
    pub fn structure_to_yaml_string(structure: &IndexMap<String, Value>) -> Result<String> {
        let yaml_value = Value::Mapping(serde_yaml::Mapping::from_iter(
            structure
                .iter()
                .map(|(k, v)| (Value::String(k.clone()), v.clone())),
        ));

        serde_yaml::to_string(&yaml_value).context("Failed to serialize structure to YAML")
    }

    /// Get the file name for this test case
    fn get_file_name(structure: &IndexMap<String, Value>) -> Result<String> {
        if let Some(Value::String(id)) = structure.get("id") {
            Ok(format!("{}.yaml", id.replace(' ', "_")))
        } else {
            anyhow::bail!("ID field not found in structure")
        }
    }
}
