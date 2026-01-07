use anyhow::Result;
use testcase_manager::{Prompts, TestCaseBuilder, TestCaseMetadata};

/// Example demonstrating the interactive test case creation workflow
///
/// This example shows:
/// 1. Interactive prompts for test case metadata (requirement, item, tc, id, description)
/// 2. Validation of metadata against the schema
/// 3. Git commit after metadata is added
/// 4. General initial conditions flow with defaults, edit prompts, and validation
/// 5. Initial conditions flow with the same capabilities
///
/// Run with: cargo run --example interactive_workflow
fn main() -> Result<()> {
    println!("╔═══════════════════════════════════════════════╗");
    println!("║  Interactive Test Case Creation - Example    ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    let temp_dir = tempfile::tempdir()?;
    let mut builder = TestCaseBuilder::new(temp_dir.path())?;

    println!("This example demonstrates:");
    println!("1. Metadata prompts (requirement, item, tc, id, description)");
    println!("2. Metadata validation");
    println!("3. Git commit after metadata");
    println!("4. General initial conditions with editor");
    println!("5. Validation against schema\n");

    demo_metadata_prompts()?;
    demo_builder_workflow(&mut builder)?;

    println!("\n✓ Example completed successfully!");
    println!("\nTo use this in production, run:");
    println!("  cargo run -- create-interactive");

    Ok(())
}

fn demo_metadata_prompts() -> Result<()> {
    println!("\n--- Metadata Structure Demo ---\n");

    let metadata = TestCaseMetadata {
        requirement: "XXX100".to_string(),
        item: 1,
        tc: 4,
        id: "4.2.2.2.1 TC_eUICC_ES6.UpdateMetadata".to_string(),
        description: "Test case for ES6.UpdateMetadata".to_string(),
    };

    println!("Example metadata:");
    println!("  Requirement: {}", metadata.requirement);
    println!("  Item: {}", metadata.item);
    println!("  TC: {}", metadata.tc);
    println!("  ID: {}", metadata.id);
    println!("  Description: {}", metadata.description);

    let yaml_map = metadata.to_yaml();
    let yaml_value = serde_yaml::Value::Mapping(serde_yaml::Mapping::from_iter(
        yaml_map
            .into_iter()
            .map(|(k, v)| (serde_yaml::Value::String(k), v)),
    ));

    let yaml_str = serde_yaml::to_string(&yaml_value)?;
    println!("\nYAML output:");
    println!("{}", yaml_str);

    Ok(())
}

fn demo_builder_workflow(builder: &mut TestCaseBuilder) -> Result<()> {
    println!("\n--- Builder Workflow Demo ---\n");

    let metadata = TestCaseMetadata {
        requirement: "XXX100".to_string(),
        item: 1,
        tc: 4,
        id: "demo_test_case".to_string(),
        description: "Demo test case".to_string(),
    };

    let yaml_map = metadata.to_yaml();
    for (key, value) in yaml_map {
        builder.add_field(key, value)?;
    }

    println!("✓ Metadata added to structure");

    let general_conditions = serde_yaml::from_str(
        r#"
- eUICC:
    - "The profile PROFILE_OPERATIONAL1 is loaded"
"#,
    )?;

    builder.add_field("general_initial_conditions".to_string(), general_conditions)?;
    println!("✓ General initial conditions added");

    let initial_conditions = serde_yaml::from_str(
        r#"
eUICC:
  - "The PROFILE_OPERATIONAL1 is Enabled"
  - "The PROFILE_OPERATIONAL2 is Enabled"
"#,
    )?;

    builder.add_field("initial_conditions".to_string(), initial_conditions)?;
    println!("✓ Initial conditions added");

    let yaml_output = builder.to_yaml_string()?;
    println!("\nGenerated YAML structure:");
    println!("{}", yaml_output);

    let file_path = builder.save()?;
    println!("\n✓ Saved to: {}", file_path.display());

    Ok(())
}
