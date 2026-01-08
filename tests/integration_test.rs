use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tempfile::TempDir;

#[allow(dead_code)]
struct Conversation {
    input: String,
    output: String,
    dialogue: Vec<String>,
}

#[allow(dead_code)]
impl Conversation {
    fn new() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            dialogue: Vec::new(),
        }
    }

    fn answerln(&mut self, answer: &str) {
        self.input.push_str(answer);
        self.input.push('\n');
        self.dialogue.push(format!("USER: {}", answer));
    }

    fn set_output(&mut self, output: String) {
        self.output = output;
    }

    fn askln(&self) -> Option<String> {
        self.output.lines().last().map(|s| s.to_string())
    }

    fn get_conversation(&self) -> Vec<String> {
        self.dialogue.clone()
    }

    fn get_input(&self) -> &str {
        &self.input
    }
}

/// TestInputReader - Injectable stdin replacement for testing
#[allow(dead_code)]
struct TestInputReader {
    inputs: Vec<String>,
    current_index: usize,
}

#[allow(dead_code)]
impl TestInputReader {
    fn new(inputs: Vec<String>) -> Self {
        Self {
            inputs,
            current_index: 0,
        }
    }

    #[allow(dead_code)]
    fn read_line(&mut self) -> Option<String> {
        if self.current_index < self.inputs.len() {
            let line = self.inputs[self.current_index].clone();
            self.current_index += 1;
            Some(line)
        } else {
            None
        }
    }
}

#[test]
#[ignore] // Ignore by default since it requires terminal interaction
fn test_end_to_end_complete_workflow() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let test_dir = temp_dir.path();
    let output_file = test_dir.join("test_case_001.yaml");

    let binary_path = get_binary_path();

    println!("Test directory: {}", test_dir.display());
    println!("Binary path: {}", binary_path.display());
    println!("Output file: {}", output_file.display());

    let mut conversation = Conversation::new();

    // Metadata prompts
    conversation.answerln("REQ-TEST-001");
    conversation.answerln("42");
    conversation.answerln("1");
    conversation.answerln("TC_Integration_Test_001");
    conversation.answerln("End-to-end integration test for testcase-manager workflow");

    // Commit metadata?
    conversation.answerln("y");

    // Add general initial conditions?
    conversation.answerln("y");
    conversation.answerln("eUICC");
    conversation.answerln("General initial condition 1");
    conversation.answerln("");

    // Commit general initial conditions?
    conversation.answerln("y");

    // Add initial conditions?
    conversation.answerln("y");
    conversation.answerln("eUICC");
    conversation.answerln("Initial condition 1");
    conversation.answerln("Initial condition 2");
    conversation.answerln("");

    // Commit initial conditions?
    conversation.answerln("y");

    // Test Sequence 1
    conversation.answerln("Test Sequence 1");
    conversation.answerln("n");
    conversation.answerln("y");
    conversation.answerln("eUICC");
    conversation.answerln("Sequence-specific condition 1");
    conversation.answerln("");

    // Commit this sequence?
    conversation.answerln("y");

    // Add steps to this sequence now?
    conversation.answerln("y");

    // Step 1
    conversation.answerln("n");
    conversation.answerln("Execute test command");
    conversation.answerln("n");
    conversation.answerln("ssh test-device");
    conversation.answerln("n");
    conversation.answerln("0x9000");
    conversation.answerln("Success output");

    // Commit this step?
    conversation.answerln("y");

    // Add another step to this sequence?
    conversation.answerln("y");

    // Step 2
    conversation.answerln("n");
    conversation.answerln("Verify results");
    conversation.answerln("n");
    conversation.answerln("ssh verify");
    conversation.answerln("n");
    conversation.answerln("OK");
    conversation.answerln("Verification passed");

    // Commit this step?
    conversation.answerln("y");

    // Add another step to this sequence?
    conversation.answerln("n");

    // Add another test sequence?
    conversation.answerln("n");

    // Commit final complete test case?
    conversation.answerln("y");

    println!("Starting testcase-manager process...");

    let scripted_input = conversation.get_input();

    let mut child = Command::new(&binary_path)
        .arg("complete")
        .arg("--output")
        .arg(&output_file)
        .arg("--commit-prefix")
        .arg("TEST")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn testcase-manager process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(scripted_input.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child
        .wait_with_output()
        .expect("Failed to wait for process");

    println!("\n=== STDOUT ===");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("\n=== STDERR ===");
    println!("{}", String::from_utf8_lossy(&output.stderr));
    println!("\n=== STATUS ===");
    println!("{}", output.status);

    assert!(
        output.status.success(),
        "testcase-manager process failed with status: {}",
        output.status
    );

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
}

