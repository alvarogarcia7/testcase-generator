# Manual Steps Scripts Examples

This directory contains bash scripts generated from YAML test case definitions that include manual steps. These examples demonstrate the test executor's capability to handle test scenarios that require human interaction alongside automated verification.

## Index of Test Cases

| ID | Test Case | Description |
|----|-----------|-------------|
| 1 | TC_MANUAL_SSH_001 | SSH device access verification with manual authentication steps |
| 2 | TC_MANUAL_HARDWARE_002 | Physical hardware setup, cable connections, and power-on sequence |
| 3 | TC_MANUAL_UI_003 | User interface verification with manual visual inspection of web elements |
| 4 | TC_MANUAL_DEVICE_004 | Device power state transitions including sleep, wake, and shutdown |
| 5 | TC_MANUAL_NETWORK_005 | Network configuration with physical cable connection and connectivity tests |
| 6 | TC_MANUAL_DATABASE_006 | Database verification with manual SQL query validation and data inspection |
| 7 | TC_MANUAL_API_007 | API token validation with manual browser-based authentication flow |
| 8 | TC_MANUAL_SECURITY_008 | Security certificate verification with manual browser inspection |
| 9 | TC_MANUAL_BACKUP_009 | Backup restoration process with manual file integrity verification |
| 10 | TC_MANUAL_MIXED_010 | Complete end-to-end system validation mixing automated checks and manual UI operations |

## Test Case Details

### TC_MANUAL_SSH_001: SSH Device Access
**Scenario:** Verifies SSH connectivity to a remote device with mixed automated and manual steps.
- **Automated Steps:** Network ping tests, local SSH service status checks
- **Manual Steps:** SSH login, execute uptime command remotely, logout
- **Generated Script:** `TC_MANUAL_SSH_001.sh`
- **JSON Log:** `TC_MANUAL_SSH_001_execution_log.json`
- **Console Output Logs:** `TC_MANUAL_SSH_001_sequence-1_step-{1,4}.actual.log`

### TC_MANUAL_HARDWARE_002: Physical Hardware Setup
**Scenario:** Covers physical hardware connection, power-up sequence, and system detection.
- **Automated Steps:** USB device enumeration, network interface detection
- **Manual Steps:** Connect power cable, connect Ethernet cable, press power button
- **Generated Script:** `TC_MANUAL_HARDWARE_002.sh`
- **JSON Log:** `TC_MANUAL_HARDWARE_002_execution_log.json`
- **Console Output Logs:** `TC_MANUAL_HARDWARE_002_sequence-1_step-{4,5}.actual.log`

### TC_MANUAL_UI_003: User Interface Verification
**Scenario:** Verifies UI elements through manual inspection and automated health checks.
- **Automated Steps:** Application server health check, console log verification
- **Manual Steps:** Navigate to homepage, inspect navigation menu, test Settings modal
- **Generated Script:** `TC_MANUAL_UI_003.sh`
- **JSON Log:** `TC_MANUAL_UI_003_execution_log.json`
- **Console Output Logs:** `TC_MANUAL_UI_003_sequence-1_step-{1,5}.actual.log`

### TC_MANUAL_DEVICE_004: Power State Transitions
**Scenario:** Tests device power state transitions including sleep, wake, and shutdown.
- **Automated Steps:** Record power state timestamps, verify system uptime
- **Manual Steps:** Initiate sleep mode, wake device, force shutdown
- **Generated Script:** `TC_MANUAL_DEVICE_004.sh`
- **JSON Log:** `TC_MANUAL_DEVICE_004_execution_log.json`
- **Console Output Logs:** `TC_MANUAL_DEVICE_004_sequence-1_step-{1,4}.actual.log`

### TC_MANUAL_NETWORK_005: Network Configuration
**Scenario:** Covers network interface configuration with physical cable connection.
- **Automated Steps:** Interface status checks, bring interface up, ping connectivity test
- **Manual Steps:** Connect Ethernet cable to switch, verify link LEDs
- **Generated Script:** `TC_MANUAL_NETWORK_005.sh`
- **JSON Log:** `TC_MANUAL_NETWORK_005_execution_log.json`
- **Console Output Logs:** `TC_MANUAL_NETWORK_005_sequence-1_step-{1,3,5}.actual.log`

