# Report Generator Library - Error Handling and Validation

The `report_generator.sh` library provides robust error handling and validation for test-plan-doc-gen (tpdg) invocations.

## Overview

This library wraps the test-plan-doc-gen CLI tool with:

- **Comprehensive error detection** - Validates exit codes and provides detailed diagnostics
- **Automatic retry logic** - Retries transient failures (I/O errors) automatically
- **Output validation** - Checks that generated files exist and contain valid content
- **Graceful degradation** - Handles missing or unavailable tpdg binary gracefully
- **Helpful error messages** - Provides actionable diagnostics for common failure scenarios

## Key Features

### 1. Exit Code Handling

The library interprets test-plan-doc-gen exit codes:

| Exit Code | Meaning | Retryable | Description |
|-----------|---------|-----------|-------------|
| 0 | Success | N/A | Report generated successfully |
| 1 | General error | No | File not found, parsing error, or invalid input |
| 2 | Invalid arguments | No | Missing required arguments or invalid usage |
| 101 | I/O error | Yes | Disk full, permissions, or temporary filesystem issue |
| 130 | Interrupted | No | User interrupted with Ctrl+C |

### 2. Automatic Retry Logic

Only I/O errors (exit code 101) are automatically retried, as they are typically transient:

```bash
# Use retry logic (recommended for production scripts)
if invoke_test_plan_doc_gen_with_retry --test-case "test.yaml" --output "report.md" --format markdown; then
    echo "Report generated successfully (possibly after retries)"
else
    echo "Report generation failed permanently"
fi
```

**Configuration:**
- `TPDG_MAX_RETRIES` - Maximum retry attempts (default: 3)
- `TPDG_RETRY_DELAY` - Delay between retries in seconds (default: 2)

```bash
# Customize retry behavior
TPDG_MAX_RETRIES=5 TPDG_RETRY_DELAY=3 invoke_test_plan_doc_gen_with_retry --test-case "test.yaml" --output "out.md"
```

### 3. Output Validation

The library validates generated report files:

**File Existence:**
```bash
# Validate multiple output files
if validate_report_output "$OUTPUT_DIR" "report1.md" "report2.adoc"; then
    echo "All files generated successfully"
fi
```

**Content Validation:**
```bash
# Validate file content (checks size and format-specific markers)
if validate_report_file_content "report.md" 100; then
    echo "File has valid content (>= 100 bytes)"
fi
```

The content validator checks:
- File exists and is readable
- File size meets minimum threshold
- File contains expected format markers (for Markdown/AsciiDoc)
- File is not empty or whitespace-only

### 4. Error Diagnostics

When tpdg fails, the library provides detailed diagnostics:

```
[ERROR] test-plan-doc-gen failed with exit code: 1
[ERROR] Error: General error (file not found, parsing error, or invalid input)
[ERROR] Common causes:
[ERROR]   - Input file not found or not readable
[ERROR]   - YAML parsing error in input file
[ERROR]   - Invalid YAML structure or missing required fields
[ERROR]   → YAML parsing error detected
[ERROR] 
[ERROR] Last output from test-plan-doc-gen:
[ERROR]   Error: Failed to parse YAML
[ERROR]   invalid type: found unit, expected struct TestCase
[ERROR]   at line 42 column 5
```

## Function Reference

### Core Functions

#### `invoke_test_plan_doc_gen <args...>`

Invokes test-plan-doc-gen with detailed error handling (single attempt).

**Returns:** Exit code from test-plan-doc-gen

**Example:**
```bash
if invoke_test_plan_doc_gen --test-case "test.yaml" --output "report.md" --format markdown; then
    echo "Success"
else
    exit_code=$?
    echo "Failed with exit code: $exit_code"
fi
```

#### `invoke_test_plan_doc_gen_with_retry <args...>`

Invokes test-plan-doc-gen with automatic retry on transient failures.

**Returns:** 0 on success, 1 on permanent failure

**Example:**
```bash
# Will retry up to 3 times on I/O errors
invoke_test_plan_doc_gen_with_retry --test-case "test.yaml" --output "report.md" --format markdown
```

#### `validate_report_output <output-dir> <expected-file>...`

Validates that expected output files were generated and contain valid content.

**Parameters:**
- `output-dir` - Directory to check for output files
- `expected-file...` - One or more filenames to validate

**Returns:** 0 if all files are valid, 1 if any are missing or invalid

**Example:**
```bash
if validate_report_output "$OUTPUT_DIR" "report1.md" "report2.adoc" "report3.md"; then
    echo "All 3 reports generated successfully"
fi
```

#### `validate_report_file_content <file-path> [min-size-bytes]`

Validates that a report file exists and has valid content.

**Parameters:**
- `file-path` - Path to file to validate
- `min-size-bytes` - Minimum file size in bytes (default: 10)

