use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use testcase_manager::{TestCase, TestExecutor};

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
    },
    /// Execute a test case by generating and running the script
    Execute {
        /// Path to the test case YAML file
        #[arg(value_name = "YAML_FILE")]
        yaml_file: PathBuf,
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
        Commands::Generate { yaml_file, output } => {
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
            } else {
                print!("{}", script);
            }

            Ok(())
        }
        Commands::Execute { yaml_file } => {
            let test_case = load_test_case(&yaml_file)?;
            let executor = TestExecutor::new();

            println!("Executing test case: {}", test_case.id);
            println!("Description: {}", test_case.description);
            println!();

            executor.execute_test_case(&test_case)?;

            println!();
            println!("Test execution completed successfully!");

            Ok(())
        }
    }
}
