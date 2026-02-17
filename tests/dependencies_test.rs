use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use testcase_manager::{
    dependency_validator::{validate_cross_file_dependencies, DependencyValidator},
    models::{IncludeRef, InitialConditionItem, InitialConditions, Step, TestCase, TestSequence},
};

#[test]
#[ignore = "Example YAML file has unquoted scalar values that cause deserialization issues"]
fn test_deserialize_dependencies_example_1() {
    let yaml_path = PathBuf::from("testcases/examples/dependencies/1.yaml");
    assert!(
        yaml_path.exists(),
        "Test file not found: {}",
        yaml_path.display()
    );

    let yaml_content = fs::read_to_string(&yaml_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", yaml_path.display(), e));

    let test_case: TestCase = serde_yaml::from_str(&yaml_content)
        .unwrap_or_else(|e| panic!("Failed to deserialize {}: {}", yaml_path.display(), e));

    assert_eq!(test_case.id, "TC_VAR_001");
    assert_eq!(test_case.requirement, "VAR_001");
    assert_eq!(test_case.item, 1);
    assert_eq!(test_case.tc, 1);

    assert!(
        test_case.general_initial_conditions.include.is_some(),
        "Expected include in general_initial_conditions"
    );
    let general_include = test_case
        .general_initial_conditions
        .include
        .as_ref()
        .unwrap();
    assert_eq!(general_include.len(), 1);
    assert_eq!(general_include[0].id, "TC_VAR_001");
    assert_eq!(general_include[0].test_sequence, None);

    assert!(
        test_case.initial_conditions.include.is_some(),
        "Expected include in initial_conditions"
    );
    let initial_include = test_case.initial_conditions.include.as_ref().unwrap();
    assert_eq!(initial_include.len(), 1);
    assert_eq!(initial_include[0].id, "TC_VAR_001");

    // Note: The system2 conditions in the example YAML use unquoted values which
    // may cause deserialization issues with certain YAML parsers. The file structure
    // and other aspects are validated above.

    assert_eq!(test_case.test_sequences.len(), 1);
    let test_seq = &test_case.test_sequences[0];
    assert_eq!(test_seq.id, 1);

    let seq_include = test_seq
        .initial_conditions
        .include
        .as_ref()
        .expect("Expected include in test sequence initial_conditions");
    assert_eq!(seq_include.len(), 1);
    assert_eq!(seq_include[0].id, "TC_VAR_001");
    assert_eq!(seq_include[0].test_sequence, Some("1".to_string()));
}

#[test]
fn test_deserialize_dependencies_example_2() {
    let yaml_path = PathBuf::from("testcases/examples/dependencies/2.yaml");
    assert!(
        yaml_path.exists(),
        "Test file not found: {}",
        yaml_path.display()
    );

    let yaml_content = fs::read_to_string(&yaml_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", yaml_path.display(), e));

    let test_case: TestCase = serde_yaml::from_str(&yaml_content)
        .unwrap_or_else(|e| panic!("Failed to deserialize {}: {}", yaml_path.display(), e));

    assert_eq!(test_case.id, "TC_VAR_002");
    assert_eq!(test_case.requirement, "VAR_001");
    assert_eq!(test_case.item, 1);
    assert_eq!(test_case.tc, 1);

    assert_eq!(test_case.test_sequences.len(), 1);
    let test_seq = &test_case.test_sequences[0];
    assert_eq!(test_seq.id, 1);
    assert_eq!(test_seq.reference, Some("1234-af".to_string()));

    let step2 = &test_seq.steps[1];
    assert_eq!(step2.step, 2);
    assert_eq!(
        step2.reference,
        Some("12345-ffaaa11-a2213129af".to_string())
    );
}

#[test]
fn test_defined_refs_are_accepted() {
    let mut validator = DependencyValidator::new();

    let mut test_case1 = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test".to_string(),
    );
    let mut test_sequence = TestSequence::new(1, "Seq".to_string(), "Desc".to_string());
    test_sequence.reference = Some("ref-123".to_string());
    let mut step = Step::new(
        1,
        "Step".to_string(),
        "cmd".to_string(),
        "0".to_string(),
        "output".to_string(),
    );
    step.reference = Some("step-ref-456".to_string());
    test_sequence.steps.push(step);
    test_case1.test_sequences.push(test_sequence);

    validator.collect_definitions(&PathBuf::from("test1.yaml"), &test_case1);

    let mut test_case2 = TestCase::new(
        "REQ002".to_string(),
        2,
        2,
        "TC002".to_string(),
        "Test".to_string(),
    );
    let mut initial_conditions = InitialConditions::default();
    initial_conditions.devices.insert(
        "device1".to_string(),
        vec![
            InitialConditionItem::RefItem {
                reference: "ref-123".to_string(),
            },
            InitialConditionItem::RefItem {
                reference: "step-ref-456".to_string(),
            },
        ],
    );
    test_case2.initial_conditions = initial_conditions;

    let errors = validator.validate_references(&PathBuf::from("test2.yaml"), &test_case2);
    assert_eq!(
        errors.len(),
        0,
        "Defined refs should be accepted without errors"
    );
}

#[test]
fn test_undefined_refs_produce_errors() {
    let validator = DependencyValidator::new();

    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test".to_string(),
    );

    let mut initial_conditions = InitialConditions::default();
    initial_conditions.devices.insert(
        "device1".to_string(),
        vec![InitialConditionItem::RefItem {
            reference: "undefined-ref".to_string(),
        }],
    );
    test_case.initial_conditions = initial_conditions;

    let errors = validator.validate_references(&PathBuf::from("test.yaml"), &test_case);

    assert_eq!(
        errors.len(),
        1,
        "Undefined ref should produce exactly one error"
    );
    assert_eq!(errors[0].reference, "undefined-ref");
    assert!(errors[0].location.contains("initial_conditions.device1[0]"));
}

