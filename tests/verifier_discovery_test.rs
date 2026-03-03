use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

// Include the functions we need to test from the verifier binary
// Since these are private functions in a binary, we'll need to copy them here
// or make them available through a library. For now, we'll reimplement them
// with the same logic for testing purposes.

/// Discovers all execution log files in the given folder recursively
fn discover_log_files(folder_path: &PathBuf) -> anyhow::Result<Vec<PathBuf>> {
    let mut log_files = Vec::new();
    discover_log_files_recursive(folder_path, &mut log_files)?;
    Ok(log_files)
}

/// Recursively searches directories for execution log files
fn discover_log_files_recursive(dir: &PathBuf, log_files: &mut Vec<PathBuf>) -> anyhow::Result<()> {
    let entries = fs::read_dir(dir)
        .map_err(|e| anyhow::anyhow!("Failed to read directory: {}: {}", dir.display(), e))?;

    for entry in entries {
        let entry = entry.map_err(|e| anyhow::anyhow!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        // Get metadata to check file type without following symlinks
        let metadata = match fs::symlink_metadata(&path) {
            Ok(m) => m,
            Err(e) => {
                log::warn!("Failed to read metadata for '{}': {}", path.display(), e);
                continue;
            }
        };

        // Skip symlinks to avoid potential infinite loops
        if metadata.is_symlink() {
            log::debug!("Skipping symlink: {}", path.display());
            continue;
        }

        if metadata.is_dir() {
            // Recursively search subdirectories
            discover_log_files_recursive(&path, log_files)?;
        } else if metadata.is_file() {
            // Check if filename matches the pattern *_execution_log.json
            if let Some(file_name) = path.file_name() {
                let file_name_str = file_name.to_string_lossy();
                if file_name_str.ends_with("_execution_log.json") {
                    log_files.push(path);
                }
            }
        }
    }

    Ok(())
}

/// Extracts test case ID from execution log filename
fn extract_test_case_id_from_filename(log_path: &Path) -> String {
    // Expected format: {test_case_id}_execution_log.json
    log_path
        .file_stem()
        .and_then(|s| s.to_str())
        .and_then(|s| s.strip_suffix("_execution_log"))
        .unwrap_or("UNKNOWN")
        .to_string()
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Creates a dummy execution log file
fn create_execution_log(path: &Path) {
    fs::write(path, "[]").unwrap();
}

/// Creates a non-log file
fn create_regular_file(path: &Path, content: &str) {
    fs::write(path, content).unwrap();
}

// ============================================================================
// Tests for discover_log_files() - Basic Functionality
// ============================================================================

#[test]
fn test_discover_log_files_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let folder_path = temp_dir.path().to_path_buf();

    let result = discover_log_files(&folder_path).unwrap();
    assert!(
        result.is_empty(),
        "Should find no log files in empty directory"
    );
}

#[test]
fn test_discover_log_files_single_file() {
    let temp_dir = TempDir::new().unwrap();
    let folder_path = temp_dir.path().to_path_buf();
    let log_file = folder_path.join("TC001_execution_log.json");

    create_execution_log(&log_file);

    let result = discover_log_files(&folder_path).unwrap();
    assert_eq!(result.len(), 1, "Should find exactly one log file");
    assert_eq!(result[0], log_file);
}

#[test]
fn test_discover_log_files_multiple_files() {
    let temp_dir = TempDir::new().unwrap();
    let folder_path = temp_dir.path().to_path_buf();

    let log1 = folder_path.join("TC001_execution_log.json");
    let log2 = folder_path.join("TC002_execution_log.json");
    let log3 = folder_path.join("TC003_execution_log.json");

    create_execution_log(&log1);
    create_execution_log(&log2);
    create_execution_log(&log3);

    let result = discover_log_files(&folder_path).unwrap();
    assert_eq!(result.len(), 3, "Should find all three log files");
    assert!(result.contains(&log1));
    assert!(result.contains(&log2));
    assert!(result.contains(&log3));
}

#[test]
fn test_discover_log_files_ignores_non_matching_files() {
    let temp_dir = TempDir::new().unwrap();
    let folder_path = temp_dir.path().to_path_buf();

    let log_file = folder_path.join("TC001_execution_log.json");
    let other_file1 = folder_path.join("test.json");
    let other_file2 = folder_path.join("execution_log.json");
    let other_file3 = folder_path.join("TC001_log.json");
    let other_file4 = folder_path.join("TC001_execution_log.txt");

    create_execution_log(&log_file);
    create_regular_file(&other_file1, "{}");
    create_regular_file(&other_file2, "{}");
    create_regular_file(&other_file3, "{}");
    create_regular_file(&other_file4, "{}");

    let result = discover_log_files(&folder_path).unwrap();
    assert_eq!(result.len(), 1, "Should only find files matching pattern");
    assert_eq!(result[0], log_file);
}

// ============================================================================
// Tests for discover_log_files_recursive() - Nested Directory Structures
// ============================================================================

#[test]
fn test_discover_log_files_nested_directories() {
    let temp_dir = TempDir::new().unwrap();
    let folder_path = temp_dir.path().to_path_buf();

    // Create nested directory structure
    let level1 = folder_path.join("level1");
    let level2 = level1.join("level2");
    let level3 = level2.join("level3");

    fs::create_dir_all(&level3).unwrap();

    // Create log files at different levels
    let log_root = folder_path.join("root_execution_log.json");
    let log_level1 = level1.join("level1_execution_log.json");
    let log_level2 = level2.join("level2_execution_log.json");
    let log_level3 = level3.join("level3_execution_log.json");

    create_execution_log(&log_root);
    create_execution_log(&log_level1);
    create_execution_log(&log_level2);
    create_execution_log(&log_level3);

    let result = discover_log_files(&folder_path).unwrap();
    assert_eq!(
        result.len(),
        4,
        "Should find all log files in nested structure"
    );
    assert!(result.contains(&log_root));
    assert!(result.contains(&log_level1));
    assert!(result.contains(&log_level2));
    assert!(result.contains(&log_level3));
}

#[test]
fn test_discover_log_files_multiple_subdirectories() {
    let temp_dir = TempDir::new().unwrap();
    let folder_path = temp_dir.path().to_path_buf();

    // Create multiple subdirectories at same level
    let dir_a = folder_path.join("dir_a");
    let dir_b = folder_path.join("dir_b");
    let dir_c = folder_path.join("dir_c");

    fs::create_dir_all(&dir_a).unwrap();
    fs::create_dir_all(&dir_b).unwrap();
    fs::create_dir_all(&dir_c).unwrap();

    // Create log files in each directory
    let log_a = dir_a.join("TC_A_execution_log.json");
    let log_b = dir_b.join("TC_B_execution_log.json");
    let log_c = dir_c.join("TC_C_execution_log.json");

    create_execution_log(&log_a);
    create_execution_log(&log_b);
    create_execution_log(&log_c);

    let result = discover_log_files(&folder_path).unwrap();
    assert_eq!(
        result.len(),
        3,
        "Should find log files from all subdirectories"
    );
    assert!(result.contains(&log_a));
    assert!(result.contains(&log_b));
    assert!(result.contains(&log_c));
}

#[test]
fn test_discover_log_files_empty_subdirectories() {
    let temp_dir = TempDir::new().unwrap();
    let folder_path = temp_dir.path().to_path_buf();

    // Create empty subdirectories
    let dir1 = folder_path.join("empty1");
    let dir2 = folder_path.join("empty2");

    fs::create_dir_all(&dir1).unwrap();
    fs::create_dir_all(&dir2).unwrap();

    // Create log file only in root
    let log_file = folder_path.join("TC001_execution_log.json");
    create_execution_log(&log_file);

    let result = discover_log_files(&folder_path).unwrap();
    assert_eq!(result.len(), 1, "Should find only root log file");
    assert_eq!(result[0], log_file);
}

#[test]
fn test_discover_log_files_complex_nested_structure() {
    let temp_dir = TempDir::new().unwrap();
    let folder_path = temp_dir.path().to_path_buf();

    // Create complex nested structure
    let tests = folder_path.join("tests");
    let unit = tests.join("unit");
    let integration = tests.join("integration");
    let e2e = tests.join("e2e");
    let api = e2e.join("api");
    let ui = e2e.join("ui");

    fs::create_dir_all(&unit).unwrap();
    fs::create_dir_all(&integration).unwrap();
    fs::create_dir_all(&api).unwrap();
    fs::create_dir_all(&ui).unwrap();

    // Create log files at various locations
    let logs = vec![
        folder_path.join("root_execution_log.json"),
        tests.join("test_suite_execution_log.json"),
        unit.join("unit_test_execution_log.json"),
        integration.join("integration_test_execution_log.json"),
        api.join("api_test_execution_log.json"),
        ui.join("ui_test_execution_log.json"),
    ];

    for log in &logs {
        create_execution_log(log);
    }

    // Add some non-matching files
    create_regular_file(&unit.join("readme.txt"), "readme");
    create_regular_file(&api.join("config.json"), "{}");

    let result = discover_log_files(&folder_path).unwrap();
    assert_eq!(
        result.len(),
        6,
        "Should find all log files in complex structure"
    );
    for log in &logs {
        assert!(result.contains(log), "Should contain {}", log.display());
    }
}

// ============================================================================
// Tests for Mixed File Types
// ============================================================================

#[test]
fn test_discover_log_files_mixed_extensions() {
    let temp_dir = TempDir::new().unwrap();
    let folder_path = temp_dir.path().to_path_buf();

    let log_file = folder_path.join("TC001_execution_log.json");
    let json_file = folder_path.join("config.json");
    let txt_file = folder_path.join("readme.txt");
    let yaml_file = folder_path.join("test.yaml");
    let xml_file = folder_path.join("data.xml");

    create_execution_log(&log_file);
    create_regular_file(&json_file, "{}");
    create_regular_file(&txt_file, "text");
    create_regular_file(&yaml_file, "key: value");
    create_regular_file(&xml_file, "<root/>");

    let result = discover_log_files(&folder_path).unwrap();
    assert_eq!(result.len(), 1, "Should only find execution log files");
    assert_eq!(result[0], log_file);
}

#[test]
fn test_discover_log_files_similar_filenames() {
    let temp_dir = TempDir::new().unwrap();
    let folder_path = temp_dir.path().to_path_buf();

    // Create files with similar but not matching names
    let valid_log = folder_path.join("TC001_execution_log.json");
    let missing_extension = folder_path.join("TC002_execution_log");
    let wrong_extension = folder_path.join("TC003_execution_log.txt");
    let missing_suffix = folder_path.join("TC004_log.json");
    let extra_text = folder_path.join("TC005_execution_log.json.bak");

    create_execution_log(&valid_log);
    create_regular_file(&missing_extension, "[]");
    create_regular_file(&wrong_extension, "[]");
    create_regular_file(&missing_suffix, "[]");
    create_regular_file(&extra_text, "[]");

    let result = discover_log_files(&folder_path).unwrap();
    assert_eq!(result.len(), 1, "Should only match exact pattern");
    assert_eq!(result[0], valid_log);
}

#[test]
fn test_discover_log_files_hidden_files() {
    let temp_dir = TempDir::new().unwrap();
    let folder_path = temp_dir.path().to_path_buf();

    // Create hidden file (starting with dot on Unix)
    let visible_log = folder_path.join("TC001_execution_log.json");
    let hidden_log = folder_path.join(".hidden_execution_log.json");

    create_execution_log(&visible_log);
    create_execution_log(&hidden_log);

    let result = discover_log_files(&folder_path).unwrap();

    // Both should be found as the pattern doesn't exclude hidden files
    assert_eq!(
        result.len(),
        2,
        "Should find both visible and hidden log files"
    );
    assert!(result.contains(&visible_log));
    assert!(result.contains(&hidden_log));
}

#[test]
fn test_discover_log_files_no_extension_files() {
    let temp_dir = TempDir::new().unwrap();
    let folder_path = temp_dir.path().to_path_buf();

    let log_file = folder_path.join("TC001_execution_log.json");
    let no_ext_file = folder_path.join("README");
    let no_ext_file2 = folder_path.join("Makefile");

    create_execution_log(&log_file);
    create_regular_file(&no_ext_file, "readme");
    create_regular_file(&no_ext_file2, "makefile");

    let result = discover_log_files(&folder_path).unwrap();
    assert_eq!(
        result.len(),
        1,
        "Should ignore files without proper extension"
    );
    assert_eq!(result[0], log_file);
}

// ============================================================================
// Tests for Edge Cases
// ============================================================================

#[test]
fn test_discover_log_files_very_long_filename() {
    let temp_dir = TempDir::new().unwrap();
    let folder_path = temp_dir.path().to_path_buf();

    // Create file with very long test case ID
    let long_id = "A".repeat(200);
    let log_file = folder_path.join(format!("{}_execution_log.json", long_id));

    create_execution_log(&log_file);

    let result = discover_log_files(&folder_path).unwrap();
    assert_eq!(result.len(), 1, "Should handle very long filenames");
    assert_eq!(result[0], log_file);
}

#[test]
fn test_discover_log_files_special_characters_in_filename() {
    let temp_dir = TempDir::new().unwrap();
    let folder_path = temp_dir.path().to_path_buf();

    // Create files with special characters
    let log1 = folder_path.join("TC-001_execution_log.json");
    let log2 = folder_path.join("TC_001_v2_execution_log.json");
    let log3 = folder_path.join("test.case.001_execution_log.json");

    create_execution_log(&log1);
    create_execution_log(&log2);
    create_execution_log(&log3);

    let result = discover_log_files(&folder_path).unwrap();
    assert_eq!(
        result.len(),
        3,
        "Should handle special characters in filenames"
    );
    assert!(result.contains(&log1));
    assert!(result.contains(&log2));
    assert!(result.contains(&log3));
}

#[test]
fn test_discover_log_files_unicode_in_filename() {
    let temp_dir = TempDir::new().unwrap();
    let folder_path = temp_dir.path().to_path_buf();

    // Create files with unicode characters
    let log1 = folder_path.join("テスト_execution_log.json");
    let log2 = folder_path.join("测试_execution_log.json");
    let log3 = folder_path.join("тест_execution_log.json");

    create_execution_log(&log1);
    create_execution_log(&log2);
    create_execution_log(&log3);

    let result = discover_log_files(&folder_path).unwrap();
    assert_eq!(
        result.len(),
        3,
        "Should handle unicode characters in filenames"
    );
    assert!(result.contains(&log1));
    assert!(result.contains(&log2));
    assert!(result.contains(&log3));
}

#[test]
fn test_discover_log_files_whitespace_in_filename() {
    let temp_dir = TempDir::new().unwrap();
    let folder_path = temp_dir.path().to_path_buf();

    // Create files with whitespace
    let log1 = folder_path.join("TC 001_execution_log.json");
    let log2 = folder_path.join("Test Case 002_execution_log.json");

    create_execution_log(&log1);
    create_execution_log(&log2);

    let result = discover_log_files(&folder_path).unwrap();
    assert_eq!(result.len(), 2, "Should handle whitespace in filenames");
    assert!(result.contains(&log1));
    assert!(result.contains(&log2));
}

#[test]
#[cfg(unix)]
fn test_discover_log_files_symlinks() {
    use std::os::unix::fs::symlink;

    let temp_dir = TempDir::new().unwrap();
    let folder_path = temp_dir.path().to_path_buf();

    // Create a real log file
    let real_log = folder_path.join("real_execution_log.json");
    create_execution_log(&real_log);

    // Create a symlink to a log file
    let symlink_log = folder_path.join("symlink_execution_log.json");
    symlink(&real_log, &symlink_log).unwrap();

    // Create a subdirectory with a log file
    let subdir = folder_path.join("subdir");
    fs::create_dir_all(&subdir).unwrap();
    let subdir_log = subdir.join("subdir_execution_log.json");
    create_execution_log(&subdir_log);

    // Create a symlink to the subdirectory
    let symlink_dir = folder_path.join("symlink_dir");
    symlink(&subdir, &symlink_dir).unwrap();

    let result = discover_log_files(&folder_path).unwrap();

    // Should find real_log and subdir_log, but skip symlinks
    assert!(
        result.len() == 2,
        "Should skip symlinks but find real files, found {} files",
        result.len()
    );
    assert!(result.contains(&real_log));
    assert!(result.contains(&subdir_log));
    assert!(
        !result.contains(&symlink_log),
        "Should not include symlinked files"
    );
}

#[test]
fn test_discover_log_files_case_sensitivity() {
    let temp_dir = TempDir::new().unwrap();
    let folder_path = temp_dir.path().to_path_buf();

    // Create files with different case variations
    let log1 = folder_path.join("TC001_execution_log.json");
    let log2 = folder_path.join("TC002_Execution_Log.json"); // Different case
    let log3 = folder_path.join("TC003_EXECUTION_LOG.json"); // All caps

    create_execution_log(&log1);
    create_execution_log(&log2);
    create_execution_log(&log3);

    let result = discover_log_files(&folder_path).unwrap();

    // Only exact match should be found (case-sensitive)
    assert_eq!(result.len(), 1, "Should be case-sensitive");
    assert_eq!(result[0], log1);
}

// ============================================================================
// Tests for extract_test_case_id_from_filename() - Basic Cases
// ============================================================================

#[test]
fn test_extract_test_case_id_simple() {
    let path = Path::new("TC001_execution_log.json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(id, "TC001");
}

#[test]
fn test_extract_test_case_id_numeric() {
    let path = Path::new("12345_execution_log.json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(id, "12345");
}

#[test]
fn test_extract_test_case_id_alphanumeric() {
    let path = Path::new("test_case_001_execution_log.json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(id, "test_case_001");
}

#[test]
fn test_extract_test_case_id_with_path() {
    let path = Path::new("/some/path/to/logs/TC001_execution_log.json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(id, "TC001");
}

#[test]
fn test_extract_test_case_id_with_relative_path() {
    let path = Path::new("./logs/nested/TC001_execution_log.json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(id, "TC001");
}

// ============================================================================
// Tests for extract_test_case_id_from_filename() - Special Characters
// ============================================================================

#[test]
fn test_extract_test_case_id_with_hyphens() {
    let path = Path::new("TC-001-ALPHA_execution_log.json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(id, "TC-001-ALPHA");
}

#[test]
fn test_extract_test_case_id_with_dots() {
    let path = Path::new("test.case.001_execution_log.json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(id, "test.case.001");
}

#[test]
fn test_extract_test_case_id_with_underscores() {
    let path = Path::new("test_case_with_underscores_execution_log.json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(id, "test_case_with_underscores");
}

#[test]
fn test_extract_test_case_id_mixed_special_chars() {
    let path = Path::new("TC-001.v2_beta_execution_log.json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(id, "TC-001.v2_beta");
}

#[test]
fn test_extract_test_case_id_with_spaces() {
    let path = Path::new("TC 001 Alpha_execution_log.json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(id, "TC 001 Alpha");
}

// ============================================================================
// Tests for extract_test_case_id_from_filename() - Unicode
// ============================================================================

#[test]
fn test_extract_test_case_id_unicode_japanese() {
    let path = Path::new("テストケース001_execution_log.json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(id, "テストケース001");
}

#[test]
fn test_extract_test_case_id_unicode_chinese() {
    let path = Path::new("测试用例001_execution_log.json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(id, "测试用例001");
}

#[test]
fn test_extract_test_case_id_unicode_cyrillic() {
    let path = Path::new("тестовый_случай_001_execution_log.json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(id, "тестовый_случай_001");
}

#[test]
fn test_extract_test_case_id_unicode_emoji() {
    let path = Path::new("test_🚀_001_execution_log.json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(id, "test_🚀_001");
}

// ============================================================================
// Tests for extract_test_case_id_from_filename() - Edge Cases
// ============================================================================

#[test]
fn test_extract_test_case_id_empty_id() {
    let path = Path::new("_execution_log.json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(id, "", "Empty ID should return empty string");
}

#[test]
fn test_extract_test_case_id_very_long() {
    let long_id = "A".repeat(300);
    let filename = format!("{}_execution_log.json", long_id);
    let path = Path::new(&filename);
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(id, long_id, "Should handle very long IDs");
}

#[test]
fn test_extract_test_case_id_missing_suffix() {
    let path = Path::new("TC001_log.json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(
        id, "UNKNOWN",
        "Should return UNKNOWN for non-matching pattern"
    );
}

#[test]
fn test_extract_test_case_id_missing_extension() {
    let path = Path::new("TC001_execution_log");
    let id = extract_test_case_id_from_filename(path);
    // The function extracts ID from file stem, regardless of extension presence
    assert_eq!(
        id, "TC001",
        "Should extract ID even without .json extension"
    );
}

#[test]
fn test_extract_test_case_id_wrong_extension() {
    let path = Path::new("TC001_execution_log.txt");
    let id = extract_test_case_id_from_filename(path);
    // The function extracts ID from file stem, regardless of extension type
    assert_eq!(id, "TC001", "Should extract ID even with wrong extension");
}

#[test]
fn test_extract_test_case_id_no_file_stem() {
    // This is a theoretical edge case - a file with only extension
    let path = Path::new(".json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(id, "UNKNOWN", "Should return UNKNOWN for file with no stem");
}

#[test]
fn test_extract_test_case_id_multiple_underscores() {
    let path = Path::new("TC_001_v2_final_execution_log.json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(id, "TC_001_v2_final", "Should handle multiple underscores");
}

#[test]
fn test_extract_test_case_id_suffix_appears_twice() {
    let path = Path::new("my_execution_log_test_execution_log.json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(
        id, "my_execution_log_test",
        "Should only strip last occurrence"
    );
}

#[test]
fn test_extract_test_case_id_case_sensitive_suffix() {
    let path = Path::new("TC001_EXECUTION_LOG.json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(id, "UNKNOWN", "Suffix matching should be case-sensitive");
}

#[test]
fn test_extract_test_case_id_uppercase_id() {
    let path = Path::new("TESTCASE001_execution_log.json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(id, "TESTCASE001");
}

#[test]
fn test_extract_test_case_id_lowercase_id() {
    let path = Path::new("testcase001_execution_log.json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(id, "testcase001");
}

#[test]
fn test_extract_test_case_id_mixed_case_id() {
    let path = Path::new("TestCase001_execution_log.json");
    let id = extract_test_case_id_from_filename(path);
    assert_eq!(id, "TestCase001");
}

// ============================================================================
// Tests for Error Handling
// ============================================================================

#[test]
fn test_discover_log_files_nonexistent_directory() {
    let folder_path = PathBuf::from("/nonexistent/directory/that/does/not/exist");
    let result = discover_log_files(&folder_path);
    assert!(
        result.is_err(),
        "Should return error for nonexistent directory"
    );
}

#[test]
fn test_discover_log_files_file_instead_of_directory() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("file.txt");
    create_regular_file(&file_path, "content");

    let result = discover_log_files(&file_path);
    assert!(result.is_err(), "Should return error when path is a file");
}

// ============================================================================
// Integration Tests - Complex Scenarios
// ============================================================================

#[test]
fn test_integration_realistic_test_suite_structure() {
    let temp_dir = TempDir::new().unwrap();
    let folder_path = temp_dir.path().to_path_buf();

    // Create realistic test suite structure
    let unit_tests = folder_path.join("unit");
    let integration_tests = folder_path.join("integration");
    let e2e_tests = folder_path.join("e2e");
    let smoke_tests = e2e_tests.join("smoke");
    let regression_tests = e2e_tests.join("regression");

    fs::create_dir_all(&unit_tests).unwrap();
    fs::create_dir_all(&integration_tests).unwrap();
    fs::create_dir_all(&smoke_tests).unwrap();
    fs::create_dir_all(&regression_tests).unwrap();

    // Create various log files
    let logs = vec![
        unit_tests.join("unit_test_001_execution_log.json"),
        unit_tests.join("unit_test_002_execution_log.json"),
        integration_tests.join("integration_test_001_execution_log.json"),
        smoke_tests.join("smoke_test_001_execution_log.json"),
        smoke_tests.join("smoke_test_002_execution_log.json"),
        regression_tests.join("regression_001_execution_log.json"),
    ];

    for log in &logs {
        create_execution_log(log);
    }

    // Add non-log files
    create_regular_file(&unit_tests.join("test_config.json"), "{}");
    create_regular_file(&smoke_tests.join("README.md"), "# Tests");
    create_regular_file(&regression_tests.join("data.csv"), "a,b,c");

    let result = discover_log_files(&folder_path).unwrap();
    assert_eq!(result.len(), 6, "Should find all 6 execution log files");
    for log in &logs {
        assert!(result.contains(log), "Should contain {}", log.display());
    }
}

#[test]
fn test_integration_extract_ids_from_discovered_files() {
    let temp_dir = TempDir::new().unwrap();
    let folder_path = temp_dir.path().to_path_buf();

    // Create log files with various ID formats
    let logs_and_ids = vec![
        ("TC-001_execution_log.json", "TC-001"),
        ("test.case.002_execution_log.json", "test.case.002"),
        ("simple_execution_log.json", "simple"),
        ("UPPER_CASE_ID_execution_log.json", "UPPER_CASE_ID"),
    ];

    for (filename, _) in &logs_and_ids {
        let log_path = folder_path.join(filename);
        create_execution_log(&log_path);
    }

    let discovered = discover_log_files(&folder_path).unwrap();
    assert_eq!(discovered.len(), 4, "Should discover all 4 log files");

    // Extract IDs from all discovered files
    for log_path in &discovered {
        let id = extract_test_case_id_from_filename(log_path);

        // Find expected ID for this filename
        let filename = log_path.file_name().unwrap().to_str().unwrap();
        let expected_id = logs_and_ids
            .iter()
            .find(|(name, _)| name == &filename)
            .map(|(_, id)| id)
            .unwrap();

        assert_eq!(
            id, *expected_id,
            "Extracted ID should match expected for {}",
            filename
        );
    }
}

#[test]
fn test_integration_deep_nesting_with_mixed_content() {
    let temp_dir = TempDir::new().unwrap();
    let folder_path = temp_dir.path().to_path_buf();

    // Create deeply nested structure with mixed content
    let path1 = folder_path.join("a/b/c/d/e");
    let path2 = folder_path.join("x/y/z");
    let path3 = folder_path.join("1/2/3/4");

    fs::create_dir_all(&path1).unwrap();
    fs::create_dir_all(&path2).unwrap();
    fs::create_dir_all(&path3).unwrap();

    // Add execution logs at various depths
    let log1 = folder_path.join("root_execution_log.json");
    let log2 = folder_path.join("a/b/level2_execution_log.json");
    let log3 = path1.join("deep_execution_log.json");
    let log4 = path2.join("mid_execution_log.json");
    let log5 = path3.join("numeric_path_execution_log.json");

    create_execution_log(&log1);
    create_execution_log(&log2);
    create_execution_log(&log3);
    create_execution_log(&log4);
    create_execution_log(&log5);

    // Add various non-log files at different levels
    create_regular_file(&folder_path.join("README.md"), "readme");
    create_regular_file(&folder_path.join("a/config.json"), "{}");
    create_regular_file(&path1.join("data.txt"), "data");
    create_regular_file(&path2.join("test.log"), "log");

    let result = discover_log_files(&folder_path).unwrap();
    assert_eq!(result.len(), 5, "Should find exactly 5 execution log files");

    let ids: Vec<String> = result
        .iter()
        .map(|p| extract_test_case_id_from_filename(p))
        .collect();

    assert!(ids.contains(&"root".to_string()));
    assert!(ids.contains(&"level2".to_string()));
    assert!(ids.contains(&"deep".to_string()));
    assert!(ids.contains(&"mid".to_string()));
    assert!(ids.contains(&"numeric_path".to_string()));
}
