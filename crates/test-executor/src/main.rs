use anyhow::{Context, Result};
use audit_traceability::{AuditTraceabilityLog, StageInfo, TestCaseAudit};
use audit_verifier::audit_log::OperationStatus;
use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use testcase_common::load_and_validate_yaml;
use testcase_execution::{AuditLogger, TestExecutor, VarHydrator};
use testcase_models::TestCase;
use testcase_storage::{TestCaseFilter, TestCaseFilterer, TestCaseStorage};
use testcase_validation::DependencyResolver;

#[derive(Parser)]
#[command(name = "test-executor")]
#[command(
    about = "Generate and execute test scripts from YAML test case files",
    version
)]
#[command(
    after_help = "ENVIRONMENT VARIABLES:\n    RUST_LOG    Set log level (trace, debug, info, warn, error). Overrides --log-level\n    AUDIT_LOG_FILE    Path to audit log file (default: audit.log.json)\n    AUDIT_LOG_ENABLED    Enable/disable audit logging (default: true)"
)]
struct Cli {
    /// Set log level (trace, debug, info, warn, error)
    #[arg(long, value_name = "LEVEL", default_value = "warn", global = true)]
    log_level: String,

    /// Enable verbose output (equivalent to --log-level=info)
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Path to audit log file (overrides AUDIT_LOG_FILE env var)
    #[arg(long, value_name = "PATH", global = true)]
    audit_log: Option<PathBuf>,

    /// Disable audit logging
    #[arg(long, global = true)]
    no_audit: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a shell script from a test case YAML file
    Generate {
        /// Path to the test case YAML file
        #[arg(value_name = "YAML_FILE")]
        yaml_file: PathBuf,

        /// Optional output file path (defaults to stdout)
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,

        /// Generate execution log JSON file alongside bash script
        #[arg(long)]
        json_log: bool,

        /// Force output even if shellcheck validation fails
        #[arg(short = 'f', long)]
        force: bool,

        /// Optional test case directory for dependency resolution (defaults to parent directory of YAML file)
        #[arg(long, value_name = "TEST_CASE_DIR")]
        test_case_dir: Option<PathBuf>,

        /// Update audit traceability log with generated artifacts
        #[arg(long, value_name = "AUDIT_LOG")]
        audit_log: Option<PathBuf>,
    },
    /// Execute a test case by generating and running the script
    Execute {
        /// Path to the test case YAML file
        #[arg(value_name = "YAML_FILE")]
        yaml_file: PathBuf,
    },
    /// Hydrate a test case YAML file with variable values from an export file
    Hydrate {
        /// Path to the test case YAML file
        #[arg(value_name = "YAML_FILE")]
        yaml_file: PathBuf,

        /// Path to the export file containing variable values
        #[arg(short, long, value_name = "EXPORT_FILE")]
        export_file: PathBuf,

        /// Optional output file path (defaults to stdout)
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
    },
    /// Generate an export file template from test case hydration_vars declarations
    GenerateExport {
        /// Path to the test case YAML file
        #[arg(value_name = "YAML_FILE")]
        yaml_file: PathBuf,

        /// Optional output file path (defaults to stdout)
        #[arg(short, long, value_name = "OUTPUT_FILE")]
        output: Option<PathBuf>,
    },
    /// Validate that an export file has all required variables from test case
    ValidateExport {
        /// Path to the test case YAML file
        #[arg(value_name = "YAML_FILE")]
        yaml_file: PathBuf,

        /// Path to the export file to validate
        #[arg(short, long, value_name = "EXPORT_FILE")]
        export_file: PathBuf,
    },
    /// List all test cases with optional filtering
    List {
        /// Optional base path to test cases directory (defaults to "testcases")
        #[arg(value_name = "BASE_PATH")]
        base_path: Option<PathBuf>,

        /// Show only test cases with manual steps
        #[arg(long, conflicts_with = "automated_only")]
        manual_only: bool,

        /// Show only test cases with automated steps
        #[arg(long, conflicts_with = "manual_only")]
        automated_only: bool,

        /// Show statistics about test cases
        #[arg(long)]
        show_stats: bool,
    },
    /// Resolve dependencies in test case YAML files
    Resolve {
        /// Paths to YAML test case files
        #[arg(value_name = "YAML_FILES", required = true)]
        yaml_files: Vec<PathBuf>,

        /// Optional output directory (defaults to stdout)
        #[arg(short, long, value_name = "OUTPUT_DIR")]
        output: Option<PathBuf>,
    },
    /// Create or update audit traceability log
    AuditLog {
        #[command(subcommand)]
        command: AuditLogCommands,
    },
}

