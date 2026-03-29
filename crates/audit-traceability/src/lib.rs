use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageInfo {
    pub path: PathBuf,
    pub sha256: String,
}

impl StageInfo {
    pub fn new(path: PathBuf, sha256: String) -> Self {
        Self { path, sha256 }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let content =
            fs::read(&path).context(format!("Failed to read file: {}", path.display()))?;
        let sha256 = compute_sha256(&content);
        Ok(Self::new(path, sha256))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCaseAudit {
    pub stages: HashMap<String, StageInfo>,
}

impl TestCaseAudit {
    pub fn new() -> Self {
        Self {
            stages: HashMap::new(),
        }
    }

    pub fn add_stage<S: Into<String>>(&mut self, stage_name: S, stage_info: StageInfo) {
        self.stages.insert(stage_name.into(), stage_info);
    }

    pub fn get_stage(&self, stage_name: &str) -> Option<&StageInfo> {
        self.stages.get(stage_name)
    }

    pub fn verify_stage(&self, stage_name: &str) -> Result<bool> {
        let stage = self
            .stages
            .get(stage_name)
            .ok_or_else(|| anyhow::anyhow!("Stage '{}' not found", stage_name))?;

        let content = fs::read(&stage.path).context(format!(
            "Failed to read file for verification: {}",
            stage.path.display()
        ))?;

        let computed_hash = compute_sha256(&content);
        Ok(computed_hash == stage.sha256)
    }
}

impl Default for TestCaseAudit {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditTraceabilityLog {
    pub date: DateTime<Utc>,
    pub witness_key: String,
    pub test_cases: HashMap<String, TestCaseAudit>,
}

impl AuditTraceabilityLog {
    pub fn new(witness_key: String) -> Self {
        Self {
            date: Utc::now(),
            witness_key,
            test_cases: HashMap::new(),
        }
    }

    pub fn add_test_case<S: Into<String>>(&mut self, tc_id: S, audit: TestCaseAudit) {
        self.test_cases.insert(tc_id.into(), audit);
    }

    pub fn get_test_case(&self, tc_id: &str) -> Option<&TestCaseAudit> {
        self.test_cases.get(tc_id)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(self)
            .context("Failed to serialize audit traceability log")?;

        fs::write(path.as_ref(), json).context(format!(
            "Failed to write audit traceability log to {}",
            path.as_ref().display()
        ))?;

        log::info!(
            "Audit traceability log saved to: {}",
            path.as_ref().display()
        );

        Ok(())
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref()).context(format!(
            "Failed to read audit traceability log from {}",
            path.as_ref().display()
        ))?;

        let log: AuditTraceabilityLog = serde_json::from_str(&content)
            .context("Failed to deserialize audit traceability log")?;

        Ok(log)
    }

    pub fn verify_test_case(&self, tc_id: &str) -> Result<VerificationResult> {
        let audit = self
            .test_cases
            .get(tc_id)
            .ok_or_else(|| anyhow::anyhow!("Test case '{}' not found in audit log", tc_id))?;

        let mut results = Vec::new();
        let mut all_passed = true;

        for (stage_name, stage_info) in &audit.stages {
            match verify_file(&stage_info.path, &stage_info.sha256) {
                Ok(true) => {
                    results.push(StageVerificationResult {
                        stage_name: stage_name.clone(),
                        passed: true,
                        message: format!("✓ Stage '{}' verified", stage_name),
                    });
                }
                Ok(false) => {
                    all_passed = false;
                    results.push(StageVerificationResult {
                        stage_name: stage_name.clone(),
                        passed: false,
                        message: format!(
                            "✗ Stage '{}' hash mismatch (file: {})",
                            stage_name,
                            stage_info.path.display()
                        ),
                    });
                }
                Err(e) => {
                    all_passed = false;
                    results.push(StageVerificationResult {
                        stage_name: stage_name.clone(),
                        passed: false,
                        message: format!("✗ Stage '{}' verification failed: {}", stage_name, e),
                    });
                }
            }
        }

        Ok(VerificationResult {
            test_case_id: tc_id.to_string(),
            all_passed,
            stage_results: results,
        })
    }

    pub fn verify_all(&self) -> Result<Vec<VerificationResult>> {
        let mut results = Vec::new();

        for tc_id in self.test_cases.keys() {
            results.push(self.verify_test_case(tc_id)?);
        }

        Ok(results)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageVerificationResult {
    pub stage_name: String,
    pub passed: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub test_case_id: String,
    pub all_passed: bool,
    pub stage_results: Vec<StageVerificationResult>,
}

impl VerificationResult {
    pub fn print_summary(&self) {
        let status = if self.all_passed { "✓" } else { "✗" };
        println!(
            "{} Test Case: {} ({})",
            status,
            self.test_case_id,
            if self.all_passed { "PASS" } else { "FAIL" }
        );

        for stage_result in &self.stage_results {
            println!("  {}", stage_result.message);
        }
    }
}

pub fn compute_sha256(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    format!("{:x}", hasher.finalize())
}

pub fn verify_file(path: &Path, expected_hash: &str) -> Result<bool> {
    if !path.exists() {
        anyhow::bail!("File not found: {}", path.display());
    }

    let content = fs::read(path).context(format!("Failed to read file: {}", path.display()))?;

    let computed_hash = compute_sha256(&content);
    Ok(computed_hash == expected_hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_compute_sha256() {
        let content = b"Hello, World!";
        let hash = compute_sha256(content);
        assert_eq!(
            hash,
            "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"
        );
    }

    #[test]
    fn test_stage_info_from_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, b"test content").unwrap();

        let stage_info = StageInfo::from_file(&file_path).unwrap();
        assert_eq!(stage_info.path, file_path);
        assert!(!stage_info.sha256.is_empty());
    }

    #[test]
    fn test_audit_log_save_and_load() {
        let temp_dir = tempfile::tempdir().unwrap();
        let log_path = temp_dir.path().join("audit.json");

        let mut log = AuditTraceabilityLog::new("test-witness".to_string());
        let mut audit = TestCaseAudit::new();

        let test_file = temp_dir.path().join("test.yaml");
        fs::write(&test_file, b"test: data").unwrap();

        let stage_info = StageInfo::from_file(&test_file).unwrap();
        audit.add_stage("initial", stage_info);

        log.add_test_case("TC001", audit);

        log.save_to_file(&log_path).unwrap();

        let loaded_log = AuditTraceabilityLog::load_from_file(&log_path).unwrap();
        assert_eq!(loaded_log.witness_key, "test-witness");
        assert_eq!(loaded_log.test_cases.len(), 1);
    }

    #[test]
    fn test_verify_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(b"test content").unwrap();
        drop(file);

        let content = fs::read(&file_path).unwrap();
        let hash = compute_sha256(&content);

        assert!(verify_file(&file_path, &hash).unwrap());

        let wrong_hash = "0000000000000000000000000000000000000000000000000000000000000000";
        assert!(!verify_file(&file_path, wrong_hash).unwrap());
    }

    #[test]
    fn test_verify_test_case() {
        let temp_dir = tempfile::tempdir().unwrap();

        let test_file = temp_dir.path().join("test.yaml");
        fs::write(&test_file, b"test: data").unwrap();

        let mut log = AuditTraceabilityLog::new("test-witness".to_string());
        let mut audit = TestCaseAudit::new();

        let stage_info = StageInfo::from_file(&test_file).unwrap();
        audit.add_stage("initial", stage_info);

        log.add_test_case("TC001", audit);

        let result = log.verify_test_case("TC001").unwrap();
        assert!(result.all_passed);
        assert_eq!(result.stage_results.len(), 1);
    }
}
