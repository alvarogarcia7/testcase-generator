use anyhow::Result;
use std::collections::VecDeque;
use std::sync::Arc;
use tempfile::TempDir;
use testcase_manager::{
    oracle::{AnswerVariant, HardcodedOracle, Oracle},
    prompts::Prompts,
    SchemaValidator, TestCaseBuilder,
};

#[test]
#[ignore = "Requires CLI"]
#[allow(clippy::arc_with_non_send_sync)]
fn test_complete_workflow_with_hardcoded_oracle() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let base_path = temp_dir.path();

    // Set up the hardcoded answers for the complete workflow
    let mut answers = VecDeque::new();

    // Metadata prompts
    answers.push_back(AnswerVariant::String("REQ-TEST-001".to_string()));
    answers.push_back(AnswerVariant::Int(42));
    answers.push_back(AnswerVariant::Int(1));
    answers.push_back(AnswerVariant::String("TC_Integration_Test_001".to_string()));
    answers.push_back(AnswerVariant::String(
        "Complete integration test using HardcodedOracle".to_string(),
    ));

    // General initial conditions - add condition?
    answers.push_back(AnswerVariant::Bool(true)); // Add general initial conditions?
    answers.push_back(AnswerVariant::String("eUICC".to_string())); // Device name
    answers.push_back(AnswerVariant::String("General condition 1".to_string())); // First condition
    answers.push_back(AnswerVariant::String("".to_string())); // Empty to finish

    // Initial conditions - add condition?
    answers.push_back(AnswerVariant::Bool(true)); // Add initial conditions?
    answers.push_back(AnswerVariant::String("eUICC".to_string())); // Device name
    answers.push_back(AnswerVariant::String("Initial condition 1".to_string())); // First condition
    answers.push_back(AnswerVariant::String("Initial condition 2".to_string())); // Second condition
    answers.push_back(AnswerVariant::String("".to_string())); // Empty to finish

    // Test Sequence 1
    answers.push_back(AnswerVariant::String("Test Sequence 1".to_string())); // Sequence name
    answers.push_back(AnswerVariant::Bool(false)); // Use fuzzy search for existing names?
    answers.push_back(AnswerVariant::Bool(false)); // Edit description in editor?
    answers.push_back(AnswerVariant::Bool(true)); // Add sequence-specific initial conditions?
    answers.push_back(AnswerVariant::Bool(false)); // Use database for initial conditions?
    answers.push_back(AnswerVariant::String("eUICC".to_string())); // Device name
    answers.push_back(AnswerVariant::String("Sequence condition 1".to_string())); // Condition
    answers.push_back(AnswerVariant::String("".to_string())); // Empty to finish

    // Step 1 of Sequence 1
    answers.push_back(AnswerVariant::Bool(false)); // Use fuzzy search for step description?
    answers.push_back(AnswerVariant::String("Execute test command".to_string())); // Step description
    answers.push_back(AnswerVariant::Bool(false)); // Is manual step?
    answers.push_back(AnswerVariant::String("ssh test-device".to_string())); // Command
    answers.push_back(AnswerVariant::Bool(true)); // Include success field?
    answers.push_back(AnswerVariant::Bool(true)); // Success value?
    answers.push_back(AnswerVariant::String("0x9000".to_string())); // Expected result
    answers.push_back(AnswerVariant::String("Success output".to_string())); // Expected output

    // Step 2 of Sequence 1
    answers.push_back(AnswerVariant::Bool(false)); // Use fuzzy search for step description?
    answers.push_back(AnswerVariant::String("Verify results".to_string())); // Step description
    answers.push_back(AnswerVariant::Bool(false)); // Is manual step?
    answers.push_back(AnswerVariant::String("ssh verify".to_string())); // Command
    answers.push_back(AnswerVariant::Bool(false)); // Include success field?
    answers.push_back(AnswerVariant::String("OK".to_string())); // Expected result
    answers.push_back(AnswerVariant::String("Verification passed".to_string())); // Expected output

    let oracle: Arc<dyn Oracle> = Arc::new(HardcodedOracle::new(answers));

    // Step 1: Build metadata
    let metadata = Prompts::prompt_metadata_with_oracle(&oracle)?;
    assert_eq!(metadata.requirement, "REQ-TEST-001");
    assert_eq!(metadata.item, 42);
    assert_eq!(metadata.tc, 1);
    assert_eq!(metadata.id, "TC_Integration_Test_001");
    assert_eq!(
        metadata.description,
        "Complete integration test using HardcodedOracle"
    );

    // Step 2: Initialize builder and add metadata
    let mut builder = TestCaseBuilder::new(base_path, oracle.clone())?;

    let yaml_map = metadata.to_yaml();
    for (key, value) in yaml_map {
        builder.structure_mut().insert(key, value);
    }

    // Validate metadata
    let validator = SchemaValidator::new()?;
    let _yaml_content = builder.to_yaml_string()?;
    // Note: Metadata alone won't validate against full schema, so we skip validation here

    // Step 3: Add general initial conditions
    if oracle.confirm("Add general initial conditions?")? {
        builder.add_general_initial_conditions(None)?;
    }

    // Step 4: Add initial conditions
    if oracle.confirm("Add initial conditions?")? {
        builder.add_initial_conditions(None)?;
    }

    // Step 5: Add test sequence
    let sequence_name = oracle.input("Sequence name")?;
    assert_eq!(sequence_name, "Test Sequence 1");

    oracle.confirm("Use fuzzy search for existing names?")?; // false
    oracle.confirm("Edit description in editor?")?; // false

    let add_seq_conditions = oracle.confirm("Add sequence-specific initial conditions?")?;
    assert!(add_seq_conditions);

    let use_db = oracle.confirm("Use database for initial conditions?")?;
    assert!(!use_db);

    // Manually construct sequence since we're bypassing interactive prompts
    let sequence_id = builder.get_next_sequence_id();
    let device_name = oracle.input("Device name")?;
    assert_eq!(device_name, "eUICC");

    let cond1 = oracle.input("Condition")?;
    assert_eq!(cond1, "Sequence condition 1");

    let cond2 = oracle.input("Condition")?;
    assert_eq!(cond2, "");

    // Build sequence structure
    use serde_yaml::Value;
    let mut seq_map = serde_yaml::Mapping::new();
    seq_map.insert(
        Value::String("id".to_string()),
        Value::Number(sequence_id.into()),
    );
    seq_map.insert(
        Value::String("name".to_string()),
        Value::String(sequence_name.clone()),
    );
    seq_map.insert(
        Value::String("steps".to_string()),
        Value::Sequence(Vec::new()),
    );

    // Add initial conditions to sequence
    let euicc_conditions = vec![Value::String(cond1.clone())];
    let mut ic_map = serde_yaml::Mapping::new();
    ic_map.insert(
        Value::String("eUICC".to_string()),
        Value::Sequence(euicc_conditions),
    );
    let ic_array = vec![Value::Mapping(ic_map)];
    seq_map.insert(
        Value::String("initial_conditions".to_string()),
        Value::Sequence(ic_array),
    );

    builder.validate_and_append_sequence(Value::Mapping(seq_map))?;

    // Step 6: Add steps to sequence
    let sequence_index = 0;

    // Add Step 1
    let use_fuzzy = oracle.confirm("Use fuzzy search for step description?")?;
    assert!(!use_fuzzy);

    let step1_desc = oracle.input("Step description")?;
    assert_eq!(step1_desc, "Execute test command");

    let is_manual = oracle.confirm("Is manual step?")?;
    assert!(!is_manual);

    let command1 = oracle.input("Command")?;
    assert_eq!(command1, "ssh test-device");

    let include_success1 = oracle.confirm("Include success field?")?;
    assert!(include_success1);

    let success_value1 = oracle.confirm("Success value?")?;
    assert!(success_value1);

    let result1 = oracle.input("Expected result")?;
    assert_eq!(result1, "0x9000");

    let output1 = oracle.input("Expected output")?;
    assert_eq!(output1, "Success output");

    // Create step 1
    let mut expected1_map = serde_yaml::Mapping::new();
    expected1_map.insert(
        Value::String("success".to_string()),
        Value::Bool(success_value1),
    );
    expected1_map.insert(
        Value::String("result".to_string()),
        Value::String(result1.clone()),
    );
    expected1_map.insert(
        Value::String("output".to_string()),
        Value::String(output1.clone()),
    );

    let step1 = builder.create_step_value(
        1,
        None,
        step1_desc.clone(),
        command1.clone(),
        Value::Mapping(expected1_map),
    )?;

    builder.validate_and_append_step(sequence_index, step1)?;

    // Add Step 2
    let use_fuzzy2 = oracle.confirm("Use fuzzy search for step description?")?;
    assert!(!use_fuzzy2);

    let step2_desc = oracle.input("Step description")?;
    assert_eq!(step2_desc, "Verify results");

    let is_manual2 = oracle.confirm("Is manual step?")?;
    assert!(!is_manual2);

    let command2 = oracle.input("Command")?;
    assert_eq!(command2, "ssh verify");

    let include_success2 = oracle.confirm("Include success field?")?;
    assert!(!include_success2);

    let result2 = oracle.input("Expected result")?;
    assert_eq!(result2, "OK");

    let output2 = oracle.input("Expected output")?;
    assert_eq!(output2, "Verification passed");

    // Create step 2
    let mut expected2_map = serde_yaml::Mapping::new();
    expected2_map.insert(
        Value::String("result".to_string()),
        Value::String(result2.clone()),
    );
    expected2_map.insert(
        Value::String("output".to_string()),
        Value::String(output2.clone()),
    );

    let step2 = builder.create_step_value(
        2,
        None,
        step2_desc.clone(),
        command2.clone(),
        Value::Mapping(expected2_map),
    )?;

    builder.validate_and_append_step(sequence_index, step2)?;

    // Step 7: Validate complete structure
    let final_yaml = builder.to_yaml_string()?;
    println!("\n=== Generated YAML ===");
    println!("{}", final_yaml);

    // Validate YAML structure
    assert!(final_yaml.contains("requirement: REQ-TEST-001"));
    assert!(final_yaml.contains("item: 42"));
    assert!(final_yaml.contains("tc: 1"));
    assert!(final_yaml.contains("id: TC_Integration_Test_001"));
    assert!(final_yaml.contains("Complete integration test using HardcodedOracle"));

    // Validate general initial conditions
    assert!(final_yaml.contains("general_initial_conditions"));
    assert!(final_yaml.contains("General condition 1"));

    // Validate initial conditions
    assert!(final_yaml.contains("initial_conditions"));
    assert!(final_yaml.contains("Initial condition 1"));
    assert!(final_yaml.contains("Initial condition 2"));

    // Validate test sequence
    assert!(final_yaml.contains("test_sequences"));
    assert!(final_yaml.contains("Test Sequence 1"));
    assert!(final_yaml.contains("Sequence condition 1"));

    // Validate steps
    assert!(final_yaml.contains("steps"));
    assert!(final_yaml.contains("Execute test command"));
    assert!(final_yaml.contains("ssh test-device"));
    assert!(final_yaml.contains("0x9000"));
    assert!(final_yaml.contains("Success output"));
    assert!(final_yaml.contains("success: true"));

    assert!(final_yaml.contains("Verify results"));
    assert!(final_yaml.contains("ssh verify"));
    assert!(final_yaml.contains("result: OK"));
    assert!(final_yaml.contains("Verification passed"));

    // Step 8: Validate against schema
    let validation_result = validator.validate_chunk(&final_yaml);
    if let Err(e) = &validation_result {
        println!("\n=== Validation Error ===");
        println!("{:?}", e);
    }
    assert!(
        validation_result.is_ok(),
        "Generated YAML should validate against schema"
    );

    // Step 9: Save file and verify it exists
    let file_path = builder.save()?;
    assert!(file_path.exists());
    assert_eq!(
        file_path.file_name().unwrap(),
        "TC_Integration_Test_001.yaml"
    );

    // Step 10: Parse saved file and verify structure
    let saved_content = std::fs::read_to_string(&file_path)?;
    let parsed: testcase_manager::TestCase = serde_yaml::from_str(&saved_content)?;

    // Verify metadata
    assert_eq!(parsed.requirement, "REQ-TEST-001");
    assert_eq!(parsed.item, 42);
    assert_eq!(parsed.tc, 1);
    assert_eq!(parsed.id, "TC_Integration_Test_001");

    // Verify general initial conditions
    assert!(!parsed.general_initial_conditions.is_empty());

    // Verify initial conditions
    let euicc = parsed.initial_conditions.devices.get("eUICC").unwrap();
    assert_eq!(euicc.len(), 2);
    assert!(matches!(
        &euicc[0],
        testcase_manager::models::InitialConditionItem::String(s) if s == "Initial condition 1"
    ));
    assert!(matches!(
        &euicc[1],
        testcase_manager::models::InitialConditionItem::String(s) if s == "Initial condition 2"
    ));

    // Verify test sequence
    assert_eq!(parsed.test_sequences.len(), 1);
    let sequence = &parsed.test_sequences[0];
    assert_eq!(sequence.id, 1);
    assert_eq!(sequence.name, "Test Sequence 1");
    assert!(!sequence.initial_conditions.is_empty());

    // Verify steps
    assert_eq!(sequence.steps.len(), 2);

    let step1_parsed = &sequence.steps[0];
    assert_eq!(step1_parsed.step, 1);
    assert_eq!(step1_parsed.description, "Execute test command");
    assert_eq!(step1_parsed.command, "ssh test-device");
    assert_eq!(step1_parsed.expected.success, Some(true));
    assert_eq!(step1_parsed.expected.result, "0x9000");
    assert_eq!(step1_parsed.expected.output, "Success output");

    let step2_parsed = &sequence.steps[1];
    assert_eq!(step2_parsed.step, 2);
    assert_eq!(step2_parsed.description, "Verify results");
    assert_eq!(step2_parsed.command, "ssh verify");
    assert_eq!(step2_parsed.expected.success, None);
    assert_eq!(step2_parsed.expected.result, "OK");
    assert_eq!(step2_parsed.expected.output, "Verification passed");

    println!("\n✓ Complete workflow test passed!");

    Ok(())
}

