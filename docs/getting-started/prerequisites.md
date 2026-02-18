# Prerequisites

## Overview

Prerequisites are conditions that must be satisfied **before** a test case begins execution. They provide a way to ensure the test environment is properly configured and ready for testing. Prerequisites are checked at the very beginning of test execution, before any initial conditions or test sequences run.

**Key Differences from Initial Conditions:**
- **Prerequisites**: Verified before test execution begins. Test will not proceed if prerequisites fail.
- **Initial Conditions**: Set up the test environment after prerequisites pass. Part of the test execution flow.

## Prerequisite Types

The Test Case Manager supports two types of prerequisites:

### 1. Manual Prerequisites

Manual prerequisites require human verification and are used for conditions that cannot be automated.

**Characteristics:**
- Requires human confirmation
- No automated verification
- Interactive mode: prompts user to press ENTER after confirming the prerequisite
- Non-interactive mode: automatically considered satisfied

**Use Cases:**
- Physical hardware verification (e.g., "Device is powered on and connected")
- Administrative access confirmation
- Environmental conditions that require human observation
- Setup steps that involve manual procedures

**Example:**
```yaml
prerequisites:
  - type: manual
    description: "Ensure you have administrative access to the test devices"
  - type: manual
    description: "Verify that all test devices are powered on and physically connected"
  - type: manual
    description: "Confirm test environment variables are properly configured in your shell"
```

### 2. Automatic Prerequisites

Automatic prerequisites are verified programmatically by executing a verification command.

**Characteristics:**
- Verified by executing a command
- Test fails immediately if verification command returns non-zero exit code
- Runs in both interactive and non-interactive modes
- Provides detailed error output if verification fails

**Use Cases:**
- Network connectivity checks
- Service availability verification
- File/directory existence checks
- Resource availability validation (disk space, memory, etc.)
- Software dependency verification

**Example:**
```yaml
prerequisites:
  - type: automatic
    description: "Check network connectivity to primary device"
    verification_command: "ping -c 3 192.168.1.100 && exit 0 || exit 1"
  - type: automatic
    description: "Verify SSH service is running and port 22 is accessible"
    verification_command: "nc -z -w 5 192.168.1.100 22 && exit 0 || exit 1"
  - type: automatic
    description: "Check that required test data file exists"
    verification_command: "test -f /tmp/test_data.json && exit 0 || exit 1"
```

## YAML Schema

### Schema Definition

Prerequisites are defined in the test case YAML file at the top level, immediately after the description field.

**Field Specification:**

```yaml
prerequisites:
  type: array
  items:
    type: object
    properties:
      type:
        type: string
        enum: ["manual", "automatic"]
        description: Type of prerequisite
      description:
        type: string
        description: Human-readable description of the prerequisite
      verification_command:
        type: string
        description: Command to verify the prerequisite (required for automatic type)
    required: ["type", "description"]
```

### Complete Example

```yaml
requirement: "REQ_TEST_001"
item: 1
tc: 1
id: 'TC_TEST_001'
description: 'Example test case with prerequisites'

prerequisites:
  - type: manual
    description: "Ensure you have administrative access to the test devices"
  - type: automatic
    description: "Check network connectivity to primary device"
    verification_command: "ping -c 3 192.168.1.100"
  - type: automatic
    description: "Verify web service is available on port 8080"
    verification_command: "curl -s -o /dev/null -w '%{http_code}' http://localhost:8080/health | grep -q '200'"

general_initial_conditions:
  system:
    - "Test execution environment is set up"

initial_conditions:
  device:
    - "Device is accessible"

test_sequences:
  - id: 1
    name: "Test Sequence"
    description: "Main test sequence"
    initial_conditions:
      device:
        - "Device is ready"
    steps:
      - step: 1
        description: "Execute test"
        command: echo "test"
        expected:
          result: "0"
          output: "test"
```

### Validation Rules

1. **Required Fields:**
   - `type`: Must be either "manual" or "automatic"
   - `description`: Must be a non-empty string

2. **Conditional Requirements:**
   - If `type` is "automatic", `verification_command` is **required**
   - If `type` is "manual", `verification_command` is **optional** (typically omitted)

3. **Field Types:**
   - `type`: String enum
   - `description`: String
   - `verification_command`: String (shell command)

## Bash Script Behavior

### Generated Script Structure

When a test case includes prerequisites, the generated bash script checks them before executing any test logic:

