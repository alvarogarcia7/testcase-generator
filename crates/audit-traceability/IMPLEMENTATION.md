# Audit Traceability Implementation Details

## Architecture

The audit traceability system is implemented as a standalone Rust crate (`audit-traceability`) that provides:

1. **Core data structures** for representing audit logs
2. **File hashing utilities** using SHA-256
3. **Verification logic** to check file integrity
4. **Serialization/deserialization** for JSON storage

## Core Types

### `AuditTraceabilityLog`

The top-level structure that represents the entire audit log.

```rust
pub struct AuditTraceabilityLog {
    pub date: DateTime<Utc>,
    pub witness_key: String,
    pub test_cases: HashMap<String, TestCaseAudit>,
}
```

**Fields:**
- `date`: Timestamp when the log was created (UTC)
- `witness_key`: Identifier for the creating authority
- `test_cases`: Map of test case ID to audit information

**Methods:**
- `new(witness_key: String) -> Self`: Create a new audit log
- `add_test_case(&mut self, tc_id: String, audit: TestCaseAudit)`: Add a test case
- `get_test_case(&self, tc_id: &str) -> Option<&TestCaseAudit>`: Retrieve a test case
- `save_to_file(&self, path: impl AsRef<Path>) -> Result<()>`: Save to JSON file
- `load_from_file(path: impl AsRef<Path>) -> Result<Self>`: Load from JSON file
- `verify_test_case(&self, tc_id: &str) -> Result<VerificationResult>`: Verify a test case
- `verify_all(&self) -> Result<Vec<VerificationResult>>`: Verify all test cases

### `TestCaseAudit`

Represents audit information for a single test case.

```rust
pub struct TestCaseAudit {
    pub stages: HashMap<String, StageInfo>,
}
```

**Fields:**
- `stages`: Map of stage name to stage information

**Methods:**
- `new() -> Self`: Create a new test case audit
- `add_stage(&mut self, stage_name: String, stage_info: StageInfo)`: Add a stage
- `get_stage(&self, stage_name: &str) -> Option<&StageInfo>`: Retrieve a stage
- `verify_stage(&self, stage_name: &str) -> Result<bool>`: Verify a specific stage

### `StageInfo`

Represents information about a single processing stage.

```rust
pub struct StageInfo {
    pub path: PathBuf,
    pub sha256: String,
}
```

**Fields:**
- `path`: File path (relative or absolute)
- `sha256`: SHA-256 hash of the file content (hex-encoded)

**Methods:**
- `new(path: PathBuf, sha256: String) -> Self`: Create stage info manually
- `from_file(path: impl AsRef<Path>) -> Result<Self>`: Create from a file (reads and hashes)

### `VerificationResult`

Represents the result of verifying a test case.

```rust
pub struct VerificationResult {
    pub test_case_id: String,
    pub all_passed: bool,
    pub stage_results: Vec<StageVerificationResult>,
}
```

**Methods:**
- `print_summary(&self)`: Print a formatted summary

### `StageVerificationResult`

Represents the result of verifying a single stage.

```rust
pub struct StageVerificationResult {
    pub stage_name: String,
    pub passed: bool,
    pub message: String,
}
```

## Cryptographic Hashing

The system uses SHA-256 for file integrity verification:

```rust
pub fn compute_sha256(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    format!("{:x}", hasher.finalize())
}
```

**Properties:**
- **Cryptographically secure**: SHA-256 is resistant to collision attacks
- **Deterministic**: Same content always produces the same hash
- **Fast**: Efficient for most file sizes
- **Standard**: Widely recognized and supported

## File Operations

### Reading and Hashing

When creating a `StageInfo` from a file:

1. Read entire file contents into memory
2. Compute SHA-256 hash of the bytes
3. Store the path and hash

```rust
impl StageInfo {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let content = fs::read(&path)?;
        let sha256 = compute_sha256(&content);
        Ok(Self::new(path, sha256))
    }
}
```

### Verification

When verifying a file:

1. Read the current file contents
2. Compute the current SHA-256 hash
3. Compare with the recorded hash

```rust
pub fn verify_file(path: &Path, expected_hash: &str) -> Result<bool> {
    if !path.exists() {
        bail!("File not found: {}", path.display());
    }
    let content = fs::read(path)?;
    let computed_hash = compute_sha256(&content);
    Ok(computed_hash == expected_hash)
}
```

## JSON Serialization

The audit log is serialized to JSON using `serde_json`:

```json
{
  "date": "2024-01-15T10:30:00Z",
  "witness_key": "system-id",
  "test_cases": {
    "TC001": {
      "stages": {
        "initial": {
          "path": "file.yaml",
          "sha256": "hash..."
        }
      }
    }
  }
}
```

