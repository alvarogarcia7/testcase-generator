use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use testcase_manager::BatchVerificationReport;
use testcase_manager::JUnitTestSuite;
use testcase_manager::LogCleaner;
use testcase_manager::TestCaseStorage;
use testcase_manager::TestVerifier;
use testcase_manager::VerificationTestExecutionLog;

#[derive(Parser)]
#[command(name = "test-verify")]
#[command(version)]
#[command(
    about = "Test verification tool for comparing test execution logs against test case definitions"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Verify execution log against test case
    /// Clean and display an execution log
    Clean {
        /// Path to the execution log file (YAML)
        #[arg(value_name = "LOG_FILE")]
        log_file: PathBuf,
    },
    /// Verify a single test execution log against a test case
    Single {
        /// Path to test execution log file
        #[arg(short, long)]
        log: PathBuf,

        /// Test case ID to verify against
        #[arg(short, long)]
        test_case_id: String,

        /// Path to test case storage directory
        #[arg(short = 'd', long, default_value = "testcases")]
        test_case_dir: PathBuf,

        /// Output format (text, json, junit)
        #[arg(short, long, default_value = "text")]
        format: String,

        /// Enable verbose output with detailed step-by-step comparisons
        #[arg(short, long)]
        verbose: bool,
    },

    /// Batch verify multiple test execution logs
    Batch {
        /// Path(s) to test execution log file(s)
        #[arg(short, long, required = true)]
        logs: Vec<PathBuf>,

        /// Path to test case storage directory
        #[arg(short = 'd', long, default_value = "testcases")]
        test_case_dir: PathBuf,

        /// Output format (text, json, junit)
        #[arg(short, long, default_value = "text")]
        format: String,

        /// Output file path (optional, defaults to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Enable verbose output with detailed step-by-step comparisons
        #[arg(short, long)]
        verbose: bool,
    },

    /// Parse and display test execution log contents
    ParseLog {
        /// Path to test execution log file
        #[arg(short, long)]
        log: PathBuf,

        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Clean { log_file } => clean_command(log_file),
        Commands::Single {
            log,
            test_case_id,
            test_case_dir,
            format,
            verbose,
        } => handle_single_verify(log, test_case_id, test_case_dir, format, verbose),

        Commands::Batch {
            logs,
            test_case_dir,
            format,
            output,
            verbose,
        } => handle_batch_verify(logs, test_case_dir, format, output, verbose),

        Commands::ParseLog { log, format } => handle_parse_log(log, format),
    }
}

fn verify_command(log_file: PathBuf, test_case_file: PathBuf) -> Result<()> {
    let log_content = fs::read_to_string(&log_file)
        .context(format!("Failed to read log file: {}", log_file.display()))?;

    let test_case_content = fs::read_to_string(&test_case_file).context(format!(
        "Failed to read test case file: {}",
        test_case_file.display()
    ))?;

    let mut execution_log: testcase_manager::TestExecutionLog =
        serde_yaml::from_str(&log_content).context("Failed to parse execution log YAML")?;

    let test_case: testcase_manager::TestCase =
        serde_yaml::from_str(&test_case_content).context("Failed to parse test case YAML")?;

    let cleaner = LogCleaner::new();
    execution_log = cleaner.clean_execution_log(&execution_log);

    println!("=== Legacy Verify Command ===");
    println!("Test Case ID: {}", test_case.id);
    println!(
        "Execution Log: test_case_id={}, sequence_id={}",
        execution_log.test_case_id, execution_log.sequence_id
    );
    println!();
    println!("Note: This is a legacy command. Please use 'single' or 'batch' commands for proper verification.");

    Ok(())
}