```bash
#!/bin/bash
set -euo pipefail

# Test Case: TC_TEST_001
# Description: Example test case with prerequisites

# Prerequisites
echo "Checking prerequisites..."

# Prerequisite 1: [description]
[prerequisite verification logic]

# Prerequisite 2: [description]
[prerequisite verification logic]

echo "All prerequisites satisfied"
echo ""

# [Rest of test execution...]
```

### Interactive vs Non-Interactive Mode

The generated script automatically detects whether it's running in interactive or non-interactive mode and adjusts behavior accordingly.

#### Detection Logic

The script checks two conditions:
1. **TTY availability**: Tests if stdin is connected to a terminal using `-t 0`
2. **DEBIAN_FRONTEND variable**: Checks if set to "noninteractive"

**Interactive Mode Conditions:**
- TTY is available (`-t 0` returns true)
- AND `DEBIAN_FRONTEND` is not set to "noninteractive"

**Non-Interactive Mode Conditions:**
- No TTY available (running in CI/CD, cron job, etc.)
- OR `DEBIAN_FRONTEND` is set to "noninteractive"

#### Manual Prerequisite Behavior

**Interactive Mode:**
```bash
# Prerequisite 1: Ensure you have administrative access to the test devices
echo "[MANUAL PREREQUISITE 1] Ensure you have administrative access to the test devices"
if [[ "${DEBIAN_FRONTEND}" != 'noninteractive' && -t 0 ]]; then
    read -p "Press ENTER to confirm this prerequisite is satisfied..."
else
    echo "Non-interactive mode: assuming prerequisite is satisfied."
fi
```

- Displays prerequisite description
- Prompts user to press ENTER
- Waits for user confirmation
- Proceeds after confirmation

**Non-Interactive Mode:**
```bash
echo "[MANUAL PREREQUISITE 1] Ensure you have administrative access to the test devices"
echo "Non-interactive mode: assuming prerequisite is satisfied."
```

- Displays prerequisite description
- Automatically assumes prerequisite is satisfied
- No user interaction required
- Continues execution immediately

#### Automatic Prerequisite Behavior

Automatic prerequisites behave **identically** in both interactive and non-interactive modes:

```bash
# Prerequisite 2: Check network connectivity to primary device
echo "[AUTOMATIC PREREQUISITE 2] Verifying: Check network connectivity to primary device"
set +e
PREREQ_OUTPUT=$({ ping -c 3 192.168.1.100; } 2>&1)
PREREQ_EXIT_CODE=$?
set -e
if [ $PREREQ_EXIT_CODE -ne 0 ]; then
    echo "ERROR: Prerequisite 2 failed: Check network connectivity to primary device"
    echo "Verification command: ping -c 3 192.168.1.100"
    echo "Exit code: $PREREQ_EXIT_CODE"
    echo "Output: $PREREQ_OUTPUT"
    exit 1
fi
echo "[PASS] Prerequisite 2 verified"
```

**Behavior:**
- Executes verification command
- Captures both stdout and stderr
- Checks exit code
- If exit code is 0: prints success message and continues
- If exit code is non-zero: prints detailed error information and exits immediately with exit code 1

**Error Output Format:**
```
ERROR: Prerequisite 2 failed: Check network connectivity to primary device
Verification command: ping -c 3 192.168.1.100
Exit code: 1
Output: ping: cannot resolve 192.168.1.100: Unknown host
```

### Prerequisite Execution Order

Prerequisites are executed in the order they appear in the YAML file:

1. Prerequisite 1
2. Prerequisite 2
3. Prerequisite 3
4. ...
5. Prerequisite N

**Important:** If any automatic prerequisite fails, execution stops immediately and subsequent prerequisites are not checked.

## Verification Command Patterns

### Network Connectivity

**Ping test:**
```yaml
- type: automatic
  description: "Check network connectivity to device"
  verification_command: "ping -c 3 192.168.1.100 && exit 0 || exit 1"
```

**Port connectivity:**
```yaml
- type: automatic
  description: "Verify SSH port is accessible"
  verification_command: "nc -z -w 5 192.168.1.100 22 && exit 0 || exit 1"
```

**Multiple ports:**
```yaml
- type: automatic
  description: "Verify web and SSH ports are accessible"
  verification_command: "nc -z 192.168.1.100 22 && nc -z 192.168.1.100 80 && exit 0 || exit 1"
```

### Service Status

**HTTP service availability:**
```yaml
- type: automatic
  description: "Confirm web service is available"
  verification_command: "curl -s -o /dev/null -w '%{http_code}' http://localhost:8080/health | grep -q '200' && exit 0 || exit 1"
```