**Benefits:**
- Human-readable
- Machine-parsable
- Compact
- Version control friendly
- Widely supported

## Error Handling

The crate uses `anyhow::Result` for error handling:

- File I/O errors are propagated with context
- Missing files produce clear error messages
- Hash mismatches return `Ok(false)` or detailed results

Example error context:

```rust
let content = fs::read(&path).context(format!(
    "Failed to read file: {}",
    path.display()
))?;
```

## CLI Integration

The `test-executor` binary integrates the audit traceability functionality:

### Commands

1. **`audit-log create`**: Create a new audit log
2. **`audit-log add`**: Add/update a test case in the log
3. **`audit-log verify`**: Verify files against the log
4. **`generate --audit-log`**: Generate script and update audit log

### Implementation

The CLI commands are implemented in `crates/test-executor/src/main.rs`:

```rust
Commands::AuditLog { command } => match command {
    AuditLogCommands::Create { output, witness_key } => {
        let log = AuditTraceabilityLog::new(witness_key);
        log.save_to_file(&output)?;
        println!("✓ Audit traceability log created: {}", output.display());
        Ok(())
    }
    // ... other commands
}
```

## Testing

The crate includes comprehensive unit tests:

- Hash computation verification
- Stage info creation from files
- Audit log save/load cycle
- Verification logic
- Error handling

Run tests with:

```bash
cargo test -p audit-traceability
```

## Performance Considerations

### File Hashing

- **Time complexity**: O(n) where n is file size
- **Space complexity**: O(1) for hashing, O(n) for reading
- **Optimization**: Files are read once when creating stage info

### Verification

- **Parallel verification**: Could be added for multiple test cases
- **Incremental updates**: Only hash changed files
- **Caching**: Consider caching hashes in memory for repeated verifications

### Large Files

For very large files:

- Current implementation reads entire file into memory
- Could be optimized to stream read for files > 100MB
- SHA-256 supports incremental hashing

## Security Properties

### Tamper Detection

- Any modification to tracked files changes the SHA-256 hash
- Hash collision probability: ~2^-256 (practically impossible)
- Modifications are detected with certainty

### Audit Trail

- Timestamp provides temporal reference
- Witness key identifies the creating authority
- Immutable record of file states

### Limitations

- **Not a signature**: Audit log itself can be modified
- **No authentication**: Anyone can create/modify audit logs
- **No encryption**: File contents and paths are readable

### Future Security Enhancements

1. **Digital signatures**: Sign audit logs with private key
2. **Merkle trees**: Hierarchical hashing for efficient partial verification
3. **Blockchain**: Immutable distributed ledger for audit logs
4. **Encryption**: Encrypt sensitive file paths or metadata

## Dependencies

- `anyhow`: Error handling
- `serde`: Serialization framework
- `serde_json`: JSON support
- `chrono`: Date/time handling
- `sha2`: SHA-256 implementation
- `log`: Logging facade

## Future Enhancements

### Planned Features

1. **Incremental updates**: Only re-hash changed files
2. **Parallel verification**: Verify multiple test cases concurrently
3. **Diff support**: Show what changed between audit log versions
4. **Merge support**: Combine audit logs from different sources
5. **Streaming hashing**: Support very large files efficiently
6. **Multiple hash algorithms**: Support SHA-512, BLAKE3, etc.
7. **Compression**: Compress audit logs for storage efficiency

### API Extensions

```rust
// Potential future APIs
impl AuditTraceabilityLog {
    // Merge two audit logs
    pub fn merge(&mut self, other: &AuditTraceabilityLog) -> Result<()>;
    
    // Get diff between two logs
    pub fn diff(&self, other: &AuditTraceabilityLog) -> AuditDiff;
    
    // Verify only changed files
    pub fn verify_incremental(&self, since: &DateTime<Utc>) -> Result<Vec<VerificationResult>>;
    
    // Sign the audit log
    pub fn sign(&mut self, private_key: &PrivateKey) -> Result<()>;
    
    // Verify signature
    pub fn verify_signature(&self, public_key: &PublicKey) -> Result<bool>;
}
```

## Contributing

When contributing to the audit traceability system:

1. Maintain backward compatibility with existing JSON format
2. Add tests for new functionality
3. Update documentation
4. Consider security implications
5. Follow Rust best practices

## References

- [SHA-256 Specification](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf)
- [JSON Schema](https://json-schema.org/)
- [Serde Documentation](https://serde.rs/)
- [Anyhow Error Handling](https://docs.rs/anyhow/)
