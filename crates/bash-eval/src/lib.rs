//! Bash evaluation engine for test verification expressions
//!
//! This crate provides a reusable bash evaluation engine that can:
//! - Evaluate bash expressions against execution context (exit code, output, variables)
//! - Generate bash verification scripts for embedding in test execution scripts
//! - Extract captured variables from command output using regex patterns

use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;

/// Represents a verification expression to evaluate via bash
///
/// This is independent of any test framework models to avoid tight coupling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BashExpression {
    /// A simple bash expression string (e.g., "[[ $EXIT_CODE -eq 0 ]]")
    Simple(String),
    /// A conditional bash expression with optional branches
    Conditional {
        /// The condition to evaluate
        condition: String,
        /// Commands to run if condition is true
        #[serde(skip_serializing_if = "Option::is_none")]
        if_true: Option<Vec<String>>,
        /// Commands to run if condition is false
        #[serde(skip_serializing_if = "Option::is_none")]
        if_false: Option<Vec<String>>,
        /// Commands to always run regardless of condition
        #[serde(skip_serializing_if = "Option::is_none")]
        always: Option<Vec<String>>,
    },
}

/// Context for bash evaluation - provides the environment variables
/// that verification expressions expect
#[derive(Debug, Clone)]
pub struct BashEvalContext {
    /// The exit code from the executed command (set as EXIT_CODE)
    pub exit_code: i32,
    /// The command output (set as COMMAND_OUTPUT)
    pub command_output: String,
    /// Additional captured variables from previous steps
    pub variables: HashMap<String, String>,
}

/// Evaluate a bash expression in the given context.
///
/// Returns `Ok(true)` if the expression evaluates to exit code 0 (success),
/// `Ok(false)` if it returns non-zero, and `Err` if bash execution fails.
pub fn evaluate(expression: &BashExpression, context: &BashEvalContext) -> Result<bool> {
    // Extract the expression string
    let expr_str = match expression {
        BashExpression::Simple(s) => s.as_str(),
        BashExpression::Conditional { condition, .. } => condition.as_str(),
    };

    // Handle trivial cases
    let trimmed = expr_str.trim();
    if trimmed == "true" {
        return Ok(true);
    }
    if trimmed == "false" {
        return Ok(false);
    }

    // Build and execute bash script
    let script = build_script(expr_str, context);
    match Command::new("bash").arg("-c").arg(&script).output() {
        Ok(output) => Ok(output.status.success()),
        Err(e) => {
            log::error!("Failed to execute bash script: {}", e);
            Err(anyhow::anyhow!("Failed to execute bash script: {}", e))
        }
    }
}

/// Escape a string value for safe embedding in a bash double-quoted string.
///
/// Handles: backslashes, double quotes, backticks, and newlines.
pub fn escape_for_bash(value: &str) -> String {
    value
        .replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("`", "\\`")
        .replace("\n", "\\n")
}

/// Generate bash script code for a verification expression.
///
/// Sets the given `var_name` to "true" if the expression passes (exit code 0).
/// This is used in script generation when no variable substitution is needed.
///
pub fn generate_verification_script(expression: &BashExpression, var_name: &str) -> String {
    match expression {
        BashExpression::Simple(s) => {
            // For simple expressions, just evaluate and set the variable
            format!("if {}; then\n    {}=true\nfi\n", s, var_name)
        }
        BashExpression::Conditional {
            condition,
            if_true,
            if_false,
            always,
        } => {
            let mut script = String::new();

            // Evaluate the condition and execute appropriate branch
            script.push_str(&format!("if {}; then\n", condition));
            // Set the variable to true when condition is met
            script.push_str(&format!("    {}=true\n", var_name));

            // Execute if_true commands
            if let Some(commands) = if_true {
                for cmd in commands {
                    script.push_str(&format!("    {}\n", cmd));
                }
            } else {
                script.push_str("    true # empty so bash does not fail\n");
            }

            script.push_str("else\n");

            // Execute if_false commands
            if let Some(commands) = if_false {
                for cmd in commands {
                    script.push_str(&format!("    {}\n", cmd));
                }
            } else {
                script.push_str("    true # empty so bash does not fail\n");
            }

            script.push_str("fi\n");

            // Always execute commands in always array
            if let Some(commands) = always {
                for cmd in commands {
                    script.push_str(&format!("{}\n", cmd));
                }
            }

            script
        }
    }
}

