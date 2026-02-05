use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use testcase_manager::{TestCase, TestExecutor, VarHydrator};

#[derive(Parser)]
#[command(name = "test-executor")]
#[command(
    about = "Generate and execute test scripts from YAML test case files",
    version
)]
struct Cli {
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
}

fn load_test_case(yaml_file: &PathBuf) -> Result<TestCase> {
    let yaml_content = fs::read_to_string(yaml_file)
        .context(format!("Failed to read YAML file: {}", yaml_file.display()))?;

    let test_case: TestCase =
        serde_yaml::from_str(&yaml_content).context("Failed to parse YAML content as TestCase")?;

    Ok(test_case)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            yaml_file,
            output,
            json_log,
        } => {
            let test_case = load_test_case(&yaml_file)?;
            let executor = TestExecutor::new();
            let script = executor.generate_test_script(&test_case);

            if let Some(output_path) = output {
                fs::write(&output_path, &script).context(format!(
                    "Failed to write script to file: {}",
                    output_path.display()
                ))?;
                println!(
                    "Test script generated successfully: {}",
                    output_path.display()
                );

                if json_log {
                    executor.generate_execution_log_template(&test_case, &output_path)?;
                }
            } else {
                print!("{}", script);
                if json_log {
                    eprintln!("Warning: --json-log requires --output to be specified");
                }
            }

            Ok(())
        }
        Commands::Execute { yaml_file } => {
            let test_case = load_test_case(&yaml_file)?;
            let executor = TestExecutor::new();

            executor.execute_test_case(&test_case)?;

            Ok(())
        }
        Commands::Hydrate {
            yaml_file,
            export_file,
            output,
        } => {
            let yaml_content = fs::read_to_string(&yaml_file)
                .context(format!("Failed to read YAML file: {}", yaml_file.display()))?;

            let mut hydrator = VarHydrator::new();
            hydrator.load_from_export_file(&export_file)?;

            let hydrated_content = hydrator.hydrate_yaml_content(&yaml_content);

            if let Some(output_path) = output {
                fs::write(&output_path, &hydrated_content).context(format!(
                    "Failed to write hydrated YAML to file: {}",
                    output_path.display()
                ))?;
                println!("Hydrated YAML written to: {}", output_path.display());
            } else {
                print!("{}", hydrated_content);
            }

            Ok(())
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
    }
}
