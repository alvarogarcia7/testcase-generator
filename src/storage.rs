use crate::models::{TestCase, TestSuite};
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Manages storage and retrieval of test cases
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
        let file_name = format!("{}.yaml", test_case.id);
        let file_path = self.base_path.join(&file_name);

        let yaml_content =
            serde_yaml::to_string(test_case).context("Failed to serialize test case to YAML")?;

        fs::write(&file_path, yaml_content)
            .context(format!("Failed to write file: {}", file_path.display()))?;

        Ok(file_path)
    }

    /// Load a test case from a YAML file
    pub fn load_test_case<P: AsRef<Path>>(&self, file_path: P) -> Result<TestCase> {
        let full_path = if file_path.as_ref().is_absolute() {
            file_path.as_ref().to_path_buf()
        } else {
            self.base_path.join(file_path.as_ref())
        };

        let content = fs::read_to_string(&full_path)
            .context(format!("Failed to read file: {}", full_path.display()))?;

        let test_case: TestCase = serde_yaml::from_str(&content).context("Failed to parse YAML")?;

        Ok(test_case)
    }

    /// Load a test case by ID
    pub fn load_test_case_by_id(&self, id: &str) -> Result<TestCase> {
        let file_name = format!("{}.yaml", id);
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
                    if ext == "yaml" || ext == "yml" {
                        files.push(path);
                    }
                }
            }
        }

        files.sort();
        Ok(files)
    }

    /// Load all test cases from the storage directory
    pub fn load_all_test_cases(&self) -> Result<Vec<TestCase>> {
        let files = self.list_test_case_files()?;
        let mut test_cases = Vec::new();

        for file in files {
            match self.load_test_case(&file) {
                Ok(test_case) => test_cases.push(test_case),
                Err(e) => {
                    eprintln!("Warning: Failed to load {}: {}", file.display(), e);
                }
            }
        }

        Ok(test_cases)
    }

    /// Delete a test case file by ID
    pub fn delete_test_case(&self, id: &str) -> Result<()> {
        let file_name = format!("{}.yaml", id);
        let file_path = self.base_path.join(&file_name);

        if file_path.exists() {
            fs::remove_file(&file_path)
                .context(format!("Failed to delete file: {}", file_path.display()))?;
        }

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
        let full_path = if file_path.as_ref().is_absolute() {
            file_path.as_ref().to_path_buf()
        } else {
            self.base_path.join(file_path.as_ref())
        };

        let content = fs::read_to_string(&full_path)
            .context(format!("Failed to read file: {}", full_path.display()))?;

        let test_suite: TestSuite =
            serde_yaml::from_str(&content).context("Failed to parse YAML")?;

        Ok(test_suite)
    }

    /// Check if a test case exists by ID
    pub fn test_case_exists(&self, id: &str) -> bool {
        let file_name = format!("{}.yaml", id);
        let file_path = self.base_path.join(&file_name);
        file_path.exists()
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

        let test_case = TestCase::new("TC001".to_string(), "Test Case 1".to_string());

        storage.save_test_case(&test_case).unwrap();
        let loaded = storage.load_test_case_by_id("TC001").unwrap();

        assert_eq!(test_case.id, loaded.id);
        assert_eq!(test_case.title, loaded.title);
    }

    #[test]
    fn test_list_test_cases() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();

        let test_case1 = TestCase::new("TC001".to_string(), "Test Case 1".to_string());
        let test_case2 = TestCase::new("TC002".to_string(), "Test Case 2".to_string());

        storage.save_test_case(&test_case1).unwrap();
        storage.save_test_case(&test_case2).unwrap();

        let files = storage.list_test_case_files().unwrap();
        assert_eq!(files.len(), 2);
    }
}