**systemd service:**
```yaml
- type: automatic
  description: "Verify database service is running"
  verification_command: "systemctl is-active postgresql || pgrep -x postgres >/dev/null && exit 0 || exit 1"
```

**Process check:**
```yaml
- type: automatic
  description: "Verify nginx is running"
  verification_command: "pgrep -x nginx >/dev/null && exit 0 || exit 1"
```

### File System

**File existence:**
```yaml
- type: automatic
  description: "Check that required test data file exists"
  verification_command: "test -f /tmp/test_data.json && exit 0 || exit 1"
```

**Directory existence:**
```yaml
- type: automatic
  description: "Verify output directory exists"
  verification_command: "test -d /var/log/test && exit 0 || exit 1"
```

**File permissions:**
```yaml
- type: automatic
  description: "Verify config file is readable"
  verification_command: "test -r /etc/myapp/config.conf && exit 0 || exit 1"
```

**Disk space:**
```yaml
- type: automatic
  description: "Ensure sufficient disk space (at least 1GB free)"
  verification_command: "df -k /tmp | awk 'NR==2 {if ($4 > 1048576) exit 0; else exit 1}'"
```

### Environment Variables

**Variable is set:**
```yaml
- type: automatic
  description: "Verify API_KEY environment variable is set"
  verification_command: "test -n \"$API_KEY\" && exit 0 || exit 1"
```

**Variable has specific value:**
```yaml
- type: automatic
  description: "Verify TEST_ENV is set to staging"
  verification_command: "test \"$TEST_ENV\" = \"staging\" && exit 0 || exit 1"
```

### Software Dependencies

**Command availability:**
```yaml
- type: automatic
  description: "Check that Docker is installed"
  verification_command: "command -v docker >/dev/null 2>&1 && exit 0 || exit 1"
```

**Docker daemon:**
```yaml
- type: automatic
  description: "Check that Docker daemon is running"
  verification_command: "docker info >/dev/null 2>&1 && exit 0 || exit 1"
```

**Python package:**
```yaml
- type: automatic
  description: "Verify pytest is installed"
  verification_command: "python3 -c 'import pytest' 2>/dev/null && exit 0 || exit 1"
```

### Database Connectivity

**PostgreSQL:**
```yaml
- type: automatic
  description: "Verify PostgreSQL database is accessible"
  verification_command: "psql -h localhost -U testuser -d testdb -c 'SELECT 1' >/dev/null 2>&1 && exit 0 || exit 1"
```

**MySQL:**
```yaml
- type: automatic
  description: "Verify MySQL database is accessible"
  verification_command: "mysql -h localhost -u testuser -ppassword -e 'SELECT 1' >/dev/null 2>&1 && exit 0 || exit 1"
```

**Redis:**
```yaml
- type: automatic
  description: "Verify Redis is running and accessible"
  verification_command: "redis-cli ping | grep -q PONG && exit 0 || exit 1"
```

### Complex Verification

**Multiple conditions with logical AND:**
```yaml
- type: automatic
  description: "Verify web server and database are both running"
  verification_command: "systemctl is-active nginx && systemctl is-active postgresql && exit 0 || exit 1"
```

**Multiple conditions with logical OR:**
```yaml
- type: automatic
  description: "Verify either systemd or process is managing the service"
  verification_command: "systemctl is-active myservice || pgrep -x myservice >/dev/null && exit 0 || exit 1"
```

**Conditional logic:**
```bash
- type: automatic
  description: "Verify appropriate log level is set"
  verification_command: |
    if [ "$ENV" = "production" ]; then
      test "$LOG_LEVEL" = "error" || test "$LOG_LEVEL" = "warn"
    else
      test "$LOG_LEVEL" = "debug" || test "$LOG_LEVEL" = "info"
    fi && exit 0 || exit 1
```

### Best Practices for Verification Commands

1. **Always use explicit exit codes:**
   ```bash
   # Good: Explicit exit codes
   test -f /tmp/file && exit 0 || exit 1
   
   # Less reliable: Implicit exit code
   test -f /tmp/file
   ```

2. **Suppress unnecessary output:**
   ```bash
   # Good: Quiet operation
   curl -s -o /dev/null http://example.com
   
   # Noisy: Shows output
   curl http://example.com
   ```

3. **Redirect error messages when appropriate:**
   ```bash
   # Good: Suppress error output when expected
   command -v docker >/dev/null 2>&1
   
   # Noisy: Shows error if command not found
   command -v docker
   ```

