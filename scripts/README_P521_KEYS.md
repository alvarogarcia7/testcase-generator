# NIST P-521 Key Management Scripts

This directory contains scripts for generating, loading, and verifying NIST P-521 elliptic curve cryptographic keys using OpenSSL.

## Overview

NIST P-521 (also known as secp521r1) is a standardized elliptic curve providing approximately 256 bits of security. These scripts provide a complete workflow for:

- **Generating** new P-521 key pairs
- **Loading** and validating existing keys
- **Verifying** key pair consistency and performing cryptographic operations

## Scripts

### 1. `generate_p521_keys.sh`

Generates new NIST P-521 elliptic curve key pairs.

#### Usage

```bash
./scripts/generate_p521_keys.sh [OPTIONS]
```

#### Options

- `-o, --output-dir DIR` - Output directory for keys (default: `./keys`)
- `-n, --name NAME` - Key name prefix (default: `p521_key`)
- `-f, --format FORMAT` - Output format: `pem` or `der` (default: `pem`)
- `-p, --password` - Encrypt private key with password (interactive)
- `-h, --help` - Show help message

#### Examples

```bash
# Generate keys with default settings
./scripts/generate_p521_keys.sh

# Generate keys in a specific directory
./scripts/generate_p521_keys.sh --output-dir /path/to/keys

# Generate keys with a custom name
./scripts/generate_p521_keys.sh --name my_server_key

# Generate password-protected keys
./scripts/generate_p521_keys.sh --password

# Generate keys in DER format
./scripts/generate_p521_keys.sh --format der

# Combined options
./scripts/generate_p521_keys.sh --output-dir ./secure_keys --name production_key --password
```

#### Output Files

The script generates four files:

- `<name>_private.pem|der` - Private key
- `<name>_public.pem|der` - Public key  
- `<name>_params.pem|der` - EC parameters file
- `<name>.info` - Key information and fingerprint

### 2. `load_p521_keys.sh`

Loads and validates existing NIST P-521 keys from files.

#### Usage

```bash
./scripts/load_p521_keys.sh [OPTIONS]
```

#### Options

- `-k, --private-key FILE` - Path to private key file (required)
- `-p, --public-key FILE` - Path to public key file (optional)
- `-f, --format FORMAT` - Key format: `pem` or `der` (default: `pem`)
- `-v, --verify` - Verify key pair consistency
- `-i, --info` - Display detailed key information
- `-e, --extract-public` - Extract public key from private key
- `-o, --output FILE` - Output file for extracted public key
- `-h, --help` - Show help message

#### Examples

```bash
# Load and display private key information
./scripts/load_p521_keys.sh --private-key keys/p521_key_private.pem --info

# Verify key pair consistency
./scripts/load_p521_keys.sh \
  --private-key keys/p521_key_private.pem \
  --public-key keys/p521_key_public.pem \
  --verify

# Extract public key from private key
./scripts/load_p521_keys.sh \
  --private-key keys/p521_key_private.pem \
  --extract-public \
  --output new_public.pem

# Load DER format keys
./scripts/load_p521_keys.sh \
  --private-key keys/p521_key_private.der \
  --format der \
  --info

# Verify DER format key pair
./scripts/load_p521_keys.sh \
  --private-key keys/p521_key_private.der \
  --public-key keys/p521_key_public.der \
  --format der \
  --verify
```

### 3. `verify_p521_keys.sh`

Verifies NIST P-521 key pairs and performs cryptographic operations.

#### Usage

```bash
./scripts/verify_p521_keys.sh [OPTIONS]
```

#### Options

- `-k, --private-key FILE` - Path to private key file (required)
- `-p, --public-key FILE` - Path to public key file (required)
- `-f, --format FORMAT` - Key format: `pem` or `der` (default: `pem`)
- `-s, --sign` - Perform signature test
- `-d, --data FILE` - File to sign (default: generated test data)
- `-h, --help` - Show help message

#### Examples

```bash
# Verify key pair consistency
./scripts/verify_p521_keys.sh \
  --private-key keys/p521_key_private.pem \
  --public-key keys/p521_key_public.pem

# Verify with signature test
./scripts/verify_p521_keys.sh \
  --private-key keys/p521_key_private.pem \
  --public-key keys/p521_key_public.pem \
  --sign

# Verify and sign specific file
./scripts/verify_p521_keys.sh \
  --private-key keys/p521_key_private.pem \
  --public-key keys/p521_key_public.pem \
  --sign \
  --data myfile.txt

# Verify DER format keys
./scripts/verify_p521_keys.sh \
  --private-key keys/p521_key_private.der \
  --public-key keys/p521_key_public.der \
  --format der \
  --sign
```

## Typical Workflows

### Workflow 1: Generate and Verify New Keys

```bash
# Step 1: Generate keys
./scripts/generate_p521_keys.sh --output-dir ./my_keys --name my_app

# Step 2: Verify the generated keys
./scripts/verify_p521_keys.sh \
  --private-key ./my_keys/my_app_private.pem \
  --public-key ./my_keys/my_app_public.pem \
  --sign

# Step 3: View key details
./scripts/load_p521_keys.sh \
  --private-key ./my_keys/my_app_private.pem \
  --info
```

### Workflow 2: Load and Validate Existing Keys

```bash
# Step 1: Load and check key format
./scripts/load_p521_keys.sh \
  --private-key existing_private.pem \
  --info

# Step 2: Extract public key if not available
./scripts/load_p521_keys.sh \
  --private-key existing_private.pem \
  --extract-public \
  --output extracted_public.pem

# Step 3: Verify key pair
./scripts/verify_p521_keys.sh \
  --private-key existing_private.pem \
  --public-key extracted_public.pem \
  --sign
```

