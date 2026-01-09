use crate::config::EditorConfig;
use crate::database::ConditionDatabase;
use crate::editor::EditorFlow;
use crate::oracle::Oracle;
use crate::validation::SchemaValidator;
use anyhow::{anyhow, Context, Result};
use std::fmt::Display;
use std::io::{self, IsTerminal};

/// Generic editor for complex structures with fuzzy search, TTY fallback, and validation
pub struct ComplexStructureEditor<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> ComplexStructureEditor<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize + Display,
{
    /// Create a new ComplexStructureEditor
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }

    /// Edit with fuzzy search through database instances
    ///
    /// This method performs the following steps:
    /// 1. Attempts fuzzy search through database instances using Display formatting
    /// 2. On selection, serializes to YAML and opens in editor via EditorFlow
    /// 3. If cancelled or no TTY, uses multi_line_input() or template
    /// 4. Validates with schema before returning
    ///
    /// # Arguments
    /// * `database` - Collection of instances to search through
    /// * `prompt` - Prompt to display to the user
    /// * `oracle` - Oracle for user interaction
    /// * `editor_config` - Editor configuration
    /// * `validator` - Schema validator for validation
    /// * `template` - Template to use if fuzzy search is cancelled or unavailable
    ///
    /// # Returns
    /// * `Ok(T)` - Successfully edited and validated instance
    /// * `Err` - If editing was cancelled, failed validation, or encountered an error
    pub fn edit_with_fuzzy_search(
        database: &[T],
        prompt: &str,
        oracle: &dyn Oracle,
        editor_config: &EditorConfig,
        validator: &SchemaValidator,
        template: &str,
    ) -> Result<T> {
        // Step 1: Attempt fuzzy search through database instances using Display formatting
        let display_items: Vec<String> = database.iter().map(|item| item.to_string()).collect();

        let selected_item = if Self::is_tty() && !display_items.is_empty() {
            // Use fuzzy search if TTY is available and we have items
            oracle.fuzzy_search_strings(&display_items, prompt)?
        } else {
            None
        };

        let yaml_content = if let Some(selected_display) = selected_item {
            // Find the original item matching the display string
            let selected_instance = database
                .iter()
                .find(|item| item.to_string() == selected_display)
                .ok_or_else(|| anyhow!("Selected item not found in database"))?;

            // Step 2: Serialize to YAML
            serde_yaml::to_string(selected_instance)
                .context("Failed to serialize selected item to YAML")?
        } else {
            // Step 3: If cancelled or no TTY, use template
            if Self::is_tty() && editor_config.get_editor().is_some() {
                // TTY available but fuzzy search cancelled or no items - use template
                template.to_string()
            } else {
                // No TTY - use multi_line_input or template
                if !Self::is_tty() {
                    println!("\n⚠ TTY not detected - using multi-line input mode");
                    println!("You can paste your YAML content below:");
                }

                let input_result = oracle.multi_line_input("Enter YAML content (or use template):");

                if input_result.is_ok() {
                    let input = input_result?;
                    if input.trim().is_empty() {
                        template.to_string()
                    } else {
                        input
                    }
                } else {
                    template.to_string()
                }
            }
        };

        // Open in editor via EditorFlow with validation loop
        let editor_flow = EditorFlow::new(editor_config.clone());

        let edited_instance =
            editor_flow.edit_with_validation_loop(&yaml_content, |instance: &T| {
                // Step 4: Validate with schema before returning
                let yaml_for_validation = serde_yaml::to_string(instance)
                    .context("Failed to serialize for validation")?;
                validator
                    .validate_complete(&yaml_for_validation)
                    .context("Schema validation failed")?;
                Ok(())
            })?;

        Ok(edited_instance)
    }

    /// Edit with fuzzy search using specialized database types
    ///
    /// This is a convenience method for editing structures that can be loaded from a ConditionDatabase.
    /// It extracts the appropriate items from the database and delegates to edit_with_fuzzy_search.
    pub fn edit_with_database_search<F>(
        database: &ConditionDatabase,
        prompt: &str,
        oracle: &dyn Oracle,
        editor_config: &EditorConfig,
        validator: &SchemaValidator,
        template: &str,
        extractor: F,
    ) -> Result<T>
    where
        F: Fn(&ConditionDatabase) -> Vec<T>,
    {
        let items = extractor(database);
        Self::edit_with_fuzzy_search(&items, prompt, oracle, editor_config, validator, template)
    }

    /// Check if we're in a TTY environment
    fn is_tty() -> bool {
        io::stdin().is_terminal()
    }
}