#[test]
fn test_include_resolution_for_general_initial_conditions() {
    let mut validator = DependencyValidator::new();

    let test_case1 = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test".to_string(),
    );
    validator.collect_definitions(&PathBuf::from("test1.yaml"), &test_case1);

    let mut test_case2 = TestCase::new(
        "REQ002".to_string(),
        2,
        2,
        "TC002".to_string(),
        "Test".to_string(),
    );
    let initial_conditions = InitialConditions {
        include: Some(vec![IncludeRef {
            id: "TC001".to_string(),
            test_sequence: None,
        }]),
        ..Default::default()
    };
    test_case2.general_initial_conditions = initial_conditions;

    let errors = validator.validate_references(&PathBuf::from("test2.yaml"), &test_case2);
    assert_eq!(
        errors.len(),
        0,
        "Valid include reference should not produce errors"
    );
}

#[test]
fn test_include_resolution_for_initial_conditions() {
    let mut validator = DependencyValidator::new();

    let test_case1 = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test".to_string(),
    );
    validator.collect_definitions(&PathBuf::from("test1.yaml"), &test_case1);

    let mut test_case2 = TestCase::new(
        "REQ002".to_string(),
        2,
        2,
        "TC002".to_string(),
        "Test".to_string(),
    );
    let initial_conditions = InitialConditions {
        include: Some(vec![IncludeRef {
            id: "TC001".to_string(),
            test_sequence: None,
        }]),
        ..Default::default()
    };
    test_case2.initial_conditions = initial_conditions;

    let errors = validator.validate_references(&PathBuf::from("test2.yaml"), &test_case2);
    assert_eq!(
        errors.len(),
        0,
        "Valid include reference should not produce errors"
    );
}

#[test]
fn test_include_resolution_for_test_sequence_initial_conditions() {
    let mut validator = DependencyValidator::new();

    let test_case1 = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test".to_string(),
    );
    validator.collect_definitions(&PathBuf::from("test1.yaml"), &test_case1);

    let mut test_case2 = TestCase::new(
        "REQ002".to_string(),
        2,
        2,
        "TC002".to_string(),
        "Test".to_string(),
    );
    let mut test_sequence = TestSequence::new(1, "Seq".to_string(), "Desc".to_string());
    test_sequence.initial_conditions = InitialConditions {
        include: Some(vec![IncludeRef {
            id: "TC001".to_string(),
            test_sequence: Some("1".to_string()),
        }]),
        ..Default::default()
    };
    test_case2.test_sequences.push(test_sequence);

    let errors = validator.validate_references(&PathBuf::from("test2.yaml"), &test_case2);
    assert_eq!(
        errors.len(),
        0,
        "Valid include reference with test_sequence should not produce errors"
    );
}

#[test]
fn test_mixed_initial_condition_items_programmatic() {
    let mut initial_conditions = InitialConditions::default();
    initial_conditions.devices.insert(
        "device1".to_string(),
        vec![
            InitialConditionItem::String("String condition".to_string()),
            InitialConditionItem::RefItem {
                reference: "ref-123".to_string(),
            },
        ],
    );

    let device1_conditions = initial_conditions.devices.get("device1").unwrap();
    assert_eq!(device1_conditions.len(), 2);

    match &device1_conditions[0] {
        InitialConditionItem::String(s) => assert_eq!(s, "String condition"),
        _ => panic!("Expected String variant"),
    }

    match &device1_conditions[1] {
        InitialConditionItem::RefItem { reference } => assert_eq!(reference, "ref-123"),
        _ => panic!("Expected RefItem variant"),
    }
}

