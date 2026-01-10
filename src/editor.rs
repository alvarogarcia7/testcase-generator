use crate::config::EditorConfig;
use crate::models::TestCase;
use crate::yaml_utils::log_yaml_parse_error;
use anyhow::{anyhow, Context, Result};
use std::fs;
use std::process::Command;

/// Editor integration with validation loop
pub struct EditorFlow {
    config: EditorConfig,
    max_retries: usize,
}

impl EditorFlow {
    pub fn new(config: EditorConfig) -> Self {
        Self::new_with_max_retries(config, 3)
    }

    pub fn new_with_max_retries(config: EditorConfig, max_retries: usize) -> Self {
        Self {
            config,
            max_retries,
        }
    }

    pub fn edit_with_validation_loop<T, F>(&self, template: &str, mut validate: F) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        F: FnMut(&T) -> Result<()>,
    {
        let mut content = template.to_string();
        let mut retry_count = 0;

        loop {
            let edited_content = self.open_editor(&content)?;

            let result: Result<T, serde_yaml::Error> = serde_yaml::from_str(&edited_content);

            match result {
                Ok(parsed) => match validate(&parsed) {
                    Ok(()) => return Ok(parsed),
                    Err(validation_error) => {
                        if retry_count >= self.max_retries {
                            return Err(anyhow!(
                                "Validation failed after {} retries ({}  total attempts): {}",
                                self.max_retries,
                                retry_count + 1,
                                validation_error
                            ));
                        }
                        retry_count += 1;
                        eprintln!(
                            "\n⚠ Validation failed (retry {}/{}): {}",
                            retry_count, self.max_retries, validation_error
                        );
                        eprintln!("The editor will reopen with error annotations.\n");
                        content = self.annotate_with_error(&edited_content, &validation_error);
                    }
                },
                Err(parse_error) => {
                    log_yaml_parse_error(&parse_error, &edited_content, "editor buffer");
                    if retry_count >= self.max_retries {
                        return Err(anyhow!(
                            "Parse error after {} retries ({} total attempts): {}",
                            self.max_retries,
                            retry_count + 1,
                            parse_error
                        ));
                    }
                    retry_count += 1;
                    eprintln!(
                        "\n⚠ Parse error (retry {}/{}): {}",
                        retry_count, self.max_retries, parse_error
                    );
                    eprintln!("The editor will reopen with error annotations.\n");
                    content = self.annotate_with_error(&edited_content, &parse_error.into());
                }
            }
        }
    }

    fn open_editor(&self, content: &str) -> Result<String> {
        let editor_path = self.config
            .get_editor()
            .ok_or_else(|| anyhow!("No editor configured. Please set VISUAL, EDITOR, or CUSTOM_FALLBACK environment variable"))?;

        let temp_file =
            tempfile::NamedTempFile::new().context("Failed to create temporary file")?;
        let temp_path = temp_file.path();

        fs::write(temp_path, content).context("Failed to write content to temporary file")?;

        let status = Command::new(&editor_path)
            .arg(temp_path)
            .status()
            .context(format!("Failed to execute editor: {}", editor_path))?;

        if !status.success() {
            return Err(anyhow!(
                "Editor exited with non-zero status: {}",
                status.code().unwrap_or(-1)
            ));
        }

        let edited_content = fs::read_to_string(temp_path)
            .context("Failed to read edited content from temporary file")?;

        Ok(edited_content)
    }

    fn annotate_with_error(&self, content: &str, error: &anyhow::Error) -> String {
        let error_message = format!("# VALIDATION ERROR: {}", error);
        let separator = "#".repeat(40);

        format!(
            r#"{}
{}
{}
{}
"#,
            separator, error_message, separator, content
        )
    }
}

/// Editor integration for test cases
pub struct TestCaseEditor;

impl TestCaseEditor {
    /// Open a test case in the default editor
    pub fn edit_test_case(test_case: &TestCase) -> Result<TestCase> {
        let yaml_content =
            serde_yaml::to_string(test_case).context("Failed to serialize test case to YAML")?;

        let edited_content = edit::edit(yaml_content).context("Failed to open editor")?;

        let edited_test_case: TestCase = match serde_yaml::from_str(&edited_content) {
            Ok(tc) => tc,
            Err(e) => {
                log_yaml_parse_error(&e, &edited_content, "editor buffer");
                return Err(anyhow::anyhow!("Failed to parse edited YAML: {}", e));
            }
        };

        Ok(edited_test_case)
    }

    /// Create a new test case using the editor
    pub fn create_test_case(template: &TestCase) -> Result<TestCase> {
        Self::edit_test_case(template)
    }

    /// Edit arbitrary YAML content with configured editor
    pub fn edit_yaml(content: &str, config: &EditorConfig) -> Result<String> {
        Self::edit_with_config(content, config)
    }

    /// Edit text content with configured editor
    pub fn edit_text(content: &str, config: &EditorConfig) -> Result<String> {
        Self::edit_with_config(content, config)
    }

    /// Edit content using the configured editor
    fn edit_with_config(content: &str, config: &EditorConfig) -> Result<String> {
        let editor_path = config
            .get_editor()
            .ok_or_else(|| anyhow!("No editor configured. Please set VISUAL, EDITOR, or CUSTOM_FALLBACK environment variable"))?;

        let temp_file =
            tempfile::NamedTempFile::new().context("Failed to create temporary file")?;
        let temp_path = temp_file.path();

        fs::write(temp_path, content).context("Failed to write content to temporary file")?;

        let status = Command::new(&editor_path)
            .arg(temp_path)
            .status()
            .context(format!("Failed to execute editor: {}", editor_path))?;

        if !status.success() {
            return Err(anyhow!(
                "Editor exited with non-zero status: {}",
                status.code().unwrap_or(-1)
            ));
        }

        let edited_content = fs::read_to_string(temp_path)
            .context("Failed to read edited content from temporary file")?;

        Ok(edited_content)
    }
}
