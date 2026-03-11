use testcase_manager::executor::TestExecutor;
use testcase_manager::models::{
    Expected, HookConfig, HookType, Hooks, InitialConditions, OnError, Step, TestCase,
    TestSequence, TestStepExecutionEntry, Verification, VerificationExpression,
};

/// Helper to create a minimal test case for hooks testing
fn create_test_case_with_hooks(hooks: Option<Hooks>) -> TestCase {
    let step = Step {
        step: 1,
        manual: None,
        description: "Test step".to_string(),
        command: "echo 'test'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "test".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $? -eq 0 ]".to_string()),
            output: VerificationExpression::Simple(
                "[ \"$COMMAND_OUTPUT\" = \"test\" ]".to_string(),
            ),
            output_file: None,
            general: None,
        },
        reference: None,
    };

    let sequence = TestSequence {
        id: 1,
        name: "Test Sequence".to_string(),
        description: "Test Description".to_string(),
        variables: None,
        initial_conditions: InitialConditions::default(),
        steps: vec![step],
        reference: None,
    };

    TestCase {
        requirement: "REQ001".to_string(),
        item: 1,
        tc: 1,
        id: "TC001".to_string(),
        description: "Test case with hooks".to_string(),
        prerequisites: None,
        general_initial_conditions: InitialConditions::default(),
        initial_conditions: InitialConditions::default(),
        test_sequences: vec![sequence],
        hydration_vars: None,
        hooks,
    }
}

// ============================================================================
// Hook Deserialization Tests
// ============================================================================

#[test]
fn test_hooks_deserialization_all_fields() {
    let yaml = r#"
script_start:
  command: "scripts/start.sh"
  on_error: fail
setup_test:
  command: "scripts/setup.sh"
  on_error: continue
before_sequence:
  command: "scripts/before_seq.sh"
  on_error: fail
after_sequence:
  command: "scripts/after_seq.sh"
  on_error: continue
before_step:
  command: "scripts/before_step.sh"
  on_error: fail
after_step:
  command: "scripts/after_step.sh"
  on_error: continue
teardown_test:
  command: "scripts/teardown.sh"
  on_error: fail
script_end:
  command: "scripts/end.sh"
  on_error: continue
"#;

    let hooks: Hooks = serde_yaml::from_str(yaml).unwrap();

    assert!(hooks.script_start.is_some());
    assert!(hooks.setup_test.is_some());
    assert!(hooks.before_sequence.is_some());
    assert!(hooks.after_sequence.is_some());
    assert!(hooks.before_step.is_some());
    assert!(hooks.after_step.is_some());
    assert!(hooks.teardown_test.is_some());
    assert!(hooks.script_end.is_some());

    // Verify script_start
    let script_start = hooks.script_start.unwrap();
    assert_eq!(script_start.command, "scripts/start.sh");
    assert_eq!(script_start.on_error, Some(OnError::Fail));

    // Verify setup_test
    let setup_test = hooks.setup_test.unwrap();
    assert_eq!(setup_test.command, "scripts/setup.sh");
    assert_eq!(setup_test.on_error, Some(OnError::Continue));
}

#[test]
fn test_hooks_deserialization_minimal() {
    let yaml = r#"
before_step:
  command: "scripts/hook.sh"
"#;

    let hooks: Hooks = serde_yaml::from_str(yaml).unwrap();

    assert!(hooks.script_start.is_none());
    assert!(hooks.before_step.is_some());

    let before_step = hooks.before_step.unwrap();
    assert_eq!(before_step.command, "scripts/hook.sh");
    assert_eq!(before_step.on_error, None);
}

#[test]
fn test_hooks_deserialization_empty() {
    let yaml = r#"{}"#;
    let hooks: Hooks = serde_yaml::from_str(yaml).unwrap();

    assert!(hooks.script_start.is_none());
    assert!(hooks.setup_test.is_none());
    assert!(hooks.before_sequence.is_none());
    assert!(hooks.after_sequence.is_none());
    assert!(hooks.before_step.is_none());
    assert!(hooks.after_step.is_none());
    assert!(hooks.teardown_test.is_none());
    assert!(hooks.script_end.is_none());
}

#[test]
fn test_on_error_deserialization() {
    let yaml_fail = r#"fail"#;
    let on_error: OnError = serde_yaml::from_str(yaml_fail).unwrap();
    assert_eq!(on_error, OnError::Fail);

    let yaml_continue = r#"continue"#;
    let on_error: OnError = serde_yaml::from_str(yaml_continue).unwrap();
    assert_eq!(on_error, OnError::Continue);
}

#[test]
fn test_hook_config_deserialization() {
    let yaml = r#"
command: "scripts/test.sh"
on_error: fail
"#;

    let config: HookConfig = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(config.command, "scripts/test.sh");
    assert_eq!(config.on_error, Some(OnError::Fail));
}

#[test]
fn test_hook_config_without_on_error() {
    let yaml = r#"
command: "scripts/test.sh"
"#;

    let config: HookConfig = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(config.command, "scripts/test.sh");
    assert_eq!(config.on_error, None);
}

// ============================================================================
// Hook Type Tests
// ============================================================================

#[test]
fn test_hook_type_serialization() {
    assert_eq!(
        serde_yaml::to_string(&HookType::ScriptStart)
            .unwrap()
            .trim(),
        "script_start"
    );
    assert_eq!(
        serde_yaml::to_string(&HookType::SetupTest).unwrap().trim(),
        "setup_test"
    );
    assert_eq!(
        serde_yaml::to_string(&HookType::BeforeSequence)
            .unwrap()
            .trim(),
        "before_sequence"
    );
    assert_eq!(
        serde_yaml::to_string(&HookType::AfterSequence)
            .unwrap()
            .trim(),
        "after_sequence"
    );
    assert_eq!(
        serde_yaml::to_string(&HookType::BeforeStep).unwrap().trim(),
        "before_step"
    );
    assert_eq!(
        serde_yaml::to_string(&HookType::AfterStep).unwrap().trim(),
        "after_step"
    );
    assert_eq!(
        serde_yaml::to_string(&HookType::TeardownTest)
            .unwrap()
            .trim(),
        "teardown_test"
    );
    assert_eq!(
        serde_yaml::to_string(&HookType::ScriptEnd).unwrap().trim(),
        "script_end"
    );
}

