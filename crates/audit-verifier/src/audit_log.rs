use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OperationType {
    GenerateScript,
    ExecuteScript,
    VerifyTest,
    HydrateYaml,
    GenerateExport,
    ValidateExport,
    ListTestCases,
    ResolveDependencies,
    ValidateYaml,
    LoadTestCase,
    SaveTestCase,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub timestamp: DateTime<Utc>,
    pub operation: OperationType,
    pub user: Option<String>,
    pub hostname: Option<String>,
    pub working_directory: Option<PathBuf>,
    pub input_files: Vec<PathBuf>,
    pub output_files: Vec<PathBuf>,
    pub input_file_hashes: Vec<(PathBuf, String)>,
    pub output_file_hashes: Vec<(PathBuf, String)>,
    pub command_args: Vec<String>,
    pub status: OperationStatus,
    pub error_message: Option<String>,
    pub duration_ms: Option<u64>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OperationStatus {
    Started,
    Success,
    Failed,
    Warning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub version: String,
    pub entries: Vec<AuditLogEntry>,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl Default for AuditLog {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            version: "1.0.0".to_string(),
            entries: Vec::new(),
            created_at: now,
            last_updated: now,
        }
    }
}

impl AuditLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .context(format!("Failed to read audit log from: {}", path.display()))?;
        let log: AuditLog =
            serde_json::from_str(&content).context("Failed to parse audit log JSON")?;
        Ok(log)
    }

    pub fn save_to_file(&mut self, path: &Path) -> Result<()> {
        self.last_updated = Utc::now();
        let json = serde_json::to_string_pretty(self).context("Failed to serialize audit log")?;
        fs::write(path, json)
            .context(format!("Failed to write audit log to: {}", path.display()))?;
        Ok(())
    }

    pub fn append_to_file(&mut self, path: &Path, entry: AuditLogEntry) -> Result<()> {
        self.entries.push(entry.clone());
        self.last_updated = Utc::now();

        let parent = path.parent();
        if let Some(dir) = parent {
            if !dir.exists() {
                fs::create_dir_all(dir)
                    .context(format!("Failed to create directory: {}", dir.display()))?;
            }
        }

        // Check if file exists and has content
        let has_content = path.exists() && fs::metadata(path).map(|m| m.len() > 0).unwrap_or(false);

        if has_content {
            let mut existing = Self::load_from_file(path)?;
            existing.entries.push(entry);
            existing.save_to_file(path)?;
        } else {
            self.save_to_file(path)?;
        }

        Ok(())
    }

    pub fn compute_hash(&self) -> String {
        let json = serde_json::to_string(self).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(json.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

impl AuditLogEntry {
    pub fn builder(operation: OperationType) -> AuditLogEntryBuilder {
        AuditLogEntryBuilder::new(operation)
    }

    pub fn compute_hash(&self) -> String {
        let json = serde_json::to_string(self).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(json.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

pub struct AuditLogEntryBuilder {
    entry: AuditLogEntry,
    start_time: DateTime<Utc>,
}

impl AuditLogEntryBuilder {
    pub fn new(operation: OperationType) -> Self {
        let now = Utc::now();
        Self {
            entry: AuditLogEntry {
                timestamp: now,
                operation,
                user: get_current_user(),
                hostname: get_hostname(),
                working_directory: std::env::current_dir().ok(),
                input_files: Vec::new(),
                output_files: Vec::new(),
                input_file_hashes: Vec::new(),
                output_file_hashes: Vec::new(),
                command_args: std::env::args().collect(),
                status: OperationStatus::Started,
                error_message: None,
                duration_ms: None,
                metadata: serde_json::Value::Object(Default::default()),
            },
            start_time: now,
        }
    }

    pub fn with_input_file(mut self, path: PathBuf) -> Self {
        if let Ok(hash) = compute_file_hash(&path) {
            self.entry.input_file_hashes.push((path.clone(), hash));
        }
        self.entry.input_files.push(path);
        self
    }

    pub fn with_input_files<I>(mut self, paths: I) -> Self
    where
        I: IntoIterator<Item = PathBuf>,
    {
        for path in paths {
            if let Ok(hash) = compute_file_hash(&path) {
                self.entry.input_file_hashes.push((path.clone(), hash));
            }
            self.entry.input_files.push(path);
        }
        self
    }

    pub fn with_output_file(mut self, path: PathBuf) -> Self {
        self.entry.output_files.push(path);
        self
    }

    pub fn with_output_files<I>(mut self, paths: I) -> Self
    where
        I: IntoIterator<Item = PathBuf>,
    {
        for path in paths {
            self.entry.output_files.push(path);
        }
        self
    }

    pub fn with_metadata<K: Into<String>, V: Serialize>(mut self, key: K, value: V) -> Self {
        if let serde_json::Value::Object(ref mut map) = self.entry.metadata {
            if let Ok(v) = serde_json::to_value(value) {
                map.insert(key.into(), v);
            }
        }
        self
    }

    pub fn with_status(mut self, status: OperationStatus) -> Self {
        self.entry.status = status;
        self
    }

    pub fn with_error(mut self, error: String) -> Self {
        self.entry.error_message = Some(error);
        self.entry.status = OperationStatus::Failed;
        self
    }

    pub fn build(mut self) -> AuditLogEntry {
        let now = Utc::now();
        self.entry.duration_ms = Some((now - self.start_time).num_milliseconds() as u64);

        for path in &self.entry.output_files {
            if let Ok(hash) = compute_file_hash(path) {
                self.entry.output_file_hashes.push((path.clone(), hash));
            }
        }

        self.entry
    }
}

pub fn compute_file_hash(path: &Path) -> Result<String> {
    let content = fs::read(path).context(format!("Failed to read file: {}", path.display()))?;
    let mut hasher = Sha256::new();
    hasher.update(&content);
    Ok(format!("{:x}", hasher.finalize()))
}

fn get_current_user() -> Option<String> {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .ok()
}

fn get_hostname() -> Option<String> {
    hostname::get().ok().and_then(|h| h.into_string().ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_audit_log_creation() {
        let log = AuditLog::new();
        assert_eq!(log.version, "1.0.0");
        assert!(log.entries.is_empty());
    }

    #[test]
    fn test_audit_log_entry_builder() {
        let entry = AuditLogEntry::builder(OperationType::GenerateScript)
            .with_metadata("test_key", "test_value")
            .with_status(OperationStatus::Success)
            .build();

        assert_eq!(entry.operation, OperationType::GenerateScript);
        assert_eq!(entry.status, OperationStatus::Success);
    }

    #[test]
    fn test_audit_log_save_and_load() {
        let temp_file = NamedTempFile::new().unwrap();
        let mut log = AuditLog::new();

        let entry = AuditLogEntry::builder(OperationType::GenerateScript)
            .with_metadata("key", "value")
            .build();

        log.entries.push(entry);
        log.save_to_file(temp_file.path()).unwrap();

        let loaded = AuditLog::load_from_file(temp_file.path()).unwrap();
        assert_eq!(loaded.entries.len(), 1);
        assert_eq!(loaded.entries[0].operation, OperationType::GenerateScript);
    }

    #[test]
    fn test_compute_file_hash() {
        let temp_file = NamedTempFile::new().unwrap();
        fs::write(temp_file.path(), b"test content").unwrap();

        let hash = compute_file_hash(temp_file.path()).unwrap();
        assert_eq!(hash.len(), 64);

        let hash2 = compute_file_hash(temp_file.path()).unwrap();
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_audit_log_append() {
        let temp_file = NamedTempFile::new().unwrap();
        let mut log = AuditLog::new();

        let entry1 = AuditLogEntry::builder(OperationType::GenerateScript).build();
        log.append_to_file(temp_file.path(), entry1).unwrap();

        let entry2 = AuditLogEntry::builder(OperationType::ExecuteScript).build();
        log.append_to_file(temp_file.path(), entry2).unwrap();

        let loaded = AuditLog::load_from_file(temp_file.path()).unwrap();
        assert_eq!(loaded.entries.len(), 2);
    }

    #[test]
    fn test_operation_type_serialization() {
        let op = OperationType::GenerateScript;
        let json = serde_json::to_string(&op).unwrap();
        assert_eq!(json, r#""generate_script""#);

        let custom = OperationType::Other("custom".to_string());
        let json = serde_json::to_string(&custom).unwrap();
        assert!(json.contains("custom"));
    }
}
