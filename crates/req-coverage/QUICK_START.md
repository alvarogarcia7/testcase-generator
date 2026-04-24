# req-coverage Quick Start Guide

## Installation

```bash
cargo build --release -p req-coverage
```

Binary available at: `target/release/req-coverage`

## Basic Usage (Without String Verification)

### 1. Generate Coverage Report

```bash
req-coverage verify \
  --test-cases-folder ./testcases \
  --test-results-folder ./test-results \
  --output coverage.json
```

### 2. Generate HTML Report

```bash
req-coverage print \
  --format html \
  --input coverage.json \
  --output ./report/
```

### 3. View Report

```bash
open ./report/index.html
```

## Advanced Usage (With String Verification)

### 1. Create Requirements File

**requirements.yaml:**
```yaml
requirements:
  - id: REQ-001
    text: "The system shall authenticate users and deny access to invalid users."
    description: "Authentication requirement"
```

### 2. Update Test Cases

**test_case_001.yaml:**
```yaml
requirement: REQ-001
requirement_coverage:
  type: partial
  covers: "authenticate users"
```

**test_case_002.yaml:**
```yaml
requirement: REQ-001
requirement_coverage:
  type: partial
  covers: "deny access to invalid users"
```

### 3. Generate Coverage Report with Verification

```bash
req-coverage verify \
  --test-cases-folder ./testcases \
  --test-results-folder ./test-results \
  --output coverage.json \
  --requirements-file ./requirements.yaml
```

### 4. Generate HTML Report

```bash
req-coverage print \
  --format html \
  --input coverage.json \
  --output ./report/
```

## Coverage Types

- **Full**: All parts of requirement text are covered
- **Partial**: Some parts of requirement text are covered
- **Uncovered**: No test cases for this requirement

## Status Values

- **covered_pass**: Full coverage, all tests passed
- **covered_fail**: Full coverage, some tests failed
- **partial_covered_pass**: Partial coverage, all tests passed
- **partial_covered_fail**: Partial coverage, some tests failed
- **uncovered**: No test coverage

## Common Options

```bash
# Enable verbose logging
req-coverage verify --verbose ...

# Custom log level
req-coverage verify --log-level debug ...

# Use custom HTML template
req-coverage print --template ./custom.html ...
```

## Examples

See example files:
- `crates/req-coverage/templates/requirements.example.yaml`
- `crates/req-coverage/templates/requirements.example.json`

## Documentation

- [README.md](README.md) - Detailed usage guide
- [docs/REQ_COVERAGE_STRING_VERIFICATION.md](../../docs/REQ_COVERAGE_STRING_VERIFICATION.md) - String verification feature
- [docs/PRD_REQ_COVERAGE.md](../../docs/PRD_REQ_COVERAGE.md) - Product requirements
