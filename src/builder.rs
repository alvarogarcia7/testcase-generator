use crate::config::EditorConfig;
use crate::creator::TestCaseCreator;
use crate::database::ConditionDatabase;
use crate::git::GitManager;
use crate::models::Step;
use crate::oracle::Oracle;
use crate::prompts::Prompts;
use crate::recovery::{RecoveryManager, RecoveryState};
use crate::sample::SampleData;
use crate::ui::{print_title, TitleStyle};
use crate::validation::SchemaValidator;
use anyhow::{Context, Result};
use indexmap::IndexMap;
use serde_yaml::Value;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Builder for creating test cases interactively
/// This struct now delegates to TestCaseCreator for all test case creation logic
pub struct TestCaseBuilder {
    base_path: PathBuf,
    git_manager: Option<GitManager>,
    structure: IndexMap<String, Value>,
    recovery_manager: RecoveryManager,
    sample: Option<SampleData>,
    creator: TestCaseCreator,
}

impl TestCaseBuilder {
    /// Create a new test case builder
    pub fn new<P: AsRef<Path>>(base_path: P, oracle: Arc<dyn Oracle>) -> Result<Self> {
        let base_path = base_path.as_ref().to_path_buf();

        let git_manager = GitManager::open(&base_path)
            .or_else(|_| GitManager::init(&base_path))
            .ok();

        let recovery_manager = RecoveryManager::new(&base_path);

        // Try to load database from base_path
        let db = ConditionDatabase::load_from_directory(&base_path).ok();

        let editor_config = EditorConfig::load();
        let creator = TestCaseCreator::new(&base_path, oracle, editor_config, db)?;

        Ok(Self {
            base_path,
            git_manager,
            structure: IndexMap::new(),
            recovery_manager,
            sample: None,
            creator,
        })
    }

    /// Create a new test case builder and check for recovery
    pub fn new_with_recovery<P: AsRef<Path>>(
        base_path: P,
        oracle: Arc<dyn Oracle>,
    ) -> Result<Self> {
        let base_path = base_path.as_ref().to_path_buf();

        let git_manager = GitManager::open(&base_path)
            .or_else(|_| GitManager::init(&base_path))
            .ok();

        let recovery_manager = RecoveryManager::new(&base_path);

        let structure = if recovery_manager.prompt_for_recovery()? {
            if let Some(state) = recovery_manager.load_state()? {
                log::info!("✓ Resuming from saved state\n");
                state.structure
            } else {
                IndexMap::new()
            }
        } else {
            IndexMap::new()
        };

        // Try to load database from base_path
        let db = ConditionDatabase::load_from_directory(&base_path).ok();

        let editor_config = EditorConfig::load();
        let creator = TestCaseCreator::new(&base_path, oracle, editor_config, db)?;

        Ok(Self {
            base_path,
            git_manager,
            structure,
            recovery_manager,
            sample: None,
            creator,
        })
    }

    /// Set sample data for the builder
    pub fn with_sample(mut self, sample: SampleData) -> Self {
        self.sample = Some(sample);
        self
    }

    /// Set oracle for the builder
    pub fn with_oracle(self, oracle: Arc<dyn Oracle>) -> Self {
        // Recreate the creator with the new oracle
        let db = self.creator.database().cloned();
        let editor_config = self.creator.editor_config().clone();
        let creator = TestCaseCreator::new(&self.base_path, oracle, editor_config, db)
            .expect("Failed to recreate creator with new oracle");

        Self { creator, ..self }
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
        self.creator.validator()
    }

    /// Prompt for and add metadata to the structure
    pub fn add_metadata(&mut self) -> Result<&mut Self> {
        self.creator.add_metadata(&mut self.structure)?;
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

            log::info!("✓ Committed: {}", message);
        } else {
            log::warn!("⚠ Git repository not available, skipping commit");
        }

