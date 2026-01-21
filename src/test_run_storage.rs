use crate::models::TestRun;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Manages storage and retrieval of test run results
pub struct TestRunStorage {
    base_path: PathBuf,
}

impl TestRunStorage {
    /// Create a new test run storage manager
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

    /// Get the folder path for a specific test case's runs
    pub fn get_test_run_folder(&self, test_case_id: &str) -> PathBuf {
        self.base_path.join(test_case_id).join("runs")
    }

    /// Save a test run to a YAML file
    pub fn save_test_run(&self, test_run: &TestRun) -> Result<PathBuf> {
        let runs_folder = self.get_test_run_folder(&test_run.test_case_id);

        if !runs_folder.exists() {
            fs::create_dir_all(&runs_folder).context(format!(
                "Failed to create runs directory: {}",
                runs_folder.display()
            ))?;
        }

        let timestamp_str = test_run.timestamp.to_rfc3339();
        let file_name = format!("{}.yaml", timestamp_str);
        let file_path = runs_folder.join(&file_name);

        let yaml_content =
            serde_yaml::to_string(test_run).context("Failed to serialize test run to YAML")?;

        fs::write(&file_path, yaml_content)
            .context(format!("Failed to write file: {}", file_path.display()))?;

        Ok(file_path)
    }