#[test]
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

    use testcase_manager::TestCase;
    let parsed: TestCase = serde_yaml::from_str(yaml_content).expect("Failed to parse YAML");

    assert_eq!(parsed.requirement, "REQ-TEST-001");
    assert_eq!(parsed.item, 42);
    assert_eq!(parsed.tc, 1);
    assert_eq!(parsed.id, "TC_Integration_Test_001");
    assert_eq!(parsed.test_sequences.len(), 1);
    assert_eq!(parsed.test_sequences[0].steps.len(), 2);
}

fn get_binary_path() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let mut binary_path = PathBuf::from(manifest_dir);
    binary_path.push("target");
    binary_path.push("debug");
    binary_path.push("testcase-manager");

    if !binary_path.exists() {
        panic!(
            "Binary not found at {}. Please run 'cargo build' first.",
            binary_path.display()
        );
    }

    binary_path
}

#[allow(dead_code)]
fn create_scripted_input() -> String {
    let mut conversation = Conversation::new();

    // Metadata prompts
    conversation.answerln("REQ-TEST-001"); // Requirement
    conversation.answerln("42"); // Item
    conversation.answerln("1"); // TC
    conversation.answerln("TC_Integration_Test_001"); // ID
    conversation.answerln("End-to-end integration test for testcase-manager workflow"); // Description

    // Commit metadata?
    conversation.answerln("y");

    // Add general initial conditions?
    conversation.answerln("y");
    // Device name for general initial conditions
    conversation.answerln("eUICC");
    // Condition #1
    conversation.answerln("General initial condition 1");
    // Condition #2 (empty to finish)
    conversation.answerln("");

    // Commit general initial conditions?
    conversation.answerln("y");

    // Add initial conditions?
    conversation.answerln("y");
    // Device name for initial conditions
    conversation.answerln("eUICC");
    // Condition #1
    conversation.answerln("Initial condition 1");
    // Condition #2
    conversation.answerln("Initial condition 2");
    // Condition #3 (empty to finish)
    conversation.answerln("");

    // Commit initial conditions?
    conversation.answerln("y");

    // === First Test Sequence ===
    // Sequence name
    conversation.answerln("Test Sequence 1");
    // Edit description in editor?
    conversation.answerln("n");
    // Add sequence-specific initial conditions?
    conversation.answerln("y");
    // Device name for sequence initial conditions
    conversation.answerln("eUICC");
    // Sequence condition #1
    conversation.answerln("Sequence-specific condition 1");
    // Sequence condition #2 (empty to finish)
    conversation.answerln("");

    // Commit this sequence?
    conversation.answerln("y");

    // Add steps to this sequence now?
    conversation.answerln("y");

    // === Step 1 ===
    // Use fuzzy search for existing descriptions?
    conversation.answerln("n");
    // Step description
    conversation.answerln("Execute test command");
    // Is this a manual step?
    conversation.answerln("n");
    // Command
    conversation.answerln("ssh test-device");
    // Include 'success' field?
    conversation.answerln("n");
    // Expected result
    conversation.answerln("0x9000");
    // Expected output
    conversation.answerln("Success output");

    // Commit this step?
    conversation.answerln("y");

    // Add another step to this sequence?
    conversation.answerln("y");

    // === Step 2 ===
    // Use fuzzy search for existing descriptions?
    conversation.answerln("n");
    // Step description
    conversation.answerln("Verify results");
    // Is this a manual step?
    conversation.answerln("n");
    // Command
    conversation.answerln("ssh verify");
    // Include 'success' field?
    conversation.answerln("n");
    // Expected result
    conversation.answerln("OK");
    // Expected output
    conversation.answerln("Verification passed");

    // Commit this step?
    conversation.answerln("y");

    // Add another step to this sequence?
    conversation.answerln("n");

    // Add another test sequence?
    conversation.answerln("n");

    // Commit final complete test case?
    conversation.answerln("y");

    conversation.get_input().to_string()
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
    use testcase_manager::TestCase;

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
    assert!(
        !parsed_test_case.general_initial_conditions[0]
            .euicc
            .is_empty(),
        "General initial conditions eUICC should not be empty"
    );

    assert!(
        !parsed_test_case.initial_conditions.euicc.is_empty(),
        "Initial conditions should not be empty"
    );
    assert_eq!(parsed_test_case.initial_conditions.euicc.len(), 2);

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
    use testcase_manager::GitManager;

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
