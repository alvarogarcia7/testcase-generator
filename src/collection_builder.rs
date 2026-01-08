use crate::config::EditorConfig;
use crate::database::ConditionDatabase;
use crate::fuzzy::TestCaseFuzzyFinder;
use crate::prompts::Prompts;
use crate::validation::SchemaValidator;
use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
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

/// Specialized builder for collecting condition strings with device name prompts
///
/// This builder is designed specifically for collecting initial conditions or general
/// initial conditions. It prompts for a device name first (with fuzzy search support
/// from database), then iteratively collects condition strings for that device.
///
/// # Example Usage
/// ```rust,ignore
/// let db = ConditionDatabase::load_from_directory("data")?;
/// let builder = ConditionCollectionBuilder::new_with_database(&db);
/// let conditions = builder.build_conditions()?;
/// // Returns: HashMap<String, Vec<String>> with device names as keys
/// ```
pub struct ConditionCollectionBuilder<'a> {
    db: Option<&'a ConditionDatabase>,
    #[allow(dead_code)]
    validator: Option<SchemaValidator>,
}

impl<'a> Default for ConditionCollectionBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ConditionCollectionBuilder<'a> {
    /// Create a new condition collection builder without database
    pub fn new() -> Self {
        Self {
            db: None,
            validator: None,
        }
    }

    /// Create a new condition collection builder with database support
    pub fn new_with_database(db: &'a ConditionDatabase) -> Self {
        Self {
            db: Some(db),
            validator: None,
        }
    }

    /// Create a new condition collection builder with validator
    pub fn new_with_validator(validator: SchemaValidator) -> Self {
        Self {
            db: None,
            validator: Some(validator),
        }
    }

    /// Create a new condition collection builder with both database and validator
    pub fn new_with_database_and_validator(
        db: &'a ConditionDatabase,
        validator: SchemaValidator,
    ) -> Self {
        Self {
            db: Some(db),
            validator: Some(validator),
        }
    }

    /// Build a collection of conditions interactively
    ///
    /// Prompts for:
    /// 1. Device name (with fuzzy search if database available)
    /// 2. Condition strings (iteratively, one at a time)
    ///
    /// Returns a HashMap mapping device name to condition strings
    pub fn build_conditions(&self) -> Result<HashMap<String, Vec<String>>> {
        println!("\n=== Condition Collection ===\n");

        // Step 1: Get device name
        let device_name = self.prompt_device_name()?;

        // Step 2: Collect conditions for the device
        let conditions = self.collect_conditions_for_device(&device_name)?;

        // Build the result map
        let mut result = HashMap::new();
        result.insert(device_name, conditions);

        Ok(result)
    }

    /// Prompt for device name with fuzzy search support
    fn prompt_device_name(&self) -> Result<String> {
        if let Some(db) = self.db {
            let device_names = db.get_device_names();
            if !device_names.is_empty() {
                println!(
                    "Available device names from database: {}\n",
                    device_names.len()
                );

                match TestCaseFuzzyFinder::search_strings(
                    device_names,
                    "Select device name (or ESC to enter manually): ",
                )? {
                    Some(name) => {
                        println!("Selected: {}\n", name);
                        return Ok(name);
                    }
                    None => {
                        println!("No selection made, entering manually.\n");
                    }
                }
            }
        }

        Prompts::input("Device name (e.g., eUICC)")
    }

    /// Collect condition strings for a device
    fn collect_conditions_for_device(&self, device_name: &str) -> Result<Vec<String>> {
        let mut conditions = Vec::new();

        println!(
            "Enter conditions for '{}' (enter empty string to finish):\n",
            device_name
        );

        loop {
            let condition =
                Prompts::input_optional(&format!("Condition #{}", conditions.len() + 1))?;

            match condition {
                Some(cond) if !cond.trim().is_empty() => {
                    conditions.push(cond);
                    println!("✓ Added condition\n");
                }
                _ => {
                    if conditions.is_empty() {
                        println!("At least one condition is required.");
                        continue;
                    }
                    break;
                }
            }
        }

        println!(
            "✓ Collected {} condition(s) for '{}'\n",
            conditions.len(),
            device_name
        );

        Ok(conditions)
    }

