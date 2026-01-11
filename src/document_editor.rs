use anyhow::{Context, Result};
use indexmap::IndexMap;
use serde_yaml::Value;
use sha2::{Digest, Sha256};
use std::path::Path;

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
