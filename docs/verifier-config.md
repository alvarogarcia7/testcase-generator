# Verifier Configuration

The verifier binary supports configuration through YAML files and CLI flags, with CLI flags taking precedence over config file values.

## Configuration File Format

The configuration file is in YAML format with the following fields:

```yaml
# Report title (optional)
title: "Test Execution Results"

# Project name (optional)
project: "Test Case Manager - Verification Results"

# Environment information (optional)
environment: "Staging"

# Platform information (optional)
platform: "Linux x86_64"

# Executor information (optional)
executor: "Jenkins v3.2"
```

All fields are optional. If not specified, the following defaults are used:
- `title`: "Test Execution Results"
- `project`: "Test Case Manager - Verification Results"
- `environment`: None
- `platform`: None
- `executor`: None

## Usage

### Using Defaults

When no config file or CLI flags are specified, the verifier uses default values:

```bash
verifier -f logs/ --format yaml --output report.yaml
```

### Using a Configuration File

Specify a configuration file with the `--config` flag:

```bash
verifier -f logs/ --format yaml --output report.yaml --config verifier-config.yaml
```

### Overriding Configuration with CLI Flags

CLI flags take precedence over configuration file values:

```bash
verifier -f logs/ --format yaml --output report.yaml \
  --config verifier-config.yaml \
  --title "Nightly Test Run" \
  --environment "Production"
```

In this example, `title` and `environment` from the CLI override the config file, while other values come from the config file.

### All CLI Flags

The following CLI flags can override configuration file values:
- `--title <TITLE>`: Report title
- `--project <PROJECT>`: Project name
- `--environment <ENV>`: Environment information
- `--platform <PLATFORM>`: Platform information
- `--executor <EXECUTOR>`: Executor information

## Configuration Precedence

The verifier applies configuration in the following order (later values override earlier ones):

1. Default values (hardcoded in the application)
2. Configuration file values (if `--config` is specified)
3. CLI flag values (if provided)

## Example Configuration Files

### Minimal Configuration

```yaml
title: "My Test Suite"
project: "My Project"
```

### Complete Configuration

```yaml
title: "Nightly Test Run"
project: "Production Suite"
environment: "Staging"
platform: "Linux x86_64"
executor: "Jenkins v3.2"
```

### CI/CD Integration

```yaml
title: "CI Pipeline Tests"
project: "Application Suite"
environment: "CI"
platform: "Ubuntu 22.04"
executor: "GitHub Actions"
```

## See Also

- [verifier-config.example.yaml](../verifier-config.example.yaml) - Example configuration file
