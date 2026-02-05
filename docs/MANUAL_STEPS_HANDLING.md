# Manual Steps Handling

This document provides a comprehensive guide to how manual steps are handled across three contexts in the test-executor system: YAML definition, bash script generation, and test execution.

## Table of Contents

1. [Overview](#overview)
2. [YAML Definition](#yaml-definition)
3. [Bash Script Generation](#bash-script-generation)
4. [Test Execution](#test-execution)
5. [Example Test Cases](#example-test-cases)
6. [JSON Log Behavior](#json-log-behavior)

---

## Overview

Manual steps allow test cases to include operations that require human intervention, such as:
- Physical hardware connections
- GUI interactions
- Visual verification
- Manual authentication
- Device power operations

The system handles manual steps differently from automated steps at each stage of the test lifecycle.

---

## YAML Definition

### Schema

Manual steps are defined in YAML using the `manual: true` flag in a step definition:

```yaml
steps:
  - step: 1
    manual: true
    description: "Manually SSH into device and verify login"
    command: ssh admin@192.168.1.100
    expected:
      success: true
      result: 0
      output: "Successfully authenticated"
    verification:
      result: "true"
      output: "Verify that SSH login prompt appears and authentication succeeds"
```

### Key Characteristics

| Field | Behavior for Manual Steps |
|-------|---------------------------|
| `manual` | Set to `true` to mark step as manual (optional field) |
| `description` | Human-readable description of what to do |
| `command` | The command/action to perform (shown to user but not executed) |
| `expected` | Expected outcome (for documentation) |
| `verification` | Verification instructions for the user |

### Examples from Test Cases

#### Example 1: Physical Hardware Connection (TC_MANUAL_HARDWARE_002)

```yaml
- step: 1
  manual: true
  description: "Connect power cable to device"
  command: "Physical connection of power cable"
  expected:
    success: true
    result: 0
    output: "Power cable connected securely"
  verification:
    result: "true"
    output: "Verify power cable is firmly connected and LED indicators are off"
```

#### Example 2: GUI Interaction (TC_MANUAL_UI_003)

```yaml
- step: 2
  manual: true
  description: "Open browser and navigate to application homepage"
  command: "Navigate to http://localhost:8080"
  expected:
    success: true
    result: 0
    output: "Homepage loads successfully"
  verification:
    result: "true"
    output: "Verify homepage displays within 3 seconds with no console errors"
```

#### Example 3: Mixed Workflow (TC_MANUAL_MIXED_010)

```yaml
steps:
  - step: 1
    description: "Automated system health check"
    command: echo "System health check started" && uptime && echo "Health check completed"
    # ... automated step verification ...
  
  - step: 2
    manual: true
    description: "Manually start application service from GUI"
    command: "Click Start button in application control panel"
    # ... manual step verification ...
  
  - step: 3
    description: "Automated verification of service startup"
    command: sleep 5 && ps aux | grep -i 'service' | grep -v grep || echo "Service process verified"
    # ... automated step verification ...
```

---

## Bash Script Generation

When generating bash scripts, manual steps receive special handling to guide the user through the required actions.

### Generated Code Structure

For each manual step, the following code is generated:

```bash
# Step 2: Manually SSH into device and verify login
echo "Step 2: Manually SSH into device and verify login"
echo "Command: ssh admin@192.168.1.100"
echo "INFO: This is a manual step. You must perform this action manually."
if [[ "${DEBIAN_FRONTEND}" != 'noninteractive' && -t 0 ]]; then
    read -p "Press ENTER to continue..."
else
    echo "Non-interactive mode detected, skipping manual step confirmation."
fi
```

### Code Components

#### 1. Echo Statements

Three informational echo statements guide the user:

```bash
echo "Step 2: Manually SSH into device and verify login"
echo "Command: ssh admin@192.168.1.100"
echo "INFO: This is a manual step. You must perform this action manually."
```

**Purpose:**
- Display step number and description
- Show the command/action to perform
- Clearly indicate this is a manual step

#### 2. Interactive Prompt

The `read` command pauses execution and waits for user confirmation:

```bash
read -p "Press ENTER to continue..."
```

**Purpose:**
- Pause script execution
- Allow user time to complete the manual action
- Require explicit user confirmation before proceeding

#### 3. TTY Detection and Non-Interactive Mode Handling

The conditional block detects whether the script is running interactively:

```bash
if [[ "${DEBIAN_FRONTEND}" != 'noninteractive' && -t 0 ]]; then
    read -p "Press ENTER to continue..."
else
    echo "Non-interactive mode detected, skipping manual step confirmation."
fi
```

**Detection Logic:**

| Condition | Test | Purpose |
|-----------|------|---------|
| `DEBIAN_FRONTEND != 'noninteractive'` | Environment variable check | Detects Debian/Ubuntu non-interactive mode |
| `-t 0` | TTY test | Checks if stdin is connected to a terminal |

**Behavior:**

- **Interactive mode** (both conditions true): Displays prompt and waits for ENTER
- **Non-interactive mode** (either condition false): Skips prompt, shows skip message, continues automatically

**Non-Interactive Scenarios:**
- CI/CD pipelines
- Automated test runs
- Piped input: `echo "" | bash script.sh`
- Redirected input: `bash script.sh < /dev/null`
- Environment: `DEBIAN_FRONTEND=noninteractive bash script.sh`

### Complete Example from TC_MANUAL_HARDWARE_002.sh

```bash
#!/bin/bash
set -euo pipefail

# Test Case: TC_MANUAL_HARDWARE_002
# Description: Test case for physical hardware setup and verification

JSON_LOG="TC_MANUAL_HARDWARE_002_execution_log.json"
TIMESTAMP=$(date +"%Y-%m-%dT%H:%M:%S")

# ... (JSON logging setup) ...

# Step 1: Connect power cable to device
echo "Step 1: Connect power cable to device"
echo "Command: Physical connection of power cable"
echo "INFO: This is a manual step. You must perform this action manually."
if [[ "${DEBIAN_FRONTEND}" != 'noninteractive' && -t 0 ]]; then
    read -p "Press ENTER to continue..."
else
    echo "Non-interactive mode detected, skipping manual step confirmation."
fi

# Step 2: Connect Ethernet cable to port 1
echo "Step 2: Connect Ethernet cable to port 1"
echo "Command: Physical connection of Ethernet cable"
echo "INFO: This is a manual step. You must perform this action manually."
if [[ "${DEBIAN_FRONTEND}" != 'noninteractive' && -t 0 ]]; then
    read -p "Press ENTER to continue..."
else
    echo "Non-interactive mode detected, skipping manual step confirmation."
fi

# Step 3: Press power button to turn on device
echo "Step 3: Press power button to turn on device"
echo "Command: Press power button for 2 seconds"
echo "INFO: This is a manual step. You must perform this action manually."
if [[ "${DEBIAN_FRONTEND}" != 'noninteractive' && -t 0 ]]; then
    read -p "Press ENTER to continue..."
else
    echo "Non-interactive mode detected, skipping manual step confirmation."
fi

# Step 4: Wait for device boot and check USB devices (automated)
LOG_FILE="TC_MANUAL_HARDWARE_002_sequence-1_step-4.actual.log"
COMMAND_OUTPUT=""
set +e
COMMAND_OUTPUT=$({ sleep 30 && lsusb | grep -i device || echo "USB enumeration complete"; } 2>&1 | tee "$LOG_FILE")
EXIT_CODE=$?
set -e
# ... (automated step verification and logging) ...
```

### What Gets Generated vs Omitted

**Generated for Manual Steps:**
- Echo statements for step info
- User prompts with TTY detection
- Comment headers

**NOT Generated for Manual Steps:**
- `LOG_FILE` variable assignment
- Command execution (`COMMAND_OUTPUT=$(...)`)
- Exit code capture
- Verification expressions
- JSON log entry
- PASS/FAIL output
- `tee` command for log files

---

## Test Execution

When executing test cases, manual steps are handled specially by the test executor.

### Console Output

Manual steps produce `[SKIP]` messages on the console:

```
[SKIP] Step 2 (Sequence 1): Manually SSH into device and verify login - Manual step
[SKIP] Step 3 (Sequence 1): Execute uptime command on remote device - Manual step
[SKIP] Step 5 (Sequence 1): Manually log out from SSH session - Manual step
```

### Skip Message Format

```
[SKIP] Step {step_number} (Sequence {sequence_id}): {description} - Manual step
```

### Console Output Example from TC_MANUAL_SSH_001

```
[PASS] Step 1: Check device network connectivity
[SKIP] Step 2 (Sequence 1): Manually SSH into device and verify login - Manual step
[SKIP] Step 3 (Sequence 1): Execute uptime command on remote device - Manual step
[PASS] Step 4: Check SSH service status locally
[SKIP] Step 5 (Sequence 1): Manually log out from SSH session - Manual step
All test sequences completed successfully
```

### Console Output Example from TC_MANUAL_HARDWARE_002

```
[SKIP] Step 1 (Sequence 1): Connect power cable to device - Manual step
[SKIP] Step 2 (Sequence 1): Connect Ethernet cable to port 1 - Manual step
[SKIP] Step 3 (Sequence 1): Press power button to turn on device - Manual step
[PASS] Step 4: Wait for device boot and check USB devices
[PASS] Step 5: Verify network interface detection
All test sequences completed successfully
```

### Console Output Example from TC_MANUAL_MIXED_010

```
[PASS] Step 1: Automated system health check
[SKIP] Step 2 (Sequence 1): Manually start application service from GUI - Manual step
[PASS] Step 3: Automated verification of service startup
[SKIP] Step 4 (Sequence 1): Manually test application functionality through UI - Manual step
[PASS] Step 5: Automated log analysis
All test sequences completed successfully
```

### Rust Code Implementation

From `src/executor.rs`:

```rust
for sequence in &test_case.test_sequences {
    for step in &sequence.steps {
        if step.manual == Some(true) {
            println!(
                "[SKIP] Step {} (Sequence {}): {} - Manual step",
                step.step, sequence.id, step.description
            );
            continue;
        }
        // ... execute automated step ...
    }
}
```

---

## JSON Log Behavior

Manual steps are **completely excluded** from the JSON execution log. Only automated steps are recorded.

### Log File Naming

```
{TEST_CASE_ID}_execution_log.json
```

Examples:
- `TC_MANUAL_SSH_001_execution_log.json`
- `TC_MANUAL_HARDWARE_002_execution_log.json`
- `TC_MANUAL_MIXED_010_execution_log.json`

### JSON Schema for Automated Steps

```json
[
  {
    "test_sequence": 1,
    "step": 1,
    "command": "ping -c 3 192.168.1.100",
    "exit_code": 0,
    "output": "PING 192.168.1.100...\n3 packets transmitted, 3 received...",
    "timestamp": "2024-01-15T10:30:00"
  },
  {
    "test_sequence": 1,
    "step": 4,
    "command": "systemctl status ssh | grep -i active || echo \"SSH service check\"",
    "exit_code": 0,
    "output": "SSH service check",
    "timestamp": "2024-01-15T10:30:05"
  }
]
```

### Example JSON Log from TC_MANUAL_SSH_001

Notice that steps 2, 3, and 5 (manual steps) are not present in the log:

```json
[
  {
    "test_sequence": 1,
    "step": 1,
    "command": "ping -c 3 192.168.1.100",
    "exit_code": 0,
    "output": "PING 192.168.1.100 (192.168.1.100) 56(84) bytes of data.\n64 bytes from 192.168.1.100: icmp_seq=1 ttl=64 time=0.123 ms\n64 bytes from 192.168.1.100: icmp_seq=2 ttl=64 time=0.156 ms\n64 bytes from 192.168.1.100: icmp_seq=3 ttl=64 time=0.142 ms\n\n--- 192.168.1.100 ping statistics ---\n3 packets transmitted, 3 received, 0% packet loss, time 2047ms",
    "timestamp": "2024-01-15T10:30:00"
  },
  {
    "test_sequence": 1,
    "step": 4,
    "command": "systemctl status ssh | grep -i active || echo \"SSH service check\"",
    "exit_code": 0,
    "output": "SSH service check",
    "timestamp": "2024-01-15T10:30:15"
  }
]
```

### Example JSON Log from TC_MANUAL_HARDWARE_002

All manual steps (1, 2, 3) are excluded; only automated steps (4, 5) are logged:

```json
[
  {
    "test_sequence": 1,
    "step": 4,
    "command": "sleep 30 && lsusb | grep -i device || echo \"USB enumeration complete\"",
    "exit_code": 0,
    "output": "USB enumeration complete",
    "timestamp": "2024-01-15T10:31:00"
  },
  {
    "test_sequence": 1,
    "step": 5,
    "command": "ip link show | grep -E 'eth[0-9]|enp' || echo \"Network interface detected\"",
    "exit_code": 0,
    "output": "Network interface detected",
    "timestamp": "2024-01-15T10:31:05"
  }
]
```

### Example JSON Log from TC_MANUAL_MIXED_010

Manual steps (2, 4) are excluded; automated steps (1, 3, 5) are logged:

```json
[
  {
    "test_sequence": 1,
    "step": 1,
    "command": "echo \"System health check started\" && uptime && echo \"Health check completed\"",
    "exit_code": 0,
    "output": "System health check started\n10:30:00 up 5 days, 12:34, 2 users, load average: 0.15, 0.10, 0.08\nHealth check completed",
    "timestamp": "2024-01-15T10:30:00"
  },
  {
    "test_sequence": 1,
    "step": 3,
    "command": "sleep 5 && ps aux | grep -i 'service' | grep -v grep || echo \"Service process verified\"",
    "exit_code": 0,
    "output": "Service process verified",
    "timestamp": "2024-01-15T10:30:10"
  },
  {
    "test_sequence": 1,
    "step": 5,
    "command": "echo \"Log analysis completed\" && grep -c \"ERROR\" /dev/null 2>/dev/null || echo \"No critical errors found\"",
    "exit_code": 0,
    "output": "Log analysis completed\nNo critical errors found",
    "timestamp": "2024-01-15T10:30:20"
  }
]
```

### Log Validation Code

From `tests/executor_json_log_test.rs`:

```rust
fn validate_log_matches_testcase(entries: &[TestStepExecutionEntry], test_case: &TestCase) {
    let mut expected_entries = Vec::new();
    for sequence in &test_case.test_sequences {
        for step in &sequence.steps {
            // Manual steps are excluded from the log
            if step.manual != Some(true) {
                expected_entries.push((sequence.id, step.step, step.command.clone()));
            }
        }
    }

    assert_eq!(
        entries.len(),
        expected_entries.len(),
        "Number of log entries must match non-manual steps in test case"
    );
    
    // Verify each entry matches expected automated steps
    for (i, entry) in entries.iter().enumerate() {
        let (expected_seq, expected_step, expected_cmd) = &expected_entries[i];
        assert_eq!(entry.test_sequence, *expected_seq, "Entry {} test_sequence mismatch", i);
        assert_eq!(entry.step, *expected_step, "Entry {} step mismatch", i);
        assert_eq!(entry.command, *expected_cmd, "Entry {} command mismatch", i);
    }
}
```

---

## Example Test Cases

The following 10 test cases demonstrate various manual step scenarios:

### 1. TC_MANUAL_SSH_001 - SSH Device Access
- **Manual Steps:** 3 (SSH login, uptime command, logout)
- **Automated Steps:** 2 (ping test, SSH service check)
- **Use Case:** Remote device authentication

### 2. TC_MANUAL_HARDWARE_002 - Physical Hardware Setup
- **Manual Steps:** 3 (power cable, Ethernet cable, power button)
- **Automated Steps:** 2 (USB enumeration, network interface detection)
- **Use Case:** Hardware connection and power-up

### 3. TC_MANUAL_UI_003 - User Interface Verification
- **Manual Steps:** 3 (browser navigation, menu inspection, settings modal)
- **Automated Steps:** 2 (server health check, console log analysis)
- **Use Case:** GUI visual inspection

### 4. TC_MANUAL_DEVICE_004 - Device Power Operations
- **Manual Steps:** 3 (sleep mode, wake from sleep, force shutdown)
- **Automated Steps:** 2 (power state logging, uptime verification)
- **Use Case:** Power state transitions

### 5. TC_MANUAL_NETWORK_005 - Network Configuration
- **Manual Steps:** Configuration verification steps
- **Automated Steps:** Network connectivity tests
- **Use Case:** Network setup and validation

### 6. TC_MANUAL_DATABASE_006 - Database Verification
- **Manual Steps:** SQL query execution and validation
- **Automated Steps:** Database connection checks
- **Use Case:** Manual SQL verification

### 7. TC_MANUAL_API_007 - API Authentication
- **Manual Steps:** Token generation and validation
- **Automated Steps:** API endpoint tests
- **Use Case:** Manual authentication workflow

### 8. TC_MANUAL_SECURITY_008 - Security Certificate
- **Manual Steps:** Certificate inspection
- **Automated Steps:** Certificate file verification
- **Use Case:** Security validation

### 9. TC_MANUAL_BACKUP_009 - Backup Restoration
- **Manual Steps:** Restoration verification
- **Automated Steps:** Backup file checks
- **Use Case:** Backup process validation

### 10. TC_MANUAL_MIXED_010 - End-to-End Workflow
- **Manual Steps:** 2 (GUI service start, UI functionality test)
- **Automated Steps:** 3 (health check, service verification, log analysis)
- **Use Case:** Complete system validation with mixed operations

---

## Summary

### Key Principles

1. **YAML Definition**: Use `manual: true` flag to mark steps requiring human intervention
2. **Script Generation**: Manual steps generate user prompts with TTY detection, not command execution
3. **Test Execution**: Manual steps are skipped with `[SKIP]` console messages
4. **JSON Logging**: Manual steps are completely excluded from execution logs

### Benefits

- **Clear Separation**: Automated and manual steps are distinctly handled
- **CI/CD Compatible**: Non-interactive mode allows automated runs to skip manual steps
- **Traceability**: JSON logs contain only verifiable automated step results
- **User Guidance**: Generated scripts provide clear instructions for manual actions

### Implementation Locations

- **YAML Schema**: `src/models.rs` - `Step` struct with `manual: Option<bool>`
- **Script Generation**: `src/creator.rs` - `generate_test_script()` function
- **Execution**: `src/executor.rs` - `execute_test_case()` function
- **JSON Logging**: Manual step filtering in both script generation and executor
- **Tests**: `tests/executor_json_log_test.rs` - Validation tests for log exclusion
