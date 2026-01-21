use anyhow::Result;
use indexmap::IndexMap;
use serde_yaml::Value;
use std::collections::VecDeque;
use std::sync::Arc;
use tempfile::TempDir;
use testcase_manager::{
    oracle::{AnswerVariant, HardcodedOracle, Oracle},
    DocumentEditor, TestCase, TestCaseStorage,
};

#[test]
fn test_edit_interactive_flow_with_hardcoded_oracle() -> Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init()
        .ok();

    let temp_dir = TempDir::new()?;
    let base_path = temp_dir.path();

    let test_case = TestCase::new(
        "REQ-EDIT-001".to_string(),
        42,
        1,
        "TC_Edit_Interactive_001".to_string(),
        "Test case for interactive editing".to_string(),
    );

    let storage = TestCaseStorage::new(base_path)?;
    let file_path = storage.save_test_case(&test_case)?;

    log::info!("=== First Edit: Modify Metadata ===");

    let document_before = DocumentEditor::load_document(&file_path)?;

    let sections: Vec<String> = document_before.keys().cloned().collect();
    assert!(sections.contains(&"requirement".to_string()));
    assert!(sections.contains(&"item".to_string()));
    assert!(sections.contains(&"tc".to_string()));
    assert!(sections.contains(&"id".to_string()));
    assert!(sections.contains(&"description".to_string()));

    let mut answers = VecDeque::new();
    answers.push_back(AnswerVariant::String("requirement".to_string()));

    let oracle: Arc<dyn Oracle> = Arc::new(HardcodedOracle::new(answers));
    let selected_section = oracle.input("Select section")?;
    assert_eq!(selected_section, "requirement");

    let mut document_after = document_before.clone();
    document_after.insert(
        "requirement".to_string(),
        Value::String("REQ-EDIT-002".to_string()),
    );

    DocumentEditor::save_document_with_change_detection(
        &file_path,
        &document_before,
        &document_after,
    )?;

    let reloaded = DocumentEditor::load_document(&file_path)?;
    assert_eq!(
        reloaded.get("requirement"),
        Some(&Value::String("REQ-EDIT-002".to_string()))
    );

    log::info!("=== Second Edit: No Changes (Verify INFO Log) ===");

    let document_before_no_change = DocumentEditor::load_document(&file_path)?;
    let document_after_no_change = document_before_no_change.clone();

    DocumentEditor::save_document_with_change_detection(
        &file_path,
        &document_before_no_change,
        &document_after_no_change,
    )?;

    let final_reloaded = DocumentEditor::load_document(&file_path)?;
    assert_eq!(
        final_reloaded.get("requirement"),
        Some(&Value::String("REQ-EDIT-002".to_string()))
    );

    log::info!("✓ Edit interactive flow test completed successfully");

    Ok(())
}

#[test]
fn test_edit_metadata_section_complete_flow() -> Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init()
        .ok();

    let temp_dir = TempDir::new()?;
    let base_path = temp_dir.path();

    let test_case = TestCase::new(
        "REQ-META-001".to_string(),
        10,
        5,
        "TC_Metadata_Edit_001".to_string(),
        "Original description".to_string(),
    );

    let storage = TestCaseStorage::new(base_path)?;
    let file_path = storage.save_test_case(&test_case)?;

    log::info!("=== Load Document ===");
    let document_before = DocumentEditor::load_document(&file_path)?;
    assert_eq!(
        document_before.get("requirement"),
        Some(&Value::String("REQ-META-001".to_string()))
    );
    assert_eq!(
        document_before.get("description"),
        Some(&Value::String("Original description".to_string()))
    );

    log::info!("=== Select and Modify Metadata Section ===");
    let mut document_after = document_before.clone();

    document_after.insert(
        "requirement".to_string(),
        Value::String("REQ-META-002".to_string()),
    );
    document_after.insert("item".to_string(), Value::Number(20.into()));
    document_after.insert(
        "description".to_string(),
        Value::String("Updated description".to_string()),
    );

    log::info!("=== Save With Change Detection ===");
    DocumentEditor::save_document_with_change_detection(
        &file_path,
        &document_before,
        &document_after,
    )?;

    log::info!("=== Verify Changes ===");
    let reloaded = DocumentEditor::load_document(&file_path)?;
    assert_eq!(
        reloaded.get("requirement"),
        Some(&Value::String("REQ-META-002".to_string()))
    );
    assert_eq!(reloaded.get("item"), Some(&Value::Number(20.into())));
    assert_eq!(
        reloaded.get("description"),
        Some(&Value::String("Updated description".to_string()))
    );

    log::info!("=== Edit Again Without Changes ===");
    let document_before_no_change = DocumentEditor::load_document(&file_path)?;
    let document_after_no_change = document_before_no_change.clone();

    DocumentEditor::save_document_with_change_detection(
        &file_path,
        &document_before_no_change,
        &document_after_no_change,
    )?;

    log::info!("✓ Metadata section edit flow completed successfully");

    Ok(())
}

