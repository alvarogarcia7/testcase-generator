use anyhow::Result;
use std::collections::VecDeque;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tempfile::TempDir;
use testcase_manager::{
    oracle::{AnswerVariant, HardcodedOracle, Oracle},
    GitManager, Prompts, TestCase, TestCaseBuilder,
};

#[test]
#[ignore = "Requires CLI"]
fn test_end_to_end_complete_workflow() -> Result<()> {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let test_dir = temp_dir.path();
    let output_file = test_dir.join("test_case_001.yaml");

    println!("Test directory: {}", test_dir.display());
    println!("Output file: {}", output_file.display());

    // Set up all hardcoded answers for complete workflow
    let mut answers = VecDeque::new();

    // Metadata prompts
    answers.push_back(AnswerVariant::String("REQ-TEST-001".to_string()));
    answers.push_back(AnswerVariant::Int(42));
    answers.push_back(AnswerVariant::Int(1));
    answers.push_back(AnswerVariant::String("TC_Integration_Test_001".to_string()));
    answers.push_back(AnswerVariant::String(
        "End-to-end integration test for testcase-manager workflow".to_string(),
    ));

    // Commit metadata?
    answers.push_back(AnswerVariant::Bool(true));

    // Add general initial conditions?
    answers.push_back(AnswerVariant::Bool(true));
    answers.push_back(AnswerVariant::String("eUICC".to_string()));
    answers.push_back(AnswerVariant::String(
        "General initial condition 1".to_string(),
    ));
    answers.push_back(AnswerVariant::String("".to_string()));

    // Commit general initial conditions?
    answers.push_back(AnswerVariant::Bool(true));

    // Add initial conditions?
    answers.push_back(AnswerVariant::Bool(true));
    answers.push_back(AnswerVariant::String("eUICC".to_string()));
    answers.push_back(AnswerVariant::String("Initial condition 1".to_string()));
    answers.push_back(AnswerVariant::String("Initial condition 2".to_string()));
    answers.push_back(AnswerVariant::String("".to_string()));

    // Commit initial conditions?
    answers.push_back(AnswerVariant::Bool(true));

    // Test Sequence 1
    answers.push_back(AnswerVariant::String("Test Sequence 1".to_string()));
    answers.push_back(AnswerVariant::Bool(false)); // Use fuzzy search?
    answers.push_back(AnswerVariant::Bool(false)); // Edit description in editor?
    answers.push_back(AnswerVariant::Bool(true)); // Add sequence-specific initial conditions?
    answers.push_back(AnswerVariant::Bool(false)); // Use database?
    answers.push_back(AnswerVariant::String("eUICC".to_string()));
    answers.push_back(AnswerVariant::String(
        "Sequence-specific condition 1".to_string(),
    ));
    answers.push_back(AnswerVariant::String("".to_string()));

    // Commit this sequence?
    answers.push_back(AnswerVariant::Bool(true));

    // Add steps to this sequence now?
    answers.push_back(AnswerVariant::Bool(true));

    // Step 1
    answers.push_back(AnswerVariant::Bool(false)); // Use fuzzy search?
    answers.push_back(AnswerVariant::String("Execute test command".to_string()));
    answers.push_back(AnswerVariant::Bool(false)); // Is manual step?
    answers.push_back(AnswerVariant::String("ssh test-device".to_string()));
    answers.push_back(AnswerVariant::Bool(false)); // Include 'success' field?
    answers.push_back(AnswerVariant::String("0x9000".to_string()));
    answers.push_back(AnswerVariant::String("Success output".to_string()));

    // Commit this step?
    answers.push_back(AnswerVariant::Bool(true));

    // Add another step to this sequence?
    answers.push_back(AnswerVariant::Bool(true));

    // Step 2
    answers.push_back(AnswerVariant::Bool(false)); // Use fuzzy search?
    answers.push_back(AnswerVariant::String("Verify results".to_string()));
    answers.push_back(AnswerVariant::Bool(false)); // Is manual step?
    answers.push_back(AnswerVariant::String("ssh verify".to_string()));
    answers.push_back(AnswerVariant::Bool(false)); // Include 'success' field?
    answers.push_back(AnswerVariant::String("OK".to_string()));
    answers.push_back(AnswerVariant::String("Verification passed".to_string()));

    // Commit this step?
    answers.push_back(AnswerVariant::Bool(true));

    // Add another step to this sequence?
    answers.push_back(AnswerVariant::Bool(false));

    // Add another test sequence?
    answers.push_back(AnswerVariant::Bool(false));

    // Commit final complete test case?
    answers.push_back(AnswerVariant::Bool(true));

    let oracle: Arc<dyn Oracle> = Arc::new(HardcodedOracle::new(answers));

    println!("Starting test case workflow...");

    // Build the complete test case using HardcodedOracle
    let metadata = Prompts::prompt_metadata_with_oracle(&oracle)?;
    assert_eq!(metadata.requirement, "REQ-TEST-001");
    assert_eq!(metadata.item, 42);
    assert_eq!(metadata.tc, 1);
    assert_eq!(metadata.id, "TC_Integration_Test_001");

    let mut builder = TestCaseBuilder::new(test_dir, oracle.clone())?;

    let yaml_map = metadata.to_yaml();
    for (key, value) in yaml_map {
        builder.structure_mut().insert(key, value);
    }

    // Commit metadata
    let commit_metadata = oracle.confirm("Commit metadata to git?")?;
    if commit_metadata {
        builder.commit("TEST: Add test case metadata")?;
    }

    // Add general initial conditions
    let add_general = oracle.confirm("Add general initial conditions?")?;
    if add_general {
        builder.add_general_initial_conditions(None)?;
    }

    // Commit general initial conditions
    let commit_general = oracle.confirm("Commit general initial conditions?")?;
    if commit_general {
        builder.commit("TEST: Add general initial conditions")?;
    }

    // Add initial conditions
    let add_ic = oracle.confirm("Add initial conditions?")?;
    if add_ic {
        builder.add_initial_conditions(None)?;
    }

    // Commit initial conditions
    let commit_ic = oracle.confirm("Commit initial conditions?")?;
    if commit_ic {
        builder.commit("TEST: Add initial conditions")?;
    }

    // Add test sequence
    builder.add_test_sequence_interactive()?;

    // Commit sequence
    let commit_seq = oracle.confirm("Commit this sequence?")?;
    if commit_seq {
        builder.commit("TEST: Add test sequence")?;
    }

    // Add steps to sequence
    let add_steps = oracle.confirm("Add steps to this sequence now?")?;
    if add_steps {
        // Add Step 1
        builder.add_steps_to_sequence_with_commits(1 - 1)?;

        // Commit step 1
        let commit_step1 = oracle.confirm("Commit this step?")?;
        if commit_step1 {
            builder.commit("TEST: Add step 1")?;
        }

        // Add another step?
        let add_step2 = oracle.confirm("Add another step to this sequence?")?;
        if add_step2 {
            // Add Step 2
            builder.add_steps_to_sequence_by_id_with_commits(2 - 1)?;

            // Commit step 2
            let commit_step2 = oracle.confirm("Commit this step?")?;
            if commit_step2 {
                builder.commit("TEST: Add step 2")?;
            }
        }

        // Add another step?
        oracle.confirm("Add another step to this sequence?")?; // false
    }

    // Add another sequence?
    oracle.confirm("Add another test sequence?")?; // false

    // Save final test case
    builder.save()?;

    // Commit final test case
    let commit_final = oracle.confirm("Commit final complete test case?")?;
    if commit_final {
        builder.commit("TEST: Complete test case with all sequences and steps")?;
    }

    println!("\n=== Workflow completed successfully ===");

    assert!(
        output_file.exists(),
        "Output YAML file was not created: {}",
        output_file.display()
    );

    let yaml_content =
        fs::read_to_string(&output_file).expect("Failed to read generated YAML file");

    println!("\n=== Generated YAML ===");
    println!("{}", yaml_content);

    validate_yaml_structure(&yaml_content);

    validate_yaml_parsing(&output_file);

    validate_git_commits(test_dir);

    println!("\n✓ All validations passed!");

    Ok(())
}

