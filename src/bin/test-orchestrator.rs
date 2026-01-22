use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use std::path::PathBuf;
use testcase_manager::fuzzy::TestCaseFuzzyFinder;
use testcase_manager::orchestrator::{RetryPolicy, TestOrchestrator, WorkerConfig};
use testcase_manager::storage::TestCaseStorage;
use testcase_manager::test_run_storage::TestRunStorage;

#[derive(Parser)]
#[command(name = "test-orchestrator")]
#[command(
    about = "Test execution orchestrator with parallel execution, retry policies, and real-time progress reporting",
    version,
    long_about = "
Test Orchestrator - Coordinate test case execution with advanced features

Features:
  • Parallel test execution with configurable worker pool
  • Automatic retry with configurable retry policies
  • Real-time progress reporting with live statistics
  • Execution result tracking and report generation
  • Integration with test case storage and verification
"
)]
struct Cli {
    /// Base path for test case storage
    #[arg(short, long, default_value = "testcases", global = true)]
    path: PathBuf,

    /// Output directory for execution logs and reports
    #[arg(short, long, default_value = "test-output", global = true)]
    output: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute specific test cases by ID
    Run {
        /// Test case IDs to execute (required unless --fuzzy is used)
        #[arg(required = false)]
        test_case_ids: Vec<String>,

        /// Number of worker threads (default: 4)
        #[arg(short = 'w', long, default_value = "4")]
        workers: usize,

        /// Enable retry on failure
        #[arg(short, long)]
        retry: bool,

        /// Maximum retry attempts (default: 3)
        #[arg(long, default_value = "3")]
        max_retries: usize,

        /// Use exponential backoff for retries
        #[arg(long)]
        exponential_backoff: bool,

        /// Base delay in milliseconds for exponential backoff (default: 100)
        #[arg(long, default_value = "100")]
        backoff_delay: u64,

        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,

        /// Save execution results to test run storage
        #[arg(short, long)]
        save: bool,

        /// Generate execution report
        #[arg(short = 'g', long)]
        report: bool,

        /// Use fuzzy search to select test cases interactively
        #[arg(short = 'f', long)]
        fuzzy: bool,
    },

    /// Execute all available test cases
    RunAll {
        /// Number of worker threads (default: 4)
        #[arg(short = 'w', long, default_value = "4")]
        workers: usize,

        /// Enable retry on failure
        #[arg(short, long)]
        retry: bool,

        /// Maximum retry attempts (default: 3)
        #[arg(long, default_value = "3")]
        max_retries: usize,

        /// Use exponential backoff for retries
        #[arg(long)]
        exponential_backoff: bool,

        /// Base delay in milliseconds for exponential backoff (default: 100)
        #[arg(long, default_value = "100")]
        backoff_delay: u64,

        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,

        /// Save execution results to test run storage
        #[arg(short, long)]
        save: bool,

        /// Generate execution report
        #[arg(short = 'g', long)]
        report: bool,
    },

    /// Verify test execution results from log files
    Verify {
        /// Log files to verify (when used without --test-case and --execution-log)
        log_files: Vec<PathBuf>,

        /// Specific test case YAML file to verify
        #[arg(long = "test-case")]
        test_case_file: Option<PathBuf>,

        /// Specific execution log JSON file to verify against
        #[arg(long = "execution-log")]
        execution_log_file: Option<PathBuf>,

        /// Enable verbose output showing detailed steps and verification results
        #[arg(short, long)]
        verbose: bool,
    },

    /// Show orchestrator configuration and status
    Info,
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();

    // Validate run subcommand parameters before proceeding
    if let Commands::Run {
        ref test_case_ids,
        fuzzy,
        ..
    } = cli.command
    {
        if test_case_ids.is_empty() && !fuzzy {
            // Print usage help and exit with non-zero status
            let mut cmd = Cli::command();
            cmd.error(
                clap::error::ErrorKind::MissingRequiredArgument,
                "No test case IDs provided. Either provide test case IDs as arguments or use --fuzzy for interactive selection.\n\nUsage: test-orchestrator run [TEST_CASE_IDS]... [OPTIONS]\n       test-orchestrator run --fuzzy [OPTIONS]",
            )
            .exit();
        }
    }

