# Verifier Scripts Reference

This document provides a quick reference for all scripts that invoke the verifier binary with container configuration support.

## Core Scripts

### 1. `scripts/run_verifier_and_generate_reports.sh`

**Purpose**: Run verifier on all test scenarios and generate comprehensive reports (PDF, documentation).

**Usage**:
```bash
# Using default config
./scripts/run_verifier_and_generate_reports.sh

# Using custom config file
./scripts/run_verifier_and_generate_reports.sh --config my_config.yml

# Using CLI overrides
./scripts/run_verifier_and_generate_reports.sh \
  --title "Nightly Test Run" \
  --environment "Production" \
  --executor "Jenkins v3.2"

# Combining config file and CLI overrides
./scripts/run_verifier_and_generate_reports.sh \
  --config staging_config.yml \
  --title "Hotfix Verification" \
  --environment "Staging"
```

**Options**:
- `--config FILE` - Path to container config file
- `--title TITLE` - Override report title
- `--project PROJECT` - Override project name
- `--environment ENV` - Override environment information
- `--platform PLATFORM` - Override platform information
- `--executor EXECUTOR` - Override executor information

### 2. `scripts/generate_documentation_reports.sh`

**Purpose**: Orchestrate end-to-end report generation with test-plan-doc-gen integration.

**Usage**:
```bash
# Using default settings
./scripts/generate_documentation_reports.sh

# Custom directories and config
./scripts/generate_documentation_reports.sh \
  --logs-dir testcases/verifier_scenarios \
  --output-dir reports/custom \
  --config container_config.yml

# With CLI overrides
./scripts/generate_documentation_reports.sh \
  --config my_config.yml \
  --environment "Staging" \
  --executor "CI Pipeline"
```

**Options**:
- `--logs-dir DIR` - Directory containing execution logs
- `--test-case-dir DIR` - Directory containing test case YAML files
- `--output-dir DIR` - Output directory for reports
- `--test-plan-doc-gen DIR` - Path to test-plan-doc-gen directory
- `--container-template FILE` - Path to container template YAML
- `--config FILE` - Path to container config file
- `--title TITLE` - Override report title
- `--project PROJECT` - Override project name
- `--environment ENV` - Override environment information
- `--platform PLATFORM` - Override platform information
- `--executor EXECUTOR` - Override executor information

### 3. `generate_reports.sh`

**Purpose**: Simple report generation for verifier scenarios.

**Usage**:
```bash
# Using default config
./generate_reports.sh

# Using custom config
./generate_reports.sh --config my_config.yml

# Using CLI flags
./generate_reports.sh \
  --title "Test Results" \
  --environment "Production"
```

**Options**:
- `--config FILE` - Path to container config file
- `--title TITLE` - Override report title
- `--project PROJECT` - Override project name
- `--environment ENV` - Override environment information
- `--platform PLATFORM` - Override platform information
- `--executor EXECUTOR` - Override executor information

### 4. `scripts/run_verifier_with_env.sh`

**Purpose**: Run verifier with environment-specific configuration based on DEPLOY_ENV.

**Usage**:
```bash
# Development environment (uses container_config.dev.yml)
DEPLOY_ENV=dev ./scripts/run_verifier_with_env.sh --folder logs/ --format yaml

# Staging environment (uses container_config.staging.yml)
DEPLOY_ENV=staging ./scripts/run_verifier_with_env.sh \
  --log execution.json \
  --test-case TEST_001 \
  --format json \
  --output report.json

# Production environment (uses container_config.prod.yml)
DEPLOY_ENV=prod ./scripts/run_verifier_with_env.sh --folder logs/ --format json
```

**Environment Variables**:
- `DEPLOY_ENV` - Environment name (dev, staging, prod). Default: dev
- `BUILD_VARIANT` - Cargo build variant (--debug or --release). Default: --release
- `BUILD_NUMBER` - Optional build number (added to executor field if set)

## Integration Test Scripts

### 1. `tests/integration/test_verifier_e2e.sh`

**Purpose**: End-to-end integration test for verifier binary. Demonstrates both config file and CLI flag patterns.

**Usage**:
```bash
# Run all tests
./tests/integration/test_verifier_e2e.sh

# Keep temporary files for debugging
./tests/integration/test_verifier_e2e.sh --no-remove
```

**What it tests**:
- Single-file mode with config file
- Single-file mode with CLI flag overrides
- JSON output format with config + CLI combination
- Folder discovery mode
- Error handling

### 2. `tests/integration/test_documentation_generation.sh`

**Purpose**: Integration test for documentation generation workflow with config file support.

**Usage**:
```bash
# Run test
./tests/integration/test_documentation_generation.sh

# Keep temporary files
./tests/integration/test_documentation_generation.sh --no-remove
```

**What it tests**:
- Verifier execution with config file
- Conversion to result YAML
- AsciiDoc and Markdown report generation

### 3. `tests/integration/test_docker_build.sh`

**Purpose**: Docker build verification with CLI flag demonstration.

