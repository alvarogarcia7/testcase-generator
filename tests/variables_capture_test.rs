use std::collections::BTreeMap;
use testcase_manager::models::{CaptureVar, CaptureVarsFormat, Expected, Step};

// ============================================================================
// CaptureVar Tests - Basic Creation and Validation
// ============================================================================

#[test]
fn test_capture_var_with_capture_pattern() {
    let capture_var = CaptureVar {
        name: "token".to_string(),
        capture: Some("\"token\":\"([^\"]+)\"".to_string()),
        command: None,
    };

    assert_eq!(capture_var.name, "token");
    assert_eq!(
        capture_var.capture,
        Some("\"token\":\"([^\"]+)\"".to_string())
    );
    assert_eq!(capture_var.command, None);
}

#[test]
fn test_capture_var_with_command() {
    let capture_var = CaptureVar {
        name: "output_len".to_string(),
        capture: None,
        command: Some("cat /tmp/hello.txt | wc -c".to_string()),
    };

    assert_eq!(capture_var.name, "output_len");
    assert_eq!(capture_var.capture, None);
    assert_eq!(
        capture_var.command,
        Some("cat /tmp/hello.txt | wc -c".to_string())
    );
}

#[test]
fn test_capture_var_validate_with_capture_only() {
    let capture_var = CaptureVar {
        name: "test_var".to_string(),
        capture: Some("pattern".to_string()),
        command: None,
    };

    assert!(capture_var.validate().is_ok());
}

#[test]
fn test_capture_var_validate_with_command_only() {
    let capture_var = CaptureVar {
        name: "test_var".to_string(),
        capture: None,
        command: Some("echo test".to_string()),
    };

    assert!(capture_var.validate().is_ok());
}

#[test]
fn test_capture_var_validate_both_fields_error() {
    let capture_var = CaptureVar {
        name: "invalid_var".to_string(),
        capture: Some("pattern".to_string()),
        command: Some("command".to_string()),
    };

    let result = capture_var.validate();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("mutually exclusive"));
    assert!(error.contains("invalid_var"));
}

#[test]
fn test_capture_var_validate_neither_field_error() {
    let capture_var = CaptureVar {
        name: "empty_var".to_string(),
        capture: None,
        command: None,
    };

    let result = capture_var.validate();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("either capture or command must be specified"));
    assert!(error.contains("empty_var"));
}

// ============================================================================
// CaptureVar Serialization/Deserialization Tests
// ============================================================================

#[test]
fn test_capture_var_serialize_with_capture() {
    let capture_var = CaptureVar {
        name: "token".to_string(),
        capture: Some("\"token\":\"([^\"]+)\"".to_string()),
        command: None,
    };

    let yaml = serde_yaml::to_string(&capture_var).unwrap();
    assert!(yaml.contains("name: token"));
    assert!(yaml.contains("capture:"));
    assert!(!yaml.contains("command:"));
}

#[test]
fn test_capture_var_serialize_with_command() {
    let capture_var = CaptureVar {
        name: "output_len".to_string(),
        capture: None,
        command: Some("wc -c".to_string()),
    };

    let yaml = serde_yaml::to_string(&capture_var).unwrap();
    assert!(yaml.contains("name: output_len"));
    assert!(yaml.contains("command:"));
    assert!(!yaml.contains("capture:"));
}

#[test]
fn test_capture_var_deserialize_with_capture() {
    let yaml = r#"
name: token
capture: '"token":"([^"]+)"'
"#;
    let capture_var: CaptureVar = serde_yaml::from_str(yaml).unwrap();

    assert_eq!(capture_var.name, "token");
    assert_eq!(
        capture_var.capture,
        Some("\"token\":\"([^\"]+)\"".to_string())
    );
    assert_eq!(capture_var.command, None);
    assert!(capture_var.validate().is_ok());
}

#[test]
fn test_capture_var_deserialize_with_command() {
    let yaml = r#"
name: output_len
command: "cat /tmp/hello.txt | wc -c"
"#;
    let capture_var: CaptureVar = serde_yaml::from_str(yaml).unwrap();

    assert_eq!(capture_var.name, "output_len");
    assert_eq!(capture_var.capture, None);
    assert_eq!(
        capture_var.command,
        Some("cat /tmp/hello.txt | wc -c".to_string())
    );
    assert!(capture_var.validate().is_ok());
}

