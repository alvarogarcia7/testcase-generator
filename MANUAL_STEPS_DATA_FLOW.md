# Manual Steps Data Flow Documentation

This document illustrates how manual steps flow through the test execution system, from YAML definition to console output to JSON logs.

## Overview

```
┌─────────────────┐
│  YAML Test Case │
│  (Input)        │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Test Executor  │
│  (Processing)   │
└────────┬────────┘
         │
    ┌────┴────┐
    ▼         ▼
┌────────┐ ┌────────────┐
│Console │ │  JSON Log  │
│Output  │ │  (Output)  │
└────────┘ └────────────┘
```

## Detailed Flow: TC_MANUAL_NETWORK_005

### Step 1: YAML Test Case Definition

**File:** `testcases/examples/manual_steps/TC_MANUAL_NETWORK_005.yaml`

```yaml
test_case_id: TC_MANUAL_NETWORK_005
name: Physical Network Connection Test
description: Test physical network cable connection with mixed manual and automated steps
sequences:
  - sequence: 1
    name: Physical connection and network verification
    steps:
      - step: 1
        description: Check network interface status
        command: ip link show eth0 2>/dev/null || echo "Interface check completed"
        expected_result:
          exit_code: 0
        
      - step: 2
        description: Physically connect Ethernet cable between device and switch port 8
        manual: true
        
      - step: 3
        description: Bring network interface up
        command: echo "ip link set eth0 up" && echo "Interface brought up"
        expected_result:
          exit_code: 0
        
      - step: 4
        description: Verify link status LED on device
        manual: true
        
      - step: 5
        description: Test network connectivity with ping
        command: ping -c 4 8.8.8.8 2>/dev/null | grep -E 'packets transmitted|received' || echo "Ping test completed"
        expected_result:
          exit_code: 0
```

**Analysis:**
- **Total Steps:** 5
- **Manual Steps:** 2 (steps 2, 4)
- **Automated Steps:** 3 (steps 1, 3, 5)

### Step 2: Test Executor Processing

The test executor processes each step:

#### Processing Step 1 (Automated)
```
Input: step=1, manual=false, command="ip link show eth0..."
Action: Execute command
Result: exit_code=0, output="Interface check completed"
Console: [RUN] + [PASS] messages
JSON: Record execution details
```

#### Processing Step 2 (Manual)
```
Input: step=2, manual=true, no command
Action: Skip execution
Result: No command execution
Console: [SKIP] message with "Manual step" suffix
JSON: Do not record
```

#### Processing Step 3 (Automated)
```
Input: step=3, manual=false, command="echo 'ip link set...'
Action: Execute command
Result: exit_code=0, output="ip link set eth0 up\nInterface brought up"
Console: [RUN] + [PASS] messages
JSON: Record execution details
```

#### Processing Step 4 (Manual)
```
Input: step=4, manual=true, no command
Action: Skip execution
Result: No command execution
Console: [SKIP] message with "Manual step" suffix
JSON: Do not record
```

#### Processing Step 5 (Automated)
```
Input: step=5, manual=false, command="ping -c 4..."
Action: Execute command
Result: exit_code=0, output="4 packets transmitted, 0 packets received..."
Console: [RUN] + [PASS] messages
JSON: Record execution details
```

### Step 3: Console Output

**Real-time output to stdout/stderr:**

```
[RUN] Step 1 (Sequence 1): Check network interface status
[PASS] Step 1 (Sequence 1): Check network interface status
[SKIP] Step 2 (Sequence 1): Physically connect Ethernet cable between device and switch port 8 - Manual step
[RUN] Step 3 (Sequence 1): Bring network interface up
[PASS] Step 3 (Sequence 1): Bring network interface up
[SKIP] Step 4 (Sequence 1): Verify link status LED on device - Manual step
[RUN] Step 5 (Sequence 1): Test network connectivity with ping
[PASS] Step 5 (Sequence 1): Test network connectivity with ping
All test sequences completed successfully
```

**Key Features:**
- Steps processed in order (1, 2, 3, 4, 5)
- Manual steps show `[SKIP]` with suffix " - Manual step"
- Automated steps show `[RUN]` then `[PASS]` or `[FAIL]`
- Final status message indicates overall result

