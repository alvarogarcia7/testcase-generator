use anyhow::Result;
use testcase_manager::{print_title, TestCaseBuilder, TitleStyle};

fn main() -> Result<()> {
    let temp_dir = std::env::temp_dir().join("testcase_example");
    std::fs::create_dir_all(&temp_dir)?;

    print_title(
        "Example: Interactive Test Case Creation Workflow",
        TitleStyle::SimpleEquals,
    );
    println!("This example demonstrates the interactive workflow for creating test cases");
    println!("with sequences and steps, including git commits at each stage.\n");
    println!("Working directory: {}\n", temp_dir.display());

    let mut builder = TestCaseBuilder::new(&temp_dir)?;

    println!("Creating a test case with metadata...\n");

    builder
        .add_field(
            "requirement".to_string(),
            serde_yaml::Value::String("XXX100".to_string()),
        )?
        .add_field("item".to_string(), serde_yaml::Value::Number(1.into()))?
        .add_field("tc".to_string(), serde_yaml::Value::Number(4.into()))?
        .add_field(
            "id".to_string(),
            serde_yaml::Value::String("test_001".to_string()),
        )?
        .add_field(
            "description".to_string(),
            serde_yaml::Value::String("Example test case".to_string()),
        )?;

    println!("✓ Metadata added\n");

    let mut general_ic = serde_yaml::Mapping::new();
    general_ic.insert(
        serde_yaml::Value::String("eUICC".to_string()),
        serde_yaml::Value::Sequence(vec![serde_yaml::Value::String(
            "General condition 1".to_string(),
        )]),
    );
    let general_ic_value =
        serde_yaml::Value::Sequence(vec![serde_yaml::Value::Mapping(general_ic)]);

    builder.add_field("general_initial_conditions".to_string(), general_ic_value)?;

    println!("✓ General initial conditions added\n");

    let mut ic = serde_yaml::Mapping::new();
    ic.insert(
        serde_yaml::Value::String("eUICC".to_string()),
        serde_yaml::Value::Sequence(vec![
            serde_yaml::Value::String("Condition 1".to_string()),
            serde_yaml::Value::String("Condition 2".to_string()),
        ]),
    );
    builder.add_field(
        "initial_conditions".to_string(),
        serde_yaml::Value::Mapping(ic),
    )?;

    println!("✓ Initial conditions added\n");

    let mut seq_map = serde_yaml::Mapping::new();
    seq_map.insert(
        serde_yaml::Value::String("id".to_string()),
        serde_yaml::Value::Number(1.into()),
    );
    seq_map.insert(
        serde_yaml::Value::String("name".to_string()),
        serde_yaml::Value::String("Test Sequence #1".to_string()),
    );
    seq_map.insert(
        serde_yaml::Value::String("description".to_string()),
        serde_yaml::Value::String("Example test sequence".to_string()),
    );
    seq_map.insert(
        serde_yaml::Value::String("steps".to_string()),
        serde_yaml::Value::Sequence(Vec::new()),
    );

    builder.validate_and_append_sequence(serde_yaml::Value::Mapping(seq_map))?;

    println!("✓ Test sequence added\n");

    let mut expected = serde_yaml::Mapping::new();
    expected.insert(
        serde_yaml::Value::String("success".to_string()),
        serde_yaml::Value::Bool(true),
    );
    expected.insert(
        serde_yaml::Value::String("result".to_string()),
        serde_yaml::Value::String("SW=0x9000".to_string()),
    );
    expected.insert(
        serde_yaml::Value::String("output".to_string()),
        serde_yaml::Value::String("Operation successful".to_string()),
    );

    let step = builder.create_step_value(
        1,
        Some(true),
        "Execute command".to_string(),
        "ssh".to_string(),
        serde_yaml::Value::Mapping(expected),
    )?;

    builder.validate_and_append_step(0, step)?;

    println!("✓ Step 1 added to sequence\n");

    let mut expected2 = serde_yaml::Mapping::new();
    expected2.insert(
        serde_yaml::Value::String("result".to_string()),
        serde_yaml::Value::String("SW=0x9000".to_string()),
    );
    expected2.insert(
        serde_yaml::Value::String("output".to_string()),
        serde_yaml::Value::String("Verification successful".to_string()),
    );

    let step2 = builder.create_step_value(
        2,
        None,
        "Verify results".to_string(),
        "ssh".to_string(),
        serde_yaml::Value::Mapping(expected2),
    )?;

    builder.validate_and_append_step(0, step2)?;

    println!("✓ Step 2 added to sequence\n");

    let file_path = builder.save()?;

    println!("✓ Test case saved to: {}\n", file_path.display());
    println!("Example completed successfully!");

    let yaml_content = builder.to_yaml_string()?;
    println!("\nGenerated YAML:\n{}", yaml_content);

    Ok(())
}
