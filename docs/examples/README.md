# Examples

Practical examples and common usage patterns.

## Quick Examples

### Creating a Test Case

```bash
# Interactive workflow
editor complete --output testcases/my_test.yml
```

### Validating Test Cases

```bash
# Validate single file
editor validate testcases/my_test.yml

# Watch mode
validate-yaml testcases/*.yml --schema schema.json --watch
```

## Example Patterns

### BDD Initial Conditions

```yaml
initial_conditions:
  setup:
    - "create directory \"/tmp/test\""
    - "set environment variable \"TEST_MODE\" to \"enabled\""
    - "wait for 5 seconds"
```

### Variable Passing

```yaml
steps:
  - step: 1
    description: "Get device ID"
    command: "echo 'Device: DEV12345'"
    capture:
      - pattern: "Device: (\\w+)"
        variable: "DEVICE_ID"
  
  - step: 2
    description: "Use captured ID"
    command: "echo \"Testing ${DEVICE_ID}\""
```

## Resources

- [Main Documentation](../index.md)
- [User Guide](../user-guide/)
- [CLI Tools](../cli-tools/)
- [Features](../features/)