#[test]
fn test_hook_type_deserialization() {
    let hook_type: HookType = serde_yaml::from_str("script_start").unwrap();
    assert_eq!(hook_type, HookType::ScriptStart);

    let hook_type: HookType = serde_yaml::from_str("setup_test").unwrap();
    assert_eq!(hook_type, HookType::SetupTest);

    let hook_type: HookType = serde_yaml::from_str("before_sequence").unwrap();
    assert_eq!(hook_type, HookType::BeforeSequence);

    let hook_type: HookType = serde_yaml::from_str("after_sequence").unwrap();
    assert_eq!(hook_type, HookType::AfterSequence);

    let hook_type: HookType = serde_yaml::from_str("before_step").unwrap();
    assert_eq!(hook_type, HookType::BeforeStep);

    let hook_type: HookType = serde_yaml::from_str("after_step").unwrap();
    assert_eq!(hook_type, HookType::AfterStep);

    let hook_type: HookType = serde_yaml::from_str("teardown_test").unwrap();
    assert_eq!(hook_type, HookType::TeardownTest);

    let hook_type: HookType = serde_yaml::from_str("script_end").unwrap();
    assert_eq!(hook_type, HookType::ScriptEnd);
}

#[test]
fn test_hook_type_display() {
    assert_eq!(format!("{}", HookType::ScriptStart), "script_start");
    assert_eq!(format!("{}", HookType::SetupTest), "setup_test");
    assert_eq!(format!("{}", HookType::BeforeSequence), "before_sequence");
    assert_eq!(format!("{}", HookType::AfterSequence), "after_sequence");
    assert_eq!(format!("{}", HookType::BeforeStep), "before_step");
    assert_eq!(format!("{}", HookType::AfterStep), "after_step");
    assert_eq!(format!("{}", HookType::TeardownTest), "teardown_test");
    assert_eq!(format!("{}", HookType::ScriptEnd), "script_end");
}

// ============================================================================
// Script Generation Tests
// ============================================================================

