use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use dialoguer::{Input, Select};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use testcase_manager::storage::TestCaseStorage;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestRun {
    test_case_id: String,
    timestamp: DateTime<Utc>,
    status: ExecutionStatus,
    output: String,
    duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ExecutionStatus {
    Passed,
    Failed,
    Skipped,
    Error,
}

impl std::fmt::Display for ExecutionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionStatus::Passed => write!(f, "PASSED"),
            ExecutionStatus::Failed => write!(f, "FAILED"),
            ExecutionStatus::Skipped => write!(f, "SKIPPED"),
            ExecutionStatus::Error => write!(f, "ERROR"),
        }
    }
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

fn list_test_runs(_base_path: &Path, test_runs_dir: &Path) -> Result<()> {
    let mut runs_by_test_case: HashMap<String, Vec<TestRun>> = HashMap::new();

    if test_runs_dir.exists() {
        for entry in fs::read_dir(test_runs_dir).context(format!(
            "Failed to read directory: {}",
            test_runs_dir.display()
        ))? {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();

            if path.is_file()
                && path
                    .extension()
                    .is_some_and(|ext| ext == "yaml" || ext == "yml")
            {
                match load_test_run(&path) {
                    Ok(test_run) => {
                        runs_by_test_case
                            .entry(test_run.test_case_id.clone())
                            .or_default()
                            .push(test_run);
                    }
                    Err(e) => {
                        log::warn!("Failed to load test run from {}: {}", path.display(), e);
                    }
                }
            }
        }
    }

    if runs_by_test_case.is_empty() {
        println!("No test runs found.");
        return Ok(());
    }

    let mut test_case_ids: Vec<String> = runs_by_test_case.keys().cloned().collect();
    test_case_ids.sort();

    for test_case_id in test_case_ids {
        if let Some(runs) = runs_by_test_case.get_mut(&test_case_id) {
            runs.sort_by_key(|r| r.timestamp);

            println!("\nTest Case: {}", test_case_id);
            println!("{}", "=".repeat(50));

            for run in runs {
                println!(
                    "  {} | {} | {}ms | {}",
                    run.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
                    run.status,
                    run.duration_ms,
                    if run.output.len() > 50 {
                        format!("{}...", &run.output[..47])
                    } else {
                        run.output.clone()
                    }
                );
            }
        }
    }

    Ok(())
}

fn add_test_run(base_path: &Path, test_runs_dir: &Path) -> Result<()> {
    let storage =
        TestCaseStorage::new(base_path).context("Failed to initialize test case storage")?;

    let test_case_id: String = Input::new()
        .with_prompt("Test Case ID")
        .interact_text()
        .context("Failed to read test case ID")?;

    if !storage.test_case_exists(&test_case_id) {
        anyhow::bail!("Test case '{}' does not exist", test_case_id);
    }

    let status_options = vec!["Passed", "Failed", "Skipped", "Error"];
    let status_selection = Select::new()
        .with_prompt("Execution status")
        .items(&status_options)
        .default(0)
        .interact()
        .context("Failed to read execution status")?;

    let status = match status_selection {
        0 => ExecutionStatus::Passed,
        1 => ExecutionStatus::Failed,
        2 => ExecutionStatus::Skipped,
        3 => ExecutionStatus::Error,
        _ => ExecutionStatus::Error,
    };

    let output: String = Input::new()
        .with_prompt("Output/Notes")
        .allow_empty(true)
        .interact_text()
        .context("Failed to read output")?;

    let duration_str: String = Input::new()
        .with_prompt("Duration (milliseconds)")
        .default("0".to_string())
        .interact_text()
        .context("Failed to read duration")?;

    let duration_ms: u64 = duration_str
        .parse()
        .context("Failed to parse duration as number")?;

    let test_run = TestRun {
        test_case_id: test_case_id.clone(),
        timestamp: Utc::now(),
        status,
        output,
        duration_ms,
    };

    let timestamp_str = test_run.timestamp.format("%Y%m%d_%H%M%S");
    let filename = format!("{}_{}.yaml", test_case_id, timestamp_str);
    let file_path = test_runs_dir.join(filename);

    let yaml_content =
        serde_yaml::to_string(&test_run).context("Failed to serialize test run to YAML")?;

    fs::write(&file_path, yaml_content)
        .context(format!("Failed to write file: {}", file_path.display()))?;

    println!("\n✓ Test run saved to: {}", file_path.display());

    Ok(())
}

fn load_test_run(path: &Path) -> Result<TestRun> {
    let content =
        fs::read_to_string(path).context(format!("Failed to read file: {}", path.display()))?;

    let test_run: TestRun = serde_yaml::from_str(&content)
        .context(format!("Failed to parse YAML from: {}", path.display()))?;

    Ok(test_run)
}