### TC_MANUAL_DATABASE_006: Database Query Validation
**Scenario:** Verifies database connectivity and validates complex data relationships.
- **Automated Steps:** Database port check, connection test, transaction log check
- **Manual Steps:** Execute row count query, verify data integrity across joined tables
- **Generated Script:** `TC_MANUAL_DATABASE_006.sh`
- **JSON Log:** `TC_MANUAL_DATABASE_006_execution_log.json`
- **Console Output Logs:** `TC_MANUAL_DATABASE_006_sequence-1_step-{1,2,5}.actual.log`

### TC_MANUAL_API_007: API Authentication Flow
**Scenario:** Tests API authentication flow with manual browser-based token validation.
- **Automated Steps:** API health endpoint check, token expiration validation
- **Manual Steps:** Navigate to login page, submit credentials, inspect JWT token in DevTools
- **Generated Script:** `TC_MANUAL_API_007.sh`
- **JSON Log:** `TC_MANUAL_API_007_execution_log.json`
- **Console Output Logs:** `TC_MANUAL_API_007_sequence-1_step-{1,5}.actual.log`

### TC_MANUAL_SECURITY_008: SSL Certificate Validation
**Scenario:** Verifies SSL certificate installation and trust chain validation.
- **Automated Steps:** HTTPS port check, OpenSSL availability check
- **Manual Steps:** Inspect certificate in browser, verify certificate chain, check expiration date
- **Generated Script:** `TC_MANUAL_SECURITY_008.sh`
- **JSON Log:** `TC_MANUAL_SECURITY_008_execution_log.json`
- **Console Output Logs:** `TC_MANUAL_SECURITY_008_sequence-1_step-{1,2}.actual.log`

### TC_MANUAL_BACKUP_009: Backup Restoration
**Scenario:** Covers backup file validation, restoration process, and data integrity verification.
- **Automated Steps:** List backup files, verify checksums
- **Manual Steps:** Extract backup archive, inspect configuration files, compare with manifest
- **Generated Script:** `TC_MANUAL_BACKUP_009.sh`
- **JSON Log:** `TC_MANUAL_BACKUP_009_execution_log.json`
- **Console Output Logs:** `TC_MANUAL_BACKUP_009_sequence-1_step-{1,2}.actual.log`

### TC_MANUAL_MIXED_010: End-to-End System Validation
**Scenario:** Complete workflow demonstrating mixed automated and manual operations.
- **Automated Steps:** System health check, service startup verification, log analysis
- **Manual Steps:** Start application from GUI, test workflow through UI
- **Generated Script:** `TC_MANUAL_MIXED_010.sh`
- **JSON Log:** `TC_MANUAL_MIXED_010_execution_log.json`
- **Console Output Logs:** `TC_MANUAL_MIXED_010_sequence-1_step-{1,3,5}.actual.log`

## Generating Scripts

To generate a bash script from a YAML test case:

```bash
# Generate a specific test case
./target/release/test-executor generate \
  testcases/examples/manual_steps/TC_MANUAL_SSH_001.yaml \
  -o examples/manual_steps_scripts/TC_MANUAL_SSH_001.sh

# Generate all test cases
for yaml_file in testcases/examples/manual_steps/*.yaml; do
  test_case=$(basename "$yaml_file" .yaml)
  ./target/release/test-executor generate \
    "$yaml_file" \
    -o "examples/manual_steps_scripts/${test_case}.sh"
done
```

## Executing Scripts

### Interactive Mode (with manual step prompts)

```bash
# Execute a single test case
bash examples/manual_steps_scripts/TC_MANUAL_SSH_001.sh

# The script will:
# 1. Execute automated steps with verification
# 2. Display manual step instructions and wait for user confirmation
# 3. Generate JSON execution log and console output files
```

### Non-Interactive Mode (CI/CD environments)

```bash
# Set DEBIAN_FRONTEND to skip manual step prompts
DEBIAN_FRONTEND=noninteractive bash examples/manual_steps_scripts/TC_MANUAL_SSH_001.sh

# Or via pipe
echo "" | bash examples/manual_steps_scripts/TC_MANUAL_SSH_001.sh

# Manual steps will be displayed but execution continues without waiting
```

## Generated Files

Each script execution produces the following output files:

