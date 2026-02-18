# Examples

This section provides practical examples and common usage patterns for Test Case Manager.

## Example Test Cases

Browse the [testcases/](../../testcases/) and [data/](../../data/) directories for real-world test case examples.

## Common Workflows

### 1. Creating a Test Case

```bash
# Interactive workflow
editor complete --output testcases/my_test.yml

# Or with metadata prompts
editor create-interactive --path ./testcases
```

### 2. Building Test Sequences

```bash
# Build sequences with steps
editor build-sequences-with-steps

# Build sequences only (no steps)
editor build-sequences
```

### 3. Validating Test Cases

```bash
# Validate single file
editor validate testcases/my_test.yml

# Validate all test cases
editor validate --all

# Watch mode - auto-validate on changes
validate-yaml testcases/*.yml --schema schema.json --watch
```

### 4. Executing Tests

```bash
# Generate bash script
test-executor generate testcase.yml --output test.sh

# Execute test directly
test-executor execute testcase.yml

# Verify execution logs
test-verify single --log execution.log --test-case-id TC001
```

### 5. Batch Verification

```bash
# Verify multiple test logs
test-verify batch --logs logs/*.log --format junit --output report.xml
```

## Example Patterns

### BDD Initial Conditions

```yaml
initial_conditions:
  eUICC:
    - "create directory \"/tmp/test\""
    - "set environment variable \"TEST_MODE\" to \"enabled\""
    - "wait for 5 seconds"
    - "file \"/etc/config.txt\" should exist"
```

### Variable Passing

```yaml
steps:
  - step: 1
    description: "Get device ID"
    command: "echo 'Device: DEV12345'"
    expected:
      result: "0"
      output: "Device: DEV12345"
    capture:
      - pattern: "Device: (\\w+)"
        variable: "DEVICE_ID"
  
  - step: 2
    description: "Use captured device ID"
    command: "echo \"Testing device ${DEVICE_ID}\""
    expected:
      result: "0"
      output: "Testing device DEV12345"
```

### Manual Steps

```yaml
steps:
  - step: 1
    manual: true
    description: "Physically inspect device LED status"
    expected:
      result: "N/A"
      output: "LED is blinking green"
  
  - step: 2
    description: "Automated check"
    command: "ping -c 3 192.168.1.100"
    expected:
      result: "0"
      output: "*"
```

### Prerequisites

```yaml
prerequisites:
  - type: manual
    description: "Ensure device is powered on and connected"
  
  - type: automatic
    description: "Check network connectivity"
    verification_command: "ping -c 3 192.168.1.100 && exit 0 || exit 1"
```

### Conditional Verification

```yaml
steps:
  - step: 1
    description: "Test with conditional verification"
    command: "echo 'Version 1.2.3'"
    expected:
      result: "0"
      output: "Version 1.2.3"
    verification:
      result: "[ $EXIT_CODE -eq 0 ]"
      output: "[[ \"$COMMAND_OUTPUT\" =~ ^Version\\ [0-9]+\\.[0-9]+\\.[0-9]+$ ]]"
```

## Integration Examples

### Docker Usage

```bash
# Build Docker image
./scripts/build-docker.sh

# Run interactively
docker run -it --rm testcase-manager:latest

# Execute specific command
docker run --rm testcase-manager:latest editor --help
```

### CI/CD Integration

See [GitLab CI Examples](../development/gitlab-ci-examples.md) for complete CI/CD integration examples.

### Watch Mode

```bash
# Monitor testcases directory for changes
./scripts/validate-files.sh \
    --pattern '\.ya?ml$' \
    --validator ./scripts/validate-yaml-wrapper.sh \
    --watch
```

## Demo Scripts

Run example demonstrations:

```bash
# Interactive workflow demo
cargo run --example interactive_workflow

# TTY fallback demo
cargo run --example tty_fallback_demo

# Test verify demo
cargo run --example test_verify_demo
```

## Real-World Scenarios

### IoT Device Testing

Test suite for IoT device with network connectivity, firmware updates, and sensor readings:

```yaml
requirement: "IOT_001"
item: 1
tc: 1
id: 'TC_IOT_001'
description: 'IoT Device Firmware Update Test'

prerequisites:
  - type: manual
    description: "Verify device is powered on and LED is green"
  - type: automatic
    description: "Check device is reachable"
    verification_command: "ping -c 3 192.168.1.100"

initial_conditions:
  device:
    - "set environment variable \"DEVICE_IP\" to \"192.168.1.100\""
    - "create directory \"/tmp/firmware\""

test_sequences:
  - id: 1
    name: "Firmware Update"
    description: "Update device firmware"
    steps:
      - step: 1
        description: "Check current firmware version"
        command: "ssh testuser@${DEVICE_IP} 'cat /etc/firmware_version'"
        expected:
          result: "0"
          output: "*"
        capture:
          - pattern: "v(\\d+\\.\\d+\\.\\d+)"
            variable: "OLD_VERSION"
```

### Web API Testing

Automated API testing with authentication and data validation:

```yaml
requirement: "API_001"
item: 1
tc: 1
id: 'TC_API_001'
description: 'REST API Authentication Test'

prerequisites:
  - type: automatic
    description: "Verify API server is responding"
    verification_command: "curl -s -o /dev/null -w '%{http_code}' http://localhost:8080/health | grep -q '200'"

test_sequences:
  - id: 1
    name: "API Authentication"
    description: "Test API login and token usage"
    steps:
      - step: 1
        description: "Login and capture token"
        command: "curl -X POST http://localhost:8080/api/login -d '{\"user\":\"admin\",\"pass\":\"secret\"}'"
        expected:
          result: "0"
          output: "*token*"
        capture:
          - pattern: "\"token\":\\s*\"([^\"]+)\""
            variable: "AUTH_TOKEN"
      
      - step: 2
        description: "Use token for authenticated request"
        command: "curl -H \"Authorization: Bearer ${AUTH_TOKEN}\" http://localhost:8080/api/user"
        expected:
          result: "0"
          output: "*success*"
```

## Additional Resources

- [Main Documentation](../index.md)
- [User Guide](../user-guide/)
- [CLI Tools Reference](../cli-tools/)
- [Features Documentation](../features/)
