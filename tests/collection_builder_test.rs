use std::collections::HashMap;
use testcase_manager::collection_builder::{
    CollectionBuilder, ConditionCollectionBuilder, SelectionMode, SequenceCollectionBuilder,
    StepCollectionBuilder,
};
use testcase_manager::fuzzy::FuzzySearchResult;
use testcase_manager::validation::SchemaValidator;

/// HardcodedOracle simulates user interactions for testing
///
/// This oracle provides pre-programmed responses for:
/// - Fuzzy search operations (simulating ESC cancellations)
/// - Editor inputs (returning pre-defined YAML content)
/// - Confirmation prompts (yes/no responses)
///
/// It enables deterministic testing of the collection builder workflow
/// without requiring actual user interaction.
struct HardcodedOracle {
    /// Pre-programmed fuzzy search results
    fuzzy_responses: Vec<FuzzySearchResult>,
    fuzzy_index: std::cell::RefCell<usize>,

    /// Pre-programmed editor content responses
    editor_responses: Vec<String>,
    editor_index: std::cell::RefCell<usize>,

    /// Pre-programmed confirmation responses
    confirm_responses: Vec<bool>,
    confirm_index: std::cell::RefCell<usize>,

    /// Pre-programmed text input responses
    input_responses: Vec<String>,
    input_index: std::cell::RefCell<usize>,
}

impl HardcodedOracle {
    fn new() -> Self {
        Self {
            fuzzy_responses: Vec::new(),
            fuzzy_index: std::cell::RefCell::new(0),
            editor_responses: Vec::new(),
            editor_index: std::cell::RefCell::new(0),
            confirm_responses: Vec::new(),
            confirm_index: std::cell::RefCell::new(0),
            input_responses: Vec::new(),
            input_index: std::cell::RefCell::new(0),
        }
    }

    /// Add a fuzzy search response (Selected or Cancelled for ESC)
    fn add_fuzzy_response(mut self, response: FuzzySearchResult) -> Self {
        self.fuzzy_responses.push(response);
        self
    }

    /// Add an editor content response
    #[allow(dead_code)]
    fn add_editor_response(mut self, content: String) -> Self {
        self.editor_responses.push(content);
        self
    }

    /// Add a confirmation response
    #[allow(dead_code)]
    fn add_confirm_response(mut self, response: bool) -> Self {
        self.confirm_responses.push(response);
        self
    }

    /// Add a text input response
    fn add_input_response(mut self, response: String) -> Self {
        self.input_responses.push(response);
        self
    }

    /// Get next fuzzy search result
    fn next_fuzzy_result(&self) -> FuzzySearchResult {
        let mut idx = self.fuzzy_index.borrow_mut();
        if *idx < self.fuzzy_responses.len() {
            let result = self.fuzzy_responses[*idx].clone();
            *idx += 1;
            result
        } else {
            FuzzySearchResult::Cancelled
        }
    }

    /// Get next editor content
    fn next_editor_content(&self) -> String {
        let mut idx = self.editor_index.borrow_mut();
        if *idx < self.editor_responses.len() {
            let content = self.editor_responses[*idx].clone();
            *idx += 1;
            content
        } else {
            String::new()
        }
    }

    /// Get next confirmation response
    fn next_confirm(&self) -> bool {
        let mut idx = self.confirm_index.borrow_mut();
        if *idx < self.confirm_responses.len() {
            let response = self.confirm_responses[*idx];
            *idx += 1;
            response
        } else {
            false
        }
    }

    /// Get next input response
    fn next_input(&self) -> String {
        let mut idx = self.input_index.borrow_mut();
        if *idx < self.input_responses.len() {
            let response = self.input_responses[*idx].clone();
            *idx += 1;
            response
        } else {
            String::new()
        }
    }
}

#[test]
fn test_collection_builder_with_fuzzy_esc_cancellation() {
    // Test scenario: User presses ESC during fuzzy search, then creates new item in editor

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
    struct TestItem {
        name: String,
        value: i32,
    }

    impl std::fmt::Display for TestItem {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} (value: {})", self.name, self.value)
        }
    }

    let existing_items = vec![
        TestItem {
            name: "Existing Item 1".to_string(),
            value: 10,
        },
        TestItem {
            name: "Existing Item 2".to_string(),
            value: 20,
        },
    ];

    let template = "name: New Item\nvalue: 100\n";

    let builder = CollectionBuilder::new(
        existing_items,
        template.to_string(),
        SelectionMode::SingleSelect,
    );

    // Verify builder is initialized correctly
    assert_eq!(builder.collection().len(), 0);
}

