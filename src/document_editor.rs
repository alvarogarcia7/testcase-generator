use anyhow::{Context, Result};
use indexmap::IndexMap;
use serde_yaml::Value;
use sha2::{Digest, Sha256};
use std::path::Path;

use crate::complex_structure_editor::ComplexStructureEditor;
use crate::config::EditorConfig;
use crate::models::Step;
use crate::oracle::Oracle;
use crate::validation::SchemaValidator;

pub struct DocumentEditor;

impl DocumentEditor {
    /// Load a YAML document and deserialize into IndexMap
    ///
    /// # Arguments
    /// * `file_path` - Path to the YAML file to load
    ///
    /// # Returns
    /// * `Ok(IndexMap<String, Value>)` - Successfully parsed document structure
    /// * `Err` - If file reading or deserialization fails
    pub fn load_document<P: AsRef<Path>>(file_path: P) -> Result<IndexMap<String, Value>> {
        let content = std::fs::read_to_string(file_path.as_ref()).context("Failed to read file")?;

        let document: IndexMap<String, Value> =
            serde_yaml::from_str(&content).context("Failed to deserialize YAML")?;

        Ok(document)
    }

    /// Compute SHA256 hash of a document structure
    ///
    /// # Arguments
    /// * `document` - The document structure to hash
    ///
    /// # Returns
    /// * `Ok(String)` - Hexadecimal string representation of the SHA256 hash
    /// * `Err` - If serialization fails
    pub fn compute_document_hash(document: &IndexMap<String, Value>) -> Result<String> {
        let yaml_content =
            serde_yaml::to_string(document).context("Failed to serialize document")?;

        let mut hasher = Sha256::new();
        hasher.update(yaml_content.as_bytes());
        let hash_result = hasher.finalize();

        Ok(format!("{:x}", hash_result))
    }

    /// Save document with change detection
    ///
    /// Compares the before and after hashes of the document. If unchanged, logs at INFO level.
    /// If modified, saves the document and logs success at INFO level.
    ///
    /// # Arguments
    /// * `file_path` - Path where to save the document
    /// * `document_before` - Original document state for comparison
    /// * `document_after` - Modified document state to save
    ///
    /// # Returns
    /// * `Ok(())` - Successfully saved or detected no changes
    /// * `Err` - If hashing, serialization, or file writing fails
    pub fn save_document_with_change_detection<P: AsRef<Path>>(
        file_path: P,
        document_before: &IndexMap<String, Value>,
        document_after: &IndexMap<String, Value>,
    ) -> Result<()> {
        let hash_before = Self::compute_document_hash(document_before)?;
        let hash_after = Self::compute_document_hash(document_after)?;

        if hash_before == hash_after {
            log::info!("No changes detected in document");
            Ok(())
        } else {
            let yaml_content = serde_yaml::to_string(document_after)
                .context("Failed to serialize document for saving")?;

            std::fs::write(file_path.as_ref(), yaml_content)
                .context("Failed to write document to file")?;

            log::info!("Document saved successfully to {:?}", file_path.as_ref());
            Ok(())
        }
    }

    /// Display an interactive section selection menu
    ///
    /// # Arguments
    /// * `sections` - List of section names to display in the menu
    ///
    /// # Returns
    /// * `Ok(String)` - The selected section name
    /// * `Err` - If menu interaction fails
    pub fn prompt_section_menu(sections: Vec<String>) -> Result<String> {
        use dialoguer::{theme::ColorfulTheme, Select};

        if sections.is_empty() {
            anyhow::bail!("No sections available to select from");
        }

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a section to edit")
            .items(&sections)
            .interact()
            .context("Failed to display section menu")?;

        Ok(sections[selection].clone())
    }