        Ok(())
    }

    /// Add general initial conditions with interactive prompts
    pub fn add_general_initial_conditions(
        &mut self,
        defaults: Option<&Value>,
    ) -> Result<&mut Self> {
        self.creator
            .add_general_initial_conditions(&mut self.structure, defaults)?;
        Ok(self)
    }

    /// Add general initial conditions with fuzzy search from database
    pub fn add_general_initial_conditions_with_search(
        &mut self,
        defaults: Option<&Value>,
        storage: &crate::storage::TestCaseStorage,
    ) -> Result<&mut Self> {
        self.creator.add_general_initial_conditions_with_search(
            &mut self.structure,
            defaults,
            storage,
        )?;
        Ok(self)
    }

    /// Add initial conditions with interactive prompts
    pub fn add_initial_conditions(&mut self, defaults: Option<&Value>) -> Result<&mut Self> {
        self.creator
            .add_initial_conditions(&mut self.structure, defaults)?;
        Ok(self)
    }

    /// Add general initial conditions from database with fuzzy search
    pub fn add_general_initial_conditions_from_database<P: AsRef<Path>>(
        &mut self,
        database_path: P,
    ) -> Result<&mut Self> {
        let db = ConditionDatabase::load_from_directory(database_path)
            .context("Failed to load condition database")?;
        self.creator
            .add_general_initial_conditions_from_database(&mut self.structure, &db)?;
        Ok(self)
    }

    /// Add initial conditions from database with fuzzy search
    pub fn add_initial_conditions_from_database<P: AsRef<Path>>(
        &mut self,
        database_path: P,
    ) -> Result<&mut Self> {
        let db = ConditionDatabase::load_from_directory(database_path)
            .context("Failed to load condition database")?;
        self.creator
            .add_initial_conditions_from_database(&mut self.structure, &db)?;
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
        self.creator
            .validator()
            .validate_chunk(&yaml_content)
            .context("Structure validation failed")
    }

    /// Convert the structure to a YAML string
    pub fn to_yaml_string(&self) -> Result<String> {
        TestCaseCreator::structure_to_yaml_string(&self.structure)
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
        self.creator.get_next_sequence_id(&self.structure)
    }

    /// Add a test sequence with interactive prompts
    pub fn add_test_sequence_interactive(&mut self) -> Result<&mut Self> {
        self.creator
            .add_test_sequence_interactive(&mut self.structure, self.sample.as_ref())?;
        Ok(self)
    }

    /// Validate a test sequence structure and append it to test_sequences
    pub fn validate_and_append_sequence(&mut self, sequence: Value) -> Result<()> {
        TestCaseCreator::validate_and_append_sequence(&mut self.structure, sequence)
    }

    /// Build test sequences interactively with git commits before each sequence
    pub fn build_test_sequences_with_commits(&mut self) -> Result<&mut Self> {
        print_title("Test Sequence Builder with Git Commits", TitleStyle::Box);

        loop {
            self.add_test_sequence_interactive()
                .context("Failed to add test sequence")?;

            if Prompts::confirm_with_oracle("Commit this sequence to git?", self.creator.oracle())?
            {
                let sequence_id = self.get_next_sequence_id() - 1;
                let commit_msg = format!("Add test sequence #{}", sequence_id);
                self.commit(&commit_msg)
                    .context("Failed to commit test sequence")?;
            }

            if !Prompts::confirm_with_oracle("\nAdd another test sequence?", self.creator.oracle())?
            {
                break;
            }
        }

        log::info!("\n✓ All test sequences added");
        Ok(self)
    }

    /// Add steps to a test sequence with git commit after each step
    pub fn add_steps_to_sequence_with_commits(
        &mut self,
        sequence_index: usize,
    ) -> Result<&mut Self> {
        print_title("Step Collection Loop with Commits", TitleStyle::Box);

        let sequence_id = self
            .creator
            .get_sequence_id_by_index(&self.structure, sequence_index)?;
        let sequence_name = self
            .creator
            .get_sequence_name_by_index(&self.structure, sequence_index)?;

        log::info!(
            "Adding steps to Sequence #{}: {}\n",
            sequence_id,
            sequence_name
        );

        loop {
            let step_number = self
                .creator
                .get_next_step_number(&self.structure, sequence_index)?;
            log::info!("\n=== Add Step #{} ===", step_number);

            let step = if let Some(db) = self.creator.database() {
                let step_items = db.get_all_steps();

                if !step_items.is_empty() {
                    let template = format!(
                        r#"step: {}
description: ""
command: ""
expected:
  result: ""
  output: ""
"#,
                        step_number
                    );

                    match crate::complex_structure_editor::ComplexStructureEditor::<Step>::edit_with_fuzzy_search(
                        &step_items,
                        "Select step (ESC to create new): ",
                        self.creator.oracle().as_ref(),
                        self.creator.editor_config(),
                        self.creator.validator(),
                        &template,
                    ) {
                        Ok(mut edited_step) => {
                            edited_step.step = step_number;
                            edited_step
                        }
                        Err(e) => {
                            log::warn!("Fuzzy search failed or cancelled: {}", e);
                            log::info!("Falling back to field-by-field prompts");
                            self.creator.prompt_for_step_fields(step_number)?
                        }
                    }
                } else {
                    self.creator.prompt_for_step_fields(step_number)?
                }
            } else {
                self.creator.prompt_for_step_fields(step_number)?
            };

            let step_value =
                serde_yaml::to_value(&step).context("Failed to convert Step to YAML value")?;

            log::info!("\n=== Validating Step ===");
            self.creator.validate_and_append_step(
                &mut self.structure,
                sequence_index,
                step_value,
            )?;
            log::info!("✓ Step validated and added\n");

            self.save().context("Failed to save file")?;

            if Prompts::confirm_with_oracle("Commit this step to git?", self.creator.oracle())? {
                let commit_msg = format!(
                    "Add step #{} to sequence #{}: {}",
                    step_number, sequence_id, step.description
                );
                self.commit(&commit_msg).context("Failed to commit step")?;
            }

            if !Prompts::confirm_with_oracle(
                "\nAdd another step to this sequence?",
                self.creator.oracle(),
            )? {
                break;
            }
        }

        log::info!("\n✓ All steps added to sequence");
        Ok(self)
    }

    /// Public accessor for get_sequence_id_by_index
    pub fn get_sequence_id_by_index(&self, index: usize) -> Result<i64> {
        self.creator
            .get_sequence_id_by_index(&self.structure, index)
    }

    /// Public accessor for get_sequence_name_by_index
    pub fn get_sequence_name_by_index(&self, index: usize) -> Result<String> {
        self.creator
            .get_sequence_name_by_index(&self.structure, index)
    }

    /// Public accessor for get_all_existing_steps
    pub fn get_all_existing_steps(&self) -> Vec<String> {
        self.creator.get_all_existing_steps(&self.structure)
    }

    /// Public accessor for prompt_for_expected
    pub fn prompt_for_expected(&self) -> Result<Value> {
        self.creator.prompt_for_expected()
    }

    /// Public accessor for get_next_step_number
    pub fn get_next_step_number(&self, sequence_index: usize) -> Result<i64> {
        self.creator
            .get_next_step_number(&self.structure, sequence_index)
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
        self.creator
            .create_step_value(step_number, manual, description, command, expected)
    }

    /// Validate a step structure and append it to the sequence
    pub fn validate_and_append_step(&mut self, sequence_index: usize, step: Value) -> Result<()> {
        self.creator
            .validate_and_append_step(&mut self.structure, sequence_index, step)
    }

    /// Build test sequences with step collection loops and commits
    pub fn build_test_sequences_with_step_commits(&mut self) -> Result<&mut Self> {
        print_title("Test Sequence & Step Builder with Commits", TitleStyle::Box);

        loop {
            self.add_test_sequence_interactive()
                .context("Failed to add test sequence")?;

            let sequence_index = self.get_sequence_count() - 1;

            if Prompts::confirm_with_oracle("Commit this sequence to git?", self.creator.oracle())?
            {
                let sequence_id = self.get_next_sequence_id() - 1;
                let commit_msg = format!("Add test sequence #{}", sequence_id);
                self.commit(&commit_msg)
                    .context("Failed to commit test sequence")?;
            }

            if Prompts::confirm_with_oracle(
                "\nAdd steps to this sequence now?",
                self.creator.oracle(),
            )? {
                self.add_steps_to_sequence_with_commits(sequence_index)
                    .context("Failed to add steps to sequence")?;
            }

            if !Prompts::confirm_with_oracle("\nAdd another test sequence?", self.creator.oracle())?
            {
                break;
            }
        }

        log::info!("\n✓ All test sequences and steps added");
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
    use crate::oracle::TtyCliOracle;
    use tempfile::TempDir;

    #[test]
    fn test_builder_creation() {
        let temp_dir = TempDir::new().unwrap();
        let builder = TestCaseBuilder::new(temp_dir.path(), Arc::new(TtyCliOracle::new()));
        assert!(builder.is_ok());
    }

    #[test]
    fn test_add_field() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder =
            TestCaseBuilder::new(temp_dir.path(), Arc::new(TtyCliOracle::new())).unwrap();

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
        let mut builder =
            TestCaseBuilder::new(temp_dir.path(), Arc::new(TtyCliOracle::new())).unwrap();

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
        let mut builder =
            TestCaseBuilder::new(temp_dir.path(), Arc::new(TtyCliOracle::new())).unwrap();

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
        let mut builder =
            TestCaseBuilder::new(temp_dir.path(), Arc::new(TtyCliOracle::new())).unwrap();

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
        let builder = TestCaseBuilder::new(temp_dir.path(), Arc::new(TtyCliOracle::new())).unwrap();

        assert_eq!(builder.get_next_sequence_id(), 1);
    }

    #[test]
    fn test_get_next_sequence_id_with_sequences() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder =
            TestCaseBuilder::new(temp_dir.path(), Arc::new(TtyCliOracle::new())).unwrap();

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
        let mut builder =
            TestCaseBuilder::new(temp_dir.path(), Arc::new(TtyCliOracle::new())).unwrap();

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
        let mut builder =
            TestCaseBuilder::new(temp_dir.path(), Arc::new(TtyCliOracle::new())).unwrap();

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
        let mut builder =
            TestCaseBuilder::new(temp_dir.path(), Arc::new(TtyCliOracle::new())).unwrap();

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
        let mut builder =
            TestCaseBuilder::new(temp_dir.path(), Arc::new(TtyCliOracle::new())).unwrap();

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
        let mut builder =
            TestCaseBuilder::new(temp_dir.path(), Arc::new(TtyCliOracle::new())).unwrap();

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

        let names = TestCaseCreator::get_existing_sequence_names(builder.structure());
        assert_eq!(names.len(), 2);
        assert_eq!(names[0], "Sequence One");
        assert_eq!(names[1], "Sequence Two");
    }

    #[test]
    fn test_sequence_with_description() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder =
            TestCaseBuilder::new(temp_dir.path(), Arc::new(TtyCliOracle::new())).unwrap();

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
        let mut builder =
            TestCaseBuilder::new(temp_dir.path(), Arc::new(TtyCliOracle::new())).unwrap();

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
        let mut builder =
            TestCaseBuilder::new(temp_dir.path(), Arc::new(TtyCliOracle::new())).unwrap();

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
        let mut builder =
            TestCaseBuilder::new(temp_dir.path(), Arc::new(TtyCliOracle::new())).unwrap();

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
        let mut builder =
            TestCaseBuilder::new(temp_dir.path(), Arc::new(TtyCliOracle::new())).unwrap();

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
        let builder = TestCaseBuilder::new(temp_dir.path(), Arc::new(TtyCliOracle::new())).unwrap();

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
        let mut builder =
            TestCaseBuilder::new(temp_dir.path(), Arc::new(TtyCliOracle::new())).unwrap();

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
        let mut builder =
            TestCaseBuilder::new(temp_dir.path(), Arc::new(TtyCliOracle::new())).unwrap();

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
        let mut builder =
            TestCaseBuilder::new(temp_dir.path(), Arc::new(TtyCliOracle::new())).unwrap();

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
        let mut builder =
            TestCaseBuilder::new(temp_dir.path(), Arc::new(TtyCliOracle::new())).unwrap();

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

        let idx = builder.find_sequence_index_by_id(20).unwrap();
        assert_eq!(idx, 1);
    }
}
