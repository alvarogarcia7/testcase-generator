# Verifier Container Configuration Guide

This guide explains how to use the container configuration system for the verifier binary, including config files and CLI flag overrides.

## Overview

The verifier binary supports rich metadata in its output reports through a container configuration system. You can provide metadata in three ways:

1. **Default values** - Built-in defaults used when no configuration is provided
2. **Configuration file** - YAML file with metadata fields
3. **CLI flags** - Command-line arguments that override config file values

**Precedence**: CLI flags > Configuration file > Default values

## Configuration File Format

Configuration files use YAML format with the following fields:

```yaml
# Report title (default: "Test Execution Results")
title: "Test Execution Results"

# Project name (default: "Test Case Manager - Verification Results")
project: "Test Case Manager - Verification Results"

# Optional: Environment information (e.g., "Development", "Staging", "Production")
environment: "Staging"

# Optional: Platform information (e.g., "Linux x86_64", "macOS ARM64")
platform: "Linux x86_64"

# Optional: Executor information (e.g., "CI Pipeline v2.1", "Jenkins v3.2")
executor: "Jenkins v3.2"
```

All fields are optional. If a field is not specified, the default value is used.

## Usage Patterns

### Pattern 1: Using Default Configuration File

The project includes a default configuration file at `container_config.yml`:

```bash
# Use the default config file
verifier -f logs/ --format yaml --output report.yaml
```

The verifier will automatically use `container_config.yml` if no `--config` flag is provided and the file exists.

### Pattern 2: Using Custom Configuration File

Create a custom configuration file for your environment:

```bash
# Create custom config
cat > my_config.yml << EOF
title: "Production Test Results"
project: "My Application - Test Suite"
environment: "Production"
platform: "AWS EC2 Linux"
executor: "Jenkins Production Pipeline v2.1"
EOF

# Use custom config file
verifier -f logs/ --format yaml --output report.yaml --config my_config.yml
```

### Pattern 3: Using CLI Flags Only

Override specific fields using command-line flags:

```bash
# Use CLI flags without config file
verifier -f logs/ --format yaml --output report.yaml \
  --title "Nightly Test Run" \
  --environment "Production" \
  --platform "Linux x86_64"
```

### Pattern 4: Combining Config File and CLI Flags

Use a config file for base configuration, then override specific fields with CLI flags:

```bash
# Use config file + CLI overrides
verifier -f logs/ --format yaml --output report.yaml \
  --config my_config.yml \
  --title "Hotfix Verification" \
  --environment "Production"
```

In this example, `title` and `environment` from CLI flags will override the values in `my_config.yml`, while other fields use the config file values.

## CLI Flags Reference

| Flag | Description | Example |
|------|-------------|---------|
| `--config <PATH>` | Path to YAML configuration file | `--config config.yml` |
| `--title <TITLE>` | Report title | `--title "Test Results"` |
| `--project <PROJECT>` | Project name | `--project "My Project"` |
| `--environment <ENV>` | Environment information | `--environment "Staging"` |
| `--platform <PLATFORM>` | Platform information | `--platform "Linux x86_64"` |
| `--executor <EXECUTOR>` | Executor information | `--executor "CI Pipeline"` |

## Script Integration Examples

### Example 1: Shell Script with Config File

```bash
#!/usr/bin/env bash
set -e

# Define config file
CONFIG_FILE="container_config.yml"

# Run verifier with config
cargo run --release --bin verifier -- \
  --log execution_log.json \
  --test-case TEST_001 \
  --format json \
  --output report.json \
  --config "$CONFIG_FILE"
```

### Example 2: Shell Script with CLI Flags

```bash
#!/usr/bin/env bash
set -e

# Get environment from user
ENVIRONMENT="${TEST_ENVIRONMENT:-Development}"

# Run verifier with CLI flags
cargo run --release --bin verifier -- \
  --log execution_log.json \
  --test-case TEST_001 \
  --format json \
  --output report.json \
  --title "Automated Test Run" \
  --environment "$ENVIRONMENT" \
  --platform "$(uname -s) $(uname -m)" \
  --executor "Local Script"
```

### Example 3: Shell Script with Dynamic Overrides