    /// Edit steps in a sequence with fuzzy search, validation, and add/delete options
    ///
    /// This method provides an interactive flow for editing steps within a sequence:
    /// 1. Lists all steps in the specified sequence
    /// 2. Allows fuzzy selection of a specific step to edit
    /// 3. Uses ComplexStructureEditor<Step> to edit the selected step
    /// 4. Validates the edited step against the schema
    /// 5. Replaces the step in the sequence
    /// 6. Provides options to add new steps or delete existing steps
    ///
    /// # Arguments
    /// * `document` - Mutable reference to the document structure
    /// * `sequence_index` - Index of the sequence in the test_sequences array
    /// * `oracle` - Oracle for user interaction
    /// * `editor_config` - Editor configuration
    /// * `validator` - Schema validator for validation
    ///
    /// # Returns
    /// * `Ok(())` - Successfully completed the editing flow
    /// * `Err` - If editing fails, validation fails, or required data is missing
    pub fn edit_sequence_steps(
        document: &mut IndexMap<String, Value>,
        sequence_index: usize,
        oracle: &dyn Oracle,
        editor_config: &EditorConfig,
        validator: &SchemaValidator,
    ) -> Result<()> {
        log::info!("\n=== Edit Sequence Steps ===\n");

        // Get the sequence at the specified index
        let sequences = document
            .get_mut("test_sequences")
            .and_then(|v| {
                if let Value::Sequence(seq) = v {
                    Some(seq)
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow::anyhow!("test_sequences not found or not a sequence"))?;

        if sequence_index >= sequences.len() {
            anyhow::bail!("Invalid sequence index: {}", sequence_index);
        }

        loop {
            // Get the current sequence
            let sequence = sequences
                .get_mut(sequence_index)
                .ok_or_else(|| anyhow::anyhow!("Sequence not found at index {}", sequence_index))?;

            // Extract sequence name for display
            let sequence_name = if let Value::Mapping(seq_map) = sequence {
                seq_map
                    .get(Value::String("name".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string()
            } else {
                anyhow::bail!("Sequence is not a mapping");
            };

            // Extract steps from the sequence
            let steps_value = if let Value::Mapping(seq_map) = sequence {
                seq_map
                    .get_mut(Value::String("steps".to_string()))
                    .ok_or_else(|| anyhow::anyhow!("steps field not found in sequence"))?
            } else {
                anyhow::bail!("Sequence is not a mapping");
            };

            let steps_array = if let Value::Sequence(steps) = steps_value {
                steps
            } else {
                anyhow::bail!("steps is not a sequence");
            };

            // Convert steps to Step structs for display
            let step_items: Vec<Step> = steps_array
                .iter()
                .filter_map(|step_value| serde_yaml::from_value(step_value.clone()).ok())
                .collect();

            if step_items.is_empty() {
                log::info!(
                    "No steps in sequence '{}'. Would you like to add a step?",
                    sequence_name
                );
                if oracle.confirm("Add a new step?")? {
                    Self::add_step_to_sequence(
                        sequences,
                        sequence_index,
                        oracle,
                        editor_config,
                        validator,
                    )?;
                    continue;
                } else {
                    log::info!("No steps to edit. Exiting step editor.");
                    break;
                }
            }

            log::info!("\nSequence: {}", sequence_name);
            log::info!("Total steps: {}\n", step_items.len());

            // Display available actions
            let actions = vec![
                "Edit a step".to_string(),
                "Add a new step".to_string(),
                "Delete a step".to_string(),
                "Done (exit step editor)".to_string(),
            ];

            let action = oracle.select("Select action", actions)?;

            match action.as_str() {
                "Edit a step" => {
                    // List all steps for fuzzy selection
                    let step_displays: Vec<String> = step_items
                        .iter()
                        .map(|step| {
                            format!(
                                "Step {}: {} ({})",
                                step.step, step.description, step.command
                            )
                        })
                        .collect();

                    let selected_display = oracle
                        .fuzzy_search_strings(&step_displays, "Select step to edit: ")?
                        .ok_or_else(|| anyhow::anyhow!("No step selected"))?;

                    // Find the index of the selected step
                    let step_index = step_displays
                        .iter()
                        .position(|s| s == &selected_display)
                        .ok_or_else(|| anyhow::anyhow!("Selected step not found"))?;

                    let selected_step = &step_items[step_index];
                    log::info!("\nEditing: {}", selected_display);

                    // Create a template from the existing step
                    let template = serde_yaml::to_string(selected_step)
                        .context("Failed to serialize step to YAML")?;

                    // Use ComplexStructureEditor to edit the step
                    let edited_step = ComplexStructureEditor::<Step>::edit_with_fuzzy_search(
                        &step_items,
                        "Select step template (ESC to edit current): ",
                        oracle,
                        editor_config,
                        validator,
                        &template,
                    )?;

                    // Validate the edited step
                    log::info!("\n=== Validating Step ===");
                    let step_yaml = serde_yaml::to_string(&edited_step)
                        .context("Failed to serialize edited step to YAML")?;
                    validator
                        .validate_chunk(&format!("test_sequences:\n  - id: 1\n    name: temp\n    description: temp\n    initial_conditions: {{}}\n    steps:\n      - {}", step_yaml))
                        .context("Step validation failed")?;
                    log::info!("✓ Step is valid\n");

                    // Convert edited step back to Value and replace in sequence
                    let edited_step_value = serde_yaml::to_value(&edited_step)
                        .context("Failed to convert edited step to YAML value")?;

                    // Replace the step in the sequence
                    if let Value::Mapping(seq_map) = sequences.get_mut(sequence_index).unwrap() {
                        if let Some(Value::Sequence(steps)) =
                            seq_map.get_mut(Value::String("steps".to_string()))
                        {
                            if step_index < steps.len() {
                                steps[step_index] = edited_step_value;
                                log::info!("✓ Step {} updated successfully\n", step_index + 1);
                            } else {
                                anyhow::bail!("Invalid step index: {}", step_index);
                            }
                        }
                    }
                }
                "Add a new step" => {
                    Self::add_step_to_sequence(
                        sequences,
                        sequence_index,
                        oracle,
                        editor_config,
                        validator,
                    )?;
                }
                "Delete a step" => {
                    Self::delete_step_from_sequence(sequences, sequence_index, oracle)?;
                }
                "Done (exit step editor)" => {
                    log::info!("Exiting step editor.");
                    break;
                }
                _ => {
                    log::warn!("Unknown action selected");
                }
            }
        }

        Ok(())
    }

    /// Add a new step to the sequence
    fn add_step_to_sequence(
        sequences: &mut [Value],
        sequence_index: usize,
        oracle: &dyn Oracle,
        editor_config: &EditorConfig,
        validator: &SchemaValidator,
    ) -> Result<()> {
        log::info!("\n=== Add New Step ===\n");

        // Get the next step number
        let next_step_number = if let Some(Value::Mapping(seq_map)) = sequences.get(sequence_index)
        {
            if let Some(Value::Sequence(steps)) = seq_map.get(Value::String("steps".to_string())) {
                let max_step = steps
                    .iter()
                    .filter_map(|step| {
                        if let Value::Mapping(step_map) = step {
                            step_map
                                .get(Value::String("step".to_string()))
                                .and_then(|v| v.as_i64())
                        } else {
                            None
                        }
                    })
                    .max()
                    .unwrap_or(0);
                max_step + 1
            } else {
                1
            }
        } else {
            1
        };

        // Create a template for a new step
        let template = format!(
            r#"step: {}
description: ""
command: ""
expected:
  result: ""
  output: ""
"#,
            next_step_number
        );

        // Get all existing steps for template selection
        let step_items: Vec<Step> = if let Some(Value::Mapping(seq_map)) =
            sequences.get(sequence_index)
        {
            if let Some(Value::Sequence(steps)) = seq_map.get(Value::String("steps".to_string())) {
                steps
                    .iter()
                    .filter_map(|step_value| serde_yaml::from_value(step_value.clone()).ok())
                    .collect()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        // Use ComplexStructureEditor to create the new step
        let new_step = ComplexStructureEditor::<Step>::edit_with_fuzzy_search(
            &step_items,
            "Select step template (ESC to create new): ",
            oracle,
            editor_config,
            validator,
            &template,
        )?;

        // Validate the new step
        log::info!("\n=== Validating Step ===");
        let step_yaml =
            serde_yaml::to_string(&new_step).context("Failed to serialize new step to YAML")?;
        validator
            .validate_chunk(&format!(
                "test_sequences:\n  - id: 1\n    name: temp\n    description: temp\n    initial_conditions: {{}}\n    steps:\n      - {}",
                step_yaml
            ))
            .context("Step validation failed")?;
        log::info!("✓ Step is valid\n");

        // Convert new step to Value and append to sequence
        let new_step_value =
            serde_yaml::to_value(&new_step).context("Failed to convert new step to YAML value")?;

        // Append the step to the sequence
        if let Some(Value::Mapping(seq_map)) = sequences.get_mut(sequence_index) {
            if let Some(Value::Sequence(steps)) =
                seq_map.get_mut(Value::String("steps".to_string()))
            {
                steps.push(new_step_value);
                log::info!("✓ Step {} added successfully\n", new_step.step);
            } else {
                anyhow::bail!("steps is not a sequence");
            }
        } else {
            anyhow::bail!("Sequence is not a mapping");
        }

        Ok(())
    }

    /// Delete a step from the sequence
    fn delete_step_from_sequence(
        sequences: &mut [Value],
        sequence_index: usize,
        oracle: &dyn Oracle,
    ) -> Result<()> {
        log::info!("\n=== Delete Step ===\n");

        // Get the steps from the sequence
        let steps_array = if let Some(Value::Mapping(seq_map)) = sequences.get(sequence_index) {
            if let Some(Value::Sequence(steps)) = seq_map.get(Value::String("steps".to_string())) {
                steps.clone()
            } else {
                anyhow::bail!("steps is not a sequence");
            }
        } else {
            anyhow::bail!("Sequence is not a mapping");
        };

        if steps_array.is_empty() {
            log::info!("No steps to delete.");
            return Ok(());
        }

        // Convert steps to Step structs for display
        let step_items: Vec<Step> = steps_array
            .iter()
            .filter_map(|step_value| serde_yaml::from_value(step_value.clone()).ok())
            .collect();

        // List all steps for selection
        let step_displays: Vec<String> = step_items
            .iter()
            .map(|step| {
                format!(
                    "Step {}: {} ({})",
                    step.step, step.description, step.command
                )
            })
            .collect();

        let selected_display = oracle
            .fuzzy_search_strings(&step_displays, "Select step to delete: ")?
            .ok_or_else(|| anyhow::anyhow!("No step selected"))?;

        // Find the index of the selected step
        let step_index = step_displays
            .iter()
            .position(|s| s == &selected_display)
            .ok_or_else(|| anyhow::anyhow!("Selected step not found"))?;

        // Confirm deletion
        let confirm_msg = format!("Are you sure you want to delete '{}'?", selected_display);
        if !oracle.confirm(&confirm_msg)? {
            log::info!("Deletion cancelled.");
            return Ok(());
        }

        // Remove the step from the sequence
        if let Some(Value::Mapping(seq_map)) = sequences.get_mut(sequence_index) {
            if let Some(Value::Sequence(steps)) =
                seq_map.get_mut(Value::String("steps".to_string()))
            {
                if step_index < steps.len() {
                    steps.remove(step_index);
                    log::info!("✓ Step deleted successfully\n");
                } else {
                    anyhow::bail!("Invalid step index: {}", step_index);
                }
            } else {
                anyhow::bail!("steps is not a sequence");
            }
        } else {
            anyhow::bail!("Sequence is not a mapping");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_document() {
        let temp_file = NamedTempFile::new().unwrap();
        let yaml_content = r#"
requirement: REQ001
item: 1
tc: 2
description: Test case
"#;
        std::fs::write(temp_file.path(), yaml_content).unwrap();

        let document = DocumentEditor::load_document(temp_file.path()).unwrap();
        assert_eq!(document.len(), 4);
        assert!(document.contains_key("requirement"));
        assert!(document.contains_key("item"));
    }

    #[test]
    fn test_load_document_invalid_path() {
        let result = DocumentEditor::load_document("/nonexistent/path/file.yaml");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_document_invalid_yaml() {
        let temp_file = NamedTempFile::new().unwrap();
        let invalid_yaml = r#"
invalid: yaml: content:
  - broken
    structure
"#;
        std::fs::write(temp_file.path(), invalid_yaml).unwrap();

        let result = DocumentEditor::load_document(temp_file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_compute_document_hash() {
        let mut document = IndexMap::new();
        document.insert("key1".to_string(), Value::String("value1".to_string()));
        document.insert("key2".to_string(), Value::Number(42.into()));

        let hash1 = DocumentEditor::compute_document_hash(&document).unwrap();
        assert_eq!(hash1.len(), 64);

        let hash2 = DocumentEditor::compute_document_hash(&document).unwrap();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_compute_document_hash_different_content() {
        let mut document1 = IndexMap::new();
        document1.insert("key1".to_string(), Value::String("value1".to_string()));

        let mut document2 = IndexMap::new();
        document2.insert("key1".to_string(), Value::String("value2".to_string()));

        let hash1 = DocumentEditor::compute_document_hash(&document1).unwrap();
        let hash2 = DocumentEditor::compute_document_hash(&document2).unwrap();

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_save_document_with_change_detection_no_changes() {
        let temp_file = NamedTempFile::new().unwrap();

        let mut document = IndexMap::new();
        document.insert("key1".to_string(), Value::String("value1".to_string()));

        let result = DocumentEditor::save_document_with_change_detection(
            temp_file.path(),
            &document,
            &document,
        );
        assert!(result.is_ok());

        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert!(content.is_empty() || content.trim().is_empty());
    }

    #[test]
    fn test_save_document_with_change_detection_with_changes() {
        let temp_file = NamedTempFile::new().unwrap();

        let mut document_before = IndexMap::new();
        document_before.insert("key1".to_string(), Value::String("value1".to_string()));

        let mut document_after = IndexMap::new();
        document_after.insert("key1".to_string(), Value::String("value2".to_string()));

        let result = DocumentEditor::save_document_with_change_detection(
            temp_file.path(),
            &document_before,
            &document_after,
        );
        assert!(result.is_ok());

        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert!(content.contains("value2"));
    }

    #[test]
    fn test_prompt_section_menu_empty() {
        let sections = Vec::new();
        let result = DocumentEditor::prompt_section_menu(sections);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No sections available"));
    }

    #[test]
    fn test_load_and_hash_roundtrip() {
        let temp_file = NamedTempFile::new().unwrap();
        let yaml_content = r#"
requirement: REQ001
item: 1
tc: 2
description: Test case
"#;
        std::fs::write(temp_file.path(), yaml_content).unwrap();

        let document = DocumentEditor::load_document(temp_file.path()).unwrap();
        let hash = DocumentEditor::compute_document_hash(&document).unwrap();

        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_hash_consistency_with_order() {
        let mut document1 = IndexMap::new();
        document1.insert("a".to_string(), Value::String("1".to_string()));
        document1.insert("b".to_string(), Value::String("2".to_string()));

        let mut document2 = IndexMap::new();
        document2.insert("b".to_string(), Value::String("2".to_string()));
        document2.insert("a".to_string(), Value::String("1".to_string()));

        let hash1 = DocumentEditor::compute_document_hash(&document1).unwrap();
        let hash2 = DocumentEditor::compute_document_hash(&document2).unwrap();

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_save_preserves_structure() {
        let temp_file = NamedTempFile::new().unwrap();

        let mut document_before = IndexMap::new();
        document_before.insert("key1".to_string(), Value::String("value1".to_string()));
        document_before.insert("key2".to_string(), Value::Number(42.into()));

        let mut document_after = IndexMap::new();
        document_after.insert("key1".to_string(), Value::String("modified".to_string()));
        document_after.insert("key2".to_string(), Value::Number(99.into()));

        DocumentEditor::save_document_with_change_detection(
            temp_file.path(),
            &document_before,
            &document_after,
        )
        .unwrap();

        let loaded = DocumentEditor::load_document(temp_file.path()).unwrap();
        assert_eq!(
            loaded.get("key1"),
            Some(&Value::String("modified".to_string()))
        );
        assert_eq!(loaded.get("key2"), Some(&Value::Number(99.into())));
    }
}
