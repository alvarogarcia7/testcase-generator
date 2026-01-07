use crate::models::TestCase;
use anyhow::{Context, Result};

/// Editor integration for test cases
pub struct TestCaseEditor;

impl TestCaseEditor {
    /// Open a test case in the default editor
    pub fn edit_test_case(test_case: &TestCase) -> Result<TestCase> {
        let yaml_content =
            serde_yaml::to_string(test_case).context("Failed to serialize test case to YAML")?;

        let edited_content = edit::edit(yaml_content).context("Failed to open editor")?;

        let edited_test_case: TestCase =
            serde_yaml::from_str(&edited_content).context("Failed to parse edited YAML")?;

        Ok(edited_test_case)
    }

    /// Create a new test case using the editor
    pub fn create_test_case(template: &TestCase) -> Result<TestCase> {
        Self::edit_test_case(template)
    }

    /// Edit arbitrary YAML content
    pub fn edit_yaml(content: &str) -> Result<String> {
        edit::edit(content).context("Failed to open editor")
    }

    /// Edit text content
    pub fn edit_text(content: &str) -> Result<String> {
        edit::edit(content).context("Failed to open editor")
    }
}
