use crate::models::{TestCase, TestStepExecutionEntry};
use anyhow::{Context, Result};
use chrono::Local;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct TestExecutor {
    output_dir: Option<PathBuf>,
}

impl TestExecutor {
    pub fn new() -> Self {
        Self { output_dir: None }
    }

    pub fn with_output_dir<P: Into<PathBuf>>(output_dir: P) -> Self {
        Self {
            output_dir: Some(output_dir.into()),
        }
    }

    pub fn generate_test_script_with_json_output(
        &self,
        test_case: &TestCase,
        json_output_path: &Path,
    ) -> String {
        let mut script = String::new();

        script.push_str("#!/bin/bash\n");
        script.push_str("set -euo pipefail\n\n");

        script.push_str("# Test Case: ");
        script.push_str(&test_case.id);
        script.push('\n');
        script.push_str("# Description: ");
        script.push_str(&test_case.description);
        script.push_str("\n\n");

        script.push_str(&format!("JSON_LOG=\"{}\"\n", json_output_path.display()));
        script.push_str("TIMESTAMP=$(date  +\"%Y-%m-%dT%H:%M:%S\")\n\n");

        // Add trap to ensure JSON file is properly closed on any exit
        script.push_str("# Trap to ensure JSON file is closed properly on exit\n");
        script.push_str("cleanup() {\n");
        script.push_str("    if [ -f \"$JSON_LOG\" ]; then\n");
        script.push_str("        # Check if JSON_LOG ends with '[' or ','\n");
        script.push_str("        LAST_CHAR=$(tail -c 2 \"$JSON_LOG\" | head -c 1)\n");
        script.push_str("        if [ \"$LAST_CHAR\" != \"]\" ]; then\n");
        script.push_str("            echo '' >> \"$JSON_LOG\"\n");
        script.push_str("            echo ']' >> \"$JSON_LOG\"\n");
        script.push_str("        fi\n");
        script.push_str("        # Validate JSON\n");
        script.push_str("        if command -v jq >/dev/null 2>&1; then\n");
        script.push_str("            if ! jq empty \"$JSON_LOG\" >/dev/null 2>&1; then\n");
        script.push_str("                echo \"500 - Internal Script Error: Generated JSON is not valid\" >&2\n");
        script.push_str("                exit 1\n");
        script.push_str("            fi\n");
        script.push_str("        fi\n");
        script.push_str("    fi\n");
        script.push_str("}\n");
        script.push_str("trap cleanup EXIT\n\n");

        script.push_str("echo '[' > \"$JSON_LOG\"\n");
        script.push_str("FIRST_ENTRY=true\n\n");

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
                script.push_str("set +e\n");
                script.push_str(&format!("COMMAND_OUTPUT=$({})\n", step.command));
                script.push_str("EXIT_CODE=$?\n");
                script.push_str("set -e\n\n");

                let escaped_command = step.command.replace("\\", "\\\\").replace("\"", "\\\"");
                script.push_str("# Escape output for JSON\n");
                script.push_str("OUTPUT_ESCAPED=$(printf '%s' \"$COMMAND_OUTPUT\" | sed 's/\\\\/\\\\\\\\/g' | sed 's/\"/\\\\\"/g' | sed -e ':a' -e '$!N;s/\\n/\\\\n/;ta')\n\n");

                script.push_str("if [ \"$FIRST_ENTRY\" = false ]; then\n");
                script.push_str("    echo ',' >> \"$JSON_LOG\"\n");
                script.push_str("fi\n");
                script.push_str("FIRST_ENTRY=false\n\n");

                script.push_str("cat >> \"$JSON_LOG\" << EOF\n");
                script.push_str("  {\n");
                script.push_str(&format!("    \"test_sequence\": {},\n", sequence.id));
                script.push_str(&format!("    \"step\": {},\n", step.step));
                script.push_str(&format!("    \"command\": \"{}\",\n", escaped_command));
                script.push_str("    \"exit_code\": $EXIT_CODE,\n");
                script.push_str("    \"output\": \"$OUTPUT_ESCAPED\",\n");
                script.push_str("    \"timestamp\": \"$TIMESTAMP\"\n");
                script.push_str("  }\n");
                script.push_str("EOF\n\n");

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

        script.push_str("echo ']' >> \"$JSON_LOG\"\n\n");

        // Add JSON schema validation
        script.push_str("# Validate JSON against schema\n");
        script.push_str("if command -v jq >/dev/null 2>&1; then\n");
        script.push_str("    if ! jq empty \"$JSON_LOG\" >/dev/null 2>&1; then\n");
        script.push_str(
            "        echo \"500 - Internal Script Error: Generated JSON is not valid\"\n",
        );
        script.push_str("        exit 1\n");
        script.push_str("    fi\n");
        script.push_str("fi\n\n");

        script.push_str("echo \"All test sequences completed successfully\"\n");
        script.push_str("exit 0\n");

        script
    }

