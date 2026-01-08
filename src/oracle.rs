use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use std::io::{self, Write};

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

/// Menu-based CLI Oracle with TTY detection fallback for all prompt types
pub struct MenuCliOracle;

impl MenuCliOracle {
    /// Create a new MenuCliOracle instance
    pub fn new() -> Self {
        Self
    }

    /// Check if we're in a TTY environment
    fn is_tty() -> bool {
        use std::io::IsTerminal;
        io::stdin().is_terminal()
    }

    /// Numbered selection fallback for single select
    fn numbered_selection<T: ToString>(items: &[T], prompt: &str) -> Result<usize> {
        if items.is_empty() {
            anyhow::bail!("Cannot select from empty list");
        }

        let item_strings: Vec<String> = items.iter().map(|i| i.to_string()).collect();

        println!("\n⚠ TTY not detected (e.g., VS Code debug console)");
        println!("Interactive menu unavailable - using numbered selection instead\n");
        println!("{}", prompt);
        println!();

        for (idx, item) in item_strings.iter().enumerate() {
            println!("{}) {}", idx + 1, item);
        }

        println!();

        loop {
            print!("Enter number (1-{}): ", item_strings.len());
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let input = input.trim();

            match input.parse::<usize>() {
                Ok(num) if num > 0 && num <= item_strings.len() => {
                    return Ok(num - 1);
                }
                _ => {
                    println!(
                        "❌ Invalid input. Please enter a number between 1 and {}\n",
                        item_strings.len()
                    );
                }
            }
        }
    }

    /// Numbered multi-selection fallback
    fn numbered_multi_selection<T: ToString>(items: &[T], prompt: &str) -> Result<Vec<usize>> {
        if items.is_empty() {
            return Ok(Vec::new());
        }

        let item_strings: Vec<String> = items.iter().map(|i| i.to_string()).collect();

        println!("\n⚠ TTY not detected (e.g., VS Code debug console)");
        println!("Interactive menu unavailable - using numbered multi-selection instead\n");
        println!("{}", prompt);
        println!();

        for (idx, item) in item_strings.iter().enumerate() {
            println!("{}) {}", idx + 1, item);
        }

        println!();
        println!("Enter numbers separated by spaces (e.g., '1 3 5'), or 0/empty to skip:");

        let mut selected = Vec::new();

        loop {
            print!("> ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let input = input.trim();

            if input == "0" || input.is_empty() {
                if selected.is_empty() {
                    println!("No items selected.\n");
                }
                break;
            }

            let mut valid = true;
            let mut temp_selected = Vec::new();

            for num_str in input.split_whitespace() {
                match num_str.parse::<usize>() {
                    Ok(num) if num > 0 && num <= item_strings.len() => {
                        let idx = num - 1;
                        if !temp_selected.contains(&idx) {
                            temp_selected.push(idx);
                        }
                    }
                    _ => {
                        println!(
                            "❌ Invalid input: '{}'. Please enter numbers between 1 and {}\n",
                            num_str,
                            item_strings.len()
                        );
                        valid = false;
                        break;
                    }
                }
            }

            if valid {
                selected = temp_selected;
                println!("✓ Selected {} item(s)\n", selected.len());
                break;
            }
        }

        Ok(selected)
    }

    /// Numbered input fallback for string input
    fn numbered_input(prompt: &str) -> Result<String> {
        println!("\n⚠ TTY not detected (e.g., VS Code debug console)");
        println!("Interactive input unavailable - using simple input instead\n");
        print!("{}: ", prompt);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(input.trim().to_string())
    }

    /// Numbered input fallback for optional string input
    fn numbered_input_optional(prompt: &str) -> Result<Option<String>> {
        println!("\n⚠ TTY not detected (e.g., VS Code debug console)");
        println!("Interactive input unavailable - using simple input instead\n");
        println!("{} (press Enter to skip):", prompt);
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input.is_empty() {
            Ok(None)
        } else {
            Ok(Some(input.to_string()))
        }
    }

    /// Numbered input fallback for integer input
    fn numbered_input_integer(prompt: &str) -> Result<i64> {
        println!("\n⚠ TTY not detected (e.g., VS Code debug console)");
        println!("Interactive input unavailable - using simple input instead\n");

        loop {
            print!("{}: ", prompt);
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            match input.trim().parse::<i64>() {
                Ok(value) => return Ok(value),
                Err(_) => println!("❌ Please enter a valid integer\n"),
            }
        }
    }

    /// Numbered confirm fallback
    fn numbered_confirm(prompt: &str) -> Result<bool> {
        println!("\n⚠ TTY not detected (e.g., VS Code debug console)");
        println!("Interactive confirmation unavailable - using simple y/n input instead\n");

        loop {
            print!("{} (y/n): ", prompt);
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            match input.trim().to_lowercase().as_str() {
                "y" | "yes" => return Ok(true),
                "n" | "no" => return Ok(false),
                _ => println!("❌ Please enter 'y' or 'n'\n"),
            }
        }
    }
}

impl Default for MenuCliOracle {
    fn default() -> Self {
        Self::new()
    }
}

impl Oracle for MenuCliOracle {
    fn input(&self, prompt: &str) -> Result<String> {
        if !Self::is_tty() {
            return Self::numbered_input(prompt);
        }

        Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .interact_text()
            .context("Failed to read input")
    }

    fn input_optional(&self, prompt: &str) -> Result<Option<String>> {
        if !Self::is_tty() {
            return Self::numbered_input_optional(prompt);
        }

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
        if !Self::is_tty() {
            return Self::numbered_input_integer(prompt);
        }

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
        if !Self::is_tty() {
            return Self::numbered_confirm(prompt);
        }

        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .interact()
            .context("Failed to read confirmation")
    }

    fn select<T: ToString>(&self, prompt: &str, items: &[T]) -> Result<usize> {
        if !Self::is_tty() {
            return Self::numbered_selection(items, prompt);
        }

        let item_strings: Vec<String> = items.iter().map(|i| i.to_string()).collect();

        Select::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .items(&item_strings)
            .interact()
            .context("Failed to read selection")
    }

    fn multi_select<T: ToString>(&self, prompt: &str, items: &[T]) -> Result<Vec<usize>> {
        if !Self::is_tty() {
            return Self::numbered_multi_selection(items, prompt);
        }

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