fn clean_command(log_file: PathBuf) -> Result<()> {
    let log_content = fs::read_to_string(&log_file)
        .context(format!("Failed to read log file: {}", log_file.display()))?;

    let execution_log: testcase_manager::TestExecutionLog =
        serde_yaml::from_str(&log_content).context("Failed to parse execution log YAML")?;

    let cleaner = LogCleaner::new();
    let cleaned_log = cleaner.clean_execution_log(&execution_log);

    let cleaned_yaml =
        serde_yaml::to_string(&cleaned_log).context("Failed to serialize cleaned log to YAML")?;

    println!("{}", cleaned_yaml);

    Ok(())
}

fn handle_parse_log(log_path: PathBuf, format: String) -> Result<()> {
    let storage =
        TestCaseStorage::new("testcases").context("Failed to initialize test case storage")?;
    let verifier = TestVerifier::from_storage(storage);

    let logs = verifier
        .parse_log_file(&log_path)
        .context("Failed to parse test execution log")?;

    match format.as_str() {
        "json" => {
            let json =
                serde_json::to_string_pretty(&logs).context("Failed to serialize logs to JSON")?;
            println!("{}", json);
        }
        _ => {
            println!(
                "Parsed {} log entries from {}",
                logs.len(),
                log_path.display()
            );
            println!();
            for log in logs {
                println!("Test Case: {}", log.test_case_id);
                println!("  Sequence: {}, Step: {}", log.sequence_id, log.step_number);
                println!("  Success: {:?}", log.success);
                println!("  Result: {}", log.actual_result);
                println!("  Output: {}", log.actual_output);
                if let Some(ts) = log.timestamp {
                    println!("  Timestamp: {}", ts);
                }
                println!();
            }
        }
    }

    Ok(())
}

fn print_verification_result(result: &testcase_manager::TestCaseVerificationResult) {
    println!("═══════════════════════════════════════════════════════════");
    println!("Test Case: {}", result.test_case_id);
    println!("Description: {}", result.description);
    println!("═══════════════════════════════════════════════════════════");
    println!();

    println!(
        "Overall Status: {}",
        if result.overall_pass {
            "✓ PASS"
        } else {
            "✗ FAIL"
        }
    );
    println!(
        "Steps: {} total, {} passed, {} failed, {} not executed",
        result.total_steps, result.passed_steps, result.failed_steps, result.not_executed_steps
    );
    println!();

    for seq_result in &result.sequences {
        println!("───────────────────────────────────────────────────────────");
        println!(
            "Sequence #{}: {} ({})",
            seq_result.sequence_id,
            seq_result.name,
            if seq_result.all_steps_passed {
                "✓ PASS"
            } else {
                "✗ FAIL"
            }
        );
        println!("───────────────────────────────────────────────────────────");

        for step_result in &seq_result.step_results {
            match step_result {
                testcase_manager::StepVerificationResultEnum::Pass { step, description } => {
                    println!("  ✓ Step {}: {} - PASS", step, description);
                }
                testcase_manager::StepVerificationResultEnum::Fail {
                    step,
                    description,
                    expected,
                    actual_result,
                    actual_output,
                    reason,
                } => {
                    println!("  ✗ Step {}: {} - FAIL", step, description);
                    println!("    Reason: {}", reason);
                    println!("    Expected:");
                    println!("      Result: {}", expected.result);
                    println!("      Output: {}", expected.output);
                    if let Some(success) = expected.success {
                        println!("      Success: {}", success);
                    }
                    println!("    Actual:");
                    println!("      Result: {}", actual_result);
                    println!("      Output: {}", actual_output);
                }
                testcase_manager::StepVerificationResultEnum::NotExecuted { step, description } => {
                    println!("  ○ Step {}: {} - NOT EXECUTED", step, description);
                }
            }
        }
        println!();
    }
}