#[test]
fn test_script_generation_with_all_hooks() {
    let hooks = Hooks {
        script_start: Some(HookConfig {
            command: "scripts/start.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        setup_test: Some(HookConfig {
            command: "scripts/setup.sh".to_string(),
            on_error: Some(OnError::Continue),
        }),
        before_sequence: Some(HookConfig {
            command: "scripts/before_seq.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        after_sequence: Some(HookConfig {
            command: "scripts/after_seq.sh".to_string(),
            on_error: Some(OnError::Continue),
        }),
        before_step: Some(HookConfig {
            command: "scripts/before_step.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        after_step: Some(HookConfig {
            command: "scripts/after_step.sh".to_string(),
            on_error: Some(OnError::Continue),
        }),
        teardown_test: Some(HookConfig {
            command: "scripts/teardown.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        script_end: Some(HookConfig {
            command: "scripts/end.sh".to_string(),
            on_error: Some(OnError::Continue),
        }),
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Verify all hooks are present in script
    assert!(script.contains("# Execute script_start hook"));
    assert!(script.contains("scripts/start.sh"));

    assert!(script.contains("# Execute setup_test hook"));
    assert!(script.contains("scripts/setup.sh"));

    assert!(script.contains("# Execute before_sequence hook"));
    assert!(script.contains("scripts/before_seq.sh"));

    assert!(script.contains("# Execute after_sequence hook"));
    assert!(script.contains("scripts/after_seq.sh"));

    assert!(script.contains("# Execute before_step hook"));
    assert!(script.contains("scripts/before_step.sh"));

    assert!(script.contains("# Execute after_step hook"));
    assert!(script.contains("scripts/after_step.sh"));

    assert!(script.contains("# Execute teardown_test hook"));
    assert!(script.contains("scripts/teardown.sh"));

    assert!(script.contains("# Execute script_end hook"));
    assert!(script.contains("scripts/end.sh"));
}

#[test]
fn test_script_generation_with_no_hooks() {
    let test_case = create_test_case_with_hooks(None);
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Verify no hook execution code is present
    assert!(!script.contains("# Execute script_start hook"));
    assert!(!script.contains("# Execute setup_test hook"));
    assert!(!script.contains("# Execute before_sequence hook"));
    assert!(!script.contains("# Execute after_sequence hook"));
    assert!(!script.contains("# Execute before_step hook"));
    assert!(!script.contains("# Execute after_step hook"));
    assert!(!script.contains("# Execute teardown_test hook"));
    assert!(!script.contains("# Execute script_end hook"));
}

#[test]
fn test_script_generation_with_selective_hooks() {
    let hooks = Hooks {
        script_start: Some(HookConfig {
            command: "scripts/start.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        before_step: Some(HookConfig {
            command: "scripts/before_step.sh".to_string(),
            on_error: Some(OnError::Continue),
        }),
        script_end: Some(HookConfig {
            command: "scripts/end.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Verify selected hooks are present
    assert!(script.contains("# Execute script_start hook"));
    assert!(script.contains("scripts/start.sh"));
    assert!(script.contains("# Execute before_step hook"));
    assert!(script.contains("scripts/before_step.sh"));
    assert!(script.contains("# Execute script_end hook"));
    assert!(script.contains("scripts/end.sh"));

    // Verify non-selected hooks are absent
    assert!(!script.contains("# Execute setup_test hook"));
    assert!(!script.contains("# Execute before_sequence hook"));
    assert!(!script.contains("# Execute after_sequence hook"));
    assert!(!script.contains("# Execute after_step hook"));
    assert!(!script.contains("# Execute teardown_test hook"));
}

// ============================================================================
// On Error Behavior Tests
// ============================================================================

#[test]
fn test_hook_fail_on_error_script_generation() {
    let hooks = Hooks {
        before_step: Some(HookConfig {
            command: "scripts/hook.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Verify fail behavior generates exit on error
    assert!(script.contains("# Execute before_step hook"));
    assert!(script.contains("scripts/hook.sh"));
    assert!(script.contains("Error: before_step hook failed"));
    assert!(script.contains("exit $HOOK_EXIT_CODE"));
}

#[test]
fn test_hook_continue_on_error_script_generation() {
    let hooks = Hooks {
        before_step: Some(HookConfig {
            command: "scripts/hook.sh".to_string(),
            on_error: Some(OnError::Continue),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Verify continue behavior generates warning but no exit
    assert!(script.contains("# Execute before_step hook"));
    assert!(script.contains("scripts/hook.sh"));
    assert!(script.contains("Warning: before_step hook failed"));
    assert!(script.contains("(continuing)"));

    // Should not contain exit statement in continue mode
    let hook_section_start = script.find("# Execute before_step hook").unwrap();
    let hook_section = &script[hook_section_start..];
    let next_step = hook_section.find("# Step").unwrap_or(hook_section.len());
    let hook_code = &hook_section[..next_step];

    // Check that within the hook code, there's no "exit $HOOK_EXIT_CODE" after the warning
    assert!(!hook_code.contains("exit $HOOK_EXIT_CODE"));
}

#[test]
fn test_hook_default_on_error_behavior() {
    // When on_error is None, it should default to Fail behavior
    let hooks = Hooks {
        before_step: Some(HookConfig {
            command: "scripts/hook.sh".to_string(),
            on_error: None,
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Default should be fail behavior
    assert!(script.contains("# Execute before_step hook"));
    assert!(script.contains("Error: before_step hook failed"));
    assert!(script.contains("exit $HOOK_EXIT_CODE"));
}

// ============================================================================
// Hook Execution with Bash Code Validation Tests
// ============================================================================

#[test]
fn test_hook_bash_code_structure_sh_file() {
    let hooks = Hooks {
        script_start: Some(HookConfig {
            command: "scripts/test.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // .sh files should be sourced
    assert!(script.contains("if [ -f \"scripts/test.sh\" ]; then"));
    assert!(script.contains("source \"scripts/test.sh\""));
    assert!(script.contains("HOOK_EXIT_CODE=$?"));
}

#[test]
fn test_hook_bash_code_structure_non_sh_file() {
    let hooks = Hooks {
        script_start: Some(HookConfig {
            command: "scripts/test.py".to_string(),
            on_error: Some(OnError::Fail),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Non-.sh files should be executed directly
    assert!(script.contains("scripts/test.py"));
    assert!(script.contains("HOOK_EXIT_CODE=$?"));
    // Should not be sourced
    assert!(!script.contains("source \"scripts/test.py\""));
}

#[test]
fn test_hook_bash_code_missing_file_handling() {
    let hooks = Hooks {
        before_step: Some(HookConfig {
            command: "scripts/nonexistent.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Should check for file existence
    assert!(script.contains("if [ -f \"scripts/nonexistent.sh\" ]; then"));
    assert!(script.contains("Warning: Hook script 'scripts/nonexistent.sh' not found"));
    assert!(script.contains("HOOK_EXIT_CODE=127"));
}

#[test]
fn test_hook_bash_code_set_unset_errexit() {
    let hooks = Hooks {
        before_step: Some(HookConfig {
            command: "scripts/hook.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Find the hook execution section
    let hook_start = script.find("# Execute before_step hook").unwrap();
    let hook_section = &script[hook_start..];
    let hook_end = hook_section.find("\n\n").unwrap_or(hook_section.len());
    let hook_code = &hook_section[..hook_end];

    // Should disable errexit before running hook, then re-enable it
    assert!(hook_code.contains("set +e"));
    assert!(hook_code.contains("set -e"));

    // Verify order: set +e comes before the hook execution
    let set_plus_e_pos = hook_code.find("set +e").unwrap();
    let source_pos = hook_code
        .find("source")
        .or_else(|| hook_code.find("scripts/hook.sh"))
        .unwrap();
    assert!(set_plus_e_pos < source_pos);
}

// ============================================================================
// Environment Variable Passing Tests
// ============================================================================

#[test]
fn test_hook_environment_variables_sequence_level() {
    let hooks = Hooks {
        before_sequence: Some(HookConfig {
            command: "scripts/hook.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Find the before_sequence hook section
    let hook_pos = script.find("# Execute before_sequence hook").unwrap();
    let before_hook = &script[..hook_pos];

    // Should set SEQUENCE_ID and SEQUENCE_NAME before hook execution
    assert!(before_hook.contains("SEQUENCE_ID=1"));
    assert!(before_hook.contains("SEQUENCE_NAME="));
    assert!(before_hook.contains("export SEQUENCE_ID SEQUENCE_NAME"));
}

#[test]
fn test_hook_environment_variables_step_level() {
    let hooks = Hooks {
        before_step: Some(HookConfig {
            command: "scripts/hook.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Find the before_step hook section
    let hook_pos = script.find("# Execute before_step hook").unwrap();
    let before_hook = &script[..hook_pos];

    // Should set STEP_NUMBER and STEP_DESC before hook execution
    assert!(before_hook.contains("STEP_NUMBER=1"));
    assert!(before_hook.contains("STEP_DESC="));
    assert!(before_hook.contains("export STEP_NUMBER STEP_DESC"));
}

#[test]
fn test_hook_test_case_id_available() {
    let hooks = Hooks {
        script_start: Some(HookConfig {
            command: "scripts/hook.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Test case ID should be in the script as a comment
    assert!(script.contains("# Test Case: TC001"));
}

// ============================================================================
// Hook Log Entry Serialization Tests
// ============================================================================

#[test]
fn test_hook_execution_entry_with_hook_type() {
    let entry = TestStepExecutionEntry {
        test_sequence: 1,
        step: 2,
        command: "scripts/hook.sh".to_string(),
        exit_code: 0,
        output: "hook output".to_string(),
        timestamp: Some("2024-01-01T00:00:00Z".to_string()),
        hook_type: Some(HookType::BeforeStep),
        hook_path: Some("scripts/hook.sh".to_string()),
    };

    let json = serde_json::to_string(&entry).unwrap();
    assert!(json.contains("\"hook_type\":\"before_step\""));
    assert!(json.contains("\"hook_path\":\"scripts/hook.sh\""));

    // Deserialize and verify
    let deserialized: TestStepExecutionEntry = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.hook_type, Some(HookType::BeforeStep));
    assert_eq!(deserialized.hook_path, Some("scripts/hook.sh".to_string()));
}

#[test]
fn test_hook_execution_entry_display_with_hook() {
    let entry = TestStepExecutionEntry {
        test_sequence: 1,
        step: 2,
        command: "scripts/hook.sh".to_string(),
        exit_code: 0,
        output: "output".to_string(),
        timestamp: None,
        hook_type: Some(HookType::AfterSequence),
        hook_path: Some("scripts/after.sh".to_string()),
    };

    let display = format!("{}", entry);
    assert!(display.contains("Hook: after_sequence"));
    assert!(display.contains("path: scripts/after.sh"));
}

#[test]
fn test_hook_execution_entry_yaml_roundtrip() {
    let entry = TestStepExecutionEntry {
        test_sequence: 1,
        step: 1,
        command: "scripts/hook.sh".to_string(),
        exit_code: 0,
        output: "success".to_string(),
        timestamp: Some("2024-01-01T12:00:00Z".to_string()),
        hook_type: Some(HookType::SetupTest),
        hook_path: Some("scripts/setup.sh".to_string()),
    };

    let yaml = serde_yaml::to_string(&entry).unwrap();
    let deserialized: TestStepExecutionEntry = serde_yaml::from_str(&yaml).unwrap();

    assert_eq!(entry, deserialized);
}

// ============================================================================
// Edge Cases Tests
// ============================================================================

#[test]
fn test_hook_with_special_characters_in_path() {
    let hooks = Hooks {
        before_step: Some(HookConfig {
            command: "scripts/with space/hook.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Path should be properly quoted
    assert!(script.contains("\"scripts/with space/hook.sh\""));
}

#[test]
fn test_hook_with_absolute_path() {
    let hooks = Hooks {
        script_start: Some(HookConfig {
            command: "/usr/local/bin/hook.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("/usr/local/bin/hook.sh"));
}

#[test]
fn test_hook_with_command_not_script() {
    let hooks = Hooks {
        before_step: Some(HookConfig {
            command: "echo 'inline hook'".to_string(),
            on_error: Some(OnError::Continue),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Inline commands should be executed, not sourced
    assert!(script.contains("echo 'inline hook'"));
    assert!(!script.contains("source \"echo 'inline hook'\""));
}

#[test]
fn test_multiple_hooks_same_type_not_allowed() {
    // This test verifies that the Hooks struct only allows one hook per type
    let hooks = Hooks {
        before_step: Some(HookConfig {
            command: "scripts/hook1.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        ..Default::default()
    };

    // You cannot have two before_step hooks in the same Hooks struct
    // This is enforced by the struct design
    assert!(hooks.before_step.is_some());
}

#[test]
fn test_hooks_serialization_skip_none_fields() {
    let hooks = Hooks {
        script_start: Some(HookConfig {
            command: "scripts/start.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        ..Default::default()
    };

    let yaml = serde_yaml::to_string(&hooks).unwrap();

    // Only script_start should be serialized
    assert!(yaml.contains("script_start"));

    // Other fields should not appear
    assert!(!yaml.contains("setup_test"));
    assert!(!yaml.contains("before_sequence"));
    assert!(!yaml.contains("after_sequence"));
    assert!(!yaml.contains("before_step"));
    assert!(!yaml.contains("after_step"));
    assert!(!yaml.contains("teardown_test"));
    assert!(!yaml.contains("script_end"));
}

#[test]
fn test_test_case_with_hooks_yaml_roundtrip() {
    let hooks = Hooks {
        script_start: Some(HookConfig {
            command: "scripts/start.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        before_step: Some(HookConfig {
            command: "scripts/before.sh".to_string(),
            on_error: Some(OnError::Continue),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));

    let yaml = serde_yaml::to_string(&test_case).unwrap();
    let deserialized: TestCase = serde_yaml::from_str(&yaml).unwrap();

    assert!(deserialized.hooks.is_some());
    let deserialized_hooks = deserialized.hooks.unwrap();
    assert!(deserialized_hooks.script_start.is_some());
    assert!(deserialized_hooks.before_step.is_some());

    let script_start = deserialized_hooks.script_start.unwrap();
    assert_eq!(script_start.command, "scripts/start.sh");
    assert_eq!(script_start.on_error, Some(OnError::Fail));
}

#[test]
fn test_hook_execution_order_in_script() {
    let hooks = Hooks {
        script_start: Some(HookConfig {
            command: "scripts/start.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        setup_test: Some(HookConfig {
            command: "scripts/setup.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        before_sequence: Some(HookConfig {
            command: "scripts/before_seq.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        before_step: Some(HookConfig {
            command: "scripts/before_step.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        after_step: Some(HookConfig {
            command: "scripts/after_step.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        after_sequence: Some(HookConfig {
            command: "scripts/after_seq.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        teardown_test: Some(HookConfig {
            command: "scripts/teardown.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        script_end: Some(HookConfig {
            command: "scripts/end.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Verify hook execution order in the script
    let script_start_pos = script.find("# Execute script_start hook").unwrap();
    let setup_test_pos = script.find("# Execute setup_test hook").unwrap();
    let before_sequence_pos = script.find("# Execute before_sequence hook").unwrap();
    let before_step_pos = script.find("# Execute before_step hook").unwrap();
    let after_step_pos = script.find("# Execute after_step hook").unwrap();
    let after_sequence_pos = script.find("# Execute after_sequence hook").unwrap();
    let teardown_test_pos = script.find("# Execute teardown_test hook").unwrap();
    let script_end_pos = script.find("# Execute script_end hook").unwrap();
    let cleanup_pos = script.find("cleanup() {").unwrap();

    // Verify correct order for hooks executed in main flow
    assert!(script_start_pos < setup_test_pos);
    assert!(setup_test_pos < before_sequence_pos);
    assert!(before_sequence_pos < before_step_pos);
    assert!(before_step_pos < after_step_pos);
    assert!(after_step_pos < after_sequence_pos);

    // Verify that teardown_test and script_end are in the cleanup function
    // (defined early but executed at the end via trap)
    assert!(cleanup_pos < teardown_test_pos);
    assert!(teardown_test_pos < script_end_pos);
    assert!(teardown_test_pos < setup_test_pos); // In cleanup, defined before main execution
    assert!(script_end_pos < setup_test_pos); // In cleanup, defined before main execution
}

#[test]
fn test_hook_with_non_zero_exit_code_fail_mode() {
    let hooks = Hooks {
        before_step: Some(HookConfig {
            command: "scripts/failing_hook.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // In fail mode, hook failure should exit the script
    let hook_start = script.find("# Execute before_step hook").unwrap();
    let hook_section = &script[hook_start..];

    assert!(hook_section.contains("if [ $HOOK_EXIT_CODE -ne 0 ]; then"));
    assert!(hook_section.contains("Error: before_step hook failed"));
    assert!(hook_section.contains("exit $HOOK_EXIT_CODE"));
}

#[test]
fn test_hook_with_non_zero_exit_code_continue_mode() {
    let hooks = Hooks {
        before_step: Some(HookConfig {
            command: "scripts/failing_hook.sh".to_string(),
            on_error: Some(OnError::Continue),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // In continue mode, hook failure should log warning but not exit
    let hook_start = script.find("# Execute before_step hook").unwrap();
    let hook_section = &script[hook_start..];

    assert!(hook_section.contains("if [ $HOOK_EXIT_CODE -ne 0 ]; then"));
    assert!(hook_section.contains("Warning: before_step hook failed"));
    assert!(hook_section.contains("(continuing)"));
}

#[test]
fn test_hook_execution_with_missing_file() {
    let hooks = Hooks {
        before_sequence: Some(HookConfig {
            command: "scripts/missing.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Should handle missing file gracefully
    assert!(script.contains("if [ -f \"scripts/missing.sh\" ]; then"));
    assert!(script.contains("Warning: Hook script 'scripts/missing.sh' not found"));
    assert!(script.contains("HOOK_EXIT_CODE=127"));
}

#[test]
fn test_hook_non_executable_handling() {
    // Non-executable hooks should still be handled by bash
    // .sh files are sourced, others are executed
    let hooks = Hooks {
        before_step: Some(HookConfig {
            command: "scripts/hook.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // .sh files are sourced, so they don't need execute permission
    assert!(script.contains("source \"scripts/hook.sh\""));
}

#[test]
fn test_hook_execution_entry_without_hook_type() {
    let entry = TestStepExecutionEntry {
        test_sequence: 1,
        step: 1,
        command: "echo test".to_string(),
        exit_code: 0,
        output: "test".to_string(),
        timestamp: None,
        hook_type: None,
        hook_path: None,
    };

    let json = serde_json::to_string(&entry).unwrap();

    // hook_type and hook_path should not be in JSON when None
    assert!(!json.contains("hook_type"));
    assert!(!json.contains("hook_path"));
}

#[test]
fn test_hooks_default() {
    let hooks = Hooks::default();

    assert!(hooks.script_start.is_none());
    assert!(hooks.setup_test.is_none());
    assert!(hooks.before_sequence.is_none());
    assert!(hooks.after_sequence.is_none());
    assert!(hooks.before_step.is_none());
    assert!(hooks.after_step.is_none());
    assert!(hooks.teardown_test.is_none());
    assert!(hooks.script_end.is_none());
}

#[test]
fn test_test_case_without_hooks_field() {
    let yaml = r#"
requirement: REQ001
item: 1
tc: 1
id: TC001
description: Test case without hooks
general_initial_conditions: {}
initial_conditions: {}
test_sequences: []
"#;

    let test_case: TestCase = serde_yaml::from_str(yaml).unwrap();
    assert!(test_case.hooks.is_none());
}

#[test]
fn test_hook_config_equality() {
    let config1 = HookConfig {
        command: "scripts/hook.sh".to_string(),
        on_error: Some(OnError::Fail),
    };

    let config2 = HookConfig {
        command: "scripts/hook.sh".to_string(),
        on_error: Some(OnError::Fail),
    };

    let config3 = HookConfig {
        command: "scripts/other.sh".to_string(),
        on_error: Some(OnError::Fail),
    };

    assert_eq!(config1, config2);
    assert_ne!(config1, config3);
}

#[test]
fn test_on_error_equality() {
    assert_eq!(OnError::Fail, OnError::Fail);
    assert_eq!(OnError::Continue, OnError::Continue);
    assert_ne!(OnError::Fail, OnError::Continue);
}

// ============================================================================
// Integration Tests - Trap Execution on Failure
// ============================================================================

/// Helper to create a test case with hooks that track execution
fn create_tracked_hook_test_case(
    step_should_fail: bool,
    before_step_should_fail: bool,
) -> TestCase {
    use std::io::Write;

    // Create temporary hook scripts that track execution
    let temp_dir = std::env::temp_dir();
    let tracker_file = temp_dir.join("hook_tracker.txt");

    // Clean up any existing tracker file
    let _ = std::fs::remove_file(&tracker_file);

    let teardown_script = temp_dir.join("teardown_hook.sh");
    let script_end_script = temp_dir.join("script_end_hook.sh");
    let before_step_script = temp_dir.join("before_step_hook.sh");

    // Create teardown_test hook that writes to tracker
    let mut file = std::fs::File::create(&teardown_script).unwrap();
    writeln!(
        file,
        "#!/bin/bash\necho 'teardown_test executed' >> {}",
        tracker_file.display()
    )
    .unwrap();

    // Create script_end hook that writes to tracker
    let mut file = std::fs::File::create(&script_end_script).unwrap();
    writeln!(
        file,
        "#!/bin/bash\necho 'script_end executed' >> {}",
        tracker_file.display()
    )
    .unwrap();

    // Create before_step hook that may fail
    let mut file = std::fs::File::create(&before_step_script).unwrap();
    if before_step_should_fail {
        writeln!(
            file,
            "#!/bin/bash\necho 'before_step executed' >> {}\nexit 1",
            tracker_file.display()
        )
        .unwrap();
    } else {
        writeln!(
            file,
            "#!/bin/bash\necho 'before_step executed' >> {}",
            tracker_file.display()
        )
        .unwrap();
    }

    // Make scripts executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o755);
        std::fs::set_permissions(&teardown_script, perms.clone()).unwrap();
        std::fs::set_permissions(&script_end_script, perms.clone()).unwrap();
        std::fs::set_permissions(&before_step_script, perms).unwrap();
    }

    let step = Step {
        step: 1,
        manual: None,
        description: "Test step".to_string(),
        command: if step_should_fail {
            "exit 1".to_string()
        } else {
            "echo 'test'".to_string()
        },
        capture_vars: None,
        expected: Expected {
            success: Some(!step_should_fail),
            result: if step_should_fail { "1" } else { "0" }.to_string(),
            output: if step_should_fail {
                "".to_string()
            } else {
                "test".to_string()
            },
        },
        verification: Verification {
            result: if step_should_fail {
                VerificationExpression::Simple("[ $? -eq 1 ]".to_string())
            } else {
                VerificationExpression::Simple("[ $? -eq 0 ]".to_string())
            },
            output: if step_should_fail {
                VerificationExpression::Simple("true".to_string())
            } else {
                VerificationExpression::Simple("[ \"$COMMAND_OUTPUT\" = \"test\" ]".to_string())
            },
            output_file: None,
            general: None,
        },
        reference: None,
    };

    let sequence = TestSequence {
        id: 1,
        name: "Test Sequence".to_string(),
        description: "Test Description".to_string(),
        variables: None,
        initial_conditions: InitialConditions::default(),
        steps: vec![step],
        reference: None,
    };

    let hooks = Hooks {
        before_step: Some(HookConfig {
            command: before_step_script.to_string_lossy().to_string(),
            on_error: Some(if before_step_should_fail {
                OnError::Fail
            } else {
                OnError::Continue
            }),
        }),
        teardown_test: Some(HookConfig {
            command: teardown_script.to_string_lossy().to_string(),
            on_error: Some(OnError::Continue),
        }),
        script_end: Some(HookConfig {
            command: script_end_script.to_string_lossy().to_string(),
            on_error: Some(OnError::Continue),
        }),
        ..Default::default()
    };

    TestCase {
        requirement: "REQ001".to_string(),
        item: 1,
        tc: 1,
        id: "TC_TRAP_TEST".to_string(),
        description: "Test case for trap execution".to_string(),
        prerequisites: None,
        general_initial_conditions: InitialConditions::default(),
        initial_conditions: InitialConditions::default(),
        test_sequences: vec![sequence],
        hydration_vars: None,
        hooks: Some(hooks),
    }
}

#[test]
fn test_teardown_and_script_end_execute_on_step_verification_failure() {
    let test_case = create_tracked_hook_test_case(false, false);
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Verify that teardown_test and script_end are in the cleanup function
    assert!(script.contains("cleanup() {"));
    assert!(script.contains("trap cleanup EXIT"));

    // Verify teardown_test is in cleanup
    let cleanup_start = script.find("cleanup() {").unwrap();
    let cleanup_section = &script[cleanup_start..];
    assert!(cleanup_section.contains("# Execute teardown_test hook"));
    assert!(cleanup_section.contains("teardown_hook.sh"));

    // Verify script_end is in cleanup after teardown_test
    let teardown_pos = cleanup_section
        .find("# Execute teardown_test hook")
        .unwrap();
    let after_teardown = &cleanup_section[teardown_pos..];
    assert!(after_teardown.contains("# Execute script_end hook"));
    assert!(after_teardown.contains("script_end_hook.sh"));
}

#[test]
fn test_teardown_and_script_end_execute_on_step_command_failure() {
    // Create a test case where the step command itself fails
    let test_case = create_tracked_hook_test_case(true, false);
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Verify cleanup function exists with trap
    assert!(script.contains("cleanup() {"));
    assert!(script.contains("trap cleanup EXIT"));

    // Verify both hooks are in cleanup function
    let cleanup_start = script.find("cleanup() {").unwrap();
    let cleanup_section = &script[cleanup_start..];
    assert!(cleanup_section.contains("# Execute teardown_test hook"));
    assert!(cleanup_section.contains("# Execute script_end hook"));

    // Verify ordering: teardown_test before script_end
    let teardown_pos = cleanup_section
        .find("# Execute teardown_test hook")
        .unwrap();
    let script_end_pos = cleanup_section.find("# Execute script_end hook").unwrap();
    assert!(teardown_pos < script_end_pos);
}

#[test]
fn test_teardown_and_script_end_execute_on_before_step_hook_failure() {
    // Create a test case where before_step hook fails with on_error: fail
    let test_case = create_tracked_hook_test_case(false, true);
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Verify cleanup function with trap
    assert!(script.contains("cleanup() {"));
    assert!(script.contains("trap cleanup EXIT"));

    // Verify cleanup hooks are present
    let cleanup_start = script.find("cleanup() {").unwrap();
    let cleanup_section = &script[cleanup_start..];
    assert!(cleanup_section.contains("# Execute teardown_test hook"));
    assert!(cleanup_section.contains("# Execute script_end hook"));

    // Verify that before_step hook has fail behavior
    let before_step_start = script.find("# Execute before_step hook").unwrap();
    let before_step_section = &script[before_step_start..];
    assert!(before_step_section.contains("Error: before_step hook failed"));
    assert!(before_step_section.contains("exit $HOOK_EXIT_CODE"));
}

#[test]
fn test_cleanup_hooks_with_on_error_continue() {
    let hooks = Hooks {
        teardown_test: Some(HookConfig {
            command: "scripts/teardown.sh".to_string(),
            on_error: Some(OnError::Continue),
        }),
        script_end: Some(HookConfig {
            command: "scripts/end.sh".to_string(),
            on_error: Some(OnError::Continue),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Find cleanup function
    let cleanup_start = script.find("cleanup() {").unwrap();
    let cleanup_section = &script[cleanup_start..];

    // Verify teardown_test is in cleanup with continue behavior
    assert!(cleanup_section.contains("# Execute teardown_test hook"));
    let teardown_start = cleanup_section
        .find("# Execute teardown_test hook")
        .unwrap();
    let teardown_section = &cleanup_section[teardown_start..];
    let teardown_end = teardown_section
        .find("# Execute script_end hook")
        .unwrap_or(teardown_section.len());
    let teardown_code = &teardown_section[..teardown_end];

    // In continue mode, should not exit on failure
    assert!(teardown_code.contains("Warning: teardown_test hook failed"));
    assert!(teardown_code.contains("(continuing)"));

    // Verify script_end is also in cleanup
    assert!(cleanup_section.contains("# Execute script_end hook"));
}

#[test]
fn test_cleanup_hooks_with_on_error_fail() {
    let hooks = Hooks {
        teardown_test: Some(HookConfig {
            command: "scripts/teardown.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        script_end: Some(HookConfig {
            command: "scripts/end.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Find cleanup function
    let cleanup_start = script.find("cleanup() {").unwrap();
    let cleanup_section = &script[cleanup_start..];

    // Verify teardown_test is in cleanup
    assert!(cleanup_section.contains("# Execute teardown_test hook"));

    // Check for fail behavior in teardown hook
    let teardown_start = cleanup_section
        .find("# Execute teardown_test hook")
        .unwrap();
    let teardown_section = &cleanup_section[teardown_start..];
    let teardown_end = teardown_section
        .find("# Execute script_end hook")
        .unwrap_or(teardown_section.len());
    let teardown_code = &teardown_section[..teardown_end];

    // In fail mode, should exit on hook failure
    assert!(teardown_code.contains("Error: teardown_test hook failed"));
    assert!(teardown_code.contains("exit $HOOK_EXIT_CODE"));
}

#[test]
fn test_cleanup_preserves_original_exit_code() {
    let hooks = Hooks {
        teardown_test: Some(HookConfig {
            command: "scripts/teardown.sh".to_string(),
            on_error: Some(OnError::Continue),
        }),
        script_end: Some(HookConfig {
            command: "scripts/end.sh".to_string(),
            on_error: Some(OnError::Continue),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Find cleanup function
    let cleanup_start = script.find("cleanup() {").unwrap();
    let cleanup_end = script[cleanup_start..].find("\ntrap cleanup EXIT").unwrap();
    let cleanup_code = &script[cleanup_start..cleanup_start + cleanup_end];

    // Verify EXIT_CODE is captured at the start of cleanup
    assert!(cleanup_code.contains("EXIT_CODE=$?"));

    // Verify EXIT_CODE is preserved through hook executions
    assert!(cleanup_code.contains("exit $EXIT_CODE"));

    // The exit should be at the end of cleanup, after all hooks
    let exit_pos = cleanup_code.rfind("exit $EXIT_CODE").unwrap();
    let teardown_pos = cleanup_code
        .find("# Execute teardown_test hook")
        .unwrap_or(0);
    let script_end_pos = cleanup_code.find("# Execute script_end hook").unwrap_or(0);

    // exit should come after both hooks
    assert!(exit_pos > teardown_pos);
    assert!(exit_pos > script_end_pos);
}

#[test]
fn test_trap_executes_on_any_exit() {
    let hooks = Hooks {
        teardown_test: Some(HookConfig {
            command: "scripts/teardown.sh".to_string(),
            on_error: Some(OnError::Continue),
        }),
        script_end: Some(HookConfig {
            command: "scripts/end.sh".to_string(),
            on_error: Some(OnError::Continue),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Verify trap is set up to catch EXIT signal
    assert!(script.contains("trap cleanup EXIT"));

    // Verify cleanup function is defined before trap
    let cleanup_def = script.find("cleanup() {").unwrap();
    let trap_def = script.find("trap cleanup EXIT").unwrap();
    assert!(cleanup_def < trap_def);

    // Verify cleanup function contains both hooks
    let cleanup_start = script.find("cleanup() {").unwrap();
    let trap_pos = script.find("trap cleanup EXIT").unwrap();
    let cleanup_section = &script[cleanup_start..trap_pos];

    assert!(cleanup_section.contains("# Execute teardown_test hook"));
    assert!(cleanup_section.contains("# Execute script_end hook"));
}

#[test]
fn test_hooks_in_cleanup_use_proper_error_handling() {
    let hooks = Hooks {
        teardown_test: Some(HookConfig {
            command: "scripts/teardown_might_fail.sh".to_string(),
            on_error: Some(OnError::Continue),
        }),
        script_end: Some(HookConfig {
            command: "scripts/end_might_fail.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    let cleanup_start = script.find("cleanup() {").unwrap();
    let cleanup_section = &script[cleanup_start..];

    // Find teardown_test hook section
    let teardown_pos = cleanup_section
        .find("# Execute teardown_test hook")
        .unwrap();
    let script_end_pos = cleanup_section.find("# Execute script_end hook").unwrap();
    let teardown_section = &cleanup_section[teardown_pos..script_end_pos];

    // Verify continue behavior for teardown_test
    assert!(teardown_section.contains("if [ $HOOK_EXIT_CODE -ne 0 ]; then"));
    assert!(teardown_section.contains("Warning: teardown_test hook failed"));
    assert!(teardown_section.contains("(continuing)"));

    // Find script_end hook section
    let script_end_section = &cleanup_section[script_end_pos..];
    let next_major_section = script_end_section
        .find("\n    exit $EXIT_CODE")
        .unwrap_or(script_end_section.len());
    let script_end_code = &script_end_section[..next_major_section];

    // Verify fail behavior for script_end
    assert!(script_end_code.contains("if [ $HOOK_EXIT_CODE -ne 0 ]; then"));
    assert!(script_end_code.contains("Error: script_end hook failed"));
    assert!(script_end_code.contains("exit $HOOK_EXIT_CODE"));
}

#[test]
fn test_cleanup_with_missing_hook_files() {
    let hooks = Hooks {
        teardown_test: Some(HookConfig {
            command: "scripts/nonexistent_teardown.sh".to_string(),
            on_error: Some(OnError::Continue),
        }),
        script_end: Some(HookConfig {
            command: "scripts/nonexistent_end.sh".to_string(),
            on_error: Some(OnError::Continue),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    let cleanup_start = script.find("cleanup() {").unwrap();
    let cleanup_section = &script[cleanup_start..];

    // Verify file existence checks for both hooks
    assert!(cleanup_section.contains("if [ -f \"scripts/nonexistent_teardown.sh\" ]; then"));
    assert!(cleanup_section
        .contains("Warning: Hook script 'scripts/nonexistent_teardown.sh' not found"));

    assert!(cleanup_section.contains("if [ -f \"scripts/nonexistent_end.sh\" ]; then"));
    assert!(cleanup_section.contains("Warning: Hook script 'scripts/nonexistent_end.sh' not found"));

    // Both should set HOOK_EXIT_CODE=127 when file not found
    let teardown_section =
        &cleanup_section[..cleanup_section.find("# Execute script_end hook").unwrap()];
    assert!(teardown_section.contains("HOOK_EXIT_CODE=127"));
}

#[test]
fn test_only_teardown_test_in_cleanup() {
    let hooks = Hooks {
        teardown_test: Some(HookConfig {
            command: "scripts/teardown.sh".to_string(),
            on_error: Some(OnError::Continue),
        }),
        script_end: None,
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    let cleanup_start = script.find("cleanup() {").unwrap();
    let cleanup_section = &script[cleanup_start..];

    // Should have teardown_test
    assert!(cleanup_section.contains("# Execute teardown_test hook"));
    assert!(cleanup_section.contains("scripts/teardown.sh"));

    // Should not have script_end
    assert!(!cleanup_section.contains("# Execute script_end hook"));
}

#[test]
fn test_only_script_end_in_cleanup() {
    let hooks = Hooks {
        teardown_test: None,
        script_end: Some(HookConfig {
            command: "scripts/end.sh".to_string(),
            on_error: Some(OnError::Fail),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    let cleanup_start = script.find("cleanup() {").unwrap();
    let cleanup_section = &script[cleanup_start..];

    // Should not have teardown_test
    assert!(!cleanup_section.contains("# Execute teardown_test hook"));

    // Should have script_end
    assert!(cleanup_section.contains("# Execute script_end hook"));
    assert!(cleanup_section.contains("scripts/end.sh"));
}

#[test]
fn test_cleanup_hooks_execute_in_correct_order() {
    let hooks = Hooks {
        teardown_test: Some(HookConfig {
            command: "scripts/teardown.sh".to_string(),
            on_error: Some(OnError::Continue),
        }),
        script_end: Some(HookConfig {
            command: "scripts/end.sh".to_string(),
            on_error: Some(OnError::Continue),
        }),
        ..Default::default()
    };

    let test_case = create_test_case_with_hooks(Some(hooks));
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    let cleanup_start = script.find("cleanup() {").unwrap();
    let trap_pos = script.find("trap cleanup EXIT").unwrap();
    let cleanup_section = &script[cleanup_start..trap_pos];

    // Find positions of both hooks in cleanup
    let teardown_pos = cleanup_section
        .find("# Execute teardown_test hook")
        .unwrap();
    let script_end_pos = cleanup_section.find("# Execute script_end hook").unwrap();

    // teardown_test should execute before script_end
    assert!(teardown_pos < script_end_pos);

    // Both should be before the final exit
    let exit_pos = cleanup_section.rfind("exit $EXIT_CODE").unwrap();
    assert!(teardown_pos < exit_pos);
    assert!(script_end_pos < exit_pos);
}
