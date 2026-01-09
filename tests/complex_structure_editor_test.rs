use anyhow::Result;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tempfile::{NamedTempFile, TempDir};
use testcase_manager::{
    complex_structure_editor::ComplexStructureEditor,
    config::EditorConfig,
    models::{Expected, Step, TestSequence},
    oracle::{AnswerVariant, HardcodedOracle, Oracle},
    validation::SchemaValidator,
};

/// Test fuzzy search cancellation triggers template/multi-line editing
#[test]
fn test_fuzzy_search_cancellation_triggers_template_editing() -> Result<()> {
    let database: Vec<Expected> = vec![
        Expected {
            success: Some(true),
            result: "SW=0x9000".to_string(),
            output: "Success".to_string(),
        },
        Expected {
            success: Some(false),
            result: "SW=0x6985".to_string(),
            output: "Conditions not satisfied".to_string(),
        },
    ];

    let template = r#"success: true
result: "SW=0x9000"
output: "Template Success"
"#;

    // Create oracle that simulates fuzzy search cancellation (empty string)
    let mut answers = VecDeque::new();
    answers.push_back(AnswerVariant::String("".to_string())); // Cancelled fuzzy search
    answers.push_back(AnswerVariant::String(template.to_string())); // multi_line_input returns template

    let oracle: Arc<dyn Oracle> = Arc::new(HardcodedOracle::new(answers));

    // Create a mock editor that just returns the input content
    let temp_file = NamedTempFile::new()?;
    let editor_script = format!(
        r#"#!/bin/bash
cat "$1"
"#
    );
    std::fs::write(temp_file.path(), editor_script)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(temp_file.path())?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(temp_file.path(), perms)?;
    }

    let editor_config = EditorConfig {
        editor: Some(temp_file.path().to_string_lossy().to_string()),
        visual: None,
        custom_fallback: None,
    };

    let validator = SchemaValidator::new()?;

    let result = ComplexStructureEditor::<Expected>::edit_with_fuzzy_search(
        &database,
        "Select Expected",
        oracle.as_ref(),
        &editor_config,
        &validator,
        template,
    );

    // Should succeed with template content
    assert!(result.is_ok(), "Expected edit to succeed: {:?}", result.err());
    let edited = result.unwrap();
    assert_eq!(edited.success, Some(true));
    assert_eq!(edited.result, "SW=0x9000");
    assert_eq!(edited.output, "Template Success");

    Ok(())
}

/// Test selected instances load properly in editor
#[test]
fn test_selected_instances_load_in_editor() -> Result<()> {
    let database: Vec<Step> = vec![
        Step::new(
            1,
            "First step".to_string(),
            "ssh device1".to_string(),
            "OK".to_string(),
            "Success".to_string(),
        ),
        Step::new(
            2,
            "Second step".to_string(),
            "ssh device2".to_string(),
            "FAIL".to_string(),
            "Error".to_string(),
        ),
    ];

    // Select the second step
    let selected_display = database[1].to_string();

    let mut answers = VecDeque::new();
    answers.push_back(AnswerVariant::String(selected_display)); // Select second step

    let oracle: Arc<dyn Oracle> = Arc::new(HardcodedOracle::new(answers));

    // Create a mock editor that modifies the step slightly
    let temp_dir = TempDir::new()?;
    let editor_script_path = temp_dir.path().join("editor.sh");
    let editor_script = r#"#!/bin/bash
# Read the input file and modify it slightly
sed 's/ssh device2/ssh device2-modified/g' "$1" > "$1.tmp"
mv "$1.tmp" "$1"
"#;
    std::fs::write(&editor_script_path, editor_script)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&editor_script_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&editor_script_path, perms)?;
    }

    let editor_config = EditorConfig {
        editor: Some(editor_script_path.to_string_lossy().to_string()),
        visual: None,
        custom_fallback: None,
    };

    let validator = SchemaValidator::new()?;

    let template = r#"step: 1
description: "Default step"
command: "ssh default"
expected:
  result: "OK"
  output: "Default output"
