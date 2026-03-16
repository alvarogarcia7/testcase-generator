use anyhow::Result;
use jsonschema::JSONSchema;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

/// Helper function to load and compile schema
fn load_schema() -> Result<(Value, JSONSchema)> {
    let schema_path = PathBuf::from("data/testcase_results_container/schema.json");
    let schema_content = fs::read_to_string(&schema_path)?;
    let schema_json: Value = serde_json::from_str(&schema_content)?;
    let compiled_schema = JSONSchema::options()
        .compile(&schema_json)
        .map_err(|e| anyhow::anyhow!("Failed to compile schema: {}", e))?;
    Ok((schema_json, compiled_schema))
}

/// Test that the container schema file is valid JSON Schema
#[test]
fn test_schema_is_valid_json_schema() -> Result<()> {
    let schema_path = PathBuf::from("data/testcase_results_container/schema.json");
    assert!(
        schema_path.exists(),
        "Schema file should exist at {}",
        schema_path.display()
    );

    let (_schema_json, _compiled_schema) = load_schema()?;
    // If we got here, schema is valid
    Ok(())
}

/// Test that the sample data file validates against the schema
#[test]
fn test_sample_data_validates() -> Result<()> {
    let sample_path = PathBuf::from("data/testcase_results_container/data_sample.yml");

    assert!(
        sample_path.exists(),
        "Sample data file should exist at {}",
        sample_path.display()
    );

    // Load schema
    let (_schema_json, compiled_schema) = load_schema()?;

    // Load sample data (YAML)
    let sample_content = fs::read_to_string(&sample_path)?;
    let sample_data: Value = serde_yaml::from_str(&sample_content)?;

    // Validate
    let result = compiled_schema.validate(&sample_data);
    if let Err(errors) = result {
        let error_messages: Vec<String> = errors
            .map(|e| format!("  - {} at {}", e, e.instance_path))
            .collect();
        panic!(
            "Sample data validation failed:\n{}",
            error_messages.join("\n")
        );
    }

    Ok(())
}

/// Test that a minimal valid container report validates
#[test]
fn test_minimal_valid_container_validates() -> Result<()> {
    // Load schema
    let (_schema_json, compiled_schema) = load_schema()?;

    // Create minimal valid container
    let minimal_container = serde_json::json!({
        "title": "Test Report",
        "project": "Test Project",
        "test_date": "2024-03-16T12:00:00Z",
        "test_results": [],
        "metadata": {
            "execution_duration": 0.0,
            "total_test_cases": 0,
            "passed_test_cases": 0,
            "failed_test_cases": 0
        }
    });

    // Validate
    let result = compiled_schema.validate(&minimal_container);
    assert!(
        result.is_ok(),
        "Minimal valid container should pass validation"
    );

    Ok(())
}

/// Test that missing required fields are detected
#[test]
fn test_missing_required_fields_detected() -> Result<()> {
    // Load schema
    let (_schema_json, compiled_schema) = load_schema()?;

    // Create invalid container (missing "metadata")
    let invalid_container = serde_json::json!({
        "title": "Test Report",
        "project": "Test Project",
        "test_date": "2024-03-16T12:00:00Z",
        "test_results": []
    });

    // Validate - should fail
    let result = compiled_schema.validate(&invalid_container);
    assert!(
        result.is_err(),
        "Container missing 'metadata' should fail validation"
    );

    Ok(())
}

/// Test that invalid step status values are detected
#[test]
fn test_invalid_step_status_detected() -> Result<()> {
    // Load schema
    let (_schema_json, compiled_schema) = load_schema()?;

    // Create container with invalid step status
    let invalid_container = serde_json::json!({
        "title": "Test Report",
        "project": "Test Project",
        "test_date": "2024-03-16T12:00:00Z",
        "test_results": [{
            "test_case_id": "TC001",
            "description": "Test",
            "sequences": [{
                "sequence_id": 1,
                "name": "Seq1",
                "step_results": [{
                    "status": "invalid_status",  // Invalid!
                    "step": 1,
                    "description": "Step 1"
                }],
                "all_steps_passed": true
            }],
            "total_steps": 1,
            "passed_steps": 1,
            "failed_steps": 0,
            "not_executed_steps": 0,
            "overall_pass": true
        }],
        "metadata": {
            "execution_duration": 0.0,
            "total_test_cases": 1,
            "passed_test_cases": 1,
            "failed_test_cases": 0
        }
    });

    // Validate - should fail
    let result = compiled_schema.validate(&invalid_container);
    assert!(
        result.is_err(),
        "Container with invalid step status should fail validation"
    );

    Ok(())
}

