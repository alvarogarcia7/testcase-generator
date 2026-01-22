use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "validate-json")]
#[command(about = "Validate a JSON payload against a JSON schema", version)]
struct Cli {
    /// Path to the JSON payload file
    #[arg(value_name = "JSON_FILE")]
    json_file: PathBuf,

    /// Path to the JSON schema file
    #[arg(value_name = "SCHEMA_FILE")]
    schema_file: PathBuf,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = if cli.verbose { "info" } else { "warn" };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    // Read the JSON file
    let json_content = fs::read_to_string(&cli.json_file).context(format!(
        "Failed to read JSON file: {}",
        cli.json_file.display()
    ))?;

    // Read the JSON schema file
    let schema_content = fs::read_to_string(&cli.schema_file).context(format!(
        "Failed to read schema file: {}",
        cli.schema_file.display()
    ))?;

    // Parse the schema
    let schema_value: serde_json::Value =
        serde_json::from_str(&schema_content).context("Failed to parse JSON schema")?;

    // Parse the JSON content
    let json_value: serde_json::Value = serde_json::from_str(&json_content).context(format!(
        "Failed to parse JSON content from {}",
        cli.json_file.display()
    ))?;

    // Compile the schema
    let compiled_schema = jsonschema::JSONSchema::compile(&schema_value)
        .map_err(|e| anyhow::anyhow!("Failed to compile JSON schema: {}", e))?;

    // Validate
    if let Err(errors) = compiled_schema.validate(&json_value) {
        log::error!("✗ Validation failed!");
        log::error!("The following schema constraint violations were found:");

        for (idx, error) in errors.enumerate() {
            let path = if error.instance_path.to_string().is_empty() {
                "root".to_string()
            } else {
                error.instance_path.to_string()
            };

            log::error!("Error #{}: Path '{}'", idx + 1, path);
            log::error!("  Constraint: {}", error);

            let instance = error.instance.as_ref();
            log::error!("  Found value: {}", instance);
        }

        anyhow::bail!("Validation failed with schema constraint violations");
    }

    log::info!("✓ Validation successful!");
    log::info!("The JSON payload is valid according to the provided schema.");
    Ok(())
}