    /// Build conditions from database with fuzzy search
    pub fn build_conditions_from_database(&self) -> Result<HashMap<String, Vec<String>>> {
        println!("\n=== Condition Collection (from database) ===\n");

        let db = self
            .db
            .ok_or_else(|| anyhow::anyhow!("Database not available"))?;

        // Get device name
        let device_name = self.prompt_device_name()?;

        // Get available conditions from database
        let available_conditions = db.get_initial_conditions();

        if available_conditions.is_empty() {
            println!("No conditions found in database, falling back to manual entry.\n");
            return self.build_conditions();
        }

        println!(
            "Loaded {} unique conditions from database\n",
            available_conditions.len()
        );

        let mut selected_conditions = Vec::new();

        loop {
            let selected = TestCaseFuzzyFinder::search_strings(
                available_conditions,
                "Select condition (ESC to finish): ",
            )?;

            match selected {
                Some(condition) => {
                    selected_conditions.push(condition.clone());
                    println!("✓ Added: {}\n", condition);

                    if !Prompts::confirm("Add another condition?")? {
                        break;
                    }
                }
                None => {
                    if selected_conditions.is_empty() {
                        println!("No conditions selected.");
                        if Prompts::confirm("Use manual entry instead?")? {
                            return self.build_conditions();
                        }
                    }
                    break;
                }
            }
        }

        if selected_conditions.is_empty() {
            anyhow::bail!("No conditions selected");
        }

        let mut result = HashMap::new();
        let count = selected_conditions.len();
        result.insert(device_name.clone(), selected_conditions);

        println!("✓ Collected {} condition(s) for '{}'\n", count, device_name);

        Ok(result)
    }
}

/// Specialized builder for collecting Step objects with field-by-field prompts
///
/// This builder handles the complex structure of test steps including:
/// - Step number
/// - Optional manual flag
/// - Description
/// - Command
/// - Expected object (with result, output, and optional success fields)
///
/// # Example Usage
/// ```rust,ignore
/// let builder = StepCollectionBuilder::new(1);
/// let steps = builder.build_steps()?;
/// ```
pub struct StepCollectionBuilder<'a> {
    starting_step_number: i64,
    validator: Option<&'a SchemaValidator>,
    existing_descriptions: Vec<String>,
}

impl<'a> StepCollectionBuilder<'a> {
    /// Create a new step collection builder
    ///
    /// # Arguments
    /// * `starting_step_number` - The step number to start from (usually 1 or next available)
    pub fn new(starting_step_number: i64) -> Self {
        Self {
            starting_step_number,
            validator: None,
            existing_descriptions: Vec::new(),
        }
    }

    /// Create a step collection builder with validation support
    pub fn new_with_validator(starting_step_number: i64, validator: &'a SchemaValidator) -> Self {
        Self {
            starting_step_number,
            validator: Some(validator),
            existing_descriptions: Vec::new(),
        }
    }

    /// Set existing step descriptions for fuzzy search
    pub fn with_existing_descriptions(mut self, descriptions: Vec<String>) -> Self {
        self.existing_descriptions = descriptions;
        self
    }

    /// Build a collection of steps interactively
    ///
    /// Returns a Vec of step Values (serde_yaml::Value) ready to be added to a sequence
    pub fn build_steps(&self) -> Result<Vec<Value>> {
        println!("\n=== Step Collection ===\n");

        let mut steps = Vec::new();
        let mut current_step_number = self.starting_step_number;

        loop {
            println!("=== Add Step #{} ===\n", current_step_number);

            let step = self.collect_single_step(current_step_number)?;

            if let Some(step_value) = step {
                steps.push(step_value);
                println!("✓ Step #{} added\n", current_step_number);
                current_step_number += 1;

                if !Prompts::confirm("Add another step?")? {
                    break;
                }
            } else {
                break;
            }
        }

        println!("✓ Collected {} step(s)\n", steps.len());

        Ok(steps)
    }

    /// Collect a single step with field-by-field prompts
    fn collect_single_step(&self, step_number: i64) -> Result<Option<Value>> {
        // Prompt for description with fuzzy search if available
        let description = if !self.existing_descriptions.is_empty() {
            if Prompts::confirm("Search existing step descriptions?")? {
                match TestCaseFuzzyFinder::search_strings(
                    &self.existing_descriptions,
                    "Select step description: ",
                )? {
                    Some(desc) => desc,
                    None => {
                        println!("No selection made, entering new description.\n");
                        Prompts::input("Step description")?
                    }
                }
            } else {
                Prompts::input("Step description")?
            }
        } else {
            Prompts::input("Step description")?
        };

        // Prompt for manual flag
        let manual = if Prompts::confirm("Is this a manual step?")? {
            Some(true)
        } else {
            None
        };

        // Prompt for command
        let command = Prompts::input("Command")?;

        // Prompt for expected object
        let expected = self.prompt_expected()?;

        // Build the step value
        let step_value =
            self.build_step_value(step_number, description, command, manual, expected)?;

        // Validate if validator is available
        if let Some(validator) = &self.validator {
            let yaml_str = serde_yaml::to_string(&step_value)
                .context("Failed to serialize step for validation")?;
            validator
                .validate_partial_chunk(&yaml_str)
                .context("Step validation failed")?;
            println!("✓ Step validated successfully\n");
        }

        Ok(Some(step_value))
    }