    /// Load all test runs for a specific test case ID
    pub fn load_test_runs_for_case(&self, test_case_id: &str) -> Result<Vec<TestRun>> {
        let runs_folder = self.get_test_run_folder(test_case_id);

        if !runs_folder.exists() {
            return Ok(Vec::new());
        }

        let mut test_runs = Vec::new();

        for entry in fs::read_dir(&runs_folder).context(format!(
            "Failed to read directory: {}",
            runs_folder.display()
        ))? {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "yaml" || ext == "yml" {
                        match self.load_test_run_from_file(&path) {
                            Ok(test_run) => test_runs.push(test_run),
                            Err(e) => {
                                log::warn!(
                                    "Warning: Failed to load test run from {}: {}",
                                    path.display(),
                                    e
                                );
                            }
                        }
                    }
                }
            }
        }

        test_runs.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        Ok(test_runs)
    }

    /// Load all test runs from all test case folders
    pub fn load_all_test_runs(&self) -> Result<Vec<TestRun>> {
        if !self.base_path.exists() {
            return Ok(Vec::new());
        }

        let mut all_test_runs = Vec::new();

        for entry in fs::read_dir(&self.base_path).context(format!(
            "Failed to read directory: {}",
            self.base_path.display()
        ))? {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(test_case_id) = path.file_name().and_then(|n| n.to_str()) {
                    match self.load_test_runs_for_case(test_case_id) {
                        Ok(mut test_runs) => {
                            all_test_runs.append(&mut test_runs);
                        }
                        Err(e) => {
                            log::warn!(
                                "Warning: Failed to load test runs for case {}: {}",
                                test_case_id,
                                e
                            );
                        }
                    }
                }
            }
        }

        all_test_runs.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        Ok(all_test_runs)
    }

    /// Helper method to load a test run from a specific file
    fn load_test_run_from_file(&self, file_path: &Path) -> Result<TestRun> {
        let content = fs::read_to_string(file_path)
            .context(format!("Failed to read file: {}", file_path.display()))?;

        let test_run: TestRun = serde_yaml::from_str(&content).context("Failed to parse YAML")?;

        Ok(test_run)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::TestRunStatus;
    use chrono::{Duration, Utc};
    use tempfile::TempDir;

    fn create_test_run(test_case_id: &str, duration_s: f64) -> TestRun {
        TestRun {
            test_case_id: test_case_id.to_string(),
            name: Some(test_case_id.to_string()),
            timestamp: Utc::now(),
            status: TestRunStatus::Pass,
            duration: duration_s,
            execution_log: "Test execution log".to_string(),
            error_message: None,
        }
    }

    #[test]
    fn test_create_storage() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestRunStorage::new(temp_dir.path()).unwrap();
        assert!(storage.base_path.exists());
    }

    #[test]
    fn test_get_test_run_folder() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestRunStorage::new(temp_dir.path()).unwrap();

        let folder = storage.get_test_run_folder("TC001");
        assert_eq!(folder, temp_dir.path().join("TC001").join("runs"));
    }

    #[test]
    fn test_save_test_run() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestRunStorage::new(temp_dir.path()).unwrap();

        let test_run = create_test_run("TC001", 1.000);
        let saved_path = storage.save_test_run(&test_run).unwrap();

        assert!(saved_path.exists());
        assert!(saved_path.to_string_lossy().contains("TC001"));
        assert!(saved_path.to_string_lossy().contains("runs"));
    }

    #[test]
    fn test_save_test_run_creates_folder_structure() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestRunStorage::new(temp_dir.path()).unwrap();

        let test_run = create_test_run("TC002", 2.000);
        storage.save_test_run(&test_run).unwrap();

        let runs_folder = storage.get_test_run_folder("TC002");
        assert!(runs_folder.exists());
        assert!(runs_folder.is_dir());
    }

    #[test]
    fn test_load_test_runs_for_case() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestRunStorage::new(temp_dir.path()).unwrap();

        let test_run1 = create_test_run("TC001", 1.000);
        let test_run2 = create_test_run("TC001", 2.000);

        storage.save_test_run(&test_run1).unwrap();
        storage.save_test_run(&test_run2).unwrap();

        let loaded_runs = storage.load_test_runs_for_case("TC001").unwrap();
        assert_eq!(loaded_runs.len(), 2);
        assert_eq!(loaded_runs[0].test_case_id, "TC001");
        assert_eq!(loaded_runs[1].test_case_id, "TC001");
        temp_dir.close().unwrap();
    }

    #[test]
    fn test_load_test_runs_for_nonexistent_case() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestRunStorage::new(temp_dir.path()).unwrap();

        let loaded_runs = storage.load_test_runs_for_case("NONEXISTENT").unwrap();
        assert_eq!(loaded_runs.len(), 0);
    }

    #[test]
    fn test_load_all_test_runs() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestRunStorage::new(temp_dir.path()).unwrap();

        let test_run1 = create_test_run("TC001", 1.000);
        let test_run2 = create_test_run("TC002", 2.000);
        let test_run3 = create_test_run("TC001", 3.000);

        storage.save_test_run(&test_run1).unwrap();
        storage.save_test_run(&test_run2).unwrap();
        storage.save_test_run(&test_run3).unwrap();

        let all_runs = storage.load_all_test_runs().unwrap();
        assert_eq!(all_runs.len(), 3);
    }

    #[test]
    fn test_load_all_test_runs_from_empty_storage() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestRunStorage::new(temp_dir.path()).unwrap();

        let all_runs = storage.load_all_test_runs().unwrap();
        assert_eq!(all_runs.len(), 0);
    }

    #[test]
    fn test_test_run_serialization_roundtrip() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestRunStorage::new(temp_dir.path()).unwrap();

        let original_run = TestRun {
            test_case_id: "TC123".to_string(),
            name: Some("TC123".to_string()),
            timestamp: Utc::now(),
            status: TestRunStatus::Fail,
            duration: 5.000,
            execution_log: "Detailed log\nwith multiple lines".to_string(),
            error_message: Some("Error occurred".to_string()),
        };

        storage.save_test_run(&original_run).unwrap();
        let loaded_runs = storage.load_test_runs_for_case("TC123").unwrap();

        assert_eq!(loaded_runs.len(), 1);
        let loaded_run = &loaded_runs[0];
        assert_eq!(loaded_run.test_case_id, original_run.test_case_id);
        assert_eq!(loaded_run.status, original_run.status);
        assert_eq!(loaded_run.duration, original_run.duration);
        assert_eq!(loaded_run.execution_log, original_run.execution_log);
        assert_eq!(loaded_run.error_message, original_run.error_message);
    }

    #[test]
    fn test_multiple_test_runs_sorted_by_timestamp() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestRunStorage::new(temp_dir.path()).unwrap();

        let now = Utc::now();

        let test_run2 = TestRun {
            test_case_id: "TC001".to_string(),
            name: Some("TC001".to_string()),
            timestamp: now + Duration::seconds(10),
            status: TestRunStatus::Pass,
            duration: 2.000,
            execution_log: "Second run".to_string(),
            error_message: None,
        };

        let test_run1 = TestRun {
            test_case_id: "TC001".to_string(),
            name: Some("TC001".to_string()),
            timestamp: now,
            status: TestRunStatus::Pass,
            duration: 1.000,
            execution_log: "First run".to_string(),
            error_message: None,
        };

        storage.save_test_run(&test_run2).unwrap();
        storage.save_test_run(&test_run1).unwrap();

        let loaded_runs = storage.load_test_runs_for_case("TC001").unwrap();
        assert_eq!(loaded_runs.len(), 2);
        assert_eq!(loaded_runs[0].execution_log, "First run");
        assert_eq!(loaded_runs[1].execution_log, "Second run");
    }
}