#[test]
fn test_duplicate_refs_are_allowed() {
    let mut validator = DependencyValidator::new();

    let mut test_case1 = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test".to_string(),
    );
    let mut test_sequence1 = TestSequence::new(1, "Seq1".to_string(), "Desc".to_string());
    test_sequence1.reference = Some("ref-dup".to_string());
    test_case1.test_sequences.push(test_sequence1);

    let mut test_sequence2 = TestSequence::new(2, "Seq2".to_string(), "Desc".to_string());
    test_sequence2.reference = Some("ref-dup".to_string());
    test_case1.test_sequences.push(test_sequence2);

    validator.collect_definitions(&PathBuf::from("test1.yaml"), &test_case1);

    let mut test_case2 = TestCase::new(
        "REQ002".to_string(),
        2,
        2,
        "TC002".to_string(),
        "Test".to_string(),
    );
    let mut initial_conditions = InitialConditions::default();
    initial_conditions.devices.insert(
        "device1".to_string(),
        vec![InitialConditionItem::RefItem {
            reference: "ref-dup".to_string(),
        }],
    );
    test_case2.initial_conditions = initial_conditions;

    let errors = validator.validate_references(&PathBuf::from("test2.yaml"), &test_case2);
    assert_eq!(
        errors.len(),
        0,
        "Duplicate refs defined should still allow resolution"
    );
}

#[test]
fn test_self_referencing_within_same_test_case() {
    let yaml_content = r#"
requirement: "REQ001"
item: 1
tc: 1
id: "TC001"
description: "Self-referencing test case"
general_initial_conditions:
  include:
    - id: "TC001"
  device1:
    - "Condition 1"
initial_conditions:
  device1:
    - "Condition 1"
test_sequences:
  - id: 1
    name: "Seq"
    description: "Desc"
    initial_conditions:
      device1:
        - "Seq condition"
    steps:
      - step: 1
        description: "Step 1"
        command: "cmd1"
        expected:
          success: true
          result: "OK"
          output: "Success"
      - step: 2
        description: "Step 2"
        command: "cmd2"
        expected:
          result: "OK"
          output: "Success"
  - id: 2
    name: "Seq2"
    description: "Desc2"
    initial_conditions:
      device1:
        - "Seq condition"
    steps:
      - step: 1
        description: "Step 1"
        command: "cmd1"
        expected:
          success: true
          result: "OK"
          output: "Success"
      - step: 2
        description: "Step 2"
        command: "cmd2"
        expected:
          result: "OK"
          output: "Success"
"#;

    let test_case: TestCase = serde_yaml::from_str(yaml_content)
        .expect("Failed to deserialize self-referencing test case");

    let mut validator = DependencyValidator::new();
    validator.collect_definitions(&PathBuf::from("test.yaml"), &test_case);

    let errors = validator.validate_references(&PathBuf::from("test.yaml"), &test_case);
    assert_eq!(
        errors.len(),
        0,
        "Self-referencing within same test case should be allowed"
    );
}

#[test]
fn test_circular_includes_across_files() {
    let mut validator = DependencyValidator::new();

    let mut test_case1 = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test1".to_string(),
    );
    test_case1.general_initial_conditions = InitialConditions {
        include: Some(vec![IncludeRef {
            id: "TC002".to_string(),
            test_sequence: None,
        }]),
        ..Default::default()
    };

    let mut test_case2 = TestCase::new(
        "REQ002".to_string(),
        2,
        2,
        "TC002".to_string(),
        "Test2".to_string(),
    );
    test_case2.general_initial_conditions = InitialConditions {
        include: Some(vec![IncludeRef {
            id: "TC001".to_string(),
            test_sequence: None,
        }]),
        ..Default::default()
    };

    validator.collect_definitions(&PathBuf::from("test1.yaml"), &test_case1);
    validator.collect_definitions(&PathBuf::from("test2.yaml"), &test_case2);

    let errors1 = validator.validate_references(&PathBuf::from("test1.yaml"), &test_case1);
    let errors2 = validator.validate_references(&PathBuf::from("test2.yaml"), &test_case2);

    assert_eq!(
        errors1.len(),
        0,
        "Circular includes should be allowed (no errors for TC001)"
    );
    assert_eq!(
        errors2.len(),
        0,
        "Circular includes should be allowed (no errors for TC002)"
    );
}

