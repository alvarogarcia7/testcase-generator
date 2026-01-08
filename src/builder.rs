use crate::config::EditorConfig;
use crate::database::ConditionDatabase;
use crate::fuzzy::{FuzzySearchResult, TestCaseFuzzyFinder};
use crate::git::GitManager;
use crate::prompts::Prompts;
use crate::recovery::{RecoveryManager, RecoveryState};
use crate::sample::SampleData;
use crate::validation::SchemaValidator;
use anyhow::{Context, Result};
use indexmap::IndexMap;
use serde_yaml::Value;
use std::path::{Path, PathBuf};

/// Builder for creating test cases interactively
pub struct TestCaseBuilder {
    base_path: PathBuf,
    validator: SchemaValidator,
    git_manager: Option<GitManager>,
    structure: IndexMap<String, Value>,
    recovery_manager: RecoveryManager,
    db: Option<ConditionDatabase>,
    sample: Option<SampleData>,
}

impl TestCaseBuilder {
    /// Create a new test case builder
    pub fn new<P: AsRef<Path>>(base_path: P) -> Result<Self> {
        let base_path = base_path.as_ref().to_path_buf();
        let validator = SchemaValidator::new().context("Failed to create schema validator")?;

        let git_manager = GitManager::open(&base_path)
            .or_else(|_| GitManager::init(&base_path))
            .ok();

        let recovery_manager = RecoveryManager::new(&base_path);

        // Try to load database from base_path
        let db = ConditionDatabase::load_from_directory(&base_path).ok();

        Ok(Self {
            base_path,
            validator,
            git_manager,
            structure: IndexMap::new(),
            recovery_manager,
            db,
            sample: None,
        })
    }

    /// Create a new test case builder and check for recovery
    pub fn new_with_recovery<P: AsRef<Path>>(base_path: P) -> Result<Self> {
        let base_path = base_path.as_ref().to_path_buf();
        let validator = SchemaValidator::new().context("Failed to create schema validator")?;

        let git_manager = GitManager::open(&base_path)
            .or_else(|_| GitManager::init(&base_path))
            .ok();

        let recovery_manager = RecoveryManager::new(&base_path);

        let structure = if recovery_manager.prompt_for_recovery()? {
            if let Some(state) = recovery_manager.load_state()? {
                println!("✓ Resuming from saved state\n");
                state.structure
            } else {
                IndexMap::new()
            }
        } else {
            IndexMap::new()
        };

        // Try to load database from base_path
        let db = ConditionDatabase::load_from_directory(&base_path).ok();

        Ok(Self {
            base_path,
            validator,
            git_manager,
            structure,
            recovery_manager,
            db,
            sample: None,
        })
    }

    /// Set sample data for the builder
    pub fn with_sample(mut self, sample: SampleData) -> Self {
        self.sample = Some(sample);
        self
    }

    /// Enable sample mode
    pub fn enable_sample_mode(&mut self) {
        self.sample = Some(SampleData::new());
    }

    /// Get a reference to the sample data if available
    pub fn sample(&self) -> Option<&SampleData> {
        self.sample.as_ref()
    }

    /// Save current state to recovery file
    pub fn save_recovery_state(&self, phase: &str) -> Result<()> {
        let state = RecoveryState::new(self.structure.clone(), phase.to_string());
        self.recovery_manager.save_state(&state)?;
        Ok(())
    }

    /// Save current state with validation errors to recovery file
    pub fn save_recovery_state_with_errors(
        &self,
        phase: &str,
        error: &anyhow::Error,
    ) -> Result<()> {
        let validation_errors = RecoveryManager::extract_validation_errors_from_anyhow(error);
        let state = RecoveryState::with_errors(
            self.structure.clone(),
            validation_errors,
            phase.to_string(),
        );
        self.recovery_manager.save_state(&state)?;
        Ok(())
    }

    /// Delete the recovery file
    pub fn delete_recovery_file(&self) -> Result<()> {
        self.recovery_manager.delete_recovery_file()
    }

    /// Get the recovery manager
    pub fn recovery_manager(&self) -> &RecoveryManager {
        &self.recovery_manager
    }

    /// Get the validator
    pub fn validator(&self) -> &SchemaValidator {
        &self.validator
    }

    /// Prompt for and add metadata to the structure
    pub fn add_metadata(&mut self) -> Result<&mut Self> {
        let metadata = Prompts::prompt_metadata().context("Failed to prompt for metadata")?;

        println!("\n=== Validating Metadata ===");
        metadata
            .validate(&self.validator)
            .context("Metadata validation failed")?;
        println!("✓ Metadata is valid\n");

        let yaml_map = metadata.to_yaml();
        for (key, value) in yaml_map {
            self.structure.insert(key, value);
        }

        Ok(self)
    }

    /// Commit the current structure to git
    pub fn commit(&self, message: &str) -> Result<()> {
        if let Some(git) = &self.git_manager {
            let yaml_content = self.to_yaml_string()?;
            let file_name = self.get_file_name()?;
            let file_path = self.base_path.join(&file_name);

            std::fs::write(&file_path, yaml_content).context("Failed to write YAML file")?;

            let author_name = std::env::var("GIT_AUTHOR_NAME")
                .unwrap_or_else(|_| "Test Case Manager".to_string());
            let author_email = std::env::var("GIT_AUTHOR_EMAIL")
                .unwrap_or_else(|_| "testcase@example.com".to_string());

            git.commit_progress(&file_name, message, &author_name, &author_email)
                .context("Failed to commit to git")?;

            println!("✓ Committed: {}", message);
        } else {
            println!("⚠ Git repository not available, skipping commit");
        }

        Ok(())
    }