fn print_verification_result_verbose(
    result: &testcase_manager::TestCaseVerificationResult,
    test_case: &testcase_manager::TestCase,
    execution_logs: &[VerificationTestExecutionLog],
) {
    use std::collections::HashMap;

    println!("═══════════════════════════════════════════════════════════");
    println!("VERBOSE TEST VERIFICATION REPORT");
    println!("═══════════════════════════════════════════════════════════");
    println!();
    println!("Test Case: {}", result.test_case_id);
    println!("Description: {}", result.description);
    println!();

    // Create a lookup map for execution logs
    let mut log_map: HashMap<(i64, i64), &VerificationTestExecutionLog> = HashMap::new();
    for log in execution_logs {
        if log.test_case_id == result.test_case_id {
            log_map.insert((log.sequence_id, log.step_number), log);
        }
    }

    // Process each sequence
    for seq_result in &result.sequences {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("SEQUENCE #{}: {}", seq_result.sequence_id, seq_result.name);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!();

        // Get the sequence from the test case for additional details
        let sequence = test_case
            .test_sequences
            .iter()
            .find(|s| s.id == seq_result.sequence_id);

        // Process each step
        for step_result in &seq_result.step_results {
            let step_num = step_result.step_number();
            let log_entry = log_map.get(&(seq_result.sequence_id, step_num));

            // Get step details from test case
            let step_details = sequence.and_then(|s| s.steps.iter().find(|st| st.step == step_num));

            println!(
                "┌─ Step {} ─────────────────────────────────────────────────",
                step_num
            );

            match step_result {
                testcase_manager::StepVerificationResultEnum::Pass { description, .. } => {
                    println!("│ Description: {}", description);
                    println!("│ Status: ✓ PASS");
                    println!("│");

                    if let Some(step) = step_details {
                        println!("│ Command: {}", step.command);
                        println!("│");
                        println!("│ Expected (from YAML):");
                        if let Some(success) = step.expected.success {
                            println!(
                                "│   Exit Code: {} (success={})",
                                if success { "0" } else { "non-zero" },
                                success
                            );
                        }
                        println!("│   Result: {}", step.expected.result);
                        println!("│   Output: {}", step.expected.output);
                        if !step.verification.result.is_empty() {
                            println!(
                                "│   Verification Expression (result): {}",
                                step.verification.result
                            );
                        }
                        if !step.verification.output.is_empty() {
                            println!(
                                "│   Verification Expression (output): {}",
                                step.verification.output
                            );
                        }
                    }

                    if let Some(log) = log_entry {
                        println!("│");
                        println!("│ Actual (from JSON log):");
                        if let Some(success) = log.success {
                            println!(
                                "│   Exit Code: {} (success={})",
                                if success { "0" } else { "non-zero" },
                                success
                            );
                        }
                        println!("│   Result: {}", log.actual_result);
                        println!("│   Output: {}", log.actual_output);
                    }

                    println!("│");
                    println!("│ ✓ All comparisons PASSED");
                }

                testcase_manager::StepVerificationResultEnum::Fail {
                    description,
                    expected,
                    actual_result,
                    actual_output,
                    reason,
                    ..
                } => {
                    println!("│ Description: {}", description);
                    println!("│ Status: ✗ FAIL");
                    println!("│");

                    if let Some(step) = step_details {
                        println!("│ Command: {}", step.command);
                        println!("│");
                    }

                    println!("│ Expected (from YAML):");
                    if let Some(success) = expected.success {
                        println!(
                            "│   Exit Code: {} (success={})",
                            if success { "0" } else { "non-zero" },
                            success
                        );
                    }
                    println!("│   Result: {}", expected.result);
                    println!("│   Output: {}", expected.output);

                    if let Some(step) = step_details {
                        if !step.verification.result.is_empty() {
                            println!(
                                "│   Verification Expression (result): {}",
                                step.verification.result
                            );
                        }
                        if !step.verification.output.is_empty() {
                            println!(
                                "│   Verification Expression (output): {}",
                                step.verification.output
                            );
                        }
                    }

                    println!("│");
                    println!("│ Actual (from JSON log):");
                    if let Some(log) = log_entry {
                        if let Some(success) = log.success {
                            println!(
                                "│   Exit Code: {} (success={})",
                                if success { "0" } else { "non-zero" },
                                success
                            );
                        }
                    }
                    println!("│   Result: {}", actual_result);
                    println!("│   Output: {}", actual_output);

                    println!("│");
                    println!("│ ✗ FAILURE REASON:");
                    println!("│   {}", reason);

                    // Show detailed comparison
                    if let Some(log) = log_entry {
                        println!("│");
                        println!("│ Detailed Comparison:");

                        if let Some(expected_success) = expected.success {
                            if let Some(actual_success) = log.success {
                                let exit_match = expected_success == actual_success;
                                println!(
                                    "│   Exit Code Match: {} (expected: {}, actual: {})",
                                    if exit_match { "✓" } else { "✗" },
                                    if expected_success { "0" } else { "non-zero" },
                                    if actual_success { "0" } else { "non-zero" }
                                );
                            }
                        }

                        let result_match = expected.result == *actual_result;
                        println!(
                            "│   Result Match: {} (expected: '{}', actual: '{}')",
                            if result_match { "✓" } else { "✗" },
                            expected.result,
                            actual_result
                        );

                        let output_match = expected.output == *actual_output;
                        println!(
                            "│   Output Match: {} (expected: '{}', actual: '{}')",
                            if output_match { "✓" } else { "✗" },
                            expected.output,
                            actual_output
                        );
                    }
                }

                testcase_manager::StepVerificationResultEnum::NotExecuted {
                    description, ..
                } => {
                    println!("│ Description: {}", description);
                    println!("│ Status: ○ NOT EXECUTED");
                    println!("│");

                    if let Some(step) = step_details {
                        println!("│ Command: {}", step.command);
                        println!("│");
                        println!("│ Expected (from YAML):");
                        if let Some(success) = step.expected.success {
                            println!(
                                "│   Exit Code: {} (success={})",
                                if success { "0" } else { "non-zero" },
                                success
                            );
                        }
                        println!("│   Result: {}", step.expected.result);
                        println!("│   Output: {}", step.expected.output);
                    }

                    println!("│");
                    println!("│ ○ No execution log found for this step");
                }
            }

            println!("└───────────────────────────────────────────────────────────");
            println!();
        }

        println!(
            "Sequence Status: {}",
            if seq_result.all_steps_passed {
                "✓ PASS"
            } else {
                "✗ FAIL"
            }
        );
        println!();
    }

    println!("═══════════════════════════════════════════════════════════");
    println!("FINAL TEST RESULT");
    println!("═══════════════════════════════════════════════════════════");
    println!();
    println!(
        "Overall Status: {}",
        if result.overall_pass {
            "✓✓✓ PASS ✓✓✓"
        } else {
            "✗✗✗ FAIL ✗✗✗"
        }
    );
    println!();
    println!(
        "Summary: {} total steps, {} passed, {} failed, {} not executed",
        result.total_steps, result.passed_steps, result.failed_steps, result.not_executed_steps
    );
    println!("═══════════════════════════════════════════════════════════");
}

