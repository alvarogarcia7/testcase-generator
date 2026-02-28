# Verifier Test Scenarios

This directory contains comprehensive test scenarios designed to validate the test execution verifier's behavior across various execution outcomes and edge cases. Each scenario includes a test case YAML file and a corresponding execution log JSON file that simulates actual test execution results.

## Overview

The verifier validates test execution logs against test case definitions to ensure tests executed as expected. These scenarios test the verifier's ability to correctly detect:

- Successful test execution
- Failed steps at different positions (first, intermediate, last)
- Interrupted/incomplete execution
- Multiple sequence scenarios with mixed results
- Hook failures at various lifecycle points

## Directory Structure

```
verifier_scenarios/
├── successful/              # Successful execution scenarios
├── failed_first/            # First step failure scenarios
├── failed_intermediate/     # Intermediate step failure scenarios
├── failed_last/             # Last step failure scenarios
├── interrupted/             # Interrupted execution scenarios
├── multiple_sequences/      # Multi-sequence with mixed results
└── hooks/                   # Hook error scenarios
    └── scripts/             # Hook scripts for testing
```

## Test Scenarios

### 1. Successful Execution (`successful/`)

**Test Case:** `TEST_SUCCESS_001`

**Purpose:** Validates verifier behavior when all steps execute successfully and pass all verifications.

**Scenario:**
- 1 test sequence with 3 steps
- All steps execute and pass exit code and output verification
- Expected verifier result: PASS

**Expected Verifier Behavior:**
- All steps should be marked as passed
- Sequence should be marked as passed
- Overall test case should be marked as passed
- No failure reasons should be reported

---

### 2. Failed First Step (`failed_first/`)

**Test Case:** `TEST_FAILED_FIRST_001`

**Purpose:** Validates verifier behavior when the first step fails, preventing subsequent steps from executing.

**Scenario:**
- 1 test sequence with 4 steps
- Step 1 fails (mkdir to invalid path returns exit code 1, expected 0)
- Steps 2-4 are not executed
- Expected verifier result: FAIL

**Expected Verifier Behavior:**
- Step 1 should be marked as failed with exit code mismatch reason
- Steps 2-4 should be marked as not executed
- Sequence should be marked as failed
- Overall test case should be marked as failed
- Failure should indicate step 1 failed verification

---

### 3. Failed Intermediate Step (`failed_intermediate/`)

**Test Case:** `TEST_FAILED_INTERMEDIATE_001`

**Purpose:** Validates verifier behavior when a step in the middle of a sequence fails.

**Scenario:**
- 1 test sequence with 5 steps
- Steps 1-2 execute successfully
- Step 3 fails (nonexistent command returns exit code 127, expected 0)
- Steps 4-5 are not executed
- Expected verifier result: FAIL

**Expected Verifier Behavior:**
- Steps 1-2 should be marked as passed
- Step 3 should be marked as failed with exit code mismatch reason
- Steps 4-5 should be marked as not executed
- Sequence should be marked as failed
- Overall test case should be marked as failed

---

### 4. Failed Last Step (`failed_last/`)

**Test Case:** `TEST_FAILED_LAST_001`

**Purpose:** Validates verifier behavior when only the final step fails due to output mismatch.

**Scenario:**
- 1 test sequence with 4 steps
- Steps 1-3 execute successfully
- Step 4 fails (output is "FAILURE" but verification expects "SUCCESS")
- Expected verifier result: FAIL

**Expected Verifier Behavior:**
- Steps 1-3 should be marked as passed
- Step 4 should be marked as failed with output verification mismatch reason
- Sequence should be marked as failed
- Overall test case should be marked as failed
- Failure should clearly indicate output verification failed for step 4

---

### 5. Interrupted Execution (`interrupted/`)

**Test Case:** `TEST_INTERRUPTED_001`

**Purpose:** Validates verifier behavior when test execution is interrupted and only partial steps complete.

**Scenario:**
- 2 test sequences defined
- Sequence 1: All 3 steps executed successfully
- Sequence 2: Not executed at all (interrupted before reaching it)
- Expected verifier result: FAIL (incomplete execution)

