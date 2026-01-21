use crate::models::TestCase;
use anyhow::{Context, Result};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use tempfile::NamedTempFile;

pub struct TestExecutor;

impl TestExecutor {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_test_script(&self, test_case: &TestCase) -> String {
        let mut script = String::new();

        script.push_str("#!/bin/bash\n");
        script.push_str("set -e\n\n");

        script.push_str("# Test Case: ");
        script.push_str(&test_case.id);
        script.push('\n');
        script.push_str("# Description: ");
        script.push_str(&test_case.description);
        script.push_str("\n\n");

        if !test_case.general_initial_conditions.is_empty() {
            script.push_str("# General Initial Conditions\n");
            for (key, values) in &test_case.general_initial_conditions {
                script.push_str(&format!("# {}: {}\n", key, values.join(", ")));
            }
            script.push('\n');
        }

        if !test_case.initial_conditions.is_empty() {
            script.push_str("# Initial Conditions\n");
            for (key, values) in &test_case.initial_conditions {
                script.push_str(&format!("# {}: {}\n", key, values.join(", ")));
            }
            script.push('\n');
        }

        for sequence in &test_case.test_sequences {
            script.push_str(&format!(
                "# Test Sequence {}: {}\n",
                sequence.id, sequence.name
            ));
            let lines = sequence.description.split("\n");
            lines.for_each(|line| script.push_str(&format!("# {}\n", line)));

            if !sequence.initial_conditions.is_empty() {
                script.push_str("# Sequence Initial Conditions\n");
                for (key, values) in &sequence.initial_conditions {
                    script.push_str(&format!("# {}: {}\n", key, values.join(", ")));
                }
            }
            script.push('\n');

            for step in &sequence.steps {
                script.push_str(&format!("# Step {}: {}\n", step.step, step.description));

                if step.manual == Some(true) {
                    script.push_str("# MANUAL STEP - Skipping automated execution\n");
                    script.push_str(&format!("# Command: {}\n", step.command));
                    script.push_str(&format!("# Expected result: {}\n", step.expected.result));
                    script.push_str(&format!("# Expected output: {}\n\n", step.expected.output));
                    continue;
                }

                script.push_str("COMMAND_OUTPUT=\"\"\n");
                script.push_str(&format!("COMMAND_OUTPUT=$({})\n", step.command));
                script.push_str("EXIT_CODE=$?\n\n");

                script.push_str(&format!(
                    "# Verification result expression: {}\n",
                    step.verification.result
                ));
                script.push_str(&format!(
                    "# Verification output expression: {}\n",
                    step.verification.output
                ));

                script.push_str("VERIFICATION_RESULT_PASS=false\n");
                script.push_str("VERIFICATION_OUTPUT_PASS=false\n\n");

                script.push_str(&format!("if {}; then\n", step.verification.result));
                script.push_str("    VERIFICATION_RESULT_PASS=true\n");
                script.push_str("fi\n\n");

                script.push_str(&format!("if {}; then\n", step.verification.output));
                script.push_str("    VERIFICATION_OUTPUT_PASS=true\n");
                script.push_str("fi\n\n");

                script.push_str("if [ \"$VERIFICATION_RESULT_PASS\" = true ] && [ \"$VERIFICATION_OUTPUT_PASS\" = true ]; then\n");
                script.push_str(&format!(
                    "    echo \"[PASS] Step {}: {}\"\n",
                    step.step, step.description
                ));
                script.push_str("else\n");
                script.push_str(&format!(
                    "    echo \"[FAIL] Step {}: {}\"\n",
                    step.step, step.description
                ));
                script.push_str("    echo \"  Command: ");
                script.push_str(&step.command.replace("\"", "\\\""));
                script.push_str("\"\n");
                script.push_str("    echo \"  Exit code: $EXIT_CODE\"\n");
                script.push_str("    echo \"  Output: $COMMAND_OUTPUT\"\n");
                script.push_str("    echo \"  Result verification: $VERIFICATION_RESULT_PASS\"\n");
                script.push_str("    echo \"  Output verification: $VERIFICATION_OUTPUT_PASS\"\n");
                script.push_str("    exit 1\n");
                script.push_str("fi\n\n");
            }
        }