fn format_batch_report_text(report: &BatchVerificationReport) -> String {
    let mut output = String::new();

    output.push_str("═══════════════════════════════════════════════════════════\n");
    output.push_str("           BATCH VERIFICATION REPORT\n");
    output.push_str("═══════════════════════════════════════════════════════════\n");
    output.push_str(&format!("Generated: {}\n", report.generated_at));
    output.push('\n');

    output.push_str("SUMMARY:\n");
    output.push_str("───────────────────────────────────────────────────────────\n");
    output.push_str(&format!("Test Cases:  {} total\n", report.total_test_cases));
    output.push_str(&format!(
        "             {} passed ({}%)\n",
        report.passed_test_cases,
        if report.total_test_cases > 0 {
            (report.passed_test_cases * 100) / report.total_test_cases
        } else {
            0
        }
    ));
    output.push_str(&format!(
        "             {} failed\n",
        report.failed_test_cases
    ));
    output.push('\n');
    output.push_str(&format!("Steps:       {} total\n", report.total_steps));
    output.push_str(&format!(
        "             {} passed ({}%)\n",
        report.passed_steps,
        if report.total_steps > 0 {
            (report.passed_steps * 100) / report.total_steps
        } else {
            0
        }
    ));
    output.push_str(&format!("             {} failed\n", report.failed_steps));
    output.push_str(&format!(
        "             {} not executed\n",
        report.not_executed_steps
    ));
    output.push('\n');

    output.push_str("TEST CASE RESULTS:\n");
    output.push_str("═══════════════════════════════════════════════════════════\n");

    for tc_result in &report.test_cases {
        let status = if tc_result.overall_pass {
            "✓ PASS"
        } else {
            "✗ FAIL"
        };

        output.push_str(&format!("\n{} {}\n", status, tc_result.test_case_id));
        output.push_str(&format!("  Description: {}\n", tc_result.description));
        output.push_str(&format!(
            "  Steps: {}/{} passed",
            tc_result.passed_steps, tc_result.total_steps
        ));

        if tc_result.failed_steps > 0 {
            output.push_str(&format!(", {} failed", tc_result.failed_steps));
        }
        if tc_result.not_executed_steps > 0 {
            output.push_str(&format!(", {} not executed", tc_result.not_executed_steps));
        }
        output.push('\n');

        // Show failed/not executed sequences
        for seq_result in &tc_result.sequences {
            if !seq_result.all_steps_passed {
                output.push_str(&format!(
                    "  └─ Sequence #{}: {}\n",
                    seq_result.sequence_id, seq_result.name
                ));

                for step_result in &seq_result.step_results {
                    match step_result {
                        testcase_manager::StepVerificationResultEnum::Fail {
                            step,
                            description,
                            reason,
                            ..
                        } => {
                            output.push_str(&format!(
                                "     ✗ Step {}: {} - {}\n",
                                step, description, reason
                            ));
                        }
                        testcase_manager::StepVerificationResultEnum::NotExecuted {
                            step,
                            description,
                        } => {
                            output.push_str(&format!(
                                "     ○ Step {}: {} - NOT EXECUTED\n",
                                step, description
                            ));
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    output.push('\n');
    output.push_str("═══════════════════════════════════════════════════════════\n");
    output.push_str(&format!("Overall: {}\n", report.summary()));
    output.push_str("═══════════════════════════════════════════════════════════\n");

    output
}

fn format_batch_report_verbose(
    report: &BatchVerificationReport,
    verifier: &TestVerifier,
    log_paths: &[PathBuf],
) -> String {
    use std::collections::HashMap;

    let mut output = String::new();

    output.push_str("═══════════════════════════════════════════════════════════\n");
    output.push_str("    VERBOSE BATCH VERIFICATION REPORT\n");
    output.push_str("═══════════════════════════════════════════════════════════\n");
    output.push_str(&format!("Generated: {}\n", report.generated_at));
    output.push('\n');

    // Parse all logs for detailed verbose output
    let mut all_logs: HashMap<String, Vec<VerificationTestExecutionLog>> = HashMap::new();
    for log_path in log_paths {
        if let Ok(logs) = verifier.parse_log_file(log_path) {
            for log in logs {
                all_logs
                    .entry(log.test_case_id.clone())
                    .or_default()
                    .push(log);
            }
        }
    }

    // Process each test case
    for tc_result in &report.test_cases {
        output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        output.push_str(&format!("TEST CASE: {}\n", tc_result.test_case_id));
        output.push_str(&format!("Description: {}\n", tc_result.description));
        output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        output.push('\n');

        // Load test case details
        let test_case = verifier
            .storage()
            .load_test_case_by_id(&tc_result.test_case_id)
            .ok();
        let logs = all_logs.get(&tc_result.test_case_id);

        if let (Some(tc), Some(logs)) = (test_case, logs) {
            // Create log map
            let mut log_map: HashMap<(i64, i64), &VerificationTestExecutionLog> = HashMap::new();
            for log in logs {
                log_map.insert((log.sequence_id, log.step_number), log);
            }

            // Process each sequence
            for seq_result in &tc_result.sequences {
                output.push_str(&format!(
                    "  SEQUENCE #{}: {}\n",
                    seq_result.sequence_id, seq_result.name
                ));
                output.push_str("  ───────────────────────────────────────────────────────\n");
                output.push('\n');

                let sequence = tc
                    .test_sequences
                    .iter()
                    .find(|s| s.id == seq_result.sequence_id);

                for step_result in &seq_result.step_results {
                    let step_num = step_result.step_number();
                    let log_entry = log_map.get(&(seq_result.sequence_id, step_num));
                    let step_details =
                        sequence.and_then(|s| s.steps.iter().find(|st| st.step == step_num));

                    output.push_str(&format!(
                        "  ┌─ Step {} ─────────────────────────\n",
                        step_num
                    ));

                    match step_result {
                        testcase_manager::StepVerificationResultEnum::Pass {
                            description, ..
                        } => {
                            output.push_str(&format!("  │ Description: {}\n", description));
                            output.push_str("  │ Status: ✓ PASS\n");
                            output.push_str("  │\n");

                            if let Some(step) = step_details {
                                output.push_str(&format!("  │ Command: {}\n", step.command));
                                output.push_str("  │\n");
                                output.push_str("  │ Expected (from YAML):\n");
                                if let Some(success) = step.expected.success {
                                    output.push_str(&format!(
                                        "  │   Exit Code: {} (success={})\n",
                                        if success { "0" } else { "non-zero" },
                                        success
                                    ));
                                }
                                output
                                    .push_str(&format!("  │   Result: {}\n", step.expected.result));
                                output
                                    .push_str(&format!("  │   Output: {}\n", step.expected.output));
                            }

                            if let Some(log) = log_entry {
                                output.push_str("  │\n");
                                output.push_str("  │ Actual (from JSON log):\n");
                                if let Some(success) = log.success {
                                    output.push_str(&format!(
                                        "  │   Exit Code: {} (success={})\n",
                                        if success { "0" } else { "non-zero" },
                                        success
                                    ));
                                }
                                output.push_str(&format!("  │   Result: {}\n", log.actual_result));
                                output.push_str(&format!("  │   Output: {}\n", log.actual_output));
                            }

                            output.push_str("  │\n");
                            output.push_str("  │ ✓ All comparisons PASSED\n");
                        }

                        testcase_manager::StepVerificationResultEnum::Fail {
                            description,
                            expected,
                            actual_result,
                            actual_output,
                            reason,
                            ..
                        } => {
                            output.push_str(&format!("  │ Description: {}\n", description));
                            output.push_str("  │ Status: ✗ FAIL\n");
                            output.push_str("  │\n");

                            if let Some(step) = step_details {
                                output.push_str(&format!("  │ Command: {}\n", step.command));
                                output.push_str("  │\n");
                            }

                            output.push_str("  │ Expected (from YAML):\n");
                            if let Some(success) = expected.success {
                                output.push_str(&format!(
                                    "  │   Exit Code: {} (success={})\n",
                                    if success { "0" } else { "non-zero" },
                                    success
                                ));
                            }
                            output.push_str(&format!("  │   Result: {}\n", expected.result));
                            output.push_str(&format!("  │   Output: {}\n", expected.output));

                            output.push_str("  │\n");
                            output.push_str("  │ Actual (from JSON log):\n");
                            if let Some(log) = log_entry {
                                if let Some(success) = log.success {
                                    output.push_str(&format!(
                                        "  │   Exit Code: {} (success={})\n",
                                        if success { "0" } else { "non-zero" },
                                        success
                                    ));
                                }
                            }
                            output.push_str(&format!("  │   Result: {}\n", actual_result));
                            output.push_str(&format!("  │   Output: {}\n", actual_output));

                            output.push_str("  │\n");
                            output.push_str("  │ ✗ FAILURE REASON:\n");
                            output.push_str(&format!("  │   {}\n", reason));

                            if let Some(log) = log_entry {
                                output.push_str("  │\n");
                                output.push_str("  │ Detailed Comparison:\n");

                                if let Some(expected_success) = expected.success {
                                    if let Some(actual_success) = log.success {
                                        let exit_match = expected_success == actual_success;
                                        output.push_str(&format!(
                                            "  │   Exit Code Match: {} (expected: {}, actual: {})\n",
                                            if exit_match { "✓" } else { "✗" },
                                            if expected_success { "0" } else { "non-zero" },
                                            if actual_success { "0" } else { "non-zero" }
                                        ));
                                    }
                                }

                                let result_match = expected.result == *actual_result;
                                output.push_str(&format!(
                                    "  │   Result Match: {} (expected: '{}', actual: '{}')\n",
                                    if result_match { "✓" } else { "✗" },
                                    expected.result,
                                    actual_result
                                ));

                                let output_match = expected.output == *actual_output;
                                output.push_str(&format!(
                                    "  │   Output Match: {} (expected: '{}', actual: '{}')\n",
                                    if output_match { "✓" } else { "✗" },
                                    expected.output,
                                    actual_output
                                ));
                            }
                        }

                        testcase_manager::StepVerificationResultEnum::NotExecuted {
                            description,
                            ..
                        } => {
                            output.push_str(&format!("  │ Description: {}\n", description));
                            output.push_str("  │ Status: ○ NOT EXECUTED\n");
                            output.push_str("  │\n");

                            if let Some(step) = step_details {
                                output.push_str(&format!("  │ Command: {}\n", step.command));
                                output.push_str("  │\n");
                                output.push_str("  │ Expected (from YAML):\n");
                                if let Some(success) = step.expected.success {
                                    output.push_str(&format!(
                                        "  │   Exit Code: {} (success={})\n",
                                        if success { "0" } else { "non-zero" },
                                        success
                                    ));
                                }
                                output
                                    .push_str(&format!("  │   Result: {}\n", step.expected.result));
                                output
                                    .push_str(&format!("  │   Output: {}\n", step.expected.output));
                            }

                            output.push_str("  │\n");
                            output.push_str("  │ ○ No execution log found for this step\n");
                        }
                    }

                    output.push_str("  └────────────────────────────────────────\n");
                    output.push('\n');
                }

                output.push_str(&format!(
                    "  Sequence Status: {}\n",
                    if seq_result.all_steps_passed {
                        "✓ PASS"
                    } else {
                        "✗ FAIL"
                    }
                ));
                output.push('\n');
            }
        }

        // Test case summary
        output.push_str(&format!(
            "Test Case Status: {}\n",
            if tc_result.overall_pass {
                "✓✓✓ PASS ✓✓✓"
            } else {
                "✗✗✗ FAIL ✗✗✗"
            }
        ));
        output.push_str(&format!(
            "Summary: {}/{} steps passed",
            tc_result.passed_steps, tc_result.total_steps
        ));
        if tc_result.failed_steps > 0 {
            output.push_str(&format!(", {} failed", tc_result.failed_steps));
        }
        if tc_result.not_executed_steps > 0 {
            output.push_str(&format!(", {} not executed", tc_result.not_executed_steps));
        }
        output.push_str("\n\n");
    }

    // Overall summary
    output.push_str("═══════════════════════════════════════════════════════════\n");
    output.push_str("FINAL BATCH RESULT\n");
    output.push_str("═══════════════════════════════════════════════════════════\n");
    output.push('\n');
    output.push_str(&format!(
        "Overall Status: {}\n",
        if report.failed_test_cases == 0 {
            "✓✓✓ PASS ✓✓✓"
        } else {
            "✗✗✗ FAIL ✗✗✗"
        }
    ));
    output.push('\n');
    output.push_str("SUMMARY:\n");
    output.push_str(&format!(
        "  Test Cases:  {} total\n",
        report.total_test_cases
    ));
    output.push_str(&format!(
        "               {} passed ({}%)\n",
        report.passed_test_cases,
        if report.total_test_cases > 0 {
            (report.passed_test_cases * 100) / report.total_test_cases
        } else {
            0
        }
    ));
    output.push_str(&format!(
        "               {} failed\n",
        report.failed_test_cases
    ));
    output.push('\n');
    output.push_str(&format!("  Steps:       {} total\n", report.total_steps));
    output.push_str(&format!(
        "               {} passed ({}%)\n",
        report.passed_steps,
        if report.total_steps > 0 {
            (report.passed_steps * 100) / report.total_steps
        } else {
            0
        }
    ));
    output.push_str(&format!("               {} failed\n", report.failed_steps));
    output.push_str(&format!(
        "               {} not executed\n",
        report.not_executed_steps
    ));
    output.push_str("═══════════════════════════════════════════════════════════\n");

    output
}

fn handle_single_verify(
    log_path: PathBuf,
    test_case_id: String,
    test_case_dir: PathBuf,
    format: String,
    verbose: bool,
) -> Result<()> {
    let storage =
        TestCaseStorage::new(&test_case_dir).context("Failed to initialize test case storage")?;
    let verifier = TestVerifier::from_storage(storage);

    // Parse log file
    let logs = verifier
        .parse_log_file(&log_path)
        .context("Failed to parse test execution log")?;

    // Load test case
    let test_case = verifier
        .storage()
        .load_test_case_by_id(&test_case_id)
        .context("Failed to load test case")?;
    // Verify
    let result = verifier.verify_test_case(&test_case, &logs);

    // Check pass status before moving result
    let overall_pass = result.overall_pass;

    // Output results
    match format.as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&result)
                .context("Failed to serialize result to JSON")?;
            println!("{}", json);
        }
        "junit" => {
            let mut report = BatchVerificationReport::new();
            report.add_test_case_result(result);
            let junit = JUnitTestSuite::from_batch_report(&report, "Single Test Verification");
            let xml = junit.to_xml().context("Failed to generate JUnit XML")?;
            println!("{}", xml);
        }
        _ => {
            if verbose {
                print_verification_result_verbose(&result, &test_case, &logs);
            } else {
                print_verification_result(&result);
            }
        }
    }

    if !overall_pass {
        std::process::exit(1);
    }

    Ok(())
}

fn handle_batch_verify(
    log_paths: Vec<PathBuf>,
    test_case_dir: PathBuf,
    format: String,
    output_path: Option<PathBuf>,
    verbose: bool,
) -> Result<()> {
    let storage =
        TestCaseStorage::new(&test_case_dir).context("Failed to initialize test case storage")?;
    let verifier = TestVerifier::from_storage(storage);

    log::info!("Processing {} log file(s)...", log_paths.len());

    // Perform batch verification
    let report = verifier
        .batch_verify(&log_paths)
        .context("Failed to perform batch verification")?;

    // Generate output
    let output = match format.as_str() {
        "json" => {
            serde_json::to_string_pretty(&report).context("Failed to serialize report to JSON")?
        }
        "junit" => {
            let junit = JUnitTestSuite::from_batch_report(&report, "Batch Test Verification");
            junit.to_xml().context("Failed to generate JUnit XML")?
        }
        _ => {
            if verbose {
                format_batch_report_verbose(&report, &verifier, &log_paths)
            } else {
                format_batch_report_text(&report)
            }
        }
    };

    // Write output
    if let Some(output_file) = output_path {
        fs::write(&output_file, &output).context(format!(
            "Failed to write output to {}",
            output_file.display()
        ))?;
        log::info!("Report written to {}", output_file.display());
    } else {
        println!("{}", output);
    }
    if report.failed_test_cases > 0 {
        std::process::exit(1);
    }
    Ok(())
}