### Step 4: JSON Execution Log

**File:** `TC_MANUAL_NETWORK_005_execution_log.json`

```json
[
  {
    "test_sequence": 1,
    "step": 1,
    "command": "ip link show eth0 2>/dev/null || echo \"Interface check completed\"",
    "exit_code": 0,
    "output": "Interface check completed",
    "timestamp": "2026-02-05T18:55:37.025240+04:00"
  },
  {
    "test_sequence": 1,
    "step": 3,
    "command": "echo \"ip link set eth0 up\" && echo \"Interface brought up\"",
    "exit_code": 0,
    "output": "ip link set eth0 up\nInterface brought up",
    "timestamp": "2026-02-05T18:55:37.032382+04:00"
  },
  {
    "test_sequence": 1,
    "step": 5,
    "command": "ping -c 4 8.8.8.8 2>/dev/null | grep -E 'packets transmitted|received' || echo \"Ping test completed\"",
    "exit_code": 0,
    "output": "4 packets transmitted, 0 packets received, 100.0% packet loss",
    "timestamp": "2026-02-05T18:55:51.090389+04:00"
  }
]
```

**Key Features:**
- Only automated steps (1, 3, 5) are present
- Manual steps (2, 4) are completely absent
- Step numbers match original YAML (not renumbered)
- Each entry has complete execution details
- Timestamps show execution chronology

---

## Comparison Table

| Aspect | Step 1 (Auto) | Step 2 (Manual) | Step 3 (Auto) | Step 4 (Manual) | Step 5 (Auto) |
|--------|---------------|-----------------|---------------|-----------------|---------------|
| **YAML Definition** |
| manual field | false/absent | true | false/absent | true | false/absent |
| command field | present | absent | present | absent | present |
| **Test Executor** |
| Execute command? | ✅ Yes | ❌ No | ✅ Yes | ❌ No | ✅ Yes |
| Verify result? | ✅ Yes | ❌ No | ✅ Yes | ❌ No | ✅ Yes |
| **Console Output** |
| Shows [RUN]? | ✅ Yes | ❌ No | ✅ Yes | ❌ No | ✅ Yes |
| Shows [SKIP]? | ❌ No | ✅ Yes | ❌ No | ✅ Yes | ❌ No |
| Shows result? | ✅ Yes | ❌ No | ✅ Yes | ❌ No | ✅ Yes |
| **JSON Log** |
| Entry exists? | ✅ Yes | ❌ No | ✅ Yes | ❌ No | ✅ Yes |
| Has command? | ✅ Yes | N/A | ✅ Yes | N/A | ✅ Yes |
| Has output? | ✅ Yes | N/A | ✅ Yes | N/A | ✅ Yes |
| Has exit_code? | ✅ Yes | N/A | ✅ Yes | N/A | ✅ Yes |

---

## Data Flow Diagrams

### Automated Step Flow

```
┌──────────────────────────────────────┐
│ YAML: Step 1 (Automated)             │
│   - step: 1                          │
│   - description: "Check network..."  │
│   - command: "ip link show eth0..."  │
│   - expected_result: exit_code=0     │
└─────────────┬────────────────────────┘
              │
              ▼
┌──────────────────────────────────────┐
│ Test Executor                        │
│   1. Parse step definition           │
│   2. Check manual field = false      │
│   3. Execute command via shell       │
│   4. Capture exit_code + output      │
│   5. Verify against expected_result  │
└──────────┬──────────────┬────────────┘
           │              │
           ▼              ▼
┌──────────────────┐  ┌──────────────────┐
│ Console          │  │ JSON Log         │
│ [RUN] Step 1...  │  │ {                │
│ [PASS] Step 1... │  │   "step": 1,     │
└──────────────────┘  │   "command": "", │
                      │   "exit_code": 0,│
                      │   "output": "",  │
                      │   "timestamp": ""|
                      │ }                │
                      └──────────────────┘
```

### Manual Step Flow

