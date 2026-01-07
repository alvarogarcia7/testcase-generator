use anyhow::{Context, Result};
use chrono::{DateTime, Local, Utc};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field_path: String,
    pub error_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryState {
    pub structure: IndexMap<String, Value>,
    pub validation_errors: Vec<ValidationError>,
    pub current_phase: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl RecoveryState {
    pub fn new(structure: IndexMap<String, Value>, current_phase: String) -> Self {
        Self {
            structure,
            validation_errors: Vec::new(),
            current_phase,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn with_errors(
        structure: IndexMap<String, Value>,
        validation_errors: Vec<ValidationError>,
        current_phase: String,
    ) -> Self {
        Self {
            structure,
            validation_errors,
            current_phase,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn add_error(&mut self, field_path: String, error_message: String) {
        self.validation_errors.push(ValidationError {
            field_path,
            error_message,
        });
    }

    pub fn clear_errors(&mut self) {
        self.validation_errors.clear();
    }

    pub fn has_errors(&self) -> bool {
        !self.validation_errors.is_empty()
    }

    pub fn get_errors_for_field(&self, field_path: &str) -> Vec<&ValidationError> {
        self.validation_errors
            .iter()
            .filter(|e| e.field_path == field_path)
            .collect()
    }
}

pub struct RecoveryManager {
    recovery_file_path: PathBuf,
}

impl RecoveryManager {
    const RECOVERY_FILE_NAME: &'static str = ".recovery.json";

    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        let recovery_file_path = base_path.as_ref().join(Self::RECOVERY_FILE_NAME);
        Self { recovery_file_path }
    }

    fn format_timestamp_with_relative(timestamp: &DateTime<Utc>) -> String {
        let local_time: DateTime<Local> = timestamp.with_timezone(&Local);
        let now = Utc::now();
        let duration = now.signed_duration_since(*timestamp);

        let relative = if duration.num_seconds() < 60 {
            format!("{} s ago", duration.num_seconds())
        } else if duration.num_minutes() < 60 {
            format!("{} min ago", duration.num_minutes())
        } else if duration.num_hours() < 24 {
            format!("{} h ago", duration.num_hours())
        } else {
            format!("{} d ago", duration.num_days())
        };

        let absolute = local_time.format("%Y-%m-%d %H:%M:%S");

        format!("{} ({})", relative, absolute)
    }

    pub fn save_state(&self, state: &RecoveryState) -> Result<()> {
        let json_content =
            serde_json::to_string_pretty(state).context("Failed to serialize recovery state")?;

        std::fs::write(&self.recovery_file_path, json_content)
            .context("Failed to write recovery file")?;

        Ok(())
    }

    pub fn load_state(&self) -> Result<Option<RecoveryState>> {
        if !self.recovery_file_path.exists() {
            return Ok(None);
        }

        let json_content = std::fs::read_to_string(&self.recovery_file_path)
            .context("Failed to read recovery file")?;

        let state: RecoveryState =
            serde_json::from_str(&json_content).context("Failed to deserialize recovery state")?;

        Ok(Some(state))
    }

    pub fn delete_recovery_file(&self) -> Result<()> {
        if self.recovery_file_path.exists() {
            std::fs::remove_file(&self.recovery_file_path)
                .context("Failed to delete recovery file")?;
        }
        Ok(())
    }

    pub fn recovery_file_exists(&self) -> bool {
        self.recovery_file_path.exists()
    }

    pub fn prompt_for_recovery(&self) -> Result<bool> {
        use crate::Prompts;

        if !self.recovery_file_exists() {
            return Ok(false);
        }

        if let Some(state) = self.load_state()? {
            println!("\n╔══════════════════════════════════════════════════════╗");
            println!("║           Recovery File Detected                    ║");
            println!("╚══════════════════════════════════════════════════════╝\n");
            println!(
                "Found recovery data from: {}",
                Self::format_timestamp_with_relative(&state.timestamp)
            );
            println!("Phase: {}", state.current_phase);

            if state.has_errors() {
                println!(
                    "\nValidation Errors Found ({}):",
                    state.validation_errors.len()
                );
                for error in &state.validation_errors {
                    println!("  • {}: {}", error.field_path, error.error_message);
                }
            }

            println!();
            let resume = Prompts::confirm("Resume from saved state?")?;

            if !resume {
                let delete = Prompts::confirm("Delete recovery file?")?;
                if delete {
                    self.delete_recovery_file()?;
                    println!("✓ Recovery file deleted");
                }
            }

            return Ok(resume);
        }

        Ok(false)
    }

    pub fn display_field_errors(&self, state: &RecoveryState, field_path: &str) {
        let errors = state.get_errors_for_field(field_path);
        if !errors.is_empty() {
            println!("\n⚠ Previous validation errors for '{}':", field_path);
            for error in errors {
                println!("  • {}", error.error_message);
            }
        }
    }

    pub fn parse_validation_error_path(error_message: &str) -> Vec<String> {
        let mut paths = Vec::new();

        if let Some(path_start) = error_message.find("Path '") {
            let path_part = &error_message[path_start + 6..];
            if let Some(path_end) = path_part.find('\'') {
                let path = &path_part[..path_end];
                if path != "root" {
                    let components: Vec<String> = path.split('/').map(|s| s.to_string()).collect();
                    paths.extend(components);
                }
            }
        }

        paths
    }

    pub fn extract_validation_errors_from_anyhow(error: &anyhow::Error) -> Vec<ValidationError> {
        let mut validation_errors = Vec::new();
        let error_message = error.to_string();

        for line in error_message.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("- Path '") || trimmed.starts_with("Path '") {
                let path_components = Self::parse_validation_error_path(trimmed);
                let field_path = if path_components.is_empty() {
                    "root".to_string()
                } else {
                    path_components.join(".")
                };

                validation_errors.push(ValidationError {
                    field_path,
                    error_message: trimmed.to_string(),
                });
            }
        }

        if validation_errors.is_empty() && !error_message.is_empty() {
            validation_errors.push(ValidationError {
                field_path: "unknown".to_string(),
                error_message: error_message.clone(),
            });
        }

        validation_errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_recovery_state_creation() {
        let mut structure = IndexMap::new();
        structure.insert("test".to_string(), Value::String("value".to_string()));

        let state = RecoveryState::new(structure.clone(), "metadata".to_string());

        assert_eq!(state.structure.len(), 1);
        assert_eq!(state.current_phase, "metadata");
        assert!(!state.has_errors());
    }

    #[test]
    fn test_recovery_state_with_errors() {
        let mut structure = IndexMap::new();
        structure.insert("test".to_string(), Value::String("value".to_string()));

        let errors = vec![ValidationError {
            field_path: "test".to_string(),
            error_message: "Test error".to_string(),
        }];

        let state = RecoveryState::with_errors(structure, errors, "metadata".to_string());

        assert!(state.has_errors());
        assert_eq!(state.validation_errors.len(), 1);
    }

    #[test]
    fn test_add_and_clear_errors() {
        let mut structure = IndexMap::new();
        structure.insert("test".to_string(), Value::String("value".to_string()));

        let mut state = RecoveryState::new(structure, "metadata".to_string());

        assert!(!state.has_errors());

        state.add_error("field1".to_string(), "Error 1".to_string());
        state.add_error("field2".to_string(), "Error 2".to_string());

        assert!(state.has_errors());
        assert_eq!(state.validation_errors.len(), 2);

        state.clear_errors();
        assert!(!state.has_errors());
    }

    #[test]
    fn test_get_errors_for_field() {
        let mut structure = IndexMap::new();
        structure.insert("test".to_string(), Value::String("value".to_string()));

        let mut state = RecoveryState::new(structure, "metadata".to_string());

        state.add_error("field1".to_string(), "Error 1A".to_string());
        state.add_error("field1".to_string(), "Error 1B".to_string());
        state.add_error("field2".to_string(), "Error 2".to_string());

        let field1_errors = state.get_errors_for_field("field1");
        assert_eq!(field1_errors.len(), 2);

        let field2_errors = state.get_errors_for_field("field2");
        assert_eq!(field2_errors.len(), 1);

        let field3_errors = state.get_errors_for_field("field3");
        assert_eq!(field3_errors.len(), 0);
    }

    #[test]
    fn test_recovery_manager_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let manager = RecoveryManager::new(temp_dir.path());

        let mut structure = IndexMap::new();
        structure.insert("test".to_string(), Value::String("value".to_string()));

        let state = RecoveryState::new(structure, "metadata".to_string());

        manager.save_state(&state).unwrap();
        assert!(manager.recovery_file_exists());

        let loaded_state = manager.load_state().unwrap();
        assert!(loaded_state.is_some());

        let loaded = loaded_state.unwrap();
        assert_eq!(loaded.current_phase, "metadata");
        assert_eq!(loaded.structure.len(), 1);
    }

    #[test]
    fn test_recovery_manager_delete() {
        let temp_dir = TempDir::new().unwrap();
        let manager = RecoveryManager::new(temp_dir.path());

        let mut structure = IndexMap::new();
        structure.insert("test".to_string(), Value::String("value".to_string()));

        let state = RecoveryState::new(structure, "metadata".to_string());

        manager.save_state(&state).unwrap();
        assert!(manager.recovery_file_exists());

        manager.delete_recovery_file().unwrap();
        assert!(!manager.recovery_file_exists());
    }

    #[test]
    fn test_recovery_manager_no_file() {
        let temp_dir = TempDir::new().unwrap();
        let manager = RecoveryManager::new(temp_dir.path());

        assert!(!manager.recovery_file_exists());

        let loaded_state = manager.load_state().unwrap();
        assert!(loaded_state.is_none());

        manager.delete_recovery_file().unwrap();
    }

    #[test]
    fn test_parse_validation_error_path() {
        let error1 = "Path '/test_sequences/0/steps': Invalid";
        let paths1 = RecoveryManager::parse_validation_error_path(error1);
        assert_eq!(paths1, vec!["", "test_sequences", "0", "steps"]);

        let error2 = "Path 'root': Missing field";
        let paths2 = RecoveryManager::parse_validation_error_path(error2);
        assert!(paths2.is_empty());

        let error3 = "  - Path '/item': Invalid type";
        let paths3 = RecoveryManager::parse_validation_error_path(error3);
        assert_eq!(paths3, vec!["", "item"]);
    }

    #[test]
    fn test_extract_validation_errors_from_anyhow() {
        let error_msg = "Schema validation failed:\n  - Path '/item': Invalid type\n  - Path '/tc': Missing required";
        let error = anyhow::anyhow!("{}", error_msg);

        let errors = RecoveryManager::extract_validation_errors_from_anyhow(&error);
        assert_eq!(errors.len(), 2);
        assert!(errors[0].error_message.contains("Invalid type"));
        assert!(errors[1].error_message.contains("Missing required"));
    }

    #[test]
    fn test_recovery_state_serialization() {
        let mut structure = IndexMap::new();
        structure.insert("test".to_string(), Value::String("value".to_string()));

        let mut state = RecoveryState::new(structure, "metadata".to_string());
        state.add_error("field1".to_string(), "Error 1".to_string());

        let json = serde_json::to_string(&state).unwrap();
        let deserialized: RecoveryState = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.current_phase, "metadata");
        assert_eq!(deserialized.validation_errors.len(), 1);
        assert_eq!(deserialized.structure.len(), 1);
    }
}