#[test]
fn test_edit_multiple_sections_with_oracle() -> Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init()
        .ok();

    let temp_dir = TempDir::new()?;
    let base_path = temp_dir.path();

    let mut test_case = TestCase::new(
        "REQ-MULTI-001".to_string(),
        1,
        1,
        "TC_Multi_Section_001".to_string(),
        "Test multiple section edits".to_string(),
    );

    test_case
        .general_initial_conditions
        .insert("eUICC".to_string(), vec!["Initial condition 1".to_string()]);

    let storage = TestCaseStorage::new(base_path)?;
    let file_path = storage.save_test_case(&test_case)?;

    log::info!("=== Edit Session 1: Modify Metadata ===");
    let document_before_1 = DocumentEditor::load_document(&file_path)?;
    let mut document_after_1 = document_before_1.clone();
    document_after_1.insert(
        "requirement".to_string(),
        Value::String("REQ-MULTI-002".to_string()),
    );

    DocumentEditor::save_document_with_change_detection(
        &file_path,
        &document_before_1,
        &document_after_1,
    )?;

    log::info!("=== Edit Session 2: Modify Description ===");
    let document_before_2 = DocumentEditor::load_document(&file_path)?;
    let mut document_after_2 = document_before_2.clone();
    document_after_2.insert(
        "description".to_string(),
        Value::String("Updated via multiple edits".to_string()),
    );

    DocumentEditor::save_document_with_change_detection(
        &file_path,
        &document_before_2,
        &document_after_2,
    )?;

    log::info!("=== Edit Session 3: No Changes ===");
    let document_before_3 = DocumentEditor::load_document(&file_path)?;
    let document_after_3 = document_before_3.clone();

    DocumentEditor::save_document_with_change_detection(
        &file_path,
        &document_before_3,
        &document_after_3,
    )?;

    log::info!("=== Verify Final State ===");
    let final_document = DocumentEditor::load_document(&file_path)?;
    assert_eq!(
        final_document.get("requirement"),
        Some(&Value::String("REQ-MULTI-002".to_string()))
    );
    assert_eq!(
        final_document.get("description"),
        Some(&Value::String("Updated via multiple edits".to_string()))
    );

    log::info!("✓ Multiple section edit flow completed successfully");

    Ok(())
}

#[test]
fn test_section_selection_with_oracle() -> Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init()
        .ok();

    let temp_dir = TempDir::new()?;
    let base_path = temp_dir.path();

    let test_case = TestCase::new(
        "REQ-SELECT-001".to_string(),
        5,
        3,
        "TC_Section_Select_001".to_string(),
        "Test section selection".to_string(),
    );

    let storage = TestCaseStorage::new(base_path)?;
    let file_path = storage.save_test_case(&test_case)?;

    let document = DocumentEditor::load_document(&file_path)?;
    let sections: Vec<String> = document.keys().cloned().collect();

    assert!(sections.contains(&"requirement".to_string()));
    assert!(sections.contains(&"item".to_string()));
    assert!(sections.contains(&"tc".to_string()));
    assert!(sections.contains(&"id".to_string()));
    assert!(sections.contains(&"description".to_string()));
    assert!(sections.contains(&"general_initial_conditions".to_string()));
    assert!(sections.contains(&"initial_conditions".to_string()));
    assert!(sections.contains(&"test_sequences".to_string()));

    // Test that Prompts::select returns the selected text, not an index
    let mut answers = VecDeque::new();
    answers.push_back(AnswerVariant::String("item".to_string()));
    answers.push_back(AnswerVariant::String("tc".to_string()));
    answers.push_back(AnswerVariant::String("description".to_string()));

    let oracle: Arc<dyn Oracle> = Arc::new(HardcodedOracle::new(answers));

    // Oracle.select returns the selected string directly
    let section1 = oracle.select("Select section 1", vec!["requirement".to_string(), "item".to_string(), "tc".to_string()])?;
    assert_eq!(section1, "item");

    let section2 = oracle.select("Select section 2", vec!["requirement".to_string(), "tc".to_string(), "description".to_string()])?;
    assert_eq!(section2, "tc");

    let section3 = oracle.select("Select section 3", vec!["id".to_string(), "description".to_string()])?;
    assert_eq!(section3, "description");

    log::info!("✓ Section selection with oracle completed successfully");

    Ok(())
}

#[test]
fn test_document_hash_consistency() -> Result<()> {
    let mut document1 = IndexMap::new();
    document1.insert(
        "requirement".to_string(),
        Value::String("REQ001".to_string()),
    );
    document1.insert("item".to_string(), Value::Number(1.into()));
    document1.insert("tc".to_string(), Value::Number(2.into()));

    let hash1 = DocumentEditor::compute_document_hash(&document1)?;
    let hash2 = DocumentEditor::compute_document_hash(&document1)?;

    assert_eq!(hash1, hash2);

    let mut document2 = document1.clone();
    document2.insert(
        "requirement".to_string(),
        Value::String("REQ002".to_string()),
    );

    let hash3 = DocumentEditor::compute_document_hash(&document2)?;
    assert_ne!(hash1, hash3);

    log::info!("✓ Document hash consistency test passed");

    Ok(())
}