"#;

    let result = ComplexStructureEditor::<Step>::edit_with_fuzzy_search(
        &database,
        "Select Step",
        oracle.as_ref(),
        &editor_config,
        &validator,
        template,
    );

    // Should succeed with modified command
    assert!(
        result.is_ok(),
        "Expected edit to succeed: {:?}",
        result.err()
    );
    let edited = result.unwrap();
    assert_eq!(edited.step, 2);
    assert_eq!(edited.description, "Second step");
    assert_eq!(edited.command, "ssh device2-modified");

    Ok(())
}

/// Test validation errors re-open editor
#[test]
fn test_validation_errors_reopen_editor() -> Result<()> {
    let database: Vec<Expected> = vec![Expected {
        success: Some(true),
        result: "SW=0x9000".to_string(),
        output: "Success".to_string(),
    }];

    let template = r#"success: true
result: "SW=0x9000"
output: "Valid output"
"#;

    let mut answers = VecDeque::new();
    answers.push_back(AnswerVariant::String("".to_string())); // Cancel fuzzy search
    answers.push_back(AnswerVariant::String(template.to_string())); // multi_line_input

    let oracle: Arc<dyn Oracle> = Arc::new(HardcodedOracle::new(answers));

    // Create a mock editor that returns invalid YAML on first call, then valid on second
    let temp_dir = TempDir::new()?;
    let counter_file = temp_dir.path().join("counter.txt");
    std::fs::write(&counter_file, "0")?;

    let editor_script_path = temp_dir.path().join("editor.sh");
    let counter_file_str = counter_file.to_string_lossy().to_string();
    let editor_script = format!(
        r#"#!/bin/bash
COUNTER_FILE="{}"
COUNT=$(cat "$COUNTER_FILE")
COUNT=$((COUNT + 1))
echo "$COUNT" > "$COUNTER_FILE"

if [ "$COUNT" -eq 1 ]; then
    # First call: return invalid YAML (missing required field)
    echo 'success: true' > "$1"
    echo 'result: "SW=0x9000"' >> "$1"
    # Missing 'output' field - validation should fail
else
    # Second call: return valid YAML
    cat "$1" > "$1.tmp"
    if ! grep -q 'output:' "$1"; then
        echo 'output: "Fixed output"' >> "$1.tmp"
    fi
    mv "$1.tmp" "$1"
fi
"#,
        counter_file_str
    );
    std::fs::write(&editor_script_path, editor_script)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&editor_script_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&editor_script_path, perms)?;
    }

    let editor_config = EditorConfig {
        editor: Some(editor_script_path.to_string_lossy().to_string()),
        visual: None,
        custom_fallback: None,
    };

    let validator = SchemaValidator::new()?;

    let result = ComplexStructureEditor::<Expected>::edit_with_fuzzy_search(
        &database,
        "Select Expected",
        oracle.as_ref(),
        &editor_config,
        &validator,
        template,
    );

    // Should eventually succeed after validation loop
    assert!(
        result.is_ok(),
        "Expected edit to succeed after validation loop: {:?}",
        result.err()
    );
    let edited = result.unwrap();
    assert_eq!(edited.success, Some(true));
    assert_eq!(edited.result, "SW=0x9000");
    assert_eq!(edited.output, "Fixed output");

    // Verify editor was called twice
    let counter = std::fs::read_to_string(&counter_file)?;
    assert_eq!(counter.trim(), "2", "Editor should have been called twice");

    Ok(())
}

/// Test non-TTY multi-line input works correctly for Expected type
#[test]
fn test_non_tty_multiline_input_expected() -> Result<()> {
    let database: Vec<Expected> = vec![];

    let yaml_input = r#"success: false
result: "SW=0x6985"
output: "Non-TTY input"
"#;

    let mut answers = VecDeque::new();
    // In non-TTY mode, fuzzy_search_strings should return None
    answers.push_back(AnswerVariant::String("".to_string())); // Empty for no selection
    answers.push_back(AnswerVariant::String(yaml_input.to_string())); // multi_line_input

    let oracle: Arc<dyn Oracle> = Arc::new(HardcodedOracle::new(answers));

    // Create a mock editor
    let temp_file = NamedTempFile::new()?;
    let editor_script = r#"#!/bin/bash
cat "$1"
"#;
    std::fs::write(temp_file.path(), editor_script)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(temp_file.path())?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(temp_file.path(), perms)?;
    }

    let editor_config = EditorConfig {
        editor: Some(temp_file.path().to_string_lossy().to_string()),
        visual: None,
        custom_fallback: None,
    };

    let validator = SchemaValidator::new()?;

    let template = r#"success: true