#[test]
fn test_capture_var_roundtrip_with_capture() {
    let original = CaptureVar {
        name: "session_id".to_string(),
        capture: Some("session=([0-9a-f]+)".to_string()),
        command: None,
    };

    let yaml = serde_yaml::to_string(&original).unwrap();
    let deserialized: CaptureVar = serde_yaml::from_str(&yaml).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn test_capture_var_roundtrip_with_command() {
    let original = CaptureVar {
        name: "timestamp".to_string(),
        capture: None,
        command: Some("date +%s".to_string()),
    };

    let yaml = serde_yaml::to_string(&original).unwrap();
    let deserialized: CaptureVar = serde_yaml::from_str(&yaml).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn test_capture_var_json_serialization() {
    let capture_var = CaptureVar {
        name: "user_id".to_string(),
        capture: Some("user_id=(\\d+)".to_string()),
        command: None,
    };

    let json = serde_json::to_string(&capture_var).unwrap();
    let deserialized: CaptureVar = serde_json::from_str(&json).unwrap();

    assert_eq!(capture_var, deserialized);
}

// ============================================================================
// CaptureVarsFormat Tests - Legacy Format
// ============================================================================

#[test]
fn test_capture_vars_format_legacy_single_var() {
    let mut map = BTreeMap::new();
    map.insert("token".to_string(), "\"token\":\"([^\"]+)\"".to_string());

    let format = CaptureVarsFormat::Legacy(map);

    match format {
        CaptureVarsFormat::Legacy(m) => {
            assert_eq!(m.len(), 1);
            assert_eq!(m.get("token"), Some(&"\"token\":\"([^\"]+)\"".to_string()));
        }
        _ => panic!("Expected Legacy variant"),
    }
}

#[test]
fn test_capture_vars_format_legacy_multiple_vars() {
    let mut map = BTreeMap::new();
    map.insert("token".to_string(), "\"token\":\"([^\"]+)\"".to_string());
    map.insert(
        "session_id".to_string(),
        "\"session_id\":\"([^\"]+)\"".to_string(),
    );
    map.insert("user_id".to_string(), "user_id=(\\d+)".to_string());

    let format = CaptureVarsFormat::Legacy(map);

    match format {
        CaptureVarsFormat::Legacy(m) => {
            assert_eq!(m.len(), 3);
            assert!(m.contains_key("token"));
            assert!(m.contains_key("session_id"));
            assert!(m.contains_key("user_id"));
        }
        _ => panic!("Expected Legacy variant"),
    }
}

#[test]
fn test_capture_vars_format_legacy_empty() {
    let map = BTreeMap::new();
    let format = CaptureVarsFormat::Legacy(map);

    match format {
        CaptureVarsFormat::Legacy(m) => {
            assert_eq!(m.len(), 0);
        }
        _ => panic!("Expected Legacy variant"),
    }
}

#[test]
fn test_capture_vars_format_legacy_deserialize() {
    let yaml = r#"
token: '"token":"([^"]+)"'
session_id: '"session_id":"([^"]+)"'
"#;
    let result: CaptureVarsFormat = serde_yaml::from_str(yaml).unwrap();

    match result {
        CaptureVarsFormat::Legacy(map) => {
            assert_eq!(map.len(), 2);
            assert_eq!(
                map.get("token"),
                Some(&"\"token\":\"([^\"]+)\"".to_string())
            );
            assert_eq!(
                map.get("session_id"),
                Some(&"\"session_id\":\"([^\"]+)\"".to_string())
            );
        }
        _ => panic!("Expected Legacy variant"),
    }
}

#[test]
fn test_capture_vars_format_legacy_serialize() {
    let mut map = BTreeMap::new();
    map.insert("var1".to_string(), "pattern1".to_string());
    map.insert("var2".to_string(), "pattern2".to_string());

    let format = CaptureVarsFormat::Legacy(map);
    let yaml = serde_yaml::to_string(&format).unwrap();

    assert!(yaml.contains("var1: pattern1"));
    assert!(yaml.contains("var2: pattern2"));
}

#[test]
fn test_capture_vars_format_legacy_roundtrip() {
    let mut map = BTreeMap::new();
    map.insert("api_key".to_string(), "api_key=([a-zA-Z0-9]+)".to_string());
    map.insert("endpoint".to_string(), "endpoint=(.+)".to_string());

    let original = CaptureVarsFormat::Legacy(map);
    let yaml = serde_yaml::to_string(&original).unwrap();
    let deserialized: CaptureVarsFormat = serde_yaml::from_str(&yaml).unwrap();

    assert_eq!(original, deserialized);
}

// ============================================================================
// CaptureVarsFormat Tests - New Array Format
// ============================================================================

#[test]
fn test_capture_vars_format_new_single_capture() {
    let vars = vec![CaptureVar {
        name: "token".to_string(),
        capture: Some("\"token\":\"([^\"]+)\"".to_string()),
        command: None,
    }];

    let format = CaptureVarsFormat::New(vars);

    match format {
        CaptureVarsFormat::New(v) => {
            assert_eq!(v.len(), 1);
            assert_eq!(v[0].name, "token");
            assert!(v[0].capture.is_some());
            assert!(v[0].command.is_none());
        }
        _ => panic!("Expected New variant"),
    }
}

#[test]
fn test_capture_vars_format_new_single_command() {
    let vars = vec![CaptureVar {
        name: "output_len".to_string(),
        capture: None,
        command: Some("wc -c".to_string()),
    }];

    let format = CaptureVarsFormat::New(vars);

    match format {
        CaptureVarsFormat::New(v) => {
            assert_eq!(v.len(), 1);
            assert_eq!(v[0].name, "output_len");
            assert!(v[0].capture.is_none());
            assert!(v[0].command.is_some());
        }
        _ => panic!("Expected New variant"),
    }
}

#[test]
fn test_capture_vars_format_new_mixed_types() {
    let vars = vec![
        CaptureVar {
            name: "token".to_string(),
            capture: Some("\"token\":\"([^\"]+)\"".to_string()),
            command: None,
        },
        CaptureVar {
            name: "output_len".to_string(),
            capture: None,
            command: Some("cat /tmp/hello.txt | wc -c".to_string()),
        },
        CaptureVar {
            name: "session_id".to_string(),
            capture: Some("session=([0-9a-f]+)".to_string()),
            command: None,
        },
    ];

    let format = CaptureVarsFormat::New(vars);

    match format {
        CaptureVarsFormat::New(v) => {
            assert_eq!(v.len(), 3);
            assert_eq!(v[0].name, "token");
            assert!(v[0].capture.is_some());
            assert_eq!(v[1].name, "output_len");
            assert!(v[1].command.is_some());
            assert_eq!(v[2].name, "session_id");
            assert!(v[2].capture.is_some());
        }
        _ => panic!("Expected New variant"),
    }
}

#[test]
fn test_capture_vars_format_new_empty() {
    let vars = Vec::new();
    let format = CaptureVarsFormat::New(vars);

    match format {
        CaptureVarsFormat::New(v) => {
            assert_eq!(v.len(), 0);
        }
        _ => panic!("Expected New variant"),
    }
}

#[test]
fn test_capture_vars_format_new_deserialize() {
    let yaml = r#"
- name: token
  capture: '"token":"([^"]+)"'
- name: output_len
  command: "cat /tmp/hello.txt | wc -c"
"#;
    let result: CaptureVarsFormat = serde_yaml::from_str(yaml).unwrap();

    match result {
        CaptureVarsFormat::New(vec) => {
            assert_eq!(vec.len(), 2);
            assert_eq!(vec[0].name, "token");
            assert_eq!(vec[0].capture, Some("\"token\":\"([^\"]+)\"".to_string()));
            assert_eq!(vec[0].command, None);
            assert_eq!(vec[1].name, "output_len");
            assert_eq!(vec[1].capture, None);
            assert_eq!(
                vec[1].command,
                Some("cat /tmp/hello.txt | wc -c".to_string())
            );
        }
        _ => panic!("Expected New variant"),
    }
}

#[test]
fn test_capture_vars_format_new_serialize() {
    let vars = vec![
        CaptureVar {
            name: "var1".to_string(),
            capture: Some("pattern1".to_string()),
            command: None,
        },
        CaptureVar {
            name: "var2".to_string(),
            capture: None,
            command: Some("command2".to_string()),
        },
    ];

    let format = CaptureVarsFormat::New(vars);
    let yaml = serde_yaml::to_string(&format).unwrap();

    assert!(yaml.contains("name: var1"));
    assert!(yaml.contains("capture: pattern1"));
    assert!(yaml.contains("name: var2"));
    assert!(yaml.contains("command: command2"));
}

#[test]
fn test_capture_vars_format_new_roundtrip() {
    let vars = vec![
        CaptureVar {
            name: "api_token".to_string(),
            capture: Some("Bearer ([a-zA-Z0-9]+)".to_string()),
            command: None,
        },
        CaptureVar {
            name: "file_size".to_string(),
            capture: None,
            command: Some("stat -f%z /tmp/file".to_string()),
        },
    ];

    let original = CaptureVarsFormat::New(vars);
    let yaml = serde_yaml::to_string(&original).unwrap();
    let deserialized: CaptureVarsFormat = serde_yaml::from_str(&yaml).unwrap();

    assert_eq!(original, deserialized);
}

// ============================================================================
// CaptureVarsFormat Tests - Backward Compatibility
// ============================================================================

#[test]
fn test_legacy_format_backward_compatibility() {
    // This test ensures that old YAML files with legacy format still work
    let yaml = r#"
token: '"token":"([^"]+)"'
session_id: '"session":"([0-9]+)"'
user_id: 'uid:(\d+)'
"#;
    let result: CaptureVarsFormat = serde_yaml::from_str(yaml).unwrap();

    match result {
        CaptureVarsFormat::Legacy(map) => {
            assert_eq!(map.len(), 3);
            assert!(map.contains_key("token"));
            assert!(map.contains_key("session_id"));
            assert!(map.contains_key("user_id"));
        }
        _ => panic!("Expected Legacy variant for backward compatibility"),
    }
}

#[test]
fn test_new_format_with_only_captures() {
    // New format can also handle only regex captures (like legacy)
    let yaml = r#"
- name: token
  capture: '"token":"([^"]+)"'
- name: session_id
  capture: '"session":"([0-9]+)"'
"#;
    let result: CaptureVarsFormat = serde_yaml::from_str(yaml).unwrap();

    match result {
        CaptureVarsFormat::New(vec) => {
            assert_eq!(vec.len(), 2);
            for var in vec {
                assert!(var.capture.is_some());
                assert!(var.command.is_none());
                assert!(var.validate().is_ok());
            }
        }
        _ => panic!("Expected New variant"),
    }
}

#[test]
fn test_format_distinction_by_yaml_structure() {
    // Legacy format: map structure
    let legacy_yaml = r#"
var1: pattern1
var2: pattern2
"#;
    let legacy_result: CaptureVarsFormat = serde_yaml::from_str(legacy_yaml).unwrap();
    assert!(matches!(legacy_result, CaptureVarsFormat::Legacy(_)));

    // New format: array structure
    let new_yaml = r#"
- name: var1
  capture: pattern1
- name: var2
  command: cmd2
"#;
    let new_result: CaptureVarsFormat = serde_yaml::from_str(new_yaml).unwrap();
    assert!(matches!(new_result, CaptureVarsFormat::New(_)));
}

// ============================================================================
// Integration Tests - Step with CaptureVars
// ============================================================================

#[test]
fn test_step_with_legacy_capture_vars() {
    let yaml = r#"
step: 1
description: "Test step"
command: "echo test"
capture_vars:
  token: '"token":"([^"]+)"'
  session_id: '"session_id":"([^"]+)"'
expected:
  result: "0"
  output: "test"
"#;
    let result: Step = serde_yaml::from_str(yaml).unwrap();

    assert_eq!(result.step, 1);
    assert!(result.capture_vars.is_some());

    match result.capture_vars.unwrap() {
        CaptureVarsFormat::Legacy(map) => {
            assert_eq!(map.len(), 2);
            assert!(map.contains_key("token"));
            assert!(map.contains_key("session_id"));
        }
        _ => panic!("Expected Legacy variant"),
    }
}

#[test]
fn test_step_with_new_capture_vars() {
    let yaml = r#"
step: 1
description: "Test step"
command: "echo test"
capture_vars:
  - name: token
    capture: '"token":"([^"]+)"'
  - name: output_len
    command: "cat /tmp/hello.txt | wc -c"
expected:
  result: "0"
  output: "test"
"#;
    let result: Step = serde_yaml::from_str(yaml).unwrap();

    assert_eq!(result.step, 1);
    assert!(result.capture_vars.is_some());

    match result.capture_vars.unwrap() {
        CaptureVarsFormat::New(vec) => {
            assert_eq!(vec.len(), 2);
            assert_eq!(vec[0].name, "token");
            assert!(vec[0].capture.is_some());
            assert_eq!(vec[1].name, "output_len");
            assert!(vec[1].command.is_some());
        }
        _ => panic!("Expected New variant"),
    }
}

#[test]
fn test_step_without_capture_vars() {
    let yaml = r#"
step: 1
description: "Test step"
command: "echo test"
expected:
  result: "0"
  output: "test"
"#;
    let result: Step = serde_yaml::from_str(yaml).unwrap();

    assert_eq!(result.step, 1);
    assert!(result.capture_vars.is_none());
}

#[test]
fn test_step_with_empty_legacy_capture_vars() {
    let yaml = r#"
step: 1
description: "Test step"
command: "echo test"
capture_vars: {}
expected:
  result: "0"
  output: "test"
"#;
    let result: Step = serde_yaml::from_str(yaml).unwrap();

    assert_eq!(result.step, 1);
    assert!(result.capture_vars.is_some());

    match result.capture_vars.unwrap() {
        CaptureVarsFormat::Legacy(map) => {
            assert_eq!(map.len(), 0);
        }
        _ => panic!("Expected Legacy variant"),
    }
}

#[test]
fn test_step_with_empty_new_capture_vars() {
    let yaml = r#"
step: 1
description: "Test step"
command: "echo test"
capture_vars: []
expected:
  result: "0"
  output: "test"
"#;
    let result: Step = serde_yaml::from_str(yaml).unwrap();

    assert_eq!(result.step, 1);
    assert!(result.capture_vars.is_some());

    match result.capture_vars.unwrap() {
        CaptureVarsFormat::New(vec) => {
            assert_eq!(vec.len(), 0);
        }
        _ => panic!("Expected New variant"),
    }
}

#[test]
fn test_step_roundtrip_with_legacy_capture_vars() {
    let mut map = BTreeMap::new();
    map.insert("var1".to_string(), "pattern1".to_string());

    let original = Step {
        step: 1,
        manual: None,
        description: "Test".to_string(),
        command: "echo test".to_string(),
        capture_vars: Some(CaptureVarsFormat::Legacy(map)),
        expected: Expected {
            success: None,
            result: "0".to_string(),
            output: "test".to_string(),
        },
        verification: testcase_manager::models::Verification {
            result: testcase_manager::models::VerificationExpression::Simple(
                "[[ $? -eq 0 ]]".to_string(),
            ),
            output: testcase_manager::models::VerificationExpression::Simple(
                "cat $COMMAND_OUTPUT | grep -q \"${OUTPUT}\"".to_string(),
            ),
            output_file: None,
            general: None,
        },
        reference: None,
    };

    let yaml = serde_yaml::to_string(&original).unwrap();
    let deserialized: Step = serde_yaml::from_str(&yaml).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn test_step_roundtrip_with_new_capture_vars() {
    let vars = vec![
        CaptureVar {
            name: "token".to_string(),
            capture: Some("pattern".to_string()),
            command: None,
        },
        CaptureVar {
            name: "len".to_string(),
            capture: None,
            command: Some("wc -c".to_string()),
        },
    ];

    let original = Step {
        step: 1,
        manual: None,
        description: "Test".to_string(),
        command: "echo test".to_string(),
        capture_vars: Some(CaptureVarsFormat::New(vars)),
        expected: Expected {
            success: None,
            result: "0".to_string(),
            output: "test".to_string(),
        },
        verification: testcase_manager::models::Verification {
            result: testcase_manager::models::VerificationExpression::Simple(
                "[[ $? -eq 0 ]]".to_string(),
            ),
            output: testcase_manager::models::VerificationExpression::Simple(
                "cat $COMMAND_OUTPUT | grep -q \"${OUTPUT}\"".to_string(),
            ),
            output_file: None,
            general: None,
        },
        reference: None,
    };

    let yaml = serde_yaml::to_string(&original).unwrap();
    let deserialized: Step = serde_yaml::from_str(&yaml).unwrap();

    assert_eq!(original, deserialized);
}

// ============================================================================
// CaptureVar Validation Tests - Edge Cases
// ============================================================================

#[test]
fn test_capture_var_with_empty_pattern() {
    let capture_var = CaptureVar {
        name: "test".to_string(),
        capture: Some("".to_string()),
        command: None,
    };

    // Empty pattern is still valid - validation just checks mutual exclusivity
    assert!(capture_var.validate().is_ok());
}

#[test]
fn test_capture_var_with_empty_command() {
    let capture_var = CaptureVar {
        name: "test".to_string(),
        capture: None,
        command: Some("".to_string()),
    };

    // Empty command is still valid - validation just checks mutual exclusivity
    assert!(capture_var.validate().is_ok());
}

#[test]
fn test_capture_var_with_complex_regex_pattern() {
    let capture_var = CaptureVar {
        name: "complex_match".to_string(),
        capture: Some(r#"(?:token|session):\s*"([a-zA-Z0-9_-]{20,})""#.to_string()),
        command: None,
    };

    assert!(capture_var.validate().is_ok());
}

#[test]
fn test_capture_var_with_shell_pipeline_command() {
    let capture_var = CaptureVar {
        name: "pipeline_result".to_string(),
        capture: None,
        command: Some("cat /tmp/file | grep pattern | awk '{print $2}' | head -1".to_string()),
    };

    assert!(capture_var.validate().is_ok());
}

#[test]
fn test_capture_var_with_special_characters_in_name() {
    let capture_var = CaptureVar {
        name: "var_with_underscores_123".to_string(),
        capture: Some("pattern".to_string()),
        command: None,
    };

    assert!(capture_var.validate().is_ok());
}

// ============================================================================
// Complex Scenario Tests
// ============================================================================

#[test]
fn test_multiple_steps_with_mixed_capture_formats() {
    let yaml = r#"
- step: 1
  description: "Legacy format step"
  command: "curl api.example.com"
  capture_vars:
    token: '"token":"([^"]+)"'
  expected:
    result: "0"
    output: "success"
- step: 2
  description: "New format step"
  command: "process data"
  capture_vars:
    - name: data_size
      command: "wc -c /tmp/data"
    - name: checksum
      capture: 'checksum:([a-f0-9]{32})'
  expected:
    result: "0"
    output: "done"
"#;
    let steps: Vec<Step> = serde_yaml::from_str(yaml).unwrap();

    assert_eq!(steps.len(), 2);

    // First step uses legacy format
    assert!(steps[0].capture_vars.is_some());
    match &steps[0].capture_vars {
        Some(CaptureVarsFormat::Legacy(map)) => {
            assert_eq!(map.len(), 1);
            assert!(map.contains_key("token"));
        }
        _ => panic!("Expected Legacy format for step 1"),
    }

    // Second step uses new format
    assert!(steps[1].capture_vars.is_some());
    match &steps[1].capture_vars {
        Some(CaptureVarsFormat::New(vec)) => {
            assert_eq!(vec.len(), 2);
            assert_eq!(vec[0].name, "data_size");
            assert!(vec[0].command.is_some());
            assert_eq!(vec[1].name, "checksum");
            assert!(vec[1].capture.is_some());
        }
        _ => panic!("Expected New format for step 2"),
    }
}

#[test]
fn test_capture_vars_format_comparison() {
    // Create same variables in both formats
    let mut legacy_map = BTreeMap::new();
    legacy_map.insert("var1".to_string(), "pattern1".to_string());
    legacy_map.insert("var2".to_string(), "pattern2".to_string());

    let new_vec = vec![
        CaptureVar {
            name: "var1".to_string(),
            capture: Some("pattern1".to_string()),
            command: None,
        },
        CaptureVar {
            name: "var2".to_string(),
            capture: Some("pattern2".to_string()),
            command: None,
        },
    ];

    let legacy_format = CaptureVarsFormat::Legacy(legacy_map);
    let new_format = CaptureVarsFormat::New(new_vec);

    // They should not be equal even if semantically similar
    assert_ne!(legacy_format, new_format);
}

#[test]
fn test_capture_var_json_roundtrip() {
    let vars = vec![
        CaptureVar {
            name: "api_key".to_string(),
            capture: Some("key=([a-z0-9]+)".to_string()),
            command: None,
        },
        CaptureVar {
            name: "timestamp".to_string(),
            capture: None,
            command: Some("date +%s".to_string()),
        },
    ];

    let original = CaptureVarsFormat::New(vars);
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: CaptureVarsFormat = serde_json::from_str(&json).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn test_capture_vars_btreemap_ordering() {
    // BTreeMap maintains sorted order by keys
    let mut map = BTreeMap::new();
    map.insert("zebra".to_string(), "pattern_z".to_string());
    map.insert("alpha".to_string(), "pattern_a".to_string());
    map.insert("beta".to_string(), "pattern_b".to_string());

    let format = CaptureVarsFormat::Legacy(map);

    let yaml = serde_yaml::to_string(&format).unwrap();

    // Verify order in serialized output
    let alpha_pos = yaml.find("alpha").unwrap();
    let beta_pos = yaml.find("beta").unwrap();
    let zebra_pos = yaml.find("zebra").unwrap();

    assert!(alpha_pos < beta_pos);
    assert!(beta_pos < zebra_pos);
}

// ============================================================================
// Validation Error Message Tests
// ============================================================================

#[test]
fn test_validation_error_message_includes_variable_name() {
    let capture_var = CaptureVar {
        name: "my_special_var".to_string(),
        capture: Some("pattern".to_string()),
        command: Some("command".to_string()),
    };

    let result = capture_var.validate();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("my_special_var"));
}

#[test]
fn test_validation_error_for_both_specified() {
    let capture_var = CaptureVar {
        name: "duplicate_var".to_string(),
        capture: Some("regex_pattern".to_string()),
        command: Some("shell_command".to_string()),
    };

    let result = capture_var.validate();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("mutually exclusive"));
    assert!(error.contains("duplicate_var"));
}

#[test]
fn test_validation_error_for_neither_specified() {
    let capture_var = CaptureVar {
        name: "empty_var".to_string(),
        capture: None,
        command: None,
    };

    let result = capture_var.validate();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("either capture or command must be specified"));
    assert!(error.contains("empty_var"));
}

// ============================================================================
// Real-world Use Case Tests
// ============================================================================

#[test]
fn test_capture_jwt_token_from_api_response() {
    let yaml = r#"
- name: jwt_token
  capture: '"access_token":"([^"]+)"'
- name: expires_in
  capture: '"expires_in":(\d+)'
"#;
    let result: CaptureVarsFormat = serde_yaml::from_str(yaml).unwrap();

    match result {
        CaptureVarsFormat::New(vec) => {
            assert_eq!(vec.len(), 2);
            assert_eq!(vec[0].name, "jwt_token");
            assert_eq!(vec[1].name, "expires_in");
            assert!(vec[0].validate().is_ok());
            assert!(vec[1].validate().is_ok());
        }
        _ => panic!("Expected New variant"),
    }
}

#[test]
fn test_capture_file_information_with_commands() {
    let yaml = r#"
- name: file_size
  command: "stat -f %z /tmp/output.bin"
- name: file_hash
  command: "sha256sum /tmp/output.bin | awk '{print $1}'"
- name: line_count
  command: "wc -l /tmp/output.txt | awk '{print $1}'"
"#;
    let result: CaptureVarsFormat = serde_yaml::from_str(yaml).unwrap();

    match result {
        CaptureVarsFormat::New(vec) => {
            assert_eq!(vec.len(), 3);
            for var in &vec {
                assert!(var.capture.is_none());
                assert!(var.command.is_some());
                assert!(var.validate().is_ok());
            }
        }
        _ => panic!("Expected New variant"),
    }
}

#[test]
fn test_mixed_capture_methods_in_single_step() {
    let yaml = r#"
- name: response_code
  capture: '"code":(\d+)'
- name: server_timestamp
  command: "date -r /tmp/response.json +%s"
- name: transaction_id
  capture: '"transaction_id":"([a-f0-9-]{36})"'
- name: payload_size
  command: "wc -c /tmp/response.json | awk '{print $1}'"
"#;
    let result: CaptureVarsFormat = serde_yaml::from_str(yaml).unwrap();

    match result {
        CaptureVarsFormat::New(vec) => {
            assert_eq!(vec.len(), 4);

            // response_code uses capture
            assert_eq!(vec[0].name, "response_code");
            assert!(vec[0].capture.is_some());
            assert!(vec[0].command.is_none());

            // server_timestamp uses command
            assert_eq!(vec[1].name, "server_timestamp");
            assert!(vec[1].capture.is_none());
            assert!(vec[1].command.is_some());

            // transaction_id uses capture
            assert_eq!(vec[2].name, "transaction_id");
            assert!(vec[2].capture.is_some());
            assert!(vec[2].command.is_none());

            // payload_size uses command
            assert_eq!(vec[3].name, "payload_size");
            assert!(vec[3].capture.is_none());
            assert!(vec[3].command.is_some());

            // All should be valid
            for var in &vec {
                assert!(var.validate().is_ok());
            }
        }
        _ => panic!("Expected New variant"),
    }
}
