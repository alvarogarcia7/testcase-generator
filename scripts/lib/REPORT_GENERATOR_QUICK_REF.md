# Report Generator Library - Quick Reference

## Error Handling Features

### Exit Codes

| Code | Meaning | Retry? | Common Cause |
|------|---------|--------|--------------|
| 0 | Success | - | Report generated successfully |
| 1 | General error | No | File not found, YAML parsing error |
| 2 | Invalid args | No | Missing --output or --test-case |
| 101 | I/O error | Yes | Disk full, permissions, temp issue |
| 130 | Interrupted | No | User pressed Ctrl+C |

### Key Functions

#### Invoke (Single Attempt)
```bash
invoke_test_plan_doc_gen --test-case "test.yaml" --output "out.md" --format markdown
# Returns: tpdg exit code
```

#### Invoke (With Retry)
```bash
invoke_test_plan_doc_gen_with_retry --test-case "test.yaml" --output "out.md" --format markdown
# Returns: 0 on success, 1 on failure (after retries)
```

#### Validate Output
```bash
validate_report_output "/path/to/output" "file1.md" "file2.adoc"
# Returns: 0 if all files exist and valid, 1 otherwise
```

#### Validate Content
```bash
validate_report_file_content "report.md" 100
# Returns: 0 if file >= 100 bytes with valid content, 1 otherwise
```

### Configuration

```bash
# Customize retry behavior
export TPDG_MAX_RETRIES=5      # Default: 3
export TPDG_RETRY_DELAY=3      # Default: 2 seconds

# Override binary location
export TEST_PLAN_DOC_GEN="/custom/path/to/tpdg"

# Enable verbose logging
export VERBOSE=1
```

### Usage Patterns

#### Pattern 1: Simple Generation
```bash
source scripts/lib/report_generator.sh

invoke_test_plan_doc_gen \
    --test-case "test.yaml" \
    --output "report.md" \
    --format markdown
```

#### Pattern 2: Production with Retry & Validation
```bash
source scripts/lib/report_generator.sh

if invoke_test_plan_doc_gen_with_retry \
    --test-case "test.yaml" \
    --output "report.md" \
    --format markdown; then
    
    validate_report_file_content "report.md" 500
fi
```

#### Pattern 3: Batch with Error Tracking
```bash
source scripts/lib/report_generator.sh

for tc in testcases/*.yml; do
    invoke_test_plan_doc_gen_with_retry \
        --test-case "$tc" \
        --output "reports/$(basename "$tc" .yml).md" \
        --format markdown || echo "Failed: $tc"
done
```

#### Pattern 4: Graceful Degradation
```bash
source scripts/lib/report_generator.sh

if check_test_plan_doc_gen_available; then
    invoke_test_plan_doc_gen_with_retry "$@" || true
else
    log_warning "Skipping reports (tpdg unavailable)"
fi
```

### Error Messages

#### Binary Not Found
```
[ERROR] test-plan-doc-gen binary not found
[ERROR] Please build it first using build_test_plan_doc_gen()
```

#### YAML Error
```
[ERROR] test-plan-doc-gen failed with exit code: 1
[ERROR] Error: General error (file not found, parsing error, or invalid input)
[ERROR] Common causes:
[ERROR]   - YAML parsing error in input file
```

#### I/O Error (Retrying)
```
[WARNING] Transient error detected (exit code: 101)
[INFO] Retrying in 2s... (attempt 2 of 3)
```

#### Invalid Output
```
[WARNING] Output file validation failed
[ERROR] File is too small (5 bytes, minimum: 10 bytes)
```

### Quick Troubleshooting

**Problem:** Binary not found
```bash
# Solution: Build it
build_test_plan_doc_gen "../test-plan-doc-gen"
```

**Problem:** YAML parsing error
```bash
# Solution: Validate YAML
cargo run --bin test-case-manager -- validate test.yaml
```

**Problem:** I/O error
```bash
# Solution: Check permissions and space
ls -ld reports/
df -h
```

**Problem:** Empty output file
```bash
# Solution: Check tpdg output
VERBOSE=1 invoke_test_plan_doc_gen --test-case test.yaml --output out.md --format markdown
```

### Function Reference

| Function | Purpose | Returns |
|----------|---------|---------|
| `invoke_test_plan_doc_gen` | Single invocation | tpdg exit code |
| `invoke_test_plan_doc_gen_with_retry` | Invocation with retry | 0 or 1 |
| `validate_report_output` | Validate multiple files | 0 or 1 |
| `validate_report_file_content` | Validate single file | 0 or 1 |
| `build_test_plan_doc_gen` | Build tpdg binary | 0 or 1 |
| `check_test_plan_doc_gen_available` | Check availability | 0 or 1 |
| `find_test_plan_doc_gen` | Find binary path | 0 or 1 (prints path) |
| `verify_test_plan_doc_gen_binary` | Verify functionality | 0 or 1 |
| `get_tpdg_error_message` | Get error description | (prints message) |
| `is_transient_error` | Check if retryable | 0 or 1 |

### Best Practices

1. ✅ Use retry for production scripts
2. ✅ Validate output after generation
3. ✅ Check binary availability before use
4. ✅ Handle graceful degradation
5. ✅ Use verbose logging for debugging

### See Also

- [Full Documentation](README_REPORT_GENERATOR.md)
- [Logger Library](logger.sh)
- [Report Generation Guide](../../README_REPORT_GENERATION.md)
