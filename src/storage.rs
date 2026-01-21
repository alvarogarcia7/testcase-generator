use crate::models::{FileValidationStatus, TestCase, TestCaseFileInfo, TestSuite};
use crate::validation::SchemaValidator;
use crate::yaml_utils::log_yaml_parse_error;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Supported YAML file extensions
const YAML_EXTENSIONS: &[&str] = &["yaml", "yml"];

/// Manages storage and retrieval of test cases
#[derive(Debug, Clone)]
pub struct TestCaseStorage {
    base_path: PathBuf,
}

impl TestCaseStorage {
    /// Create a new storage manager
    pub fn new<P: AsRef<Path>>(base_path: P) -> Result<Self> {
        let base_path = base_path.as_ref().to_path_buf();

        if !base_path.exists() {
            fs::create_dir_all(&base_path).context(format!(
                "Failed to create directory: {}",
                base_path.display()
            ))?;
        }

        Ok(Self { base_path })
    }

    /// Get the base path
    pub fn base_path(&self) -> &Path {
        &self.base_path
    }

    /// Save a test case to a YAML file
    pub fn save_test_case(&self, test_case: &TestCase) -> Result<PathBuf> {
        let file_name = format!("{}.{}", test_case.id, YAML_EXTENSIONS[0]);
        let file_path = self.base_path.join(&file_name);

        let yaml_content =
            serde_yaml::to_string(test_case).context("Failed to serialize test case to YAML")?;

        fs::write(&file_path, yaml_content)
            .context(format!("Failed to write file: {}", file_path.display()))?;

        Ok(file_path)
    }

    /// Resolve file path, checking existence and supporting substring matching
    fn resolve_file_path(&self, file_path: &Path) -> Result<PathBuf> {
        let full_path = if file_path.is_absolute() {
            file_path.to_path_buf()
        } else {
            // Check if file exists directly as provided (already includes base path)
            if file_path.exists() {
                file_path.to_path_buf()
            } else {
                let joined_path = self.base_path.join(file_path);

                // Verify if the joined path exists
                if joined_path.exists() {
                    return Ok(joined_path);
                }

                // If not found, try substring matching
                let file_path_str = file_path.to_string_lossy();
                let matching_files = self.find_files_by_substring(&file_path_str)?;

                if matching_files.len() == 1 {
                    log::info!(
                        "Choosing file '{}' matching substring '{}'",
                        matching_files[0].display(),
                        file_path_str
                    );
                    return Ok(matching_files[0].clone());
                } else if matching_files.len() > 1 {
                    anyhow::bail!(
                        "Multiple files match substring '{}': {}",
                        file_path_str,
                        matching_files
                            .iter()
                            .map(|p| p.display().to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }

                // File not found
                anyhow::bail!(
                    "File not found: {}. Tried paths: '{}' and '{}'",
                    file_path_str,
                    file_path.display(),
                    joined_path.display()
                );
            }
        };

        // Final verification that the resolved path exists
        if !full_path.exists() {
            anyhow::bail!("File not found: {}", full_path.display());
        }

        Ok(full_path)
    }

    /// Find files in the base directory that contain the given substring
    fn find_files_by_substring(&self, substring: &str) -> Result<Vec<PathBuf>> {
        let mut matching_files = Vec::new();

        if !self.base_path.exists() {
            return Ok(matching_files);
        }

        for entry in fs::read_dir(&self.base_path).context(format!(
            "Failed to read directory: {}",
            self.base_path.display()
        ))? {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if YAML_EXTENSIONS.contains(&ext.to_string_lossy().as_ref()) {
                        if let Some(file_name) = path.file_name() {
                            if file_name.to_string_lossy().contains(substring) {
                                matching_files.push(path);
                            }
                        }
                    }
                }
            }
        }

        Ok(matching_files)
    }