    pub fn generate_test_script(&self, test_case: &TestCase) -> String {
        let json_path_str = format!("{}_execution_log.json", test_case.id);
        let json_path = Path::new(&json_path_str);
        self.generate_test_script_with_json_output(test_case, json_path)
    }

    pub fn execute_test_case(&self, test_case: &TestCase) -> Result<()> {
        let mut execution_entries = Vec::new();
        let mut execution_error = None;

        for sequence in &test_case.test_sequences {
            for step in &sequence.steps {
                if step.manual == Some(true) {
                    println!(
                        "[SKIP] Step {} (Sequence {}): {} - Manual step",
                        step.step, sequence.id, step.description
                    );
                    continue;
                }

                println!(
                    "[RUN] Step {} (Sequence {}): {}",
                    step.step, sequence.id, step.description
                );

                match Command::new("bash").arg("-c").arg(&step.command).output() {
                    Ok(output) => {
                        let exit_code = output.status.code().unwrap_or(-1);
                        let command_output = String::from_utf8_lossy(&output.stdout).to_string();

                        let timestamp = Local::now().to_rfc3339();
                        let entry = TestStepExecutionEntry::with_timestamp(
                            sequence.id,
                            step.step,
                            step.command.clone(),
                            exit_code,
                            command_output.clone(),
                            timestamp,
                        );

                        execution_entries.push(entry);
                    }
                    Err(e) => {
                        // Store the error but continue to write the log
                        // Create an entry with exit code -1 to indicate execution failure
                        let timestamp = Local::now().to_rfc3339();
                        let entry = TestStepExecutionEntry::with_timestamp(
                            sequence.id,
                            step.step,
                            step.command.clone(),
                            -1,
                            format!("Failed to execute: {}", e),
                            timestamp,
                        );
                        execution_entries.push(entry);

                        if execution_error.is_none() {
                            execution_error = Some(anyhow::anyhow!(
                                "Failed to execute command '{}': {}",
                                step.command,
                                e
                            ));
                        }
                    }
                }
            }
        }

        // Always write the execution log, even if there were errors
        self.write_execution_log(test_case, &execution_entries)?;

        // Now return any execution error after the log is written
        if let Some(err) = execution_error {
            return Err(err);
        }

        Ok(())
    }

    fn write_execution_log(
        &self,
        test_case: &TestCase,
        entries: &[TestStepExecutionEntry],
    ) -> Result<()> {
        let log_filename = format!("{}_execution_log.json", test_case.id);
        let log_path = if let Some(ref output_dir) = self.output_dir {
            output_dir.join(&log_filename)
        } else {
            PathBuf::from(&log_filename)
        };

        let json_content = serde_json::to_string_pretty(entries)
            .context("Failed to serialize execution entries to JSON")?;

        fs::write(&log_path, json_content)
            .context(format!("Failed to write log file: {}", log_path.display()))?;

        Ok(())
    }

    pub fn generate_execution_log_template(
        &self,
        test_case: &TestCase,
        script_path: &Path,
    ) -> Result<()> {
        // Determine the output path for the JSON log
        let json_log_path = if let Some(parent) = script_path.parent() {
            let stem = script_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or(&test_case.id);
            parent.join(format!("{}_execution_log.json", stem))
        } else {
            Path::new(&format!("{}_execution_log.json", test_case.id)).to_path_buf()
        };

        // Create execution entries template with all steps from the test case
        let mut template_entries: Vec<TestStepExecutionEntry> = Vec::new();

        // Base timestamp for template (arbitrary starting point in local timezone)
        let base_time = chrono::DateTime::parse_from_rfc3339("2026-01-22T10:30:00Z")
            .unwrap()
            .with_timezone(&Local);
        let mut step_index = 0;

        for sequence in &test_case.test_sequences {
            for step in &sequence.steps {
                // Skip manual steps
                if step.manual == Some(true) {
                    continue;
                }

                // Parse expected exit code from expected.result field
                let exit_code = step.expected.result.parse::<i32>().unwrap_or(0);

                // Use expected output, handling special cases
                let output = if step.expected.output == "true" || step.expected.output.is_empty() {
                    String::new()
                } else {
                    // Replace escaped newlines with actual newlines
                    step.expected.output.replace("\\n", "\n")
                };

                // Generate timestamp incrementing by 1 second per step
                let timestamp = base_time + chrono::Duration::seconds(step_index);

                // Create a template entry with expected values
                let entry = TestStepExecutionEntry {
                    test_sequence: sequence.id,
                    step: step.step,
                    command: step.command.clone(),
                    exit_code,
                    output,
                    timestamp: Some(timestamp.to_rfc3339()),
                };

                template_entries.push(entry);
                step_index += 1;
            }
        }

        // Serialize to JSON
        let json_content = serde_json::to_string_pretty(&template_entries)
            .context("Failed to serialize execution log template to JSON")?;

        // Write to file
        fs::write(&json_log_path, json_content).context(format!(
            "Failed to write execution log template: {}",
            json_log_path.display()
        ))?;

        println!(
            "Execution log template generated: {}",
            json_log_path.display()
        );

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