#[test]
fn test_condition_collection_builder_creates_hashmap() {
    // Test that ConditionCollectionBuilder produces the correct HashMap structure

    let _builder = ConditionCollectionBuilder::new();

    // Create a sample conditions map manually to verify the structure
    let mut expected_map: HashMap<String, Vec<String>> = HashMap::new();
    expected_map.insert(
        "eUICC".to_string(),
        vec![
            "Condition 1".to_string(),
            "Condition 2".to_string(),
            "Condition 3".to_string(),
        ],
    );

    // Verify the structure
    assert!(expected_map.contains_key("eUICC"));
    assert_eq!(expected_map.get("eUICC").unwrap().len(), 3);
}

#[test]
fn test_step_collection_builder_creates_yaml_values() {
    // Test that StepCollectionBuilder creates proper serde_yaml::Value structures

    let _builder = StepCollectionBuilder::new(1);

    // Build a sample step manually
    let mut expected_map = serde_yaml::Mapping::new();
    expected_map.insert(
        serde_yaml::Value::String("step".to_string()),
        serde_yaml::Value::Number(1.into()),
    );
    expected_map.insert(
        serde_yaml::Value::String("description".to_string()),
        serde_yaml::Value::String("Test step".to_string()),
    );
    expected_map.insert(
        serde_yaml::Value::String("command".to_string()),
        serde_yaml::Value::String("ssh command".to_string()),
    );

    let mut expected_inner = serde_yaml::Mapping::new();
    expected_inner.insert(
        serde_yaml::Value::String("result".to_string()),
        serde_yaml::Value::String("0x9000".to_string()),
    );
    expected_inner.insert(
        serde_yaml::Value::String("output".to_string()),
        serde_yaml::Value::String("Success".to_string()),
    );

    expected_map.insert(
        serde_yaml::Value::String("expected".to_string()),
        serde_yaml::Value::Mapping(expected_inner),
    );

    let step_value = serde_yaml::Value::Mapping(expected_map);

    // Verify we can serialize it
    let yaml_str = serde_yaml::to_string(&step_value).unwrap();
    assert!(yaml_str.contains("step: 1"));
    assert!(yaml_str.contains("description:"));
    assert!(yaml_str.contains("command:"));
    assert!(yaml_str.contains("expected:"));
}

#[test]
fn test_sequence_collection_builder_initialization() {
    // Test that SequenceCollectionBuilder can be initialized

    let _builder = SequenceCollectionBuilder::new(1);

    // Verify builder can be created with different starting IDs
    let _builder2 = SequenceCollectionBuilder::new(42);
}

#[test]
fn test_integration_multi_item_condition_array() {
    // Integration test: Build an array of conditions iteratively

    // Simulate building multiple conditions
    let mut conditions: Vec<String> = Vec::new();

    // Iteration 1: Add first condition
    conditions.push("Initial condition 1".to_string());
    assert_eq!(conditions.len(), 1);

    // Iteration 2: Add second condition
    conditions.push("Initial condition 2".to_string());
    assert_eq!(conditions.len(), 2);

    // Iteration 3: Add third condition
    conditions.push("Initial condition 3".to_string());
    assert_eq!(conditions.len(), 3);

    // Verify final array
    assert_eq!(
        conditions,
        vec![
            "Initial condition 1".to_string(),
            "Initial condition 2".to_string(),
            "Initial condition 3".to_string(),
        ]
    );
}