#[test]
#[ignore = "SKIPPED until the parsing of the YAML / JSON SCHEMA is correct again"]
fn test_yaml_structure_only() {
    // This is a simpler test that doesn't require spawning the binary
    // It validates that we can create and parse a valid YAML structure
    let yaml_content = r#"
requirement: REQ-TEST-001
item: 42
tc: 1
id: TC_Integration_Test_001
description: End-to-end integration test for testcase-manager workflow
general_initial_conditions:
  - eUICC:
      - General initial condition 1
initial_conditions:
  eUICC:
    - Initial condition 1
    - Initial condition 2
test_sequences:
  - id: 1
    name: Test Sequence 1
    description: ""
    initial_conditions:
      - eUICC:
          - Sequence-specific condition 1
    steps:
      - step: 1
        description: Execute test command
        command: ssh test-device
        expected:
          result: "0x9000"
          output: Success output
      - step: 2
        description: Verify results
        command: ssh verify
        expected:
          result: OK
          output: Verification passed
"#;

    validate_yaml_structure(yaml_content);

    let parsed: TestCase = serde_yaml::from_str(yaml_content).expect("Failed to parse YAML");

    assert_eq!(parsed.requirement, "REQ-TEST-001");
    assert_eq!(parsed.item, 42);
    assert_eq!(parsed.tc, 1);
    assert_eq!(parsed.id, "TC_Integration_Test_001");
    assert_eq!(parsed.test_sequences.len(), 1);
    assert_eq!(parsed.test_sequences[0].steps.len(), 2);
}