4. **Use timeout for potentially hanging commands:**
   ```bash
   # Good: Timeout after 10 seconds
   timeout 10 ping -c 1 192.168.1.100 && exit 0 || exit 1
   
   # Risk: Could hang indefinitely
   ping -c 1 192.168.1.100
   ```

5. **Make commands portable (BSD/GNU compatible):**
   ```bash
   # Good: Works on both BSD and GNU
   df -k /tmp | awk 'NR==2 {print $4}'
   
   # Less portable: GNU-specific flag
   df --output=avail /tmp
   ```

## Migration Guide: Initial Conditions to Prerequisites

If you have existing test cases using initial conditions for prerequisites-like checks, here's how to migrate them:

### Before: Using Initial Conditions

```yaml
requirement: "REQ_001"
item: 1
tc: 1
id: 'TC_001'
description: 'Test case with setup checks in initial conditions'

general_initial_conditions:
  system:
    - "Verify Docker is running"
    - "Check network connectivity to test device"
    - "Ensure test data directory exists"
    - "Operator has access to admin console"

initial_conditions:
  device:
    - "Device is powered on and connected"
    - "Device has default configuration"

test_sequences:
  - id: 1
    name: "Main Test"
    description: "Test sequence"
    initial_conditions:
      device:
        - "Device is ready for testing"
    steps: []
```

**Problems with this approach:**
- Initial conditions are just comments by default (not verified)
- Can use BDD patterns, but these execute during test, not before
- No distinction between prerequisites and setup steps
- Failures don't prevent test execution from starting
- Manual prerequisites mixed with automated checks

### After: Using Prerequisites

```yaml
requirement: "REQ_001"
item: 1
tc: 1
id: 'TC_001'
description: 'Test case with proper prerequisites'

prerequisites:
  # Manual prerequisites
  - type: manual
    description: "Ensure device is powered on and connected"
  - type: manual
    description: "Verify operator has access to admin console"
  
  # Automatic prerequisites
  - type: automatic
    description: "Check that Docker daemon is running"
    verification_command: "docker info >/dev/null 2>&1 && exit 0 || exit 1"
  - type: automatic
    description: "Verify network connectivity to test device"
    verification_command: "ping -c 3 192.168.1.100 && exit 0 || exit 1"
  - type: automatic
    description: "Check that test data directory exists"
    verification_command: "test -d /tmp/test_data && exit 0 || exit 1"

general_initial_conditions:
  system:
    - "Test execution environment is configured"

initial_conditions:
  device:
    - "create directory \"/tmp/test_output\""
    - "set environment variable \"TEST_MODE\" to \"enabled\""

test_sequences:
  - id: 1
    name: "Main Test"
    description: "Test sequence"
    initial_conditions:
      device:
        - "wait for 2 seconds"
        - "Device configuration is loaded"
    steps: []
```

**Benefits of this approach:**
- Clear separation between prerequisites and setup
- Prerequisites verified before test execution begins
- Automatic prerequisites fail fast if conditions not met
- Manual prerequisites handled appropriately in interactive/non-interactive modes
- Initial conditions can now focus on test-specific setup (using BDD patterns or comments)

### Migration Steps

1. **Identify prerequisite candidates:**
   - Conditions that must be true BEFORE test starts
   - Environment readiness checks
   - Hardware/network availability
   - Service status verification
   - Manual verification steps

2. **Categorize as manual or automatic:**
   - Manual: Physical checks, human verification, access confirmation
   - Automatic: Commands that can verify the condition programmatically

3. **Create verification commands for automatic prerequisites:**
   - Use patterns from the "Verification Command Patterns" section above
   - Ensure commands exit with 0 for success, non-zero for failure
   - Test commands independently before adding to YAML

4. **Move remaining items to appropriate initial conditions:**
   - Keep test-specific setup in initial_conditions
   - Use BDD patterns for executable setup steps
   - Use plain text for documentation/context

5. **Test the migrated YAML:**
   ```bash
   # Validate YAML syntax and schema
   test-executor validate your_test.yaml
   
   # Generate script to review prerequisite checks
   test-executor generate your_test.yaml output.sh
   
   # Review generated prerequisite checks
   head -50 output.sh
   ```

### Example Migration

**Before:**
```yaml
initial_conditions:
  eUICC:
    - "Ensure you have physical access to the device"
    - "Verify SSH port 22 is accessible"
    - "Check that test configuration file exists"
    - "Device should be in factory reset state"
```

