use testcase_manager::{print_title, SchemaValidator, TestCaseStorage, TitleStyle};

fn main() -> anyhow::Result<()> {
    let validator = SchemaValidator::new()?;

    print_title(
        "Schema Validation with Detailed Error Reporting",
        TitleStyle::TripleEquals,
    );

    let valid_yaml = r#"
requirement: XXX100
item: 1
tc: 4
id: '4.2.2.2.1 TC_eUICC_ES6.UpdateMetadata'
description: 'Throughout all the ES6.UpdateMetadata test cases, SMS is used as the secure OTA channel.'
general_initial_conditions:
  - eUICC:
      - "The profile The PROFILE_OPERATIONAL1 with #METADATA_WITH_PPRS_AND_ICON is loaded on the eUICC."
initial_conditions:
  eUICC:
    - "The PROFILE_OPERATIONAL1 is Enabled."
    - "The PROFILE_OPERATIONAL2 is Enabled."
test_sequences:
  - id: 1
    name: "Test Sequence #01 Nominal: Unset PPR1"
    description: |
                   This test case verifies that the eUICC correctly processes an ES6.UpdateMetadata command to unset PPR1
                   when the profile is in the operational state and PPR1 is currently set.
    initial_conditions:
      - eUICC:
          - "The PROFILE_OPERATIONAL3 is Enabled."
    steps:
      - step: 1
        manual: true
        description: "MTD_SENDS_SMS_PP([INSTALL_PERSO_RES_ISDP]; MTD_STORE_DATA_SCRIPT(#REMOVE_PPR1, FALSE))"
        command: ssh
        expected:
          success: false
          result: "SW=0x91XX"
          output: "This operation was successful."
      - step: 2
        description: "Fetch 'XX'"
        command: ssh
        expected:
          result: "MTD_CHECK_SMS_POR(0x9000)"
          output: "This operation was successful."
  - id: 2
    name: "Test Sequence #02 Nominal: Unset PPPR2 and update icon"
    description: |
                   The purpose of this test is to verify that the MNO can unset PPR2 and update the icon and
                   icon type values from a Profile.
    initial_conditions:
      - eUICC:
          - "The PROFILE_OPERATIONAL3 is Enabled."
    steps:
      - step: 1
        description: "MTD_SENDS_SMS_PP([INSTALL_PERSO_RES_ISDP]; MTD_STORE_DATA_SCRIPT(#REMOVE_PPR1, FALSE))"
        command: ssh
        expected:
          success: false
          result: "SW=0x91XX"
          output: "This operation was successful."
      - step: 2
        description: "Fetch 'XX'"
        command: ssh
        expected:
          result: "MTD_CHECK_SMS_POR(0x9000)"
          output: "This operation was successful."
"#;

    println!("Validating complete YAML structure...");
    match validator.validate_chunk(valid_yaml) {
        Ok(_) => println!("✓ Validation successful!"),
        Err(e) => println!("✗ Validation failed: {}", e),
    }

    println!("\nValidating partial YAML structure (incomplete)...");
    let partial_yaml = r#"
requirement: XXX100
item: 1
tc: 4
"#;

    match validator.validate_chunk(partial_yaml) {
        Ok(_) => println!("✓ Partial validation successful!"),
        Err(e) => println!("✗ Partial validation failed (expected):\n{}", e),
    }

    println!("\nValidating with wrong type...");
    let invalid_yaml = r#"
requirement: XXX100
item: "not_an_integer"
tc: 4
id: '4.2.2.2.1'
description: 'Test'
general_initial_conditions:
  - eUICC:
      - "Condition"
initial_conditions:
  eUICC:
    - "Condition 1"
    - "Condition 2"
test_sequences:
  - id: 1
    name: "Test"
    description: "Test"
    initial_conditions:
      - eUICC:
          - "Condition"
    steps:
      - step: 1
        description: "Step"
        command: "ssh"
        expected:
          success: true
          result: "SW=0x9000"
          output: "Success"
      - step: 2
        description: "Step 2"
        command: "ssh"
        expected:
          result: "SW=0x9000"
          output: "Success"
  - id: 2
    name: "Test 2"
    description: "Test 2"
    initial_conditions:
      - eUICC:
          - "Condition"
    steps:
      - step: 1
        description: "Step"
        command: "ssh"
        expected:
          success: true
          result: "SW=0x9000"
          output: "Success"
      - step: 2
        description: "Step 2"
        command: "ssh"
        expected:
          result: "SW=0x9000"
          output: "Success"
"#;

    match validator.validate_chunk(invalid_yaml) {
        Ok(_) => println!("✓ Invalid validation successful (unexpected)"),
        Err(e) => println!("✗ Invalid validation failed (expected):\n{}", e),
    }

    print_title(
        "Detailed Validation Error Reporting",
        TitleStyle::TripleEquals,
    );
    println!("Using validate_with_details() for structured error information...\n");

    match validator.validate_with_details(invalid_yaml) {
        Ok(errors) => {
            if errors.is_empty() {
                println!("✓ No validation errors found");
            } else {
                println!("✗ Found {} validation error(s):\n", errors.len());
                for (idx, error) in errors.iter().enumerate() {
                    println!("Error #{}:", idx + 1);
                    println!("  JSON Path: {}", error.path);
                    println!("  Constraint Type: {}", error.constraint);
                    println!("  Expected: {}", error.expected_constraint);
                    println!("  Found Value: {}", error.found_value);
                    println!();
                }
            }
        }
        Err(e) => println!("Failed to perform validation: {}", e),
    }

    print_title("Batch Validation of Files", TitleStyle::TripleEquals);
    println!("Demonstrating load_all_with_validation() for directory scanning...\n");

    if let Ok(storage) = TestCaseStorage::new("data") {
        match storage.load_all_with_validation() {
            Ok(file_infos) => {
                println!("Found {} file(s):\n", file_infos.len());

                for file_info in file_infos {
                    let file_name = file_info
                        .path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");

                    match &file_info.status {
                        testcase_manager::FileValidationStatus::Valid => {
                            println!("✓ {} - VALID", file_name);
                        }
                        testcase_manager::FileValidationStatus::ParseError { message } => {
                            println!("✗ {} - PARSE ERROR", file_name);
                            println!("  Message: {}", message);
                        }
                        testcase_manager::FileValidationStatus::ValidationError { errors } => {
                            println!(
                                "✗ {} - SCHEMA VALIDATION FAILED ({} errors)",
                                file_name,
                                errors.len()
                            );
                            for (idx, error) in errors.iter().enumerate().take(3) {
                                println!(
                                    "  Error #{}: Path '{}' - {} (Expected: {})",
                                    idx + 1,
                                    error.path,
                                    error.constraint,
                                    error.expected_constraint
                                );
                            }
                            if errors.len() > 3 {
                                println!("  ... and {} more error(s)", errors.len() - 3);
                            }
                        }
                    }
                    println!();
                }
            }
            Err(e) => println!("Failed to load files: {}", e),
        }
    } else {
        println!("Note: 'data' directory not found. This is expected in test environments.");
    }

    Ok(())
}