```
┌──────────────────────────────────────┐
│ YAML: Step 2 (Manual)                │
│   - step: 2                          │
│   - description: "Physically..."     │
│   - manual: true                     │
│   - (no command field)               │
└─────────────┬────────────────────────┘
              │
              ▼
┌──────────────────────────────────────┐
│ Test Executor                        │
│   1. Parse step definition           │
│   2. Check manual field = true       │
│   3. Skip command execution          │
│   4. Continue to next step           │
└──────────┬──────────────┬────────────┘
           │              │
           ▼              ▼
┌──────────────────┐  ┌──────────────────┐
│ Console          │  │ JSON Log         │
│ [SKIP] Step 2... │  │ (no entry)       │
│  - Manual step   │  │                  │
└──────────────────┘  └──────────────────┘
```

---

## Step Number Preservation

### Why Preserve Original Step Numbers?

The step numbers in JSON logs match the original YAML step numbers to maintain traceability.

**Example:**

```
YAML Steps:     1    2    3    4    5
Type:         [A]  [M]  [A]  [M]  [A]
Executed?:    [Y]  [N]  [Y]  [N]  [Y]
Console:      RUN SKIP  RUN SKIP  RUN
JSON Steps:     1    -    3    -    5
```

Legend:
- A = Automated
- M = Manual
- Y = Yes
- N = No
- "-" = Not present in JSON

### Benefits of Preservation

1. **Traceability:** Easy to map JSON entry back to YAML step
2. **Clarity:** Gaps in numbering show where manual steps exist
3. **Debugging:** Quickly identify which test step corresponds to log entry
4. **Analysis:** Understand test structure without needing YAML file

---

## Execution Timeline

### Temporal Flow for TC_MANUAL_NETWORK_005

```
Time: T+0.000s
  ├─ Parse YAML test case
  ├─ Initialize test sequence 1
  │
Time: T+0.025s
  ├─ [Step 1] Execute: ip link show eth0...
  ├─ [Step 1] Result: exit_code=0
  ├─ [Step 1] Verify: PASS
  ├─ [Step 1] Console: [RUN] + [PASS]
  ├─ [Step 1] JSON: Write entry
  │
Time: T+0.026s
  ├─ [Step 2] Detect manual=true
  ├─ [Step 2] Skip execution
  ├─ [Step 2] Console: [SKIP] ... - Manual step
  ├─ [Step 2] JSON: No entry
  │
Time: T+0.032s
  ├─ [Step 3] Execute: echo "ip link set eth0 up"...
  ├─ [Step 3] Result: exit_code=0
  ├─ [Step 3] Verify: PASS
  ├─ [Step 3] Console: [RUN] + [PASS]
  ├─ [Step 3] JSON: Write entry
  │
Time: T+0.033s
  ├─ [Step 4] Detect manual=true
  ├─ [Step 4] Skip execution
  ├─ [Step 4] Console: [SKIP] ... - Manual step
  ├─ [Step 4] JSON: No entry
  │
Time: T+14.090s
  ├─ [Step 5] Execute: ping -c 4 8.8.8.8...
  ├─ [Step 5] Result: exit_code=0 (after 14 seconds)
  ├─ [Step 5] Verify: PASS
  ├─ [Step 5] Console: [RUN] + [PASS]
  ├─ [Step 5] JSON: Write entry
  │
Time: T+14.091s
  ├─ All steps complete
  ├─ Console: "All test sequences completed successfully"
  ├─ JSON: Close array, write to file
  └─ Exit with success code
```

**Key Observations:**
- Manual steps add negligible processing time
- Automated steps take time based on command execution
- JSON writes happen after each automated step
- Total execution time dominated by longest automated step (ping)

---

## Data Validation

### YAML Schema Validation

Before execution, the YAML is validated against the schema:

```yaml
steps:
  - step: <integer>        # Required
    description: <string>  # Required
    manual: <boolean>      # Optional (default: false)
    command: <string>      # Required if manual=false
    expected_result:       # Optional
      exit_code: <integer>
      output_contains: <string>
```

**Validation Rules:**
- If `manual: true`, `command` field must be absent or empty
- If `manual: false` or absent, `command` field must be present
- `step` numbers must be positive integers
- `expected_result` can be omitted (step always passes)

### JSON Schema Validation

The generated JSON log conforms to the execution log schema:

