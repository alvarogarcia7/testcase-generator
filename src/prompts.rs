use crate::models::{Priority, Status, TestType};
use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};

/// Interactive prompt utilities
pub struct Prompts;

impl Prompts {
    /// Prompt for a string input
    pub fn input(prompt: &str) -> Result<String> {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .interact_text()
            .context("Failed to read input")
    }

    /// Prompt for an optional string input
    pub fn input_optional(prompt: &str) -> Result<Option<String>> {
        let input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .allow_empty(true)
            .interact_text()
            .context("Failed to read input")?;

        if input.trim().is_empty() {
            Ok(None)
        } else {
            Ok(Some(input))
        }
    }

    /// Prompt for a string with a default value
    pub fn input_with_default(prompt: &str, default: &str) -> Result<String> {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(default.to_string())
            .interact_text()
            .context("Failed to read input")
    }

    /// Prompt for a confirmation (yes/no)
    pub fn confirm(prompt: &str) -> Result<bool> {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .interact()
            .context("Failed to read confirmation")
    }

    /// Prompt for a confirmation with a default value
    pub fn confirm_with_default(prompt: &str, default: bool) -> Result<bool> {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(default)
            .interact()
            .context("Failed to read confirmation")
    }

    /// Select a priority level
    pub fn select_priority() -> Result<Priority> {
        let priorities = vec!["Low", "Medium", "High", "Critical"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select priority")
            .items(&priorities)
            .default(1)
            .interact()
            .context("Failed to read selection")?;

        match selection {
            0 => Ok(Priority::Low),
            1 => Ok(Priority::Medium),
            2 => Ok(Priority::High),
            3 => Ok(Priority::Critical),
            _ => unreachable!(),
        }
    }

    /// Select a status
    pub fn select_status() -> Result<Status> {
        let statuses = vec!["Draft", "Active", "Deprecated", "Archived"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select status")
            .items(&statuses)
            .default(0)
            .interact()
            .context("Failed to read selection")?;

        match selection {
            0 => Ok(Status::Draft),
            1 => Ok(Status::Active),
            2 => Ok(Status::Deprecated),
            3 => Ok(Status::Archived),
            _ => unreachable!(),
        }
    }

    /// Select a test type
    pub fn select_test_type() -> Result<TestType> {
        let types = vec![
            "Functional",
            "Integration",
            "Regression",
            "Smoke",
            "Performance",
            "Security",
            "User Acceptance",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select test type")
            .items(&types)
            .default(0)
            .interact()
            .context("Failed to read selection")?;

        match selection {
            0 => Ok(TestType::Functional),
            1 => Ok(TestType::Integration),
            2 => Ok(TestType::Regression),
            3 => Ok(TestType::Smoke),
            4 => Ok(TestType::Performance),
            5 => Ok(TestType::Security),
            6 => Ok(TestType::UserAcceptance),
            _ => unreachable!(),
        }
    }

    /// Select from a list of items
    pub fn select<T: ToString>(prompt: &str, items: &[T]) -> Result<usize> {
        let item_strings: Vec<String> = items.iter().map(|i| i.to_string()).collect();

        Select::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .items(&item_strings)
            .interact()
            .context("Failed to read selection")
    }

    /// Multi-select from a list of items
    pub fn multi_select<T: ToString>(prompt: &str, items: &[T]) -> Result<Vec<usize>> {
        let item_strings: Vec<String> = items.iter().map(|i| i.to_string()).collect();

        MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .items(&item_strings)
            .interact()
            .context("Failed to read selection")
    }

    /// Prompt for tags (comma-separated)
    pub fn input_tags(prompt: &str) -> Result<Vec<String>> {
        let input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .allow_empty(true)
            .interact_text()
            .context("Failed to read input")?;

        if input.trim().is_empty() {
            Ok(Vec::new())
        } else {
            Ok(input
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect())
        }
    }
}
