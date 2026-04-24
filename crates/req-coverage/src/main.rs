mod coverage;
mod html;
mod models;
mod report;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

use coverage::CoverageAnalyzer;
use report::ReportGenerator;

#[derive(Parser)]
#[command(name = "req-coverage")]
#[command(version)]
#[command(about = "Generate requirement coverage reports from test cases and verification results")]
#[command(
    after_help = "ENVIRONMENT VARIABLES:\n    RUST_LOG    Set log level (trace, debug, info, warn, error). Overrides --log-level"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, value_name = "LEVEL", default_value = "info", global = true)]
    log_level: String,

    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Analyze test cases and verification results to generate coverage report")]
    Verify {
        #[arg(long, value_name = "PATH", required = true)]
        test_cases_folder: PathBuf,

        #[arg(long, value_name = "PATH", required = true)]
        test_results_folder: PathBuf,

        #[arg(long, value_name = "FILE", required = true)]
        output: PathBuf,
    },

    #[command(about = "Generate HTML report from coverage JSON")]
    Print {
        #[arg(long, value_name = "FORMAT", default_value = "html")]
        format: String,

        #[arg(long, value_name = "FILE", required = true)]
        input: PathBuf,

        #[arg(long, value_name = "DIR", required = true)]
        output: PathBuf,

        #[arg(long, value_name = "FILE")]
        template: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = if cli.verbose { "debug" } else { &cli.log_level };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    match cli.command {
        Commands::Verify {
            test_cases_folder,
            test_results_folder,
            output,
        } => {
            handle_verify_command(&test_cases_folder, &test_results_folder, &output)?;
        }
        Commands::Print {
            format,
            input,
            output,
            template,
        } => {
            handle_print_command(&format, &input, &output, template.as_deref())?;
        }
    }

    Ok(())
}

fn handle_verify_command(
    test_cases_folder: &PathBuf,
    test_results_folder: &PathBuf,
    output: &PathBuf,
) -> Result<()> {
    log::info!("=== Requirement Coverage Verification ===");
    log::info!("Test cases folder: {:?}", test_cases_folder);
    log::info!("Test results folder: {:?}", test_results_folder);
    log::info!("Output file: {:?}", output);

    if !test_cases_folder.exists() {
        anyhow::bail!("Test cases folder does not exist: {:?}", test_cases_folder);
    }

    if !test_cases_folder.is_dir() {
        anyhow::bail!(
            "Test cases folder is not a directory: {:?}",
            test_cases_folder
        );
    }

    if !test_results_folder.exists() {
        anyhow::bail!(
            "Test results folder does not exist: {:?}",
            test_results_folder
        );
    }

    if !test_results_folder.is_dir() {
        anyhow::bail!(
            "Test results folder is not a directory: {:?}",
            test_results_folder
        );
    }

    let analyzer = CoverageAnalyzer::new(test_cases_folder)
        .context("Failed to initialize coverage analyzer")?;

    let report = analyzer
        .analyze(test_results_folder)
        .context("Failed to analyze coverage")?;

    ReportGenerator::save_coverage_report(&report, output)
        .context("Failed to save coverage report")?;

    log::info!("=== Coverage Analysis Complete ===");
    log::info!("Total requirements: {}", report.total_requirements);
    log::info!(
        "Fully covered: {} ({:.1}%)",
        report.fully_covered_requirements,
        (report.fully_covered_requirements as f64 / report.total_requirements as f64) * 100.0
    );
    log::info!(
        "Partially covered: {} ({:.1}%)",
        report.partially_covered_requirements,
        (report.partially_covered_requirements as f64 / report.total_requirements as f64) * 100.0
    );
    log::info!(
        "Uncovered: {} ({:.1}%)",
        report.uncovered_requirements,
        (report.uncovered_requirements as f64 / report.total_requirements as f64) * 100.0
    );

    Ok(())
}

fn handle_print_command(
    format: &str,
    input: &PathBuf,
    output: &PathBuf,
    template: Option<&Path>,
) -> Result<()> {
    log::info!("=== Generating Coverage Report ===");
    log::info!("Format: {}", format);
    log::info!("Input file: {:?}", input);
    log::info!("Output directory: {:?}", output);
    if let Some(template_path) = template {
        log::info!("Template file: {:?}", template_path);
    }

    if !input.exists() {
        anyhow::bail!("Input file does not exist: {:?}", input);
    }

    if let Some(template_path) = template {
        if !template_path.exists() {
            anyhow::bail!("Template file does not exist: {:?}", template_path);
        }
    }

    let format_lower = format.to_lowercase();
    if format_lower != "html" {
        anyhow::bail!(
            "Unsupported format '{}'. Currently only 'html' is supported.",
            format
        );
    }

    let report =
        ReportGenerator::load_coverage_report(input).context("Failed to load coverage report")?;

    ReportGenerator::generate_html(&report, output, template)
        .context("Failed to generate HTML report")?;

    log::info!("=== Report Generation Complete ===");
    log::info!("HTML report available at: {:?}/index.html", output);

    Ok(())
}
