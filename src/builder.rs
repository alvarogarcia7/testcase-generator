use crate::editor::TestCaseEditor;
use crate::git::GitManager;
use crate::prompts::Prompts;
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
}

impl TestCaseBuilder {
    /// Create a new test case builder
    pub fn new<P: AsRef<Path>>(base_path: P) -> Result<Self> {
        let base_path = base_path.as_ref().to_path_buf();
        let validator = SchemaValidator::new()
            .context("Failed to create schema validator")?;
        
        let git_manager = GitManager::open(&base_path)
            .or_else(|_| GitManager::init(&base_path))
            .ok();

        Ok(Self {
            base_path,
            validator,
            git_manager,
            structure: IndexMap::new(),
        })
    }

    /// Prompt for and add metadata to the structure
    pub fn add_metadata(&mut self) -> Result<&mut Self> {
        let metadata = Prompts::prompt_metadata()
            .context("Failed to prompt for metadata")?;
        
        println!("\n=== Validating Metadata ===");
        metadata.validate(&self.validator)
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

            std::fs::write(&file_path, yaml_content)
                .context("Failed to write YAML file")?;

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
        let conditions = Prompts::prompt_general_initial_conditions(defaults, &self.validator)
            .context("Failed to prompt for general initial conditions")?;

        self.structure.insert("general_initial_conditions".to_string(), conditions);

        Ok(self)
    }

    /// Add initial conditions with interactive prompts
    pub fn add_initial_conditions(
        &mut self,
        defaults: Option<&Value>,
    ) -> Result<&mut Self> {
        println!("\n=== Initial Conditions ===\n");

        if let Some(default_value) = defaults {
            let yaml_str = serde_yaml::to_string(default_value)
                .context("Failed to serialize defaults")?;
            
            println!("Current defaults:");
            println!("{}", yaml_str);
            println!();

            let keep_defaults = Prompts::confirm_with_default("Keep these defaults?", true)?;
            
            if keep_defaults {
                self.structure.insert("initial_conditions".to_string(), default_value.clone());
                return Ok(self);
            }
        }

        loop {
            let template = r#"# Initial Conditions
# Example:
# eUICC:
#   - "Condition 1"
#   - "Condition 2"

eUICC:
  - ""
"#;

            let edited_content = TestCaseEditor::edit_text(template)
                .context("Failed to open editor")?;

            let parsed: Value = serde_yaml::from_str(&edited_content)
                .context("Failed to parse YAML")?;

            let yaml_for_validation = serde_yaml::to_string(&parsed)
                .context("Failed to serialize for validation")?;

            match self.validator.validate_partial_chunk(&yaml_for_validation) {
                Ok(_) => {
                    println!("✓ Valid structure");
                    self.structure.insert("initial_conditions".to_string(), parsed);
                    return Ok(self);
                }
                Err(e) => {
                    println!("✗ Validation failed: {}", e);
                    let retry = Prompts::confirm("Try again?")?;
                    if !retry {
                        anyhow::bail!("Validation failed, user cancelled");
                    }
                }
            }
        }
    }

    /// Add a custom field with validation
    pub fn add_field(&mut self, key: String, value: Value) -> Result<&mut Self> {
        self.structure.insert(key, value);
        Ok(self)
    }

    /// Validate the entire structure
    pub fn validate(&self) -> Result<()> {
        let yaml_content = self.to_yaml_string()?;
        self.validator.validate_chunk(&yaml_content)
            .context("Structure validation failed")
    }

    /// Convert the structure to a YAML string
    pub fn to_yaml_string(&self) -> Result<String> {
        let yaml_value = Value::Mapping(serde_yaml::Mapping::from_iter(
            self.structure.iter().map(|(k, v)| (Value::String(k.clone()), v.clone()))
        ));
        
        serde_yaml::to_string(&yaml_value)
            .context("Failed to serialize structure to YAML")
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

        std::fs::write(&file_path, yaml_content)
            .context("Failed to write YAML file")?;

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
        
        builder.add_field(
            "test_field".to_string(),
            Value::String("test_value".to_string())
        ).unwrap();

        assert_eq!(
            builder.structure().get("test_field"),
            Some(&Value::String("test_value".to_string()))
        );
    }

    #[test]
    fn test_to_yaml_string() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();
        
        builder.add_field(
            "requirement".to_string(),
            Value::String("REQ001".to_string())
        ).unwrap();

        builder.add_field(
            "item".to_string(),
            Value::Number(1.into())
        ).unwrap();

        let yaml = builder.to_yaml_string().unwrap();
        assert!(yaml.contains("requirement"));
        assert!(yaml.contains("REQ001"));
        assert!(yaml.contains("item"));
    }

    #[test]
    fn test_save_file() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();
        
        builder.add_field(
            "id".to_string(),
            Value::String("test_case_001".to_string())
        ).unwrap();

        builder.add_field(
            "requirement".to_string(),
            Value::String("REQ001".to_string())
        ).unwrap();

        let file_path = builder.save().unwrap();
        assert!(file_path.exists());
        assert_eq!(file_path.file_name().unwrap(), "test_case_001.yaml");
    }

    #[test]
    fn test_complete_metadata() {
        let temp_dir = TempDir::new().unwrap();
        let mut builder = TestCaseBuilder::new(temp_dir.path()).unwrap();
        
        builder.add_field("requirement".to_string(), Value::String("XXX100".to_string())).unwrap();
        builder.add_field("item".to_string(), Value::Number(1.into())).unwrap();
        builder.add_field("tc".to_string(), Value::Number(4.into())).unwrap();
        builder.add_field("id".to_string(), Value::String("test_001".to_string())).unwrap();
        builder.add_field("description".to_string(), Value::String("Test description".to_string())).unwrap();

        let yaml = builder.to_yaml_string().unwrap();
        assert!(yaml.contains("requirement: XXX100"));
        assert!(yaml.contains("item: 1"));
        assert!(yaml.contains("tc: 4"));
        assert!(yaml.contains("test_001"));
        assert!(yaml.contains("Test description"));
    }
}