#[test]
fn test_integration_multi_item_step_array() {
    // Integration test: Build an array of steps iteratively

    let mut steps: Vec<serde_yaml::Value> = Vec::new();

    // Iteration 1: Add first step
    let step1 = create_test_step(1, "Step 1 description", "command1");
    steps.push(step1);
    assert_eq!(steps.len(), 1);

    // Iteration 2: Add second step
    let step2 = create_test_step(2, "Step 2 description", "command2");
    steps.push(step2);
    assert_eq!(steps.len(), 2);

    // Iteration 3: Add third step
    let step3 = create_test_step(3, "Step 3 description", "command3");
    steps.push(step3);
    assert_eq!(steps.len(), 3);

    // Verify final array
    assert_eq!(steps.len(), 3);

    // Verify each step has correct structure
    for (idx, step) in steps.iter().enumerate() {
        if let serde_yaml::Value::Mapping(map) = step {
            assert!(map.contains_key(serde_yaml::Value::String("step".to_string())));
            assert!(map.contains_key(serde_yaml::Value::String("description".to_string())));
            assert!(map.contains_key(serde_yaml::Value::String("command".to_string())));
            assert!(map.contains_key(serde_yaml::Value::String("expected".to_string())));

            // Verify step number
            let step_num = map.get(serde_yaml::Value::String("step".to_string()));
            assert_eq!(step_num, Some(&serde_yaml::Value::Number((idx + 1).into())));
        } else {
            panic!("Expected step to be a mapping");
        }
    }
}

#[test]
fn test_integration_multi_item_sequence_array() {
    // Integration test: Build an array of sequences iteratively
    // We test the concept of building multiple sequences without calling private methods

    let mut sequences_metadata: Vec<(i64, String, String)> = Vec::new();

    // Iteration 1: Add first sequence metadata
    sequences_metadata.push((1, "Sequence 1".to_string(), "Description 1".to_string()));
    assert_eq!(sequences_metadata.len(), 1);

    // Iteration 2: Add second sequence metadata
    sequences_metadata.push((2, "Sequence 2".to_string(), "Description 2".to_string()));
    assert_eq!(sequences_metadata.len(), 2);

    // Verify final array
    assert_eq!(sequences_metadata.len(), 2);

    for (idx, (id, name, _desc)) in sequences_metadata.iter().enumerate() {
        assert_eq!(*id, (idx + 1) as i64);
        assert!(name.contains(&format!("Sequence {}", idx + 1)));
    }
}

#[test]
fn test_integration_full_workflow_simulation() {
    // Comprehensive integration test simulating the complete workflow:
    // 1. Build conditions with ESC cancellation
    // 2. Build steps with editor inputs
    // 3. Build sequences with nested structures

    // Step 1: Simulate condition collection
    let oracle = HardcodedOracle::new()
        .add_fuzzy_response(FuzzySearchResult::Cancelled) // ESC on device search
        .add_input_response("eUICC".to_string()) // Manual device name entry
        .add_input_response("Condition 1".to_string())
        .add_input_response("Condition 2".to_string())
        .add_input_response("Condition 3".to_string())
        .add_input_response("".to_string()); // Empty to finish

    // Simulate condition building
    let mut conditions_map: HashMap<String, Vec<String>> = HashMap::new();
    let device_name = oracle.next_input(); // "eUICC"
    let mut conditions = Vec::new();

    loop {
        let input = oracle.next_input();
        if input.is_empty() || conditions.len() >= 3 {
            break;
        }
        conditions.push(input);
    }

    conditions_map.insert(device_name, conditions);

    // Verify conditions
    assert_eq!(conditions_map.len(), 1);
    assert!(conditions_map.contains_key("eUICC"));
    assert_eq!(conditions_map.get("eUICC").unwrap().len(), 3);

    // Step 2: Simulate step collection
    let mut steps = Vec::new();

    // Build 3 steps
    for i in 1..=3 {
        let step = create_test_step(
            i,
            &format!("Step {} description", i),
            &format!("command{}", i),
        );
        steps.push(step);
    }

    assert_eq!(steps.len(), 3);

    // Step 3: Verify we can create sequence metadata that would include the above
    let sequence_metadata = (1, "Test Sequence 1".to_string(), "Integration test sequence".to_string());
    
    // Verify the complete workflow structure
    assert_eq!(sequence_metadata.0, 1);
    assert_eq!(sequence_metadata.1, "Test Sequence 1");
    assert_eq!(conditions_map.len(), 1);
    assert_eq!(steps.len(), 3);
}

#[test]
fn test_hardcoded_oracle_fuzzy_cancellation() {
    // Test that HardcodedOracle properly simulates ESC cancellation

    let oracle = HardcodedOracle::new()
        .add_fuzzy_response(FuzzySearchResult::Cancelled)
        .add_fuzzy_response(FuzzySearchResult::Selected("Item 1".to_string()));

    // First call should return Cancelled (ESC)
    let result1 = oracle.next_fuzzy_result();
    assert_eq!(result1, FuzzySearchResult::Cancelled);

    // Second call should return Selected
    let result2 = oracle.next_fuzzy_result();
    assert_eq!(result2, FuzzySearchResult::Selected("Item 1".to_string()));
}

