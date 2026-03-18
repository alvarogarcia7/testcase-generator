# Acceptance Suite Orchestrator - Implementation Summary

This document provides a technical summary of the `run_acceptance_suite.sh` implementation.

## Implementation Overview

The master orchestrator is implemented as a single bash script (`run_acceptance_suite.sh`) that orchestrates six sequential stages of acceptance testing.

## Technical Details

### Language and Compatibility

- **Language:** Bash 3.2+ (macOS/Linux compatible)
- **Dependencies:** Uses centralized logging library (`scripts/lib/logger.sh`)
- **Portability:** BSD/GNU utilities compatible

### Architecture

**Sequential Pipeline:**
```
Test YAMLs → Validation → Generation → Execution → Verification → Container Validation → Documentation
```

**Data Flow:**
1. `test_cases/*.yaml` → Schema validation
2. `test_cases/*.yaml` → `scripts/*.sh` (generated)
3. `scripts/*.sh` → `execution_logs/*.json` (execution output)
4. `execution_logs/*.json` + `test_cases/*.yaml` → `verification_results/*_container.yaml`
5. `verification_results/*_container.yaml` → Schema validation
6. `verification_results/*_container.yaml` + `test_cases/*.yaml` → `reports/{asciidoc,markdown}/*`

### Statistics Tracking

The script tracks comprehensive statistics using bash integer variables:

**Counters:**
- `TOTAL_TEST_CASES` - Total YAML files found
- Per stage: `{STAGE}_PASSED`, `{STAGE}_FAILED`
- Execution: `EXECUTION_SKIPPED` (manual tests)

**Tracking Files:**
- `validation_failures.txt`
- `generation_failures.txt`
- `execution_failures.txt`
- `verification_failures.txt`
- `container_validation_failures.txt`
- `documentation_failures.txt`
- `manual_tests.txt`

### Binary Dependencies

**Required:**
- `validate-yaml` - YAML schema validation
- `test-executor` - Script generation
- `verifier` - Container YAML generation
- `validate-json` - JSON validation

**Optional:**
- `test-plan-documentation-generator` (TPDG) - Documentation generation

**Location Discovery:**
- Uses hardcoded paths: `${PROJECT_ROOT}/target/debug/{binary}`
- TPDG: Uses `$TEST_PLAN_DOC_GEN` environment variable or PATH lookup

### Command Line Parsing

**Options Supported:**
- `--verbose` - Enable verbose output (sets `VERBOSE=1`)
- `--include-manual` - Include manual tests (sets `INCLUDE_MANUAL=1`)
- `--skip-generation` - Skip stage 2 (sets `SKIP_GENERATION=1`)
- `--skip-execution` - Skip stage 3 (sets `SKIP_EXECUTION=1`)
- `--skip-verification` - Skip stages 4-5 (sets `SKIP_VERIFICATION=1`)
- `--skip-documentation` - Skip stage 6 (sets `SKIP_DOCUMENTATION=1`)
- `-h, --help` - Show usage and exit

**Implementation:**
- Simple while loop with case statement
- Unknown options trigger error and usage display

### Stage Implementations

#### Stage 1: YAML Validation

**Function:** `validate_test_cases()`

**Logic:**
1. Find all `*.yaml` files using `find` with `-print0` (handles spaces)
2. Sort files for deterministic ordering
3. Validate each against `schemas/test-case.schema.json`
4. Track pass/fail counts
5. Continue on failure (validate all files)

**Key Features:**
- Uses `validate-yaml --schema <schema> <file>`
- Captures output to temp file for verbose mode
- Non-blocking: all files validated even if some fail

#### Stage 2: Script Generation

**Function:** `generate_test_scripts()`

**Logic:**
1. Create `scripts/` directory
2. Find all test case YAMLs
3. Generate bash script for each using `test-executor generate --json-log`
4. Make scripts executable with `chmod +x`
5. Track pass/fail counts

**Key Features:**
- Output files: `scripts/{basename}.sh`
- Uses `--json-log` flag for structured logging
- Skippable with `--skip-generation`

#### Stage 3: Test Execution

**Function:** `execute_test_scripts()`

**Logic:**
1. Find all generated `*.sh` files in `scripts/`
2. For each script:
   - Find corresponding YAML to check if manual
   - Skip manual tests unless `--include-manual`
   - Execute script, redirect output to JSON log
   - Validate JSON structure with `python3 -m json.tool`
