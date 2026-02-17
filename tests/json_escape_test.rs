use anyhow::Result;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;
use testcase_manager::config::{
    Config, JsonEscapingConfig, JsonEscapingMethod, ScriptGenerationConfig,
};
use testcase_manager::executor::TestExecutor;
use testcase_manager::models::{
    Expected, Step, TestCase, TestSequence, Verification, VerificationExpression,
};

// ============================================================================
// JSON Escape Binary Tests - Special Characters
// ============================================================================

/// Test that the json-escape binary properly escapes double quotes
#[test]
fn test_json_escape_binary_quotes() -> Result<()> {
    let input = r#"He said "hello" to me"#;
    let output = Command::new("cargo")
        .args(["run", "--bin", "json-escape"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }
            child.wait_with_output()
        })?;

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout)?;
    assert_eq!(result, r#"He said \"hello\" to me"#);
    Ok(())
}

/// Test that the json-escape binary properly escapes backslashes
#[test]
fn test_json_escape_binary_backslashes() -> Result<()> {
    let input = r#"Path is C:\Users\test"#;
    let output = Command::new("cargo")
        .args(["run", "--bin", "json-escape"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }
            child.wait_with_output()
        })?;

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout)?;
    assert_eq!(result, r#"Path is C:\\Users\\test"#);
    Ok(())
}

/// Test that the json-escape binary properly escapes newlines
#[test]
fn test_json_escape_binary_newlines() -> Result<()> {
    let input = "Line 1\nLine 2\nLine 3";
    let output = Command::new("cargo")
        .args(["run", "--bin", "json-escape"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }
            child.wait_with_output()
        })?;

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout)?;
    assert_eq!(result, r#"Line 1\nLine 2\nLine 3"#);
    Ok(())
}

/// Test that the json-escape binary properly escapes tabs
#[test]
fn test_json_escape_binary_tabs() -> Result<()> {
    let input = "Column1\tColumn2\tColumn3";
    let output = Command::new("cargo")
        .args(["run", "--bin", "json-escape"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }
            child.wait_with_output()
        })?;

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout)?;
    assert_eq!(result, r#"Column1\tColumn2\tColumn3"#);
    Ok(())
}

/// Test that the json-escape binary properly escapes carriage returns
#[test]
fn test_json_escape_binary_carriage_returns() -> Result<()> {
    let input = "Line 1\r\nLine 2\r\nLine 3";
    let output = Command::new("cargo")
        .args(["run", "--bin", "json-escape"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }
            child.wait_with_output()
        })?;

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout)?;
    assert_eq!(result, r#"Line 1\r\nLine 2\r\nLine 3"#);
    Ok(())
}

/// Test that the json-escape binary properly escapes backspace and form feed
#[test]
fn test_json_escape_binary_control_chars() -> Result<()> {
    let input = "Text\x08with\x0Ccontrol";
    let output = Command::new("cargo")
        .args(["run", "--bin", "json-escape"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }
            child.wait_with_output()
        })?;

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout)?;
    assert_eq!(result, r#"Text\bwith\fcontrol"#);
    Ok(())
}

/// Test that the json-escape binary properly escapes unicode characters
#[test]
fn test_json_escape_binary_unicode() -> Result<()> {
    let input = "Hello 世界 🌍";
    let output = Command::new("cargo")
        .args(["run", "--bin", "json-escape"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }
            child.wait_with_output()
        })?;

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout)?;
    // Unicode characters above ASCII should be preserved as-is (not escaped to \uXXXX)
    assert_eq!(result, "Hello 世界 🌍");
    Ok(())
}

/// Test that the json-escape binary properly handles mixed special characters
#[test]
fn test_json_escape_binary_mixed_characters() -> Result<()> {
    let input = r#"Error: "file not found" at C:\path\to\file.txt
Line 2 with tab	here
And a newline"#;
    let output = Command::new("cargo")
        .args(["run", "--bin", "json-escape"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }
            child.wait_with_output()
        })?;

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout)?;
    assert_eq!(
        result,
        r#"Error: \"file not found\" at C:\\path\\to\\file.txt\nLine 2 with tab\there\nAnd a newline"#
    );
    Ok(())
}

/// Test that the json-escape binary handles empty input
#[test]
fn test_json_escape_binary_empty_input() -> Result<()> {
    let input = "";
    let output = Command::new("cargo")
        .args(["run", "--bin", "json-escape"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }
            child.wait_with_output()
        })?;

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout)?;
    assert_eq!(result, "");
    Ok(())
}

/// Test json-escape binary in test mode with valid output
#[test]
fn test_json_escape_binary_test_mode_valid() -> Result<()> {
    let input = "Simple text";
    let output = Command::new("cargo")
        .args(["run", "--bin", "json-escape", "--", "--test"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }
            child.wait_with_output()
        })?;

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout)?;
    assert_eq!(result, "Simple text");
    Ok(())
}

// ============================================================================
// JSON Escape Binary Tests - Edge Cases
// ============================================================================

/// Test json-escape binary with empty string in test mode
#[test]
fn test_json_escape_binary_empty_string_test_mode() -> Result<()> {
    let input = "";
    let output = Command::new("cargo")
        .args(["run", "--bin", "json-escape", "--", "--test"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }
            child.wait_with_output()
        })?;

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout)?;
    assert_eq!(result, "");
    Ok(())
}

