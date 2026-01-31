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

    // Read the JSON schema file
    let schema_content = fs::read_to_string(&cli.schema).context(format!(
        "Failed to read schema file: {}",
        cli.schema.display()
    ))?;

    // Parse the schema
    let schema_value: serde_json::Value =
        serde_json::from_str(&schema_content).context("Failed to parse JSON schema")?;

    // Compile the schema
    let compiled_schema = jsonschema::JSONSchema::compile(&schema_value)
        .map_err(|e| anyhow::anyhow!("Failed to compile JSON schema: {}", e))?;

    // Validate each YAML file
    let mut results = Vec::new();

    for yaml_file in &cli.yaml_files {
        let result = validate_yaml_file(yaml_file, &compiled_schema);
        results.push(result);
    }

    // Display per-file validation status
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

    // Print summary
    let total = results.len();
    let passed = results.iter().filter(|r| r.success).count();
    let failed = total - passed;

    println!();
    println!("{}Summary:{}", COLOR_BOLD, COLOR_RESET);
    println!("  Total files validated: {}", total);
    println!("  {}Passed: {}{}", COLOR_GREEN, passed, COLOR_RESET);
    println!("  {}Failed: {}{}", COLOR_RED, failed, COLOR_RESET);

    // Exit with appropriate code
    if failed > 0 {
        process::exit(1);
    }

    Ok(())
}

fn validate_yaml_file(
    yaml_file: &PathBuf,
    compiled_schema: &jsonschema::JSONSchema,
) -> ValidationResult {
    let mut result = ValidationResult {
        file_path: yaml_file.clone(),
        success: false,
        error_messages: Vec::new(),
    };

    // Read the YAML file
    let yaml_content = match fs::read_to_string(yaml_file) {
        Ok(content) => content,
        Err(e) => {
            result
                .error_messages
                .push(format!("Failed to read file: {}", e));
            return result;
        }
    };

    // Parse the YAML content
    let yaml_value: serde_yaml::Value = match serde_yaml::from_str(&yaml_content) {
        Ok(value) => value,
        Err(e) => {
            log_yaml_parse_error(&e, &yaml_content, &yaml_file.to_string_lossy());
            result
                .error_messages
                .push(format!("Failed to parse YAML: {}", e));
            return result;
        }
    };

    // Convert YAML to JSON Value for validation
    let json_value: serde_json::Value = match serde_json::to_value(&yaml_value) {
        Ok(value) => value,
        Err(e) => {
            result
                .error_messages
                .push(format!("Failed to convert YAML to JSON: {}", e));
            return result;
        }
    };

    // Validate
    if let Err(errors) = compiled_schema.validate(&json_value) {
        result
            .error_messages
            .push("Schema constraint violations:".to_string());

        for (idx, error) in errors.enumerate() {
            let path = if error.instance_path.to_string().is_empty() {
                "root".to_string()
            } else {
                error.instance_path.to_string()
            };

            result
                .error_messages
                .push(format!("  Error #{}: Path '{}'", idx + 1, path));
            result
                .error_messages
                .push(format!("    Constraint: {}", error));

            let instance = error.instance.as_ref();
            result
                .error_messages
                .push(format!("    Found value: {}", instance));
        }

        return result;
    }

    result.success = true;
    result
}
