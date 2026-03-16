use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use testcase_manager::models::TestCase;
use testcase_manager::validate_cross_file_dependencies;
use testcase_manager::yaml_utils::log_yaml_parse_error;

#[cfg(not(target_os = "windows"))]
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
#[cfg(not(target_os = "windows"))]
use std::collections::HashSet;
#[cfg(not(target_os = "windows"))]
use std::sync::mpsc::channel;
#[cfg(not(target_os = "windows"))]
use std::time::Duration;

#[derive(Parser)]
#[command(name = "validate-yaml")]
#[command(about = "Validate YAML payloads against a JSON schema", version)]
#[command(
    after_help = "ENVIRONMENT VARIABLES:\n    RUST_LOG    Set log level (trace, debug, info, warn, error). Overrides --log-level"
)]
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

    /// Set log level (trace, debug, info, warn, error)
    #[arg(long, value_name = "LEVEL", default_value = "warn")]
    log_level: String,

    /// Enable verbose output (equivalent to --log-level=info)
    #[arg(short, long)]
    verbose: bool,
}

struct ValidationResult {
    file_path: PathBuf,
    success: bool,
    error_messages: Vec<String>,
    test_case: Option<TestCase>,
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

fn validate_files(yaml_files: &[PathBuf], schema_path: &Path) -> Vec<ValidationResult> {
    let mut results = Vec::new();

    for yaml_file in yaml_files {
        let validation_result = validate_single_file(yaml_file, schema_path);

        let result = match validation_result {
            Ok(_) => {
                let test_case = parse_test_case(yaml_file);
                ValidationResult {
                    file_path: yaml_file.clone(),
                    success: true,
                    error_messages: Vec::new(),
                    test_case,
                }
            }
            Err(e) => ValidationResult {
                file_path: yaml_file.clone(),
                success: false,
                error_messages: e.to_string().lines().map(String::from).collect(),
                test_case: None,
            },
        };

        results.push(result);
    }

    results
}

fn parse_test_case(yaml_file: &Path) -> Option<TestCase> {
    let yaml_content = fs::read_to_string(yaml_file).ok()?;
    serde_yaml::from_str(&yaml_content).ok()
}

fn validate_dependencies(results: &[ValidationResult]) -> Result<(), Vec<String>> {
    let successful_files: Vec<(PathBuf, TestCase)> = results
        .iter()
        .filter_map(|r| {
            if r.success {
                r.test_case
                    .as_ref()
                    .map(|tc| (r.file_path.clone(), tc.clone()))
            } else {
                None
            }
        })
        .collect();

    if successful_files.len() <= 1 {
        return Ok(());
    }

    match validate_cross_file_dependencies(&successful_files) {
        Ok(()) => Ok(()),
        Err(errors) => {
            let error_messages: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
            Err(error_messages)
        }
    }
}

fn print_results(results: &[ValidationResult]) {
    for result in results {
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
}

fn print_summary(results: &[ValidationResult], dependency_errors: Option<&Vec<String>>) {
    let total = results.len();
    let passed = results.iter().filter(|r| r.success).count();
    let failed = total - passed;

    println!();
    println!("{}Summary:{}", COLOR_BOLD, COLOR_RESET);
    println!("  Total files validated: {}", total);
    println!("  {}Passed: {}{}", COLOR_GREEN, passed, COLOR_RESET);
    println!("  {}Failed: {}{}", COLOR_RED, failed, COLOR_RESET);

    if let Some(errors) = dependency_errors {
        println!();
        println!(
            "{}{}Dependency Validation:{}",
            COLOR_BOLD, COLOR_RED, COLOR_RESET
        );
        for error in errors {
            println!("  {}", error);
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn discover_schema_dependencies(schema_path: &Path) -> Result<HashSet<PathBuf>> {
    let mut schemas = HashSet::new();
    let mut to_process = vec![schema_path.to_path_buf()];
    let mut processed = HashSet::new();

    while let Some(current_schema) = to_process.pop() {
        let canonical = current_schema.canonicalize().context(format!(
            "Failed to canonicalize schema path: {}",
            current_schema.display()
        ))?;

        if processed.contains(&canonical) {
            continue;
        }

        processed.insert(canonical.clone());
        schemas.insert(canonical.clone());

        // Read schema file and look for $ref references
        let content = fs::read_to_string(&current_schema).context(format!(
            "Failed to read schema file: {}",
            current_schema.display()
        ))?;

        let schema_value: serde_json::Value = serde_json::from_str(&content).context(format!(
            "Failed to parse schema JSON: {}",
            current_schema.display()
        ))?;

        // Find all $ref values in the schema
        find_external_refs(&schema_value, &current_schema, &mut to_process)?;
    }

    Ok(schemas)
}

#[cfg(not(target_os = "windows"))]
fn find_external_refs(
    value: &serde_json::Value,
    schema_path: &Path,
    to_process: &mut Vec<PathBuf>,
) -> Result<()> {
    match value {
        serde_json::Value::Object(map) => {
            for (key, val) in map {
                if key == "$ref" {
                    if let serde_json::Value::String(ref_str) = val {
                        // Only process external references (not internal #/definitions/...)
                        if !ref_str.starts_with('#') {
                            // Extract the file path from the reference
                            let ref_path = ref_str.split('#').next().unwrap_or(ref_str);

                            // Resolve relative to the current schema's directory
                            if let Some(parent) = schema_path.parent() {
                                let resolved = parent.join(ref_path);
                                if resolved.exists() {
                                    to_process.push(resolved);
                                }
                            }
                        }
                    }
                } else {
                    find_external_refs(val, schema_path, to_process)?;
                }
            }
        }
        serde_json::Value::Array(arr) => {
            for item in arr {
                find_external_refs(item, schema_path, to_process)?;
            }
        }
        _ => {}
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn run_watch_mode(yaml_files: Vec<PathBuf>, schema_path: PathBuf) -> Result<()> {
    const COLOR_BLUE: &str = "\x1b[34m";
    const COLOR_YELLOW: &str = "\x1b[33m";

    // Discover all schema dependencies
    let schema_files = discover_schema_dependencies(&schema_path)?;

    println!(
        "{}{}Watch mode enabled{}",
        COLOR_BOLD, COLOR_BLUE, COLOR_RESET
    );
    println!(
        "Monitoring {} YAML file(s) and {} schema file(s) for changes...\n",
        yaml_files.len(),
        schema_files.len()
    );

    println!("{}Initial validation:{}", COLOR_BOLD, COLOR_RESET);
    let results = validate_files(&yaml_files, &schema_path);
    print_results(&results);

    let dependency_errors = validate_dependencies(&results).err();
    print_summary(&results, dependency_errors.as_ref());
    println!();

    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        },
        Config::default(),
    )
    .context("Failed to create file watcher")?;

    // Watch YAML files
    for yaml_file in &yaml_files {
        let canonical_path = yaml_file.canonicalize().context(format!(
            "Failed to canonicalize path: {}",
            yaml_file.display()
        ))?;
        watcher
            .watch(&canonical_path, RecursiveMode::NonRecursive)
            .context(format!(
                "Failed to watch file: {}",
                canonical_path.display()
            ))?;
    }

    // Watch all schema files (including transitive dependencies)
    for schema_file in &schema_files {
        watcher
            .watch(schema_file, RecursiveMode::NonRecursive)
            .context(format!(
                "Failed to watch schema file: {}",
                schema_file.display()
            ))?;
    }

    let mut changed_files = HashSet::new();
    let mut schema_changed = false;
    let mut last_event_time = std::time::Instant::now();
    let debounce_duration = Duration::from_millis(300);

    loop {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(event) => {
                if matches!(event.kind, EventKind::Modify(_)) {
                    for path in event.paths {
                        // Check if it's a YAML file
                        let is_yaml = yaml_files.iter().any(|f| {
                            f.canonicalize()
                                .ok()
                                .as_ref()
                                .map(|p| p == &path)
                                .unwrap_or(false)
                        });

                        // Check if it's a schema file
                        let is_schema = schema_files.contains(&path);

                        if is_yaml {
                            changed_files.insert(path.clone());
                            last_event_time = std::time::Instant::now();
                        } else if is_schema {
                            schema_changed = true;
                            last_event_time = std::time::Instant::now();
                        }
                    }
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                if (schema_changed || !changed_files.is_empty())
                    && last_event_time.elapsed() >= debounce_duration
                {
                    println!(
                        "\n{}{}File changes detected:{}",
                        COLOR_BOLD, COLOR_YELLOW, COLOR_RESET
                    );

                    if schema_changed {
                        println!("  → Schema file(s) modified");
                    }

                    for changed_file in &changed_files {
                        println!("  → {}", changed_file.display());
                    }
                    println!();

                    // If schema changed, re-validate all YAML files
                    if schema_changed {
                        println!(
                            "{}Schema changed - re-validating all YAML files:{}",
                            COLOR_BOLD, COLOR_RESET
                        );
                        let full_results = validate_files(&yaml_files, &schema_path);
                        print_results(&full_results);
                        let dependency_errors = validate_dependencies(&full_results).err();
                        print_summary(&full_results, dependency_errors.as_ref());
                    } else {
                        // Only YAML files changed - validate changed files first
                        let changed_yaml_files: Vec<PathBuf> = yaml_files
                            .iter()
                            .filter(|f| {
                                f.canonicalize()
                                    .ok()
                                    .as_ref()
                                    .map(|p| changed_files.contains(p))
                                    .unwrap_or(false)
                            })
                            .cloned()
                            .collect();

                        println!("{}Validating changed files:{}", COLOR_BOLD, COLOR_RESET);
                        let changed_results = validate_files(&changed_yaml_files, &schema_path);
                        print_results(&changed_results);

                        let all_changed_passed = changed_results.iter().all(|r| r.success);

                        if all_changed_passed {
                            println!();
                            println!(
                                "{}All changed files passed! Running full validation...{}",
                                COLOR_BOLD, COLOR_RESET
                            );
                            println!();

                            let full_results = validate_files(&yaml_files, &schema_path);
                            print_results(&full_results);
                            let dependency_errors = validate_dependencies(&full_results).err();
                            print_summary(&full_results, dependency_errors.as_ref());
                        } else {
                            let passed = changed_results.iter().filter(|r| r.success).count();
                            let failed = changed_results.len() - passed;
                            println!();
                            println!("{}Changed files summary:{}", COLOR_BOLD, COLOR_RESET);
                            println!("  {}Passed: {}{}", COLOR_GREEN, passed, COLOR_RESET);
                            println!("  {}Failed: {}{}", COLOR_RED, failed, COLOR_RESET);
                        }
                    }

                    println!();
                    println!("{}Watching for changes...{}", COLOR_BOLD, COLOR_RESET);

                    changed_files.clear();
                    schema_changed = false;
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                return Err(anyhow::anyhow!("Watch channel disconnected"));
            }
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = if cli.verbose { "info" } else { &cli.log_level };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    #[cfg(not(target_os = "windows"))]
    if cli.watch {
        return run_watch_mode(cli.yaml_files, cli.schema);
    }

    let results = validate_files(&cli.yaml_files, &cli.schema);
    print_results(&results);

    let dependency_validation = validate_dependencies(&results);
    let dependency_errors = dependency_validation.as_ref().err();
    print_summary(&results, dependency_errors);

    let failed = results.iter().filter(|r| !r.success).count();
    if failed > 0 || dependency_validation.is_err() {
        process::exit(1);
    }

    Ok(())
}
