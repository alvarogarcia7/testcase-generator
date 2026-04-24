use anyhow::Result;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_can_create_temp_dirs_and_files() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_cases_dir = temp_dir.path().join("testcases");
    fs::create_dir(&test_cases_dir)?;

    let test_file = test_cases_dir.join("test.yaml");
    fs::write(&test_file, "content")?;

    assert!(test_file.exists());
    assert_eq!(fs::read_to_string(&test_file)?, "content");

    Ok(())
}

#[test]
fn test_testcase_storage_can_load() -> Result<()> {
    use req_coverage::coverage::CoverageAnalyzer;

    let temp_dir = TempDir::new()?;
    let test_cases_dir = temp_dir.path().join("testcases");
    fs::create_dir(&test_cases_dir)?;

    // Just check we can create the analyzer
    let _analyzer = CoverageAnalyzer::new(&test_cases_dir)?;

    Ok(())
}
