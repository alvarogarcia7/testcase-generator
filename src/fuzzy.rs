use crate::models::TestCase;
use anyhow::Result;
use skim::prelude::*;
use std::io::Cursor;

/// Fuzzy finder for test cases
pub struct TestCaseFuzzyFinder;

impl TestCaseFuzzyFinder {
    /// Fuzzy search through test cases
    pub fn search(test_cases: &[TestCase]) -> Result<Option<TestCase>> {
        if test_cases.is_empty() {
            return Ok(None);
        }

        let items: Vec<String> = test_cases
            .iter()
            .map(|tc| format!("{} - {} ({})", tc.id, tc.title, tc.status as u8))
            .collect();

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

    /// Fuzzy search through a list of strings
    pub fn search_strings(items: &[String], prompt: &str) -> Result<Option<String>> {
        if items.is_empty() {
            return Ok(None);
        }

        let item_reader = SkimItemReader::default();
        let skim_items = item_reader.of_bufread(Cursor::new(items.join("\n")));

        let options = SkimOptionsBuilder::default()
            .height(Some("50%"))
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

    /// Multi-select fuzzy search
    pub fn multi_select(items: &[String], prompt: &str) -> Result<Vec<String>> {
        if items.is_empty() {
            return Ok(Vec::new());
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
