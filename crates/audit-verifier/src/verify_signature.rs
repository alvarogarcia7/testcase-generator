use anyhow::{Context, Result};
use p521::ecdsa::{signature::Verifier, Signature};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct SignedAuditOutput {
    pub verification_result: VerificationResult,
    pub execution_log_sha256: String,
    pub signature: String,
    pub public_key: String,
    pub key_id: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VerificationResult {
    pub computed_hash: String,
    pub total_entries: usize,
    pub hash_mismatches: usize,
    pub missing_hash_fields: usize,
    pub verification_passed: bool,
}

pub fn verify_signed_audit(signed_output_path: &Path) -> Result<bool> {
    let content = fs::read_to_string(signed_output_path).context(format!(
        "Failed to read signed output file: {}",
        signed_output_path.display()
    ))?;

    let signed_output: SignedAuditOutput =
        serde_json::from_str(&content).context("Failed to parse signed output JSON")?;

    verify_signature(&signed_output)
}

pub fn verify_signature(signed_output: &SignedAuditOutput) -> Result<bool> {
    let public_key = crate::signing::public_key_from_pem(&signed_output.public_key)
        .context("Failed to parse public key PEM")?;

    let signature_bytes =
        hex::decode(&signed_output.signature).context("Failed to decode signature hex")?;

    let signature = Signature::from_slice(&signature_bytes).context("Failed to parse signature")?;

    let verification_json = serde_json::to_string(&signed_output.verification_result)
        .context("Failed to serialize verification result")?;

    let mut hasher = Sha256::new();
    hasher.update(verification_json.as_bytes());
    let message_hash = hasher.finalize();

    match public_key.verify(&message_hash, &signature) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signing;

    #[test]
    fn test_verify_valid_signature() {
        let private_key = signing::generate_private_key();
        let public_key = signing::get_public_key(&private_key);
        let public_key_pem = signing::public_key_to_pem(&public_key);

        let verification_result = VerificationResult {
            computed_hash: "abc123".to_string(),
            total_entries: 10,
            hash_mismatches: 0,
            missing_hash_fields: 0,
            verification_passed: true,
        };

        let verification_json = serde_json::to_string(&verification_result).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(verification_json.as_bytes());
        let message_hash = hasher.finalize();

        let signature = signing::sign_message(&private_key, &message_hash);
        let signature_hex = hex::encode(signature);

        let signed_output = SignedAuditOutput {
            verification_result,
            execution_log_sha256: "def456".to_string(),
            signature: signature_hex,
            public_key: public_key_pem,
            key_id: "test-key".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
        };

        let is_valid = verify_signature(&signed_output).unwrap();
        assert!(is_valid, "Signature should be valid");
    }

    #[test]
    fn test_verify_invalid_signature() {
        let private_key = signing::generate_private_key();
        let public_key = signing::get_public_key(&private_key);
        let public_key_pem = signing::public_key_to_pem(&public_key);

        let verification_result = VerificationResult {
            computed_hash: "abc123".to_string(),
            total_entries: 10,
            hash_mismatches: 0,
            missing_hash_fields: 0,
            verification_passed: true,
        };

        // Create a valid signature but then modify the data to make verification fail
        let verification_json = serde_json::to_string(&verification_result).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(verification_json.as_bytes());
        let message_hash = hasher.finalize();

        let signature = signing::sign_message(&private_key, &message_hash);
        let signature_hex = hex::encode(signature);

        // Modify the verification result to make the signature invalid
        let mut modified_verification_result = verification_result;
        modified_verification_result.computed_hash = "different_hash".to_string();

        let signed_output = SignedAuditOutput {
            verification_result: modified_verification_result,
            execution_log_sha256: "def456".to_string(),
            signature: signature_hex,
            public_key: public_key_pem,
            key_id: "test-key".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
        };

        let is_valid = verify_signature(&signed_output).unwrap();
        assert!(!is_valid, "Invalid signature should not verify");
    }
}
