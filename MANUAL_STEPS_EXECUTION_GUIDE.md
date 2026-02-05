# Manual Steps Test Execution Guide

This guide provides step-by-step instructions to execute all 10 manual step test cases and verify the behavior.

## Prerequisites

1. Build the project:
```bash
cargo build --release
```

2. Verify test cases exist:
```bash
ls -l testcases/examples/manual_steps/
```

Expected output:
```
TC_MANUAL_API_007.yaml
TC_MANUAL_BACKUP_009.yaml
TC_MANUAL_DATABASE_006.yaml
TC_MANUAL_DEVICE_004.yaml
TC_MANUAL_HARDWARE_002.yaml
TC_MANUAL_MIXED_010.yaml
TC_MANUAL_NETWORK_005.yaml
TC_MANUAL_SECURITY_008.yaml
TC_MANUAL_SSH_001.yaml
TC_MANUAL_UI_003.yaml
```

## Executing Individual Test Cases

### Test Case 1: SSH Device Connection Test
```bash
./target/release/test-executor execute testcases/examples/manual_steps/TC_MANUAL_SSH_001.yaml
```

**Expected Console Output:**
- Step 1: `[RUN]` then `[FAIL]` (network not reachable in test environment)
- Steps 2, 3, 5: `[SKIP] ... - Manual step`
- Step 4: `[RUN]` then `[PASS]`

**JSON Log Location:** `TC_MANUAL_SSH_001_execution_log.json`

**Expected JSON Content:** Only steps 1 and 4 (automated steps)

---

### Test Case 2: Hardware Connection Test
```bash
./target/release/test-executor execute testcases/examples/manual_steps/TC_MANUAL_HARDWARE_002.yaml
```

**Expected Console Output:**
- Steps 1, 2, 3: `[SKIP] ... - Manual step`
- Steps 4, 5: `[RUN]` then `[PASS]`
- Final: `All test sequences completed successfully`

**JSON Log Location:** `TC_MANUAL_HARDWARE_002_execution_log.json`

**Expected JSON Content:** Only steps 4 and 5 (automated steps)

---

### Test Case 3: UI Navigation Test
```bash
./target/release/test-executor execute testcases/examples/manual_steps/TC_MANUAL_UI_003.yaml
```

**Expected Console Output:**
- Step 1: `[RUN]` then `[FAIL]` (server not running in test environment)
- Steps 2, 3, 4: `[SKIP] ... - Manual step`
- Step 5: `[RUN]` then `[PASS]`

**JSON Log Location:** `TC_MANUAL_UI_003_execution_log.json`

**Expected JSON Content:** Only steps 1 and 5 (automated steps)

---

### Test Case 4: Device Power Cycle Test
```bash
./target/release/test-executor execute testcases/examples/manual_steps/TC_MANUAL_DEVICE_004.yaml
```

**Expected Console Output:**
- Step 1: `[RUN]` then `[PASS]`
- Steps 2, 3: `[SKIP] ... - Manual step`
- Step 4: `[RUN]` then `[PASS]`
- Step 5: `[SKIP] ... - Manual step`
- Final: `All test sequences completed successfully`

**JSON Log Location:** `TC_MANUAL_DEVICE_004_execution_log.json`

**Expected JSON Content:** Only steps 1 and 4 (automated steps)

---

### Test Case 5: Physical Network Connection Test
```bash
./target/release/test-executor execute testcases/examples/manual_steps/TC_MANUAL_NETWORK_005.yaml
```

**Expected Console Output:**
- Step 1: `[RUN]` then `[PASS]`
- Step 2: `[SKIP] ... - Manual step`
- Step 3: `[RUN]` then `[PASS]`
- Step 4: `[SKIP] ... - Manual step`
- Step 5: `[RUN]` then `[PASS]`
- Final: `All test sequences completed successfully`

**JSON Log Location:** `TC_MANUAL_NETWORK_005_execution_log.json`

**Expected JSON Content:** Only steps 1, 3, and 5 (automated steps)

---

### Test Case 6: Database Manual Inspection Test
```bash
./target/release/test-executor execute testcases/examples/manual_steps/TC_MANUAL_DATABASE_006.yaml
```

**Expected Console Output:**
- Steps 1, 2: `[RUN]` then `[PASS]`
- Steps 3, 4: `[SKIP] ... - Manual step`
- Step 5: `[RUN]` then `[PASS]`
- Final: `All test sequences completed successfully`

**JSON Log Location:** `TC_MANUAL_DATABASE_006_execution_log.json`

**Expected JSON Content:** Only steps 1, 2, and 5 (automated steps)

---

### Test Case 7: API Login Flow Test
```bash
./target/release/test-executor execute testcases/examples/manual_steps/TC_MANUAL_API_007.yaml
```

**Expected Console Output:**
- Step 1: `[RUN]` then `[PASS]`
- Steps 2, 3, 4: `[SKIP] ... - Manual step`
- Step 5: `[RUN]` then `[PASS]`
- Final: `All test sequences completed successfully`

**JSON Log Location:** `TC_MANUAL_API_007_execution_log.json`

**Expected JSON Content:** Only steps 1 and 5 (automated steps)

---

### Test Case 8: SSL Certificate Inspection Test
```bash
./target/release/test-executor execute testcases/examples/manual_steps/TC_MANUAL_SECURITY_008.yaml
```

**Expected Console Output:**
- Steps 1, 2: `[RUN]` then `[PASS]`
- Steps 3, 4, 5: `[SKIP] ... - Manual step`
- Final: `All test sequences completed successfully`

