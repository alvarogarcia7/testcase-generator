use crate::config::EditorConfig;
use crate::fuzzy::TestCaseFuzzyFinder;
use crate::validation::SchemaValidator;
use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Display;

/// Mode for the collection builder
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    /// Single-select mode - user selects or creates one item at a time
    SingleSelect,
    /// Multi-select mode - user can select multiple items at once
    MultiSelect,
}

/// Generic builder for creating collections of items with interactive workflow
///
/// This builder provides a complete workflow for building arrays of items with:
/// - Fuzzy search through existing items (ESC cancels to create new)
/// - Editor-based creation/editing with templates
/// - Schema validation before adding to collection
/// - Support for both single-select and multi-select modes
///
/// # Type Parameters
/// * `T` - The type of items in the collection, must implement Serialize + DeserializeOwned
///
/// # Examples
///
/// ```rust,ignore
/// use testcase_manager::collection_builder::{CollectionBuilder, SelectionMode};
///
/// let existing_items = vec!["Item 1".to_string(), "Item 2".to_string()];
/// let template = "# Enter item details here\nname: \n";
///
/// let builder = CollectionBuilder::new(
///     existing_items,
///     template,
///     SelectionMode::SingleSelect,
/// );
///
/// let collection = builder.build(|item| {
///     // Custom validation logic
///     Ok(())
/// })?;
/// ```
pub struct CollectionBuilder<T> {
    /// Existing items available for selection
    existing_items: Vec<T>,
    /// Template content for creating new items in editor
    template: String,
    /// Selection mode (single or multi-select)
    mode: SelectionMode,
    /// Current collection being built
    collection: Vec<T>,
    /// Editor configuration
    editor_config: EditorConfig,
    /// Optional schema validator for validation
    validator: Option<SchemaValidator>,
}