**Expected Verifier Behavior:**
- Sequence 1 steps 1-3 should be marked as passed
- Sequence 2 steps should be marked as not executed
- Sequence 1 should be marked as passed
- Sequence 2 should be marked as incomplete/not executed
- Overall test case should be marked as failed due to incomplete execution

---

### 6. Multiple Sequences with Mixed Results (`multiple_sequences/`)

**Test Case:** `TEST_MULTI_SEQ_001`

**Purpose:** Validates verifier behavior with multiple sequences where some pass and some fail.

**Scenario:**
- 3 test sequences defined
- Sequence 1: Both steps execute successfully (PASS)
- Sequence 2: Step 1 passes, step 2 fails with output mismatch (FAIL)
- Sequence 3: Not executed due to sequence 2 failure
- Expected verifier result: FAIL

**Expected Verifier Behavior:**
- Sequence 1 should be fully passed
- Sequence 2 step 1 should be passed
- Sequence 2 step 2 should be failed with output mismatch
- Sequence 3 steps should be marked as not executed
- Sequence 1 marked as passed
- Sequence 2 marked as failed
- Sequence 3 marked as not executed
- Overall test case should be marked as failed

---

### 7. Hook Error Scenarios (`hooks/`)

These scenarios test verifier behavior when hooks fail at different lifecycle points.

#### 7.1 Script Start Hook Error

**Test Case:** `TEST_HOOK_SCRIPT_START_001`

**Purpose:** Validates behavior when script_start hook fails immediately.

**Scenario:**
- script_start hook exits with error code 1
- No test sequences execute
- Expected verifier result: FAIL

**Expected Verifier Behavior:**
- All steps should be marked as not executed
- Overall test should be marked as failed
- Failure should indicate script_start hook error

---

#### 7.2 Setup Test Hook Error

**Test Case:** `TEST_HOOK_SETUP_TEST_001`

**Purpose:** Validates behavior when setup_test hook references non-existent script.

**Scenario:**
- setup_test hook references non-existent script
- No test sequences execute
- Expected verifier result: FAIL

**Expected Verifier Behavior:**
- All steps should be marked as not executed
- Overall test should be marked as failed
- Failure should indicate setup_test hook error

---

#### 7.3 Before Sequence Hook Error

**Test Case:** `TEST_HOOK_BEFORE_SEQ_001`

**Purpose:** Validates behavior when before_sequence hook fails on second sequence.

**Scenario:**
- 3 test sequences defined
- Sequence 1: before_sequence hook succeeds, both steps execute successfully
- Sequence 2: before_sequence hook fails, no steps execute
- Sequence 3: Not executed due to hook failure
- Expected verifier result: FAIL

**Expected Verifier Behavior:**
- Sequence 1 should be fully passed
- Sequence 2 steps should be marked as not executed
- Sequence 3 steps should be marked as not executed
- Failure should indicate before_sequence hook error for sequence 2

---

#### 7.4 After Sequence Hook Error

**Test Case:** `TEST_HOOK_AFTER_SEQ_001`

**Purpose:** Validates behavior when after_sequence hook fails after first sequence.

**Scenario:**
- 3 test sequences defined
- Sequence 1: All steps execute successfully, after_sequence hook fails
- Sequence 2-3: Not executed due to hook failure
- Expected verifier result: FAIL

**Expected Verifier Behavior:**
- Sequence 1 steps should be marked as passed
- Sequence 2-3 steps should be marked as not executed
- Sequence 1 marked as passed (steps completed before hook failed)
- Failure should indicate after_sequence hook error after sequence 1

---

#### 7.5 Before Step Hook Error

**Test Case:** `TEST_HOOK_BEFORE_STEP_001`

**Purpose:** Validates behavior when before_step hook fails before step 3.

**Scenario:**
- 2 test sequences defined
- Sequence 1: Steps 1-2 execute successfully, before_step hook fails before step 3
- Steps 3-4 not executed
- Sequence 2: Not executed
- Expected verifier result: FAIL

**Expected Verifier Behavior:**
- Sequence 1 steps 1-2 should be marked as passed
- Sequence 1 steps 3-4 should be marked as not executed
- Sequence 2 steps should be marked as not executed
- Failure should indicate before_step hook error before step 3

---

#### 7.6 After Step Hook Error

**Test Case:** `TEST_HOOK_AFTER_STEP_001`

