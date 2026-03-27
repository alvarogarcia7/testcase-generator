use anyhow::{Context, Result};
use chrono::Utc;
use p521::ecdsa::{Signature, SigningKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

use crate::audit_log::AuditLog;
use crate::signing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedAuditLog {
    pub audit_log: AuditLog,
    pub log_hash: String,
    pub signature: String,
    pub public_key: String,
    pub key_id: String,
    pub signed_at: String,
}

impl SignedAuditLog {
    pub fn sign_log(
        log: AuditLog,
        private_key: &SigningKey,
        key_id: String,
    ) -> Result<Self> {
        let log_json = serde_json::to_string(&log)
            .context("Failed to serialize audit log")?;

        let mut hasher = Sha256::new();
        hasher.update(log_json.as_bytes());
        let log_hash_bytes = hasher.finalize();
        let log_hash = format!("{:x}", log_hash_bytes);

        let signature_bytes = signing::sign_message(private_key, &log_hash_bytes);
        let signature = hex::encode(signature_bytes);

        let public_key = signing::get_public_key(private_key);
        let public_key_pem = signing::public_key_to_pem(&public_key);

        Ok(Self {
            audit_log: log,
            log_hash,
            signature,
            public_key: public_key_pem,
            key_id,
            signed_at: Utc::now().to_rfc3339(),
        })
    }

    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)
            .context("Failed to serialize signed audit log")?;
        fs::write(path, json)
            .context(format!("Failed to write signed audit log to: {}", path.display()))?;
        Ok(())
    }

    pub fn load_from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .context(format!("Failed to read signed audit log from: {}", path.display()))?;
        let signed_log: SignedAuditLog = serde_json::from_str(&content)
            .context("Failed to parse signed audit log JSON")?;
        Ok(signed_log)
    }

    pub fn verify_signature(&self) -> Result<bool> {
        let public_key = signing::public_key_from_pem(&self.public_key)
            .context("Failed to parse public key from PEM")?;

        let log_json = serde_json::to_string(&self.audit_log)
            .context("Failed to serialize audit log for verification")?;

        let mut hasher = Sha256::new();
        hasher.update(log_json.as_bytes());
        let computed_hash_bytes = hasher.finalize();
        let computed_hash = format!("{:x}", computed_hash_bytes);

        if computed_hash != self.log_hash {
            log::error!("Hash mismatch: computed '{}', stored '{}'", computed_hash, self.log_hash);
            return Ok(false);
        }

        let signature_bytes = hex::decode(&self.signature)
            .context("Failed to decode signature from hex")?;

        let signature = Signature::from_slice(&signature_bytes)
            .context("Failed to parse signature")?;

        use p521::ecdsa::signature::Verifier;
        match public_key.verify(&computed_hash_bytes, &signature) {
            Ok(_) => Ok(true),
            Err(e) => {
                log::error!("Signature verification failed: {}", e);
                Ok(false)
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureVerificationReport {
    pub is_valid: bool,
    pub log_hash_verified: bool,
    pub signature_verified: bool,
    pub key_id: String,
    pub signed_at: String,
    pub verified_at: String,
    pub total_entries: usize,
    pub errors: Vec<String>,
}

impl SignatureVerificationReport {
    pub fn verify(signed_log: &SignedAuditLog) -> Self {
        let mut errors = Vec::new();
        let verified_at = Utc::now().to_rfc3339();

        let log_json = match serde_json::to_string(&signed_log.audit_log) {
            Ok(json) => json,
            Err(e) => {
                errors.push(format!("Failed to serialize audit log: {}", e));
                return Self {
                    is_valid: false,
                    log_hash_verified: false,
                    signature_verified: false,
                    key_id: signed_log.key_id.clone(),
                    signed_at: signed_log.signed_at.clone(),
                    verified_at,
                    total_entries: signed_log.audit_log.entries.len(),
                    errors,
                };
            }
        };

        let mut hasher = Sha256::new();
        hasher.update(log_json.as_bytes());
        let computed_hash = format!("{:x}", hasher.finalize());

        let log_hash_verified = computed_hash == signed_log.log_hash;
        if !log_hash_verified {
            errors.push(format!(
                "Log hash mismatch: computed '{}', stored '{}'",
                computed_hash, signed_log.log_hash
            ));
        }

        let signature_verified = match signed_log.verify_signature() {
            Ok(valid) => {
                if !valid {
                    errors.push("Signature verification failed".to_string());
                }
                valid
            }
            Err(e) => {
                errors.push(format!("Signature verification error: {}", e));
                false
            }
        };

        Self {
            is_valid: log_hash_verified && signature_verified,
            log_hash_verified,
            signature_verified,
            key_id: signed_log.key_id.clone(),
            signed_at: signed_log.signed_at.clone(),
            verified_at,
            total_entries: signed_log.audit_log.entries.len(),
            errors,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit_log::{AuditLogEntry, OperationType};
    use tempfile::NamedTempFile;

    #[test]
    fn test_sign_and_verify_audit_log() {
        let mut log = AuditLog::new();
        let entry = AuditLogEntry::builder(OperationType::GenerateScript)
            .build();
        log.entries.push(entry);

        let private_key = signing::generate_private_key();
        let signed_log = SignedAuditLog::sign_log(
            log,
            &private_key,
            "test-key".to_string(),
        ).unwrap();

        assert!(signed_log.verify_signature().unwrap());
    }

    #[test]
    fn test_signed_log_save_and_load() {
        let mut log = AuditLog::new();
        let entry = AuditLogEntry::builder(OperationType::ExecuteScript)
            .build();
        log.entries.push(entry);

        let private_key = signing::generate_private_key();
        let signed_log = SignedAuditLog::sign_log(
            log,
            &private_key,
            "test-key".to_string(),
        ).unwrap();

        let temp_file = NamedTempFile::new().unwrap();
        signed_log.save_to_file(temp_file.path()).unwrap();

        let loaded = SignedAuditLog::load_from_file(temp_file.path()).unwrap();
        assert!(loaded.verify_signature().unwrap());
        assert_eq!(loaded.key_id, "test-key");
    }

    #[test]
    fn test_verification_report() {
        let mut log = AuditLog::new();
        let entry = AuditLogEntry::builder(OperationType::ValidateYaml)
            .build();
        log.entries.push(entry);

        let private_key = signing::generate_private_key();
        let signed_log = SignedAuditLog::sign_log(
            log,
            &private_key,
            "test-key".to_string(),
        ).unwrap();

        let report = SignatureVerificationReport::verify(&signed_log);
        assert!(report.is_valid);
        assert!(report.log_hash_verified);
        assert!(report.signature_verified);
        assert_eq!(report.total_entries, 1);
        assert!(report.errors.is_empty());
    }

    #[test]
    fn test_tampered_log_detection() {
        let mut log = AuditLog::new();
        let entry = AuditLogEntry::builder(OperationType::GenerateScript)
            .build();
        log.entries.push(entry);

        let private_key = signing::generate_private_key();
        let mut signed_log = SignedAuditLog::sign_log(
            log,
            &private_key,
            "test-key".to_string(),
        ).unwrap();

        signed_log.audit_log.entries.push(
            AuditLogEntry::builder(OperationType::ExecuteScript).build()
        );

        assert!(!signed_log.verify_signature().unwrap());
    }
}
