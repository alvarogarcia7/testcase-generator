use anyhow::{Context, Result};
use chrono::Utc;
use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use testcase_manager::fuzzy::TestCaseFuzzyFinder;
use testcase_manager::models::{TestRun, TestRunStatus};
use testcase_manager::prompts::Prompts;
use testcase_manager::storage::TestCaseStorage;
use testcase_manager::test_run_storage::TestRunStorage;

#[derive(Parser)]
#[command(name = "test-run-manager")]
#[command(about = "Manage test run execution records", version)]
struct Cli {
    #[arg(short, long, default_value = "testcases", global = true)]
    path: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,
    Add,
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();

    let test_runs_dir = cli.path.join("test-runs");
    if !test_runs_dir.exists() {
        fs::create_dir_all(&test_runs_dir).context(format!(
            "Failed to create directory: {}",
            test_runs_dir.display()
        ))?;
    }

    match cli.command {
        Commands::List => list_test_runs(&cli.path, &test_runs_dir)?,
        Commands::Add => add_test_run(&cli.path, &test_runs_dir)?,
    }

    Ok(())
}

fn list_test_runs(base_path: &Path, _test_runs_dir: &Path) -> Result<()> {
    let test_run_storage = TestRunStorage::new(base_path.join("test-runs"))
        .context("Failed to initialize test run storage")?;

    let all_runs = test_run_storage
        .load_all_test_runs()
        .context("Failed to load test runs")?;

    if all_runs.is_empty() {
        println!("No test runs found.");
        return Ok(());
    }

    let mut runs_by_test_case: HashMap<String, Vec<TestRun>> = HashMap::new();
    for run in all_runs {
        runs_by_test_case
            .entry(run.test_case_id.clone())
            .or_default()
            .push(run);
    }

    let mut test_case_ids: Vec<String> = runs_by_test_case.keys().cloned().collect();
    test_case_ids.sort();

    println!(
        "\n{:<30} {:<10} {:<25} {:<15}",
        "Test Case ID", "Run Count", "Latest Run", "Status Summary"
    );
    println!("{}", "=".repeat(85));

    for test_case_id in test_case_ids {
        if let Some(runs) = runs_by_test_case.get_mut(&test_case_id) {
            runs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

            let run_count = runs.len();
            let latest_timestamp = runs.first().map(|r| r.timestamp).unwrap();

            let mut pass_count = 0;
            let mut fail_count = 0;
            let mut skip_count = 0;

            for run in runs.iter() {
                match run.status {
                    TestRunStatus::Pass => pass_count += 1,
                    TestRunStatus::Fail => fail_count += 1,
                    TestRunStatus::Skip => skip_count += 1,
                }
            }

            let status_summary = format!("P:{} F:{} S:{}", pass_count, fail_count, skip_count);

            println!(
                "{:<30} {:<10} {:<25} {:<15}",
                test_case_id,
                run_count,
                latest_timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
                status_summary
            );
        }
    }

    Ok(())
}

fn add_test_run(base_path: &Path, _test_runs_dir: &Path) -> Result<()> {
    let test_case_storage =
        TestCaseStorage::new(base_path).context("Failed to initialize test case storage")?;

    let test_run_storage = TestRunStorage::new(base_path.join("test-runs"))
        .context("Failed to initialize test run storage")?;

    println!("\n=== Add Test Run ===\n");

    let test_case_id: String = loop {
        let id_input: String = Prompts::input("Test Case ID (or press Enter to search)")
            .context("Failed to read test case ID")?;

        if id_input.trim().is_empty() {
            let all_test_cases = test_case_storage
                .load_all_test_cases()
                .context("Failed to load test cases")?;

            if all_test_cases.is_empty() {
                anyhow::bail!("No test cases found in storage");
            }

            let test_case_ids: Vec<String> =
                all_test_cases.iter().map(|tc| tc.id.clone()).collect();

            match TestCaseFuzzyFinder::search_strings(&test_case_ids, "Select test case: ")? {
                Some(id) => break id,
                None => {
                    println!("No test case selected. Try entering ID manually.");
                    continue;
                }
            }
        } else {
            break id_input;
        }
    };

    if !test_case_storage.test_case_exists(&test_case_id) {
        anyhow::bail!("Test case '{}' does not exist", test_case_id);
    }

    let status_options = vec!["Pass".to_string(), "Fail".to_string(), "Skip".to_string()];
    let status_str = Prompts::select("Execution status", status_options)?;

    let status = match status_str.as_str() {
        "Pass" => TestRunStatus::Pass,
        "Fail" => TestRunStatus::Fail,
        "Skip" => TestRunStatus::Skip,
        _ => TestRunStatus::Fail,
    };

    let duration: f64 = Prompts::input_with_default("Duration (milliseconds)", "0")
        .context("Failed to read duration")?
        .parse()
        .context("Failed to parse duration as number")?;

    let execution_log: String = Prompts::input_with_default("Execution log/notes", "")?;

    let error_message: Option<String> = if status == TestRunStatus::Fail {
        Some(Prompts::input_with_default("Error message", "")?)
    } else {
        None
    };

    let test_run = TestRun {
        name: Some("HARDCODED NAME".to_string()),
        test_case_id: test_case_id.clone(),
        timestamp: Utc::now(),
        status,
        duration,
        execution_log,
        error_message,
    };

    let file_path = test_run_storage
        .save_test_run(&test_run)
        .context("Failed to save test run")?;

    println!("\n✓ Test run saved to: {}", file_path.display());

    Ok(())
}