**After:**
```yaml
prerequisites:
  - type: manual
    description: "Ensure you have physical access to the device"
  - type: manual
    description: "Verify device is in factory reset state"
  - type: automatic
    description: "Check that SSH port 22 is accessible"
    verification_command: "nc -z -w 5 192.168.1.100 22 && exit 0 || exit 1"
  - type: automatic
    description: "Verify test configuration file exists"
    verification_command: "test -f /etc/test/config.json && exit 0 || exit 1"

initial_conditions:
  eUICC:
    - "Device configuration will be loaded during test execution"
```

## Creating Prerequisites Interactively

You can add prerequisites when building a test case using the interactive mode:

```bash
test-case-creator create
```

The tool will prompt you:

1. **Add prerequisites?** (yes/no)
   - Select "yes" to add prerequisites

2. **For each prerequisite:**
   - **Type**: Choose between manual or automatic
   - **Description**: Enter a human-readable description
   - **Verification command** (automatic only): Enter the command to verify the prerequisite

3. **Add another prerequisite?** (yes/no)
   - Select "yes" to add more prerequisites
   - Select "no" to finish and continue with the test case

### Example Interactive Session

```
=== Prerequisites ===

Add prerequisites? [y/n]: y

--- Adding Prerequisite #1 ---
Select prerequisite type:
  1. manual
  2. automatic
Enter choice [1-2]: 2

Prerequisite description: Check network connectivity to primary device
Verification command: ping -c 3 192.168.1.100 && exit 0 || exit 1

Add another prerequisite? [y/n]: y

--- Adding Prerequisite #2 ---
Select prerequisite type:
  1. manual
  2. automatic
Enter choice [1-2]: 1

Prerequisite description: Ensure you have administrative access to the test devices

Add another prerequisite? [y/n]: n

✓ Added 2 prerequisite(s)
```

## Testing Prerequisites

### Generate Test Script

Generate the bash script to see how prerequisites will be executed:

```bash
test-executor generate testcases/TC_TEST_001.yaml output.sh
cat output.sh
```

Review the prerequisite checking logic at the beginning of the script.

### Dry Run

Execute the script to test prerequisites without running the full test:

```bash
# Make script executable
chmod +x output.sh

# Run the script (will stop at first test step)
./output.sh
```

If prerequisites fail, you'll see detailed error messages indicating which prerequisite failed and why.

### Test in Non-Interactive Mode

Simulate CI/CD execution:

```bash
# Set DEBIAN_FRONTEND to noninteractive
DEBIAN_FRONTEND=noninteractive ./output.sh

# Or redirect stdin to simulate no TTY
./output.sh < /dev/null
```

Manual prerequisites will be automatically skipped with a message.

## Common Patterns and Examples

### Example 1: Web Application Testing

```yaml
prerequisites:
  # Manual prerequisites
  - type: manual
    description: "Ensure you have admin credentials for the test environment"
  
  # Automatic prerequisites - infrastructure
  - type: automatic
    description: "Verify Docker daemon is running"
    verification_command: "docker info >/dev/null 2>&1 && exit 0 || exit 1"
  
  - type: automatic
    description: "Check that web server container is running"
    verification_command: "docker ps | grep -q web-server && exit 0 || exit 1"
  
  - type: automatic
    description: "Verify web service is responding on port 8080"
    verification_command: "curl -s -o /dev/null -w '%{http_code}' http://localhost:8080/health | grep -q '200' && exit 0 || exit 1"
  
  # Automatic prerequisites - database
  - type: automatic
    description: "Verify PostgreSQL is accessible"
    verification_command: "pg_isready -h localhost -p 5432 && exit 0 || exit 1"
  
  # Automatic prerequisites - resources
  - type: automatic
    description: "Ensure sufficient disk space (at least 1GB free)"
    verification_command: "df -k /tmp | awk 'NR==2 {if ($4 > 1048576) exit 0; else exit 1}'"
```

### Example 2: IoT Device Testing