**Purpose:** Validates behavior when after_step hook fails after step 2.

**Scenario:**
- 2 test sequences defined
- Sequence 1: Steps 1-2 execute successfully, after_step hook fails after step 2
- Steps 3-4 not executed
- Sequence 2: Not executed
- Expected verifier result: FAIL

**Expected Verifier Behavior:**
- Sequence 1 steps 1-2 should be marked as passed
- Sequence 1 steps 3-4 should be marked as not executed
- Sequence 2 steps should be marked as not executed
- Failure should indicate after_step hook error after step 2

---

#### 7.7 Teardown Test Hook Error

**Test Case:** `TEST_HOOK_TEARDOWN_001`

**Purpose:** Validates behavior when teardown_test hook fails during cleanup.

**Scenario:**
- 3 test sequences, all execute successfully
- teardown_test hook fails after all sequences complete
- Expected verifier result: FAIL (or PASS depending on implementation)

**Expected Verifier Behavior:**
- All sequence steps should be marked as passed
- All sequences should be marked as passed
- Overall result depends on whether teardown failures fail the test
- Failure should indicate teardown_test hook error

---

#### 7.8 Script End Hook Error

**Test Case:** `TEST_HOOK_SCRIPT_END_001`

**Purpose:** Validates behavior when script_end hook fails at final termination.

**Scenario:**
- 3 test sequences, all execute successfully
- script_end hook fails at the very end
- Expected verifier result: FAIL (or PASS depending on implementation)

**Expected Verifier Behavior:**
- All sequence steps should be marked as passed
- All sequences should be marked as passed
- Overall result depends on whether script_end failures fail the test
- Failure should indicate script_end hook error

---

## Running Verification Tests

### Prerequisites

Build the project and ensure the `verifier` binary is available:

```bash
make build
```

### Single File Verification

Verify a single execution log against its test case:

```bash
cargo run --bin verifier -- \
  --log testcases/verifier_scenarios/successful/TEST_SUCCESS_001_execution_log.json \
  --test-case TEST_SUCCESS_001 \
  --format yaml
```

**Parameters:**
- `--log` or `-l`: Path to the execution log JSON file
- `--test-case` or `-c`: Test case ID to verify against
- `--format` or `-F`: Output format (`yaml` or `json`, default: `yaml`)
- `--output` or `-o`: Output file path (optional, defaults to stdout)
- `--test-case-dir` or `-d`: Path to test case storage directory (default: `testcases`)

**Example with output file:**

```bash
cargo run --bin verifier -- \
  --log testcases/verifier_scenarios/failed_first/TEST_FAILED_FIRST_001_execution_log.json \
  --test-case TEST_FAILED_FIRST_001 \
  --format json \
  --output /tmp/verification_result.json
```

### Folder Discovery Mode

Verify all execution logs in a directory tree:

```bash
cargo run --bin verifier -- \
  --folder testcases/verifier_scenarios \
  --format yaml
```

This will:
1. Recursively search the folder for all `*_execution_log.json` files
2. Extract the test case ID from each filename (e.g., `TEST_SUCCESS_001` from `TEST_SUCCESS_001_execution_log.json`)
3. Load the corresponding test case definition
4. Verify the execution log against the test case
5. Generate a batch report with all results

**Example with specific subdirectory:**

```bash
cargo run --bin verifier -- \
  --folder testcases/verifier_scenarios/hooks \
  --format yaml \
  --output /tmp/hooks_verification.yaml
```

### Using the Makefile

If there's a make target for verification (check your Makefile):

```bash
make verify-scenarios
```

### Exit Codes

- **0**: All tests passed verification
- **1**: One or more tests failed verification

### Understanding Verification Output

#### YAML Format (Default)

```yaml
summary:
  total_test_cases: 1
  passed_test_cases: 0
  failed_test_cases: 1
  total_steps: 4
  passed_steps: 0
  failed_steps: 1
  not_executed_steps: 3

test_cases:
  - test_case_id: TEST_FAILED_FIRST_001
    overall_pass: false
    total_steps: 4
    passed_steps: 0
    failed_steps: 1
    not_executed_steps: 3
    sequences:
      - sequence_id: 1
        name: "Failed First Step Sequence"
        all_steps_passed: false
        step_results:
          - status: fail
            step: 1
            description: "Attempt to create directory (fails)"
            reason: "Exit code mismatch: expected 0, got 1"
          - status: not_executed
            step: 2
            description: "Create a file (not executed)"
          # ... more steps
```

