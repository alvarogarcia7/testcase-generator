# Acceptance Suite Success Tests Run - Failures and Fixes

## Overview

This document captures the failures encountered when running the acceptance suite on success test cases only, and the fixes applied to resolve them.

## Test Environment Setup

1. **Test Directory Isolation**: Moved non-success test directories aside
   - Created backup: `test-acceptance/.backup_test_cases/`
   - Backed up directories: `bash_commands`, `complex`, `dependencies`, `failure`, `hooks`, `manual`, `prerequisites`, `variables`
   - Kept only: `success/` directory with 13 test cases

## Failures Identified

### 1. Invalid JSON Execution Logs (Stage 3)

**Problem**: The acceptance suite was capturing stdout from test scripts instead of using the generated JSON log files.

**Root Cause**: Generated scripts create JSON logs in the `scripts/` directory with pattern `<basename>_execution_log.json`, but the acceptance suite was redirecting stdout to `execution_logs/<basename>.json`.

**Impact**: 61 execution failures due to invalid JSON (text output captured instead of JSON)

**Fix**: Modified `execute_test_scripts()` function in `test-acceptance/run_acceptance_suite.sh`:
- Changed from capturing stdout to copying the generated JSON log file
- Updated log file path resolution: `$(dirname "$script_file")/${basename}_execution_log.json`
- Copy generated log to execution_logs directory for verification stage
- Added JSON validation using `jq` as fallback validator

**Code Changes**:
```bash
# Old approach:
"$script_file" > "$log_file" 2>&1

# New approach:
local generated_log="$(dirname "$script_file")/${basename}_execution_log.json"
local log_file="$EXECUTION_LOGS_DIR/${basename}.json"
"$script_file" > /dev/null 2>&1
cp "$generated_log" "$log_file"
```

### 2. Wrong Verifier Arguments (Stage 4)

**Problem**: The verifier invocation used `--execution-log` argument which doesn't exist.

**Root Cause**: Mismatch between acceptance suite and actual verifier CLI interface.

**Impact**: 13 verification failures - all test cases failed verification

**Verifier Help Output**:
```
Options:
  -l, --log <PATH>                 Single-file mode: path to log file
  -c, --test-case <ID>             Single-file mode: test case ID to verify against
  -d, --test-case-dir <DIR>        Path to test case storage directory [default: testcases]
```

**Fix**: Modified `verify_execution_logs()` function in `test-acceptance/run_acceptance_suite.sh`:
- Changed `--execution-log` to `--log`
- Changed `--test-case "$test_case_yaml"` to `--test-case "$test_case_id"`
- Added `--test-case-dir "$TEST_CASES_DIR"` to specify test case location
- Extract test case ID from YAML filename (basename without extension)

**Code Changes**:
```bash
# Old approach:
"$VERIFIER" \
    --test-case "$test_case_yaml" \
    --execution-log "$log_file" \
    ...

# New approach:
local test_case_id=$(basename "$test_case_yaml" .yaml)
"$VERIFIER" \
    --test-case "$test_case_id" \
    --log "$log_file" \
    --test-case-dir "$TEST_CASES_DIR" \
    ...
```

### 3. Stale Generated Scripts from Moved Directories

**Problem**: Scripts directory contained previously generated scripts from test directories that were moved aside (hooks, manual, etc.).

**Root Cause**: Script generation occurred before test directories were moved, leaving 40+ stale scripts in `test-acceptance/scripts/`.

**Impact**: Acceptance suite attempted to execute 102 scripts instead of just 13 success test cases.

**Fix**: Clean up scripts and regenerate only for success test cases
- Remove all scripts: `rm -rf test-acceptance/scripts/*`
- Remove execution logs: `rm -rf test-acceptance/execution_logs/*`
- Regenerate scripts only for success test cases

### 4. Hooks Test Scripts Execution

**Problem**: Scripts from hooks test cases (`fail_hook.sh`, `fail_hook_continue.sh`, etc.) were being executed and reported as failures.

**Root Cause**: Same as issue #3 - stale generated scripts from hooks directory.

**Impact**: Multiple "exit code: 0" failures from hook test scripts that weren't success cases.

## Summary of Fixes Applied

1. **test-acceptance/run_acceptance_suite.sh** - `execute_test_scripts()` function:
   - Copy generated JSON logs instead of capturing stdout
   - Use correct JSON log file naming pattern
   - Added `jq` as fallback JSON validator

2. **test-acceptance/run_acceptance_suite.sh** - `verify_execution_logs()` function:
   - Changed `--execution-log` to `--log`
   - Extract test case ID from YAML filename
   - Added `--test-case-dir` argument

3. **Clean Up Process**:
   - Remove stale scripts: `rm -rf test-acceptance/scripts/*`
   - Remove stale logs: `rm -rf test-acceptance/execution_logs/*`
   - Remove nested backup directory: `rm -rf test-acceptance/test_cases/test-acceptance/`

## Expected Results After Fixes

After applying all fixes and cleaning up stale scripts, the acceptance suite should:

1. **Stage 1 (Validation)**: Pass all 13 success test cases
2. **Stage 2 (Generation)**: Generate 13 scripts successfully
3. **Stage 3 (Execution)**: Execute 13 scripts, copy valid JSON logs
4. **Stage 4 (Verification)**: Verify 13 execution logs successfully
5. **Stage 5 (Container Validation)**: Validate 13 container YAMLs
6. **Stage 6 (Documentation)**: Generate docs for 13 test cases (if TPDG available)
7. **Stage 7 (Consolidated Docs)**: Generate unified documentation (if TPDG available)

## Failure Patterns Documented

### Pattern 1: JSON Log File Handling
- **Symptom**: "Invalid JSON in execution log"
- **Cause**: Capturing stdout instead of generated JSON file
- **Solution**: Copy generated `<basename>_execution_log.json` from scripts directory

### Pattern 2: CLI Argument Mismatch
- **Symptom**: "error: unexpected argument '--execution-log' found"
- **Cause**: Using outdated or incorrect CLI argument names
- **Solution**: Consult `--help` output to verify correct argument names

### Pattern 3: Stale Artifacts
- **Symptom**: Unexpected test cases being executed
- **Cause**: Previously generated artifacts not cleaned up
- **Solution**: Clean all generated artifacts before regenerating

### Pattern 4: Test Case Directory Management
- **Symptom**: "Test case YAML not found"
- **Cause**: Verifier looking in wrong directory
- **Solution**: Use `--test-case-dir` to specify correct location

## Next Steps

1. Clean up all generated artifacts
2. Run acceptance suite on success tests only
3. Verify all 7 stages complete successfully
4. Document any remaining failures
5. Create comprehensive test case validation process
