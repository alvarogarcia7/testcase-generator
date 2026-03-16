# Report Generator Error Handling Implementation

## Summary

Implemented comprehensive error handling and validation in `scripts/lib/report_generator.sh` for test-plan-doc-gen (tpdg) invocations.

## Changes Made

### 1. Enhanced Error Handling

**File:** `scripts/lib/report_generator.sh`

Added comprehensive error handling with:

- **Exit code interpretation** - Maps tpdg exit codes to human-readable error messages
- **Contextual diagnostics** - Provides specific troubleshooting guidance based on error type
- **Output validation** - Verifies generated files exist and contain valid content
- **Graceful failure** - Scripts fail with meaningful diagnostics when tpdg is unavailable or encounters errors

### 2. Retry Logic for Transient Failures

**New Function:** `invoke_test_plan_doc_gen_with_retry()`

- Automatically retries on I/O errors (exit code 101)
- Configurable via environment variables:
  - `TPDG_MAX_RETRIES` (default: 3)
  - `TPDG_RETRY_DELAY` (default: 2 seconds)
- Distinguishes between transient and permanent errors
- Only retries when appropriate (I/O errors), not for parsing errors or invalid arguments

### 3. Exit Code Handling

**New Function:** `get_tpdg_error_message()`

Maps exit codes to meaningful error messages:

| Exit Code | Type | Retryable | Error Message |
|-----------|------|-----------|---------------|
| 0 | Success | N/A | Success |
| 1 | General Error | No | File not found, parsing error, or invalid input |
| 2 | Invalid Arguments | No | Invalid command-line arguments or usage error |
| 101 | I/O Error | Yes | I/O error (failed to read input or write output) |
| 130 | Interrupted | No | Interrupted by user (Ctrl+C) |

### 4. Output File Validation

**Enhanced Function:** `validate_report_output()`

- Validates file existence
- Validates file content (size and format-specific checks)
- Supports Markdown and AsciiDoc format validation
- Provides detailed error messages for missing or invalid files

**New Function:** `validate_report_file_content()`

- Checks file size meets minimum threshold
- Validates format-specific content markers
- Detects empty or whitespace-only files
- Supports custom minimum size requirements

### 5. Binary Verification

**New Function:** `verify_test_plan_doc_gen_binary()`

- Verifies tpdg binary is executable
- Tests binary functionality with `--help` command
- Validates binary output format
- Provides early detection of binary issues

**Enhanced Function:** `build_test_plan_doc_gen()`

- Improved error detection during build
- Validates Cargo.toml exists and is correct
- Verifies binary was created after build
- Extracts and logs build errors

**Enhanced Function:** `find_test_plan_doc_gen()`

- Checks `TEST_PLAN_DOC_GEN` environment variable first
- Searches sibling directory (release and debug builds)
- Falls back to system PATH
- Provides warnings for non-executable binaries

### 6. Enhanced Diagnostics

**Enhanced Function:** `invoke_test_plan_doc_gen()`

- Captures and logs all tpdg output
- Provides exit-code-specific error guidance
- Suggests common solutions for each error type
- Extracts specific errors from tpdg output
- Auto-validates output file after successful generation

Example diagnostic output:

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
```

### 7. Transient Error Detection

**New Function:** `is_transient_error()`

- Identifies errors that can be safely retried
- Currently only I/O errors (101) are considered transient
- Prevents unnecessary retries for permanent errors
- Used by retry logic to determine retry eligibility

## New Features

### Automatic Retry with Intelligent Error Detection

```bash
# Automatically retries on I/O errors, fails fast on permanent errors
invoke_test_plan_doc_gen_with_retry \
    --test-case "test.yaml" \
    --output "report.md" \
    --format markdown
```

### Comprehensive Output Validation

```bash
# Validates all expected files exist and have valid content
validate_report_output "$OUTPUT_DIR" \
    "report1.md" \
    "report2.adoc" \
    "report3.md"