### JSON Execution Log
- **Filename Pattern:** `TC_MANUAL_*_execution_log.json`
- **Content:** Structured JSON array containing execution results for automated steps
- **Fields:** test_sequence, step, command, exit_code, output, timestamp
- **Example:**
```json
[
  {
    "test_sequence": 1,
    "step": 1,
    "command": "ping -c 3 192.168.1.100",
    "exit_code": 0,
    "output": "PING 192.168.1.100 ...\n3 packets transmitted, 3 received",
    "timestamp": "2024-01-15T10:30:00"
  }
]
```

### Console Output Logs
- **Filename Pattern:** `TC_MANUAL_*_sequence-{N}_step-{M}.actual.log`
- **Content:** Raw command output and stderr for automated steps
- **Usage:** Used by verification expressions to validate step execution
- **Note:** Only created for automated steps with commands

### Standard Output
Scripts write execution progress to stdout:
- `[PASS]` or `[FAIL]` indicators for automated steps
- Manual step instructions with command details
- Non-interactive mode notifications
- Final success/failure message

## Manual Step Features

Each generated script handles manual steps with:

### 1. Echo Statements
Display clear instructions for the user:
```bash
echo "Step 2: Manually SSH into device and verify login"
echo "Command: ssh admin@192.168.1.100"
echo "INFO: This is a manual step. You must perform this action manually."
```

### 2. User Prompts (Interactive Mode)
Wait for user confirmation before proceeding:
```bash
read -p "Press ENTER to continue..."
```

### 3. Automatic Mode Detection
Scripts detect non-interactive environments using two methods:
- `DEBIAN_FRONTEND` environment variable
- TTY status test (`-t 0`)

```bash
if [[ "${DEBIAN_FRONTEND}" != 'noninteractive' && -t 0 ]]; then
    read -p "Press ENTER to continue..."
else
    echo "Non-interactive mode detected, skipping manual step confirmation."
fi
```

## Automated Step Features

Automated steps in the generated scripts include:

### Command Execution
- Captures both stdout and stderr
- Logs output to both file and variable
- Preserves exit codes for verification

### Verification
Each automated step includes:
- **Result Verification:** Checks exit code against expected value
- **Output Verification:** Validates command output using grep, regex, or other checks
- **File-based Verification:** Can verify against logged output files

### Error Handling
- `set -euo pipefail` for strict error handling
- Temporary `set +e` around command execution to capture exit codes
- Exit with code 1 on any verification failure
- Detailed failure output showing command, exit code, and verification results

### JSON Logging
- Valid JSON array structure
- Automatic cleanup on script exit
- Optional JSON validation with `jq` if available
- BSD/GNU compatible JSON escaping using multiple fallback methods

## Shell Script Compatibility

All generated scripts are compatible with:
- **Bash versions:** 3.2+ (macOS default bash)
- **Operating systems:** macOS (BSD) and Linux (GNU)
- **Command-line tools:** Portable syntax for sed, grep, awk, etc.

Key compatibility features:
- No GNU-specific flags (e.g., uses `sed -E` instead of `sed -r`)
- No bash 4.0+ features (e.g., no associative arrays)
- Portable regex patterns
- Multiple fallback methods for JSON escaping (python3, python, perl, sed/awk)

## Source Test Cases

All YAML source files are located in:
```
testcases/examples/manual_steps/
├── TC_MANUAL_SSH_001.yaml
├── TC_MANUAL_HARDWARE_002.yaml
├── TC_MANUAL_UI_003.yaml
├── TC_MANUAL_DEVICE_004.yaml
├── TC_MANUAL_NETWORK_005.yaml
├── TC_MANUAL_DATABASE_006.yaml
├── TC_MANUAL_API_007.yaml
├── TC_MANUAL_SECURITY_008.yaml
├── TC_MANUAL_BACKUP_009.yaml
└── TC_MANUAL_MIXED_010.yaml
```

Each YAML file contains:
- Test case metadata (ID, description, requirements)
- General and specific initial conditions
- Test sequences with steps
- Manual step indicators (`manual: true`)
- Expected results and verification expressions

## References

- **Test Executor Documentation:** See main repository README
- **YAML Schema:** See test case schema documentation
- **Script Generator:** `src/script_generator.rs` in the test-executor codebase
- **Manual Steps Implementation:** Step handling logic in script generator