### Workflow 3: Generate Secure Production Keys

```bash
# Generate password-protected keys
./scripts/generate_p521_keys.sh \
  --output-dir ./production_keys \
  --name prod_server \
  --password

# Verify keys work correctly (will prompt for password)
./scripts/verify_p521_keys.sh \
  --private-key ./production_keys/prod_server_private.pem \
  --public-key ./production_keys/prod_server_public.pem \
  --sign

# Store keys securely
chmod 600 ./production_keys/prod_server_private.pem
chmod 644 ./production_keys/prod_server_public.pem
```

### Workflow 4: Convert Between PEM and DER Formats

```bash
# Generate in PEM format
./scripts/generate_p521_keys.sh --name test_key

# Convert to DER using OpenSSL directly
openssl ec -in keys/test_key_private.pem -outform DER -out keys/test_key_private.der
openssl ec -in keys/test_key_public.pem -pubin -outform DER -out keys/test_key_public.der

# Or generate directly in DER format
./scripts/generate_p521_keys.sh --name test_key_der --format der

# Verify DER format keys
./scripts/verify_p521_keys.sh \
  --private-key keys/test_key_der_private.der \
  --public-key keys/test_key_der_public.der \
  --format der
```

## Key Formats

### PEM Format (Default)

- **Human-readable**: Base64-encoded with header/footer
- **Extension**: `.pem`
- **Usage**: Most common, works with most tools
- **Example**:
  ```
  -----BEGIN EC PRIVATE KEY-----
  MIHcAgEBBEIB...
  -----END EC PRIVATE KEY-----
  ```

### DER Format

- **Binary**: Distinguished Encoding Rules
- **Extension**: `.der`
- **Usage**: Compact binary format, often used in certificates
- **Not human-readable**

## Security Considerations

### Private Key Protection

1. **Use password encryption** for production keys:
   ```bash
   ./scripts/generate_p521_keys.sh --password
   ```

2. **Set appropriate file permissions**:
   ```bash
   chmod 600 private_key.pem  # Owner read/write only
   chmod 644 public_key.pem   # World readable
   ```

3. **Store keys securely**:
   - Use encrypted filesystems
   - Consider hardware security modules (HSM)
   - Use key management systems (KMS)
   - Never commit private keys to version control

4. **Rotate keys regularly**:
   - Generate new keys periodically
   - Maintain key rotation schedule
   - Archive old keys securely

### Best Practices

- ✅ Always verify key pairs after generation
- ✅ Test signing/verification before deployment
- ✅ Use password protection for sensitive keys
- ✅ Store private keys separately from public keys
- ✅ Backup keys to secure, encrypted storage
- ✅ Document key usage and lifecycle
- ❌ Never share private keys
- ❌ Never transmit private keys over insecure channels
- ❌ Never log or print private key contents

## Troubleshooting

### Encrypted Key Issues

If you get errors about encrypted keys:

```bash
# For load_p521_keys.sh
# The script will detect encryption but cannot display details
# You'll need to provide the password when OpenSSL prompts

# For verify_p521_keys.sh
# Ensure you can decrypt the key first:
openssl ec -in encrypted_key.pem -text -noout
# Then run the verification
```

### Format Detection

Keys must match the specified format:

```bash
# Check if a key is PEM format
head -n 1 key.pem
# Should show: -----BEGIN EC PRIVATE KEY----- or similar

# Check if a key is DER format  
file key.der
# Should show: data or DER encoded

# Convert between formats:
openssl ec -in key.pem -outform DER -out key.der
openssl ec -in key.der -inform DER -outform PEM -out key.pem
```

### Key Pair Mismatch

If verification fails:

```bash
# Extract public key from private key
./scripts/load_p521_keys.sh \
  --private-key private.pem \
  --extract-public \
  --output correct_public.pem

# Verify the extracted key
./scripts/verify_p521_keys.sh \
  --private-key private.pem \
  --public-key correct_public.pem
```

### Wrong Curve

Ensure keys use secp521r1 (P-521):

```bash
# Check the curve
openssl ec -in key.pem -text -noout | grep "ASN1 OID"
# Should show: ASN1 OID: secp521r1

# Generate with correct curve (scripts do this automatically)
openssl ecparam -name secp521r1 -genkey -out key.pem
```

## Requirements

- **OpenSSL**: Version 1.0.2 or later
  - Check version: `openssl version`
  - P-521 support: `openssl ecparam -list_curves | grep secp521r1`

- **Bash**: Version 3.2 or later
  - Check version: `bash --version`

- **Standard Unix tools**: `mktemp`, `cmp`, `awk`, `sed`

## Technical Details

### NIST P-521 Curve

- **Full Name**: NIST P-521 / secp521r1
- **Security Level**: ~256 bits
- **Key Size**: 521 bits (not 512!)
- **Standardized**: FIPS 186-4, SEC 2
- **Use Cases**: High-security applications, long-term data protection

### Cryptographic Operations

These scripts use:

- **Key Generation**: OpenSSL EC parameter generation
- **Signature Algorithm**: ECDSA with SHA-512
- **Key Derivation**: Standard EC point operations
- **Encryption**: AES-256 for password-protected keys

## References

- [NIST SP 800-186](https://csrc.nist.gov/publications/detail/sp/800-186/final) - Elliptic Curve Cryptography
- [SEC 2](http://www.secg.org/sec2-v2.pdf) - Recommended Elliptic Curve Domain Parameters
- [OpenSSL EC Documentation](https://www.openssl.org/docs/man1.1.1/man1/ec.html)
- [RFC 5480](https://tools.ietf.org/html/rfc5480) - ECC SubjectPublicKeyInfo Format

## License

These scripts are part of the testcase-generator project and follow the same license terms.