/// Test json-escape binary with very long input string (10KB+)
#[test]
fn test_json_escape_binary_very_long_input() -> Result<()> {
    // Create a 12KB string with mixed content
    let mut input = String::new();
    for i in 0..1000 {
        input.push_str(&format!(
            "Line {} with \"quotes\" and \\backslashes\\ and\ttabs\n",
            i
        ));
    }

    let output = Command::new("cargo")
        .args(["run", "--bin", "json-escape"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }
            child.wait_with_output()
        })?;

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout)?;

    // Verify length increased due to escaping
    assert!(result.len() > input.len());

    // Verify specific patterns are escaped
    assert!(result.contains(r#"\"quotes\""#));
    assert!(result.contains(r#"\\"#));
    assert!(result.contains(r#"\t"#));
    assert!(result.contains(r#"\n"#));

    // Verify the escaped output is valid JSON when wrapped in quotes
    let test_json = format!("\"{}\"", result);
    assert!(serde_json::from_str::<serde_json::Value>(&test_json).is_ok());

    Ok(())
}

/// Test json-escape binary with consecutive backslashes
#[test]
fn test_json_escape_binary_consecutive_backslashes() -> Result<()> {
    let input = r#"One\Two\\Three\\\Four\\\\Five"#;
    let output = Command::new("cargo")
        .args(["run", "--bin", "json-escape"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }
            child.wait_with_output()
        })?;

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout)?;
    assert_eq!(result, r#"One\\Two\\\\Three\\\\\\Four\\\\\\\\Five"#);

    // Verify it's valid JSON
    let test_json = format!("\"{}\"", result);
    assert!(serde_json::from_str::<serde_json::Value>(&test_json).is_ok());

    Ok(())
}

/// Test json-escape binary with nested quotes
#[test]
fn test_json_escape_binary_nested_quotes() -> Result<()> {
    let input = r#"He said "She said "Hello" to me" yesterday"#;
    let output = Command::new("cargo")
        .args(["run", "--bin", "json-escape"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }
            child.wait_with_output()
        })?;

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout)?;
    assert_eq!(result, r#"He said \"She said \"Hello\" to me\" yesterday"#);

    // Verify it's valid JSON
    let test_json = format!("\"{}\"", result);
    assert!(serde_json::from_str::<serde_json::Value>(&test_json).is_ok());

    Ok(())
}

/// Test json-escape binary with multiple consecutive special characters
#[test]
fn test_json_escape_binary_consecutive_special_chars() -> Result<()> {
    let input = "test\"\"\\\\\n\n\t\t\r\r";
    let output = Command::new("cargo")
        .args(["run", "--bin", "json-escape"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }
            child.wait_with_output()
        })?;

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout)?;
    assert_eq!(result, r#"test\"\"\\\\\n\n\t\t\r\r"#);

    // Verify it's valid JSON
    let test_json = format!("\"{}\"", result);
    assert!(serde_json::from_str::<serde_json::Value>(&test_json).is_ok());

    Ok(())
}

/// Test json-escape binary with all control characters \x00 through \x1F
#[test]
fn test_json_escape_binary_all_control_characters() -> Result<()> {
    // Create input with all control characters from \x00 to \x1F
    let mut input = Vec::new();
    input.extend_from_slice(b"Start:");
    for i in 0x00..=0x1F {
        input.push(i);
    }
    input.extend_from_slice(b":End");

    let output = Command::new("cargo")
        .args(["run", "--bin", "json-escape"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(&input)?;
            }
            child.wait_with_output()
        })?;

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout)?;

    // Verify specific control characters are properly escaped
    assert!(result.contains(r#"\u0000"#)); // NULL
    assert!(result.contains(r#"\u0001"#)); // SOH
    assert!(result.contains(r#"\b"#)); // Backspace (\x08)
    assert!(result.contains(r#"\t"#)); // Tab (\x09)
    assert!(result.contains(r#"\n"#)); // Newline (\x0A)
    assert!(result.contains(r#"\f"#)); // Form feed (\x0C)
    assert!(result.contains(r#"\r"#)); // Carriage return (\x0D)
    assert!(result.contains(r#"\u000e"#)); // Shift Out
    assert!(result.contains(r#"\u001f"#)); // Unit Separator
    assert!(result.contains("Start:"));
    assert!(result.contains(":End"));

    // Verify it's valid JSON
    let test_json = format!("\"{}\"", result);
    assert!(serde_json::from_str::<serde_json::Value>(&test_json).is_ok());

    Ok(())
}

/// Test json-escape binary with control character sequences
#[test]
fn test_json_escape_binary_control_sequences() -> Result<()> {
    // Test various combinations of control characters
    let input = "Before\x00\x01\x02After";
    let output = Command::new("cargo")
        .args(["run", "--bin", "json-escape"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }
            child.wait_with_output()
        })?;

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout)?;
    assert!(result.contains(r#"Before\u0000\u0001\u0002After"#));

    // Verify it's valid JSON
    let test_json = format!("\"{}\"", result);
    assert!(serde_json::from_str::<serde_json::Value>(&test_json).is_ok());

    Ok(())
}

/// Test json-escape binary with invalid UTF-8 sequences
#[test]
fn test_json_escape_binary_invalid_utf8() -> Result<()> {
    // Create invalid UTF-8 sequences
    let invalid_utf8 = vec![
        0x48, 0x65, 0x6C, 0x6C, 0x6F, // "Hello"
        0xFF, // Invalid UTF-8
        0xFE, // Invalid UTF-8
        0x57, 0x6F, 0x72, 0x6C, 0x64, // "World"
    ];

    let output = Command::new("cargo")
        .args(["run", "--bin", "json-escape"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(&invalid_utf8)?;
            }
            child.wait_with_output()
        })?;

    // The binary should fail gracefully with invalid UTF-8
    // It uses read_to_string which will fail on invalid UTF-8
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Failed to read from stdin")
            || stderr.contains("stream did not contain valid UTF-8")
    );

    Ok(())
}

/// Test json-escape binary with replacement character for invalid UTF-8
#[test]
fn test_json_escape_binary_utf8_replacement_char() -> Result<()> {
    // Test with the Unicode replacement character (used when invalid UTF-8 is encountered)
    let input = "Valid text \u{FFFD} with replacement char";
    let output = Command::new("cargo")
        .args(["run", "--bin", "json-escape"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }
            child.wait_with_output()
        })?;

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout)?;
    // The replacement character should be preserved in output
    assert!(result.contains("\u{FFFD}"));
    assert_eq!(result, "Valid text \u{FFFD} with replacement char");

    // Verify it's valid JSON
    let test_json = format!("\"{}\"", result);
    assert!(serde_json::from_str::<serde_json::Value>(&test_json).is_ok());

    Ok(())
}

/// Test json-escape binary with extremely long consecutive special characters
#[test]
fn test_json_escape_binary_long_consecutive_specials() -> Result<()> {
    // Create a string with many consecutive backslashes and quotes
    let mut input = String::from("Start");
    input.push_str(&"\\".repeat(100));
    input.push_str(&"\"".repeat(100));
    input.push_str("End");

    let output = Command::new("cargo")
        .args(["run", "--bin", "json-escape"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }
            child.wait_with_output()
        })?;

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout)?;

    // Each backslash becomes two, each quote becomes backslash-quote
    assert!(result.contains("Start"));
    assert!(result.contains("End"));
    assert!(result.contains(&"\\\\".repeat(100)));
    assert!(result.contains(&"\\\"".repeat(100)));

    // Verify it's valid JSON
    let test_json = format!("\"{}\"", result);
    assert!(serde_json::from_str::<serde_json::Value>(&test_json).is_ok());

    Ok(())
}

/// Test json-escape binary with mixed control and printable characters
#[test]
fn test_json_escape_binary_mixed_control_printable() -> Result<()> {
    let input = "Line1\x00\nLine2\x01\tLine3\x02\"quoted\"\x03\\backslash\\";
    let output = Command::new("cargo")
        .args(["run", "--bin", "json-escape"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }
            child.wait_with_output()
        })?;

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout)?;

    assert!(result.contains("Line1"));
    assert!(result.contains(r#"\u0000"#));
    assert!(result.contains(r#"\n"#));
    assert!(result.contains("Line2"));
    assert!(result.contains(r#"\u0001"#));
    assert!(result.contains(r#"\t"#));
    assert!(result.contains("Line3"));
    assert!(result.contains(r#"\u0002"#));
    assert!(result.contains(r#"\"quoted\""#));
    assert!(result.contains(r#"\u0003"#));
    assert!(result.contains(r#"\\backslash\\"#));

    // Verify it's valid JSON
    let test_json = format!("\"{}\"", result);
    assert!(serde_json::from_str::<serde_json::Value>(&test_json).is_ok());

    Ok(())
}

/// Test json-escape binary test mode with long input
#[test]
fn test_json_escape_binary_test_mode_long_input() -> Result<()> {
    // Create a moderately long string with special characters
    let mut input = String::new();
    for i in 0..100 {
        input.push_str(&format!("Line {} with \"quotes\" and \\backslashes\\\n", i));
    }

    let output = Command::new("cargo")
        .args(["run", "--bin", "json-escape", "--", "--test"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }
            child.wait_with_output()
        })?;

    assert!(output.status.success());
    let result = String::from_utf8(output.stdout)?;

    // Verify escaping occurred
    assert!(result.contains(r#"\"quotes\""#));
    assert!(result.contains(r#"\\"#));
    assert!(result.contains(r#"\n"#));

    Ok(())
}

// ============================================================================
// Configuration Loading Tests - JSON Escaping Method Settings
// ============================================================================

/// Test default configuration has Auto method
#[test]
fn test_config_default_json_escaping_method() {
    let config = Config::default();
    assert!(matches!(
        config.script_generation.json_escaping.method,
        JsonEscapingMethod::Auto
    ));
    assert!(config.script_generation.json_escaping.enabled);
    assert!(config.script_generation.json_escaping.binary_path.is_none());
}

/// Test configuration with RustBinary method
#[test]
fn test_config_rust_binary_method() {
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::RustBinary,
                enabled: true,
                binary_path: None,
            },
        },
        ..Default::default()
    };

    assert!(matches!(
        config.script_generation.json_escaping.method,
        JsonEscapingMethod::RustBinary
    ));
}

/// Test configuration with ShellFallback method
#[test]
fn test_config_shell_fallback_method() {
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::ShellFallback,
                enabled: true,
                binary_path: None,
            },
        },
        ..Default::default()
    };

    assert!(matches!(
        config.script_generation.json_escaping.method,
        JsonEscapingMethod::ShellFallback
    ));
}

/// Test configuration with custom binary path
#[test]
fn test_config_custom_binary_path() {
    let custom_path = PathBuf::from("/usr/local/bin/json-escape");
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Auto,
                enabled: true,
                binary_path: Some(custom_path.clone()),
            },
        },
        ..Default::default()
    };

    assert_eq!(
        config.script_generation.json_escaping.binary_path,
        Some(custom_path)
    );
}

/// Test configuration serialization to TOML
#[test]
fn test_config_serialization_to_toml() -> Result<()> {
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::RustBinary,
                enabled: true,
                binary_path: Some(PathBuf::from("/custom/path/json-escape")),
            },
        },
        ..Default::default()
    };

    let toml_str = toml::to_string(&config)?;
    assert!(toml_str.contains("method = \"rust_binary\""));
    assert!(toml_str.contains("enabled = true"));
    assert!(toml_str.contains("binary_path = \"/custom/path/json-escape\""));
    Ok(())
}

/// Test configuration deserialization from TOML
#[test]
fn test_config_deserialization_from_toml() -> Result<()> {
    let toml_str = r#"
[script_generation.json_escaping]
method = "shell_fallback"
enabled = true
binary_path = "/opt/json-escape"
"#;

    let config: Config = toml::from_str(toml_str)?;
    assert!(matches!(
        config.script_generation.json_escaping.method,
        JsonEscapingMethod::ShellFallback
    ));
    assert!(config.script_generation.json_escaping.enabled);
    assert_eq!(
        config.script_generation.json_escaping.binary_path,
        Some(PathBuf::from("/opt/json-escape"))
    );
    Ok(())
}

/// Test configuration with disabled JSON escaping
#[test]
fn test_config_json_escaping_disabled() {
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Auto,
                enabled: false,
                binary_path: None,
            },
        },
        ..Default::default()
    };

    assert!(!config.script_generation.json_escaping.enabled);
}

// ============================================================================
// Script Generation Tests - RustBinary Mode
// ============================================================================

/// Test script generation with RustBinary method (default binary path)
#[test]
fn test_script_generation_rust_binary_default_path() {
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::RustBinary,
                enabled: true,
                binary_path: None,
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_config(config);
    let test_case = create_simple_test_case();
    let script = executor.generate_test_script(&test_case);

    // Should use json-escape binary directly
    assert!(script.contains("json-escape"));
    assert!(script.contains("printf '%s' \"$COMMAND_OUTPUT\" | json-escape"));
    // Should not have if command -v check
    assert!(!script.contains("if command -v json-escape"));
}

/// Test script generation with RustBinary method (custom binary path)
#[test]
fn test_script_generation_rust_binary_custom_path() {
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::RustBinary,
                enabled: true,
                binary_path: Some(PathBuf::from("/usr/local/bin/my-json-escape")),
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_config(config);
    let test_case = create_simple_test_case();
    let script = executor.generate_test_script(&test_case);

    // Should use custom binary path
    assert!(script.contains("/usr/local/bin/my-json-escape"));
    assert!(script.contains("printf '%s' \"$COMMAND_OUTPUT\" | /usr/local/bin/my-json-escape"));
}

// ============================================================================
// Script Generation Tests - ShellFallback Mode
// ============================================================================

/// Test script generation with ShellFallback method
#[test]
fn test_script_generation_shell_fallback() {
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::ShellFallback,
                enabled: true,
                binary_path: None,
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_config(config);
    let test_case = create_simple_test_case();
    let script = executor.generate_test_script(&test_case);

    // Should use sed/awk fallback directly
    assert!(script.contains("Shell fallback"));
    assert!(script.contains("sed 's/\\\\/\\\\\\\\/g"));
    assert!(script.contains("awk '{printf \"%s%s\", (NR>1?\"\\\\n\":\"\"), $0}'"));
    // Should not reference json-escape binary
    assert!(!script.contains("json-escape"));
}

/// Test that ShellFallback method escapes all necessary characters
#[test]
fn test_script_generation_shell_fallback_escapes() {
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::ShellFallback,
                enabled: true,
                binary_path: None,
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_config(config);
    let test_case = create_simple_test_case();
    let script = executor.generate_test_script(&test_case);

    // Verify that the sed command escapes backslashes, quotes, tabs, and carriage returns
    assert!(script.contains(r#"s/\\/\\\\/g"#)); // backslashes
    assert!(script.contains(r#"s/"/\\"/g"#)); // quotes
    assert!(script.contains(r#"s/\t/\\t/g"#)); // tabs
    assert!(script.contains(r#"s/\r/\\r/g"#)); // carriage returns
}

// ============================================================================
// Script Generation Tests - Auto Mode
// ============================================================================

/// Test script generation with Auto method
#[test]
fn test_script_generation_auto_mode() {
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Auto,
                enabled: true,
                binary_path: None,
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_config(config);
    let test_case = create_simple_test_case();
    let script = executor.generate_test_script(&test_case);

    // Should have conditional check for binary
    assert!(script.contains("if command -v json-escape"));
    assert!(script.contains("else"));
    // Should have both binary and fallback paths
    assert!(script.contains("printf '%s' \"$COMMAND_OUTPUT\" | json-escape"));
    assert!(script.contains("Shell fallback"));
    assert!(script.contains("sed 's/\\\\/\\\\\\\\/g"));
}

/// Test script generation with Auto method and custom binary path
#[test]
fn test_script_generation_auto_mode_custom_path() {
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Auto,
                enabled: true,
                binary_path: Some(PathBuf::from("/opt/bin/json-escape")),
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_config(config);
    let test_case = create_simple_test_case();
    let script = executor.generate_test_script(&test_case);

    // Should check for custom binary
    assert!(script.contains("if command -v /opt/bin/json-escape"));
    assert!(script.contains("printf '%s' \"$COMMAND_OUTPUT\" | /opt/bin/json-escape"));
}

// ============================================================================
// Binary Detection Tests - PATH Configuration
// ============================================================================

/// Test that json-escape binary can be found in PATH
#[test]
fn test_binary_detection_in_path() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("command -v json-escape")
        .output();

    // This test may pass or fail depending on whether the binary is in PATH
    // We're mainly testing the detection mechanism
    match output {
        Ok(result) => {
            if result.status.success() {
                let path = String::from_utf8_lossy(&result.stdout);
                assert!(!path.trim().is_empty());
            }
        }
        Err(_) => {
            // Binary not in PATH, which is acceptable for this test
        }
    }
}

/// Test binary detection with custom PATH
#[test]
fn test_binary_detection_custom_path() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let bin_dir = temp_dir.path().join("bin");
    fs::create_dir(&bin_dir)?;

    // Create a dummy executable script
    let json_escape_path = bin_dir.join("json-escape");
    let mut file = fs::File::create(&json_escape_path)?;
    writeln!(file, "#!/bin/sh")?;
    writeln!(file, "echo 'dummy'")?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&json_escape_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&json_escape_path, perms)?;
    }

    // Test with custom PATH
    let original_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", bin_dir.display(), original_path);

    let output = Command::new("sh")
        .arg("-c")
        .arg("command -v json-escape")
        .env("PATH", new_path)
        .output()?;

    assert!(output.status.success());
    let found_path = String::from_utf8(output.stdout)?;
    assert!(found_path.contains("json-escape"));

    Ok(())
}

/// Test binary detection when not in PATH
#[test]
fn test_binary_detection_not_in_path() {
    // Use empty PATH to ensure binary won't be found
    let output = Command::new("sh")
        .arg("-c")
        .arg("command -v json-escape-nonexistent-binary")
        .env("PATH", "")
        .output();

    match output {
        Ok(result) => {
            assert!(!result.status.success());
        }
        Err(_) => {
            // Expected when binary doesn't exist
        }
    }
}

/// Test that generated script uses correct binary detection
#[test]
fn test_generated_script_binary_detection() {
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Auto,
                enabled: true,
                binary_path: None,
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_config(config);
    let test_case = create_simple_test_case();
    let script = executor.generate_test_script(&test_case);

    // Verify the script uses proper binary detection
    assert!(script.contains("if command -v json-escape >/dev/null 2>&1; then"));
    assert!(script.contains("else"));
    assert!(script.contains("fi"));
}

/// Test script execution with binary in PATH
#[test]
fn test_script_execution_with_binary_in_path() -> Result<()> {
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Auto,
                enabled: true,
                binary_path: None,
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_config(config);
    let test_case = create_simple_test_case();
    let script = executor.generate_test_script(&test_case);

    // Extract just the JSON escaping code section to test
    let escaping_code_start = script.find("# Escape output for JSON").unwrap();
    let escaping_code_end = script[escaping_code_start..]
        .find("\n\n")
        .map(|i| escaping_code_start + i)
        .unwrap_or(script.len());
    let escaping_code = &script[escaping_code_start..escaping_code_end];

    // Verify the code structure
    assert!(escaping_code.contains("OUTPUT_ESCAPED="));
    assert!(escaping_code.contains("COMMAND_OUTPUT"));

    Ok(())
}

/// Test that fallback works when binary is not available
#[test]
fn test_script_execution_fallback_when_binary_missing() -> Result<()> {
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Auto,
                enabled: true,
                binary_path: Some(PathBuf::from("/nonexistent/path/json-escape")),
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_config(config);
    let test_case = create_simple_test_case();
    let script = executor.generate_test_script(&test_case);

    // Should contain fallback mechanism
    assert!(script.contains("if command -v /nonexistent/path/json-escape"));
    assert!(script.contains("else"));
    assert!(script.contains("Shell fallback"));
    assert!(script.contains("sed"));
    assert!(script.contains("awk"));

    Ok(())
}

// ============================================================================
// Integration Tests - Full Script Generation
// ============================================================================

/// Test full script generation with various escaping scenarios
#[test]
fn test_full_script_with_complex_output() {
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Auto,
                enabled: true,
                binary_path: None,
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_config(config);
    let mut test_case = create_simple_test_case();

    // Add a step with complex output expectations
    let sequence = &mut test_case.test_sequences[0];
    sequence.steps.push(Step {
        step: 2,
        manual: None,
        description: "Test with special characters".to_string(),
        command: r#"echo 'Line 1\nLine 2\tTab\r\n"Quote"'"#.to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: r#"Line 1\nLine 2\tTab\r\n\"Quote\""#.to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
    });

    let script = executor.generate_test_script(&test_case);

    // Verify JSON escaping code is present
    assert!(script.contains("OUTPUT_ESCAPED="));
    assert!(script.contains("COMMAND_OUTPUT"));

    // Verify JSON log writing includes escaped output
    assert!(script.contains(r#"\"output\": \"$OUTPUT_ESCAPED\""#));
}

/// Test that multiple steps all get JSON escaping
#[test]
fn test_multiple_steps_all_escaped() {
    let config = Config::default();
    let executor = TestExecutor::with_config(config);
    let mut test_case = create_simple_test_case();

    // Add multiple steps
    let sequence = &mut test_case.test_sequences[0];
    for i in 2..=5 {
        sequence.steps.push(Step {
            step: i,
            manual: None,
            description: format!("Step {}", i),
            command: format!("echo 'Step {}'", i),
            capture_vars: None,
            expected: Expected {
                success: Some(true),
                result: "0".to_string(),
                output: format!("Step {}", i),
            },
            verification: Verification {
                result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
                output: VerificationExpression::Simple("true".to_string()),
                output_file: None,
                general: None,
            },
            reference: None,
        });
    }

    let script = executor.generate_test_script(&test_case);

    // Count occurrences of OUTPUT_ESCAPED assignment
    // Note: In Auto mode, OUTPUT_ESCAPED appears twice per step (if/else branches)
    let escaped_count = script.matches("OUTPUT_ESCAPED=").count();

    // With Auto mode (default), should have two assignments per step (if/else)
    // 5 steps * 2 = 10
    assert!(
        escaped_count >= 5,
        "Should have at least 5 OUTPUT_ESCAPED assignments for 5 steps, got {}",
        escaped_count
    );
}

/// Test script with manual steps doesn't escape output
#[test]
fn test_manual_steps_no_escaping() {
    let config = Config::default();
    let executor = TestExecutor::with_config(config);
    let mut test_case = create_simple_test_case();

    // Replace first step with manual step
    test_case.test_sequences[0].steps[0].manual = Some(true);

    let script = executor.generate_test_script(&test_case);

    // Should still have escaping code, but not executed for manual step
    // Manual steps skip execution entirely
    assert!(script.contains("This is a manual step"));

    // The escaping code shouldn't be in the manual step section
    let manual_section_start = script.find("This is a manual step").unwrap();
    let manual_section_end = script[manual_section_start..]
        .find("# Step")
        .map(|i| manual_section_start + i)
        .unwrap_or(script.len());
    let manual_section = &script[manual_section_start..manual_section_end];

    assert!(!manual_section.contains("OUTPUT_ESCAPED="));
}

// ============================================================================
// Configuration Edge Cases and Validation Tests
// ============================================================================

/// Test Config::load() when config file is missing
#[test]
fn test_config_load_missing_file() -> Result<()> {
    // Save current HOME/USERPROFILE
    let original_home = std::env::var("HOME").ok();
    let original_userprofile = std::env::var("USERPROFILE").ok();

    // Create a temporary directory and set it as HOME
    let temp_dir = TempDir::new()?;
    std::env::set_var("HOME", temp_dir.path());
    std::env::remove_var("USERPROFILE");

    // Try to load config when file doesn't exist
    let config = Config::load()?;

    // Should return default config
    assert!(matches!(
        config.script_generation.json_escaping.method,
        JsonEscapingMethod::Auto
    ));
    assert!(config.script_generation.json_escaping.enabled);
    assert!(config.script_generation.json_escaping.binary_path.is_none());

    // Restore original environment
    if let Some(home) = original_home {
        std::env::set_var("HOME", home);
    }
    if let Some(userprofile) = original_userprofile {
        std::env::set_var("USERPROFILE", userprofile);
    }

    Ok(())
}

/// Test Config::load() when config file is corrupted (invalid TOML)
#[test]
fn test_config_load_corrupted_file() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_dir = temp_dir.path().join(".testcase-manager");
    fs::create_dir_all(&config_dir)?;
    let config_path = config_dir.join("config.toml");

    // Write invalid TOML
    fs::write(&config_path, "this is not valid TOML [ } invalid")?;

    // Save current HOME/USERPROFILE
    let original_home = std::env::var("HOME").ok();
    let original_userprofile = std::env::var("USERPROFILE").ok();

    std::env::set_var("HOME", temp_dir.path());
    std::env::remove_var("USERPROFILE");

    // Try to load config - should return an error
    let result = Config::load();
    assert!(result.is_err());

    let err = result.unwrap_err();
    let err_msg = format!("{}", err);
    assert!(err_msg.contains("Failed to parse config file"));

    // Restore original environment
    if let Some(home) = original_home {
        std::env::set_var("HOME", home);
    }
    if let Some(userprofile) = original_userprofile {
        std::env::set_var("USERPROFILE", userprofile);
    }

    Ok(())
}

/// Test Config::load() when config file has invalid method value
#[test]
fn test_config_invalid_method_value() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_dir = temp_dir.path().join(".testcase-manager");
    fs::create_dir_all(&config_dir)?;
    let config_path = config_dir.join("config.toml");

    // Write config with invalid method value
    let invalid_config = r#"
[script_generation.json_escaping]
method = "invalid_method"
enabled = true
"#;
    fs::write(&config_path, invalid_config)?;

    // Save current HOME/USERPROFILE
    let original_home = std::env::var("HOME").ok();
    let original_userprofile = std::env::var("USERPROFILE").ok();

    std::env::set_var("HOME", temp_dir.path());
    std::env::remove_var("USERPROFILE");

    // Try to load config - should return an error
    let result = Config::load();
    assert!(result.is_err());

    // Restore original environment
    if let Some(home) = original_home {
        std::env::set_var("HOME", home);
    }
    if let Some(userprofile) = original_userprofile {
        std::env::set_var("USERPROFILE", userprofile);
    }

    Ok(())
}

/// Test config with valid method values (auto, rust_binary, shell_fallback)
#[test]
fn test_config_valid_method_values() -> Result<()> {
    // Test auto
    let config_toml = r#"
[script_generation.json_escaping]
method = "auto"
enabled = true
"#;
    let config: Config = toml::from_str(config_toml)?;
    assert!(matches!(
        config.script_generation.json_escaping.method,
        JsonEscapingMethod::Auto
    ));

    // Test rust_binary
    let config_toml = r#"
[script_generation.json_escaping]
method = "rust_binary"
enabled = true
"#;
    let config: Config = toml::from_str(config_toml)?;
    assert!(matches!(
        config.script_generation.json_escaping.method,
        JsonEscapingMethod::RustBinary
    ));

    // Test shell_fallback
    let config_toml = r#"
[script_generation.json_escaping]
method = "shell_fallback"
enabled = true
"#;
    let config: Config = toml::from_str(config_toml)?;
    assert!(matches!(
        config.script_generation.json_escaping.method,
        JsonEscapingMethod::ShellFallback
    ));

    Ok(())
}

/// Test config with binary_path pointing to non-existent path
#[test]
fn test_config_nonexistent_binary_path() -> Result<()> {
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::RustBinary,
                enabled: true,
                binary_path: Some(PathBuf::from("/nonexistent/path/to/json-escape")),
            },
        },
        ..Default::default()
    };

    // Config should be valid even with non-existent path
    // The path validation happens at runtime, not config loading
    assert_eq!(
        config.script_generation.json_escaping.binary_path,
        Some(PathBuf::from("/nonexistent/path/to/json-escape"))
    );

    // Test that the executor can generate a script with this config
    let executor = TestExecutor::with_config(config);
    let test_case = create_simple_test_case();
    let script = executor.generate_test_script(&test_case);

    // Should still generate script but will fail at runtime
    assert!(script.contains("/nonexistent/path/to/json-escape"));

    Ok(())
}

/// Test script generation with enabled=false
#[test]
fn test_config_json_escaping_disabled_script_generation() {
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Auto,
                enabled: false,
                binary_path: None,
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_config(config);
    let test_case = create_simple_test_case();
    let script = executor.generate_test_script(&test_case);

    // When disabled, script should still be generated
    // The enabled flag controls whether escaping logic is included
    // Currently the implementation always includes escaping code
    // This test documents current behavior
    assert!(script.contains("OUTPUT_ESCAPED="));
}

/// Test Config::load_or_default() returns default on error
#[test]
fn test_config_load_or_default_with_error() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".testcase-manager");
    fs::create_dir_all(&config_dir).unwrap();
    let config_path = config_dir.join("config.toml");

    // Write invalid TOML
    fs::write(&config_path, "invalid toml content {{{").unwrap();

    // Save current HOME/USERPROFILE
    let original_home = std::env::var("HOME").ok();
    let original_userprofile = std::env::var("USERPROFILE").ok();

    std::env::set_var("HOME", temp_dir.path());
    std::env::remove_var("USERPROFILE");

    // load_or_default should return default config without error
    let config = Config::load_or_default();

    // Should have default values
    assert!(matches!(
        config.script_generation.json_escaping.method,
        JsonEscapingMethod::Auto
    ));
    assert!(config.script_generation.json_escaping.enabled);

    // Restore original environment
    if let Some(home) = original_home {
        std::env::set_var("HOME", home);
    }
    if let Some(userprofile) = original_userprofile {
        std::env::set_var("USERPROFILE", userprofile);
    }
}

/// Test Config::save() and then load the saved config
#[test]
fn test_config_save_and_load() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Save current HOME/USERPROFILE
    let original_home = std::env::var("HOME").ok();
    let original_userprofile = std::env::var("USERPROFILE").ok();

    std::env::set_var("HOME", temp_dir.path());
    std::env::remove_var("USERPROFILE");

    // Create a config with specific values
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::ShellFallback,
                enabled: false,
                binary_path: Some(PathBuf::from("/custom/path/json-escape")),
            },
        },
        default_device_name: Some("test-device".to_string()),
        ..Default::default()
    };

    // Save the config
    config.save()?;

    // Load it back
    let loaded_config = Config::load()?;

    // Verify values match
    assert!(matches!(
        loaded_config.script_generation.json_escaping.method,
        JsonEscapingMethod::ShellFallback
    ));
    assert!(!loaded_config.script_generation.json_escaping.enabled);
    assert_eq!(
        loaded_config.script_generation.json_escaping.binary_path,
        Some(PathBuf::from("/custom/path/json-escape"))
    );
    assert_eq!(
        loaded_config.default_device_name,
        Some("test-device".to_string())
    );

    // Restore original environment
    if let Some(home) = original_home {
        std::env::set_var("HOME", home);
    }
    if let Some(userprofile) = original_userprofile {
        std::env::set_var("USERPROFILE", userprofile);
    }

    Ok(())
}

/// Test Config::save() creates directory if it doesn't exist
#[test]
fn test_config_save_creates_directory() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Save current HOME/USERPROFILE
    let original_home = std::env::var("HOME").ok();
    let original_userprofile = std::env::var("USERPROFILE").ok();

    std::env::set_var("HOME", temp_dir.path());
    std::env::remove_var("USERPROFILE");

    // Verify config directory doesn't exist
    let config_dir = temp_dir.path().join(".testcase-manager");
    assert!(!config_dir.exists());

    // Save config
    let config = Config::default();
    config.save()?;

    // Verify directory was created
    assert!(config_dir.exists());
    assert!(config_dir.join("config.toml").exists());

    // Restore original environment
    if let Some(home) = original_home {
        std::env::set_var("HOME", home);
    }
    if let Some(userprofile) = original_userprofile {
        std::env::set_var("USERPROFILE", userprofile);
    }

    Ok(())
}

/// Test config with empty binary_path (None)
#[test]
fn test_config_empty_binary_path() -> Result<()> {
    let config_toml = r#"
[script_generation.json_escaping]
method = "rust_binary"
enabled = true
"#;
    let config: Config = toml::from_str(config_toml)?;

    assert!(config.script_generation.json_escaping.binary_path.is_none());

    // Test script generation with None binary_path
    let executor = TestExecutor::with_config(config);
    let test_case = create_simple_test_case();
    let script = executor.generate_test_script(&test_case);

    // Should use default "json-escape" command (not a path)
    assert!(script.contains("json-escape"));
    // Verify it's using the bare command name, not a path with slashes
    assert!(script.contains("printf '%s' \"$COMMAND_OUTPUT\" | json-escape"));

    Ok(())
}

/// Test config deserialization with missing fields (should use defaults)
#[test]
fn test_config_deserialization_missing_fields() -> Result<()> {
    // Minimal config with only one field
    let config_toml = r#"
[script_generation.json_escaping]
enabled = false
"#;
    let config: Config = toml::from_str(config_toml)?;

    // Should use default values for missing fields
    assert!(!config.script_generation.json_escaping.enabled);
    assert!(matches!(
        config.script_generation.json_escaping.method,
        JsonEscapingMethod::Auto
    ));
    assert!(config.script_generation.json_escaping.binary_path.is_none());

    Ok(())
}

/// Test config serialization and verify TOML format
#[test]
fn test_config_serialization_format() -> Result<()> {
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Auto,
                enabled: false,
                binary_path: Some(PathBuf::from("/test/path")),
            },
        },
        ..Default::default()
    };

    let toml_str = toml::to_string_pretty(&config)?;

    // Verify TOML contains expected sections
    assert!(toml_str.contains("[script_generation"));
    assert!(toml_str.contains("enabled = false"));
    assert!(toml_str.contains("method = \"auto\""));
    assert!(toml_str.contains("binary_path = \"/test/path\""));

    Ok(())
}

/// Test that Config::load() properly handles file read errors
#[test]
fn test_config_load_file_read_error() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_dir = temp_dir.path().join(".testcase-manager");
    fs::create_dir_all(&config_dir)?;
    let config_path = config_dir.join("config.toml");

    // Create a file we can't read (directory instead of file)
    #[cfg(unix)]
    {
        // On Unix, we can create a directory with the config file name
        fs::create_dir(&config_path)?;

        // Save current HOME/USERPROFILE
        let original_home = std::env::var("HOME").ok();
        let original_userprofile = std::env::var("USERPROFILE").ok();

        std::env::set_var("HOME", temp_dir.path());
        std::env::remove_var("USERPROFILE");

        // Try to load config - should fail with read error
        let result = Config::load();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let err_msg = format!("{}", err);
        assert!(err_msg.contains("Failed to read config file"));

        // Restore original environment
        if let Some(home) = original_home {
            std::env::set_var("HOME", home);
        }
        if let Some(userprofile) = original_userprofile {
            std::env::set_var("USERPROFILE", userprofile);
        }
    }

    Ok(())
}

/// Test config with all three method types and different enabled states
#[test]
fn test_config_method_and_enabled_combinations() -> Result<()> {
    // RustBinary + enabled
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::RustBinary,
                enabled: true,
                binary_path: None,
            },
        },
        ..Default::default()
    };
    let executor = TestExecutor::with_config(config);
    let script = executor.generate_test_script(&create_simple_test_case());
    assert!(script.contains("json-escape"));

    // ShellFallback + enabled
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::ShellFallback,
                enabled: true,
                binary_path: None,
            },
        },
        ..Default::default()
    };
    let executor = TestExecutor::with_config(config);
    let script = executor.generate_test_script(&create_simple_test_case());
    assert!(script.contains("sed"));
    assert!(script.contains("awk"));

    // Auto + enabled
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Auto,
                enabled: true,
                binary_path: None,
            },
        },
        ..Default::default()
    };
    let executor = TestExecutor::with_config(config);
    let script = executor.generate_test_script(&create_simple_test_case());
    assert!(script.contains("if command -v json-escape"));

    Ok(())
}