/// Generate bash script code with variable substitution support.
///
/// Used when capture variables are present and need runtime substitution.
/// This generates a script that iterates through `STEP_VAR_NAMES` and performs
/// sed-based substitution of `${var_name}` patterns.
pub fn generate_verification_with_var_subst(expression: &BashExpression, var_name: &str) -> String {
    match expression {
        BashExpression::Simple(s) => {
            // For simple expressions with potential variables, perform substitution
            let mut script = String::new();
            let escaped_expr = s
                .replace("\\", "\\\\")
                .replace("$", "\\$")
                .replace("\"", "\\\"");
            script.push_str(&format!("EXPR=\"{}\"\n", escaped_expr));
            script.push_str("if [ -n \"$CAPTURED_VAR_NAMES\" ]; then\n");
            script.push_str("    for var_name in $CAPTURED_VAR_NAMES; do\n");
            script.push_str("        eval \"var_value=\\$$var_name\"\n");
            script.push_str("        # Escape special characters for sed\n");
            script.push_str(
                "        escaped_value=$(printf '%s' \"$var_value\" | sed 's/[&/\\]/\\\\&/g')\n",
            );
            script.push_str("        # Replace ${var_name} pattern\n");
            script.push_str(
                "        EXPR=$(echo \"$EXPR\" | sed \"s/\\${$var_name}/$escaped_value/g\")\n",
            );
            script.push_str("    done\n");
            script.push_str("fi\n");
            script.push_str(&format!(
                "if eval \"$EXPR\"; then\n    {}=true\nfi\n",
                var_name
            ));
            script
        }
        BashExpression::Conditional {
            condition,
            if_true,
            if_false,
            always,
        } => {
            // For conditional expressions, substitute variables in all parts
            let mut script = String::new();

            // Prepare condition with variable substitution
            let escaped_condition = condition
                .replace("\\", "\\\\")
                .replace("$", "\\$")
                .replace("\"", "\\\"");
            script.push_str(&format!("COND_EXPR=\"{}\"\n", escaped_condition));
            script.push_str("if [ -n \"$CAPTURED_VAR_NAMES\" ]; then\n");
            script.push_str("    for var_name in $CAPTURED_VAR_NAMES; do\n");
            script.push_str("        eval \"var_value=\\$$var_name\"\n");
            script.push_str(
                "        escaped_value=$(printf '%s' \"$var_value\" | sed 's/[&/\\]/\\\\&/g')\n",
            );
            script.push_str("        COND_EXPR=$(echo \"$COND_EXPR\" | sed \"s/\\${$var_name}/$escaped_value/g\")\n");
            script.push_str("    done\n");
            script.push_str("fi\n");

            script.push_str("if eval \"$COND_EXPR\"; then\n");
            script.push_str(&format!("    {}=true\n", var_name));

            // Execute if_true commands
            if let Some(commands) = if_true {
                for cmd in commands {
                    script.push_str(&format!("    {}\n", cmd));
                }
            } else {
                script.push_str("    true # empty so bash does not fail\n");
            }

            script.push_str("else\n");

            // Execute if_false commands
            if let Some(commands) = if_false {
                for cmd in commands {
                    script.push_str(&format!("    {}\n", cmd));
                }
            } else {
                script.push_str("    true # empty so bash does not fail\n");
            }

            script.push_str("fi\n");

            // Always execute commands
            if let Some(commands) = always {
                for cmd in commands {
                    script.push_str(&format!("{}\n", cmd));
                }
            }

            script
        }
    }
}

/// Extract a captured value from text using a regex pattern.
///
/// Returns the first capture group if the pattern matches, or None otherwise.
pub fn extract_capture(text: &str, pattern: &str) -> Option<String> {
    match Regex::new(pattern) {
        Ok(regex) => regex
            .captures(text)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string())),
        Err(e) => {
            log::warn!("Invalid regex pattern '{}': {}", pattern, e);
            None
        }
    }
}