result: "SW=0x9000"
output: "Template output"
"#;

    let result = ComplexStructureEditor::<Expected>::edit_with_fuzzy_search(
        &database,
        "Select Expected",
        oracle.as_ref(),
        &editor_config,
        &validator,
        template,
    );

    assert!(
        result.is_ok(),
        "Expected edit to succeed: {:?}",
        result.err()
    );
    let edited = result.unwrap();
    assert_eq!(edited.success, Some(false));
    assert_eq!(edited.result, "SW=0x6985");
    assert_eq!(edited.output, "Non-TTY input");

    Ok(())
}

/// Test non-TTY multi-line input works correctly for Step type
#[test]
fn test_non_tty_multiline_input_step() -> Result<()> {
    let database: Vec<Step> = vec![];

    let yaml_input = r#"step: 5
description: "Non-TTY Step"
command: "ssh non-tty-device"
expected:
  result: "OK"
  output: "Non-TTY output"
"#;

    let mut answers = VecDeque::new();
    answers.push_back(AnswerVariant::String("".to_string())); // Empty for no selection
    answers.push_back(AnswerVariant::String(yaml_input.to_string())); // multi_line_input

    let oracle: Arc<dyn Oracle> = Arc::new(HardcodedOracle::new(answers));

    let temp_file = NamedTempFile::new()?;
    let editor_script = r#"#!/bin/bash
cat "$1"
"#;
    std::fs::write(temp_file.path(), editor_script)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(temp_file.path())?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(temp_file.path(), perms)?;
    }

    let editor_config = EditorConfig {
        editor: Some(temp_file.path().to_string_lossy().to_string()),
        visual: None,
        custom_fallback: None,
    };

    let validator = SchemaValidator::new()?;

    let template = r#"step: 1
description: "Default step"
command: "ssh default"
expected:
  result: "OK"
  output: "Default"
"#;

    let result = ComplexStructureEditor::<Step>::edit_with_fuzzy_search(
        &database,
        "Select Step",
        oracle.as_ref(),
        &editor_config,
        &validator,
        template,
    );

    assert!(
        result.is_ok(),
        "Expected edit to succeed: {:?}",
        result.err()
    );
    let edited = result.unwrap();
    assert_eq!(edited.step, 5);
    assert_eq!(edited.description, "Non-TTY Step");
    assert_eq!(edited.command, "ssh non-tty-device");
    assert_eq!(edited.expected.result, "OK");
    assert_eq!(edited.expected.output, "Non-TTY output");

    Ok(())
}

/// Test non-TTY multi-line input works correctly for TestSequence type
#[test]
fn test_non_tty_multiline_input_test_sequence() -> Result<()> {
    let database: Vec<TestSequence> = vec![];

    let yaml_input = r#"id: 10
name: "Non-TTY Sequence"
description: "Sequence created via non-TTY"
initial_conditions:
  eUICC:
    - "Non-TTY condition"
steps: []
"#;

    let mut answers = VecDeque::new();
    answers.push_back(AnswerVariant::String("".to_string())); // Empty for no selection
    answers.push_back(AnswerVariant::String(yaml_input.to_string())); // multi_line_input

    let oracle: Arc<dyn Oracle> = Arc::new(HardcodedOracle::new(answers));

    let temp_file = NamedTempFile::new()?;
    let editor_script = r#"#!/bin/bash
cat "$1"
"#;
    std::fs::write(temp_file.path(), editor_script)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(temp_file.path())?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(temp_file.path(), perms)?;
    }

    let editor_config = EditorConfig {
        editor: Some(temp_file.path().to_string_lossy().to_string()),
        visual: None,
        custom_fallback: None,
    };

    let validator = SchemaValidator::new()?;

    let template = r#"id: 1
name: "Default Sequence"
description: "Default description"
initial_conditions:
  eUICC: []
