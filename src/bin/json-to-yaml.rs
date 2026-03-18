use anyhow::{anyhow, Result};
use clap::Parser;
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "json-to-yaml",
    about = "Convert JSON verification output to YAML result files"
)]
struct Args {
    /// Input JSON file path
    input: PathBuf,

    /// Output directory for YAML files
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Read JSON file
    let json_content =
        fs::read_to_string(&args.input).map_err(|e| anyhow!("Failed to read input file: {}", e))?;

    let data: Value =
        serde_json::from_str(&json_content).map_err(|e| anyhow!("Invalid JSON: {}", e))?;

    // Create output directory
    let output_dir = args.output.unwrap_or_else(|| PathBuf::from("."));
    fs::create_dir_all(&output_dir)
        .map_err(|e| anyhow!("Failed to create output directory: {}", e))?;

    // Extract test cases
    let test_cases =
        if let Some(test_cases_array) = data.get("test_cases").and_then(|v| v.as_array()) {
            // Batch report format
            test_cases_array.clone()
        } else if data.get("test_case_id").is_some() {
            // Single test case format
            vec![data.clone()]
        } else {
            return Err(anyhow!("No test_cases or test_case_id found in JSON"));
        };

    if test_cases.is_empty() {
        return Err(anyhow!("No test cases found in JSON"));
    }

    // Convert each test case to YAML
    for test_case in test_cases {
        let test_case_id = test_case
            .get("test_case_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Test case missing test_case_id"))?;

        // Create result with type: result
        let mut yaml_data = json!({
            "type": "result"
        });

        // Merge test case data
        if let Some(obj) = yaml_data.as_object_mut() {
            if let Some(test_obj) = test_case.as_object() {
                for (key, value) in test_obj {
                    obj.insert(key.clone(), value.clone());
                }
            }
        }

        // Convert to YAML
        let yaml_string = serde_yaml::to_string(&yaml_data)
            .map_err(|e| anyhow!("Failed to convert to YAML: {}", e))?;

        // Write YAML file
        let output_file = output_dir.join(format!("{}_result.yaml", test_case_id));
        fs::write(&output_file, yaml_string)
            .map_err(|e| anyhow!("Failed to write output file: {}", e))?;

        println!("Generated: {}", output_file.display());
    }

    Ok(())
}
