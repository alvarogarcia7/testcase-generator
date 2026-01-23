use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use testcase_manager::LogCleaner;

#[derive(Parser)]
#[command(name = "script-cleanup")]
#[command(
    about = "Clean script capture output by removing ANSI codes, backspaces, and control characters",
    version
)]
struct Cli {
    /// Path to the input file to clean
    #[arg(short, long, value_name = "INPUT_FILE")]
    input: PathBuf,

    /// Path to the output file (defaults to stdout if not provided)
    #[arg(short, long, value_name = "OUTPUT_FILE")]
    output: Option<PathBuf>,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = if cli.verbose { "info" } else { "warn" };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    log::info!("Reading input file: {}", cli.input.display());
    let input_content = fs::read_to_string(&cli.input).context(format!(
        "Failed to read input file: {}",
        cli.input.display()
    ))?;

    log::info!("Cleaning script capture output");
    let cleaner = LogCleaner::new();
    let cleaned_output = cleaner.clean_script_capture(&input_content);

    if let Some(output_path) = cli.output {
        log::info!("Writing cleaned output to: {}", output_path.display());
        fs::write(&output_path, &cleaned_output).context(format!(
            "Failed to write output file: {}",
            output_path.display()
        ))?;
        log::info!(
            "✓ Successfully cleaned and wrote output to {}",
            output_path.display()
        );
    } else {
        print!("{}", cleaned_output);
    }

    Ok(())
}