    /// Add general initial conditions with interactive prompts
    pub fn add_general_initial_conditions(
        &mut self,
        defaults: Option<&Value>,
    ) -> Result<&mut Self> {
        let editor_config = EditorConfig::load();
        let conditions =
            Prompts::prompt_general_initial_conditions(defaults, &self.validator, &editor_config)
                .context("Failed to prompt for general initial conditions")?;

        self.structure
            .insert("general_initial_conditions".to_string(), conditions);

        Ok(self)
    }

    /// Add general initial conditions with fuzzy search from database
    pub fn add_general_initial_conditions_with_search(
        &mut self,
        defaults: Option<&Value>,
        storage: &crate::storage::TestCaseStorage,
    ) -> Result<&mut Self> {
        let editor_config = EditorConfig::load();
        let conditions = Prompts::prompt_general_initial_conditions_with_search(
            defaults,
            &self.validator,
            storage,
            &editor_config,
        )
        .context("Failed to prompt for general initial conditions")?;

        self.structure
            .insert("general_initial_conditions".to_string(), conditions);

        Ok(self)
    }

    /// Add initial conditions with interactive prompts
    pub fn add_initial_conditions(&mut self, defaults: Option<&Value>) -> Result<&mut Self> {
        let prompts = if let Some(ref db) = self.db {
            Prompts::new_with_database(db)
        } else {
            Prompts::new()
        };
        let conditions = prompts
            .prompt_initial_conditions(defaults, &self.validator)
            .context("Failed to prompt for initial conditions")?;

        self.structure
            .insert("initial_conditions".to_string(), conditions);

        Ok(self)
    }