#[test]
#[allow(clippy::arc_with_non_send_sync)]
fn test_minimal_workflow_with_hardcoded_oracle() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let base_path = temp_dir.path();

    let mut answers = VecDeque::new();

    // Metadata only
    answers.push_back(AnswerVariant::String("REQ-MIN-001".to_string()));
    answers.push_back(AnswerVariant::Int(1));
    answers.push_back(AnswerVariant::Int(1));
    answers.push_back(AnswerVariant::String("TC_Minimal_001".to_string()));
    answers.push_back(AnswerVariant::String("Minimal test case".to_string()));

    let oracle: Arc<dyn Oracle> = Arc::new(HardcodedOracle::new(answers));

    // Build metadata
    let metadata = Prompts::prompt_metadata_with_oracle(&oracle)?;
    assert_eq!(metadata.requirement, "REQ-MIN-001");
    assert_eq!(metadata.item, 1);
    assert_eq!(metadata.tc, 1);
    assert_eq!(metadata.id, "TC_Minimal_001");

    // Initialize builder
    let mut builder = TestCaseBuilder::new(base_path, oracle.clone())?;

    let yaml_map = metadata.to_yaml();
    for (key, value) in yaml_map {
        builder.structure_mut().insert(key, value);
    }

    // Add minimal required fields for validation
    use serde_yaml::Value;

    // Add empty general_initial_conditions
    let empty_general_ic = vec![Value::Mapping({
        let mut map = serde_yaml::Mapping::new();
        map.insert(
            Value::String("eUICC".to_string()),
            Value::Sequence(Vec::new()),
        );
        map
    })];
    builder.structure_mut().insert(
        "general_initial_conditions".to_string(),
        Value::Sequence(empty_general_ic),
    );

    // Add empty initial_conditions
    let mut ic_map = serde_yaml::Mapping::new();
    ic_map.insert(
        Value::String("eUICC".to_string()),
        Value::Sequence(Vec::new()),
    );
    builder
        .structure_mut()
        .insert("initial_conditions".to_string(), Value::Mapping(ic_map));

    // Add empty test_sequences
    builder
        .structure_mut()
        .insert("test_sequences".to_string(), Value::Sequence(Vec::new()));

    let yaml_content = builder.to_yaml_string()?;
    println!("\n=== Minimal YAML ===");
    println!("{}", yaml_content);

    // Validate structure
    assert!(yaml_content.contains("requirement: REQ-MIN-001"));
    assert!(yaml_content.contains("TC_Minimal_001"));

    println!("\n✓ Minimal workflow test passed!");

    Ok(())
}

