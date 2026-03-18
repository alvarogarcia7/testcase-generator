# Variable Test Cases

This directory contains 11 comprehensive test cases that validate variable capture, substitution, and hydration functionality.

## Test Cases Overview

### TC_VAR_001 - Single Variable Capture
- **Purpose**: Test capturing a single variable from command output using regex pattern
- **Features**: Basic regex-based variable capture
- **Validates**: Variable capture, numeric validation, value verification

### TC_VAR_002 - Multiple Variable Capture
- **Purpose**: Test capturing multiple variables from single step output using regex patterns
- **Features**: Multiple capture_vars in one step, JSON-like output parsing
- **Validates**: Simultaneous capture of multiple fields, email format validation

### TC_VAR_003 - Use in Subsequent Steps
- **Purpose**: Test using captured variable in subsequent step commands
- **Features**: Variable persistence across steps, variable substitution
- **Validates**: Cross-step variable availability, substitution in commands

### TC_VAR_004 - Hydration Export
- **Purpose**: Test variable hydration from export file with environment variables
- **Features**: Hydration variables with ${#VAR_NAME} syntax
- **Validates**: Hydration variable usage, export file integration
- **Hydration Vars**: TEST_API_URL, TEST_API_KEY, TEST_TIMEOUT

### TC_VAR_005 - Optional vs Required Variables
- **Purpose**: Test optional vs required hydration variables with default values
- **Features**: Required/optional flags, default values, variable validation
- **Validates**: Required variable enforcement, optional variable handling
- **Hydration Vars**: REQUIRED_VAR, REQUIRED_WITH_DEFAULT, OPTIONAL_VAR, OPTIONAL_NO_DEFAULT

### TC_VAR_006 - Command-Based Capture
- **Purpose**: Test command-based variable capture using command execution
- **Features**: Command-based capture (alternative to regex)
- **Validates**: System command execution for variable capture (date, hostname, whoami, wc)

### TC_VAR_007 - Across Sequences
- **Purpose**: Test variables captured and used across multiple test sequences
- **Features**: Cross-sequence variable persistence
- **Validates**: Variable availability across 3 sequences, variable immutability

### TC_VAR_008 - Complex Substitution
- **Purpose**: Test complex variable substitution with default values and bash parameter expansion
- **Features**: ${VAR:-default} syntax, nested variable substitution
- **Validates**: Bash parameter expansion, default value fallbacks
- **Hydration Vars**: OPTIONAL_CONFIG, SERVICE_NAME

### TC_VAR_009 - Mixed Capture
- **Purpose**: Test mixing regex-based and command-based variable captures in same step
- **Features**: Combines regex and command capture methods
- **Validates**: Both capture methods working together in single step

### TC_VAR_010 - JSON Extraction
- **Purpose**: Test extracting variables from JSON output using regex patterns
- **Features**: JSON parsing with regex, nested field extraction
- **Validates**: JSON field extraction, data manipulation with captured vars

### TC_VAR_011 - Sequence Variables
- **Purpose**: Test sequence-scoped variables declared in test sequence variables section
- **Features**: Sequence-level variable declarations
- **Validates**: Sequence-scoped variables, variable scope isolation between sequences

## Validation Status

All 11 test cases have been validated:
- ✅ YAML syntax validation
- ✅ Schema compliance (test-case.schema.json)
- ✅ Script generation
- ✅ Export file generation (for hydration test cases)

## Changes Made

### Fixed Files

1. **TC_VAR_004_hydration_export.yaml**
   - Fixed hydration variable syntax (using ${#VAR_NAME} correctly)
   - Added comprehensive verification conditions
   - Ensured proper default value usage

2. **TC_VAR_005_optional_required.yaml**
   - Fixed hydration variable syntax
   - Added default values for required variables
   - Enhanced verification to check actual values

3. **TC_VAR_008_complex_substitution.yaml**
   - Maintained proper ${#VAR_NAME} syntax for hydration vars
   - Used ${VAR:-default} for bash parameter expansion
   - Added verification for default value behavior

4. **TC_VAR_009_mixed_capture.yaml**
   - Fixed command-based capture using printf instead of echo with \n
   - Changed from: `echo 'text\ntext'` 
   - Changed to: `printf 'text\\ntext\\n'`
   - Ensures proper newline handling across all shells

## Key Syntax Patterns

### Hydration Variables (YAML Level)
```yaml
# In YAML files, use ${#VAR_NAME} for variables that will be hydrated
command: "echo ${#TEST_API_URL}"
```

### Bash Variables (Script Level)
```yaml
# For regular variable substitution in generated scripts
command: "echo ${VAR_NAME}"
command: "echo ${VAR_NAME:-default_value}"
```

### Variable Capture Methods

#### Regex-Based Capture
```yaml
capture_vars:
  - name: user_id
    capture: 'User ID: ([0-9]+)'
```

#### Command-Based Capture
```yaml
capture_vars:
  - name: timestamp
    command: "date +%s"
```

#### Mixed Capture
```yaml
capture_vars:
  - name: user_id
    capture: 'User ID: ([0-9]+)'
  - name: timestamp
    command: "date +%s"
```

## Hydration Variables

To generate export files for test cases with hydration_vars:

```bash
# Generate export template
target/debug/test-executor generate-export TC_VAR_004_hydration_export.yaml -o TC_VAR_004.env

# Edit the export file with your values
vi TC_VAR_004.env

# Validate export file has all required variables
target/debug/test-executor validate-export TC_VAR_004_hydration_export.yaml TC_VAR_004.env
```

## Running Tests

```bash
# Generate test script
target/debug/test-executor generate TC_VAR_001_single_capture.yaml -o TC_VAR_001.sh

# Execute test
target/debug/test-executor execute TC_VAR_001_single_capture.yaml

# For tests with hydration variables
target/debug/test-executor execute TC_VAR_004_hydration_export.yaml --export TC_VAR_004.env
```