fn build_script(expr_str: &str, context: &BashEvalContext) -> String {
    let mut script = String::new();
    script.push_str(&format!("EXIT_CODE={}\n", context.exit_code));
    script.push_str(&format!(
        "COMMAND_OUTPUT=\"{}\"\n",
        escape_for_bash(&context.command_output)
    ));

    for (var_name, var_value) in &context.variables {
        script.push_str(&format!(
            "{}=\"{}\"\n",
            var_name,
            escape_for_bash(var_value)
        ));
    }

    script.push_str(&format!(
        "if {}; then\n    exit 0\nelse\n    exit 1\nfi",
        expr_str
    ));
    script
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_simple_true() {
        let expr = BashExpression::Simple("true".to_string());
        let context = BashEvalContext {
            exit_code: 0,
            command_output: String::new(),
            variables: HashMap::new(),
        };
        assert!(evaluate(&expr, &context).unwrap());
    }

    #[test]
    fn test_evaluate_simple_false() {
        let expr = BashExpression::Simple("false".to_string());
        let context = BashEvalContext {
            exit_code: 0,
            command_output: String::new(),
            variables: HashMap::new(),
        };
        assert!(!evaluate(&expr, &context).unwrap());
    }

    #[test]
    fn test_evaluate_exit_code_zero() {
        let expr = BashExpression::Simple("[[ $EXIT_CODE -eq 0 ]]".to_string());
        let context = BashEvalContext {
            exit_code: 0,
            command_output: String::new(),
            variables: HashMap::new(),
        };
        assert!(evaluate(&expr, &context).unwrap());
    }

    #[test]
    fn test_evaluate_exit_code_nonzero() {
        let expr = BashExpression::Simple("[[ $EXIT_CODE -eq 0 ]]".to_string());
        let context = BashEvalContext {
            exit_code: 1,
            command_output: String::new(),
            variables: HashMap::new(),
        };
        assert!(!evaluate(&expr, &context).unwrap());
    }

    #[test]
    fn test_evaluate_grep_output() {
        let expr = BashExpression::Simple("grep -q 'hello' <<< \"$COMMAND_OUTPUT\"".to_string());
        let context = BashEvalContext {
            exit_code: 0,
            command_output: "hello world".to_string(),
            variables: HashMap::new(),
        };
        assert!(evaluate(&expr, &context).unwrap());
    }

    #[test]
    fn test_evaluate_grep_no_match() {
        let expr = BashExpression::Simple("grep -q 'goodbye' <<< \"$COMMAND_OUTPUT\"".to_string());
        let context = BashEvalContext {
            exit_code: 0,
            command_output: "hello world".to_string(),
            variables: HashMap::new(),
        };
        assert!(!evaluate(&expr, &context).unwrap());
    }

    #[test]
    fn test_evaluate_with_variable() {
        let expr = BashExpression::Simple("[[ \"$MY_VAR\" == \"expected\" ]]".to_string());
        let mut vars = HashMap::new();
        vars.insert("MY_VAR".to_string(), "expected".to_string());
        let context = BashEvalContext {
            exit_code: 0,
            command_output: String::new(),
            variables: vars,
        };
        assert!(evaluate(&expr, &context).unwrap());
    }

    #[test]
    fn test_escape_for_bash() {
        assert_eq!(escape_for_bash("hello"), "hello");
        assert_eq!(escape_for_bash("hello\\world"), "hello\\\\world");
        assert_eq!(escape_for_bash("hello\"world"), "hello\\\"world");
        assert_eq!(escape_for_bash("hello`world"), "hello\\`world");
        assert_eq!(escape_for_bash("hello\nworld"), "hello\\nworld");
    }

    #[test]
    fn test_generate_verification_script_simple() {
        let expr = BashExpression::Simple("[[ $EXIT_CODE -eq 0 ]]".to_string());
        let script = generate_verification_script(&expr, "MY_VAR");
        assert!(script.contains("if [[ $EXIT_CODE -eq 0 ]]; then"));
        assert!(script.contains("MY_VAR=true"));
    }

    #[test]
    fn test_generate_verification_script_conditional() {
        let expr = BashExpression::Conditional {
            condition: "[[ $EXIT_CODE -eq 0 ]]".to_string(),
            if_true: Some(vec!["echo success".to_string()]),
            if_false: None,
            always: None,
        };
        let script = generate_verification_script(&expr, "MY_VAR");
        assert!(script.contains("if [[ $EXIT_CODE -eq 0 ]]; then"));
        assert!(script.contains("MY_VAR=true"));
        assert!(script.contains("echo success"));
    }

    #[test]
    fn test_extract_capture_with_match() {
        let text = r#"{"token":"abc123","session":"xyz"}"#;
        let pattern = r#""token":"([^"]+)""#;
        let result = extract_capture(text, pattern);
        assert_eq!(result, Some("abc123".to_string()));
    }

    #[test]
    fn test_extract_capture_no_match() {
        let text = r#"{"token":"abc123"}"#;
        let pattern = r#""session":"([^"]+)""#;
        let result = extract_capture(text, pattern);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_capture_invalid_regex() {
        let text = "hello";
        let pattern = "[invalid(regex";
        let result = extract_capture(text, pattern);
        assert_eq!(result, None);
    }
}