/// Test that valid step statuses are accepted
#[test]
fn test_valid_step_statuses_accepted() -> Result<()> {
    // Load schema
    let (_schema_json, compiled_schema) = load_schema()?;

    // Test each valid status
    for status in &["pass", "fail", "not_executed"] {
        let mut step = serde_json::json!({
            "status": status,
            "step": 1,
            "description": "Test step"
        });

        // Add required fields for "fail" status
        if *status == "fail" {
            step["expected"] = serde_json::json!({
                "result": "SUCCESS",
                "output": "Expected output"
            });
            step["actual_result"] = serde_json::json!("FAILURE");
            step["actual_output"] = serde_json::json!("Actual output");
            step["reason"] = serde_json::json!("Test reason");
        }

        let container = serde_json::json!({
            "title": "Test Report",
            "project": "Test Project",
            "test_date": "2024-03-16T12:00:00Z",
            "test_results": [{
                "test_case_id": "TC001",
                "description": "Test",
                "sequences": [{
                    "sequence_id": 1,
                    "name": "Seq1",
                    "step_results": [step],
                    "all_steps_passed": *status == "pass"
                }],
                "total_steps": 1,
                "passed_steps": if *status == "pass" { 1 } else { 0 },
                "failed_steps": if *status == "fail" { 1 } else { 0 },
                "not_executed_steps": if *status == "not_executed" { 1 } else { 0 },
                "overall_pass": *status == "pass"
            }],
            "metadata": {
                "execution_duration": 0.0,
                "total_test_cases": 1,
                "passed_test_cases": if *status == "pass" { 1 } else { 0 },
                "failed_test_cases": if *status == "pass" { 0 } else { 1 }
            }
        });

        let result = compiled_schema.validate(&container);
        assert!(
            result.is_ok(),
            "Container with status '{}' should pass validation",
            status
        );
    }

    Ok(())
}

/// Test that string length constraints are enforced
#[test]
fn test_string_length_constraints() -> Result<()> {
    // Load schema
    let (_schema_json, compiled_schema) = load_schema()?;

    // Test title too long (max 200 chars)
    let title_too_long = "x".repeat(201);
    let invalid_container = serde_json::json!({
        "title": title_too_long,
        "project": "Test Project",
        "test_date": "2024-03-16T12:00:00Z",
        "test_results": [],
        "metadata": {
            "execution_duration": 0.0,
            "total_test_cases": 0,
            "passed_test_cases": 0,
            "failed_test_cases": 0
        }
    });

    let result = compiled_schema.validate(&invalid_container);
    assert!(
        result.is_err(),
        "Container with title > 200 chars should fail validation"
    );

    Ok(())
}

/// Test that numeric minimum constraints are enforced
#[test]
fn test_numeric_minimum_constraints() -> Result<()> {
    // Load schema
    let (_schema_json, compiled_schema) = load_schema()?;

    // Test negative execution_duration (minimum 0)
    let invalid_container = serde_json::json!({
        "title": "Test Report",
        "project": "Test Project",
        "test_date": "2024-03-16T12:00:00Z",
        "test_results": [],
        "metadata": {
            "execution_duration": -1.0,  // Invalid!
            "total_test_cases": 0,
            "passed_test_cases": 0,
            "failed_test_cases": 0
        }
    });

    let result = compiled_schema.validate(&invalid_container);
    assert!(
        result.is_err(),
        "Container with negative execution_duration should fail validation"
    );

    Ok(())
}

/// Test that optional fields can be omitted
#[test]
fn test_optional_fields_can_be_omitted() -> Result<()> {
    // Load schema
    let (_schema_json, compiled_schema) = load_schema()?;

    // Container without optional metadata fields
    let container_without_optionals = serde_json::json!({
        "title": "Test Report",
        "project": "Test Project",
        "test_date": "2024-03-16T12:00:00Z",
        "test_results": [],
        "metadata": {
            // No environment, platform, or executor
            "execution_duration": 0.0,
            "total_test_cases": 0,
            "passed_test_cases": 0,
            "failed_test_cases": 0
        }
    });

    let result = compiled_schema.validate(&container_without_optionals);
    assert!(
        result.is_ok(),
        "Container without optional metadata fields should pass validation"
    );

    Ok(())
}

/// Test that additional properties are not allowed at container level
#[test]
fn test_additional_properties_not_allowed() -> Result<()> {
    // Load schema
    let (_schema_json, compiled_schema) = load_schema()?;

    // Container with additional property
    let container_with_extra = serde_json::json!({
        "title": "Test Report",
        "project": "Test Project",
        "test_date": "2024-03-16T12:00:00Z",
        "test_results": [],
        "metadata": {
            "execution_duration": 0.0,
            "total_test_cases": 0,
            "passed_test_cases": 0,
            "failed_test_cases": 0
        },
        "extra_field": "not allowed"  // This should cause validation failure
    });

    let result = compiled_schema.validate(&container_with_extra);
    assert!(
        result.is_err(),
        "Container with additional properties should fail validation"
    );

    Ok(())
}