3. Track passed/failed/skipped counts

**Key Features:**
- Output files: `execution_logs/{basename}.json`
- Manual test detection via `is_manual_test()` helper
- JSON validation ensures structured output
- Skippable with `--skip-execution`

**Manual Test Detection:**
```bash
is_manual_test() {
    local yaml_file="$1"
    if grep -q "manual: true" "$yaml_file" 2>/dev/null; then
        return 0  # Is manual
    fi
    return 1  # Not manual
}
```

#### Stage 4: Verification

**Function:** `verify_execution_logs()`

**Logic:**
1. Find all `*.json` execution logs
2. For each log:
   - Find corresponding test case YAML
   - Run `verifier` with metadata flags
   - Generate container YAML with proper metadata
3. Track pass/fail counts

**Verifier Invocation:**
```bash
"$VERIFIER" \
    --title "Acceptance Test Results - $(basename "$test_case_yaml")" \
    --project "Test Case Manager - Acceptance Suite" \
    --environment "Automated Test Environment - $hostname" \
    --test-case "$test_case_yaml" \
    --execution-log "$log_file" \
    --output "$container_file"
```

**Key Features:**
- Output files: `verification_results/{basename}_container.yaml`
- Includes title, project, environment metadata
- Skippable with `--skip-verification`

#### Stage 5: Container Validation

**Function:** `validate_container_yamls()`

**Logic:**
1. Find all `*_container.yaml` files
2. Validate each against `data/testcase_results_container/schema.json`
3. Track pass/fail counts

**Key Features:**
- Uses `validate-yaml --schema <schema> <file>`
- Ensures TPDG compatibility
- Auto-skipped if verification was skipped

#### Stage 6: Documentation Generation

**Function:** `generate_documentation()`

**Logic:**
1. Check for TPDG binary availability
2. Find all container YAMLs
3. For each container:
   - Find corresponding test case YAML
   - Generate AsciiDoc report
   - Generate Markdown report
4. Track pass/fail counts

**TPDG Invocation:**
```bash
# AsciiDoc
"$TPDG_BIN" \
    --input "$container_file" \
    --output "$asciidoc_file" \
    --format asciidoc \
    --test-case "$test_case_yaml"

# Markdown
"$TPDG_BIN" \
    --input "$container_file" \
    --output "$markdown_file" \
    --format markdown \
    --test-case "$test_case_yaml"
```

**Key Features:**
- Output files: `reports/asciidoc/{basename}.adoc`, `reports/markdown/{basename}.md`
- Graceful handling if TPDG not available
- Both formats generated per test case
- Skippable with `--skip-documentation`

### Summary Report

**Function:** `generate_summary_report()`

**Logic:**
1. Collect all statistics from stages
2. Generate comprehensive text report
3. Display to console via `tee`
4. Save to `reports/acceptance_suite_summary.txt`

**Report Sections:**
- Execution metadata (date, total tests)
- Per-stage statistics (passed/failed/skipped)
- Failure details (file lists)
- Overall result (SUCCESS/FAILURE)

### Error Handling

**Binary Verification:**
- `verify_binaries()` checks all required binaries at startup
- Exits early with clear error messages if missing
- Provides build instructions

**Stage Failures:**
- Each stage returns 0 (success) or 1 (failure)
- Failures don't stop subsequent stages (best effort)
- All failures tracked and reported in summary

**Graceful Degradation:**
- TPDG optional: warning if missing, stage skipped
- Manual tests: skipped by default, opt-in with flag
- Skip flags: allow partial execution for debugging

### Cleanup Management

**Temporary Files:**
- Uses `mktemp -d` for temp directory
- Registered with logger's `setup_cleanup()`
- Automatic cleanup on exit via trap

**Tracked Files:**
- Failure tracking files
- Manual test list
- Command output captures

### Logging

**Color-Coded Output:**
- `pass()` - Green checkmark (✓)
- `fail()` - Red X (✗)
- `info()` - Blue info symbol (ℹ)
- `section()` - Yellow section headers

**Log Levels:**
- `log_info()` - Standard information
- `log_warning()` - Warnings (continue execution)
- `log_error()` - Errors (indicate failures)
- `log_verbose()` - Verbose details (only if `--verbose`)
- `log_debug()` - Debug messages (only if `--verbose`)