    let test_case_storage =
        TestCaseStorage::new(&cli.path).context("Failed to initialize test case storage")?;
    let test_run_storage = TestRunStorage::new(cli.path.join("test-runs"))
        .context("Failed to initialize test run storage")?;

    let orchestrator =
        TestOrchestrator::new(test_case_storage, test_run_storage, cli.output.clone())
            .context("Failed to initialize test orchestrator")?;

    match cli.command {
        Commands::Run {
            mut test_case_ids,
            workers,
            retry,
            max_retries,
            exponential_backoff,
            backoff_delay,
            verbose,
            save,
            report,
            fuzzy: _,
        } => {
            if test_case_ids.is_empty() {
                // This branch is only reached when fuzzy is true (due to validation above)
                let all_test_cases = orchestrator
                    .select_all_test_cases()
                    .context("Failed to load test cases")?;

                if all_test_cases.is_empty() {
                    anyhow::bail!("No test cases found in storage");
                }

                let test_case_options: Vec<String> = all_test_cases
                    .iter()
                    .map(|tc| format!("{} - {}", tc.id, tc.description))
                    .collect();

                match TestCaseFuzzyFinder::search_strings(
                    &test_case_options,
                    "Select test cases to execute (Tab to select multiple, Enter to confirm): ",
                )? {
                    Some(selected) => {
                        test_case_ids = selected
                            .split(" - ")
                            .next()
                            .unwrap()
                            .to_string()
                            .split_whitespace()
                            .map(String::from)
                            .collect();
                    }
                    None => {
                        println!("No test cases selected. Exiting.");
                        return Ok(());
                    }
                }
            }

            let retry_policy = if retry {
                if exponential_backoff {
                    RetryPolicy::exponential_backoff(max_retries, backoff_delay)
                } else {
                    RetryPolicy::fixed_retries(max_retries)
                }
            } else {
                RetryPolicy::no_retry()
            };

            let config = WorkerConfig::new(workers).with_retry_policy(retry_policy);

            let test_cases = orchestrator
                .select_test_cases(test_case_ids)
                .context("Failed to load test cases")?;

            let results = orchestrator
                .execute_tests(test_cases, config, verbose)
                .context("Failed to execute tests")?;

            if save {
                orchestrator
                    .save_results(&results)
                    .context("Failed to save test results")?;
                println!("\n✓ Test results saved to storage");
            }

            if report {
                let report_path = cli.output.clone().join("execution_report.md");
                orchestrator
                    .generate_execution_report(&results, &report_path)
                    .context("Failed to generate execution report")?;
                println!("✓ Execution report saved to: {}", report_path.display());
            }

            let failed_count = results.iter().filter(|r| !r.success).count();
            if failed_count > 0 {
                std::process::exit(1);
            }
        }

        Commands::RunAll {
            workers,
            retry,
            max_retries,
            exponential_backoff,
            backoff_delay,
            verbose,
            save,
            report,
        } => {
            let retry_policy = if retry {
                if exponential_backoff {
                    RetryPolicy::exponential_backoff(max_retries, backoff_delay)
                } else {
                    RetryPolicy::fixed_retries(max_retries)
                }
            } else {
                RetryPolicy::no_retry()
            };

            let config = WorkerConfig::new(workers).with_retry_policy(retry_policy);

            let test_cases = orchestrator
                .select_all_test_cases()
                .context("Failed to load all test cases")?;

            if test_cases.is_empty() {
                println!("No test cases found in storage.");
                return Ok(());
            }

            let results = orchestrator
                .execute_tests(test_cases, config, verbose)
                .context("Failed to execute tests")?;

            if save {
                orchestrator
                    .save_results(&results)
                    .context("Failed to save test results")?;
                println!("\n✓ Test results saved to storage");
            }

            if report {
                let report_path = cli.output.join("execution_report.md");
                orchestrator
                    .generate_execution_report(&results, &report_path)
                    .context("Failed to generate execution report")?;
                println!("✓ Execution report saved to: {}", report_path.display());
            }

            let failed_count = results.iter().filter(|r| !r.success).count();
            if failed_count > 0 {
                std::process::exit(1);
            }
        }

        Commands::Verify {
            log_files,
            test_case_file,
            execution_log_file,
            verbose,
        } => {
            // Check if specific test case and execution log are provided
            if test_case_file.is_some() || execution_log_file.is_some() {
                // Both must be provided together
                let tc_file = test_case_file.ok_or_else(|| {
                    anyhow::anyhow!("--test-case must be provided when using --execution-log")
                })?;
                let log_file = execution_log_file.ok_or_else(|| {
                    anyhow::anyhow!("--execution-log must be provided when using --test-case")
                })?;

                println!("\n=== Verifying Specific Test Case ===\n");

                if verbose {
                    println!("Test case file: {}", tc_file.display());
                    println!("Execution log file: {}", log_file.display());
                    println!();
                }

                let verification_results = orchestrator
                    .verify_test_case_with_log(&tc_file, &log_file)
                    .context("Failed to verify test case")?;

                let status = if verification_results.overall_pass {
                    "✓ PASS"
                } else {
                    "✗ FAIL"
                };
                println!(
                    "{} {} - {} ({}/{} steps passed)",
                    status,
                    verification_results.test_case_id,
                    verification_results.description,
                    verification_results.passed_steps,
                    verification_results.total_steps
                );

                if verbose || !verification_results.overall_pass {
                    for sequence in &verification_results.sequences {
                        println!("\n  Sequence {}: {}", sequence.sequence_id, sequence.name);
                        println!("  {}", "-".repeat(60));

                        for step_result in &sequence.step_results {
                            use testcase_manager::verification::StepVerificationResultEnum;

                            match step_result {
                                StepVerificationResultEnum::Pass { step, description } => {
                                    if verbose {
                                        println!("  ✓ Step {}: {}", step, description);
                                    }
                                }
                                StepVerificationResultEnum::Fail {
                                    step,
                                    description,
                                    expected,
                                    actual_result,
                                    actual_output,
                                    reason,
                                } => {
                                    println!("  ✗ Step {}: {}", step, description);
                                    if verbose {
                                        println!("    Reason: {}", reason);
                                        println!("    Expected:");
                                        if let Some(success) = expected.success {
                                            println!("      Success: {}", success);
                                        }
                                        println!("      Result: {}", expected.result);
                                        println!("      Output: {}", expected.output);
                                        println!("    Actual:");
                                        println!("      Result: {}", actual_result);
                                        println!("      Output: {}", actual_output);
                                    } else {
                                        println!("    {}", reason);
                                    }
                                }
                                StepVerificationResultEnum::NotExecuted { step, description } => {
                                    println!("  ⚠ Step {}: {} (NOT EXECUTED)", step, description);
                                }
                            }
                        }
                    }
                }

                if !verification_results.overall_pass {
                    std::process::exit(1);
                }
            } else {
                // Original batch verification logic
                if log_files.is_empty() {
                    anyhow::bail!("No log files provided for verification");
                }

                println!("\n=== Verifying Test Results ===\n");

                if verbose {
                    println!("Log files to verify:");
                    for log_file in &log_files {
                        println!("  - {}", log_file.display());
                    }
                    println!();
                }

                let verification_results = orchestrator
                    .verify_results(log_files)
                    .context("Failed to verify test results")?;

                if verification_results.is_empty() {
                    println!("No test cases found in the provided log files.");
                    return Ok(());
                }

                println!("Verification Results:\n");

                let mut total_passed = 0;
                let mut total_failed = 0;

                for result in &verification_results {
                    let status = if result.overall_pass {
                        "✓ PASS"
                    } else {
                        "✗ FAIL"
                    };
                    println!(
                        "{} {} - {} ({}/{} steps passed)",
                        status,
                        result.test_case_id,
                        result.description,
                        result.passed_steps,
                        result.total_steps
                    );

                    if result.overall_pass {
                        total_passed += 1;
                    } else {
                        total_failed += 1;
                    }

                    if verbose || !result.overall_pass {
                        for sequence in &result.sequences {
                            println!("\n  Sequence {}: {}", sequence.sequence_id, sequence.name);
                            println!("  {}", "-".repeat(60));

                            for step_result in &sequence.step_results {
                                use testcase_manager::verification::StepVerificationResultEnum;

                                match step_result {
                                    StepVerificationResultEnum::Pass { step, description } => {
                                        if verbose {
                                            println!("  ✓ Step {}: {}", step, description);
                                        }
                                    }
                                    StepVerificationResultEnum::Fail {
                                        step,
                                        description,
                                        expected,
                                        actual_result,
                                        actual_output,
                                        reason,
                                    } => {
                                        println!("  ✗ Step {}: {}", step, description);
                                        if verbose {
                                            println!("    Reason: {}", reason);
                                            println!("    Expected:");
                                            if let Some(success) = expected.success {
                                                println!("      Success: {}", success);
                                            }
                                            println!("      Result: {}", expected.result);
                                            println!("      Output: {}", expected.output);
                                            println!("    Actual:");
                                            println!("      Result: {}", actual_result);
                                            println!("      Output: {}", actual_output);
                                        } else {
                                            println!("    {}", reason);
                                        }
                                    }
                                    StepVerificationResultEnum::NotExecuted {
                                        step,
                                        description,
                                    } => {
                                        println!(
                                            "  ⚠ Step {}: {} (NOT EXECUTED)",
                                            step, description
                                        );
                                    }
                                }
                            }
                        }
                    }
                }

                println!("\n=== Verification Summary ===");
                println!("Total test cases: {}", verification_results.len());
                println!("Passed: {}", total_passed);
                println!("Failed: {}", total_failed);

                if total_failed > 0 {
                    std::process::exit(1);
                }
            }
        }

        Commands::Info => {
            println!("\n=== Test Orchestrator Configuration ===\n");
            println!("Test case storage path: {}", cli.path.display());
            println!("Output directory: {}", cli.output.display());

            let test_cases = orchestrator
                .select_all_test_cases()
                .context("Failed to load test cases")?;

            println!("\nAvailable test cases: {}", test_cases.len());

            if !test_cases.is_empty() {
                println!("\nTest Cases:");
                for (idx, tc) in test_cases.iter().enumerate().take(10) {
                    println!("  {}. {} - {}", idx + 1, tc.id, tc.description);
                }
                if test_cases.len() > 10 {
                    println!("  ... and {} more", test_cases.len() - 10);
                }
            }

            println!("\nDefault Configuration:");
            println!("  Workers: 4");
            println!("  Retry policy: No retry");
            println!("  Verbose mode: Disabled");

            println!("\nUsage Examples:");
            println!("  # Run specific test cases");
            println!("  test-orchestrator run TC001 TC002");
            println!();
            println!("  # Run all test cases with 8 workers");
            println!("  test-orchestrator run-all -w 8");
            println!();
            println!("  # Run with retry (3 attempts)");
            println!("  test-orchestrator run TC001 --retry --max-retries 3");
            println!();
            println!("  # Run with exponential backoff");
            println!(
                "  test-orchestrator run TC001 --retry --exponential-backoff --backoff-delay 100"
            );
            println!();
            println!("  # Run interactively with fuzzy search");
            println!("  test-orchestrator run --fuzzy");
            println!();
            println!("  # Run and save results with report generation");
            println!("  test-orchestrator run-all --save --report");
        }
    }

    Ok(())
}
