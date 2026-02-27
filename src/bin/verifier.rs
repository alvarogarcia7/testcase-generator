use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use testcase_manager::{BatchVerificationReport, TestCaseStorage, TestVerifier};

#[derive(Parser)]
#[command(name = "verifier")]
#[command(version)]
#[command(about = "Verify test execution logs against test case definitions")]
struct Cli {
    /// Single-file mode: path to log file
    #[arg(short, long, value_name = "PATH")]
    log: Option<PathBuf>,

    /// Single-file mode: test case ID to verify against
    #[arg(short = 'c', long = "test-case", value_name = "ID")]
    test_case_id: Option<String>,

    /// Folder discovery mode: path to folder containing log files
    #[arg(short, long, value_name = "PATH")]
    folder: Option<PathBuf>,

    /// Output format (yaml or json)
    #[arg(short = 'f', long, default_value = "yaml", value_name = "FORMAT")]
    format: String,

    /// Output file path (optional, defaults to stdout)
    #[arg(short, long, value_name = "PATH")]
    output: Option<PathBuf>,

    /// Path to test case storage directory
    #[arg(short = 'd', long, default_value = "testcases", value_name = "DIR")]
    test_case_dir: PathBuf,
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let cli = Cli::parse();

    // Validate CLI arguments
    let (mode, log_path, test_case_id, folder_path) = validate_args(&cli)?;

    // Initialize storage and verifier
    let storage = TestCaseStorage::new(&cli.test_case_dir)
        .context("Failed to initialize test case storage")?;
    let verifier = TestVerifier::from_storage(storage);

    // Execute appropriate mode
    let report = match mode {
        Mode::SingleFile => {
            let log_path = log_path.unwrap();
            let test_case_id = test_case_id.unwrap();
            handle_single_file_mode(&verifier, &log_path, &test_case_id)?
        }
        Mode::FolderDiscovery => {
            let folder_path = folder_path.unwrap();
            handle_folder_mode(&verifier, &folder_path)?
        }
    };

    // Generate output in requested format
    let output_content = generate_output(&verifier, &report, &cli.format)?;

    // Write to file or stdout
    write_output(&output_content, cli.output.as_ref())?;

    // Exit with non-zero code if any tests failed
    if report.failed_test_cases > 0 {
        std::process::exit(1);
    }

    Ok(())
}

#[derive(Debug)]
enum Mode {
    SingleFile,
    FolderDiscovery,
}

type ValidationResult = (Mode, Option<PathBuf>, Option<String>, Option<PathBuf>);

fn validate_args(cli: &Cli) -> Result<ValidationResult> {
    // Check for single-file mode
    let single_file_mode = cli.log.is_some() && cli.test_case_id.is_some();
    // Check for folder discovery mode
    let folder_mode = cli.folder.is_some();

    // Validate mutually exclusive modes
    if !single_file_mode && !folder_mode {
        anyhow::bail!("Must specify either single-file mode (--log and --test-case) or folder discovery mode (--folder)");
    }

    if single_file_mode && folder_mode {
        anyhow::bail!("Cannot use both single-file mode (--log/--test-case) and folder mode (--folder) simultaneously");
    }

    // Validate format
    let format = cli.format.to_lowercase();
    if format != "yaml" && format != "json" {
        anyhow::bail!("Format must be 'yaml' or 'json', got: {}", format);
    }

    if single_file_mode {
        let log_path = cli.log.clone().unwrap();
        let test_case_id = cli.test_case_id.clone().unwrap();

        if !log_path.exists() {
            anyhow::bail!("Log file does not exist: {}", log_path.display());
        }

        Ok((Mode::SingleFile, Some(log_path), Some(test_case_id), None))
    } else {
        let folder_path = cli.folder.clone().unwrap();

        if !folder_path.exists() {
            anyhow::bail!("Folder does not exist: {}", folder_path.display());
        }

        if !folder_path.is_dir() {
            anyhow::bail!("Path is not a directory: {}", folder_path.display());
        }

        Ok((Mode::FolderDiscovery, None, None, Some(folder_path)))
    }
}

fn handle_single_file_mode(
    verifier: &TestVerifier,
    log_path: &PathBuf,
    test_case_id: &str,
) -> Result<BatchVerificationReport> {
    // Parse log file with specified test case ID
    let logs = verifier
        .parse_log_file_with_test_case_id(log_path, test_case_id)
        .context("Failed to parse test execution log")?;

    // Load test case
    let test_case = verifier
        .storage()
        .load_test_case_by_id(test_case_id)
        .context(format!("Failed to load test case: {}", test_case_id))?;

    // Verify
    let result = verifier.verify_test_case(&test_case, &logs);

    // Create batch report with single test case
    let mut report = BatchVerificationReport::new();
    report.add_test_case_result(result);

    Ok(report)
}

fn handle_folder_mode(
    verifier: &TestVerifier,
    folder_path: &PathBuf,
) -> Result<BatchVerificationReport> {
    // Discover all log files in the folder
    let log_files = discover_log_files(folder_path)?;

    if log_files.is_empty() {
        log::warn!("No log files found in folder: {}", folder_path.display());
        return Ok(BatchVerificationReport::new());
    }

    log::info!("Found {} log file(s) in {}", log_files.len(), folder_path.display());

    // Perform batch verification
    verifier
        .batch_verify(&log_files)
        .context("Failed to perform batch verification")
}

fn discover_log_files(folder_path: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut log_files = Vec::new();

    let entries = fs::read_dir(folder_path)
        .context(format!("Failed to read directory: {}", folder_path.display()))?;

    for entry in entries {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                // Look for common log file extensions
                if ext_str == "json" || ext_str == "log" || ext_str == "txt" {
                    log_files.push(path);
                }
            }
        }
    }

    Ok(log_files)
}

fn generate_output(
    verifier: &TestVerifier,
    report: &BatchVerificationReport,
    format: &str,
) -> Result<String> {
    match format.to_lowercase().as_str() {
        "yaml" => verifier
            .generate_container_report(report, "yaml")
            .context("Failed to generate YAML report"),
        "json" => verifier
            .generate_container_report(report, "json")
            .context("Failed to generate JSON report"),
        _ => anyhow::bail!("Unsupported format: {}", format),
    }
}

fn write_output(content: &str, output_path: Option<&PathBuf>) -> Result<()> {
    if let Some(path) = output_path {
        fs::write(path, content)
            .context(format!("Failed to write output to {}", path.display()))?;
        log::info!("Report written to {}", path.display());
    } else {
        println!("{}", content);
    }

    Ok(())
}