steps: []
"#;

    let result = ComplexStructureEditor::<TestSequence>::edit_with_fuzzy_search(
        &database,
        "Select Sequence",
        oracle.as_ref(),
        &editor_config,
        &validator,
        template,
    );

    assert!(
        result.is_ok(),
        "Expected edit to succeed: {:?}",
        result.err()
    );
    let edited = result.unwrap();
    assert_eq!(edited.id, 10);
    assert_eq!(edited.name, "Non-TTY Sequence");
    assert_eq!(edited.description, "Sequence created via non-TTY");
    
    let euicc_conditions = edited.initial_conditions.get("eUICC");
    assert!(euicc_conditions.is_some(), "eUICC key should exist");
    assert_eq!(euicc_conditions.unwrap().len(), 1);
    assert_eq!(euicc_conditions.unwrap()[0], "Non-TTY condition");
    assert_eq!(edited.steps.len(), 0);

    Ok(())
}

/// Test fuzzy search with actual selection (not cancelled)
#[test]
fn test_fuzzy_search_with_selection() -> Result<()> {
    let database: Vec<Expected> = vec![
        Expected {
            success: Some(true),
            result: "SW=0x9000".to_string(),
            output: "First".to_string(),
        },
        Expected {
            success: Some(false),
            result: "SW=0x6985".to_string(),
            output: "Second".to_string(),
        },
        Expected {
            success: None,
            result: "SW=0x6A82".to_string(),
            output: "Third".to_string(),
        },
    ];

    // Select the second item
    let selected_display = database[1].to_string();

    let mut answers = VecDeque::new();
    answers.push_back(AnswerVariant::String(selected_display));

    let oracle: Arc<dyn Oracle> = Arc::new(HardcodedOracle::new(answers));

    let temp_file = NamedTempFile::new()?;
    let editor_script = r#"#!/bin/bash
cat "$1"
"#;
    std::fs::write(temp_file.path(), editor_script)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(temp_file.path())?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(temp_file.path(), perms)?;
    }

    let editor_config = EditorConfig {
        editor: Some(temp_file.path().to_string_lossy().to_string()),
        visual: None,
        custom_fallback: None,
    };

    let validator = SchemaValidator::new()?;

    let template = ""; // Not used since we're selecting

    let result = ComplexStructureEditor::<Expected>::edit_with_fuzzy_search(
        &database,
        "Select Expected",
        oracle.as_ref(),
        &editor_config,
        &validator,
        template,
    );

    assert!(
        result.is_ok(),
        "Expected edit to succeed: {:?}",
        result.err()
    );
    let edited = result.unwrap();
    assert_eq!(edited.success, Some(false));
    assert_eq!(edited.result, "SW=0x6985");
    assert_eq!(edited.output, "Second");

    Ok(())
}

/// Test empty database uses template
#[test]
fn test_empty_database_uses_template() -> Result<()> {
    let database: Vec<Expected> = vec![]; // Empty database

    let template = r#"success: true
result: "SW=0x9000"
output: "Template used for empty database"
"#;

    let mut answers = VecDeque::new();
    // With empty database, fuzzy search isn't called, goes straight to multi_line_input
    answers.push_back(AnswerVariant::String(template.to_string())); // multi_line_input

    let oracle: Arc<dyn Oracle> = Arc::new(HardcodedOracle::new(answers));

    let temp_file = NamedTempFile::new()?;
    let editor_script = r#"#!/bin/bash
cat "$1"
"#;
    std::fs::write(temp_file.path(), editor_script)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(temp_file.path())?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(temp_file.path(), perms)?;
    }

    let editor_config = EditorConfig {
        editor: Some(temp_file.path().to_string_lossy().to_string()),
        visual: None,
        custom_fallback: None,
    };

    let validator = SchemaValidator::new()?;

    let result = ComplexStructureEditor::<Expected>::edit_with_fuzzy_search(
        &database,
        "Select Expected",
        oracle.as_ref(),
        &editor_config,
        &validator,
        template,
    );

    assert!(
        result.is_ok(),
        "Expected edit to succeed: {:?}",
        result.err()
    );
    let edited = result.unwrap();
    assert_eq!(edited.success, Some(true));
    assert_eq!(edited.result, "SW=0x9000");
    assert_eq!(edited.output, "Template used for empty database");

    Ok(())
}