#[derive(Subcommand)]
enum AuditLogCommands {
    /// Create a new audit traceability log
    Create {
        /// Output path for the audit log JSON file
        #[arg(short, long, default_value = "audit-traceability-log.json")]
        output: PathBuf,

        /// Witness key for the audit log
        #[arg(short, long, default_value = "default-witness")]
        witness_key: String,
    },
    /// Add a test case to the audit log
    Add {
        /// Path to the audit log JSON file
        #[arg(short, long, default_value = "audit-traceability-log.json")]
        log_file: PathBuf,

        /// Test case ID
        #[arg(long)]
        test_case_id: String,

        /// Path to initial YAML file
        #[arg(long)]
        initial: PathBuf,

        /// Path to generated shell script (stage: 05_shell_script)
        #[arg(long)]
        shell_script: Option<PathBuf>,
    },
    /// Verify all files in the audit log
    Verify {
        /// Path to the audit log JSON file
        #[arg(short, long, default_value = "audit-traceability-log.json")]
        log_file: PathBuf,

        /// Optional test case ID to verify (verifies all if not specified)
        #[arg(long)]
        test_case_id: Option<String>,
    },
}

fn load_test_case(yaml_file: &PathBuf) -> Result<TestCase> {
    load_and_validate_yaml::<TestCase, _, _>(yaml_file, "schemas/").context(format!(
        "Failed to load and validate YAML file: {}",
        yaml_file.display()
    ))
}

fn load_all_yaml_files_from_dir(dir: &PathBuf) -> Result<Vec<(PathBuf, TestCase)>> {
    let mut test_cases = Vec::new();

    if !dir.is_dir() {
        return Ok(test_cases);
    }

    for entry in
        fs::read_dir(dir).context(format!("Failed to read directory: {}", dir.display()))?
    {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if ext_str == "yaml" || ext_str == "yml" {
                    match load_test_case(&path) {
                        Ok(test_case) => {
                            test_cases.push((path, test_case));
                        }
                        Err(e) => {
                            eprintln!("Warning: Failed to load {}: {}", path.display(), e);
                        }
                    }
                }
            }
        }
    }

    Ok(test_cases)
}

fn build_dependency_resolver(yaml_file: &Path) -> Result<DependencyResolver> {
    let dir = yaml_file
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Failed to get parent directory"))?
        .to_path_buf();

    let test_cases = load_all_yaml_files_from_dir(&dir)?;

    let mut index = HashMap::new();
    for (_, test_case) in test_cases {
        index.insert(test_case.id.clone(), test_case);
    }

    Ok(DependencyResolver::new(index))
}

fn build_dependency_resolver_from_dir(test_case_dir: &Path) -> Result<DependencyResolver> {
    let test_cases = load_all_yaml_files_from_dir_recursive(test_case_dir)?;

    let mut index = HashMap::new();
    for (_, test_case) in test_cases {
        index.insert(test_case.id.clone(), test_case);
    }

    Ok(DependencyResolver::new(index))
}

fn load_all_yaml_files_from_dir_recursive(dir: &Path) -> Result<Vec<(PathBuf, TestCase)>> {
    let mut test_cases = Vec::new();

    if !dir.is_dir() {
        return Ok(test_cases);
    }

    // Load YAML files from this directory
    for entry in
        fs::read_dir(dir).context(format!("Failed to read directory: {}", dir.display()))?
    {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if ext_str == "yaml" || ext_str == "yml" {
                    match load_test_case(&path) {
                        Ok(test_case) => {
                            test_cases.push((path, test_case));
                        }
                        Err(e) => {
                            eprintln!("Warning: Failed to load {}: {}", path.display(), e);
                        }
                    }
                }
            }
        } else if path.is_dir() {
            // Recursively load from subdirectories
            match load_all_yaml_files_from_dir_recursive(&path) {
                Ok(mut sub_test_cases) => {
                    test_cases.append(&mut sub_test_cases);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to load from {}: {}", path.display(), e);
                }
            }
        }
    }

    Ok(test_cases)
}