#[test]
#[allow(clippy::arc_with_non_send_sync)]
fn test_workflow_with_multiple_sequences() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let base_path = temp_dir.path();

    let mut answers = VecDeque::new();

    // Metadata
    answers.push_back(AnswerVariant::String("REQ-MULTI-001".to_string()));
    answers.push_back(AnswerVariant::Int(10));
    answers.push_back(AnswerVariant::Int(5));
    answers.push_back(AnswerVariant::String("TC_Multi_Seq_001".to_string()));
    answers.push_back(AnswerVariant::String(
        "Test case with multiple sequences".to_string(),
    ));

    let oracle: Arc<dyn Oracle> = Arc::new(HardcodedOracle::new(answers));

    let metadata = Prompts::prompt_metadata_with_oracle(&oracle)?;
    let mut builder = TestCaseBuilder::new(base_path, oracle.clone())?;

    let yaml_map = metadata.to_yaml();
    for (key, value) in yaml_map {
        builder.structure_mut().insert(key, value);
    }

    // Add minimal required fields
    use serde_yaml::Value;

    let empty_general_ic = vec![Value::Mapping({
        let mut map = serde_yaml::Mapping::new();
        map.insert(
            Value::String("eUICC".to_string()),
            Value::Sequence(Vec::new()),
        );
        map
    })];
    builder.structure_mut().insert(
        "general_initial_conditions".to_string(),
        Value::Sequence(empty_general_ic),
    );

    let mut ic_map = serde_yaml::Mapping::new();
    ic_map.insert(
        Value::String("eUICC".to_string()),
        Value::Sequence(Vec::new()),
    );
    builder
        .structure_mut()
        .insert("initial_conditions".to_string(), Value::Mapping(ic_map));

    // Add two sequences
    for i in 1..=2 {
        let mut seq_map = serde_yaml::Mapping::new();
        seq_map.insert(Value::String("id".to_string()), Value::Number(i.into()));
        seq_map.insert(
            Value::String("name".to_string()),
            Value::String(format!("Sequence {}", i)),
        );
        seq_map.insert(
            Value::String("description".to_string()),
            Value::String(format!("Description for sequence {}", i)),
        );
        seq_map.insert(
            Value::String("initial_conditions".to_string()),
            Value::Sequence(Vec::new()),
        );
        seq_map.insert(
            Value::String("steps".to_string()),
            Value::Sequence(Vec::new()),
        );

        builder.validate_and_append_sequence(Value::Mapping(seq_map))?;
    }

    // Verify sequences were added
    assert_eq!(builder.get_sequence_count(), 2);
    assert_eq!(builder.get_next_sequence_id(), 3);

    let yaml_content = builder.to_yaml_string()?;
    println!("\n=== Multiple Sequences YAML ===");
    println!("{}", yaml_content);

    assert!(yaml_content.contains("Sequence 1"));
    assert!(yaml_content.contains("Sequence 2"));
    assert!(yaml_content.contains("Description for sequence 1"));
    assert!(yaml_content.contains("Description for sequence 2"));

    println!("\n✓ Multiple sequences workflow test passed!");

    Ok(())
}