```

### Detailed Error Messages

Scripts now provide actionable error messages:

- Identifies common problems (missing files, parsing errors, permission issues)
- Suggests specific solutions based on error type
- Shows relevant excerpts from tpdg output
- Distinguishes between user errors and system issues

### Configurable Retry Behavior

```bash
# Customize retry attempts and delays
TPDG_MAX_RETRIES=5 TPDG_RETRY_DELAY=3 invoke_test_plan_doc_gen_with_retry "$@"
```

## Documentation

**New File:** `scripts/lib/README_REPORT_GENERATOR.md`

Comprehensive documentation covering:

- Overview of error handling features
- Exit code reference table
- Function reference with examples
- Usage patterns for common scenarios
- Error recovery strategies
- Troubleshooting guide
- Best practices
- Environment variable reference

## Function Summary

### Core Functions

1. **`invoke_test_plan_doc_gen <args...>`** - Single invocation with detailed error handling
2. **`invoke_test_plan_doc_gen_with_retry <args...>`** - Invocation with automatic retry
3. **`validate_report_output <dir> <files...>`** - Validate multiple output files
4. **`validate_report_file_content <file> [min-size]`** - Validate single file content

### Utility Functions

5. **`build_test_plan_doc_gen <dir>`** - Build tpdg binary with validation
6. **`check_test_plan_doc_gen_available [dir]`** - Check binary availability
7. **`find_test_plan_doc_gen [dir]`** - Locate tpdg binary
8. **`verify_test_plan_doc_gen_binary [path]`** - Verify binary functionality
9. **`get_tpdg_error_message <exit-code>`** - Get error description
10. **`is_transient_error <exit-code>`** - Check if error is retryable

## Usage Examples

### Basic Usage

```bash
source scripts/lib/report_generator.sh

# Generate report with error handling
invoke_test_plan_doc_gen \
    --test-case "testcases/TC001.yml" \
    --output "reports/TC001.md" \
    --format markdown
```

### Production Usage with Retry and Validation

```bash
source scripts/lib/report_generator.sh

# Generate with retry
if invoke_test_plan_doc_gen_with_retry \
    --test-case "testcases/TC001.yml" \
    --output "reports/TC001.md" \
    --format markdown; then
    
    # Validate output
    if validate_report_file_content "reports/TC001.md" 500; then
        echo "Report generated and validated"
    fi
fi
```

### Batch Processing with Error Tracking

```bash
source scripts/lib/report_generator.sh

declare -a FAILED_REPORTS=()