    /// Load a test case from a YAML file
    pub fn load_test_case<P: AsRef<Path>>(&self, file_path: P) -> Result<TestCase> {
        let file_path_ref = file_path.as_ref();
        let full_path = self.resolve_file_path(file_path_ref)?;

        let content = fs::read_to_string(&full_path)
            .context(format!("Failed to read file: {}", full_path.display()))?;

        let test_case: TestCase = match serde_yaml::from_str(&content) {
            Ok(tc) => tc,
            Err(e) => {
                log_yaml_parse_error(&e, &content, full_path.to_str().unwrap_or("unknown"));
                return Err(anyhow::anyhow!("Failed to parse YAML: {}", e));
            }
        };

        Ok(test_case)
    }

    /// Load a test case by ID
    pub fn load_test_case_by_id(&self, id: &str) -> Result<TestCase> {
        // First check if id is already a complete file path with extension
        let id_path = Path::new(id);
        if let Some(ext) = id_path.extension() {
            if YAML_EXTENSIONS.contains(&ext.to_string_lossy().as_ref()) {
                // ID already has a valid YAML extension, try to load it as-is
                let file_path = self.base_path.join(id);
                if file_path.exists() {
                    return self.load_test_case(id);
                }
                // Also try it without base_path in case it's a full path like "testcases/file.yaml"
                if id_path.exists() {
                    return self.load_test_case(id);
                }
            }
        }

        // Try each extension
        for ext in YAML_EXTENSIONS {
            let file_name = format!("{}.{}", id, ext);
            let file_path = self.base_path.join(&file_name);

            if file_path.exists() {
                return self.load_test_case(&file_name);
            }
        }

        // If no file exists, try the default extension and let error handling kick in
        let file_name = format!("{}.{}", id, YAML_EXTENSIONS[0]);
        self.load_test_case(&file_name)
    }

    /// List all test case files in the storage directory
    pub fn list_test_case_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        if !self.base_path.exists() {
            return Ok(files);
        }

