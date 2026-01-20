# Test-Verify Quick Reference

## Quick Start

```bash
# Build
cargo build --release --bin test-verify

# Single test
test-verify single -l test.log -t TC001

# Batch verify
test-verify batch -l logs/*.log -f junit -o report.xml

# Run demo
cargo run --example test_verify_demo
```

## Commands

| Command | Description | Example |
|---------|-------------|---------|
| `single` | Verify one test | `test-verify single -l test.log -t TC001` |
| `batch` | Verify multiple tests | `test-verify batch -l logs/*.log` |
| `parse-log` | Parse log file | `test-verify parse-log -l test.log` |

## Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--log` | `-l` | Log file path | Required |
| `--logs` | `-l` | Multiple log files (batch) | Required |
| `--test-case-id` | `-t` | Test case ID (single) | Required |
| `--test-case-dir` | `-d` | Test case directory | `testcases` |
| `--format` | `-f` | Output format (text/json/junit) | `text` |
| `--output` | `-o` | Output file (batch) | stdout |

## Log Format

```
[TIMESTAMP] TestCase: <id>, Sequence: <seq_id>, Step: <step_num>, Success: <true/false/->, Result: <result>, Output: <output>
```

Example:
```
[2024-01-15T10:30:00Z] TestCase: TC001, Sequence: 1, Step: 1, Success: true, Result: SW=0x9000, Output: Success
```

## Pattern Matching

| Type | Format | Example |
|------|--------|---------|
| Exact | `text` | `SW=0x9000` |
| Wildcard | `*` | `SW=*` matches `SW=0x9000` |
| Regex | `/pattern/` | `/SW=0x[0-9A-F]{4}/` |

## Output Formats

### Text
```bash
test-verify batch -l logs/*.log -f text
```

### JSON
```bash
test-verify batch -l logs/*.log -f json -o report.json
```

### JUnit XML
```bash
test-verify batch -l logs/*.log -f junit -o junit.xml
```

## Exit Codes

- `0`: All tests passed
- `1`: One or more tests failed

## Common Use Cases

### Verify after test run
```bash
test-verify single -l latest.log -t MyTest
```

### CI/CD integration
```bash
test-verify batch -l results/*.log -f junit -o junit-report.xml
```

### Debug log format
```bash
test-verify parse-log -l test.log -f json
```

### Custom test case location
```bash
test-verify batch -l logs/*.log -d /path/to/testcases
```

## CI/CD Examples

### GitHub Actions
```yaml
- run: test-verify batch -l logs/*.log -f junit -o junit.xml
- uses: EnricoMi/publish-unit-test-result-action@v2
  with:
    files: junit.xml
```

### Jenkins
```groovy
sh 'test-verify batch -l logs/*.log -f junit -o junit.xml'
junit 'junit.xml'
```

## Files

- Documentation: `docs/TEST_VERIFY_USAGE.md`
- Examples: `examples/test_verify_*.rs`
- Sample log: `data/example_test_execution.log`
