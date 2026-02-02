use crate::bdd_parser::BddStepRegistry;
use crate::models::{TestCase, TestStepExecutionEntry};
use anyhow::{Context, Result};
use chrono::Local;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct TestExecutor {
    output_dir: Option<PathBuf>,
}

/// Check if a test case uses variables that require bash 4.0+ associative arrays
fn test_case_uses_variables(test_case: &TestCase) -> bool {
    // Check if any sequence has variables
    for sequence in &test_case.test_sequences {
        if let Some(ref vars) = sequence.variables {
            if !vars.is_empty() {
                return true;
            }
        }

        // Check if any step has capture_vars or uses variable substitution
        for step in &sequence.steps {
            if let Some(ref capture_vars) = step.capture_vars {
                if !capture_vars.is_empty() {
                    return true;
                }
            }

            // Check if command uses variable substitution
            if step.command.contains("${") || step.command.contains("$STEP_VARS") {
                return true;
            }

            // Check if verification expressions use variables
            if step.verification.result.contains("${")
                || step.verification.result.contains("$STEP_VARS")
            {
                return true;
            }
            if step.verification.output.contains("${")
                || step.verification.output.contains("$STEP_VARS")
            {
                return true;
            }
            if let Some(ref output_file) = step.verification.output_file {
                if output_file.contains("${") || output_file.contains("$STEP_VARS") {
                    return true;
                }
            }
        }
    }

    false
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
        script.push_str("TIMESTAMP=$(date +\"%Y-%m-%dT%H:%M:%S\")\n\n");

        // Check if test case uses variables (requires bash 4.0+ associative arrays)
        let uses_variables = test_case_uses_variables(test_case);

        if uses_variables {
            // Initialize STEP_VARS associative array
            script.push_str("# Initialize STEP_VARS associative array\n");
            script.push_str("declare -a STEP_VARS\n\n");
        }

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

        // Instantiate BDD step registry
        let bdd_registry = BddStepRegistry::load_from_toml("data/bdd_step_definitions.toml")
            .unwrap_or_else(|e| {
                eprintln!(
                    "Warning: Failed to load BDD step definitions from data/bdd_step_definitions.toml: {}",
                    e
                );
                eprintln!("BDD patterns will not be available. Initial conditions will be treated as comments.");
                BddStepRegistry::new()
            });

        if !test_case.general_initial_conditions.is_empty() {
            script.push_str("# General Initial Conditions\n");
            for (key, values) in &test_case.general_initial_conditions {
                for value in values {
                    if let Some(command) = bdd_registry.try_parse_as_bdd(value) {
                        script.push_str(&format!("# {}: {}\n", key, value));
                        script.push_str(&format!("{}\n", command));
                    } else {
                        script.push_str(&format!("# {}: {}\n", key, value));
                    }
                }
            }
            script.push('\n');
        }

        if !test_case.initial_conditions.is_empty() {
            script.push_str("# Initial Conditions\n");
            for (key, values) in &test_case.initial_conditions {
                for value in values {
                    if let Some(command) = bdd_registry.try_parse_as_bdd(value) {
                        script.push_str(&format!("# {}: {}\n", key, value));
                        script.push_str(&format!("{}\n", command));
                    } else {
                        script.push_str(&format!("# {}: {}\n", key, value));
                    }
                }
            }
            script.push('\n');
        }

        for sequence in &test_case.test_sequences {
            script.push_str(&format!(
                "# Test Sequence {}: {}\n",
                sequence.id, sequence.name
            ));
            for line in sequence.description.split('\n') {
                script.push_str(&format!("# {}\n", line));
            }

            if !sequence.initial_conditions.is_empty() {
                script.push_str("# Sequence Initial Conditions\n");
                for (key, values) in &sequence.initial_conditions {
                    for value in values {
                        if let Some(command) = bdd_registry.try_parse_as_bdd(value) {
                            script.push_str(&format!("# {}: {}\n", key, value));
                            script.push_str(&format!("{}\n", command));
                        } else {
                            script.push_str(&format!("# {}: {}\n", key, value));
                        }
                    }
                }
            }
            script.push('\n');

            // Initialize sequence-level variables
            if let Some(ref variables) = sequence.variables {
                if !variables.is_empty() {
                    script.push_str("# Initialize sequence variables\n");
                    for (var_name, var_value) in variables {
                        script.push_str(&format!(
                            "STEP_VARS[{}]={}\n",
                            var_name,
                            bash_escape(var_value)
                        ));
                    }
                    script.push('\n');
                }
            }

            for step in &sequence.steps {
                script.push_str(&format!("# Step {}: {}\n", step.step, step.description));

                if step.manual == Some(true) {
                    script.push_str(&format!(
                        "echo \"Step {}: {}\"\n",
                        step.step, step.description
                    ));
                    script.push_str(&format!(
                        "echo \"Command: {}\"\n",
                        step.command.replace("\"", "\\\"")
                    ));
                    script.push_str("echo \"INFO: This is a manual step. You must perform this action manually.\"\n");
                    script.push_str(
                        "if [[ \"${DEBIAN_FRONTEND}\" != 'noninteractive' && -t 0 ]]; then\n",
                    );
                    script.push_str("    read -p \"Press ENTER to continue...\"\n");
                    script.push_str("else\n");
                    script.push_str("    echo \"Non-interactive mode detected, skipping manual step confirmation.\"\n");
                    script.push_str("fi\n\n");
                    continue;
                }

                script.push_str(&format!(
                    "LOG_FILE=\"{}_sequence-{}_step-{}.actual.log\"\n",
                    test_case.id, sequence.id, step.step
                ));
                script.push_str("COMMAND_OUTPUT=\"\"\n");
                script.push_str("set +e\n");

                // Check if the command needs variable substitution
                let needs_substitution =
                    step.command.contains("${") || step.command.contains("$STEP_VARS");

                if needs_substitution {
                    // Generate bash code to perform variable substitution on the command
                    // Store the original command in a variable
                    let escaped_command = step.command.replace("\\", "\\\\").replace("\"", "\\\"");
                    script.push_str(&format!("ORIGINAL_COMMAND=\"{}\"\n", escaped_command));

                    // Perform variable substitution: replace ${var_name} or $STEP_VARS[var_name] patterns
                    script.push_str("SUBSTITUTED_COMMAND=\"$ORIGINAL_COMMAND\"\n");
                    script.push_str("for var_name in \"${!STEP_VARS[@]}\"; do\n");
                    script.push_str("    var_value=\"${STEP_VARS[$var_name]}\"\n");
                    script.push_str("    # Escape special characters for sed\n");
                    script.push_str(
                        "    escaped_value=$(printf '%s' \"$var_value\" | sed 's/[&/\\]/\\\\&/g')\n",
                    );
                    script.push_str("    # Replace ${var_name} pattern\n");
                    script.push_str("    SUBSTITUTED_COMMAND=$(echo \"$SUBSTITUTED_COMMAND\" | sed \"s/\\${$var_name}/$escaped_value/g\")\n");
                    script.push_str("    # Replace ${STEP_VARS[var_name]} pattern\n");
                    script.push_str("    SUBSTITUTED_COMMAND=$(echo \"$SUBSTITUTED_COMMAND\" | sed \"s/\\${STEP_VARS\\[$var_name\\]}/$escaped_value/g\")\n");
                    script.push_str("done\n");

                    // Wrap command in subshell with braces and redirect stderr to stdout before piping to tee
                    // This ensures both stdout and stderr are captured in the log file
                    script.push_str(
                        "COMMAND_OUTPUT=$({ eval \"$SUBSTITUTED_COMMAND\"; } 2>&1 | tee \"$LOG_FILE\")\n",
                    );
                } else {
                    // No substitution needed - inline the command directly
                    script.push_str(&format!(
                        "COMMAND_OUTPUT=$({{ {}; }} 2>&1 | tee \"$LOG_FILE\")\n",
                        step.command
                    ));
                }

                script.push_str("EXIT_CODE=$?\n");
                script.push_str("set -e\n\n");

                // Variable capture: extract values from COMMAND_OUTPUT using regex patterns
                if let Some(ref capture_vars) = step.capture_vars {
                    if !capture_vars.is_empty() {
                        script.push_str("# Capture variables from output\n");
                        for (var_name, pattern) in capture_vars {
                            script.push_str(&format!(
                                "STEP_VARS[{}]=$(echo \"$COMMAND_OUTPUT\" | grep -oP {} | head -n 1 || echo \"\")\n",
                                var_name,
                                bash_escape(pattern)
                            ));
                        }
                        script.push('\n');
                    }
                }

                script.push_str(&format!(
                    "# Verification result expression: {}\n",
                    step.verification.result
                ));

                // Determine which output verification to use
                let output_verification =
                    if let Some(ref output_file_verification) = step.verification.output_file {
                        script.push_str(&format!(
                            "# Verification output expression (from file): {}\n",
                            output_file_verification
                        ));
                        output_file_verification.as_str()
                    } else {
                        script.push_str(&format!(
                            "# Verification output expression (from variable): {}\n",
                            step.verification.output
                        ));
                        step.verification.output.as_str()
                    };

                script.push_str("VERIFICATION_RESULT_PASS=false\n");
                script.push_str("VERIFICATION_OUTPUT_PASS=false\n\n");

                // Perform variable substitution on verification expressions
                let result_needs_subst = step.verification.result.contains("${")
                    || step.verification.result.contains("$STEP_VARS");
                let escaped_result_expr = step
                    .verification
                    .result
                    .replace("\\", "\\\\")
                    .replace("$", "\\$")
                    .replace("\"", "\\\"");
                script.push_str(&format!("RESULT_EXPR=\"{}\"\n", escaped_result_expr));

                if result_needs_subst && uses_variables {
                    script.push_str("for var_name in \"${!STEP_VARS[@]}\"; do\n");
                    script.push_str("    var_value=\"${STEP_VARS[$var_name]}\"\n");
                    script.push_str("    # Escape special characters for sed\n");
                    script.push_str(
                        "    escaped_value=$(printf '%s' \"$var_value\" | sed 's/[&/\\]/\\\\&/g')\n",
                    );
                    script.push_str("    # Replace ${var_name} pattern\n");
                    script.push_str("    RESULT_EXPR=$(echo \"$RESULT_EXPR\" | sed \"s/\\${$var_name}/$escaped_value/g\")\n");
                    script.push_str("    # Replace ${STEP_VARS[var_name]} pattern\n");
                    script.push_str("    RESULT_EXPR=$(echo \"$RESULT_EXPR\" | sed \"s/\\${STEP_VARS\\[$var_name\\]}/$escaped_value/g\")\n");
                    script.push_str("done\n");
                }
                script.push('\n');

                let output_needs_subst = output_verification.contains("${")
                    || output_verification.contains("$STEP_VARS");
                let escaped_output_expr = output_verification
                    .replace("\\", "\\\\")
                    .replace("$", "\\$")
                    .replace("\"", "\\\"");
                script.push_str(&format!("OUTPUT_EXPR=\"{}\"\n", escaped_output_expr));

                if output_needs_subst && uses_variables {
                    script.push_str("for var_name in \"${!STEP_VARS[@]}\"; do\n");
                    script.push_str("    var_value=\"${STEP_VARS[$var_name]}\"\n");
                    script.push_str("    # Escape special characters for sed\n");
                    script.push_str(
                        "    escaped_value=$(printf '%s' \"$var_value\" | sed 's/[&/\\]/\\\\&/g')\n",
                    );
                    script.push_str("    # Replace ${var_name} pattern\n");
                    script.push_str("    OUTPUT_EXPR=$(echo \"$OUTPUT_EXPR\" | sed \"s/\\${$var_name}/$escaped_value/g\")\n");
                    script.push_str("    # Replace ${STEP_VARS[var_name]} pattern\n");
                    script.push_str("    OUTPUT_EXPR=$(echo \"$OUTPUT_EXPR\" | sed \"s/\\${STEP_VARS\\[$var_name\\]}/$escaped_value/g\")\n");
                    script.push_str("done\n");
                }
                script.push('\n');

                script.push_str("if eval \"$RESULT_EXPR\"; then\n");
                script.push_str("    VERIFICATION_RESULT_PASS=true\n");
                script.push_str("fi\n\n");

                script.push_str("if eval \"$OUTPUT_EXPR\"; then\n");
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
        let mut verification_failed = false;

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
                        let command_output = String::from_utf8_lossy(&output.stdout)
                            .trim_end()
                            .to_string();

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

                        // Perform verification
                        let result_verification_passed = self.evaluate_verification(
                            &step.verification.result,
                            exit_code,
                            &command_output,
                        )?;
                        let output_verification_passed = self.evaluate_verification(
                            &step.verification.output,
                            exit_code,
                            &command_output,
                        )?;

                        if result_verification_passed && output_verification_passed {
                            println!(
                                "[PASS] Step {} (Sequence {}): {}",
                                step.step, sequence.id, step.description
                            );
                        } else {
                            println!(
                                "[FAIL] Step {} (Sequence {}): {}",
                                step.step, sequence.id, step.description
                            );
                            println!("  Command: {}", step.command);
                            println!("  EXIT_CODE: {}", exit_code);
                            println!("  COMMAND_OUTPUT: {}", command_output);
                            println!("  Result verification: {}", result_verification_passed);
                            println!("  Output verification: {}", output_verification_passed);
                            verification_failed = true;

                            if execution_error.is_none() {
                                execution_error = Some(anyhow::anyhow!(
                                    "Test execution failed: Step {} verification failed",
                                    step.step
                                ));
                            }
                        }
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

        if !verification_failed {
            println!("All test sequences completed successfully");
        }

        Ok(())
    }

    fn evaluate_verification(
        &self,
        expression: &str,
        exit_code: i32,
        command_output: &str,
    ) -> Result<bool> {
        // Handle simple true/false cases
        let trimmed = expression.trim();
        if trimmed == "true" {
            return Ok(true);
        }
        if trimmed == "false" {
            return Ok(false);
        }

        // Build a bash script that evaluates the verification expression
        // We need to set EXIT_CODE and COMMAND_OUTPUT variables and then evaluate the expression
        let script = format!(
            r#"EXIT_CODE={}
COMMAND_OUTPUT="{}"
if {}; then
    exit 0
else
    exit 1
fi"#,
            exit_code,
            command_output
                .replace("\\", "\\\\")
                .replace("\"", "\\\"")
                .replace("\n", "\\n"),
            expression
        );

        match Command::new("bash").arg("-c").arg(&script).output() {
            Ok(output) => Ok(output.status.success()),
            Err(_) => Ok(false),
        }
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
            .expect("Failed to parse hardcoded RFC3339 timestamp")
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

/// Performs variable substitution in a command string using the STEP_VARS associative array.
///
/// This function substitutes patterns like `${STEP_VARS[var_name]}` or `$var_name` with their
/// corresponding values from the provided variables map. The substituted values are properly
/// escaped for safe use in bash commands.
///
/// # Arguments
///
/// * `command` - The command string containing variable references to substitute
/// * `step_vars` - A reference to a HashMap containing variable names and their values
///
/// # Returns
///
/// A new String with all variable references substituted and properly escaped
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use testcase_manager::executor::substitute_variables;
///
/// let mut vars = HashMap::new();
/// vars.insert("username".to_string(), "john.doe".to_string());
/// vars.insert("path".to_string(), "/home/user".to_string());
///
/// let cmd = "echo ${STEP_VARS[username]} lives at $path";
/// let result = substitute_variables(cmd, &vars);
/// assert_eq!(result, "echo 'john.doe' lives at '/home/user'");
/// ```
pub fn substitute_variables(command: &str, step_vars: &HashMap<String, String>) -> String {
    let mut result = command.to_string();

    // Pattern for ${STEP_VARS[var_name]} - using lazy matching and capturing the var name
    let array_pattern = Regex::new(r"\$\{STEP_VARS\[([^\]]+)\]\}").unwrap();

    // Pattern for $var_name (word characters, digits, underscores)
    let simple_pattern = Regex::new(r"\$([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();

    // First, substitute ${STEP_VARS[var_name]} patterns
    result = array_pattern
        .replace_all(&result, |caps: &regex::Captures| {
            let var_name = &caps[1];
            if let Some(value) = step_vars.get(var_name) {
                bash_escape(value)
            } else {
                // If variable not found, leave the reference unchanged
                caps[0].to_string()
            }
        })
        .to_string();

    // Then, substitute $var_name patterns
    // We need to be careful not to substitute variables that are not in STEP_VARS
    // or are bash special variables
    result = simple_pattern
        .replace_all(&result, |caps: &regex::Captures| {
            let var_name = &caps[1];
            if let Some(value) = step_vars.get(var_name) {
                bash_escape(value)
            } else {
                // If variable not found, leave the reference unchanged
                caps[0].to_string()
            }
        })
        .to_string();

    result
}

/// Escapes a string value for safe use in bash commands.
///
/// This function wraps the value in single quotes and escapes any single quotes
/// within the value by replacing them with '\'' (end quote, escaped quote, start quote).
///
/// # Arguments
///
/// * `value` - The string value to escape
///
/// # Returns
///
/// A bash-escaped string wrapped in single quotes
///
/// # Examples
///
/// ```
/// use testcase_manager::executor::bash_escape;
///
/// assert_eq!(bash_escape("hello"), "'hello'");
/// assert_eq!(bash_escape("it's"), "'it'\\''s'");
/// assert_eq!(bash_escape("a\"b"), "'a\"b'");
/// ```
pub fn bash_escape(value: &str) -> String {
    // In bash, the safest way to escape a string is to wrap it in single quotes
    // and escape any single quotes by ending the quote, adding an escaped quote, and starting again
    format!("'{}'", value.replace('\'', r"'\''"))
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
            capture_vars: None,
            expected: Expected {
                success: Some(true),
                result: "[ $EXIT_CODE -eq 0 ]".to_string(),
                output: "[ \"$COMMAND_OUTPUT\" = \"hello\" ]".to_string(),
            },
            verification: Verification {
                result: "[ $EXIT_CODE -eq 0 ]".to_string(),
                output: "[ \"$COMMAND_OUTPUT\" = \"hello\" ]".to_string(),
                output_file: None,
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
            capture_vars: None,
            expected: Expected {
                success: Some(true),
                result: "connected".to_string(),
                output: "success".to_string(),
            },
            verification: Verification {
                result: "true".to_string(),
                output: "true".to_string(),
                output_file: None,
            },
        };
        sequence.steps.push(step);
        test_case.test_sequences.push(sequence);

        let script = executor.generate_test_script(&test_case);

        assert!(script.contains("Manual verification"));
        assert!(script.contains("ssh device"));
        assert!(script.contains("echo \"Step 1: Manual verification\""));
        assert!(script.contains("echo \"Command: ssh device\""));
        assert!(script.contains(
            "echo \"INFO: This is a manual step. You must perform this action manually.\""
        ));
        assert!(script.contains("read -p \"Press ENTER to continue...\""));
        assert!(
            script.contains("if [[ \"${DEBIAN_FRONTEND}\" != 'noninteractive' && -t 0 ]]; then\n")
        );
        assert!(
            script.contains("Non-interactive mode detected, skipping manual step confirmation.")
        );
        assert!(!script.contains("MANUAL STEP - Skipping"));
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

    #[test]
    fn test_log_file_variable_in_generated_script() {
        let executor = TestExecutor::new();
        let mut test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test for LOG_FILE".to_string(),
        );

        let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());
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
                result: "[ $EXIT_CODE -eq 0 ]".to_string(),
                output: "true".to_string(),
                output_file: None,
            },
        };
        sequence.steps.push(step);
        test_case.test_sequences.push(sequence);

        let script = executor.generate_test_script(&test_case);

        assert!(
            script.contains("LOG_FILE=\"TC001_sequence-1_step-1.actual.log\""),
            "Script should contain LOG_FILE variable declaration with .actual.log extension"
        );
    }

    #[test]
    fn test_tee_command_in_generated_script() {
        let executor = TestExecutor::new();
        let mut test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test for tee command".to_string(),
        );

        let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());
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
                result: "[ $EXIT_CODE -eq 0 ]".to_string(),
                output: "true".to_string(),
                output_file: None,
            },
        };
        sequence.steps.push(step);
        test_case.test_sequences.push(sequence);

        let script = executor.generate_test_script(&test_case);

        assert!(
            script.contains("| tee \"$LOG_FILE\""),
            "Script should contain tee command piping to $LOG_FILE"
        );
    }

    #[test]
    fn test_log_file_and_tee_for_multiple_steps() {
        let executor = TestExecutor::new();
        let mut test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC002".to_string(),
            "Test with multiple steps".to_string(),
        );

        let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());

        let step1 = Step {
            step: 1,
            manual: None,
            description: "First step".to_string(),
            command: "echo 'step1'".to_string(),
            capture_vars: None,
            expected: Expected {
                success: Some(true),
                result: "0".to_string(),
                output: "step1".to_string(),
            },
            verification: Verification {
                result: "[ $EXIT_CODE -eq 0 ]".to_string(),
                output: "true".to_string(),
                output_file: None,
            },
        };

        let step2 = Step {
            step: 2,
            manual: None,
            description: "Second step".to_string(),
            command: "echo 'step2'".to_string(),
            capture_vars: None,
            expected: Expected {
                success: Some(true),
                result: "0".to_string(),
                output: "step2".to_string(),
            },
            verification: Verification {
                result: "[ $EXIT_CODE -eq 0 ]".to_string(),
                output: "true".to_string(),
                output_file: None,
            },
        };

        sequence.steps.push(step1);
        sequence.steps.push(step2);
        test_case.test_sequences.push(sequence);

        let script = executor.generate_test_script(&test_case);

        assert!(
            script.contains("LOG_FILE=\"TC002_sequence-1_step-1.actual.log\""),
            "Script should contain LOG_FILE for step 1"
        );
        assert!(
            script.contains("LOG_FILE=\"TC002_sequence-1_step-2.actual.log\""),
            "Script should contain LOG_FILE for step 2"
        );

        let tee_count = script.matches("| tee \"$LOG_FILE\"").count();
        assert_eq!(
            tee_count, 2,
            "Script should contain tee command twice for two non-manual steps"
        );
    }

    #[test]
    fn test_log_file_and_tee_for_multiple_sequences() {
        let executor = TestExecutor::new();
        let mut test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC003".to_string(),
            "Test with multiple sequences".to_string(),
        );

        let mut sequence1 = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());
        let step1 = Step {
            step: 1,
            manual: None,
            description: "Step in sequence 1".to_string(),
            command: "echo 'seq1'".to_string(),
            capture_vars: None,
            expected: Expected {
                success: Some(true),
                result: "0".to_string(),
                output: "seq1".to_string(),
            },
            verification: Verification {
                result: "[ $EXIT_CODE -eq 0 ]".to_string(),
                output: "true".to_string(),
                output_file: None,
            },
        };
        sequence1.steps.push(step1);

        let mut sequence2 = TestSequence::new(2, "Seq2".to_string(), "Second sequence".to_string());
        let step2 = Step {
            step: 1,
            manual: None,
            description: "Step in sequence 2".to_string(),
            command: "echo 'seq2'".to_string(),
            capture_vars: None,
            expected: Expected {
                success: Some(true),
                result: "0".to_string(),
                output: "seq2".to_string(),
            },
            verification: Verification {
                result: "[ $EXIT_CODE -eq 0 ]".to_string(),
                output: "true".to_string(),
                output_file: None,
            },
        };
        sequence2.steps.push(step2);

        test_case.test_sequences.push(sequence1);
        test_case.test_sequences.push(sequence2);

        let script = executor.generate_test_script(&test_case);

        assert!(
            script.contains("LOG_FILE=\"TC003_sequence-1_step-1.actual.log\""),
            "Script should contain LOG_FILE for sequence 1"
        );
        assert!(
            script.contains("LOG_FILE=\"TC003_sequence-2_step-1.actual.log\""),
            "Script should contain LOG_FILE for sequence 2"
        );

        let tee_count = script.matches("| tee \"$LOG_FILE\"").count();
        assert_eq!(
            tee_count, 2,
            "Script should contain tee command twice for two sequences"
        );
    }

    #[test]
    fn test_manual_steps_skip_log_file_and_tee() {
        let executor = TestExecutor::new();
        let mut test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC004".to_string(),
            "Test with manual step".to_string(),
        );

        let mut sequence =
            TestSequence::new(1, "Seq1".to_string(), "Sequence with manual".to_string());

        let manual_step = Step {
            step: 1,
            manual: Some(true),
            description: "Manual step".to_string(),
            command: "manual command".to_string(),
            capture_vars: None,
            expected: Expected {
                success: Some(true),
                result: "0".to_string(),
                output: "output".to_string(),
            },
            verification: Verification {
                result: "true".to_string(),
                output: "true".to_string(),
                output_file: None,
            },
        };

        let auto_step = Step {
            step: 2,
            manual: None,
            description: "Automated step".to_string(),
            command: "echo 'auto'".to_string(),
            capture_vars: None,
            expected: Expected {
                success: Some(true),
                result: "0".to_string(),
                output: "auto".to_string(),
            },
            verification: Verification {
                result: "[ $EXIT_CODE -eq 0 ]".to_string(),
                output: "true".to_string(),
                output_file: None,
            },
        };

        sequence.steps.push(manual_step);
        sequence.steps.push(auto_step);
        test_case.test_sequences.push(sequence);

        let script = executor.generate_test_script(&test_case);

        let log_file_count = script.matches("LOG_FILE=").count();
        assert_eq!(
            log_file_count, 1,
            "Script should contain only one LOG_FILE declaration (manual step skipped)"
        );

        assert!(
            script.contains("LOG_FILE=\"TC004_sequence-1_step-2.actual.log\""),
            "Script should contain LOG_FILE only for non-manual step"
        );

        let tee_count = script.matches("| tee \"$LOG_FILE\"").count();
        assert_eq!(
            tee_count, 1,
            "Script should contain only one tee command (manual step skipped)"
        );
    }

    #[test]
    fn test_log_file_pattern_with_regex() {
        let executor = TestExecutor::new();
        let mut test_case = TestCase::new(
            "REQ999".to_string(),
            1,
            1,
            "TC999".to_string(),
            "Test LOG_FILE pattern".to_string(),
        );

        let mut sequence = TestSequence::new(5, "Seq5".to_string(), "Test sequence".to_string());
        let step = Step {
            step: 10,
            manual: None,
            description: "Test step".to_string(),
            command: "ls -la".to_string(),
            capture_vars: None,
            expected: Expected {
                success: Some(true),
                result: "0".to_string(),
                output: "files".to_string(),
            },
            verification: Verification {
                result: "[ $EXIT_CODE -eq 0 ]".to_string(),
                output: "true".to_string(),
                output_file: None,
            },
        };
        sequence.steps.push(step);
        test_case.test_sequences.push(sequence);

        let script = executor.generate_test_script(&test_case);

        use regex::Regex;
        let log_file_pattern = Regex::new(r#"LOG_FILE="[^"]*\.actual\.log""#).unwrap();
        assert!(
            log_file_pattern.is_match(&script),
            "Script should match LOG_FILE pattern with .actual.log extension"
        );

        let tee_pattern = Regex::new(r#"\| tee "\$LOG_FILE""#).unwrap();
        assert!(
            tee_pattern.is_match(&script),
            "Script should match tee command pattern with $LOG_FILE variable"
        );
    }

    #[test]
    fn test_stderr_capture_in_generated_script() {
        let executor = TestExecutor::new();
        let mut test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test stderr capture".to_string(),
        );

        let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());
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
                result: "[ $EXIT_CODE -eq 0 ]".to_string(),
                output: "true".to_string(),
                output_file: None,
            },
        };
        sequence.steps.push(step);
        test_case.test_sequences.push(sequence);

        let script = executor.generate_test_script(&test_case);

        assert!(
            script.contains("2>&1 | tee"),
            "Script should contain '2>&1 | tee' pattern to capture stderr before tee command"
        );
    }

    #[test]
    fn test_bash_escape_basic() {
        assert_eq!(bash_escape("hello"), "'hello'");
        assert_eq!(bash_escape("hello world"), "'hello world'");
        assert_eq!(bash_escape("123"), "'123'");
    }

    #[test]
    fn test_bash_escape_with_single_quotes() {
        assert_eq!(bash_escape("it's"), "'it'\\''s'");
        assert_eq!(bash_escape("'quoted'"), "''\\''quoted'\\'''");
        assert_eq!(bash_escape("don't"), "'don'\\''t'");
    }

    #[test]
    fn test_bash_escape_with_special_chars() {
        assert_eq!(bash_escape("a\"b"), "'a\"b'");
        assert_eq!(bash_escape("a$b"), "'a$b'");
        assert_eq!(bash_escape("a`b"), "'a`b'");
        assert_eq!(bash_escape("a\\b"), "'a\\b'");
        assert_eq!(bash_escape("a!b"), "'a!b'");
        assert_eq!(bash_escape("a&b"), "'a&b'");
        assert_eq!(bash_escape("a|b"), "'a|b'");
        assert_eq!(bash_escape("a;b"), "'a;b'");
    }

    #[test]
    fn test_bash_escape_empty_string() {
        assert_eq!(bash_escape(""), "''");
    }

    #[test]
    fn test_substitute_variables_array_syntax() {
        let mut vars = HashMap::new();
        vars.insert("username".to_string(), "john.doe".to_string());
        vars.insert("server".to_string(), "example.com".to_string());

        let cmd = "ssh ${STEP_VARS[username]}@${STEP_VARS[server]}";
        let result = substitute_variables(cmd, &vars);
        assert_eq!(result, "ssh 'john.doe'@'example.com'");
    }

    #[test]
    fn test_substitute_variables_simple_syntax() {
        let mut vars = HashMap::new();
        vars.insert("username".to_string(), "jane".to_string());
        vars.insert("path".to_string(), "/home/user".to_string());

        let cmd = "echo $username lives at $path";
        let result = substitute_variables(cmd, &vars);
        assert_eq!(result, "echo 'jane' lives at '/home/user'");
    }

    #[test]
    fn test_substitute_variables_mixed_syntax() {
        let mut vars = HashMap::new();
        vars.insert("user".to_string(), "admin".to_string());
        vars.insert("host".to_string(), "server1".to_string());
        vars.insert("port".to_string(), "8080".to_string());

        let cmd = "curl http://$user:password@${STEP_VARS[host]}:$port/api";
        let result = substitute_variables(cmd, &vars);
        assert_eq!(result, "curl http://'admin':password@'server1':'8080'/api");
    }

    #[test]
    fn test_substitute_variables_undefined_var() {
        let mut vars = HashMap::new();
        vars.insert("defined".to_string(), "value".to_string());

        let cmd = "echo $defined $undefined ${STEP_VARS[missing]}";
        let result = substitute_variables(cmd, &vars);
        assert_eq!(result, "echo 'value' $undefined ${STEP_VARS[missing]}");
    }

    #[test]
    fn test_substitute_variables_with_special_chars_in_values() {
        let mut vars = HashMap::new();
        vars.insert("msg".to_string(), "hello 'world'!".to_string());
        vars.insert("cmd".to_string(), "ls -la | grep test".to_string());

        let cmd = "echo ${STEP_VARS[msg]} && $cmd";
        let result = substitute_variables(cmd, &vars);
        assert_eq!(
            result,
            "echo 'hello '\\''world'\\''!' && 'ls -la | grep test'"
        );
    }

    #[test]
    fn test_substitute_variables_empty_command() {
        let vars = HashMap::new();
        let cmd = "";
        let result = substitute_variables(cmd, &vars);
        assert_eq!(result, "");
    }

    #[test]
    fn test_substitute_variables_no_vars_in_command() {
        let mut vars = HashMap::new();
        vars.insert("unused".to_string(), "value".to_string());

        let cmd = "echo hello world";
        let result = substitute_variables(cmd, &vars);
        assert_eq!(result, "echo hello world");
    }

    #[test]
    fn test_substitute_variables_multiple_same_var() {
        let mut vars = HashMap::new();
        vars.insert("val".to_string(), "xyz".to_string());

        let cmd = "echo $val $val ${STEP_VARS[val]}";
        let result = substitute_variables(cmd, &vars);
        assert_eq!(result, "echo 'xyz' 'xyz' 'xyz'");
    }

    #[test]
    fn test_substitute_variables_var_with_numbers_and_underscores() {
        let mut vars = HashMap::new();
        vars.insert("var_name_1".to_string(), "value1".to_string());
        vars.insert("_var2".to_string(), "value2".to_string());
        vars.insert("VAR3".to_string(), "value3".to_string());

        let cmd = "echo $var_name_1 $_var2 $VAR3";
        let result = substitute_variables(cmd, &vars);
        assert_eq!(result, "echo 'value1' 'value2' 'value3'");
    }

    #[test]
    fn test_substitute_variables_preserves_bash_special_vars() {
        let vars = HashMap::new();

        let cmd = "echo $? $$ $! $# $@ $* $0 $1 $EXIT_CODE $COMMAND_OUTPUT";
        let result = substitute_variables(cmd, &vars);
        assert_eq!(
            result,
            "echo $? $$ $! $# $@ $* $0 $1 $EXIT_CODE $COMMAND_OUTPUT"
        );
    }

    #[test]
    fn test_substitute_variables_complex_scenario() {
        let mut vars = HashMap::new();
        vars.insert("user".to_string(), "test_user".to_string());
        vars.insert("file".to_string(), "config.txt".to_string());
        vars.insert("dir".to_string(), "/tmp/test dir".to_string());

        let cmd = "scp $user@server:${STEP_VARS[dir]}/${STEP_VARS[file]} .";
        let result = substitute_variables(cmd, &vars);
        assert_eq!(
            result,
            "scp 'test_user'@server:'/tmp/test dir'/'config.txt' ."
        );
    }

    #[test]
    fn test_substitute_variables_escaped_dollar_signs() {
        let mut vars = HashMap::new();
        vars.insert("price".to_string(), "100".to_string());

        // Note: In actual bash, \$ would prevent expansion.
        // This test shows current behavior - escaped $ is still processed
        let cmd = "echo The price is $price";
        let result = substitute_variables(cmd, &vars);
        assert_eq!(result, "echo The price is '100'");
    }

    #[test]
    fn test_generate_test_script_with_capture_vars() {
        let executor = TestExecutor::new();
        let mut test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test with variable capture".to_string(),
        );

        let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());

        let mut capture_vars = std::collections::BTreeMap::new();
        capture_vars.insert("user_id".to_string(), r#"(?<=user_id=)\d+"#.to_string());
        capture_vars.insert(
            "token".to_string(),
            r#"(?<=token=)[a-zA-Z0-9]+"#.to_string(),
        );

        let step = Step {
            step: 1,
            manual: None,
            description: "Extract variables".to_string(),
            command: "echo 'user_id=12345 token=abc123'".to_string(),
            capture_vars: Some(capture_vars),
            expected: Expected {
                success: Some(true),
                result: "[ $EXIT_CODE -eq 0 ]".to_string(),
                output: "true".to_string(),
            },
            verification: Verification {
                result: "[ $EXIT_CODE -eq 0 ]".to_string(),
                output: "true".to_string(),
                output_file: None,
            },
        };
        sequence.steps.push(step);
        test_case.test_sequences.push(sequence);

        let script = executor.generate_test_script(&test_case);

        assert!(script.contains("# Capture variables from output"));
        assert!(script.contains("STEP_VARS[user_id]=$(echo \"$COMMAND_OUTPUT\" | grep -oP"));
        assert!(script.contains("STEP_VARS[token]=$(echo \"$COMMAND_OUTPUT\" | grep -oP"));
        assert!(script.contains("| head -n 1 || echo \"\")"));
    }

    #[test]
    fn test_variable_capture_in_generated_script() {
        let executor = TestExecutor::new();
        let mut test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test variable capture initialization".to_string(),
        );

        let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());

        let mut capture_vars = std::collections::BTreeMap::new();
        capture_vars.insert(
            "session_id".to_string(),
            r#"(?<=session=)[a-f0-9]+"#.to_string(),
        );

        let step = Step {
            step: 1,
            manual: None,
            description: "Capture session ID".to_string(),
            command: "echo 'session=abc123def'".to_string(),
            capture_vars: Some(capture_vars),
            expected: Expected {
                success: Some(true),
                result: "[ $EXIT_CODE -eq 0 ]".to_string(),
                output: "true".to_string(),
            },
            verification: Verification {
                result: "[ $EXIT_CODE -eq 0 ]".to_string(),
                output: "true".to_string(),
                output_file: None,
            },
        };
        sequence.steps.push(step);
        test_case.test_sequences.push(sequence);

        let script = executor.generate_test_script(&test_case);

        // Verify STEP_VARS array initialization
        assert!(script.contains("declare -a STEP_VARS"));
        assert!(script.contains("# Initialize STEP_VARS associative array"));

        // Verify capture code generation
        assert!(script.contains("# Capture variables from output"));
        assert!(script.contains("STEP_VARS[session_id]=$(echo \"$COMMAND_OUTPUT\" | grep -oP '(?<=session=)[a-f0-9]+' | head -n 1 || echo \"\")"));
    }

    #[test]
    fn test_variable_substitution_in_commands() {
        let executor = TestExecutor::new();
        let mut test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test variable substitution in commands".to_string(),
        );

        let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());

        let step = Step {
            step: 1,
            manual: None,
            description: "Use variable in command".to_string(),
            command: "echo ${user_name} works at ${STEP_VARS[company]}".to_string(),
            capture_vars: None,
            expected: Expected {
                success: Some(true),
                result: "[ $EXIT_CODE -eq 0 ]".to_string(),
                output: "true".to_string(),
            },
            verification: Verification {
                result: "[ $EXIT_CODE -eq 0 ]".to_string(),
                output: "true".to_string(),
                output_file: None,
            },
        };
        sequence.steps.push(step);
        test_case.test_sequences.push(sequence);

        let script = executor.generate_test_script(&test_case);

        // Verify substitution logic is present
        assert!(script.contains("ORIGINAL_COMMAND="));
        assert!(script.contains("SUBSTITUTED_COMMAND=\"$ORIGINAL_COMMAND\""));
        assert!(script.contains("for var_name in \"${!STEP_VARS[@]}\"; do"));
        assert!(script.contains("var_value=\"${STEP_VARS[$var_name]}\""));
        assert!(script.contains("# Replace ${var_name} pattern"));
        assert!(script.contains("SUBSTITUTED_COMMAND=$(echo \"$SUBSTITUTED_COMMAND\" | sed \"s/\\${$var_name}/$escaped_value/g\")"));
        assert!(script.contains("# Replace ${STEP_VARS[var_name]} pattern"));
        assert!(script.contains("SUBSTITUTED_COMMAND=$(echo \"$SUBSTITUTED_COMMAND\" | sed \"s/\\${STEP_VARS\\[$var_name\\]}/$escaped_value/g\")"));
        assert!(script.contains(
            "COMMAND_OUTPUT=$({ eval \"$SUBSTITUTED_COMMAND\"; } 2>&1 | tee \"$LOG_FILE\")"
        ));
    }

    #[test]
    fn test_variable_substitution_in_verification() {
        let executor = TestExecutor::new();
        let mut test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test variable substitution in verification".to_string(),
        );

        let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());

        let step = Step {
            step: 1,
            manual: None,
            description: "Verify with variables".to_string(),
            command: "echo 'status: OK'".to_string(),
            capture_vars: None,
            expected: Expected {
                success: Some(true),
                result: "[ $EXIT_CODE -eq ${expected_code} ]".to_string(),
                output: "[[ \"$COMMAND_OUTPUT\" == *\"${STEP_VARS[status]}\"* ]]".to_string(),
            },
            verification: Verification {
                result: "[ $EXIT_CODE -eq ${expected_code} ]".to_string(),
                output: "[[ \"$COMMAND_OUTPUT\" == *\"${STEP_VARS[status]}\"* ]]".to_string(),
                output_file: None,
            },
        };
        sequence.steps.push(step);
        test_case.test_sequences.push(sequence);

        let script = executor.generate_test_script(&test_case);

        // Verify substitution in result expression
        assert!(script.contains("RESULT_EXPR="));
        assert!(script.contains("for var_name in \"${!STEP_VARS[@]}\"; do"));
        assert!(script.contains(
            "RESULT_EXPR=$(echo \"$RESULT_EXPR\" | sed \"s/\\${$var_name}/$escaped_value/g\")"
        ));
        assert!(script.contains("RESULT_EXPR=$(echo \"$RESULT_EXPR\" | sed \"s/\\${STEP_VARS\\[$var_name\\]}/$escaped_value/g\")"));

        // Verify substitution in output expression
        assert!(script.contains("OUTPUT_EXPR="));
        assert!(script.contains(
            "OUTPUT_EXPR=$(echo \"$OUTPUT_EXPR\" | sed \"s/\\${$var_name}/$escaped_value/g\")"
        ));
        assert!(script.contains("OUTPUT_EXPR=$(echo \"$OUTPUT_EXPR\" | sed \"s/\\${STEP_VARS\\[$var_name\\]}/$escaped_value/g\")"));

        // Verify evaluation of substituted expressions
        assert!(script.contains("if eval \"$RESULT_EXPR\"; then"));
        assert!(script.contains("if eval \"$OUTPUT_EXPR\"; then"));
    }

    #[test]
    fn test_multiple_variables_from_single_step() {
        let executor = TestExecutor::new();
        let mut test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test multiple variable captures from single step".to_string(),
        );

        let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());

        let mut capture_vars = std::collections::BTreeMap::new();
        capture_vars.insert("user_id".to_string(), r#"(?<=user_id=)\d+"#.to_string());
        capture_vars.insert(
            "session_token".to_string(),
            r#"(?<=token=)[A-Z0-9]+"#.to_string(),
        );
        capture_vars.insert("timestamp".to_string(), r#"(?<=ts=)\d{10}"#.to_string());

        let step = Step {
            step: 1,
            manual: None,
            description: "Capture multiple variables".to_string(),
            command: "echo 'user_id=12345 token=ABC123XYZ ts=1234567890'".to_string(),
            capture_vars: Some(capture_vars),
            expected: Expected {
                success: Some(true),
                result: "[ $EXIT_CODE -eq 0 ]".to_string(),
                output: "true".to_string(),
            },
            verification: Verification {
                result: "[ $EXIT_CODE -eq 0 ]".to_string(),
                output: "true".to_string(),
                output_file: None,
            },
        };
        sequence.steps.push(step);
        test_case.test_sequences.push(sequence);

        let script = executor.generate_test_script(&test_case);

        // Verify all three variables are captured
        assert!(script.contains("# Capture variables from output"));
        assert!(script.contains("STEP_VARS[user_id]=$(echo \"$COMMAND_OUTPUT\" | grep -oP '(?<=user_id=)\\d+' | head -n 1 || echo \"\")"));
        assert!(script.contains("STEP_VARS[session_token]=$(echo \"$COMMAND_OUTPUT\" | grep -oP '(?<=token=)[A-Z0-9]+' | head -n 1 || echo \"\")"));
        assert!(script.contains("STEP_VARS[timestamp]=$(echo \"$COMMAND_OUTPUT\" | grep -oP '(?<=ts=)\\d{10}' | head -n 1 || echo \"\")"));

        // Verify the capture block appears exactly once for this step
        let capture_count = script.matches("# Capture variables from output").count();
        assert_eq!(
            capture_count, 1,
            "Should have exactly one capture block for the single step"
        );
    }
}
