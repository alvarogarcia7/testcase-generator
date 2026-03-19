# Precomputed Match Strategy Test Cases Summary

This document describes all test cases created for validating the `--match-strategy precomputed` option in the verifier.

## Overview

The precomputed match strategy allows verification to be performed based on pre-computed verification results stored in the execution log JSON files. Instead of evaluating verification expressions at runtime, the verifier checks the `result_verification_pass` and `output_verification_pass` fields in the execution log.

**Total Test Cases:** 19  
**Created:** 2026-03-01

---

## Test Case Categories

### 1. Successful Cases (7 test cases)

These test cases demonstrate successful verification scenarios where all steps pass.

#### TEST_PRECOMP_ALL_PASS_001
- **Location:** `successful/TEST_PRECOMP_ALL_PASS_001.yml`
- **Description:** All steps pass with precomputed result and output verification
- **Test Sequences:** 1
- **Total Steps:** 3
- **Expected Outcome:** ✅ PASS - All 3 steps should pass verification
- **Verification Details:**
  - Step 1: `result_verification_pass=true`, `output_verification_pass=true`
  - Step 2: `result_verification_pass=true`, `output_verification_pass=true`
  - Step 3: `result_verification_pass=true`, `output_verification_pass=true`

#### TEST_PRECOMP_SINGLE_STEP_001
- **Location:** `successful/TEST_PRECOMP_SINGLE_STEP_001.yml`
- **Description:** Single step passing all verifications
- **Test Sequences:** 1
- **Total Steps:** 1
- **Expected Outcome:** ✅ PASS - Single step should pass verification
- **Verification Details:**
  - Step 1: `result_verification_pass=true`, `output_verification_pass=true`

#### TEST_PRECOMP_EMPTY_OUTPUT_001
- **Location:** `successful/TEST_PRECOMP_EMPTY_OUTPUT_001.yml`
- **Description:** Steps with empty output pass verification
- **Test Sequences:** 1
- **Total Steps:** 3
- **Expected Outcome:** ✅ PASS - All 3 steps with empty output should pass
- **Verification Details:**
  - All steps execute silent commands (`true`, `:`) with empty output
  - All steps: `result_verification_pass=true`, `output_verification_pass=true`

#### TEST_PRECOMP_LONG_OUTPUT_001
- **Location:** `successful/TEST_PRECOMP_LONG_OUTPUT_001.yml`
- **Description:** Steps with longer multi-line output
- **Test Sequences:** 1
- **Total Steps:** 2
- **Expected Outcome:** ✅ PASS - All steps with multi-line output should pass
- **Verification Details:**
  - Step 1: Multi-line output with newlines
  - Step 2: Longer single-line output
  - Both steps: `result_verification_pass=true`, `output_verification_pass=true`

#### TEST_PRECOMP_FIVE_STEPS_001
- **Location:** `successful/TEST_PRECOMP_FIVE_STEPS_001.yml`
- **Description:** Five steps all passing
- **Test Sequences:** 1
- **Total Steps:** 5
- **Expected Outcome:** ✅ PASS - All 5 steps should pass verification
- **Verification Details:**
  - All 5 steps: `result_verification_pass=true`, `output_verification_pass=true`

#### TEST_PRECOMP_SPECIAL_CHARS_001
- **Location:** `successful/TEST_PRECOMP_SPECIAL_CHARS_001.yml`
- **Description:** Output with special characters
- **Test Sequences:** 1
- **Total Steps:** 2
- **Expected Outcome:** ✅ PASS - All steps with special characters should pass
- **Verification Details:**
  - Step 1: Output contains quotes (`"`)
  - Step 2: Output contains special symbols (`$`, `@`, `#`)
  - Both steps: `result_verification_pass=true`, `output_verification_pass=true`

