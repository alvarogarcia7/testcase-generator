use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use testcase_common::resolve_schema_from_payload;

#[derive(Parser)]
#[command(name = "validate-json")]
#[command(about = "Validate a JSON payload against a JSON schema", version)]
#[command(
    after_help = "ENVIRONMENT VARIABLES:\n    RUST_LOG    Set log level (trace, debug, info, warn, error). Overrides --log-level"
)]
struct Cli {
    /// Path to the JSON payload file
    #[arg(value_name = "JSON_FILE")]
    json_file: PathBuf,

    /// Path to the JSON schema file (optional, auto-resolved from 'schema' field if not provided)
    /// Canonical paths:
    ///   - Legacy schemas: schemas/*.schema.json
    ///   - Versioned schemas: schemas/tcms/*.schema.v1.json
    #[arg(value_name = "SCHEMA_FILE")]
    schema_file: Option<PathBuf>,

    /// Root directory containing schema files for auto-resolution
    /// Default: schemas/ (contains both legacy and versioned schemas)
    #[arg(long, value_name = "SCHEMAS_ROOT", default_value = "schemas/")]
    schemas_root: String,

    /// Set log level (trace, debug, info, warn, error)
    #[arg(long, value_name = "LEVEL", default_value = "warn")]
    log_level: String,

    /// Enable verbose output (equivalent to --log-level=info)
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = if cli.verbose { "info" } else { &cli.log_level };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    // Resolve schema path
    let schema_path = if let Some(explicit_schema) = cli.schema_file {
        log::debug!("Using explicit schema: {}", explicit_schema.display());
        explicit_schema
    } else {
        log::debug!(
            "Auto-resolving schema from payload with schemas root: {}",
            cli.schemas_root
        );
        resolve_schema_from_payload(&cli.json_file, &cli.schemas_root)
            .context("Failed to auto-resolve schema from payload")?
    };

    log::info!("Using schema: {}", schema_path.display());

    // Read the JSON file
    let json_content = fs::read_to_string(&cli.json_file).context(format!(
        "Failed to read JSON file: {}",
        cli.json_file.display()
    ))?;

    // Read the JSON schema file
    let schema_content = fs::read_to_string(&schema_path).context(format!(
        "Failed to read schema file: {}",
        schema_path.display()
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