**Usage**:
```bash
# Run test
./tests/integration/test_docker_build.sh

# Keep Docker image after test
./tests/integration/test_docker_build.sh --no-remove
```

**What it tests**:
- Docker image builds correctly
- Verifier binary works in container
- CLI flags work in Docker environment

## Configuration File Examples

### Default Configuration (`container_config.yml`)
```yaml
title: "Test Execution Results"
project: "Test Case Manager - Verification Results"
environment: "Development"
platform: "Linux x86_64"
executor: "Local Development"
```

### Development Environment (`container_config.dev.yml`)
```yaml
title: "Development Test Results"
project: "Test Case Manager - Development"
environment: "Development"
platform: "Local Development Machine"
executor: "Local Development Environment"
```

### Staging Environment (`container_config.staging.yml`)
```yaml
title: "Staging Test Results"
project: "Test Case Manager - Staging Verification"
environment: "Staging"
platform: "AWS EC2 Linux x86_64"
executor: "Jenkins Staging Pipeline v3.2"
```

### Production Environment (`container_config.prod.yml`)
```yaml
title: "Production Test Results"
project: "Test Case Manager - Production Verification"
environment: "Production"
platform: "AWS EC2 Linux x86_64"
executor: "Jenkins Production Pipeline v3.2"
```

## Common Patterns

### Pattern 1: Local Development
```bash
# Use default config for local development
./scripts/run_verifier_and_generate_reports.sh
```

### Pattern 2: CI/CD Pipeline
```bash
# Use environment variable to select config
DEPLOY_ENV=staging ./scripts/run_verifier_with_env.sh \
  --folder logs/ \
  --format json \
  --output report.json

# Or use CLI flags for dynamic values
./scripts/run_verifier_and_generate_reports.sh \
  --config container_config.yml \
  --environment "$CI_ENVIRONMENT" \
  --executor "Build #$BUILD_NUMBER"
```

### Pattern 3: Manual Testing
```bash
# Override specific fields for ad-hoc testing
./generate_reports.sh \
  --title "Manual Test Run - $(date +%Y%m%d)" \
  --executor "$(whoami)@$(hostname)"
```

### Pattern 4: Environment-Specific Automation
```bash
# Create wrapper script that automatically selects config
#!/usr/bin/env bash
ENVIRONMENT="${1:-dev}"
CONFIG="container_config.${ENVIRONMENT}.yml"

if [[ ! -f "$CONFIG" ]]; then
  echo "Config not found: $CONFIG"
  exit 1
fi

./scripts/run_verifier_and_generate_reports.sh --config "$CONFIG"
```

## Precedence Rules

When using both config files and CLI flags, remember the precedence order:

1. **CLI flags** (highest priority)
2. **Config file values**
3. **Default values** (lowest priority)

Example:
```bash
# Config file has: environment: "Staging"
# CLI flag has: --environment "Production"
# Result: "Production" (CLI flag wins)

./scripts/run_verifier_and_generate_reports.sh \
  --config staging_config.yml \
  --environment "Production"
```

## Quick Command Reference

| Task | Command |
|------|---------|
| Run with default config | `./scripts/run_verifier_and_generate_reports.sh` |
| Run with custom config | `./scripts/run_verifier_and_generate_reports.sh --config my_config.yml` |
| Run with CLI overrides | `./scripts/run_verifier_and_generate_reports.sh --title "My Title" --environment "Prod"` |
| Run with environment selection | `DEPLOY_ENV=staging ./scripts/run_verifier_with_env.sh --folder logs/` |
| Generate simple reports | `./generate_reports.sh --config my_config.yml` |
| Run e2e tests | `./tests/integration/test_verifier_e2e.sh` |
| Run doc generation tests | `./tests/integration/test_documentation_generation.sh` |
| Run Docker tests | `./tests/integration/test_docker_build.sh` |

## Troubleshooting

### Config file not found
**Problem**: Error message about missing config file

**Solution**: 
- Verify the path is correct
- Use absolute path if needed: `--config /full/path/to/config.yml`
- Check that the file exists: `ls -la container_config.yml`

### CLI flags not working
**Problem**: CLI flag values not appearing in output

**Solution**:
- Ensure flags come after `--` in cargo commands: `cargo run -- --title "Title"`
- Check for typos in flag names
- Verify output format supports metadata (YAML/JSON do, text might not)

### Wrong environment config used
**Problem**: Script uses wrong environment configuration

**Solution**:
- Check DEPLOY_ENV variable: `echo $DEPLOY_ENV`
- Verify environment-specific config exists: `ls container_config.*.yml`
- Use explicit config path: `--config container_config.staging.yml`

## See Also

- [VERIFIER_CONFIG_GUIDE.md](VERIFIER_CONFIG_GUIDE.md) - Detailed configuration guide
- [AGENTS.md](AGENTS.md) - Project documentation with verifier details
- [verifier-config.example.yaml](verifier-config.example.yaml) - Example configuration file