    /// Add general initial conditions from database with fuzzy search
    pub fn add_general_initial_conditions_from_database<P: AsRef<Path>>(
        &mut self,
        database_path: P,
    ) -> Result<&mut Self> {
        let db = ConditionDatabase::load_from_directory(database_path)
            .context("Failed to load condition database")?;

        let conditions = db.get_general_conditions();

        if conditions.is_empty() {
            println!("No general initial conditions found in database.");
            return Ok(self);
        }

        println!(
            "Loaded {} unique general initial conditions from database\n",
            conditions.len()
        );

        let mut selected_conditions = Vec::new();

        loop {
            println!("\n=== Select General Initial Condition ===");

            let selected = TestCaseFuzzyFinder::search_strings(
                conditions,
                "Select condition (ESC to finish): ",
            )?;

            match selected {
                FuzzySearchResult::Selected(condition) => {
                    selected_conditions.push(condition.clone());
                    println!("✓ Added: {}\n", condition);

                    if !Prompts::confirm("Add another general initial condition?")? {
                        break;
                    }
                }
                FuzzySearchResult::Cancelled => {
                    if selected_conditions.is_empty() {
                        println!("No conditions selected.");
                        if !Prompts::confirm("Continue without general initial conditions?")? {
                            continue;
                        }
                    }
                    break;
                }
                FuzzySearchResult::Error(e) => {
                    return Err(anyhow::anyhow!("Search error: {}", e));
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

            self.structure.insert(
                "general_initial_conditions".to_string(),
                Value::Sequence(general_conditions_array),
            );

            println!("\n✓ General initial conditions added to test case");
        }

        Ok(self)
    }

    /// Add initial conditions from database with fuzzy search
    pub fn add_initial_conditions_from_database<P: AsRef<Path>>(
        &mut self,
        database_path: P,
    ) -> Result<&mut Self> {
        let db = ConditionDatabase::load_from_directory(database_path)
            .context("Failed to load condition database")?;

        let conditions = db.get_initial_conditions();

        if conditions.is_empty() {
            println!("No initial conditions found in database.");
            return Ok(self);
        }

        println!(
            "Loaded {} unique initial conditions from database\n",
            conditions.len()
        );

        let mut selected_conditions = Vec::new();

        loop {
            println!("\n=== Select Initial Condition ===");

            let selected = TestCaseFuzzyFinder::search_strings(
                conditions,
                "Select condition (ESC to finish): ",
            )?;

            match selected {
                FuzzySearchResult::Selected(condition) => {
                    selected_conditions.push(condition.clone());
                    println!("✓ Added: {}\n", condition);

                    if !Prompts::confirm("Add another initial condition?")? {
                        break;
                    }
                }
                FuzzySearchResult::Cancelled => {
                    if selected_conditions.is_empty() {
                        println!("No conditions selected.");
                        if !Prompts::confirm("Continue without initial conditions?")? {
                            continue;
                        }
                    }
                    break;
                }
                FuzzySearchResult::Error(e) => {
                    return Err(anyhow::anyhow!("Search error: {}", e));
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

            self.structure.insert(
                "initial_conditions".to_string(),
                Value::Mapping(initial_cond_map),
            );

            println!("\n✓ Initial conditions added to test case");
        }

        Ok(self)
    }

    /// Add a custom field with validation
    pub fn add_field(&mut self, key: String, value: Value) -> Result<&mut Self> {
        self.structure.insert(key, value);
        Ok(self)
    }

    /// Validate the entire structure
    pub fn validate(&self) -> Result<()> {
        let yaml_content = self.to_yaml_string()?;
        self.validator
            .validate_chunk(&yaml_content)
            .context("Structure validation failed")
    }

    /// Convert the structure to a YAML string
    pub fn to_yaml_string(&self) -> Result<String> {
        let yaml_value = Value::Mapping(serde_yaml::Mapping::from_iter(
            self.structure
                .iter()
                .map(|(k, v)| (Value::String(k.clone()), v.clone())),
        ));

        serde_yaml::to_string(&yaml_value).context("Failed to serialize structure to YAML")
    }

    /// Get the file name for this test case
    fn get_file_name(&self) -> Result<String> {
        if let Some(Value::String(id)) = self.structure.get("id") {
            Ok(format!("{}.yaml", id.replace(' ', "_")))
        } else {
            anyhow::bail!("ID field not found in structure")
        }
    }

    /// Save the structure to a file
    pub fn save(&self) -> Result<PathBuf> {
        let yaml_content = self.to_yaml_string()?;
        let file_name = self.get_file_name()?;
        let file_path = self.base_path.join(&file_name);

        std::fs::write(&file_path, yaml_content).context("Failed to write YAML file")?;

        Ok(file_path)
    }

    /// Get a reference to the current structure
    pub fn structure(&self) -> &IndexMap<String, Value> {
        &self.structure
    }

    /// Get a mutable reference to the current structure
    pub fn structure_mut(&mut self) -> &mut IndexMap<String, Value> {
        &mut self.structure
    }

    /// Get the next test sequence ID
    pub fn get_next_sequence_id(&self) -> i64 {
        if let Some(Value::Sequence(sequences)) = self.structure.get("test_sequences") {
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
    pub fn add_test_sequence_interactive(&mut self) -> Result<&mut Self> {
        use crate::editor::TestCaseEditor;
        use crate::prompts::Prompts;

        println!("\n=== Add Test Sequence ===\n");

        let sequence_id = self.get_next_sequence_id();
        println!("Sequence ID: {}", sequence_id);

        let existing_sequences = self.get_existing_sequence_names();
        let sequence_name = if let Some(sample) = &self.sample {
            let prompts = Prompts::new_with_sample(sample);
            prompts.input_with_sample("Sequence name", &sample.sequence_name())?
        } else if !existing_sequences.is_empty() {
            println!("\nYou can select from existing sequence names or type a new one.");

            if Prompts::confirm("Use fuzzy search to select from existing names?")? {
                match TestCaseFuzzyFinder::search_strings(
                    &existing_sequences,
                    "Select sequence name: ",
                )? {
                    FuzzySearchResult::Selected(name) => name,
                    FuzzySearchResult::Cancelled => {
                        println!("No selection made, entering new name.");
                        Prompts::input("Sequence name")?
                    }
                    FuzzySearchResult::Error(e) => {
                        return Err(anyhow::anyhow!("Search error: {}", e));
                    }
                }
            } else {
                Prompts::input("Sequence name")?
            }
        } else {
            Prompts::input("Sequence name")?
        };

        let description = if let Some(sample) = &self.sample {
            let prompts = Prompts::new_with_sample(sample);
            if let Some(desc) = sample.sequence_description() {
                prompts.input_optional_with_sample("Description", &desc)?
            } else {
                None
            }
        } else if Prompts::confirm("\nEdit description in editor?")? {
            let template = format!(
                "# Description for: {}\n# Enter the sequence description below:\n\n",
                sequence_name
            );
            let editor_config = EditorConfig::load();
            let edited = TestCaseEditor::edit_text(&template, &editor_config)?;

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
            Prompts::input_optional("Description")?
        };

        let add_initial_conditions = if let Some(sample) = &self.sample {
            let prompts = Prompts::new_with_sample(sample);
            prompts.confirm_with_sample(
                "\nAdd sequence-specific initial conditions?",
                sample.confirm_add_sequence_initial_conditions(),
            )?
        } else {
            Prompts::confirm("\nAdd sequence-specific initial conditions?")?
        };

        let database_path = if let Some(sample) = &self.sample {
            sample.database_path()
        } else {
            Prompts::input_with_default("Database path", "data")?
        };

        let db = ConditionDatabase::load_from_directory(&database_path)
            .context("Failed to load condition database")?;

        let prompts = if let Some(sample) = &self.sample {
            Prompts::new_with_database_and_sample(&db, sample)
        } else {
            Prompts::new_with_database(&db)
        };

        let initial_conditions = if add_initial_conditions {
            let use_db = if let Some(sample) = &self.sample {
                let sample_prompts = Prompts::new_with_sample(sample);
                sample_prompts.confirm_with_sample(
                    "Use database for initial conditions?",
                    sample.confirm_use_database(),
                )?
            } else {
                Prompts::confirm("Use database for initial conditions?")?
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
                            FuzzySearchResult::Selected(condition) => {
                                selected_conditions.push(condition.clone());
                                println!("✓ Added: {}", condition);

                                if !Prompts::confirm("Add another condition?")? {
                                    break;
                                }
                            }
                            FuzzySearchResult::Cancelled => break,
                            FuzzySearchResult::Error(e) => {
                                return Err(anyhow::anyhow!("Search error: {}", e));
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

                        Some(Value::Mapping(initial_cond_map))
                    } else {
                        None
                    }
                } else {
                    println!("No conditions in database, using manual entry.");
                    Some(prompts.prompt_initial_conditions(None, &self.validator)?)
                }
            } else {
                Some(prompts.prompt_initial_conditions(None, &self.validator)?)
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

        println!("\n=== Validating Test Sequence ===");
        self.validate_and_append_sequence(sequence_value)?;
        println!("✓ Test sequence validated and added\n");

        Ok(self)
    }

    /// Get existing sequence names from the structure
    fn get_existing_sequence_names(&self) -> Vec<String> {
        let mut names = Vec::new();

        if let Some(Value::Sequence(sequences)) = self.structure.get("test_sequences") {
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
    pub fn validate_and_append_sequence(&mut self, sequence: Value) -> Result<()> {
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

        let sequences = self
            .structure
            .entry("test_sequences".to_string())
            .or_insert_with(|| Value::Sequence(Vec::new()));

        if let Value::Sequence(seq_vec) = sequences {
            seq_vec.push(sequence);
        } else {
            anyhow::bail!("test_sequences must be a sequence");
        }

        Ok(())
    }

    /// Build test sequences interactively with git commits before each sequence
    pub fn build_test_sequences_with_commits(&mut self) -> Result<&mut Self> {
        println!("\n╔═══════════════════════════════════════════════╗");
        println!("║    Test Sequence Builder with Git Commits    ║");
        println!("╚═══════════════════════════════════════════════╝\n");

        loop {
            self.add_test_sequence_interactive()
                .context("Failed to add test sequence")?;

            if Prompts::confirm("Commit this sequence to git?")? {
                let sequence_id = self.get_next_sequence_id() - 1;
                let commit_msg = format!("Add test sequence #{}", sequence_id);
                self.commit(&commit_msg)
                    .context("Failed to commit test sequence")?;
            }

            if !Prompts::confirm("\nAdd another test sequence?")? {
                break;
            }
        }

        println!("\n✓ All test sequences added");
        Ok(self)
    }

    /// Add steps to a test sequence with git commit after each step
    pub fn add_steps_to_sequence_with_commits(
        &mut self,
        sequence_index: usize,
    ) -> Result<&mut Self> {
        println!("\n╔═══════════════════════════════════════════════╗");
        println!("║      Step Collection Loop with Commits       ║");
        println!("╚═══════════════════════════════════════════════╝\n");

        let sequence_id = self.get_sequence_id_by_index_internal(sequence_index)?;
        let sequence_name = self.get_sequence_name_by_index_internal(sequence_index)?;

        println!(
            "Adding steps to Sequence #{}: {}\n",
            sequence_id, sequence_name
        );

        let existing_steps = self.get_all_existing_steps_internal();

        loop {
            let step_number = self.get_next_step_number_internal(sequence_index)?;
            println!("\n=== Add Step #{} ===", step_number);

            let step_description = if !existing_steps.is_empty() {
                println!("\nYou can select from existing step descriptions or enter a new one.");

                if Prompts::confirm("Use fuzzy search to select from existing descriptions?")? {
                    match TestCaseFuzzyFinder::search_strings(
                        &existing_steps,
                        "Select step description: ",
                    )? {
                        FuzzySearchResult::Selected(desc) => desc,
                        FuzzySearchResult::Cancelled => {
                            println!("No selection made, entering new description.");
                            Prompts::input("Step description")?
                        }
                        FuzzySearchResult::Error(e) => {
                            return Err(anyhow::anyhow!("Search error: {}", e));
                        }
                    }
                } else {
                    Prompts::input("Step description")?
                }
            } else {
                Prompts::input("Step description")?
            };

            let manual = if Prompts::confirm("Is this a manual step?")? {
                Some(true)
            } else {
                None
            };

            let command = Prompts::input("Command")?;

            let expected = self.prompt_for_expected_internal()?;

            let step = self.create_step_value(
                step_number,
                manual,
                step_description.clone(),
                command,
                expected,
            )?;

            println!("\n=== Validating Step ===");
            self.validate_and_append_step(sequence_index, step)?;
            println!("✓ Step validated and added\n");

            self.save().context("Failed to save file")?;

            if Prompts::confirm("Commit this step to git?")? {
                let commit_msg = format!(
                    "Add step #{} to sequence #{}: {}",
                    step_number, sequence_id, step_description
                );
                self.commit(&commit_msg).context("Failed to commit step")?;
            }

            if !Prompts::confirm("\nAdd another step to this sequence?")? {
                break;
            }
        }

        println!("\n✓ All steps added to sequence");
        Ok(self)
    }

    /// Public accessor for get_sequence_id_by_index
    pub fn get_sequence_id_by_index(&self, index: usize) -> Result<i64> {
        self.get_sequence_id_by_index_internal(index)
    }

    /// Internal implementation of get_sequence_id_by_index
    fn get_sequence_id_by_index_internal(&self, index: usize) -> Result<i64> {
        if let Some(Value::Sequence(sequences)) = self.structure.get("test_sequences") {
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

    /// Public accessor for get_sequence_name_by_index
    pub fn get_sequence_name_by_index(&self, index: usize) -> Result<String> {
        self.get_sequence_name_by_index_internal(index)
    }

    /// Internal implementation of get_sequence_name_by_index
    fn get_sequence_name_by_index_internal(&self, index: usize) -> Result<String> {
        if let Some(Value::Sequence(sequences)) = self.structure.get("test_sequences") {
            if let Some(Value::Mapping(seq_map)) = sequences.get(index) {
                if let Some(Value::String(name)) = seq_map.get(Value::String("name".to_string())) {
                    return Ok(name.clone());
                }
            }
        }
        anyhow::bail!("Sequence not found at index {}", index)
    }

    /// Public accessor for get_all_existing_steps
    pub fn get_all_existing_steps(&self) -> Vec<String> {
        self.get_all_existing_steps_internal()
    }

    /// Internal implementation of get_all_existing_steps
    fn get_all_existing_steps_internal(&self) -> Vec<String> {
        let mut descriptions = Vec::new();

        if let Some(Value::Sequence(sequences)) = self.structure.get("test_sequences") {
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

    /// Public accessor for prompt_for_expected
    pub fn prompt_for_expected(&self) -> Result<Value> {
        self.prompt_for_expected_internal()
    }

    /// Internal implementation of prompt_for_expected
    fn prompt_for_expected_internal(&self) -> Result<Value> {
        let (include_success, result, output, success_value) = if let Some(sample) = &self.sample {
            let prompts = Prompts::new_with_sample(sample);
            let include = prompts.confirm_with_sample(
                "Include 'success' field?",
                sample.confirm_include_success_field(),
            )?;
            let res = prompts.input_with_sample("Expected result", &sample.expected_result())?;
            let out = prompts.input_with_sample("Expected output", &sample.expected_output())?;
            let success = if include {
                prompts.confirm_with_sample(
                    "Success value (true/false)?",
                    sample.expected_success_value(),
                )?
            } else {
                false
            };
            (include, res, out, success)
        } else {
            let include = Prompts::confirm("Include 'success' field?")?;
            let res = Prompts::input("Expected result")?;
            let out = Prompts::input("Expected output")?;
            let success = if include {
                Prompts::confirm("Success value (true/false)?")?
            } else {
                false
            };
            (include, res, out, success)
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

    /// Public accessor for get_next_step_number
    pub fn get_next_step_number(&self, sequence_index: usize) -> Result<i64> {
        self.get_next_step_number_internal(sequence_index)
    }

    /// Internal implementation of get_next_step_number
    fn get_next_step_number_internal(&self, sequence_index: usize) -> Result<i64> {
        if let Some(Value::Sequence(sequences)) = self.structure.get("test_sequences") {
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
    pub fn validate_and_append_step(&mut self, sequence_index: usize, step: Value) -> Result<()> {
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

        let sequences = self
            .structure
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

    /// Build test sequences with step collection loops and commits
    pub fn build_test_sequences_with_step_commits(&mut self) -> Result<&mut Self> {
        println!("\n╔═══════════════════════════════════════════════╗");
        println!("║  Test Sequence & Step Builder with Commits   ║");
        println!("╚═══════════════════════════════════════════════╝\n");

        loop {
            self.add_test_sequence_interactive()
                .context("Failed to add test sequence")?;

            let sequence_index = self.get_sequence_count() - 1;

            if Prompts::confirm("Commit this sequence to git?")? {
                let sequence_id = self.get_next_sequence_id() - 1;
                let commit_msg = format!("Add test sequence #{}", sequence_id);
                self.commit(&commit_msg)
                    .context("Failed to commit test sequence")?;
            }

            if Prompts::confirm("\nAdd steps to this sequence now?")? {
                self.add_steps_to_sequence_with_commits(sequence_index)
                    .context("Failed to add steps to sequence")?;
            }

            if !Prompts::confirm("\nAdd another test sequence?")? {
                break;
            }
        }

        println!("\n✓ All test sequences and steps added");
        Ok(self)
    }

    /// Get the count of sequences
    pub fn get_sequence_count(&self) -> usize {
        if let Some(Value::Sequence(sequences)) = self.structure.get("test_sequences") {
            sequences.len()
        } else {
            0
        }
    }

    /// Add steps to an existing sequence by sequence ID
    pub fn add_steps_to_sequence_by_id_with_commits(
        &mut self,
        sequence_id: i64,
    ) -> Result<&mut Self> {
        let sequence_index = self.find_sequence_index_by_id(sequence_id)?;
        self.add_steps_to_sequence_with_commits(sequence_index)
    }

    /// Find sequence index by ID
    fn find_sequence_index_by_id(&self, sequence_id: i64) -> Result<usize> {
        if let Some(Value::Sequence(sequences)) = self.structure.get("test_sequences") {
            for (idx, seq) in sequences.iter().enumerate() {
                if let Value::Mapping(seq_map) = seq {
                    if let Some(Value::Number(id)) = seq_map.get(Value::String("id".to_string())) {
                        if id.as_i64() == Some(sequence_id) {
                            return Ok(idx);
                        }
                    }
                }
            }
        }
        anyhow::bail!("Sequence with ID {} not found", sequence_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_builder_creation() {
        let temp_dir = TempDir::new().unwrap();
        let builder = TestCaseBuilder::new(temp_dir.path());
        assert!(builder.is_ok());
    }

    #[test]
    fn test_add_field() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();

        builder
            .add_field(
                "test_field".to_string(),
                Value::String("test_value".to_string()),
            )
            .unwrap();

        assert_eq!(
            builder.structure().get("test_field"),
            Some(&Value::String("test_value".to_string()))
        );
    }

    #[test]
    fn test_to_yaml_string() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();

        builder
            .add_field(
                "requirement".to_string(),
                Value::String("REQ001".to_string()),
            )
            .unwrap();

        builder
            .add_field("item".to_string(), Value::Number(1.into()))
            .unwrap();

        let yaml = builder.to_yaml_string().unwrap();
        assert!(yaml.contains("requirement"));
        assert!(yaml.contains("REQ001"));
        assert!(yaml.contains("item"));
    }

    #[test]
    fn test_save_file() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();

        builder
            .add_field("id".to_string(), Value::String("test_case_001".to_string()))
            .unwrap();

        builder
            .add_field(
                "requirement".to_string(),
                Value::String("REQ001".to_string()),
            )
            .unwrap();

        let file_path = builder.save().unwrap();
        assert!(file_path.exists());
        assert_eq!(file_path.file_name().unwrap(), "test_case_001.yaml");
    }

    #[test]
    fn test_complete_metadata() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();

        builder
            .add_field(
                "requirement".to_string(),
                Value::String("XXX100".to_string()),
            )
            .unwrap();
        builder
            .add_field("item".to_string(), Value::Number(1.into()))
            .unwrap();
        builder
            .add_field("tc".to_string(), Value::Number(4.into()))
            .unwrap();
        builder
            .add_field("id".to_string(), Value::String("test_001".to_string()))
            .unwrap();
        builder
            .add_field(
                "description".to_string(),
                Value::String("Test description".to_string()),
            )
            .unwrap();

        let yaml = builder.to_yaml_string().unwrap();
        assert!(yaml.contains("requirement: XXX100"));
        assert!(yaml.contains("item: 1"));
        assert!(yaml.contains("tc: 4"));
        assert!(yaml.contains("test_001"));
        assert!(yaml.contains("Test description"));
    }

    #[test]
    fn test_get_next_sequence_id_empty() {
        let temp_dir = TempDir::new().unwrap();
        let builder = TestCaseBuilder::new(temp_dir.path()).unwrap();

        assert_eq!(builder.get_next_sequence_id(), 1);
    }

    #[test]
    fn test_get_next_sequence_id_with_sequences() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();

        let mut seq1 = serde_yaml::Mapping::new();
        seq1.insert(Value::String("id".to_string()), Value::Number(1.into()));
        seq1.insert(
            Value::String("name".to_string()),
            Value::String("Seq 1".to_string()),
        );
        seq1.insert(
            Value::String("steps".to_string()),
            Value::Sequence(Vec::new()),
        );

        let mut seq2 = serde_yaml::Mapping::new();
        seq2.insert(Value::String("id".to_string()), Value::Number(3.into()));
        seq2.insert(
            Value::String("name".to_string()),
            Value::String("Seq 2".to_string()),
        );
        seq2.insert(
            Value::String("steps".to_string()),
            Value::Sequence(Vec::new()),
        );

        builder
            .validate_and_append_sequence(Value::Mapping(seq1))
            .unwrap();
        builder
            .validate_and_append_sequence(Value::Mapping(seq2))
            .unwrap();

        assert_eq!(builder.get_next_sequence_id(), 4);
    }

    #[test]
    fn test_validate_and_append_sequence() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();

        let mut seq_map = serde_yaml::Mapping::new();
        seq_map.insert(Value::String("id".to_string()), Value::Number(1.into()));
        seq_map.insert(
            Value::String("name".to_string()),
            Value::String("Test Sequence".to_string()),
        );
        seq_map.insert(
            Value::String("steps".to_string()),
            Value::Sequence(Vec::new()),
        );

        builder
            .validate_and_append_sequence(Value::Mapping(seq_map))
            .unwrap();

        let sequences = builder.structure().get("test_sequences").unwrap();
        if let Value::Sequence(seq_vec) = sequences {
            assert_eq!(seq_vec.len(), 1);
        } else {
            panic!("test_sequences is not a sequence");
        }
    }

    #[test]
    fn test_validate_and_append_sequence_missing_id() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();

        let mut seq_map = serde_yaml::Mapping::new();
        seq_map.insert(
            Value::String("name".to_string()),
            Value::String("Test Sequence".to_string()),
        );
        seq_map.insert(
            Value::String("steps".to_string()),
            Value::Sequence(Vec::new()),
        );

        let result = builder.validate_and_append_sequence(Value::Mapping(seq_map));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("must have an 'id'"));
    }

    #[test]
    fn test_validate_and_append_sequence_missing_name() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();

        let mut seq_map = serde_yaml::Mapping::new();
        seq_map.insert(Value::String("id".to_string()), Value::Number(1.into()));
        seq_map.insert(
            Value::String("steps".to_string()),
            Value::Sequence(Vec::new()),
        );

        let result = builder.validate_and_append_sequence(Value::Mapping(seq_map));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("must have a 'name'"));
    }

    #[test]
    fn test_validate_and_append_sequence_missing_steps() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();

        let mut seq_map = serde_yaml::Mapping::new();
        seq_map.insert(Value::String("id".to_string()), Value::Number(1.into()));
        seq_map.insert(
            Value::String("name".to_string()),
            Value::String("Test Sequence".to_string()),
        );

        let result = builder.validate_and_append_sequence(Value::Mapping(seq_map));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("must have a 'steps'"));
    }

    #[test]
    fn test_get_existing_sequence_names() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();

        let mut seq1 = serde_yaml::Mapping::new();
        seq1.insert(Value::String("id".to_string()), Value::Number(1.into()));
        seq1.insert(
            Value::String("name".to_string()),
            Value::String("Sequence One".to_string()),
        );
        seq1.insert(
            Value::String("steps".to_string()),
            Value::Sequence(Vec::new()),
        );

        let mut seq2 = serde_yaml::Mapping::new();
        seq2.insert(Value::String("id".to_string()), Value::Number(2.into()));
        seq2.insert(
            Value::String("name".to_string()),
            Value::String("Sequence Two".to_string()),
        );
        seq2.insert(
            Value::String("steps".to_string()),
            Value::Sequence(Vec::new()),
        );

        builder
            .validate_and_append_sequence(Value::Mapping(seq1))
            .unwrap();
        builder
            .validate_and_append_sequence(Value::Mapping(seq2))
            .unwrap();

        let names = builder.get_existing_sequence_names();
        assert_eq!(names.len(), 2);
        assert_eq!(names[0], "Sequence One");
        assert_eq!(names[1], "Sequence Two");
    }

    #[test]
    fn test_sequence_with_description() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();

        let mut seq_map = serde_yaml::Mapping::new();
        seq_map.insert(Value::String("id".to_string()), Value::Number(1.into()));
        seq_map.insert(
            Value::String("name".to_string()),
            Value::String("Test Sequence".to_string()),
        );
        seq_map.insert(
            Value::String("description".to_string()),
            Value::String("This is a test".to_string()),
        );
        seq_map.insert(
            Value::String("steps".to_string()),
            Value::Sequence(Vec::new()),
        );

        builder
            .validate_and_append_sequence(Value::Mapping(seq_map))
            .unwrap();

        let yaml = builder.to_yaml_string().unwrap();
        assert!(yaml.contains("Test Sequence"));
        assert!(yaml.contains("This is a test"));
    }

    #[test]
    fn test_get_sequence_id_by_index() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();

        let mut seq_map = serde_yaml::Mapping::new();
        seq_map.insert(Value::String("id".to_string()), Value::Number(5.into()));
        seq_map.insert(
            Value::String("name".to_string()),
            Value::String("Test Sequence".to_string()),
        );
        seq_map.insert(
            Value::String("steps".to_string()),
            Value::Sequence(Vec::new()),
        );

        builder
            .validate_and_append_sequence(Value::Mapping(seq_map))
            .unwrap();

        let id = builder.get_sequence_id_by_index(0).unwrap();
        assert_eq!(id, 5);
    }

    #[test]
    fn test_get_sequence_name_by_index() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();

        let mut seq_map = serde_yaml::Mapping::new();
        seq_map.insert(Value::String("id".to_string()), Value::Number(1.into()));
        seq_map.insert(
            Value::String("name".to_string()),
            Value::String("My Test Sequence".to_string()),
        );
        seq_map.insert(
            Value::String("steps".to_string()),
            Value::Sequence(Vec::new()),
        );

        builder
            .validate_and_append_sequence(Value::Mapping(seq_map))
            .unwrap();

        let name = builder.get_sequence_name_by_index(0).unwrap();
        assert_eq!(name, "My Test Sequence");
    }

    #[test]
    fn test_get_next_step_number_empty() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();

        let mut seq_map = serde_yaml::Mapping::new();
        seq_map.insert(Value::String("id".to_string()), Value::Number(1.into()));
        seq_map.insert(
            Value::String("name".to_string()),
            Value::String("Test Sequence".to_string()),
        );
        seq_map.insert(
            Value::String("steps".to_string()),
            Value::Sequence(Vec::new()),
        );

        builder
            .validate_and_append_sequence(Value::Mapping(seq_map))
            .unwrap();

        let next_step = builder.get_next_step_number(0).unwrap();
        assert_eq!(next_step, 1);
    }

    #[test]
    fn test_get_next_step_number_with_existing() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();

        let mut step1 = serde_yaml::Mapping::new();
        step1.insert(Value::String("step".to_string()), Value::Number(1.into()));
        step1.insert(
            Value::String("description".to_string()),
            Value::String("Step 1".to_string()),
        );
        step1.insert(
            Value::String("command".to_string()),
            Value::String("ssh".to_string()),
        );

        let mut expected = serde_yaml::Mapping::new();
        expected.insert(
            Value::String("result".to_string()),
            Value::String("success".to_string()),
        );
        expected.insert(
            Value::String("output".to_string()),
            Value::String("ok".to_string()),
        );
        step1.insert(
            Value::String("expected".to_string()),
            Value::Mapping(expected),
        );

        let steps_vec = vec![Value::Mapping(step1)];

        let mut seq_map = serde_yaml::Mapping::new();
        seq_map.insert(Value::String("id".to_string()), Value::Number(1.into()));
        seq_map.insert(
            Value::String("name".to_string()),
            Value::String("Test Sequence".to_string()),
        );
        seq_map.insert(
            Value::String("steps".to_string()),
            Value::Sequence(steps_vec),
        );

        builder
            .validate_and_append_sequence(Value::Mapping(seq_map))
            .unwrap();

        let next_step = builder.get_next_step_number(0).unwrap();
        assert_eq!(next_step, 2);
    }

    #[test]
    fn test_create_step_value() {
        let temp_dir = TempDir::new().unwrap();
        let builder = TestCaseBuilder::new(temp_dir.path()).unwrap();

        let mut expected = serde_yaml::Mapping::new();
        expected.insert(
            Value::String("result".to_string()),
            Value::String("0x9000".to_string()),
        );
        expected.insert(
            Value::String("output".to_string()),
            Value::String("Success".to_string()),
        );

        let step = builder
            .create_step_value(
                1,
                Some(true),
                "Test step".to_string(),
                "ssh".to_string(),
                Value::Mapping(expected),
            )
            .unwrap();

        if let Value::Mapping(step_map) = step {
            assert_eq!(
                step_map.get(Value::String("step".to_string())),
                Some(&Value::Number(1.into()))
            );
            assert_eq!(
                step_map.get(Value::String("manual".to_string())),
                Some(&Value::Bool(true))
            );
            assert_eq!(
                step_map.get(Value::String("description".to_string())),
                Some(&Value::String("Test step".to_string()))
            );
        } else {
            panic!("Step should be a mapping");
        }
    }

    #[test]
    fn test_validate_and_append_step() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();

        let mut seq_map = serde_yaml::Mapping::new();
        seq_map.insert(Value::String("id".to_string()), Value::Number(1.into()));
        seq_map.insert(
            Value::String("name".to_string()),
            Value::String("Test Sequence".to_string()),
        );
        seq_map.insert(
            Value::String("steps".to_string()),
            Value::Sequence(Vec::new()),
        );

        builder
            .validate_and_append_sequence(Value::Mapping(seq_map))
            .unwrap();

        let mut expected = serde_yaml::Mapping::new();
        expected.insert(
            Value::String("result".to_string()),
            Value::String("0x9000".to_string()),
        );
        expected.insert(
            Value::String("output".to_string()),
            Value::String("Success".to_string()),
        );

        let step = builder
            .create_step_value(
                1,
                None,
                "Test step".to_string(),
                "ssh".to_string(),
                Value::Mapping(expected),
            )
            .unwrap();

        builder.validate_and_append_step(0, step).unwrap();

        if let Some(Value::Sequence(sequences)) = builder.structure().get("test_sequences") {
            if let Some(Value::Mapping(seq_map)) = sequences.first() {
                if let Some(Value::Sequence(steps)) =
                    seq_map.get(Value::String("steps".to_string()))
                {
                    assert_eq!(steps.len(), 1);
                } else {
                    panic!("steps is not a sequence");
                }
            } else {
                panic!("sequence is not a mapping");
            }
        } else {
            panic!("test_sequences not found");
        }
    }

    #[test]
    fn test_validate_step_missing_fields() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();

        let mut seq_map = serde_yaml::Mapping::new();
        seq_map.insert(Value::String("id".to_string()), Value::Number(1.into()));
        seq_map.insert(
            Value::String("name".to_string()),
            Value::String("Test Sequence".to_string()),
        );
        seq_map.insert(
            Value::String("steps".to_string()),
            Value::Sequence(Vec::new()),
        );

        builder
            .validate_and_append_sequence(Value::Mapping(seq_map))
            .unwrap();

        let mut step_map = serde_yaml::Mapping::new();
        step_map.insert(Value::String("step".to_string()), Value::Number(1.into()));

        let result = builder.validate_and_append_step(0, Value::Mapping(step_map));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("must have a 'description'"));
    }

    #[test]
    fn test_get_all_existing_steps() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();

        let mut expected = serde_yaml::Mapping::new();
        expected.insert(
            Value::String("result".to_string()),
            Value::String("success".to_string()),
        );
        expected.insert(
            Value::String("output".to_string()),
            Value::String("ok".to_string()),
        );

        let mut step1 = serde_yaml::Mapping::new();
        step1.insert(Value::String("step".to_string()), Value::Number(1.into()));
        step1.insert(
            Value::String("description".to_string()),
            Value::String("First step".to_string()),
        );
        step1.insert(
            Value::String("command".to_string()),
            Value::String("ssh".to_string()),
        );
        step1.insert(
            Value::String("expected".to_string()),
            Value::Mapping(expected.clone()),
        );

        let mut step2 = serde_yaml::Mapping::new();
        step2.insert(Value::String("step".to_string()), Value::Number(2.into()));
        step2.insert(
            Value::String("description".to_string()),
            Value::String("Second step".to_string()),
        );
        step2.insert(
            Value::String("command".to_string()),
            Value::String("ssh".to_string()),
        );
        step2.insert(
            Value::String("expected".to_string()),
            Value::Mapping(expected.clone()),
        );

        let steps_vec = vec![Value::Mapping(step1), Value::Mapping(step2)];

        let mut seq_map = serde_yaml::Mapping::new();
        seq_map.insert(Value::String("id".to_string()), Value::Number(1.into()));
        seq_map.insert(
            Value::String("name".to_string()),
            Value::String("Test Sequence".to_string()),
        );
        seq_map.insert(
            Value::String("steps".to_string()),
            Value::Sequence(steps_vec),
        );

        builder
            .validate_and_append_sequence(Value::Mapping(seq_map))
            .unwrap();

        let existing_steps = builder.get_all_existing_steps();
        assert_eq!(existing_steps.len(), 2);
        assert!(existing_steps.contains(&"First step".to_string()));
        assert!(existing_steps.contains(&"Second step".to_string()));
    }

    #[test]
    fn test_find_sequence_index_by_id() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();

        let mut seq1 = serde_yaml::Mapping::new();
        seq1.insert(Value::String("id".to_string()), Value::Number(10.into()));
        seq1.insert(
            Value::String("name".to_string()),
            Value::String("Sequence 1".to_string()),
        );
        seq1.insert(
            Value::String("steps".to_string()),
            Value::Sequence(Vec::new()),
        );

        let mut seq2 = serde_yaml::Mapping::new();
        seq2.insert(Value::String("id".to_string()), Value::Number(20.into()));
        seq2.insert(
            Value::String("name".to_string()),
            Value::String("Sequence 2".to_string()),
        );
        seq2.insert(
            Value::String("steps".to_string()),
            Value::Sequence(Vec::new()),
        );

        builder
            .validate_and_append_sequence(Value::Mapping(seq1))
            .unwrap();
        builder
            .validate_and_append_sequence(Value::Mapping(seq2))
            .unwrap();

        let index = builder.find_sequence_index_by_id(20).unwrap();
        assert_eq!(index, 1);
    }

    #[test]
    fn test_get_sequence_count() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();

        assert_eq!(builder.get_sequence_count(), 0);

        let mut seq_map = serde_yaml::Mapping::new();
        seq_map.insert(Value::String("id".to_string()), Value::Number(1.into()));
        seq_map.insert(
            Value::String("name".to_string()),
            Value::String("Test Sequence".to_string()),
        );
        seq_map.insert(
            Value::String("steps".to_string()),
            Value::Sequence(Vec::new()),
        );

        builder
            .validate_and_append_sequence(Value::Mapping(seq_map))
            .unwrap();

        assert_eq!(builder.get_sequence_count(), 1);
    }
}
