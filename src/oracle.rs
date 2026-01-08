use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use std::collections::VecDeque;
use std::io::{self, Write};

/// Trait defining the interface for user input operations
pub trait Oracle {
    /// Prompt for a string input
    fn input(&self, prompt: &str) -> Result<String>;

    /// Prompt for a string input with initial text
    fn input_with_initial_text(&self, prompt: &str, initial_text: &str) -> Result<String>;

    /// Prompt for a string input with a default value
    fn input_with_default(&self, prompt: &str, default: &str) -> Result<String>;

    /// Prompt for an optional string input
    fn input_optional(&self, prompt: &str) -> Result<Option<String>>;

    /// Prompt for an optional string input with initial text
    fn input_optional_with_initial_text(
        &self,
        prompt: &str,
        initial_text: &str,
    ) -> Result<Option<String>>;

    /// Prompt for an integer input
    fn input_integer(&self, prompt: &str) -> Result<i64>;

    /// Prompt for an integer input with initial text
    fn input_integer_with_initial_text(&self, prompt: &str, initial_text: &str) -> Result<i64>;

    /// Prompt for a confirmation (yes/no)
    fn confirm(&self, prompt: &str) -> Result<bool>;

    /// Prompt for a confirmation with a default value
    fn confirm_with_default(&self, prompt: &str, default: bool) -> Result<bool>;

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

    fn input_with_initial_text(&self, prompt: &str, initial_text: &str) -> Result<String> {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .with_initial_text(initial_text)
            .interact_text()
            .context("Failed to read input")
    }