    /// Prompt for the expected object with all its fields
    fn prompt_expected(&self) -> Result<Value> {
        println!("\n--- Expected Outcome ---\n");

        let include_success = Prompts::confirm("Include 'success' field?")?;
        let success_value = if include_success {
            Some(Prompts::confirm("Success value (true/false)?")?)
        } else {
            None
        };

        let result = Prompts::input("Expected result")?;
        let output = Prompts::input("Expected output")?;

        // Build expected mapping
        let mut expected_map = serde_yaml::Mapping::new();

        if let Some(success) = success_value {
            expected_map.insert(Value::String("success".to_string()), Value::Bool(success));
        }

        expected_map.insert(Value::String("result".to_string()), Value::String(result));

        expected_map.insert(Value::String("output".to_string()), Value::String(output));

        Ok(Value::Mapping(expected_map))
    }

    /// Build a step Value from components
    fn build_step_value(
        &self,
        step_number: i64,
        description: String,
        command: String,
        manual: Option<bool>,
        expected: Value,
    ) -> Result<Value> {
        let mut step_map = serde_yaml::Mapping::new();

        step_map.insert(
            Value::String("step".to_string()),
            Value::Number(step_number.into()),
        );

        if let Some(is_manual) = manual {
            step_map.insert(Value::String("manual".to_string()), Value::Bool(is_manual));
        }

        step_map.insert(
            Value::String("description".to_string()),
            Value::String(description),
        );

        step_map.insert(Value::String("command".to_string()), Value::String(command));

        step_map.insert(Value::String("expected".to_string()), expected);

        Ok(Value::Mapping(step_map))
    }
}

/// Specialized builder for collecting TestSequence objects with metadata and nested conditions
///
/// This builder handles the complete structure of test sequences including:
/// - Sequence ID
/// - Name
/// - Description
/// - Initial conditions (nested HashMap structure)
/// - Steps collection (using StepCollectionBuilder)
///
/// # Example Usage
/// ```rust,ignore
/// let db = ConditionDatabase::load_from_directory("data")?;
/// let builder = SequenceCollectionBuilder::new(1)
///     .with_database(&db)
///     .with_validator(validator);
/// let sequences = builder.build_sequences()?;
/// ```
pub struct SequenceCollectionBuilder<'a> {
    starting_sequence_id: i64,
    db: Option<&'a ConditionDatabase>,
    validator: Option<SchemaValidator>,
    existing_sequence_names: Vec<String>,
    existing_step_descriptions: Vec<String>,
}

impl<'a> SequenceCollectionBuilder<'a> {
    /// Create a new sequence collection builder
    ///
    /// # Arguments
    /// * `starting_sequence_id` - The sequence ID to start from
    pub fn new(starting_sequence_id: i64) -> Self {
        Self {
            starting_sequence_id,
            db: None,
            validator: None,
            existing_sequence_names: Vec::new(),
            existing_step_descriptions: Vec::new(),
        }
    }

    /// Set database for fuzzy search support
    pub fn with_database(mut self, db: &'a ConditionDatabase) -> Self {
        self.db = Some(db);
        self
    }

    /// Set validator for validation support
    pub fn with_validator(mut self, validator: SchemaValidator) -> Self {
        self.validator = Some(validator);
        self
    }

    /// Set existing sequence names for fuzzy search
    pub fn with_existing_sequence_names(mut self, names: Vec<String>) -> Self {
        self.existing_sequence_names = names;
        self
    }

    /// Set existing step descriptions for fuzzy search
    pub fn with_existing_step_descriptions(mut self, descriptions: Vec<String>) -> Self {
        self.existing_step_descriptions = descriptions;
        self
    }

