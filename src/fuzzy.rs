use crate::models::TestCase;
use crate::oracle::Oracle;
use anyhow::Result;
use skim::prelude::*;
use std::io::{self, Cursor, Write};

/// Fuzzy finder for test cases
pub struct TestCaseFuzzyFinder;

impl TestCaseFuzzyFinder {
    /// Check if we're in a TTY environment
    fn is_tty() -> bool {
        use std::io::IsTerminal;
        io::stdin().is_terminal()
    }

    /// Fallback numbered selection when TTY is not available
    fn numbered_selection(items: &[String], prompt: &str) -> Result<Option<String>> {
        if items.is_empty() {
            return Ok(None);
        }

        println!("\n⚠ TTY not detected (e.g., VS Code debug console)");
        println!("Fuzzy search unavailable - using numbered selection instead\n");
        println!("{}", prompt);
        println!();

        for (idx, item) in items.iter().enumerate() {
            println!("{}) {}", idx + 1, item);
        }

        println!();

        loop {
            print!("Enter number (1-{}) or 0 to cancel: ", items.len());
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let input = input.trim();

            match input.parse::<usize>() {
                Ok(0) => {
                    println!("Selection cancelled.\n");
                    return Ok(None);
                }
                Ok(num) if num > 0 && num <= items.len() => {
                    return Ok(Some(items[num - 1].clone()));
                }
                _ => {
                    println!(
                        "❌ Invalid input. Please enter a number between 0 and {}\n",
                        items.len()
                    );
                }
            }
        }
    }

    /// Multi-select with numbered input when TTY is not available
    fn numbered_multi_selection(items: &[String], prompt: &str) -> Result<Vec<String>> {
        if items.is_empty() {
            return Ok(Vec::new());
        }

        println!("\n⚠ TTY not detected (e.g., VS Code debug console)");
        println!("Fuzzy search unavailable - using numbered multi-selection instead\n");
        println!("{}", prompt);
        println!();

        for (idx, item) in items.iter().enumerate() {
            println!("{}) {}", idx + 1, item);
        }

        println!();
        println!("Enter numbers separated by spaces (e.g., '1 3 5'), or 0/empty to finish:");

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
                    Ok(num) if num > 0 && num <= items.len() => {
                        if !temp_selected.contains(&items[num - 1]) {
                            temp_selected.push(items[num - 1].clone());
                        }
                    }
                    _ => {
                        println!(
                            "❌ Invalid input: '{}'. Please enter numbers between 1 and {}\n",
                            num_str,
                            items.len()
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

    /// Fuzzy search through test cases
    pub fn search(test_cases: &[TestCase]) -> Result<Option<TestCase>> {
        if test_cases.is_empty() {
            return Ok(None);
        }

        let items: Vec<String> = test_cases
            .iter()
            .map(|tc| {
                format!(
                    "{} - {} (Item: {}, TC: {})",
                    tc.id, /*tc.requirement*/ "TODO AGB", tc.item, tc.tc
                )
            })
            .collect();

        if !Self::is_tty() {
            let selected = Self::numbered_selection(&items, "Select test case:")?;
            if let Some(selected_text) = selected {
                let id = selected_text.split(" - ").next().unwrap_or("");
                let test_case = test_cases.iter().find(|tc| tc.id == id).cloned();
                return Ok(test_case);
            }
            return Ok(None);
        }

        let item_reader = SkimItemReader::default();
        let items = item_reader.of_bufread(Cursor::new(items.join("\n")));

        let options = SkimOptionsBuilder::default()
            .height(Some("50%"))
            .multi(false)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build skim options: {}", e))?;

        let selected = Skim::run_with(&options, Some(items)).and_then(|output| {
            if output.is_abort {
                None
            } else {
                output
                    .selected_items
                    .first()
                    .map(|item| item.output().to_string())
            }
        });

        if let Some(selected_text) = selected {
            let id = selected_text.split(" - ").next().unwrap_or("");
            let test_case = test_cases.iter().find(|tc| tc.id == id).cloned();
            Ok(test_case)
        } else {
            Ok(None)
        }
    }

    /// Fuzzy search through a list of strings (delegates to TtyCliOracle)
    pub fn search_strings(items: &[String], prompt: &str) -> Result<Option<String>> {
        use crate::oracle::TtyCliOracle;
        let oracle = TtyCliOracle::new();
        oracle.fuzzy_search_strings(items, prompt)
    }

    /// Fuzzy search with TTY fallback (delegates to MenuCliOracle for non-TTY handling)
    pub fn search_strings_with_fallback(items: &[String], prompt: &str) -> Result<Option<String>> {
        if !Self::is_tty() {
            return Self::numbered_selection(items, prompt);
        }

        let item_reader = SkimItemReader::default();
        let skim_items = item_reader.of_bufread(Cursor::new(items.join("\n")));

        let options = SkimOptionsBuilder::default()
            .multi(false)
            .prompt(Some(prompt))
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build skim options: {}", e))?;

        let selected = Skim::run_with(&options, Some(skim_items)).and_then(|output| {
            if output.is_abort {
                None
            } else {
                output
                    .selected_items
                    .first()
                    .map(|item| item.output().to_string())
            }
        });

        Ok(selected)
    }

    /// Multi-select fuzzy search (delegates to TtyCliOracle)
    pub fn multi_select(items: &[String], prompt: &str) -> Result<Vec<String>> {
        use crate::oracle::TtyCliOracle;
        let oracle = TtyCliOracle::new();
        oracle.fuzzy_multi_select(items, prompt)
    }

    /// Multi-select fuzzy search with TTY fallback (delegates to MenuCliOracle for non-TTY handling)
    pub fn multi_select_with_fallback(items: &[String], prompt: &str) -> Result<Vec<String>> {
        if !Self::is_tty() {
            return Self::numbered_multi_selection(items, prompt);
        }

        let item_reader = SkimItemReader::default();
        let skim_items = item_reader.of_bufread(Cursor::new(items.join("\n")));

        let options = SkimOptionsBuilder::default()
            .height(Some("50%"))
            .multi(true)
            .prompt(Some(prompt))
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build skim options: {}", e))?;

        let selected = Skim::run_with(&options, Some(skim_items))
            .map(|output| {
                if output.is_abort {
                    Vec::new()
                } else {
                    output
                        .selected_items
                        .iter()
                        .map(|item| item.output().to_string())
                        .collect()
                }
            })
            .unwrap_or_default();

        Ok(selected)
    }
}
