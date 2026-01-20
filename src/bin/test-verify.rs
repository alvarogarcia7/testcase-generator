use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use testcase_manager::{BatchVerificationReport, JUnitTestSuite, TestCaseStorage, TestVerifier};

#[derive(Parser)]
#[command(name = "test-verify")]
#[command(
    about = "Test verification tool for comparing test execution logs against test case definitions"
)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Single {
            log,
            test_case_id,
            test_case_dir,
            format,
        } => handle_single_verify(log, test_case_id, test_case_dir, format),

        Commands::Batch {
            logs,
            test_case_dir,
            format,
            output,
        } => handle_batch_verify(logs, test_case_dir, format, output),

        Commands::ParseLog { log, format } => handle_parse_log(log, format),
    }
}

fn handle_single_verify(
    log_path: PathBuf,
    test_case_id: String,
    test_case_dir: PathBuf,
    format: String,
) -> Result<()> {
    let storage =
        TestCaseStorage::new(&test_case_dir).context("Failed to initialize test case storage")?;
    let verifier = TestVerifier::new(storage);

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
            print_verification_result(&result);
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
) -> Result<()> {
    let storage =
        TestCaseStorage::new(&test_case_dir).context("Failed to initialize test case storage")?;
    let verifier = TestVerifier::new(storage);

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
        _ => format_batch_report_text(&report),
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

fn handle_parse_log(log_path: PathBuf, format: String) -> Result<()> {
    let storage =
        TestCaseStorage::new("testcases").context("Failed to initialize test case storage")?;
    let verifier = TestVerifier::new(storage);

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
                testcase_manager::StepVerificationResult::Pass { step, description } => {
                    println!("  ✓ Step {}: {} - PASS", step, description);
                }
                testcase_manager::StepVerificationResult::Fail {
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
                testcase_manager::StepVerificationResult::NotExecuted { step, description } => {
                    println!("  ○ Step {}: {} - NOT EXECUTED", step, description);
                }
            }
        }
        println!();
    }
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
                        testcase_manager::StepVerificationResult::Fail {
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
                        testcase_manager::StepVerificationResult::NotExecuted {
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