fn build_resolver_from_files(yaml_files: &[PathBuf]) -> Result<DependencyResolver> {
    let mut index = HashMap::new();

    for yaml_file in yaml_files {
        let test_case = load_test_case(yaml_file)?;
        index.insert(test_case.id.clone(), test_case);
    }

    Ok(DependencyResolver::new(index))
}

fn list_test_cases(
    base_path: Option<PathBuf>,
    manual_only: bool,
    automated_only: bool,
    show_stats: bool,
) -> Result<()> {
    let path = base_path.unwrap_or_else(|| PathBuf::from("testcases"));
    let storage = TestCaseStorage::new(&path)?;

    let test_cases = storage.load_all_test_cases()?;

    let filter = if manual_only {
        TestCaseFilter::ManualOnly
    } else if automated_only {
        TestCaseFilter::AutomatedOnly
    } else {
        TestCaseFilter::All
    };

    let filterer = TestCaseFilterer::new();
    let filtered_cases = filterer.filter_test_cases(test_cases.clone(), filter);

    if filtered_cases.is_empty() {
        println!("No test cases found.");
        return Ok(());
    }

    println!("Test Cases:");
    println!();

    for test_case in &filtered_cases {
        let manual_step_count = test_case.get_manual_step_count();
        let manual_indicator = if manual_step_count > 0 {
            format!(" [M:{}]", manual_step_count)
        } else {
            String::new()
        };

        println!(
            "  {}{} - {}",
            test_case.id, manual_indicator, test_case.description
        );
    }

    if show_stats {
        println!();
        println!("Statistics:");

        let total_count = filtered_cases.len();
        let manual_count = filtered_cases
            .iter()
            .filter(|tc| tc.has_manual_steps())
            .count();
        let automated_count = filtered_cases
            .iter()
            .filter(|tc| tc.has_automated_steps())
            .count();
        let total_manual_steps: usize = filtered_cases
            .iter()
            .map(|tc| tc.get_manual_step_count())
            .sum();

        println!("  Total test cases: {}", total_count);
        println!("  Test cases with manual steps: {}", manual_count);
        println!("  Test cases with automated steps: {}", automated_count);
        println!("  Total manual steps: {}", total_manual_steps);
    }

    Ok(())
}

