use anyhow::{Context, Result};
use p521::ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey};
use p521::SecretKey;
use rand_core::OsRng;
use std::fs;
use std::path::Path;

pub fn generate_private_key() -> SigningKey {
    SigningKey::random(&mut OsRng)
}

pub fn load_private_key(path: &Path) -> Result<SigningKey> {
    let pem_content = fs::read_to_string(path).context(format!(
        "Failed to read private key from: {}",
        path.display()
    ))?;

    let secret_key =
        SecretKey::from_sec1_pem(&pem_content).context("Failed to parse private key PEM")?;

    let signing_key_bytes = secret_key.to_bytes();
    let signing_key = SigningKey::from_bytes(&signing_key_bytes)
        .context("Failed to create signing key from bytes")?;

    Ok(signing_key)
}

pub fn save_private_key(key: &SigningKey, path: &Path) -> Result<()> {
    let key_bytes = key.to_bytes();
    let secret_key =
        SecretKey::from_bytes(&key_bytes).context("Failed to create secret key from bytes")?;
    let pem = secret_key
        .to_sec1_pem(Default::default())
        .context("Failed to encode private key to PEM")?;

    fs::write(path, pem.as_bytes()).context(format!(
        "Failed to write private key to: {}",
        path.display()
    ))?;

    Ok(())
}

pub fn get_public_key(private_key: &SigningKey) -> VerifyingKey {
    VerifyingKey::from(private_key)
}

pub fn public_key_to_pem(public_key: &VerifyingKey) -> String {
    let encoded_point = public_key.to_encoded_point(false);
    let pem_label = "PUBLIC KEY";
    let sec1_bytes = encoded_point.as_bytes();

    format!(
        "-----BEGIN {}-----\n{}\n-----END {}-----",
        pem_label,
        base64_encode_multiline(sec1_bytes),
        pem_label
    )
}

fn base64_encode_multiline(data: &[u8]) -> String {
    use base64ct::{Base64, Encoding};
    let encoded = Base64::encode_string(data);
    let mut result = String::new();
    for chunk in encoded.as_bytes().chunks(64) {
        result.push_str(std::str::from_utf8(chunk).unwrap());
        result.push('\n');
    }
    result.trim_end().to_string()
}

pub fn public_key_from_pem(pem: &str) -> Result<VerifyingKey> {
    let pem = pem.trim();
    let lines: Vec<&str> = pem.lines().collect();

    if lines.len() < 3 {
        anyhow::bail!("Invalid PEM format");
    }

    let base64_data: String = lines[1..lines.len() - 1].join("");

    use base64ct::{Base64, Encoding};
    let sec1_bytes = Base64::decode_vec(&base64_data).context("Failed to decode base64")?;

    let encoded_point =
        p521::EncodedPoint::from_bytes(&sec1_bytes).context("Failed to parse encoded point")?;

    VerifyingKey::from_encoded_point(&encoded_point)
        .context("Failed to create verifying key from encoded point")
}

pub fn sign_message(private_key: &SigningKey, message_hash: &[u8]) -> Vec<u8> {
    let signature: Signature = private_key.sign(message_hash);
    signature.to_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use p521::ecdsa::signature::Verifier;
    use tempfile::NamedTempFile;

    #[test]
    fn test_generate_and_sign() {
        let private_key = generate_private_key();
        let public_key = get_public_key(&private_key);

        let message = b"test message";
        let mut hasher = sha2::Sha256::new();
        use sha2::Digest;
        hasher.update(message);
        let hash = hasher.finalize();

        let signature_bytes = sign_message(&private_key, &hash);
        let signature = Signature::from_slice(&signature_bytes).unwrap();

        assert!(public_key.verify(&hash, &signature).is_ok());
    }

    #[test]
    fn test_save_and_load_key() {
        let original_key = generate_private_key();
        let temp_file = NamedTempFile::new().unwrap();

        save_private_key(&original_key, temp_file.path()).unwrap();
        let loaded_key = load_private_key(temp_file.path()).unwrap();

        let message = b"test message";
        let mut hasher = sha2::Sha256::new();
        use sha2::Digest;
        hasher.update(message);
        let hash = hasher.finalize();

        let sig1 = sign_message(&original_key, &hash);
        let public_key = get_public_key(&loaded_key);
        let signature = Signature::from_slice(&sig1).unwrap();

        assert!(public_key.verify(&hash, &signature).is_ok());
    }

    #[test]
    fn test_public_key_pem() {
        let private_key = generate_private_key();
        let public_key = get_public_key(&private_key);
        let pem = public_key_to_pem(&public_key);

        assert!(pem.starts_with("-----BEGIN PUBLIC KEY-----"));
        assert!(pem.contains("-----END PUBLIC KEY-----"));

        let recovered = public_key_from_pem(&pem).unwrap();
        assert_eq!(
            public_key.to_encoded_point(false).as_bytes(),
            recovered.to_encoded_point(false).as_bytes()
        );
    }
}