        script.push_str("echo \"All test sequences completed successfully\"\n");
        script.push_str("exit 0\n");

        script
    }

    pub fn execute_test_case(&self, test_case: &TestCase) -> Result<()> {
        let script_content = self.generate_test_script(test_case);

        let temp_file =
            NamedTempFile::new().context("Failed to create temporary file for test script")?;

        fs::write(temp_file.path(), script_content)
            .context("Failed to write test script to temporary file")?;

        let mut perms = fs::metadata(temp_file.path())
            .context("Failed to get file metadata")?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(temp_file.path(), perms)
            .context("Failed to set executable permissions on test script")?;

        let output = Command::new(temp_file.path())
            .output()
            .context("Failed to execute test script")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            anyhow::bail!(
                "Test execution failed:\nStdout:\n{}\nStderr:\n{}",
                stdout,
                stderr
            );
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);

        Ok(())
    }
}

impl Default for TestExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Expected, Step, TestSequence, Verification};
    use std::collections::HashMap;

    #[test]
    fn test_generate_test_script_basic() {
        let executor = TestExecutor::new();
        let mut test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Basic test".to_string(),
        );

        let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());
        let step = Step {
            step: 1,
            manual: None,
            description: "Echo test".to_string(),
            command: "echo 'hello'".to_string(),
            expected: Expected {
                success: Some(true),
                result: "[ $EXIT_CODE -eq 0 ]".to_string(),
                output: "[ \"$COMMAND_OUTPUT\" = \"hello\" ]".to_string(),
            },
            verification: Verification {
                result: "[ $EXIT_CODE -eq 0 ]".to_string(),
                output: "[ \"$COMMAND_OUTPUT\" = \"hello\" ]".to_string(),
            },
        };
        sequence.steps.push(step);
        test_case.test_sequences.push(sequence);

        let script = executor.generate_test_script(&test_case);

        assert!(script.contains("#!/bin/bash"));
        assert!(script.contains("TC001"));
        assert!(script.contains("Basic test"));
        assert!(script.contains("Echo test"));
        assert!(script.contains("echo 'hello'"));
        assert!(script.contains("COMMAND_OUTPUT"));
        assert!(script.contains("EXIT_CODE=$?"));
        assert!(script.contains("[ $EXIT_CODE -eq 0 ]"));
    }

    #[test]
    fn test_generate_test_script_with_manual_step() {
        let executor = TestExecutor::new();
        let mut test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test with manual step".to_string(),
        );

        let mut sequence =
            TestSequence::new(1, "Seq1".to_string(), "Sequence with manual".to_string());
        let step = Step {
            step: 1,
            manual: Some(true),
            description: "Manual verification".to_string(),
            command: "ssh device".to_string(),
            expected: Expected {
                success: Some(true),
                result: "connected".to_string(),
                output: "success".to_string(),
            },
            verification: Verification {
                result: "true".to_string(),
                output: "true".to_string(),
            },
        };
        sequence.steps.push(step);
        test_case.test_sequences.push(sequence);

        let script = executor.generate_test_script(&test_case);

        assert!(script.contains("MANUAL STEP"));
        assert!(script.contains("Manual verification"));
        assert!(script.contains("ssh device"));
    }

    #[test]
    fn test_generate_test_script_with_initial_conditions() {
        let executor = TestExecutor::new();
        let mut test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test with conditions".to_string(),
        );

        let mut general_conditions = HashMap::new();
        general_conditions.insert("Device".to_string(), vec!["Powered on".to_string()]);
        test_case.general_initial_conditions = general_conditions;

        let mut conditions = HashMap::new();
        conditions.insert("Connection".to_string(), vec!["Established".to_string()]);
        test_case.initial_conditions = conditions;

        let sequence = TestSequence::new(1, "Seq1".to_string(), "Sequence".to_string());
        test_case.test_sequences.push(sequence);

        let script = executor.generate_test_script(&test_case);

        assert!(script.contains("General Initial Conditions"));
        assert!(script.contains("Device: Powered on"));
        assert!(script.contains("Initial Conditions"));
        assert!(script.contains("Connection: Established"));
    }
}