**JSON Log Location:** `TC_MANUAL_SECURITY_008_execution_log.json`

**Expected JSON Content:** Only steps 1 and 2 (automated steps)

---

### Test Case 9: Backup Restoration Test
```bash
./target/release/test-executor execute testcases/examples/manual_steps/TC_MANUAL_BACKUP_009.yaml
```

**Expected Console Output:**
- Steps 1, 2: `[RUN]` then `[PASS]`
- Steps 3, 4, 5: `[SKIP] ... - Manual step`
- Final: `All test sequences completed successfully`

**JSON Log Location:** `TC_MANUAL_BACKUP_009_execution_log.json`

**Expected JSON Content:** Only steps 1 and 2 (automated steps)

---

### Test Case 10: Mixed Automated and Manual Test
```bash
./target/release/test-executor execute testcases/examples/manual_steps/TC_MANUAL_MIXED_010.yaml
```

**Expected Console Output:**
- Step 1: `[RUN]` then `[PASS]`
- Step 2: `[SKIP] ... - Manual step`
- Step 3: `[RUN]` then `[PASS]`
- Step 4: `[SKIP] ... - Manual step`
- Step 5: `[RUN]` then `[PASS]`
- Final: `All test sequences completed successfully`

**JSON Log Location:** `TC_MANUAL_MIXED_010_execution_log.json`

**Expected JSON Content:** Only steps 1, 3, and 5 (automated steps)

---

## Executing All Tests at Once

Run all 10 tests sequentially:

```bash
for test in testcases/examples/manual_steps/TC_MANUAL_*.yaml; do
    echo "===================================="
    echo "Executing: $test"
    echo "===================================="
    ./target/release/test-executor execute "$test"
    echo ""
done
```

## Verifying JSON Execution Logs

### Check all JSON logs were created:
```bash
ls -1 TC_MANUAL_*_execution_log.json
```

Expected: 10 JSON files

### View a specific JSON log:
```bash
cat TC_MANUAL_HARDWARE_002_execution_log.json
```

### Verify manual steps are excluded from JSON:
```bash
# Count total steps in YAML (should be 5 for each test)
grep "^  - step:" testcases/examples/manual_steps/TC_MANUAL_HARDWARE_002.yaml | wc -l

# Count manual steps in YAML (should be 3)
grep "manual: true" testcases/examples/manual_steps/TC_MANUAL_HARDWARE_002.yaml | wc -l

# Count steps in JSON log (should be 2 = 5 total - 3 manual)
grep '"step":' TC_MANUAL_HARDWARE_002_execution_log.json | wc -l
```

### Check for manual steps in JSON (should return nothing):
```bash
jq '.[] | select(.step == 1 or .step == 2 or .step == 3)' TC_MANUAL_HARDWARE_002_execution_log.json
```

Expected: Empty output (because steps 1-3 are manual and excluded)

```bash
jq '.[] | select(.step == 4 or .step == 5)' TC_MANUAL_HARDWARE_002_execution_log.json
```

Expected: Two JSON objects for steps 4 and 5

## Verification Checklist

For each test case, verify:

- [ ] Console shows `[SKIP]` messages for manual steps
- [ ] Console shows `[RUN]` and `[PASS]`/`[FAIL]` for automated steps
- [ ] JSON log file was created
- [ ] JSON log contains only automated steps
- [ ] JSON log has correct step numbers (matching the YAML)
- [ ] JSON log has gaps in step numbers where manual steps were skipped
- [ ] Each JSON entry has: test_sequence, step, command, exit_code, output, timestamp

## Understanding the Output

### Console Skip Message Format:
```
[SKIP] Step {number} (Sequence {seq}): {description} - Manual step
```

### Console Run Message Format:
```
[RUN] Step {number} (Sequence {seq}): {description}
[PASS] Step {number} (Sequence {seq}): {description}
```

or

```
[RUN] Step {number} (Sequence {seq}): {description}
[FAIL] Step {number} (Sequence {seq}): {description}
  Command: {command}
  EXIT_CODE: {code}
  COMMAND_OUTPUT: {output}
  Result verification: {true|false}
  Output verification: {true|false}
```

### JSON Log Format:
```json
[
  {
    "test_sequence": 1,
    "step": 4,
    "command": "sleep 30 && lsusb | grep -i device || echo \"USB enumeration complete\"",
    "exit_code": 0,
    "output": "USB enumeration complete",
    "timestamp": "2026-02-05T18:55:30.347274+04:00"
  }
]
```

## Troubleshooting

### If test-executor not found:
```bash
cargo build --release
```

### If test case files not found:
```bash
# Verify you're in the project root
pwd
# Should show the testcase-generator directory

# List test cases
ls testcases/examples/manual_steps/
```

### If JSON logs are empty or missing:
- Check that test execution completed (even with failures)
- JSON logs are written after automated steps execute
- Manual-only test cases won't produce JSON logs (no automated steps to log)

## Summary

After executing all tests, you should have:
1. **10 YAML test case files** in `testcases/examples/manual_steps/`
2. **10 JSON execution log files** in the current directory
3. **Console output** showing manual steps being skipped
4. **JSON logs** containing only automated step results

This demonstrates that the manual step feature:
- Skips manual steps during execution
- Shows clear skip messages in console output
- Excludes manual steps from JSON execution logs
- Continues test execution despite manual steps being present
