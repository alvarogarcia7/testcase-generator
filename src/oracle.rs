use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};

/// Trait defining the interface for user input operations
pub trait Oracle {
    /// Prompt for a string input
    fn input(&self, prompt: &str) -> Result<String>;

    /// Prompt for an optional string input
    fn input_optional(&self, prompt: &str) -> Result<Option<String>>;

    /// Prompt for an integer input
    fn input_integer(&self, prompt: &str) -> Result<i64>;

    /// Prompt for a confirmation (yes/no)
    fn confirm(&self, prompt: &str) -> Result<bool>;

    /// Select from a list of items, returns the index of the selected item
    fn select<T: ToString>(&self, prompt: &str, items: &[T]) -> Result<usize>;

    /// Multi-select from a list of items, returns indices of selected items
    fn multi_select<T: ToString>(&self, prompt: &str, items: &[T]) -> Result<Vec<usize>>;

    /// Fuzzy search through a list of strings, returns the selected string
    fn fuzzy_search_strings(&self, items: &[String], prompt: &str) -> Result<Option<String>>;
}

/// TTY-based CLI Oracle implementation using dialoguer and skim
pub struct TtyCliOracle;

impl TtyCliOracle {
    /// Create a new TtyCliOracle instance
    pub fn new() -> Self {
        Self
    }
}

impl Default for TtyCliOracle {
    fn default() -> Self {
        Self::new()
    }
}

impl Oracle for TtyCliOracle {
    fn input(&self, prompt: &str) -> Result<String> {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .interact_text()
            .context("Failed to read input")
    }

    fn input_optional(&self, prompt: &str) -> Result<Option<String>> {
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

    fn input_integer(&self, prompt: &str) -> Result<i64> {
        loop {
            let input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .interact_text()
                .context("Failed to read input")?;

            match input.trim().parse::<i64>() {
                Ok(value) => return Ok(value),
                Err(_) => println!("Please enter a valid integer"),
            }
        }
    }

    fn confirm(&self, prompt: &str) -> Result<bool> {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .interact()
            .context("Failed to read confirmation")
    }

    fn select<T: ToString>(&self, prompt: &str, items: &[T]) -> Result<usize> {
        let item_strings: Vec<String> = items.iter().map(|i| i.to_string()).collect();

        Select::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .items(&item_strings)
            .interact()
            .context("Failed to read selection")
    }

    fn multi_select<T: ToString>(&self, prompt: &str, items: &[T]) -> Result<Vec<usize>> {
        let item_strings: Vec<String> = items.iter().map(|i| i.to_string()).collect();

        MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .items(&item_strings)
            .interact()
            .context("Failed to read selection")
    }

    fn fuzzy_search_strings(&self, items: &[String], prompt: &str) -> Result<Option<String>> {
        use crate::fuzzy::TestCaseFuzzyFinder;
        TestCaseFuzzyFinder::search_strings(items, prompt)
    }
}