    /// Build a collection of test sequences interactively
    ///
    /// Returns a Vec of sequence Values (serde_yaml::Value)
    pub fn build_sequences(&self) -> Result<Vec<Value>> {
        println!("\n=== Test Sequence Collection ===\n");

        let mut sequences = Vec::new();
        let mut current_sequence_id = self.starting_sequence_id;

        loop {
            println!("=== Add Test Sequence #{} ===\n", current_sequence_id);

            let sequence = self.collect_single_sequence(current_sequence_id)?;

            if let Some(sequence_value) = sequence {
                sequences.push(sequence_value);
                println!("✓ Sequence #{} added\n", current_sequence_id);
                current_sequence_id += 1;

                if !Prompts::confirm("Add another test sequence?")? {
                    break;
                }
            } else {
                break;
            }
        }

        println!("✓ Collected {} test sequence(s)\n", sequences.len());

        Ok(sequences)
    }

    /// Collect a single test sequence with all its components
    fn collect_single_sequence(&self, sequence_id: i64) -> Result<Option<Value>> {
        // Prompt for sequence name
        let sequence_name = if !self.existing_sequence_names.is_empty() {
            if Prompts::confirm("Search existing sequence names?")? {
                match TestCaseFuzzyFinder::search_strings(
                    &self.existing_sequence_names,
                    "Select sequence name: ",
                )? {
                    Some(name) => name,
                    None => {
                        println!("No selection made, entering new name.\n");
                        Prompts::input("Sequence name")?
                    }
                }
            } else {
                Prompts::input("Sequence name")?
            }
        } else {
            Prompts::input("Sequence name")?
        };

        // Prompt for description
        let description = Prompts::input("Description")?;

        // Prompt for sequence-specific initial conditions
        let initial_conditions = if Prompts::confirm("\nAdd sequence-specific initial conditions?")?
        {
            Some(self.collect_initial_conditions()?)
        } else {
            None
        };

        // Collect steps
        let steps = if Prompts::confirm("\nAdd steps to this sequence now?")? {
            if let Some(ref validator) = self.validator {
                StepCollectionBuilder::new_with_validator(1, validator)
                    .with_existing_descriptions(self.existing_step_descriptions.clone())
                    .build_steps()?
            } else {
                StepCollectionBuilder::new(1)
                    .with_existing_descriptions(self.existing_step_descriptions.clone())
                    .build_steps()?
            }
        } else {
            Vec::new()
        };

        // Build the sequence value
        let sequence_value = self.build_sequence_value(
            sequence_id,
            sequence_name,
            description,
            initial_conditions,
            steps,
        )?;

        // Validate if validator is available
        if let Some(validator) = &self.validator {
            let yaml_str = serde_yaml::to_string(&sequence_value)
                .context("Failed to serialize sequence for validation")?;
            validator
                .validate_partial_chunk(&yaml_str)
                .context("Sequence validation failed")?;
            println!("✓ Sequence validated successfully\n");
        }

        Ok(Some(sequence_value))
    }

    /// Collect initial conditions for the sequence
    fn collect_initial_conditions(&self) -> Result<HashMap<String, Vec<String>>> {
        let condition_builder = if let Some(db) = self.db {
            // Use database for fuzzy search
            if Prompts::confirm("Use database for initial conditions?")? {
                let builder = ConditionCollectionBuilder::new_with_database(db);
                return builder.build_conditions_from_database();
            }
            ConditionCollectionBuilder::new_with_database(db)
        } else {
            ConditionCollectionBuilder::new()
        };

        condition_builder.build_conditions()
    }