#### JSON Format

```json
{
  "summary": {
    "total_test_cases": 1,
    "passed_test_cases": 0,
    "failed_test_cases": 1,
    "total_steps": 4,
    "passed_steps": 0,
    "failed_steps": 1,
    "not_executed_steps": 3
  },
  "test_cases": [
    {
      "test_case_id": "TEST_FAILED_FIRST_001",
      "overall_pass": false,
      "total_steps": 4,
      "passed_steps": 0,
      "failed_steps": 1,
      "not_executed_steps": 3,
      "sequences": [...]
    }
  ]
}
```

## Creating New Test Scenarios

To add a new verifier test scenario:

1. **Create the test case YAML file:**
   ```bash
   # Choose appropriate subdirectory based on scenario type
   touch testcases/verifier_scenarios/your_category/TEST_YOUR_SCENARIO_001.yml
   ```

2. **Define the test case:** Write a complete test case definition following the project schema.

3. **Create the execution log:**
   ```bash
   touch testcases/verifier_scenarios/your_category/TEST_YOUR_SCENARIO_001_execution_log.json
   ```

4. **Populate the execution log:** Create a JSON array of execution log entries that simulate the desired execution outcome:
   ```json
   [
     {
       "test_sequence": 1,
       "step": 1,
       "command": "echo \"test\"",
       "exit_code": 0,
       "output": "test",
       "timestamp": "2024-01-15T10:00:00.000000+00:00"
     }
   ]
   ```

5. **Test the scenario:**
   ```bash
   cargo run --bin verifier -- \
     --log testcases/verifier_scenarios/your_category/TEST_YOUR_SCENARIO_001_execution_log.json \
     --test-case TEST_YOUR_SCENARIO_001 \
     --format yaml
   ```

## Execution Log Format

Each execution log entry contains:

- `test_sequence` (integer): The sequence ID (must match test case)
- `step` (integer): The step number within the sequence
- `command` (string): The command that was executed
- `exit_code` (integer): The exit code returned by the command
- `output` (string): The output produced by the command
- `timestamp` (string): ISO 8601 timestamp with timezone

**Important Notes:**
- Logs must be valid JSON array format
- Entries should be in chronological/sequential order
- Only include entries for steps that actually executed
- Exit codes and output must match what actually happened (not what was expected)

## Troubleshooting

### Verifier can't find test case

**Error:** `Failed to load test case: TEST_XXX_001`

**Solution:** Ensure the test case ID in the execution log filename matches a test case in the testcases directory. The verifier searches recursively from the `--test-case-dir` path.

### Invalid JSON in execution log

**Error:** `Failed to parse test execution log`

**Solution:** Validate your JSON:
```bash
python3 -m json.tool testcases/verifier_scenarios/your_scenario/TEST_XXX_001_execution_log.json
```

### Folder mode finds no files

**Error:** `No execution log files (*_execution_log.json) found in folder`

**Solution:** Ensure your execution log files follow the naming convention: `{test_case_id}_execution_log.json`

### Unexpected verification failure

**Solution:** Compare the execution log values against the test case expectations:
- Check exit codes match
- Check output strings match exactly (including whitespace)
- Verify verification expressions would pass with the logged output
- Ensure steps are in the correct sequence

## CI/CD Integration

To integrate verifier tests in CI/CD pipelines:

```bash
#!/bin/bash
set -e

# Run verifier on all scenarios
cargo run --bin verifier -- \
  --folder testcases/verifier_scenarios \
  --format json \
  --output verification_results.json

# Check exit code
if [ $? -ne 0 ]; then
  echo "Verification failed! Check verification_results.json for details"
  exit 1
fi

echo "All verifier scenarios passed!"
```

## Additional Resources

- **Main Documentation:** See root `README.md` and `AGENTS.md` for project overview
- **Test Case Schema:** See schema documentation for test case YAML structure
- **Verification Module:** See `src/verification.rs` for verification logic implementation
- **Verifier Source:** See `src/bin/verifier.rs` for verifier CLI implementation