#### TEST_PRECOMP_NUMERIC_OUTPUT_001
- **Location:** `edge_cases/TEST_PRECOMP_NUMERIC_OUTPUT_001.yml`
- **Description:** Steps with numeric output values
- **Test Sequences:** 1
- **Total Steps:** 3
- **Expected Outcome:** ✅ PASS - All steps with numeric output should pass
- **Verification Details:**
  - Step 1: Simple number (42)
  - Step 2: Decimal number (3.14159)
  - Step 3: Negative number (-100)
  - All steps: `result_verification_pass=true`, `output_verification_pass=true`

---

### 2. Failed First Cases (3 test cases)

These test cases demonstrate failures in the first step.

#### TEST_PRECOMP_RESULT_FAIL_001
- **Location:** `failed_first/TEST_PRECOMP_RESULT_FAIL_001.yml`
- **Description:** First step fails result verification
- **Test Sequences:** 1
- **Total Steps:** 2
- **Expected Outcome:** ❌ FAIL - First step fails, second step passes
- **Verification Details:**
  - Step 1: `result_verification_pass=false`, `output_verification_pass=true`
  - Step 2: `result_verification_pass=true`, `output_verification_pass=true`
- **Failure Reason:** Result verification failed (precomputed)

#### TEST_PRECOMP_OUTPUT_FAIL_001
- **Location:** `failed_first/TEST_PRECOMP_OUTPUT_FAIL_001.yml`
- **Description:** First step fails output verification
- **Test Sequences:** 1
- **Total Steps:** 2
- **Expected Outcome:** ❌ FAIL - First step fails, second step passes
- **Verification Details:**
  - Step 1: `result_verification_pass=true`, `output_verification_pass=false`
  - Step 2: `result_verification_pass=true`, `output_verification_pass=true`
- **Failure Reason:** Output verification failed (precomputed)

#### TEST_PRECOMP_BOTH_FAIL_001
- **Location:** `failed_first/TEST_PRECOMP_BOTH_FAIL_001.yml`
- **Description:** First step fails both result and output verification
- **Test Sequences:** 1
- **Total Steps:** 2
- **Expected Outcome:** ❌ FAIL - First step fails both verifications, second step passes
- **Verification Details:**
  - Step 1: `result_verification_pass=false`, `output_verification_pass=false`
  - Step 2: `result_verification_pass=true`, `output_verification_pass=true`
- **Failure Reason:** Both result and output verification failed (precomputed)

---

### 3. Failed Last Cases (1 test case)

These test cases demonstrate failures in the last step.

#### TEST_PRECOMP_LAST_FAIL_001
- **Location:** `failed_last/TEST_PRECOMP_LAST_FAIL_001.yml`
- **Description:** Last step fails verification
- **Test Sequences:** 1
- **Total Steps:** 3
- **Expected Outcome:** ❌ FAIL - First two steps pass, last step fails
- **Verification Details:**
  - Step 1: `result_verification_pass=true`, `output_verification_pass=true`
  - Step 2: `result_verification_pass=true`, `output_verification_pass=true`
  - Step 3: `result_verification_pass=false`, `output_verification_pass=true`
- **Failure Reason:** Result verification failed (precomputed) on last step

---

### 4. Failed Intermediate Cases (2 test cases)

These test cases demonstrate failures in middle steps.

#### TEST_PRECOMP_MID_FAIL_001
- **Location:** `failed_intermediate/TEST_PRECOMP_MID_FAIL_001.yml`
- **Description:** Middle step fails verification
- **Test Sequences:** 1
- **Total Steps:** 3
- **Expected Outcome:** ❌ FAIL - First and last steps pass, middle step fails
- **Verification Details:**
  - Step 1: `result_verification_pass=true`, `output_verification_pass=true`
  - Step 2: `result_verification_pass=true`, `output_verification_pass=false`
  - Step 3: `result_verification_pass=true`, `output_verification_pass=true`
- **Failure Reason:** Output verification failed (precomputed) on middle step