```yaml
prerequisites:
  # Manual prerequisites
  - type: manual
    description: "Verify device is powered on and LED is blinking green"
  - type: manual
    description: "Ensure device is connected to test bench via USB"
  - type: manual
    description: "Confirm SIM card is inserted in the device"
  
  # Automatic prerequisites - network
  - type: automatic
    description: "Check network connectivity to device"
    verification_command: "ping -c 5 192.168.1.100 && exit 0 || exit 1"
  
  - type: automatic
    description: "Verify SSH access to device"
    verification_command: "ssh -o ConnectTimeout=5 -o BatchMode=yes testuser@192.168.1.100 exit && exit 0 || exit 1"
  
  # Automatic prerequisites - files
  - type: automatic
    description: "Check that device firmware file exists"
    verification_command: "test -f /tmp/firmware/device_v1.2.3.bin && exit 0 || exit 1"
  
  - type: automatic
    description: "Verify test data directory is present"
    verification_command: "test -d /opt/test_data && exit 0 || exit 1"
```

### Example 3: CI/CD Pipeline Testing

```yaml
prerequisites:
  # No manual prerequisites in CI/CD - all must be automatic
  
  # Automatic prerequisites - services
  - type: automatic
    description: "Verify Jenkins agent is reachable"
    verification_command: "ping -c 2 jenkins-agent-01 && exit 0 || exit 1"
  
  - type: automatic
    description: "Check that Docker is available"
    verification_command: "docker version >/dev/null 2>&1 && exit 0 || exit 1"
  
  - type: automatic
    description: "Verify kubectl can connect to cluster"
    verification_command: "kubectl cluster-info >/dev/null 2>&1 && exit 0 || exit 1"
  
  # Automatic prerequisites - environment
  - type: automatic
    description: "Verify CI environment variables are set"
    verification_command: "test -n \"$CI_PROJECT_ID\" && test -n \"$CI_COMMIT_SHA\" && exit 0 || exit 1"
  
  # Automatic prerequisites - resources
  - type: automatic
    description: "Check container registry is accessible"
    verification_command: "docker pull alpine:latest >/dev/null 2>&1 && exit 0 || exit 1"
```

## Troubleshooting

### Prerequisite Always Fails

**Problem:** Automatic prerequisite fails even though condition seems met.

**Solutions:**
1. Test the verification command manually:
   ```bash
   # Run the command and check exit code
   ping -c 3 192.168.1.100
   echo $?  # Should be 0 for success
   ```

2. Add explicit exit codes:
   ```yaml
   # Before (might not work reliably)
   verification_command: "ping -c 3 192.168.1.100"
   
   # After (explicit exit codes)
   verification_command: "ping -c 3 192.168.1.100 && exit 0 || exit 1"
   ```

3. Check for command availability:
   ```yaml
   # Verify command exists before using it
   verification_command: "command -v nc >/dev/null 2>&1 && nc -z 192.168.1.100 22 && exit 0 || exit 1"
   ```

### Manual Prerequisite Hangs

**Problem:** Script hangs when checking manual prerequisite in interactive mode.

**Cause:** The script is waiting for ENTER key press, but input is redirected or not available.

**Solutions:**
1. Run in non-interactive mode:
   ```bash
   DEBIAN_FRONTEND=noninteractive ./test_script.sh
   ```

2. Ensure TTY is available:
   ```bash
   # Run with TTY allocated
   ./test_script.sh
   
   # For SSH connections, use -t flag
   ssh -t user@host './test_script.sh'
   ```

### Prerequisite Command Timeout

**Problem:** Prerequisite verification command takes too long or hangs.

**Solution:** Add timeout to verification command:
```yaml
- type: automatic
  description: "Check network connectivity with timeout"
  verification_command: "timeout 10 ping -c 3 192.168.1.100 && exit 0 || exit 1"
```

### Environment Variables Not Available

**Problem:** Verification command references environment variable that isn't set.

**Solutions:**
1. Check variable is set in prerequisite:
   ```yaml
   - type: automatic
     description: "Verify API_KEY is set"
     verification_command: "test -n \"$API_KEY\" && exit 0 || exit 1"
   ```

2. Provide default value in command:
   ```yaml
   verification_command: "test -n \"${API_KEY:-}\" || echo 'API_KEY not set' && exit 1"
   ```

### Command Output Shows Errors

**Problem:** Prerequisite passes but shows error messages in output.

**Solution:** Redirect error output:
```yaml
# Before (shows errors)
verification_command: "docker info"

# After (suppresses errors)
verification_command: "docker info >/dev/null 2>&1 && exit 0 || exit 1"
```

## See Also

- [BDD Initial Conditions](../features/bdd-initial-conditions.md) - For executable setup steps
- [Quick Start Guide](index.md) - Getting started with test cases
- [Test Executor Usage](../cli-tools/test-verify-usage.md) - Running test scripts
- [Schema Validation](../user-guide/validation.md) - YAML schema validation