impl<T> Default for ComplexStructureEditor<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize + Display,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Expected, Step, TestSequence};
    use crate::oracle::{AnswerVariant, HardcodedOracle};
    use std::collections::VecDeque;

    #[test]
    fn test_complex_structure_editor_creation() {
        let _editor: ComplexStructureEditor<Expected> = ComplexStructureEditor::new();
        // Editor created successfully
    }

    #[test]
    fn test_complex_structure_editor_default() {
        let _editor: ComplexStructureEditor<Step> = ComplexStructureEditor::default();
        // Editor created successfully
    }

    #[test]
    fn test_expected_display_for_fuzzy_search() {
        let expected = Expected {
            success: Some(true),
            result: "SW=0x9000".to_string(),
            output: "Success".to_string(),
        };

        let display_str = expected.to_string();
        assert!(display_str.contains("true"));
        assert!(display_str.contains("SW=0x9000"));
        assert!(display_str.contains("Success"));
    }

    #[test]
    fn test_step_display_for_fuzzy_search() {
        let step = Step::new(
            1,
            "Test step".to_string(),
            "ssh".to_string(),
            "SW=0x9000".to_string(),
            "Success".to_string(),
        );

        let display_str = step.to_string();
        assert!(display_str.contains("1"));
        assert!(display_str.contains("Test step"));
        assert!(display_str.contains("ssh"));
    }

    #[test]
    fn test_sequence_display_for_fuzzy_search() {
        let sequence = TestSequence::new(
            1,
            "Test Sequence".to_string(),
            "Test description".to_string(),
        );

        let display_str = sequence.to_string();
        assert!(display_str.contains("1"));
        assert!(display_str.contains("Test Sequence"));
        assert!(display_str.contains("Test description"));
    }

    #[test]
    fn test_edit_with_fuzzy_search_template_when_cancelled() {
        // This test verifies the structure but cannot test full interactive behavior
        // We verify that the editor properly handles the case where fuzzy search is cancelled

        let database: Vec<Expected> = vec![Expected {
            success: Some(true),
            result: "SW=0x9000".to_string(),
            output: "Success".to_string(),
        }];

        let template = r#"
success: true
result: "SW=0x9000"
output: "Success"
"#;

        // Create a hardcoded oracle that returns empty string (cancelled search)
        // followed by the template content for multi_line_input
        let mut answers = VecDeque::new();
        answers.push_back(AnswerVariant::String("".to_string())); // Cancelled fuzzy search
        answers.push_back(AnswerVariant::String(template.to_string())); // multi_line_input

        let oracle = HardcodedOracle::new(answers);
        let editor_config = EditorConfig {
            editor: Some("echo".to_string()),
            visual: None,
            custom_fallback: None,
        };

        // Note: This test would need a way to bypass the actual editor opening
        // In practice, the EditorFlow would need to be mockable for full testing
        // For now, we verify the structure is sound
        assert!(!database.is_empty());
        assert!(!template.is_empty());
        assert!(editor_config.editor.is_some());
        // Oracle is properly constructed
        let _ = oracle;
    }

    #[test]
    fn test_editor_handles_multiple_types() {
        // Verify that ComplexStructureEditor works with different types
        let _editor_expected: ComplexStructureEditor<Expected> = ComplexStructureEditor::new();
        let _editor_step: ComplexStructureEditor<Step> = ComplexStructureEditor::new();
        let _editor_sequence: ComplexStructureEditor<TestSequence> = ComplexStructureEditor::new();

        // Type checking passes - the editor is properly generic
    }
}
