use anyhow::Result;
use audit_traceability::{AuditTraceabilityLog, StageInfo, TestCaseAudit};
use std::fs;
use std::io::Write;

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("=== Audit Traceability Demo ===\n");

    let temp_dir = tempfile::tempdir()?;
    let base_path = temp_dir.path();

    println!("Creating test files in: {}\n", base_path.display());

    let tc1_yaml = base_path.join("TC001.yaml");
    fs::write(
        &tc1_yaml,
        r#"id: TC001
description: Test Case 001
requirement: REQ-001
item: 1
tc: 1
test_sequences:
  - id: 1
    name: Basic Test
    steps:
      - step: 1
        description: Run test
        command: echo "test"
        verification:
          result: "0"
          output: "test"
"#,
    )?;

    let tc1_script = base_path.join("TC001.sh");
    let mut script_file = fs::File::create(&tc1_script)?;
    script_file.write_all(
        b"#!/bin/bash\nset -euo pipefail\necho 'Test script for TC001'\necho 'test'\n",
    )?;
    drop(script_file);

    println!("✓ Created test files:");
    println!("  - {}", tc1_yaml.display());
    println!("  - {}", tc1_script.display());
    println!();

    println!("=== Step 1: Create Audit Log ===\n");

    let mut log = AuditTraceabilityLog::new("demo-witness".to_string());
    println!("✓ Created audit log with witness key: {}", log.witness_key);
    println!("  Timestamp: {}", log.date);
    println!();

    println!("=== Step 2: Add Test Case to Audit Log ===\n");

    let mut tc1_audit = TestCaseAudit::new();

    let initial_stage = StageInfo::from_file(&tc1_yaml)?;
    println!(
        "✓ Added 'initial' stage: {} (SHA-256: {}...)",
        tc1_yaml.display(),
        &initial_stage.sha256[..16]
    );
    tc1_audit.add_stage("initial", initial_stage);

    let script_stage = StageInfo::from_file(&tc1_script)?;
    println!(
        "✓ Added '05_shell_script' stage: {} (SHA-256: {}...)",
        tc1_script.display(),
        &script_stage.sha256[..16]
    );
    tc1_audit.add_stage("05_shell_script", script_stage);

    log.add_test_case("TC001", tc1_audit);
    println!("\n✓ Test case 'TC001' added to audit log");
    println!();

    println!("=== Step 3: Save Audit Log ===\n");

    let log_file = base_path.join("audit-traceability-log.json");
    log.save_to_file(&log_file)?;
    println!("✓ Audit log saved to: {}", log_file.display());
    println!();

    let log_content = fs::read_to_string(&log_file)?;
    println!("Audit log content:");
    println!("{}", log_content);
    println!();

    println!("=== Step 4: Load and Verify ===\n");

    let loaded_log = AuditTraceabilityLog::load_from_file(&log_file)?;
    println!("✓ Audit log loaded successfully");
    println!("  Test cases: {}", loaded_log.test_cases.keys().count());
    println!();

    let verification_result = loaded_log.verify_test_case("TC001")?;
    verification_result.print_summary();
    println!();

    println!("=== Step 5: Simulate File Modification ===\n");

    fs::write(&tc1_script, b"#!/bin/bash\necho 'Modified script'\n")?;
    println!("✓ Modified file: {}", tc1_script.display());
    println!();

    let verification_result_after_mod = loaded_log.verify_test_case("TC001")?;
    verification_result_after_mod.print_summary();
    println!();

    if !verification_result_after_mod.all_passed {
        println!("✓ Verification correctly detected the file modification!");
    }

    println!("=== Demo Complete ===");

    Ok(())
}