fn validate_yaml_structure(yaml_content: &str) {
    assert!(
        yaml_content.contains("requirement:"),
        "YAML missing 'requirement' field"
    );
    assert!(
        yaml_content.contains("REQ-TEST-001"),
        "YAML missing requirement value"
    );

    assert!(yaml_content.contains("item:"), "YAML missing 'item' field");
    assert!(yaml_content.contains("42"), "YAML missing item value");

    assert!(yaml_content.contains("tc:"), "YAML missing 'tc' field");

    assert!(yaml_content.contains("id:"), "YAML missing 'id' field");
    assert!(
        yaml_content.contains("TC_Integration_Test_001"),
        "YAML missing id value"
    );

    assert!(
        yaml_content.contains("description:"),
        "YAML missing 'description' field"
    );
    assert!(
        yaml_content.contains("End-to-end integration test"),
        "YAML missing description value"
    );

    assert!(
        yaml_content.contains("general_initial_conditions:"),
        "YAML missing 'general_initial_conditions' field"
    );
    assert!(
        yaml_content.contains("General initial condition 1"),
        "YAML missing general initial condition value"
    );

    assert!(
        yaml_content.contains("initial_conditions:"),
        "YAML missing 'initial_conditions' field"
    );
    assert!(
        yaml_content.contains("Initial condition 1"),
        "YAML missing initial condition 1"
    );
    assert!(
        yaml_content.contains("Initial condition 2"),
        "YAML missing initial condition 2"
    );

    assert!(
        yaml_content.contains("test_sequences:"),
        "YAML missing 'test_sequences' field"
    );

    assert!(
        yaml_content.contains("Test Sequence 1"),
        "YAML missing sequence name"
    );

    assert!(
        yaml_content.contains("Sequence-specific condition 1"),
        "YAML missing sequence-specific initial condition"
    );

    assert!(
        yaml_content.contains("steps:"),
        "YAML missing 'steps' field"
    );

    assert!(
        yaml_content.contains("Execute test command"),
        "YAML missing step 1 description"
    );
    assert!(
        yaml_content.contains("ssh test-device"),
        "YAML missing step 1 command"
    );
    assert!(
        yaml_content.contains("0x9000"),
        "YAML missing step 1 expected result"
    );
    assert!(
        yaml_content.contains("Success output"),
        "YAML missing step 1 expected output"
    );

    assert!(
        yaml_content.contains("Verify results"),
        "YAML missing step 2 description"
    );
    assert!(
        yaml_content.contains("ssh verify"),
        "YAML missing step 2 command"
    );
    assert!(
        yaml_content.contains("Verification passed"),
        "YAML missing step 2 expected output"
    );
}