/// Run shellcheck validation on generated script content.
/// Returns true if validation passed (or shellcheck not available), false if errors found.
fn run_shellcheck_validation(script_content: &str, force: bool) -> Result<bool> {
    let shellcheck_available = std::process::Command::new("shellcheck")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok();

    if !shellcheck_available {
        eprintln!("Warning: shellcheck not found on PATH, skipping validation");
        return Ok(true);
    }

    let temp_file = tempfile::Builder::new()
        .suffix(".sh")
        .tempfile()
        .context("Failed to create temporary file for shellcheck")?;

    fs::write(temp_file.path(), script_content)
        .context("Failed to write script to temporary file")?;

    let output = std::process::Command::new("shellcheck")
        .arg("-S")
        .arg("error")
        .arg(temp_file.path())
        .output()
        .context("Failed to run shellcheck")?;

    if output.status.success() {
        eprintln!("shellcheck: validation passed");
        Ok(true)
    } else {
        let findings = String::from_utf8_lossy(&output.stdout);
        if force {
            eprintln!(
                "Warning: shellcheck found errors (--force specified, continuing):\n{}",
                findings
            );
            Ok(true)
        } else {
            eprintln!("Error: shellcheck validation failed:\n{}", findings);
            eprintln!("Use -f/--force to skip shellcheck validation");
            Ok(false)
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = if cli.verbose { "info" } else { &cli.log_level };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    let audit_log_path = cli
        .audit_log
        .clone()
        .or_else(|| std::env::var("AUDIT_LOG_FILE").ok().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("audit.log.json"));

    let mut audit_logger = AuditLogger::with_file(&audit_log_path);
    if cli.no_audit {
        audit_logger.disable();
    }

    match cli.command {
        Commands::Generate {
            yaml_file,
            output,
            json_log,
            force,
            test_case_dir,
            audit_log,
        } => {
            let result = (|| -> Result<()> {
                let yaml_bytes = fs::read(&yaml_file)
                    .context(format!("Failed to read YAML file: {}", yaml_file.display()))?;
                let mut test_case = load_test_case(&yaml_file)?;

                let resolver = if let Some(dir) = test_case_dir {
                    build_dependency_resolver_from_dir(&dir)?
                } else {
                    build_dependency_resolver(&yaml_file)?
                };
                test_case = resolver
                    .resolve(&test_case)
                    .context("Failed to resolve dependencies")?;

                let executor = TestExecutor::new();
                let script = executor.generate_test_script_from_yaml_with_path(
                    &test_case,
                    &yaml_bytes,
                    &yaml_file,
                );

                if let Some(output_path) = &output {
                    fs::write(output_path, &script).context(format!(
                        "Failed to write script to file: {}",
                        output_path.display()
                    ))?;
                    println!(
                        "Test script generated successfully: {}",
                        output_path.display()
                    );

                    if json_log {
                        executor.generate_execution_log_template(&test_case, output_path)?;
                    }

                    if let Some(audit_log_path) = audit_log {
                        let mut log = if audit_log_path.exists() {
                            AuditTraceabilityLog::load_from_file(&audit_log_path)?
                        } else {
                            AuditTraceabilityLog::new("default-witness".to_string())
                        };

                        let mut audit = TestCaseAudit::new();

                        let initial_stage = StageInfo::from_file(&yaml_file).context(format!(
                            "Failed to create stage info for initial file: {}",
                            yaml_file.display()
                        ))?;
                        audit.add_stage("initial", initial_stage);

                        let script_stage = StageInfo::from_file(output_path).context(format!(
                            "Failed to create stage info for shell script: {}",
                            output_path.display()
                        ))?;
                        audit.add_stage("05_shell_script", script_stage);

                        log.add_test_case(&test_case.id, audit);
                        log.save_to_file(&audit_log_path)?;

                        println!(
                            "✓ Test case '{}' added to audit log: {}",
                            test_case.id,
                            audit_log_path.display()
                        );
                    }
                } else {
                    print!("{}", script);
                    if json_log {
                        eprintln!("Warning: --json-log requires --output to be specified");
                    }
                    if audit_log.is_some() {
                        eprintln!("Warning: --audit-log requires --output to be specified");
                    }
                }

                let shellcheck_passed = run_shellcheck_validation(&script, force)?;
                if !shellcheck_passed {
                    std::process::exit(1);
                }

                Ok(())
            })();

            let status = if result.is_ok() {
                OperationStatus::Success
            } else {
                OperationStatus::Failed
            };
            let error = result.as_ref().err().map(|e| e.to_string());
            audit_logger.log_generate_script(
                &yaml_file,
                output.as_deref(),
                status,
                error.clone(),
            )?;

            result
        }
        Commands::Execute { yaml_file } => {
            let result = (|| -> Result<()> {
                let mut test_case = load_test_case(&yaml_file)?;

                let resolver = build_dependency_resolver(&yaml_file)?;
                test_case = resolver
                    .resolve(&test_case)
                    .context("Failed to resolve dependencies")?;

                let executor = TestExecutor::new();
                executor.execute_test_case(&test_case)?;

                Ok(())
            })();

            let status = if result.is_ok() {
                OperationStatus::Success
            } else {
                OperationStatus::Failed
            };
            let error = result.as_ref().err().map(|e| e.to_string());
            audit_logger.log_execute_script(&yaml_file, status, error.clone())?;

            result
        }
        Commands::Hydrate {
            yaml_file,
            export_file,
            output,
        } => {
            let result = (|| -> Result<()> {
                let yaml_content = fs::read_to_string(&yaml_file)
                    .context(format!("Failed to read YAML file: {}", yaml_file.display()))?;

                let mut hydrator = VarHydrator::new();
                hydrator.load_from_export_file(&export_file)?;

                let hydrated_content = hydrator.hydrate_yaml_content(&yaml_content);

                if let Some(output_path) = &output {
                    fs::write(output_path, &hydrated_content).context(format!(
                        "Failed to write hydrated YAML to file: {}",
                        output_path.display()
                    ))?;
                    println!("Hydrated YAML written to: {}", output_path.display());
                } else {
                    print!("{}", hydrated_content);
                }

                Ok(())
            })();

            let status = if result.is_ok() {
                OperationStatus::Success
            } else {
                OperationStatus::Failed
            };
            let error = result.as_ref().err().map(|e| e.to_string());
            audit_logger.log_hydrate_yaml(
                &yaml_file,
                &export_file,
                output.as_deref(),
                status,
                error.clone(),
            )?;

            result
        }
        Commands::GenerateExport { yaml_file, output } => {
            let test_case = load_test_case(&yaml_file)?;

            let mut hydrator = VarHydrator::new();

            if let Some(hydration_vars) = &test_case.hydration_vars {
                for (var_name, env_var) in hydration_vars {
                    let value = env_var.default_value.as_deref().unwrap_or("");
                    hydrator.set(var_name.clone(), value.to_string());
                }
            }

            if let Some(output_path) = output {
                hydrator.generate_export_file(&output_path)?;
                println!("Export file template generated: {}", output_path.display());
            } else {
                let temp_file =
                    tempfile::NamedTempFile::new().context("Failed to create temporary file")?;
                hydrator.generate_export_file(temp_file.path())?;
                let content = fs::read_to_string(temp_file.path())
                    .context("Failed to read temporary file")?;
                print!("{}", content);
            }

            Ok(())
        }
        Commands::ValidateExport {
            yaml_file,
            export_file,
        } => {
            let test_case = load_test_case(&yaml_file)?;

            let mut hydrator = VarHydrator::new();
            hydrator.load_from_export_file(&export_file)?;

            let mut missing_vars = Vec::new();
            let mut missing_required_vars = Vec::new();

            if let Some(hydration_vars) = &test_case.hydration_vars {
                for (var_name, env_var) in hydration_vars {
                    if !hydrator.contains(var_name) {
                        missing_vars.push(var_name.clone());
                        if env_var.required {
                            missing_required_vars.push(var_name.clone());
                        }
                    }
                }
            }

            if missing_required_vars.is_empty() {
                if missing_vars.is_empty() {
                    println!("✓ Export file is valid: all variables are present");
                } else {
                    println!("✓ Export file is valid: all required variables are present");
                    println!("  Optional variables missing: {}", missing_vars.join(", "));
                }
                Ok(())
            } else {
                eprintln!("✗ Export file validation failed");
                eprintln!(
                    "  Required variables missing: {}",
                    missing_required_vars.join(", ")
                );
                if !missing_vars.is_empty() {
                    let optional_missing: Vec<String> = missing_vars
                        .into_iter()
                        .filter(|v| !missing_required_vars.contains(v))
                        .collect();
                    if !optional_missing.is_empty() {
                        eprintln!(
                            "  Optional variables missing: {}",
                            optional_missing.join(", ")
                        );
                    }
                }
                std::process::exit(1);
            }
        }
        Commands::List {
            base_path,
            manual_only,
            automated_only,
            show_stats,
        } => list_test_cases(base_path, manual_only, automated_only, show_stats),
        Commands::Resolve { yaml_files, output } => {
            let resolver = build_resolver_from_files(&yaml_files)?;

            if let Some(output_dir) = output {
                if !output_dir.exists() {
                    fs::create_dir_all(&output_dir).context(format!(
                        "Failed to create output directory: {}",
                        output_dir.display()
                    ))?;
                }

                for yaml_file in &yaml_files {
                    let test_case = load_test_case(yaml_file)?;
                    let resolved = resolver.resolve(&test_case)?;

                    let output_filename = format!("{}_resolved.yaml", resolved.id);
                    let output_path = output_dir.join(&output_filename);

                    let yaml_content = serde_yaml::to_string(&resolved)
                        .context("Failed to serialize resolved test case")?;

                    fs::write(&output_path, yaml_content).context(format!(
                        "Failed to write resolved YAML to: {}",
                        output_path.display()
                    ))?;

                    println!("Resolved test case written to: {}", output_path.display());
                }
            } else {
                for yaml_file in &yaml_files {
                    let test_case = load_test_case(yaml_file)?;
                    let resolved = resolver.resolve(&test_case)?;

                    let yaml_content = serde_yaml::to_string(&resolved)
                        .context("Failed to serialize resolved test case")?;

                    println!("---");
                    println!("# Resolved: {}", yaml_file.display());
                    print!("{}", yaml_content);
                }
            }

            Ok(())
        }
        Commands::AuditLog { command } => match command {
            AuditLogCommands::Create {
                output,
                witness_key,
            } => {
                let log = AuditTraceabilityLog::new(witness_key);
                log.save_to_file(&output)?;
                println!("✓ Audit traceability log created: {}", output.display());
                Ok(())
            }
            AuditLogCommands::Add {
                log_file,
                test_case_id,
                initial,
                shell_script,
            } => {
                let mut log = if log_file.exists() {
                    AuditTraceabilityLog::load_from_file(&log_file)?
                } else {
                    AuditTraceabilityLog::new("default-witness".to_string())
                };

                let mut audit = TestCaseAudit::new();

                let initial_stage = StageInfo::from_file(&initial).context(format!(
                    "Failed to create stage info for initial file: {}",
                    initial.display()
                ))?;
                audit.add_stage("initial", initial_stage);

                if let Some(script_path) = shell_script {
                    let script_stage = StageInfo::from_file(&script_path).context(format!(
                        "Failed to create stage info for shell script: {}",
                        script_path.display()
                    ))?;
                    audit.add_stage("05_shell_script", script_stage);
                }

                log.add_test_case(&test_case_id, audit);
                log.save_to_file(&log_file)?;

                println!(
                    "✓ Test case '{}' added to audit log: {}",
                    test_case_id,
                    log_file.display()
                );
                Ok(())
            }
            AuditLogCommands::Verify {
                log_file,
                test_case_id,
            } => {
                let log = AuditTraceabilityLog::load_from_file(&log_file)?;

                if let Some(tc_id) = test_case_id {
                    let result = log.verify_test_case(&tc_id)?;
                    result.print_summary();

                    if !result.all_passed {
                        std::process::exit(1);
                    }
                } else {
                    let results = log.verify_all()?;

                    if results.is_empty() {
                        println!("No test cases found in audit log.");
                        return Ok(());
                    }

                    println!("\n=== Audit Verification Results ===\n");

                    let mut all_passed = true;
                    for result in &results {
                        result.print_summary();
                        println!();
                        if !result.all_passed {
                            all_passed = false;
                        }
                    }

                    println!("=== Summary ===");
                    let passed_count = results.iter().filter(|r| r.all_passed).count();
                    let failed_count = results.len() - passed_count;
                    println!("Total test cases: {}", results.len());
                    println!("Passed: {}", passed_count);
                    println!("Failed: {}", failed_count);

                    if !all_passed {
                        std::process::exit(1);
                    }
                }

                Ok(())
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shellcheck_validation_valid_script() {
        if std::process::Command::new("shellcheck")
            .arg("--version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .is_err()
        {
            eprintln!("shellcheck not available, skipping test");
            return;
        }

        let script = "#!/bin/bash\nset -euo pipefail\necho 'hello world'\n";
        let result = run_shellcheck_validation(script, false).unwrap();
        assert!(result, "Valid script should pass shellcheck");
    }

    #[test]
    fn test_shellcheck_validation_force_flag() {
        if std::process::Command::new("shellcheck")
            .arg("--version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .is_err()
        {
            eprintln!("shellcheck not available, skipping test");
            return;
        }

        // Even if shellcheck finds issues, force=true should return Ok(true)
        let script = "#!/bin/bash\necho 'hello world'\n";
        let result = run_shellcheck_validation(script, true).unwrap();
        assert!(result, "Force flag should always return true");
    }

    #[test]
    fn test_shellcheck_validation_generated_script() {
        if std::process::Command::new("shellcheck")
            .arg("--version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .is_err()
        {
            eprintln!("shellcheck not available, skipping test");
            return;
        }

        // Load a real test case YAML file and generate a script from it
        let yaml_path = PathBuf::from("testcases/self_validated_example.yml");
        if !yaml_path.exists() {
            eprintln!("Test YAML file not found, skipping test");
            return;
        }

        let test_case = load_test_case(&yaml_path).expect("Failed to load test case YAML");
        let executor = TestExecutor::new();
        let script = executor.generate_test_script(&test_case);
        let result = run_shellcheck_validation(&script, false).unwrap();
        assert!(
            result,
            "Generated test script should pass shellcheck validation"
        );
    }
}