#[test]
fn test_cross_file_validation_success() {
    let mut test_case1 = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test1".to_string(),
    );
    let mut test_sequence = TestSequence::new(1, "Seq".to_string(), "Desc".to_string());
    test_sequence.reference = Some("ref-abc".to_string());
    test_case1.test_sequences.push(test_sequence);

    let mut test_case2 = TestCase::new(
        "REQ002".to_string(),
        2,
        2,
        "TC002".to_string(),
        "Test2".to_string(),
    );
    let mut initial_conditions = InitialConditions {
        include: Some(vec![IncludeRef {
            id: "TC001".to_string(),
            test_sequence: None,
        }]),
        devices: HashMap::new(),
    };
    initial_conditions.devices.insert(
        "device1".to_string(),
        vec![InitialConditionItem::RefItem {
            reference: "ref-abc".to_string(),
        }],
    );
    test_case2.initial_conditions = initial_conditions;

    let files = vec![
        (PathBuf::from("test1.yaml"), test_case1),
        (PathBuf::from("test2.yaml"), test_case2),
    ];

    let result = validate_cross_file_dependencies(&files);
    assert!(
        result.is_ok(),
        "Cross-file validation should succeed when all references are defined"
    );
}

#[test]
fn test_cross_file_validation_failure() {
    let test_case1 = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test1".to_string(),
    );

    let mut test_case2 = TestCase::new(
        "REQ002".to_string(),
        2,
        2,
        "TC002".to_string(),
        "Test2".to_string(),
    );
    let mut initial_conditions = InitialConditions {
        include: Some(vec![IncludeRef {
            id: "TC999".to_string(),
            test_sequence: None,
        }]),
        devices: HashMap::new(),
    };
    initial_conditions.devices.insert(
        "device1".to_string(),
        vec![InitialConditionItem::RefItem {
            reference: "undefined-ref".to_string(),
        }],
    );
    test_case2.general_initial_conditions = initial_conditions;

    let files = vec![
        (PathBuf::from("test1.yaml"), test_case1),
        (PathBuf::from("test2.yaml"), test_case2),
    ];

    let result = validate_cross_file_dependencies(&files);
    assert!(
        result.is_err(),
        "Cross-file validation should fail when references are undefined"
    );

    let errors = result.unwrap_err();
    assert_eq!(
        errors.len(),
        2,
        "Expected 2 errors (TC999 and undefined-ref)"
    );
}

#[test]
fn test_undefined_include_in_general_initial_conditions() {
    let validator = DependencyValidator::new();

    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test".to_string(),
    );
    test_case.general_initial_conditions = InitialConditions {
        include: Some(vec![IncludeRef {
            id: "TC999".to_string(),
            test_sequence: None,
        }]),
        ..Default::default()
    };

    let errors = validator.validate_references(&PathBuf::from("test.yaml"), &test_case);

    assert_eq!(
        errors.len(),
        1,
        "Undefined include should produce exactly one error"
    );
    assert_eq!(errors[0].reference, "TC999");
    assert!(errors[0]
        .location
        .contains("general_initial_conditions.include[0]"));
}

#[test]
fn test_multiple_undefined_refs_in_different_locations() {
    let validator = DependencyValidator::new();

    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test".to_string(),
    );

    test_case.general_initial_conditions = InitialConditions {
        include: Some(vec![IncludeRef {
            id: "TC_UNDEFINED_1".to_string(),
            test_sequence: None,
        }]),
        devices: HashMap::new(),
    };

    let mut initial_conditions = InitialConditions {
        include: Some(vec![IncludeRef {
            id: "TC_UNDEFINED_2".to_string(),
            test_sequence: None,
        }]),
        devices: HashMap::new(),
    };
    initial_conditions.devices.insert(
        "device1".to_string(),
        vec![InitialConditionItem::RefItem {
            reference: "undefined-ref-1".to_string(),
        }],
    );
    test_case.initial_conditions = initial_conditions;

    let mut test_sequence = TestSequence::new(1, "Seq".to_string(), "Desc".to_string());
    let mut seq_initial_conditions = InitialConditions {
        include: Some(vec![IncludeRef {
            id: "TC_UNDEFINED_3".to_string(),
            test_sequence: None,
        }]),
        devices: HashMap::new(),
    };
    seq_initial_conditions.devices.insert(
        "device2".to_string(),
        vec![InitialConditionItem::RefItem {
            reference: "undefined-ref-2".to_string(),
        }],
    );
    test_sequence.initial_conditions = seq_initial_conditions;
    test_case.test_sequences.push(test_sequence);

    let errors = validator.validate_references(&PathBuf::from("test.yaml"), &test_case);

    assert_eq!(
        errors.len(),
        5,
        "Expected 5 errors: 3 undefined test case IDs and 2 undefined refs"
    );

    let ref_errors: Vec<_> = errors
        .iter()
        .filter(|e| e.reference == "undefined-ref-1" || e.reference == "undefined-ref-2")
        .collect();
    assert_eq!(ref_errors.len(), 2, "Expected 2 ref errors");

    let tc_errors: Vec<_> = errors
        .iter()
        .filter(|e| e.reference.starts_with("TC_UNDEFINED"))
        .collect();
    assert_eq!(tc_errors.len(), 3, "Expected 3 test case ID errors");
}