#### TEST_PRECOMP_TWO_FAILURES_001
- **Location:** `failed_intermediate/TEST_PRECOMP_TWO_FAILURES_001.yml`
- **Description:** Two steps fail in different ways
- **Test Sequences:** 1
- **Total Steps:** 4
- **Expected Outcome:** ❌ FAIL - Steps 2 and 4 fail, steps 1 and 3 pass
- **Verification Details:**
  - Step 1: `result_verification_pass=true`, `output_verification_pass=true`
  - Step 2: `result_verification_pass=false`, `output_verification_pass=true`
  - Step 3: `result_verification_pass=true`, `output_verification_pass=true`
  - Step 4: `result_verification_pass=true`, `output_verification_pass=false`
- **Failure Reason:** Step 2 fails result verification, step 4 fails output verification

---

### 5. Multiple Sequences Cases (2 test cases)

These test cases demonstrate verification across multiple test sequences.

#### TEST_PRECOMP_MULTI_SEQ_001
- **Location:** `multiple_sequences/TEST_PRECOMP_MULTI_SEQ_001.yml`
- **Description:** Multiple sequences with mixed results
- **Test Sequences:** 2
- **Total Steps:** 4 (2 in seq 1, 2 in seq 2)
- **Expected Outcome:** ❌ FAIL - Sequence 1 passes, sequence 2 fails
- **Verification Details:**
  - Sequence 1, Step 1: `result_verification_pass=true`, `output_verification_pass=true`
  - Sequence 1, Step 2: `result_verification_pass=true`, `output_verification_pass=true`
  - Sequence 2, Step 1: `result_verification_pass=true`, `output_verification_pass=true`
  - Sequence 2, Step 2: `result_verification_pass=false`, `output_verification_pass=true`
- **Failure Reason:** Sequence 2, step 2 fails result verification

#### TEST_PRECOMP_THREE_SEQ_001
- **Location:** `multiple_sequences/TEST_PRECOMP_THREE_SEQ_001.yml`
- **Description:** Three sequences with varying results
- **Test Sequences:** 3
- **Total Steps:** 4 (1 in seq 1, 2 in seq 2, 1 in seq 3)
- **Expected Outcome:** ❌ FAIL - Sequences 1 and 3 pass, sequence 2 fails
- **Verification Details:**
  - Sequence 1, Step 1: `result_verification_pass=true`, `output_verification_pass=true`
  - Sequence 2, Step 1: `result_verification_pass=true`, `output_verification_pass=true`
  - Sequence 2, Step 2: `result_verification_pass=false`, `output_verification_pass=true`
  - Sequence 3, Step 1: `result_verification_pass=true`, `output_verification_pass=true`
- **Failure Reason:** Sequence 2, step 2 fails result verification

---

### 6. Edge Cases (4 test cases)

These test cases demonstrate edge cases and unusual scenarios.

#### TEST_PRECOMP_PARTIAL_EXEC_001
- **Location:** `edge_cases/TEST_PRECOMP_PARTIAL_EXEC_001.yml`
- **Description:** Partial execution with missing steps
- **Test Sequences:** 1
- **Total Steps:** 3 (defined), 2 (executed)
- **Expected Outcome:** ❌ FAIL - Steps 1 and 3 pass, step 2 not executed
- **Verification Details:**
  - Step 1: `result_verification_pass=true`, `output_verification_pass=true`
  - Step 2: NOT EXECUTED (missing from log)
  - Step 3: `result_verification_pass=true`, `output_verification_pass=true`
- **Failure Reason:** Step 2 was not executed

#### TEST_PRECOMP_MIXED_RESULTS_001
- **Location:** `edge_cases/TEST_PRECOMP_MIXED_RESULTS_001.yml`
- **Description:** Mixed pass and fail results across steps
- **Test Sequences:** 1
- **Total Steps:** 4
- **Expected Outcome:** ❌ FAIL - Steps 1 and 3 pass, steps 2 and 4 fail
- **Verification Details:**
  - Step 1: `result_verification_pass=true`, `output_verification_pass=true`
  - Step 2: `result_verification_pass=false`, `output_verification_pass=true`
  - Step 3: `result_verification_pass=true`, `output_verification_pass=true`
  - Step 4: `result_verification_pass=true`, `output_verification_pass=false`