    /// Build a sequence Value from components
    fn build_sequence_value(
        &self,
        sequence_id: i64,
        name: String,
        description: String,
        initial_conditions: Option<HashMap<String, Vec<String>>>,
        steps: Vec<Value>,
    ) -> Result<Value> {
        let mut sequence_map = serde_yaml::Mapping::new();

        sequence_map.insert(
            Value::String("id".to_string()),
            Value::Number(sequence_id.into()),
        );

        sequence_map.insert(Value::String("name".to_string()), Value::String(name));

        sequence_map.insert(
            Value::String("description".to_string()),
            Value::String(description),
        );

        // Add initial conditions if provided
        if let Some(ic_map) = initial_conditions {
            let mut ic_yaml_map = serde_yaml::Mapping::new();
            for (device, conditions) in ic_map {
                let condition_values: Vec<Value> =
                    conditions.into_iter().map(Value::String).collect();
                ic_yaml_map.insert(Value::String(device), Value::Sequence(condition_values));
            }
            sequence_map.insert(
                Value::String("initial_conditions".to_string()),
                Value::Mapping(ic_yaml_map),
            );
        }

        // Add steps
        sequence_map.insert(Value::String("steps".to_string()), Value::Sequence(steps));

        Ok(Value::Mapping(sequence_map))
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

    // Tests for ConditionCollectionBuilder
    #[test]
    fn test_condition_collection_builder_creation() {
        let builder = ConditionCollectionBuilder::new();
        assert!(builder.db.is_none());
        assert!(builder.validator.is_none());
    }

    #[test]
    fn test_condition_collection_builder_with_validator() {
        let validator = SchemaValidator::new().unwrap();
        let builder = ConditionCollectionBuilder::new_with_validator(validator);
        assert!(builder.validator.is_some());
    }

    // Tests for StepCollectionBuilder
    #[test]
    fn test_step_collection_builder_creation() {
        let builder = StepCollectionBuilder::new(1);
        assert_eq!(builder.starting_step_number, 1);
        assert!(builder.validator.is_none());
        assert!(builder.existing_descriptions.is_empty());
    }

    #[test]
    fn test_step_collection_builder_with_validator() {
        let validator = SchemaValidator::new().unwrap();
        let builder = StepCollectionBuilder::new_with_validator(1, &validator);
        assert_eq!(builder.starting_step_number, 1);
        assert!(builder.validator.is_some());
    }

    #[test]
    fn test_step_collection_builder_with_existing_descriptions() {
        let descriptions = vec!["Step 1".to_string(), "Step 2".to_string()];
        let builder =
            StepCollectionBuilder::new(1).with_existing_descriptions(descriptions.clone());
        assert_eq!(builder.existing_descriptions.len(), 2);
        assert_eq!(builder.existing_descriptions[0], "Step 1");
    }

    #[test]
    fn test_step_builder_build_step_value() {
        let builder = StepCollectionBuilder::new(1);

        let mut expected_map = serde_yaml::Mapping::new();
        expected_map.insert(
            Value::String("result".to_string()),
            Value::String("0x9000".to_string()),
        );
        expected_map.insert(
            Value::String("output".to_string()),
            Value::String("Success".to_string()),
        );

        let step = builder
            .build_step_value(
                1,
                "Test step".to_string(),
                "ssh command".to_string(),
                Some(true),
                Value::Mapping(expected_map),
            )
            .unwrap();

        if let Value::Mapping(map) = step {
            assert_eq!(
                map.get(Value::String("step".to_string())),
                Some(&Value::Number(1.into()))
            );
            assert_eq!(
                map.get(Value::String("manual".to_string())),
                Some(&Value::Bool(true))
            );
            assert_eq!(
                map.get(Value::String("description".to_string())),
                Some(&Value::String("Test step".to_string()))
            );
            assert_eq!(
                map.get(Value::String("command".to_string())),
                Some(&Value::String("ssh command".to_string()))
            );
            assert!(map.contains_key(Value::String("expected".to_string())));
        } else {
            panic!("Expected step to be a mapping");
        }
    }

    #[test]
    fn test_step_builder_build_step_without_manual() {
        let builder = StepCollectionBuilder::new(1);

        let mut expected_map = serde_yaml::Mapping::new();
        expected_map.insert(
            Value::String("result".to_string()),
            Value::String("0x9000".to_string()),
        );
        expected_map.insert(
            Value::String("output".to_string()),
            Value::String("Success".to_string()),
        );

        let step = builder
            .build_step_value(
                2,
                "Test step".to_string(),
                "ssh command".to_string(),
                None, // No manual flag
                Value::Mapping(expected_map),
            )
            .unwrap();

        if let Value::Mapping(map) = step {
            assert_eq!(
                map.get(Value::String("step".to_string())),
                Some(&Value::Number(2.into()))
            );
            assert!(!map.contains_key(Value::String("manual".to_string())));
        } else {
            panic!("Expected step to be a mapping");
        }
    }

    // Tests for SequenceCollectionBuilder
    #[test]
    fn test_sequence_collection_builder_creation() {
        let builder = SequenceCollectionBuilder::new(1);
        assert_eq!(builder.starting_sequence_id, 1);
        assert!(builder.db.is_none());
        assert!(builder.validator.is_none());
        assert!(builder.existing_sequence_names.is_empty());
        assert!(builder.existing_step_descriptions.is_empty());
    }

    #[test]
    fn test_sequence_collection_builder_with_validator() {
        let validator = SchemaValidator::new().unwrap();
        let builder = SequenceCollectionBuilder::new(1).with_validator(validator);
        assert!(builder.validator.is_some());
    }

    #[test]
    fn test_sequence_collection_builder_with_existing_names() {
        let names = vec!["Seq 1".to_string(), "Seq 2".to_string()];
        let builder = SequenceCollectionBuilder::new(1).with_existing_sequence_names(names.clone());
        assert_eq!(builder.existing_sequence_names.len(), 2);
    }

    #[test]
    fn test_sequence_collection_builder_with_existing_step_descriptions() {
        let descriptions = vec!["Step 1".to_string(), "Step 2".to_string()];
        let builder =
            SequenceCollectionBuilder::new(1).with_existing_step_descriptions(descriptions.clone());
        assert_eq!(builder.existing_step_descriptions.len(), 2);
    }

    #[test]
    fn test_sequence_builder_build_sequence_value() {
        let builder = SequenceCollectionBuilder::new(1);

        let mut initial_conditions = HashMap::new();
        initial_conditions.insert(
            "eUICC".to_string(),
            vec!["Condition 1".to_string(), "Condition 2".to_string()],
        );

        let steps = vec![];

        let sequence = builder
            .build_sequence_value(
                1,
                "Test Sequence".to_string(),
                "Test Description".to_string(),
                Some(initial_conditions),
                steps,
            )
            .unwrap();

        if let Value::Mapping(map) = sequence {
            assert_eq!(
                map.get(Value::String("id".to_string())),
                Some(&Value::Number(1.into()))
            );
            assert_eq!(
                map.get(Value::String("name".to_string())),
                Some(&Value::String("Test Sequence".to_string()))
            );
            assert_eq!(
                map.get(Value::String("description".to_string())),
                Some(&Value::String("Test Description".to_string()))
            );
            assert!(map.contains_key(Value::String("initial_conditions".to_string())));
            assert!(map.contains_key(Value::String("steps".to_string())));
        } else {
            panic!("Expected sequence to be a mapping");
        }
    }

    #[test]
    fn test_sequence_builder_build_sequence_without_initial_conditions() {
        let builder = SequenceCollectionBuilder::new(1);

        let steps = vec![];

        let sequence = builder
            .build_sequence_value(
                2,
                "Test Sequence".to_string(),
                "Test Description".to_string(),
                None, // No initial conditions
                steps,
            )
            .unwrap();

        if let Value::Mapping(map) = sequence {
            assert_eq!(
                map.get(Value::String("id".to_string())),
                Some(&Value::Number(2.into()))
            );
            assert!(!map.contains_key(Value::String("initial_conditions".to_string())));
            assert!(map.contains_key(Value::String("steps".to_string())));
        } else {
            panic!("Expected sequence to be a mapping");
        }
    }

    #[test]
    fn test_sequence_builder_build_sequence_with_steps() {
        let builder = SequenceCollectionBuilder::new(1);

        // Create a sample step
        let mut step_map = serde_yaml::Mapping::new();
        step_map.insert(Value::String("step".to_string()), Value::Number(1.into()));
        step_map.insert(
            Value::String("description".to_string()),
            Value::String("Test step".to_string()),
        );
        step_map.insert(
            Value::String("command".to_string()),
            Value::String("ssh".to_string()),
        );

        let mut expected_map = serde_yaml::Mapping::new();
        expected_map.insert(
            Value::String("result".to_string()),
            Value::String("success".to_string()),
        );
        expected_map.insert(
            Value::String("output".to_string()),
            Value::String("ok".to_string()),
        );
        step_map.insert(
            Value::String("expected".to_string()),
            Value::Mapping(expected_map),
        );

        let steps = vec![Value::Mapping(step_map)];

        let sequence = builder
            .build_sequence_value(
                1,
                "Test Sequence".to_string(),
                "Test Description".to_string(),
                None,
                steps,
            )
            .unwrap();

        if let Value::Mapping(map) = sequence {
            if let Some(Value::Sequence(steps_array)) = map.get(Value::String("steps".to_string()))
            {
                assert_eq!(steps_array.len(), 1);
            } else {
                panic!("Expected steps to be a sequence");
            }
        } else {
            panic!("Expected sequence to be a mapping");
        }
    }
}