**Returns:** 0 if file is valid, 1 if invalid

**Example:**
```bash
# Require at least 1KB of content
if validate_report_file_content "report.md" 1024; then
    echo "Report has sufficient content"
fi
```

### Utility Functions

#### `build_test_plan_doc_gen <sibling-directory-path>`

Builds test-plan-doc-gen binary from source.

**Returns:** 0 on success, 1 on failure

**Example:**
```bash
if build_test_plan_doc_gen "../test-plan-doc-gen"; then
    echo "Binary built successfully"
fi
```

#### `check_test_plan_doc_gen_available [sibling-directory-path]`

Checks if test-plan-doc-gen binary is available.

**Returns:** 0 if available, 1 if not

**Example:**
```bash
if check_test_plan_doc_gen_available; then
    echo "Binary is available"
else
    echo "Binary not found, building..."
    build_test_plan_doc_gen "../test-plan-doc-gen"
fi
```

#### `find_test_plan_doc_gen [sibling-directory-path]`

Finds test-plan-doc-gen binary in sibling directory or PATH.

**Returns:** Prints path to binary on stdout, returns 0 if found, 1 if not

**Example:**
```bash
binary_path=$(find_test_plan_doc_gen "../test-plan-doc-gen")
if [ $? -eq 0 ]; then
    echo "Found binary at: $binary_path"
fi
```

#### `verify_test_plan_doc_gen_binary [binary-path]`

Verifies that test-plan-doc-gen binary works correctly.

**Returns:** 0 if binary works, 1 if not

**Example:**
```bash
if verify_test_plan_doc_gen_binary; then
    echo "Binary is functional"
fi
```

#### `get_tpdg_error_message <exit-code>`

Returns a human-readable error message for a tpdg exit code.

**Returns:** Prints error message on stdout

**Example:**
```bash
exit_code=1
error_msg=$(get_tpdg_error_message $exit_code)
echo "Error: $error_msg"
```

#### `is_transient_error <exit-code>`

Checks if an exit code represents a transient error that can be retried.

**Returns:** 0 if error is transient, 1 if permanent

**Example:**
```bash
if is_transient_error 101; then
    echo "This error can be retried"
fi
```

## Usage Patterns

### Basic Usage

```bash
#!/usr/bin/env bash
set -e

# Source the library
source scripts/lib/report_generator.sh

# Build tpdg if needed
if ! check_test_plan_doc_gen_available; then
    build_test_plan_doc_gen "../test-plan-doc-gen"
fi

# Generate a report
invoke_test_plan_doc_gen \
    --test-case "testcases/TC001.yml" \
    --output "reports/TC001_report.md" \
    --format markdown
```

### Production Usage with Retry and Validation

```bash
#!/usr/bin/env bash
set -e

source scripts/lib/report_generator.sh

# Ensure tpdg is available
if ! check_test_plan_doc_gen_available; then
    log_error "test-plan-doc-gen not available"
    exit 1
fi

# Generate report with retry
if invoke_test_plan_doc_gen_with_retry \
    --test-case "testcases/TC001.yml" \
    --output "reports/TC001_report.md" \
    --format markdown; then
    
    # Validate the output
    if validate_report_file_content "reports/TC001_report.md" 500; then
        echo "Report generated and validated successfully"
    else
        echo "Report validation failed"
        exit 1
    fi
else
    echo "Report generation failed after retries"
    exit 1
fi
```

### Batch Processing with Error Handling

```bash
#!/usr/bin/env bash

source scripts/lib/report_generator.sh

# Track failures
declare -a FAILED_REPORTS=()

# Process multiple test cases
for test_case in testcases/*.yml; do
    basename=$(basename "$test_case" .yml)
    output="reports/${basename}_report.md"
    
    log_info "Generating report for: $basename"
    
    if invoke_test_plan_doc_gen_with_retry \
        --test-case "$test_case" \
        --output "$output" \
        --format markdown; then
        
        pass "Report generated: $output"
    else
        fail "Failed to generate report for: $basename"
        FAILED_REPORTS+=("$basename")
    fi
done

# Report failures
if [ ${#FAILED_REPORTS[@]} -gt 0 ]; then
    log_error "Failed to generate ${#FAILED_REPORTS[@]} report(s):"
    for failed in "${FAILED_REPORTS[@]}"; do
        log_error "  - $failed"
    done
    exit 1
fi

echo "All reports generated successfully"
```

### Graceful Degradation

```bash
#!/usr/bin/env bash

source scripts/lib/report_generator.sh

# Try to generate reports, but don't fail if tpdg is unavailable
if check_test_plan_doc_gen_available; then
    log_info "Generating documentation reports..."
    
    invoke_test_plan_doc_gen_with_retry \
        --test-case "test.yaml" \
        --output "report.md" \
        --format markdown || true
else
    log_warning "test-plan-doc-gen not available"
    log_warning "Skipping documentation report generation"
fi

# Continue with other tasks...
```