- **Failure Reason:** Step 2 fails result, step 4 fails output verification

#### TEST_PRECOMP_ALL_FAIL_001
- **Location:** `edge_cases/TEST_PRECOMP_ALL_FAIL_001.yml`
- **Description:** All steps fail verification
- **Test Sequences:** 1
- **Total Steps:** 3
- **Expected Outcome:** ❌ FAIL - All steps fail verification
- **Verification Details:**
  - Step 1: `result_verification_pass=false`, `output_verification_pass=true`
  - Step 2: `result_verification_pass=true`, `output_verification_pass=false`
  - Step 3: `result_verification_pass=false`, `output_verification_pass=false`
- **Failure Reason:** All steps fail in different ways

#### TEST_PRECOMP_ONLY_RESULT_PASS_001
- **Location:** `edge_cases/TEST_PRECOMP_ONLY_RESULT_PASS_001.yml`
- **Description:** Only result_verification_pass field set
- **Test Sequences:** 1
- **Total Steps:** 2
- **Expected Outcome:** ❌ FAIL - All steps fail output verification
- **Verification Details:**
  - Step 1: `result_verification_pass=true`, `output_verification_pass` not set (null)
  - Step 2: `result_verification_pass=true`, `output_verification_pass` not set (null)
- **Failure Reason:** Output verification field missing (treated as fail)

---

## Usage

To run verification with precomputed strategy:

```bash
# Single test case
./target/debug/verifier \
  --log testcases/verifier_scenarios/successful/TEST_PRECOMP_ALL_PASS_001_execution_log.json \
  --test-case TEST_PRECOMP_ALL_PASS_001 \
  --match-strategy precomputed \
  --format yaml

# Folder discovery mode
./target/debug/verifier \
  --folder testcases/verifier_scenarios/successful \
  --match-strategy precomputed \
  --format yaml
```

---

## Key Features Tested

### Verification Field Combinations
- ✅ Both `result_verification_pass` and `output_verification_pass` = true (pass)
- ❌ `result_verification_pass` = false (fail)
- ❌ `output_verification_pass` = false (fail)
- ❌ Both fields = false (fail)
- ❌ One or both fields missing/null (fail)

### Output Types
- Empty output
- Single-line output
- Multi-line output
- Special characters (quotes, symbols)
- Numeric values (integers, decimals, negatives)

### Execution Scenarios
- All steps executed and pass
- First step fails
- Last step fails
- Middle step fails
- Multiple steps fail
- Steps not executed (missing from log)
- Multiple test sequences
- Partial execution

### Success Field Handling
In precomputed mode, the `success` field in expected values is **ignored**. The verifier only checks the `result_verification_pass` and `output_verification_pass` fields from the execution log.

---

## Expected Test Results Summary