for test_case in testcases/*.yml; do
    if ! invoke_test_plan_doc_gen_with_retry \
        --test-case "$test_case" \
        --output "reports/$(basename "$test_case" .yml).md" \
        --format markdown; then
        
        FAILED_REPORTS+=("$(basename "$test_case")")
    fi
done

# Report failures
if [ ${#FAILED_REPORTS[@]} -gt 0 ]; then
    echo "Failed: ${FAILED_REPORTS[@]}"
    exit 1
fi
```

### Graceful Degradation

```bash
source scripts/lib/report_generator.sh

# Optional report generation - don't fail if tpdg unavailable
if check_test_plan_doc_gen_available; then
    invoke_test_plan_doc_gen_with_retry \
        --test-case "test.yaml" \
        --output "report.md" \
        --format markdown || true
else
    log_warning "Skipping report generation (tpdg not available)"
fi
```

## Error Scenarios Handled

### 1. tpdg Binary Not Found

**Before:**
```bash
# Would fail with cryptic "command not found" error
./test-plan-doc-gen: command not found
```

**After:**
```bash
[ERROR] test-plan-doc-gen binary not found
[ERROR] Please build it first using build_test_plan_doc_gen()
[ERROR] Or set TEST_PLAN_DOC_GEN environment variable to the binary path
```

### 2. YAML Parsing Error

**Before:**
```bash
# Generic error with no context
Error: Failed to parse YAML
```

**After:**
```bash
[ERROR] test-plan-doc-gen failed with exit code: 1
[ERROR] Error: General error (file not found, parsing error, or invalid input)
[ERROR] Common causes:
[ERROR]   - Input file not found or not readable
[ERROR]   - YAML parsing error in input file
[ERROR]   - Invalid YAML structure or missing required fields
[ERROR]   → YAML parsing error detected
[ERROR] 
[ERROR] Last output from test-plan-doc-gen:
[ERROR]   Error: Failed to parse YAML at line 42
```

### 3. I/O Error (Transient)

**Before:**
```bash
# Would fail immediately without retry
Error: Failed to write output file
```

**After:**
```bash
[WARNING] Transient error detected (exit code: 101)
[INFO] Retrying in 2s... (attempt 2 of 3)
[✓] test-plan-doc-gen completed successfully
```

### 4. Invalid Output File

**Before:**
```bash
# Would report success even if file is empty/corrupted
✓ Report generated
```

**After:**
```bash
[✓] test-plan-doc-gen completed successfully
[WARNING] Output file validation failed, but tpdg reported success
[WARNING] File may be incomplete or corrupted: report.md
[ERROR] validate_report_file_content: file is too small (5 bytes, minimum: 10 bytes)
```

## Integration with Existing Scripts

The enhanced error handling is automatically used by existing scripts that source `report_generator.sh`:

- `scripts/generate_documentation_reports.sh`
- `scripts/run_verifier_and_generate_reports.sh`
- `scripts/validate_tpdg_integration.sh`
- `scripts/test_container_yaml_compatibility.sh`

No changes required to existing scripts - they automatically benefit from improved error handling.

## Testing Recommendations

To validate the error handling implementation:

1. **Test with missing tpdg binary:**
   ```bash
   unset TEST_PLAN_DOC_GEN
   ./scripts/generate_documentation_reports.sh
   ```

2. **Test with invalid YAML:**
   ```bash
   echo "invalid: yaml: content" > /tmp/bad.yaml
   source scripts/lib/report_generator.sh
   invoke_test_plan_doc_gen --test-case /tmp/bad.yaml --output /tmp/out.md --format markdown
   ```

3. **Test retry logic:**
   ```bash
   # Simulate I/O error by making output directory read-only
   mkdir -p /tmp/readonly_reports
   chmod 444 /tmp/readonly_reports
   source scripts/lib/report_generator.sh
   invoke_test_plan_doc_gen_with_retry --test-case test.yaml --output /tmp/readonly_reports/out.md --format markdown
   ```

4. **Test output validation:**
   ```bash
   source scripts/lib/report_generator.sh
   echo "" > /tmp/empty.md
   validate_report_file_content /tmp/empty.md
   ```

## Benefits

1. **Improved Reliability** - Automatic retry on transient failures
2. **Better Diagnostics** - Clear error messages with actionable guidance
3. **Early Detection** - Validates output files to catch generation issues
4. **Graceful Degradation** - Scripts can continue when tpdg is unavailable
5. **Developer Experience** - Helpful error messages reduce debugging time
6. **Production Ready** - Robust error handling suitable for CI/CD pipelines

## Backward Compatibility

All existing function signatures are preserved. New functionality is additive:

- Existing calls to `invoke_test_plan_doc_gen()` work unchanged
- New retry function is opt-in via `invoke_test_plan_doc_gen_with_retry()`
- Enhanced validation is backwards compatible
- All exported functions maintain their original behavior

## Configuration

Environment variables for customization:

- `TEST_PLAN_DOC_GEN` - Override binary path
- `TPDG_MAX_RETRIES` - Maximum retry attempts (default: 3)
- `TPDG_RETRY_DELAY` - Retry delay in seconds (default: 2)
- `VERBOSE` - Enable verbose logging (for debugging)

## Files Modified

1. **scripts/lib/report_generator.sh** - Enhanced with comprehensive error handling
2. **scripts/lib/README_REPORT_GENERATOR.md** - New comprehensive documentation

## Implementation Complete

All requested functionality has been implemented:

- ✅ Check tpdg exit codes
- ✅ Validate report output files
- ✅ Provide helpful error messages
- ✅ Retry logic for transient failures
- ✅ Graceful failure with diagnostics
- ✅ Handle unavailable tpdg binary
- ✅ Validate output file content
- ✅ Comprehensive documentation