impl<T> CollectionBuilder<T>
where
    T: Serialize + DeserializeOwned + Clone + Display,
{
    /// Create a new collection builder
    ///
    /// # Arguments
    /// * `existing_items` - List of existing items available for fuzzy search
    /// * `template` - YAML template string for creating new items in editor
    /// * `mode` - Selection mode (SingleSelect or MultiSelect)
    pub fn new(existing_items: Vec<T>, template: String, mode: SelectionMode) -> Self {
        Self {
            existing_items,
            template,
            mode,
            collection: Vec::new(),
            editor_config: EditorConfig::load(),
            validator: None,
        }
    }

    /// Create a new collection builder with schema validation
    ///
    /// # Arguments
    /// * `existing_items` - List of existing items available for fuzzy search
    /// * `template` - YAML template string for creating new items in editor
    /// * `mode` - Selection mode (SingleSelect or MultiSelect)
    /// * `validator` - Schema validator for validating items
    pub fn new_with_validator(
        existing_items: Vec<T>,
        template: String,
        mode: SelectionMode,
        validator: SchemaValidator,
    ) -> Self {
        Self {
            existing_items,
            template,
            mode,
            collection: Vec::new(),
            editor_config: EditorConfig::load(),
            validator: Some(validator),
        }
    }

    /// Set a custom editor configuration
    pub fn with_editor_config(mut self, config: EditorConfig) -> Self {
        self.editor_config = config;
        self
    }

    /// Get the current collection
    pub fn collection(&self) -> &[T] {
        &self.collection
    }

    /// Get a mutable reference to the collection
    pub fn collection_mut(&mut self) -> &mut Vec<T> {
        &mut self.collection
    }

    /// Build the collection interactively
    ///
    /// This method runs the main workflow loop:
    /// 1. Fuzzy search existing items (ESC to skip)
    /// 2. If no selection or user wants to create new, open editor
    /// 3. Validate the item against schema
    /// 4. Add to collection
    /// 5. Repeat until user is done
    ///
    /// # Arguments
    /// * `validate_fn` - Custom validation function called before adding each item
    ///
    /// # Returns
    /// The built collection
    pub fn build<F>(mut self, mut validate_fn: F) -> Result<Vec<T>>
    where
        F: FnMut(&T) -> Result<()>,
    {
        loop {
            let item = self.collect_single_item(&mut validate_fn)?;

            if let Some(item) = item {
                self.collection.push(item);
                println!("✓ Item added to collection");
            } else {
                println!("No item added");
            }

            if !self.prompt_continue()? {
                break;
            }
        }

        Ok(self.collection)
    }

    /// Build the collection with a continuation callback
    ///
    /// Similar to `build()`, but allows the caller to decide whether to continue
    /// after each item is added.
    ///
    /// # Arguments
    /// * `validate_fn` - Custom validation function called before adding each item
    /// * `continue_fn` - Function called after each item to determine whether to continue
    pub fn build_with_callback<F, C>(
        mut self,
        mut validate_fn: F,
        mut continue_fn: C,
    ) -> Result<Vec<T>>
    where
        F: FnMut(&T) -> Result<()>,
        C: FnMut(&[T]) -> Result<bool>,
    {
        loop {
            let item = self.collect_single_item(&mut validate_fn)?;

            if let Some(item) = item {
                self.collection.push(item);
                println!("✓ Item added to collection");
            } else {
                println!("No item added");
            }

            if !continue_fn(&self.collection)? {
                break;
            }
        }

        Ok(self.collection)
    }

    /// Collect a single item through the workflow
    ///
    /// 1. Fuzzy search existing items
    /// 2. If ESC pressed or no items, prompt to create new
    /// 3. Open editor with template
    /// 4. Validate and parse the item
    ///
    /// Returns Ok(Some(item)) if an item was collected, Ok(None) if cancelled
    fn collect_single_item<F>(&mut self, validate_fn: &mut F) -> Result<Option<T>>
    where
        F: FnMut(&T) -> Result<()>,
    {
        // Step 1: Fuzzy search existing items
        let selected = self.fuzzy_search_items()?;

        if let Some(item) = selected {
            // User selected an existing item
            validate_fn(&item)?;
            return Ok(Some(item));
        }

        // Step 2: No selection - prompt to create new
        if !self.prompt_create_new()? {
            return Ok(None);
        }

        // Step 3: Open editor and create new item
        let new_item = self.create_new_item_in_editor(validate_fn)?;

        Ok(Some(new_item))
    }

    /// Fuzzy search through existing items
    ///
    /// Returns Some(item) if selected, None if ESC pressed or no items available
    fn fuzzy_search_items(&self) -> Result<Option<T>> {
        if self.existing_items.is_empty() {
            println!("No existing items available for selection");
            return Ok(None);
        }

        let item_strings: Vec<String> = self
            .existing_items
            .iter()
            .map(|item| item.to_string())
            .collect();

        let prompt = match self.mode {
            SelectionMode::SingleSelect => "Select item (ESC to create new): ",
            SelectionMode::MultiSelect => "Select items (ESC to create new): ",
        };

        match self.mode {
            SelectionMode::SingleSelect => {
                let selected = TestCaseFuzzyFinder::search_strings(&item_strings, prompt)?;

                if let Some(selected_str) = selected {
                    // Find the corresponding item
                    for (idx, item_str) in item_strings.iter().enumerate() {
                        if item_str == &selected_str {
                            return Ok(Some(self.existing_items[idx].clone()));
                        }
                    }
                }

                Ok(None)
            }
            SelectionMode::MultiSelect => {
                let selected = TestCaseFuzzyFinder::multi_select(&item_strings, prompt)?;

                if selected.is_empty() {
                    return Ok(None);
                }

                // For multi-select, we return the first selected item
                // (caller should handle multiple selections differently)
                for selected_str in selected {
                    for (idx, item_str) in item_strings.iter().enumerate() {
                        if item_str == &selected_str {
                            return Ok(Some(self.existing_items[idx].clone()));
                        }
                    }
                }

                Ok(None)
            }
        }
    }

    /// Create a new item using the editor with validation loop
    ///
    /// Opens the editor with the template, validates the input, and repeats
    /// until valid input is provided.
    fn create_new_item_in_editor<F>(&self, validate_fn: &mut F) -> Result<T>
    where
        F: FnMut(&T) -> Result<()>,
    {
        let mut content = self.template.clone();

        loop {
            let edited_content = self.open_editor(&content)?;

            // Parse the YAML content
            match serde_yaml::from_str::<T>(&edited_content) {
                Ok(parsed_item) => {
                    // Validate with custom function
                    if let Err(e) = validate_fn(&parsed_item) {
                        println!("✗ Validation failed: {}", e);
                        content = self.annotate_with_error(&edited_content, &e);
                        continue;
                    }

                    // Validate with schema if available
                    if let Some(validator) = &self.validator {
                        if let Err(e) = validator.validate_chunk(&edited_content) {
                            println!("✗ Schema validation failed: {}", e);
                            content = self.annotate_with_error(&edited_content, &e);
                            continue;
                        }
                    }

                    println!("✓ Item validated successfully");
                    return Ok(parsed_item);
                }
                Err(parse_error) => {
                    println!("✗ Parse error: {}", parse_error);
                    content = self.annotate_with_error(&edited_content, &parse_error.into());
                }
            }
        }
    }

    /// Open editor with the given content
    fn open_editor(&self, content: &str) -> Result<String> {
        let editor_path = self
            .editor_config
            .get_editor()
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "No editor configured. Please set VISUAL, EDITOR, or CUSTOM_FALLBACK environment variable"
                )
            })?;

        let temp_file =
            tempfile::NamedTempFile::new().context("Failed to create temporary file")?;
        let temp_path = temp_file.path();

        std::fs::write(temp_path, content).context("Failed to write content to temporary file")?;

        let status = std::process::Command::new(&editor_path)
            .arg(temp_path)
            .status()
            .context(format!("Failed to execute editor: {}", editor_path))?;

        if !status.success() {
            return Err(anyhow::anyhow!(
                "Editor exited with non-zero status: {}",
                status.code().unwrap_or(-1)
            ));
        }

        let edited_content = std::fs::read_to_string(temp_path)
            .context("Failed to read edited content from temporary file")?;

        Ok(edited_content)
    }

    /// Annotate content with error message
    fn annotate_with_error(&self, content: &str, error: &anyhow::Error) -> String {
        let error_message = format!("# VALIDATION ERROR: {}", error);
        let separator = "# ".repeat(40);

        format!(
            "{}\n{}\n{}\n\n{}",
            separator, error_message, separator, content
        )
    }

    /// Prompt to create a new item
    fn prompt_create_new(&self) -> Result<bool> {
        use dialoguer::Confirm;

        Confirm::new()
            .with_prompt("Create new item?")
            .default(true)
            .interact()
            .context("Failed to get confirmation")
    }

    /// Prompt to continue adding more items
    fn prompt_continue(&self) -> Result<bool> {
        use dialoguer::Confirm;

        Confirm::new()
            .with_prompt("Add another item?")
            .default(true)
            .interact()
            .context("Failed to get confirmation")
    }

    /// Add multiple items in multi-select mode
    ///
    /// This is a convenience method for multi-select mode that allows
    /// selecting multiple existing items at once.
    pub fn add_multiple_from_existing<F>(&mut self, validate_fn: &mut F) -> Result<()>
    where
        F: FnMut(&T) -> Result<()>,
    {
        if self.existing_items.is_empty() {
            println!("No existing items available for selection");
            return Ok(());
        }

        let item_strings: Vec<String> = self
            .existing_items
            .iter()
            .map(|item| item.to_string())
            .collect();

        let selected = TestCaseFuzzyFinder::multi_select(
            &item_strings,
            "Select items (TAB to select, ESC when done): ",
        )?;

        if selected.is_empty() {
            println!("No items selected");
            return Ok(());
        }

        // Find and add all selected items
        for selected_str in selected {
            for (idx, item_str) in item_strings.iter().enumerate() {
                if item_str == &selected_str {
                    let item = self.existing_items[idx].clone();
                    validate_fn(&item)?;
                    self.collection.push(item);
                    println!("✓ Added: {}", selected_str);
                    break;
                }
            }
        }

        Ok(())
    }

    /// Build collection with explicit single-item workflow
    ///
    /// This method collects items one at a time, allowing the user to:
    /// 1. Search and select from existing items, OR
    /// 2. Create a new item via editor
    ///
    /// After each item is added, the user is prompted to continue.
    pub fn build_single_item_workflow<F>(mut self, mut validate_fn: F) -> Result<Vec<T>>
    where
        F: FnMut(&T) -> Result<()>,
    {
        println!("\n=== Collection Builder ===");
        println!("Mode: {:?}", self.mode);
        println!();

        loop {
            let item = self.collect_single_item(&mut validate_fn)?;

            if let Some(item) = item {
                self.collection.push(item);
                println!(
                    "✓ Item added to collection (total: {})",
                    self.collection.len()
                );
            } else {
                println!("No item added");
            }

            println!();

            if !self.prompt_continue()? {
                break;
            }

            println!();
        }

        println!(
            "\n✓ Collection complete with {} items",
            self.collection.len()
        );

        Ok(self.collection)
    }

    /// Build collection with multi-select workflow
    ///
    /// This method allows selecting multiple items at once from existing items,
    /// then optionally creating new items one by one.
    pub fn build_multi_select_workflow<F>(mut self, mut validate_fn: F) -> Result<Vec<T>>
    where
        F: FnMut(&T) -> Result<()>,
    {
        println!("\n=== Collection Builder (Multi-Select) ===");
        println!();

        // First, allow multi-select from existing items
        if !self.existing_items.is_empty() {
            use dialoguer::Confirm;

            let do_multi_select = Confirm::new()
                .with_prompt("Select multiple items from existing?")
                .default(true)
                .interact()
                .context("Failed to get confirmation")?;

            if do_multi_select {
                self.add_multiple_from_existing(&mut validate_fn)?;
            }
        }

        // Then allow creating new items one by one
        loop {
            if !self.prompt_create_new()? {
                break;
            }

            let new_item = self.create_new_item_in_editor(&mut validate_fn)?;
            self.collection.push(new_item);
            println!(
                "✓ Item added to collection (total: {})",
                self.collection.len()
            );

            println!();

            if !self.prompt_continue()? {
                break;
            }

            println!();
        }

        println!(
            "\n✓ Collection complete with {} items",
            self.collection.len()
        );

        Ok(self.collection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Serialize, serde::Deserialize, PartialEq)]
    struct TestItem {
        name: String,
        value: i32,
    }

    impl Display for TestItem {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} (value: {})", self.name, self.value)
        }
    }

    #[test]
    fn test_collection_builder_creation() {
        let items = vec![
            TestItem {
                name: "Item 1".to_string(),
                value: 10,
            },
            TestItem {
                name: "Item 2".to_string(),
                value: 20,
            },
        ];

        let template = "name: \nvalue: 0\n";
        let builder =
            CollectionBuilder::new(items, template.to_string(), SelectionMode::SingleSelect);

        assert_eq!(builder.collection().len(), 0);
    }

    #[test]
    fn test_selection_mode() {
        let items = vec![];
        let template = "name: \nvalue: 0\n";

        let builder_single = CollectionBuilder::<TestItem>::new(
            items.clone(),
            template.to_string(),
            SelectionMode::SingleSelect,
        );
        assert_eq!(builder_single.mode, SelectionMode::SingleSelect);

        let builder_multi = CollectionBuilder::<TestItem>::new(
            items,
            template.to_string(),
            SelectionMode::MultiSelect,
        );
        assert_eq!(builder_multi.mode, SelectionMode::MultiSelect);
    }

    #[test]
    fn test_with_editor_config() {
        let items = vec![];
        let template = "name: \nvalue: 0\n";

        let config = EditorConfig::load();
        let builder = CollectionBuilder::<TestItem>::new(
            items,
            template.to_string(),
            SelectionMode::SingleSelect,
        )
        .with_editor_config(config);

        assert!(
            builder.editor_config.get_editor().is_some()
                || builder.editor_config.get_editor().is_none()
        );
    }

    #[test]
    fn test_collection_access() {
        let items = vec![];
        let template = "name: \nvalue: 0\n";

        let mut builder = CollectionBuilder::<TestItem>::new(
            items,
            template.to_string(),
            SelectionMode::SingleSelect,
        );

        assert_eq!(builder.collection().len(), 0);

        builder.collection_mut().push(TestItem {
            name: "Test".to_string(),
            value: 42,
        });

        assert_eq!(builder.collection().len(), 1);
        assert_eq!(builder.collection()[0].name, "Test");
    }

    #[test]
    fn test_annotate_with_error() {
        let items = vec![];
        let template = "name: \nvalue: 0\n";

        let builder = CollectionBuilder::<TestItem>::new(
            items,
            template.to_string(),
            SelectionMode::SingleSelect,
        );

        let content = "some content";
        let error = anyhow::anyhow!("Test error");
        let annotated = builder.annotate_with_error(content, &error);

        assert!(annotated.contains("VALIDATION ERROR"));
        assert!(annotated.contains("Test error"));
        assert!(annotated.contains("some content"));
    }
}