**Structured Output:**
- Section headers for each stage
- Per-file status indicators
- Stage summaries with counts
- Final summary report

### File Organization

**Input:**
- `test_cases/**/*.yaml` - Test case definitions

**Generated (gitignored):**
- `scripts/*.sh` - Executable test scripts
- `execution_logs/*.json` - Execution outputs
- `verification_results/*_container.yaml` - Verification containers
- `reports/asciidoc/*.adoc` - AsciiDoc reports
- `reports/markdown/*.md` - Markdown reports
- `reports/acceptance_suite_summary.txt` - Summary report

**Temporary (auto-cleaned):**
- `$TEMP_DIR/*_failures.txt` - Failure tracking
- `$TEMP_DIR/manual_tests.txt` - Manual test list
- `$TEMP_DIR/*_output.txt` - Command output captures

### Exit Codes

- `0` - All stages successful (no failures)
- `1` - One or more stages had failures

**Exit Logic:**
```bash
total_failures=$((VALIDATION_FAILED + GENERATION_FAILED + \
                  EXECUTION_FAILED + VERIFICATION_FAILED + \
                  CONTAINER_VALIDATION_FAILED + DOCUMENTATION_FAILED))

if [[ $total_failures -eq 0 ]]; then
    exit 0
else
    exit 1
fi
```

## Key Implementation Decisions

### Why Sequential Execution?

- Simpler implementation and debugging
- Each stage depends on previous output
- Clear progress tracking
- Easier to skip stages for debugging

### Why Bash?

- Native integration with test harness
- Direct execution of generated bash scripts
- No additional runtime dependencies
- Consistent with project tooling

### Why Track All Failures?

- Best-effort execution provides maximum information
- Don't stop on first failure
- Generate reports even with partial failures
- Useful for debugging multiple issues

### Why Separate Stages?

- Clear separation of concerns
- Independent skip flags for each stage
- Easier testing and debugging
- Reusable intermediate artifacts

### Why Manual Test Skipping?

- Manual tests require human interaction
- Default to automated workflow
- Opt-in for full validation
- Clearly tracked in statistics

## Performance Characteristics

**Sequential Processing:**
- O(n) time complexity per stage
- No parallelization (intentional for clarity)

**File I/O:**
- Multiple passes over test case directory
- Could be optimized with single pass + indexing

**Scaling:**
- Linear with number of test cases
- Execution stage is bottleneck (actual test runtime)
- Documentation generation second bottleneck (TPDG invocations)

**Typical Runtime (93 test cases):**
- Validation: ~5-10 seconds
- Generation: ~10-20 seconds
- Execution: ~2-5 minutes (depends on tests)
- Verification: ~10-20 seconds
- Container Validation: ~5-10 seconds
- Documentation: ~30-60 seconds

**Total: ~3-7 minutes for full suite**

## Testing Strategy

**Script Validation:**
- Syntax check: `bash -n run_acceptance_suite.sh`
- Help output: `./run_acceptance_suite.sh --help`
- Dry run stages: Use skip flags

**Integration Testing:**
- Run on subset: Move test cases temporarily
- Test individual stages: Use skip flags
- Verify output artifacts: Check generated files

## Future Optimizations

**Potential Improvements:**
1. Parallel execution of independent tests
2. Incremental execution (only changed tests)
3. Cached validation results
4. Progress bars for long stages
5. HTML summary report
6. JUnit XML export
7. Baseline comparison
8. Performance profiling

## Code Statistics

- **Lines of Code:** ~850
- **Functions:** 8 main functions + helpers
- **Stages:** 6
- **Command Line Options:** 7
- **Exit Codes:** 2
- **Tracked Statistics:** 13 counters
- **Generated Files:** 4 directories of artifacts

## Dependencies

**Required Tools:**
- bash 3.2+
- find, grep, sed (BSD/GNU compatible)
- python3 (for JSON validation)
- cargo/rust binaries (validate-yaml, test-executor, verifier)

**Optional Tools:**
- test-plan-documentation-generator (TPDG)

## Maintainability

**Code Organization:**
- Clear function separation
- Consistent naming conventions
- Comprehensive comments
- Error handling throughout

**Extensibility Points:**
- Easy to add new stages
- Simple to add command line options
- Straightforward to add new statistics
- Clear places for custom logic

**Documentation:**
- Inline comments for complex logic
- Usage function with full help
- Comprehensive external documentation
- Implementation summary (this document)