fn validate_yaml_parsing(yaml_file: &Path) {
    let yaml_content = fs::read_to_string(yaml_file).expect("Failed to read YAML file for parsing");

    let parsed_test_case: TestCase =
        serde_yaml::from_str(&yaml_content).expect("Failed to parse YAML as TestCase");

    assert_eq!(parsed_test_case.requirement, "REQ-TEST-001");
    assert_eq!(parsed_test_case.item, 42);
    assert_eq!(parsed_test_case.tc, 1);
    assert_eq!(parsed_test_case.id, "TC_Integration_Test_001");
    assert!(parsed_test_case
        .description
        .contains("End-to-end integration test"));

    assert!(
        !parsed_test_case.general_initial_conditions.is_empty(),
        "General initial conditions should not be empty"
    );

    let ic_euicc = parsed_test_case.initial_conditions.get("eUICC").unwrap();

    assert!(
        !ic_euicc.is_empty(),
        "Initial conditions should not be empty"
    );
    assert_eq!(ic_euicc.len(), 2);

    assert!(
        !parsed_test_case.test_sequences.is_empty(),
        "Test sequences should not be empty"
    );

    let sequence = &parsed_test_case.test_sequences[0];
    assert_eq!(sequence.id, 1);
    assert_eq!(sequence.name, "Test Sequence 1");

    assert!(
        !sequence.initial_conditions.is_empty(),
        "Sequence initial conditions should not be empty"
    );

    assert!(!sequence.steps.is_empty(), "Steps should not be empty");
    assert_eq!(sequence.steps.len(), 2);

    let step1 = &sequence.steps[0];
    assert_eq!(step1.step, 1);
    assert_eq!(step1.description, "Execute test command");
    assert_eq!(step1.command, "ssh test-device");
    assert_eq!(step1.expected.result, "0x9000");
    assert_eq!(step1.expected.output, "Success output");

    let step2 = &sequence.steps[1];
    assert_eq!(step2.step, 2);
    assert_eq!(step2.description, "Verify results");
    assert_eq!(step2.command, "ssh verify");
    assert_eq!(step2.expected.result, "OK");
    assert_eq!(step2.expected.output, "Verification passed");

    println!("✓ YAML parsed successfully with correct structure");
}

fn validate_git_commits(repo_path: &Path) {
    let git = GitManager::open(repo_path).expect("Failed to open git repository");

    let commits = git.log(20).expect("Failed to get commit log");

    assert!(!commits.is_empty(), "No git commits found");

    println!("\n=== Git Commits ===");
    for commit in &commits {
        println!(
            "{} - {}",
            &commit.id[..7],
            commit.message.lines().next().unwrap_or("")
        );
    }

    let commit_messages: Vec<String> = commits.iter().map(|c| c.message.clone()).collect();

    let has_metadata_commit = commit_messages
        .iter()
        .any(|msg| msg.contains("metadata") || msg.contains("TEST: Add test case metadata"));

    assert!(
        has_metadata_commit,
        "No metadata commit found in git history"
    );

    let has_initial_conditions_commit = commit_messages
        .iter()
        .any(|msg| msg.contains("initial conditions"));

    assert!(
        has_initial_conditions_commit,
        "No initial conditions commit found in git history"
    );

    let has_sequence_commit = commit_messages
        .iter()
        .any(|msg| msg.contains("sequence") || msg.contains("TEST: Add test sequence"));

    assert!(
        has_sequence_commit,
        "No sequence commit found in git history"
    );

    let has_step_commits = commit_messages
        .iter()
        .any(|msg| msg.contains("step") || msg.contains("TEST: Add step"));

    assert!(has_step_commits, "No step commits found in git history");

    let has_final_commit = commit_messages
        .iter()
        .any(|msg| msg.contains("Complete test case") || msg.contains("all sequences and steps"));

    assert!(has_final_commit, "No final commit found in git history");

    assert!(
        commits.len() >= 5,
        "Expected at least 5 commits (metadata, general IC, IC, sequence, step(s), final), but found {}",
        commits.len()
    );

    println!("✓ Git commits validated successfully");
    println!("  Total commits: {}", commits.len());
}
