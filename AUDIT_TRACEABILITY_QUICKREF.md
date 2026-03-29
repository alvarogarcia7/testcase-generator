# Audit Traceability Quick Reference

## Quick Start

```bash
# 1. Create audit log
test-executor audit-log create --output audit-log.json

# 2. Generate script with audit tracking
test-executor generate TC001.yaml -o TC001.sh --audit-log audit-log.json

# 3. Verify integrity
test-executor audit-log verify --log-file audit-log.json
```

## Common Commands

### Create New Audit Log

```bash
test-executor audit-log create \
  --output audit-log.json \
  --witness-key "my-system"
```

### Generate Script + Update Audit Log

```bash
test-executor generate testcases/TC001.yaml \
  --output output/TC001.sh \
  --audit-log audit-log.json
```

### Manually Add Test Case

```bash
test-executor audit-log add \
  --log-file audit-log.json \
  --test-case-id TC001 \
  --initial testcases/TC001.yaml \
  --shell-script output/TC001.sh
```

### Verify All Test Cases

```bash
test-executor audit-log verify --log-file audit-log.json
```

### Verify Specific Test Case

```bash
test-executor audit-log verify \
  --log-file audit-log.json \
  --test-case-id TC001
```

## Batch Processing

### Generate Multiple Scripts

```bash
for tc in testcases/*.yaml; do
  tc_id=$(basename "$tc" .yaml)
  test-executor generate "$tc" \
    --output "output/${tc_id}.sh" \
    --audit-log audit-log.json
done
```

### Verify Before Commit

```bash
if test-executor audit-log verify --log-file audit-log.json; then
  git add audit-log.json output/*.sh
  git commit -m "Generated scripts with audit trail"
else
  echo "Verification failed - files may have been tampered with"
  exit 1
fi
```

## JSON Structure

```json
{
  "date": "2024-01-15T10:30:00Z",
  "witness_key": "system-id",
  "test_cases": {
    "TC001": {
      "stages": {
        "initial": {
          "path": "testcases/TC001.yaml",
          "sha256": "abc123..."
        },
        "05_shell_script": {
          "path": "output/TC001.sh",
          "sha256": "def456..."
        }
      }
    }
  }
}
```

## Exit Codes

- `0`: Success / Verification passed
- `1`: Failure / Verification failed

## Common Stage Names

- `initial` - Original YAML file
- `05_shell_script` - Generated bash script
- `resolved` - Dependency-resolved YAML
- `execution_log` - Execution log JSON

## Tips

✓ Commit audit logs with generated files  
✓ Use descriptive witness keys  
✓ Verify before deployment  
✓ Re-generate to update hashes after intentional changes  
✗ Don't manually edit audit log JSON  
✗ Don't move files without updating the log  

## Error Messages

**"Stage verification failed"**: File was modified  
→ Re-generate with `--audit-log` to update

**"File not found"**: File was moved or deleted  
→ Update the audit log with correct path

**"Test case not found in audit log"**: Missing entry  
→ Use `audit-log add` to add the test case

## Integration Examples

### GitHub Actions

```yaml
- name: Generate scripts with audit
  run: |
    test-executor generate TC001.yaml \
      -o TC001.sh --audit-log audit-log.json
    
- name: Verify integrity
  run: test-executor audit-log verify --log-file audit-log.json
```

### GitLab CI

```yaml
generate:
  script:
    - test-executor generate TC001.yaml -o TC001.sh --audit-log audit-log.json
  artifacts:
    paths:
      - TC001.sh
      - audit-log.json

verify:
  script:
    - test-executor audit-log verify --log-file audit-log.json
```

### Makefile

```makefile
generate: audit-log.json
	test-executor generate TC001.yaml -o TC001.sh --audit-log $<

verify: audit-log.json
	test-executor audit-log verify --log-file $<

audit-log.json:
	test-executor audit-log create --output $@
```

## Library Usage (Rust)

```rust
use audit_traceability::{AuditTraceabilityLog, StageInfo, TestCaseAudit};

// Create and populate
let mut log = AuditTraceabilityLog::new("witness".into());
let mut audit = TestCaseAudit::new();
audit.add_stage("initial", StageInfo::from_file("TC001.yaml")?);
log.add_test_case("TC001", audit);
log.save_to_file("audit-log.json")?;

// Load and verify
let log = AuditTraceabilityLog::load_from_file("audit-log.json")?;
let result = log.verify_test_case("TC001")?;
assert!(result.all_passed);
```

## Documentation

- Full Guide: `AUDIT_TRACEABILITY_GUIDE.md`
- Implementation: `crates/audit-traceability/IMPLEMENTATION.md`
- Summary: `AUDIT_TRACEABILITY_SUMMARY.md`

## Support

Run tests: `cargo test -p audit-traceability`  
Run example: `cargo run --example audit_traceability_demo`  
Run demo: `./scripts/examples/audit_workflow_demo.sh`