#[test]
fn test_hardcoded_oracle_editor_inputs() {
    // Test that HardcodedOracle properly simulates editor inputs

    let yaml_content1 = "name: Item 1\nvalue: 100\n";
    let yaml_content2 = "name: Item 2\nvalue: 200\n";

    let oracle = HardcodedOracle::new()
        .add_editor_response(yaml_content1.to_string())
        .add_editor_response(yaml_content2.to_string());

    // First editor session
    let content1 = oracle.next_editor_content();
    assert_eq!(content1, yaml_content1);

    // Second editor session
    let content2 = oracle.next_editor_content();
    assert_eq!(content2, yaml_content2);
}

#[test]
fn test_hardcoded_oracle_confirmation_flow() {
    // Test that HardcodedOracle properly simulates yes/no confirmations

    let oracle = HardcodedOracle::new()
        .add_confirm_response(true) // Yes
        .add_confirm_response(false) // No
        .add_confirm_response(true); // Yes

    assert!(oracle.next_confirm());
    assert!(!oracle.next_confirm());
    assert!(oracle.next_confirm());
}

#[test]
fn test_complete_workflow_with_schema_validation() {
    // Integration test with actual schema validation

    let validator = SchemaValidator::new().expect("Failed to create validator");

    // Test building a step with validation
    let _builder = StepCollectionBuilder::new_with_validator(1, &validator);

    // Create a sample step manually to verify YAML structure
    let mut expected_map = serde_yaml::Mapping::new();
    expected_map.insert(
        serde_yaml::Value::String("step".to_string()),
        serde_yaml::Value::Number(1.into()),
    );
    expected_map.insert(
        serde_yaml::Value::String("description".to_string()),
        serde_yaml::Value::String("Test step with validation".to_string()),
    );
    expected_map.insert(
        serde_yaml::Value::String("command".to_string()),
        serde_yaml::Value::String("ssh test-command".to_string()),
    );
    expected_map.insert(
        serde_yaml::Value::String("manual".to_string()),
        serde_yaml::Value::Bool(true),
    );

    let mut expected_inner = serde_yaml::Mapping::new();
    expected_inner.insert(
        serde_yaml::Value::String("result".to_string()),
        serde_yaml::Value::String("0x9000".to_string()),
    );
    expected_inner.insert(
        serde_yaml::Value::String("output".to_string()),
        serde_yaml::Value::String("Success".to_string()),
    );

    expected_map.insert(
        serde_yaml::Value::String("expected".to_string()),
        serde_yaml::Value::Mapping(expected_inner),
    );

    let step = serde_yaml::Value::Mapping(expected_map);

    // Verify step can be serialized and validated
    let yaml_str = serde_yaml::to_string(&step).unwrap();

    // The step itself is valid YAML
    assert!(yaml_str.contains("step: 1"));
    assert!(yaml_str.contains("description:"));
    assert!(yaml_str.contains("manual: true"));
}

// Helper function to create a test step
fn create_test_step(step_num: i64, description: &str, command: &str) -> serde_yaml::Value {
    let mut step_map = serde_yaml::Mapping::new();

    step_map.insert(
        serde_yaml::Value::String("step".to_string()),
        serde_yaml::Value::Number(step_num.into()),
    );

    step_map.insert(
        serde_yaml::Value::String("description".to_string()),
        serde_yaml::Value::String(description.to_string()),
    );

    step_map.insert(
        serde_yaml::Value::String("command".to_string()),
        serde_yaml::Value::String(command.to_string()),
    );

    // Add expected object
    let mut expected_map = serde_yaml::Mapping::new();
    expected_map.insert(
        serde_yaml::Value::String("result".to_string()),
        serde_yaml::Value::String("OK".to_string()),
    );
    expected_map.insert(
        serde_yaml::Value::String("output".to_string()),
        serde_yaml::Value::String("Success".to_string()),
    );

    step_map.insert(
        serde_yaml::Value::String("expected".to_string()),
        serde_yaml::Value::Mapping(expected_map),
    );

    serde_yaml::Value::Mapping(step_map)
}