    fn input_with_default(&self, prompt: &str, default: &str) -> Result<String> {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(default.to_string())
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

    fn input_optional_with_initial_text(
        &self,
        prompt: &str,
        initial_text: &str,
    ) -> Result<Option<String>> {
        let input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .with_initial_text(initial_text)
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

    fn input_integer_with_initial_text(&self, prompt: &str, initial_text: &str) -> Result<i64> {
        loop {
            let input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .with_initial_text(initial_text)
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

    fn confirm_with_default(&self, prompt: &str, default: bool) -> Result<bool> {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(default)
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

    /// Numbered input fallback for string input with initial text
    fn numbered_input_with_initial_text(prompt: &str, initial_text: &str) -> Result<String> {
        println!("\n⚠ TTY not detected (e.g., VS Code debug console)");
        println!("Interactive input unavailable - using simple input instead\n");
        println!("{} [default: {}]:", prompt, initial_text);
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input.is_empty() {
            Ok(initial_text.to_string())
        } else {
            Ok(input.to_string())
        }
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

    /// Numbered input fallback for integer input with initial text
    fn numbered_input_integer_with_initial_text(prompt: &str, initial_text: &str) -> Result<i64> {
        println!("\n⚠ TTY not detected (e.g., VS Code debug console)");
        println!("Interactive input unavailable - using simple input instead\n");

        loop {
            println!("{} [default: {}]:", prompt, initial_text);
            print!("> ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let input = input.trim();
            let value_to_parse = if input.is_empty() {
                initial_text
            } else {
                input
            };

            match value_to_parse.parse::<i64>() {
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

    /// Numbered confirm fallback with default
    fn numbered_confirm_with_default(prompt: &str, default: bool) -> Result<bool> {
        println!("\n⚠ TTY not detected (e.g., VS Code debug console)");
        println!("Interactive confirmation unavailable - using simple y/n input instead\n");

        let default_str = if default { "Y/n" } else { "y/N" };

        loop {
            print!("{} ({}): ", prompt, default_str);
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let input = input.trim();
            if input.is_empty() {
                return Ok(default);
            }

            match input.to_lowercase().as_str() {
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

    fn input_with_initial_text(&self, prompt: &str, initial_text: &str) -> Result<String> {
        if !Self::is_tty() {
            return Self::numbered_input_with_initial_text(prompt, initial_text);
        }

        Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .with_initial_text(initial_text)
            .interact_text()
            .context("Failed to read input")
    }

    fn input_with_default(&self, prompt: &str, default: &str) -> Result<String> {
        if !Self::is_tty() {
            return Self::numbered_input_with_initial_text(prompt, default);
        }

        Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(default.to_string())
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

    fn input_optional_with_initial_text(
        &self,
        prompt: &str,
        initial_text: &str,
    ) -> Result<Option<String>> {
        if !Self::is_tty() {
            let result = Self::numbered_input_with_initial_text(prompt, initial_text)?;
            if result.is_empty() {
                Ok(None)
            } else {
                Ok(Some(result))
            }
        } else {
            let input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .with_initial_text(initial_text)
                .allow_empty(true)
                .interact_text()
                .context("Failed to read input")?;

            if input.trim().is_empty() {
                Ok(None)
            } else {
                Ok(Some(input))
            }
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

    fn input_integer_with_initial_text(&self, prompt: &str, initial_text: &str) -> Result<i64> {
        if !Self::is_tty() {
            return Self::numbered_input_integer_with_initial_text(prompt, initial_text);
        }

        loop {
            let input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .with_initial_text(initial_text)
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

    fn confirm_with_default(&self, prompt: &str, default: bool) -> Result<bool> {
        if !Self::is_tty() {
            return Self::numbered_confirm_with_default(prompt, default);
        }

        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(default)
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

/// Enum representing different types of answers for hardcoded responses
#[derive(Debug, Clone)]
pub enum AnswerVariant {
    String(String),
    Int(i64),
    Bool(bool),
    Strings(Vec<String>),
}

/// Hardcoded Oracle for testing that uses a queue-based answer system
pub struct HardcodedOracle {
    answers: std::cell::RefCell<VecDeque<AnswerVariant>>,
}

impl HardcodedOracle {
    /// Create a new HardcodedOracle with a queue of answers
    pub fn new(answers: VecDeque<AnswerVariant>) -> Self {
        Self {
            answers: std::cell::RefCell::new(answers),
        }
    }

    /// Create an empty HardcodedOracle
    pub fn empty() -> Self {
        Self {
            answers: std::cell::RefCell::new(VecDeque::new()),
        }
    }

    /// Add an answer to the queue
    pub fn add_answer(&self, answer: AnswerVariant) {
        self.answers.borrow_mut().push_back(answer);
    }

    /// Get the next answer from the queue
    fn next_answer(&self) -> Result<AnswerVariant> {
        self.answers
            .borrow_mut()
            .pop_front()
            .ok_or_else(|| anyhow::anyhow!("No more hardcoded answers available"))
    }
}

impl Oracle for HardcodedOracle {
    fn input(&self, _prompt: &str) -> Result<String> {
        match self.next_answer()? {
            AnswerVariant::String(s) => Ok(s),
            _ => anyhow::bail!("Expected String answer variant"),
        }
    }

    fn input_with_initial_text(&self, _prompt: &str, _initial_text: &str) -> Result<String> {
        match self.next_answer()? {
            AnswerVariant::String(s) => Ok(s),
            _ => anyhow::bail!("Expected String answer variant"),
        }
    }

    fn input_with_default(&self, _prompt: &str, _default: &str) -> Result<String> {
        match self.next_answer()? {
            AnswerVariant::String(s) => Ok(s),
            _ => anyhow::bail!("Expected String answer variant"),
        }
    }

    fn input_optional(&self, _prompt: &str) -> Result<Option<String>> {
        match self.next_answer()? {
            AnswerVariant::String(s) => {
                if s.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(s))
                }
            }
            _ => anyhow::bail!("Expected String answer variant"),
        }
    }

    fn input_optional_with_initial_text(
        &self,
        _prompt: &str,
        _initial_text: &str,
    ) -> Result<Option<String>> {
        match self.next_answer()? {
            AnswerVariant::String(s) => {
                if s.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(s))
                }
            }
            _ => anyhow::bail!("Expected String answer variant"),
        }
    }

    fn input_integer(&self, _prompt: &str) -> Result<i64> {
        match self.next_answer()? {
            AnswerVariant::Int(i) => Ok(i),
            _ => anyhow::bail!("Expected Int answer variant"),
        }
    }

    fn input_integer_with_initial_text(&self, _prompt: &str, _initial_text: &str) -> Result<i64> {
        match self.next_answer()? {
            AnswerVariant::Int(i) => Ok(i),
            _ => anyhow::bail!("Expected Int answer variant"),
        }
    }

    fn confirm(&self, _prompt: &str) -> Result<bool> {
        match self.next_answer()? {
            AnswerVariant::Bool(b) => Ok(b),
            _ => anyhow::bail!("Expected Bool answer variant"),
        }
    }

    fn confirm_with_default(&self, _prompt: &str, _default: bool) -> Result<bool> {
        match self.next_answer()? {
            AnswerVariant::Bool(b) => Ok(b),
            _ => anyhow::bail!("Expected Bool answer variant"),
        }
    }

    fn select<T: ToString>(&self, _prompt: &str, _items: &[T]) -> Result<usize> {
        match self.next_answer()? {
            AnswerVariant::Int(i) => {
                if i < 0 {
                    anyhow::bail!("Expected non-negative integer for select")
                }
                Ok(i as usize)
            }
            _ => anyhow::bail!("Expected Int answer variant"),
        }
    }

    fn multi_select<T: ToString>(&self, _prompt: &str, _items: &[T]) -> Result<Vec<usize>> {
        match self.next_answer()? {
            AnswerVariant::Strings(strings) => {
                let indices: Result<Vec<usize>> = strings
                    .iter()
                    .map(|s| {
                        s.parse::<usize>()
                            .context("Failed to parse string as usize for multi_select")
                    })
                    .collect();
                indices
            }
            _ => anyhow::bail!("Expected Strings answer variant"),
        }
    }

    fn fuzzy_search_strings(&self, _items: &[String], _prompt: &str) -> Result<Option<String>> {
        match self.next_answer()? {
            AnswerVariant::String(s) => {
                if s.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(s))
                }
            }
            _ => anyhow::bail!("Expected String answer variant"),
        }
    }
}