        for entry in fs::read_dir(&self.base_path).context(format!(
            "Failed to read directory: {}",
            self.base_path.display()
        ))? {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if YAML_EXTENSIONS.contains(&ext.to_string_lossy().as_ref()) {
                        files.push(path);
                    }
                }
            }
        }

        files.sort();
        Ok(files)
    }

    /// Load all test cases from the storage directory
    ///
    /// This method attempts to load all test case files. Files that fail to parse
    /// or validate are skipped with a warning message that includes validation details.
    pub fn load_all_test_cases(&self) -> Result<Vec<TestCase>> {
        let files = self.list_test_case_files()?;
        let mut test_cases = Vec::new();
        let validator = SchemaValidator::new().ok();

        for file in files {
            let file_name = file
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");

            match fs::read_to_string(&file) {
                Ok(content) => match serde_yaml::from_str::<TestCase>(&content) {
                    Ok(test_case) => {
                        if let Some(ref v) = validator {
                            if let Ok(errors) = v.validate_with_details(&content) {
                                if !errors.is_empty() {
                                    log::warn!("Warning: {} has validation errors:", file_name);
                                    for error in errors.iter().take(3) {
                                        log::warn!(
                                            "  - Path '{}': {} (Expected: {})",
                                            error.path,
                                            error.constraint,
                                            error.expected_constraint
                                        );
                                    }
                                    if errors.len() > 3 {
                                        log::warn!("  ... and {} more error(s)", errors.len() - 3);
                                    }
                                }
                            }
                        }
                        test_cases.push(test_case);
                    }
                    Err(e) => {
                        log_yaml_parse_error(&e, &content, file_name);
                        log::warn!("Warning: Failed to parse {}: {}", file_name, e);
                    }
                },
                Err(e) => {
                    log::warn!("Warning: Failed to read {}: {}", file_name, e);
                }
            }
        }

        Ok(test_cases)
    }

    /// Load all test case files with validation information
    ///
    /// This method loads all YAML files and validates them, returning detailed
    /// information about each file including validation errors.
    pub fn load_all_with_validation(&self) -> Result<Vec<TestCaseFileInfo>> {
        let files = self.list_test_case_files()?;
        let validator = SchemaValidator::new()?;
        let mut results = Vec::new();

        for file in files {
            let file_info = self.load_file_with_validation(&file, &validator);
            results.push(file_info);
        }

        Ok(results)
    }

    /// Load a single file with validation details
    fn load_file_with_validation(
        &self,
        payload_path: &Path,
        validator: &SchemaValidator,
    ) -> TestCaseFileInfo {
        log::info!("Validating payload file: {}", payload_path.display());
        let content = match fs::read_to_string(payload_path) {
            Ok(c) => c,
            Err(e) => {
                return TestCaseFileInfo {
                    path: payload_path.to_path_buf(),
                    status: FileValidationStatus::ParseError {
                        message: format!("Failed to read file: {}", e),
                    },
                    test_case: None,
                };
            }
        };

        log::debug!("\tpath is valid: {}", payload_path.display());

        let yaml_content = fs::read_to_string(payload_path)
            .context(format!(
                "Failed to read YAML file: {}",
                payload_path.display()
            ))
            .unwrap();

        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&yaml_content)
            .context("Failed to parse YAML content")
            .unwrap();

        log::debug!("\tYaml parsed successfully as a string.");
        log::debug!("YAML Value: {:?}", yaml_value);

        let deserializer = serde_yaml::Deserializer::from_str(&yaml_content);
        // map_err(|e| {
        //     println!("\tError creating deserializer: {}", e);
        //     e
        // }).unwrap();

        let result: Result<TestCase, _> = serde_path_to_error::deserialize(deserializer);
        match result {
            Ok(_) => {
                log::debug!("\tDeserialization succeeded.");
            }
            Err(err) => {
                let path = err.path().to_string();
                log::debug!("\tDeserialization error at path: {}", path);
            }
        }

        let x1: Result<TestCase, serde_yaml::Error> =
            serde_yaml::from_value(yaml_value).map_err(|e| {
                log::debug!("\tError parsing TestCase: {}", e);
                e
            });
        match x1 {
            Ok(test_case) => {
                let validation_errors = match validator.validate_with_details(&content) {
                    Ok(errors) => errors,
                    Err(e) => {
                        return TestCaseFileInfo {
                            path: payload_path.to_path_buf(),
                            status: FileValidationStatus::ParseError {
                                message: format!("Validation check failed: {}", e),
                            },
                            test_case: Some(test_case),
                        };
                    }
                };

                let status = if validation_errors.is_empty() {
                    FileValidationStatus::Valid
                } else {
                    FileValidationStatus::ValidationError {
                        errors: validation_errors,
                    }
                };

                TestCaseFileInfo {
                    path: payload_path.to_path_buf(),
                    status,
                    test_case: Some(test_case),
                }
            }
            Err(e) => {
                let file_name = payload_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                log_yaml_parse_error(&e, &yaml_content, file_name);
                TestCaseFileInfo {
                    path: payload_path.to_path_buf(),
                    status: FileValidationStatus::ParseError {
                        message: format!("YAML parsing error: {}", e),
                    },
                    test_case: None,
                }
            }
        }
    }

    /// Delete a test case file by ID
    pub fn delete_test_case(&self, id: &str) -> Result<()> {
        // First check if id is already a complete file path with extension
        let id_path = Path::new(id);
        if let Some(ext) = id_path.extension() {
            if YAML_EXTENSIONS.contains(&ext.to_string_lossy().as_ref()) {
                // ID already has a valid YAML extension, try to delete it as-is
                let file_path = self.base_path.join(id);
                if file_path.exists() {
                    fs::remove_file(&file_path)
                        .context(format!("Failed to delete file: {}", file_path.display()))?;
                    return Ok(());
                }
                // Also try it without base_path in case it's a full path
                if id_path.exists() {
                    fs::remove_file(id_path)
                        .context(format!("Failed to delete file: {}", id_path.display()))?;
                    return Ok(());
                }
            }
        }

        // Try each extension
        for ext in YAML_EXTENSIONS {
            let file_name = format!("{}.{}", id, ext);
            let file_path = self.base_path.join(&file_name);

            if file_path.exists() {
                fs::remove_file(&file_path)
                    .context(format!("Failed to delete file: {}", file_path.display()))?;
                return Ok(());
            }
        }

        // TODO AGB: Need to return error if you cannot delete it
        Ok(())
    }

    /// Save a test suite to a YAML file
    pub fn save_test_suite(&self, test_suite: &TestSuite, file_name: &str) -> Result<PathBuf> {
        let file_path = self.base_path.join(file_name);

        let yaml_content =
            serde_yaml::to_string(test_suite).context("Failed to serialize test suite to YAML")?;

        fs::write(&file_path, yaml_content)
            .context(format!("Failed to write file: {}", file_path.display()))?;

        Ok(file_path)
    }

    /// Load a test suite from a YAML file
    pub fn load_test_suite<P: AsRef<Path>>(&self, file_path: P) -> Result<TestSuite> {
        let file_path_ref = file_path.as_ref();
        let full_path = self.resolve_file_path(file_path_ref)?;

        let content = fs::read_to_string(&full_path)
            .context(format!("Failed to read file: {}", full_path.display()))?;

        let test_suite: TestSuite =
            serde_yaml::from_str(&content).context("Failed to parse YAML")?;

        Ok(test_suite)
    }

    /// Check if a test case exists by ID
    pub fn test_case_exists(&self, id: &str) -> bool {
        // First check if id is already a complete file path with extension
        let id_path = Path::new(id);
        if let Some(ext) = id_path.extension() {
            if YAML_EXTENSIONS.contains(&ext.to_string_lossy().as_ref()) {
                // ID already has a valid YAML extension, check if it exists as-is
                let file_path = self.base_path.join(id);
                if file_path.exists() {
                    return true;
                }
                // Also try it without base_path in case it's a full path
                if id_path.exists() {
                    return true;
                }
            }
        }

        for ext in YAML_EXTENSIONS {
            let file_name = format!("{}.{}", id, ext);
            let file_path = self.base_path.join(&file_name);

            if file_path.exists() {
                return true;
            }
        }

        false
    }

    /// Get all unique general initial conditions from existing test cases
    /// Returns them as YAML strings for fuzzy search
    pub fn get_all_general_initial_conditions(&self) -> Result<Vec<String>> {
        let test_cases = self.load_all_test_cases()?;
        let mut conditions_set = std::collections::HashSet::new();

        for test_case in test_cases {
            if !test_case.general_initial_conditions.is_empty() {
                // Serialize the general_initial_conditions to YAML
                if let Ok(yaml_str) = serde_yaml::to_string(&test_case.general_initial_conditions) {
                    conditions_set.insert(yaml_str);
                }
            }
        }

        let mut conditions: Vec<String> = conditions_set.into_iter().collect();
        conditions.sort();
        Ok(conditions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_save_and_load_test_case() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();

        let test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test Case 1".to_string(),
        );

        storage.save_test_case(&test_case).unwrap();
        let loaded = storage.load_test_case_by_id("TC001").unwrap();

        assert_eq!(test_case.id, loaded.id);
        assert_eq!(test_case.requirement, loaded.requirement);
        assert_eq!(test_case.description, loaded.description);
    }

    #[test]
    fn test_list_test_cases() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();

        let test_case1 = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test Case 1".to_string(),
        );
        let test_case2 = TestCase::new(
            "REQ002".to_string(),
            2,
            2,
            "TC002".to_string(),
            "Test Case 2".to_string(),
        );

        storage.save_test_case(&test_case1).unwrap();
        storage.save_test_case(&test_case2).unwrap();

        let files = storage.list_test_case_files().unwrap();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_resolve_file_path_absolute() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();

        let test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test Case 1".to_string(),
        );
        let saved_path = storage.save_test_case(&test_case).unwrap();

        let resolved = storage.resolve_file_path(&saved_path).unwrap();
        assert_eq!(resolved, saved_path);
        assert!(resolved.exists());
    }

    #[test]
    fn test_resolve_file_path_relative_exists_directly() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();

        let test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test Case 1".to_string(),
        );
        let saved_path = storage.save_test_case(&test_case).unwrap();

        let resolved = storage.resolve_file_path(&saved_path).unwrap();
        assert_eq!(resolved, saved_path);
        assert!(resolved.exists());
    }

    #[test]
    fn test_resolve_file_path_relative_needs_join() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();

        let test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test Case 1".to_string(),
        );
        storage.save_test_case(&test_case).unwrap();

        let resolved = storage.resolve_file_path(Path::new("TC001.yaml")).unwrap();
        assert!(resolved.exists());
        assert_eq!(resolved.file_name().unwrap(), "TC001.yaml");
    }

    #[test]
    fn test_resolve_file_path_substring_single_match() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();

        let test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "unique_test_123".to_string(),
            "Test Case".to_string(),
        );
        storage.save_test_case(&test_case).unwrap();

        let resolved = storage.resolve_file_path(Path::new("unique_test")).unwrap();
        assert!(resolved.exists());
        assert!(resolved.to_string_lossy().contains("unique_test_123"));
    }

    #[test]
    fn test_resolve_file_path_substring_multiple_matches() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();

        let test_case1 = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "test_abc".to_string(),
            "Test Case 1".to_string(),
        );
        let test_case2 = TestCase::new(
            "REQ002".to_string(),
            2,
            2,
            "test_xyz".to_string(),
            "Test Case 2".to_string(),
        );
        storage.save_test_case(&test_case1).unwrap();
        storage.save_test_case(&test_case2).unwrap();

        let result = storage.resolve_file_path(Path::new("test_"));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Multiple files match"));
    }

    #[test]
    fn test_resolve_file_path_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();

        let result = storage.resolve_file_path(Path::new("nonexistent.yaml"));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("File not found"));
    }

    #[test]
    fn test_load_test_case_by_id_with_yaml_extension() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();

        let test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test Case 1".to_string(),
        );
        storage.save_test_case(&test_case).unwrap();

        let loaded = storage.load_test_case_by_id("TC001").unwrap();
        assert_eq!(loaded.id, "TC001");
        assert_eq!(loaded.description, "Test Case 1");
    }

    #[test]
    fn test_load_test_case_by_id_with_yml_extension() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();

        let test_case = TestCase::new(
            "REQ002".to_string(),
            2,
            2,
            "TC002".to_string(),
            "Test Case 2".to_string(),
        );
        let yaml_content = serde_yaml::to_string(&test_case).unwrap();

        let file_path = temp_dir.path().join("TC002.yml");
        fs::write(&file_path, yaml_content).unwrap();

        let loaded = storage.load_test_case_by_id("TC002").unwrap();
        assert_eq!(loaded.id, "TC002");
        assert_eq!(loaded.description, "Test Case 2");
    }

    #[test]
    fn test_test_case_exists_with_yaml() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();

        let test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test Case 1".to_string(),
        );
        storage.save_test_case(&test_case).unwrap();

        assert!(storage.test_case_exists("TC001"));
        assert!(!storage.test_case_exists("TC999"));
    }

    #[test]
    fn test_test_case_exists_with_yml() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();

        let test_case = TestCase::new(
            "REQ002".to_string(),
            2,
            2,
            "TC002".to_string(),
            "Test Case 2".to_string(),
        );
        let yaml_content = serde_yaml::to_string(&test_case).unwrap();

        let file_path = temp_dir.path().join("TC002.yml");
        fs::write(&file_path, yaml_content).unwrap();

        assert!(storage.test_case_exists("TC002"));
    }

    #[test]
    fn test_delete_test_case_with_yaml() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();

        let test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test Case 1".to_string(),
        );
        storage.save_test_case(&test_case).unwrap();

        assert!(storage.test_case_exists("TC001"));
        storage.delete_test_case("TC001").unwrap();
        assert!(!storage.test_case_exists("TC001"));
    }

    #[test]
    fn test_delete_test_case_with_yml() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();

        let test_case = TestCase::new(
            "REQ002".to_string(),
            2,
            2,
            "TC002".to_string(),
            "Test Case 2".to_string(),
        );
        let yaml_content = serde_yaml::to_string(&test_case).unwrap();

        let file_path = temp_dir.path().join("TC002.yml");
        fs::write(&file_path, yaml_content).unwrap();

        assert!(storage.test_case_exists("TC002"));
        storage.delete_test_case("TC002").unwrap();
        assert!(!storage.test_case_exists("TC002"));
    }

    #[test]
    fn test_find_files_by_substring() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();

        let test_case1 = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "gsma_4_4_2_2".to_string(),
            "Test 1".to_string(),
        );
        let test_case2 = TestCase::new(
            "REQ002".to_string(),
            2,
            2,
            "gsma_4_4_2_3".to_string(),
            "Test 2".to_string(),
        );
        let test_case3 = TestCase::new(
            "REQ003".to_string(),
            3,
            3,
            "other_test".to_string(),
            "Test 3".to_string(),
        );

        storage.save_test_case(&test_case1).unwrap();
        storage.save_test_case(&test_case2).unwrap();
        storage.save_test_case(&test_case3).unwrap();

        let matches = storage.find_files_by_substring("gsma_4_4").unwrap();
        assert_eq!(matches.len(), 2);

        let matches_other = storage.find_files_by_substring("other").unwrap();
        assert_eq!(matches_other.len(), 1);

        let no_matches = storage.find_files_by_substring("nonexistent").unwrap();
        assert_eq!(no_matches.len(), 0);
    }

    #[test]
    fn test_load_test_case_by_id_with_complete_filename() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();

        let test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test Case 1".to_string(),
        );
        storage.save_test_case(&test_case).unwrap();

        let loaded = storage.load_test_case_by_id("TC001.yaml").unwrap();
        assert_eq!(loaded.id, "TC001");
        assert_eq!(loaded.description, "Test Case 1");
    }

    #[test]
    fn test_load_test_case_by_id_with_complete_filename_yml() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();

        let test_case = TestCase::new(
            "REQ002".to_string(),
            2,
            2,
            "TC002".to_string(),
            "Test Case 2".to_string(),
        );
        let yaml_content = serde_yaml::to_string(&test_case).unwrap();

        let file_path = temp_dir.path().join("TC002.yml");
        fs::write(&file_path, yaml_content).unwrap();

        let loaded = storage.load_test_case_by_id("TC002.yml").unwrap();
        assert_eq!(loaded.id, "TC002");
        assert_eq!(loaded.description, "Test Case 2");
    }

    #[test]
    fn test_test_case_exists_with_complete_filename() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();

        let test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test Case 1".to_string(),
        );
        storage.save_test_case(&test_case).unwrap();

        assert!(storage.test_case_exists("TC001.yaml"));
        assert!(storage.test_case_exists("TC001"));
        assert!(!storage.test_case_exists("TC999.yaml"));
    }

    #[test]
    fn test_delete_test_case_with_complete_filename() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();

        let test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test Case 1".to_string(),
        );
        storage.save_test_case(&test_case).unwrap();

        assert!(storage.test_case_exists("TC001.yaml"));

        storage.delete_test_case("TC001.yaml").unwrap();
        assert!(!storage.test_case_exists("TC001"));
        assert!(!storage.test_case_exists("TC001.yaml"));
    }

    #[test]
    fn test_load_test_case_by_id_with_path_prefix() {
        let temp_dir = TempDir::new().unwrap();

        let data_dir = temp_dir.path().join("data");
        fs::create_dir_all(&data_dir).unwrap();

        let storage = TestCaseStorage::new(&data_dir).unwrap();

        let test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "gsma_4_4_2_2_TC".to_string(),
            "GSMA Test".to_string(),
        );
        storage.save_test_case(&test_case).unwrap();

        let loaded = storage
            .load_test_case_by_id("gsma_4_4_2_2_TC.yaml")
            .unwrap();
        assert_eq!(loaded.id, "gsma_4_4_2_2_TC");
        assert_eq!(loaded.description, "GSMA Test");
    }
}
