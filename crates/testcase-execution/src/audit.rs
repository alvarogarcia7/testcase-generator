use anyhow::Result;
use audit_verifier::audit_log::{AuditLog, AuditLogEntry, OperationType, OperationStatus};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

pub struct AuditLogger {
    log: Arc<Mutex<AuditLog>>,
    log_file: Option<PathBuf>,
    enabled: bool,
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl AuditLogger {
    pub fn new() -> Self {
        Self {
            log: Arc::new(Mutex::new(AuditLog::new())),
            log_file: None,
            enabled: std::env::var("AUDIT_LOG_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse::<bool>()
                .unwrap_or(true),
        }
    }

    pub fn with_file<P: AsRef<Path>>(path: P) -> Self {
        let log_file = path.as_ref().to_path_buf();
        
        let log = if log_file.exists() {
            match AuditLog::load_from_file(&log_file) {
                Ok(existing_log) => Arc::new(Mutex::new(existing_log)),
                Err(e) => {
                    log::warn!("Failed to load existing audit log, creating new one: {}", e);
                    Arc::new(Mutex::new(AuditLog::new()))
                }
            }
        } else {
            Arc::new(Mutex::new(AuditLog::new()))
        };

        Self {
            log,
            log_file: Some(log_file),
            enabled: std::env::var("AUDIT_LOG_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse::<bool>()
                .unwrap_or(true),
        }
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn log_entry(&self, entry: AuditLogEntry) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        if let Ok(mut log) = self.log.lock() {
            if let Some(ref path) = self.log_file {
                log.append_to_file(path, entry)?;
            } else {
                log.entries.push(entry);
            }
        }

        Ok(())
    }

    pub fn log_generate_script(
        &self,
        yaml_file: &Path,
        output_file: Option<&Path>,
        status: OperationStatus,
        error: Option<String>,
    ) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let mut builder = AuditLogEntry::builder(OperationType::GenerateScript)
            .with_input_file(yaml_file.to_path_buf())
            .with_status(status);

        if let Some(output) = output_file {
            builder = builder.with_output_file(output.to_path_buf());
        }

        if let Some(err) = error {
            builder = builder.with_error(err);
        }

        let entry = builder.build();
        self.log_entry(entry)
    }

    pub fn log_execute_script(
        &self,
        yaml_file: &Path,
        status: OperationStatus,
        error: Option<String>,
    ) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let mut builder = AuditLogEntry::builder(OperationType::ExecuteScript)
            .with_input_file(yaml_file.to_path_buf())
            .with_status(status);

        if let Some(err) = error {
            builder = builder.with_error(err);
        }

        let entry = builder.build();
        self.log_entry(entry)
    }

    pub fn log_hydrate_yaml(
        &self,
        yaml_file: &Path,
        export_file: &Path,
        output_file: Option<&Path>,
        status: OperationStatus,
        error: Option<String>,
    ) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let mut builder = AuditLogEntry::builder(OperationType::HydrateYaml)
            .with_input_file(yaml_file.to_path_buf())
            .with_input_file(export_file.to_path_buf())
            .with_status(status);

        if let Some(output) = output_file {
            builder = builder.with_output_file(output.to_path_buf());
        }

        if let Some(err) = error {
            builder = builder.with_error(err);
        }

        let entry = builder.build();
        self.log_entry(entry)
    }

    pub fn log_validate_yaml(
        &self,
        yaml_file: &Path,
        status: OperationStatus,
        error: Option<String>,
    ) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let mut builder = AuditLogEntry::builder(OperationType::ValidateYaml)
            .with_input_file(yaml_file.to_path_buf())
            .with_status(status);

        if let Some(err) = error {
            builder = builder.with_error(err);
        }

        let entry = builder.build();
        self.log_entry(entry)
    }

    pub fn save(&self) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        if let Some(ref path) = self.log_file {
            if let Ok(mut log) = self.log.lock() {
                log.save_to_file(path)?;
            }
        }

        Ok(())
    }

    pub fn get_log(&self) -> Option<AuditLog> {
        if let Ok(log) = self.log.lock() {
            Some(log.clone())
        } else {
            None
        }
    }
}

thread_local! {
    static GLOBAL_AUDIT_LOGGER: Arc<Mutex<AuditLogger>> = {
        let log_path = std::env::var("AUDIT_LOG_FILE")
            .unwrap_or_else(|_| "audit.log.json".to_string());
        Arc::new(Mutex::new(AuditLogger::with_file(&log_path)))
    };
}

pub fn get_global_logger() -> Arc<Mutex<AuditLogger>> {
    GLOBAL_AUDIT_LOGGER.with(Arc::clone)
}

pub fn log_operation(entry: AuditLogEntry) -> Result<()> {
    let logger = get_global_logger();
    if let Ok(audit) = logger.lock() {
        audit.log_entry(entry)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_audit_logger_creation() {
        let logger = AuditLogger::new();
        assert!(logger.is_enabled());
    }

    #[test]
    fn test_audit_logger_with_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let logger = AuditLogger::with_file(temp_file.path());
        assert!(logger.is_enabled());
    }

    #[test]
    fn test_audit_logger_enable_disable() {
        let mut logger = AuditLogger::new();
        assert!(logger.is_enabled());
        
        logger.disable();
        assert!(!logger.is_enabled());
        
        logger.enable();
        assert!(logger.is_enabled());
    }

    #[test]
    fn test_log_generate_script() {
        let temp_file = NamedTempFile::new().unwrap();
        let logger = AuditLogger::with_file(temp_file.path());

        let yaml_path = PathBuf::from("/tmp/test.yaml");
        logger.log_generate_script(
            &yaml_path,
            Some(&PathBuf::from("/tmp/test.sh")),
            OperationStatus::Success,
            None,
        ).unwrap();

        let log = logger.get_log().unwrap();
        assert_eq!(log.entries.len(), 1);
        assert_eq!(log.entries[0].operation, OperationType::GenerateScript);
    }

    #[test]
    fn test_audit_logger_disabled() {
        let mut logger = AuditLogger::new();
        logger.disable();

        let yaml_path = PathBuf::from("/tmp/test.yaml");
        logger.log_generate_script(
            &yaml_path,
            None,
            OperationStatus::Success,
            None,
        ).unwrap();

        let log = logger.get_log().unwrap();
        assert_eq!(log.entries.len(), 0);
    }
}