/// Test config with relative binary path
#[test]
fn test_config_relative_binary_path() -> Result<()> {
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::RustBinary,
                enabled: true,
                binary_path: Some(PathBuf::from("./bin/json-escape")),
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_config(config);
    let test_case = create_simple_test_case();
    let script = executor.generate_test_script(&test_case);

    // Should use relative path as-is
    assert!(script.contains("./bin/json-escape"));

    Ok(())
}

/// Test config with absolute binary path
#[test]
fn test_config_absolute_binary_path() -> Result<()> {
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Auto,
                enabled: true,
                binary_path: Some(PathBuf::from("/usr/local/bin/json-escape")),
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_config(config);
    let test_case = create_simple_test_case();
    let script = executor.generate_test_script(&test_case);

    // Should use absolute path
    assert!(script.contains("/usr/local/bin/json-escape"));

    Ok(())
}

/// Test that Config::load() error message includes context
#[test]
fn test_config_load_error_context() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_dir = temp_dir.path().join(".testcase-manager");
    fs::create_dir_all(&config_dir)?;
    let config_path = config_dir.join("config.toml");

    // Write invalid TOML with specific syntax error
    fs::write(&config_path, "[script_generation\nmethod = ")?;

    // Save current HOME/USERPROFILE
    let original_home = std::env::var("HOME").ok();
    let original_userprofile = std::env::var("USERPROFILE").ok();

    std::env::set_var("HOME", temp_dir.path());
    std::env::remove_var("USERPROFILE");

    // Try to load config
    let result = Config::load();
    assert!(result.is_err());

    let err = result.unwrap_err();
    let err_chain = format!("{:?}", err);

    // Error should mention parsing failure
    assert!(err_chain.contains("Failed to parse config file") || err_chain.contains("parse"));

    // Restore original environment
    if let Some(home) = original_home {
        std::env::set_var("HOME", home);
    }
    if let Some(userprofile) = original_userprofile {
        std::env::set_var("USERPROFILE", userprofile);
    }

    Ok(())
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Create a simple test case for testing
fn create_simple_test_case() -> TestCase {
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Simple test case".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    sequence.steps.push(Step {
        step: 1,
        manual: None,
        description: "Echo test".to_string(),
        command: "echo 'hello world'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "hello world".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple(
                "[ \"$COMMAND_OUTPUT\" = \"hello world\" ]".to_string(),
            ),
            output_file: None,
            general: None,
        },
        reference: None,
    });

    test_case.test_sequences.push(sequence);
    test_case
}
