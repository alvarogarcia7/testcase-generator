use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use testcase_manager::yaml_utils::log_yaml_parse_error;

#[derive(Parser)]
#[command(name = "validate-yaml")]
#[command(about = "Validate YAML payloads against a JSON schema", version)]
struct Cli {
    /// Path(s) to the YAML payload file(s)
    #[arg(value_name = "YAML_FILES", required = true, num_args = 1..)]
    yaml_files: Vec<PathBuf>,

    /// Path to the JSON schema file
    #[arg(short, long, value_name = "SCHEMA_FILE")]
    schema: PathBuf,

    /// Watch mode - monitor YAML files for changes and re-validate
    #[cfg(not(target_os = "windows"))]
    #[arg(short, long)]
    watch: bool,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

struct ValidationResult {
    file_path: PathBuf,
    success: bool,
    error_messages: Vec<String>,
}

const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_RESET: &str = "\x1b[0m";
const COLOR_BOLD: &str = "\x1b[1m";

pub fn validate_single_file<P: AsRef<Path>, S: AsRef<Path>>(
    yaml_path: P,
    schema_path: S,
) -> Result<()> {
    let yaml_path = yaml_path.as_ref();
    let schema_path = schema_path.as_ref();

    let schema_content = fs::read_to_string(schema_path).context(format!(
        "Failed to read schema file: {}",
        schema_path.display()
    ))?;

    let schema_value: serde_json::Value =
        serde_json::from_str(&schema_content).context("Failed to parse JSON schema")?;

    let compiled_schema = jsonschema::JSONSchema::compile(&schema_value)
        .map_err(|e| anyhow::anyhow!("Failed to compile JSON schema: {}", e))?;

    let yaml_content = fs::read_to_string(yaml_path)
        .context(format!("Failed to read YAML file: {}", yaml_path.display()))?;

    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&yaml_content).map_err(|e| {
        log_yaml_parse_error(&e, &yaml_content, &yaml_path.to_string_lossy());
        anyhow::anyhow!("Failed to parse YAML: {}", e)
    })?;

    let json_value: serde_json::Value =
        serde_json::to_value(&yaml_value).context("Failed to convert YAML to JSON")?;

    if let Err(errors) = compiled_schema.validate(&json_value) {
        let mut error_messages = vec!["Schema constraint violations:".to_string()];

        for (idx, error) in errors.enumerate() {
            let path = if error.instance_path.to_string().is_empty() {
                "root".to_string()
            } else {
                error.instance_path.to_string()
            };

            error_messages.push(format!("  Error #{}: Path '{}'", idx + 1, path));
            error_messages.push(format!("    Constraint: {}", error));

            let instance = error.instance.as_ref();
            error_messages.push(format!("    Found value: {}", instance));
        }

        return Err(anyhow::anyhow!(error_messages.join("\n")));
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = if cli.verbose { "info" } else { "warn" };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    let mut results = Vec::new();

    for yaml_file in &cli.yaml_files {
        let validation_result = validate_single_file(yaml_file, &cli.schema);

        let result = match validation_result {
            Ok(_) => ValidationResult {
                file_path: yaml_file.clone(),
                success: true,
                error_messages: Vec::new(),
            },
            Err(e) => ValidationResult {
                file_path: yaml_file.clone(),
                success: false,
                error_messages: e.to_string().lines().map(String::from).collect(),
            },
        };

        results.push(result);
    }

    for result in &results {
        if result.success {
            println!(
                "{}{COLOR_GREEN}✓{COLOR_RESET} {}",
                COLOR_BOLD,
                result.file_path.display()
            );
        } else {
            println!(
                "{}{COLOR_RED}✗{COLOR_RESET} {}",
                COLOR_BOLD,
                result.file_path.display()
            );
            for error_msg in &result.error_messages {
                println!("  {}", error_msg);
            }
        }
    }

    let total = results.len();
    let passed = results.iter().filter(|r| r.success).count();
    let failed = total - passed;

    println!();
    println!("{}Summary:{}", COLOR_BOLD, COLOR_RESET);
    println!("  Total files validated: {}", total);
    println!("  {}Passed: {}{}", COLOR_GREEN, passed, COLOR_RESET);
    println!("  {}Failed: {}{}", COLOR_RED, failed, COLOR_RESET);

    if failed > 0 {
        process::exit(1);
    }

    Ok(())
}