| Test Case | Total Steps | Pass | Fail | Not Executed | Overall Result |
|-----------|-------------|------|------|--------------|----------------|
| TEST_PRECOMP_ALL_PASS_001 | 3 | 3 | 0 | 0 | ✅ PASS |
| TEST_PRECOMP_SINGLE_STEP_001 | 1 | 1 | 0 | 0 | ✅ PASS |
| TEST_PRECOMP_EMPTY_OUTPUT_001 | 3 | 3 | 0 | 0 | ✅ PASS |
| TEST_PRECOMP_LONG_OUTPUT_001 | 2 | 2 | 0 | 0 | ✅ PASS |
| TEST_PRECOMP_FIVE_STEPS_001 | 5 | 5 | 0 | 0 | ✅ PASS |
| TEST_PRECOMP_SPECIAL_CHARS_001 | 2 | 2 | 0 | 0 | ✅ PASS |
| TEST_PRECOMP_NUMERIC_OUTPUT_001 | 3 | 3 | 0 | 0 | ✅ PASS |
| TEST_PRECOMP_RESULT_FAIL_001 | 2 | 1 | 1 | 0 | ❌ FAIL |
| TEST_PRECOMP_OUTPUT_FAIL_001 | 2 | 1 | 1 | 0 | ❌ FAIL |
| TEST_PRECOMP_BOTH_FAIL_001 | 2 | 1 | 1 | 0 | ❌ FAIL |
| TEST_PRECOMP_LAST_FAIL_001 | 3 | 2 | 1 | 0 | ❌ FAIL |
| TEST_PRECOMP_MID_FAIL_001 | 3 | 2 | 1 | 0 | ❌ FAIL |
| TEST_PRECOMP_TWO_FAILURES_001 | 4 | 2 | 2 | 0 | ❌ FAIL |
| TEST_PRECOMP_MULTI_SEQ_001 | 4 | 3 | 1 | 0 | ❌ FAIL |
| TEST_PRECOMP_THREE_SEQ_001 | 4 | 3 | 1 | 0 | ❌ FAIL |
| TEST_PRECOMP_PARTIAL_EXEC_001 | 3 | 2 | 0 | 1 | ❌ FAIL |
| TEST_PRECOMP_MIXED_RESULTS_001 | 4 | 2 | 2 | 0 | ❌ FAIL |
| TEST_PRECOMP_ALL_FAIL_001 | 3 | 0 | 3 | 0 | ❌ FAIL |
| TEST_PRECOMP_ONLY_RESULT_PASS_001 | 2 | 0 | 2 | 0 | ❌ FAIL |

**Total:** 19 test cases  
**Expected to Pass:** 7  
**Expected to Fail:** 12

---

## Schema Validation Status

All 19 test cases have been validated against the test case schema and passed validation:

```
✓ TEST_PRECOMP_ALL_FAIL_001.yml
✓ TEST_PRECOMP_MIXED_RESULTS_001.yml
✓ TEST_PRECOMP_NUMERIC_OUTPUT_001.yml
✓ TEST_PRECOMP_ONLY_RESULT_PASS_001.yml
✓ TEST_PRECOMP_PARTIAL_EXEC_001.yml
✓ TEST_PRECOMP_BOTH_FAIL_001.yml
✓ TEST_PRECOMP_OUTPUT_FAIL_001.yml
✓ TEST_PRECOMP_RESULT_FAIL_001.yml
✓ TEST_PRECOMP_MID_FAIL_001.yml
✓ TEST_PRECOMP_TWO_FAILURES_001.yml
✓ TEST_PRECOMP_LAST_FAIL_001.yml
✓ TEST_PRECOMP_MULTI_SEQ_001.yml
✓ TEST_PRECOMP_THREE_SEQ_001.yml
✓ TEST_PRECOMP_ALL_PASS_001.yml
✓ TEST_PRECOMP_EMPTY_OUTPUT_001.yml
✓ TEST_PRECOMP_FIVE_STEPS_001.yml
✓ TEST_PRECOMP_LONG_OUTPUT_001.yml
✓ TEST_PRECOMP_SINGLE_STEP_001.yml
✓ TEST_PRECOMP_SPECIAL_CHARS_001.yml
```

---

## Notes

1. **Precomputed Strategy Behavior**: The precomputed match strategy relies entirely on the `result_verification_pass` and `output_verification_pass` fields in the execution log. It does not evaluate verification expressions or check the `success` field.

2. **Missing Verification Fields**: If either `result_verification_pass` or `output_verification_pass` is missing (null), the verification for that aspect is treated as failed.

3. **Test Case Organization**: Test cases are organized into folders based on their behavior:
   - `successful/` - All steps pass
   - `failed_first/` - First step fails
   - `failed_last/` - Last step fails
   - `failed_intermediate/` - Middle steps fail
   - `multiple_sequences/` - Multiple test sequences
   - `edge_cases/` - Special scenarios and edge cases

4. **Execution Logs**: Each test case has a corresponding `_execution_log.json` file that contains the precomputed verification results.