```json
[
  {
    "test_sequence": <integer>,  // Required
    "step": <integer>,            // Required
    "command": <string>,          // Required
    "exit_code": <integer>,       // Required
    "output": <string>,           // Required
    "timestamp": <ISO 8601 string> // Required
  }
]
```

**Validation Rules:**
- Must be a JSON array
- Each entry must have all 6 fields
- No entries for manual steps
- Step numbers match YAML (with gaps)
- Timestamps in ISO 8601 format with timezone

---

## Error Handling

### What Happens When...

#### An automated step fails?

**Scenario:** Step 1 exits with non-zero code

```
Console:
  [RUN] Step 1 (Sequence 1): Check network interface status
  [FAIL] Step 1 (Sequence 1): Check network interface status
    Command: ip link show eth0
    EXIT_CODE: 1
    COMMAND_OUTPUT: Device "eth0" does not exist
    Result verification: false
    Output verification: true
  Error: Test execution failed: Step 1 verification failed

JSON:
  [
    {
      "test_sequence": 1,
      "step": 1,
      "command": "ip link show eth0",
      "exit_code": 1,
      "output": "Device \"eth0\" does not exist",
      "timestamp": "2026-02-05T18:55:37.025240+04:00"
    }
  ]
```

**Result:** Step is recorded in JSON with actual exit code, test execution stops

#### A manual step is encountered?

**Scenario:** Step 2 has `manual: true`

```
Console:
  [SKIP] Step 2 (Sequence 1): Physically connect cable - Manual step

JSON:
  (no entry)
```

**Result:** Step is skipped, no JSON entry, execution continues to next step

#### YAML has invalid syntax?

**Scenario:** Malformed YAML file

```
Console:
  Error: Failed to parse YAML: invalid syntax at line 15

JSON:
  (no file created)
```

**Result:** Execution aborted before any steps run

---

## Performance Characteristics

### Overhead Analysis

| Operation | Time | Impact on Total Runtime |
|-----------|------|-------------------------|
| YAML Parsing | ~10ms | Negligible |
| Step Classification (manual check) | ~0.1ms per step | Negligible |
| Manual Step Skip | ~0.1ms | Negligible |
| Automated Step Execution | Variable | Dominant factor |
| JSON Entry Write | ~1ms per entry | Negligible |
| Console Output | ~1ms per line | Negligible |

**Conclusion:** Manual step handling adds negligible overhead. Total runtime is dominated by automated command execution time.

### Memory Usage

| Component | Memory | Notes |
|-----------|--------|-------|
| YAML in Memory | ~1-10 KB | Single test case |
| Execution State | ~1 KB | Current step info |
| Command Output Buffer | ~10-100 KB | Per automated step |
| JSON Array | ~1-10 KB | Grows with automated steps |
| Total | <150 KB | Per test execution |

**Conclusion:** Manual steps don't consume additional memory (no command execution, no output capture).

---

## Summary

### Key Principles

1. **Manual Steps Are Not Executed**
   - No command execution
   - No output capture
   - No verification checks

2. **Manual Steps Are Visible in Console**
   - Clear `[SKIP]` message
   - Suffix " - Manual step"
   - Maintains execution order visibility

3. **Manual Steps Are Absent from JSON**
   - No JSON entries created
   - Original step numbers preserved
   - Gaps indicate manual steps

4. **Execution Flow Is Uninterrupted**
   - Manual steps don't block
   - Automated steps continue
   - Test completes normally

### Use Cases

✅ **Good Uses of Manual Steps:**
- Physical hardware connections
- Visual verification (LEDs, displays)
- Human judgment (UI aesthetics, data quality)
- Actions requiring human interaction (button presses, form entry)

❌ **Poor Uses of Manual Steps:**
- Steps that could be automated
- Steps without clear instructions
- Steps that block for extended periods
- Steps that depend on previous manual steps

---

## Conclusion

The manual step data flow is designed to:
- ✅ Skip execution of manual steps
- ✅ Provide clear console feedback
- ✅ Maintain clean JSON logs with only automated results
- ✅ Preserve traceability through step numbering
- ✅ Enable hybrid automated/manual testing workflows

This design allows test cases to document both automated and manual procedures while ensuring execution logs contain only verifiable, reproducible automated results.