```bash
#!/usr/bin/env bash
set -e

# Base config file
CONFIG_FILE="container_config.yml"

# Build verifier command
VERIFIER_CMD="verifier -f logs/ --format yaml --output report.yaml"

# Add config file if it exists
if [[ -f "$CONFIG_FILE" ]]; then
  VERIFIER_CMD="$VERIFIER_CMD --config \"$CONFIG_FILE\""
fi

# Add CLI overrides from environment variables
if [[ -n "$TEST_TITLE" ]]; then
  VERIFIER_CMD="$VERIFIER_CMD --title \"$TEST_TITLE\""
fi

if [[ -n "$TEST_ENVIRONMENT" ]]; then
  VERIFIER_CMD="$VERIFIER_CMD --environment \"$TEST_ENVIRONMENT\""
fi

# Execute command
eval "$VERIFIER_CMD"
```

## Environment-Specific Configurations

You can maintain separate configuration files for different environments:

```bash
# Development environment
container_config.dev.yml

# Staging environment
container_config.staging.yml

# Production environment
container_config.prod.yml
```

Then use the appropriate config file:

```bash
# Select config based on environment
ENV="${DEPLOY_ENV:-dev}"
CONFIG_FILE="container_config.${ENV}.yml"

verifier -f logs/ --format yaml --output report.yaml --config "$CONFIG_FILE"
```

## CI/CD Integration

### Example: Jenkins Pipeline

```groovy
pipeline {
    stages {
        stage('Verify Tests') {
            steps {
                sh '''
                    verifier -f logs/ --format json --output report.json \
                      --config container_config.yml \
                      --environment "${JENKINS_ENV}" \
                      --executor "Jenkins Build #${BUILD_NUMBER}"
                '''
            }
        }
    }
}
```

### Example: GitLab CI

```yaml
verify_tests:
  script:
    - |
      verifier -f logs/ --format json --output report.json \
        --config container_config.yml \
        --environment "${CI_ENVIRONMENT_NAME}" \
        --platform "${CI_RUNNER_DESCRIPTION}" \
        --executor "GitLab CI Pipeline ${CI_PIPELINE_ID}"
```

### Example: GitHub Actions

```yaml
- name: Verify Tests
  run: |
    verifier -f logs/ --format json --output report.json \
      --config container_config.yml \
      --environment "${{ github.ref_name }}" \
      --platform "${{ runner.os }}" \
      --executor "GitHub Actions Run ${{ github.run_number }}"
```

## Schema Validation

The configuration file format is validated against the JSON schema at `schemas/container_config.schema.json`.

To validate your configuration file:

```bash
# Using a JSON schema validator
ajv validate -s schemas/container_config.schema.json -d container_config.yml
```

## Default Configuration File

The project includes a default `container_config.yml` file at the repository root. This file is used by default if no `--config` flag is specified. You can customize this file for your local development environment.

## Best Practices

1. **Use config files for base configuration**: Define common metadata in config files
2. **Use CLI flags for dynamic values**: Override fields that change per run (e.g., environment, executor)
3. **Keep configs in version control**: Check in environment-specific config files
4. **Document your fields**: Add comments to config files explaining their purpose
5. **Use meaningful values**: Provide descriptive information that helps readers understand the test context

## Troubleshooting

### Config file not found
If you get an error about a missing config file:
- Verify the path is correct relative to where you're running the command
- Use an absolute path if necessary: `--config /full/path/to/config.yml`

### Values not being applied
Check the precedence order:
1. CLI flags take highest priority
2. Config file values are used if no CLI flag is provided
3. Default values are used if neither CLI flag nor config file specify a value

### Invalid config file
If the config file is invalid:
- Check YAML syntax (proper indentation, no tabs)
- Verify all field names match the schema
- Ensure string values are properly quoted if they contain special characters

## Migration Guide

If you have existing scripts using the verifier, update them to use the new config system:

**Before:**
```bash
verifier -f logs/ --format yaml --output report.yaml
```

**After (using config file):**
```bash
verifier -f logs/ --format yaml --output report.yaml --config container_config.yml
```

**After (using CLI flags):**
```bash
verifier -f logs/ --format yaml --output report.yaml \
  --title "Test Results" \
  --environment "Production"
```

The verifier remains backward compatible - if you don't specify any config options, it will use default values.