## Error Recovery Strategies

### Strategy 1: Retry with Exponential Backoff

```bash
# Custom retry with exponential backoff
attempt=1
delay=1

while [ $attempt -le 5 ]; do
    if invoke_test_plan_doc_gen --test-case "test.yaml" --output "out.md" --format markdown; then
        break
    fi
    
    exit_code=$?
    if is_transient_error $exit_code; then
        log_warning "Retry $attempt failed, waiting ${delay}s..."
        sleep $delay
        delay=$((delay * 2))
        ((attempt++))
    else
        log_error "Permanent error, not retrying"
        exit 1
    fi
done
```

### Strategy 2: Fallback to Alternative Format

```bash
# Try Markdown, fallback to AsciiDoc
if ! invoke_test_plan_doc_gen --test-case "test.yaml" --output "report.md" --format markdown; then
    log_warning "Markdown generation failed, trying AsciiDoc..."
    invoke_test_plan_doc_gen --test-case "test.yaml" --output "report.adoc" --format asciidoc
fi
```

### Strategy 3: Continue on Error with Logging

```bash
# Generate reports for all test cases, log failures but don't stop
for test_case in testcases/*.yml; do
    if ! invoke_test_plan_doc_gen_with_retry \
        --test-case "$test_case" \
        --output "reports/$(basename "$test_case" .yml).md" \
        --format markdown; then
        
        log_error "Failed to generate report for: $test_case"
        # Continue processing other test cases
        continue
    fi
done
```

## Troubleshooting

### tpdg Not Found

**Error:**
```
[ERROR] test-plan-doc-gen binary not found
[ERROR] Please build it first using build_test_plan_doc_gen()
```

**Solution:**
```bash
build_test_plan_doc_gen "../test-plan-doc-gen"
```

### YAML Parsing Error

**Error:**
```
[ERROR] test-plan-doc-gen failed with exit code: 1
[ERROR] Error: General error (file not found, parsing error, or invalid input)
[ERROR]   → YAML parsing error detected
```

**Solution:**
- Check YAML syntax in the input file
- Validate against schema using `cargo run --bin test-case-manager -- validate`
- Check for missing required fields

### I/O Error

**Error:**
```
[ERROR] test-plan-doc-gen failed with exit code: 101
[ERROR] Error: I/O error (failed to read input or write output)
```

**Solution:**
- Check disk space: `df -h`
- Check output directory permissions: `ls -ld reports/`
- Use retry logic: `invoke_test_plan_doc_gen_with_retry`

### Output File Validation Failed

**Error:**
```
[WARNING] Output file validation failed, but tpdg reported success
[WARNING] File may be incomplete: report.md
```

**Solution:**
- Check file content manually: `cat report.md`
- Increase minimum size threshold if file is intentionally small
- Check for errors in tpdg output

## Best Practices

1. **Always use retry logic in production scripts**
   ```bash
   invoke_test_plan_doc_gen_with_retry  # Recommended
   # instead of
   invoke_test_plan_doc_gen  # Use only when retry is not desired
   ```

2. **Validate output files after generation**
   ```bash
   invoke_test_plan_doc_gen_with_retry "$@"
   validate_report_file_content "$output_file"
   ```

3. **Check binary availability before use**
   ```bash
   if ! check_test_plan_doc_gen_available; then
       build_test_plan_doc_gen "../test-plan-doc-gen" || exit 1
   fi
   ```

4. **Use verbose logging for debugging**
   ```bash
   VERBOSE=1 invoke_test_plan_doc_gen_with_retry "$@"
   ```

5. **Handle graceful degradation for optional reports**
   ```bash
   if check_test_plan_doc_gen_available; then
       invoke_test_plan_doc_gen_with_retry "$@" || log_warning "Report generation failed"
   else
       log_warning "Skipping report generation (tpdg not available)"
   fi
   ```

## Environment Variables

- `TEST_PLAN_DOC_GEN` - Path to test-plan-doc-gen binary (overrides auto-detection)
- `TPDG_MAX_RETRIES` - Maximum retry attempts (default: 3)
- `TPDG_RETRY_DELAY` - Delay between retries in seconds (default: 2)
- `VERBOSE` - Enable verbose logging (set to 1)

## Related Documentation

- [Logger Library](logger.sh) - Centralized logging functions
- [Test Plan Documentation Generator](../../docs/TEST_PLAN_DOC_GEN_COMPATIBILITY.md) - tpdg compatibility guide
- [Report Generation Guide](../README_REPORT_GENERATION.md) - End-to-end report generation
