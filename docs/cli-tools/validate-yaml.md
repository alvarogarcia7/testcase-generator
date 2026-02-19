# validate-yaml Quick Reference

## Overview

The `validate-yaml` binary validates YAML files against JSON schemas with optional watch mode for continuous monitoring.

## Quick Start

```bash
# Single file validation
validate-yaml testcase.yml --schema schema.json

# Multiple files
validate-yaml file1.yml file2.yml file3.yml --schema schema.json

# Watch mode (auto-revalidate on changes)
validate-yaml testcase.yml --schema schema.json --watch

# Watch multiple files
validate-yaml testcases/*.yml --schema schema.json --watch
```

## Command Syntax

```
validate-yaml [OPTIONS] <YAML_FILES>... --schema <SCHEMA_FILE>
```

### Arguments

- `<YAML_FILES>...` - One or more YAML files to validate (required)
- `--schema <SCHEMA_FILE>` - JSON schema file for validation (required)

### Options

| Flag | Long Form | Description | Platform |
|------|-----------|-------------|----------|
| `-s` | `--schema <FILE>` | JSON schema file path | All |
| `-w` | `--watch` | Enable watch mode | Linux/macOS only |
| `-v` | `--verbose` | Enable verbose logging | All |
| `-h` | `--help` | Print help information | All |
| `-V` | `--version` | Print version | All |

## Usage Examples

### Basic Validation

**Validate single file:**
```bash
validate-yaml testcase.yml --schema schema.json
```

**Validate multiple files:**
```bash
validate-yaml test1.yml test2.yml test3.yml --schema schema.json
```

**Validate with glob patterns:**
```bash
validate-yaml testcases/*.yml --schema testcases/schema.json
validate-yaml testcases/**/*.yml --schema schema.json
```

### Watch Mode

**Watch single file:**
```bash
validate-yaml testcase.yml --schema schema.json --watch
```

**Watch multiple files:**
```bash
validate-yaml testcases/test1.yml testcases/test2.yml --schema schema.json --watch
```

**Watch all YAML files in directory:**
```bash
validate-yaml testcases/*.yml --schema schema.json --watch
```

**Watch with verbose logging:**
```bash
validate-yaml testcases/*.yml --schema schema.json --watch --verbose
```

## Watch Mode Behavior

### Features

1. **Initial Validation** - Validates all specified files on startup
2. **File Monitoring** - Watches for file modifications in real-time
3. **Instant Feedback** - Re-validates changed files immediately
4. **Smart Re-validation** - Runs full validation when all changed files pass
5. **Debounced Events** - Groups rapid changes (300ms window) to avoid duplicates
6. **Color-coded Output** - Visual feedback with green (✓) and red (✗)

### Workflow

```
Start Watch Mode
    ↓
Initial Validation (all files)
    ↓
Display Results & Summary
    ↓
Monitor for Changes
    ↓
[File Modified] → Detect Change → Debounce (300ms)
    ↓
Validate Changed File(s)
    ↓
Display Results
    ↓
[All Changed Files Pass] → Run Full Validation → Display Complete Summary
    ↓
Continue Monitoring
```

### Example Session

```
$ validate-yaml testcases/*.yml --schema schema.json --watch

Watch mode enabled
Monitoring 3 files for changes...

Initial validation:
✓ testcases/test1.yml
✓ testcases/test2.yml
✓ testcases/test3.yml

Summary:
  Total files validated: 3
  Passed: 3
  Failed: 0

Watching for changes...

File changes detected:
  → /path/to/testcases/test2.yml

Validating changed files:
✓ testcases/test2.yml

All changed files passed! Running full validation...

✓ testcases/test1.yml
✓ testcases/test2.yml
✓ testcases/test3.yml

Summary:
  Total files validated: 3
  Passed: 3
  Failed: 0

Watching for changes...
```

### Exit Watch Mode

Press `Ctrl+C` to stop watching and exit.

## Output Format

### Success Output

```
✓ testcases/test1.yml
✓ testcases/test2.yml

Summary:
  Total files validated: 2
  Passed: 2
  Failed: 0
```

### Error Output

```
✗ testcases/test_bad.yml
  Schema constraint violations:
    Error #1: Path '/item'
      Constraint: "not a integer"
      Found value: "not_an_integer"
    Error #2: Path 'root'
      Constraint: Missing required property 'test_sequences'
      Found value: {...}

Summary:
  Total files validated: 1
  Passed: 0
  Failed: 1
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | All validations passed |
| 1 | One or more validations failed |

## Platform Support

### Linux

- **Status**: ✅ Full support
- **Backend**: inotify (via `notify` crate)
- **Watch Mode**: Available
- **Requirements**: None (built-in kernel support)

### macOS

- **Status**: ✅ Full support
- **Backend**: FSEvents (via `notify` crate)
- **Watch Mode**: Available
- **Requirements**: None (built-in system support)

### Windows

- **Status**: ⚠️ Partial support
- **Validation**: Available
- **Watch Mode**: ❌ Not available
- **Reason**: Platform-specific limitations
- **Workarounds**:
  - Use WSL (Windows Subsystem for Linux)
  - Use PowerShell FileSystemWatcher
  - Use scheduled tasks for periodic validation

## Common Use Cases

### Development Workflow

Monitor test cases during development:
```bash
validate-yaml testcases/*.yml --schema schema.json --watch
```

### CI/CD Pipeline

Validate all test cases before commit:
```bash
validate-yaml testcases/*.yml --schema schema.json || exit 1
```

### Batch Validation

Validate multiple test case files:
```bash
validate-yaml testcases/tc_*.yml --schema testcases/schema.json
```

### Debug Mode

Enable verbose logging for troubleshooting:
```bash
validate-yaml testcase.yml --schema schema.json --verbose
```

## Troubleshooting

### Watch mode not working

**Problem**: `--watch` flag not recognized

**Solution**: You may be on Windows where watch mode is disabled. Use standard validation or WSL.

### Files not being detected

**Problem**: Changes to files aren't triggering validation

**Solution**: 
- Ensure you specified the correct files on command line
- Check that the file paths are valid
- Verify the files are being modified (not just accessed)
- Try verbose mode to see file system events: `--verbose`

### Validation errors unclear

**Problem**: Error messages are hard to understand

**Solution**:
- Check the JSON path shown in the error
- Review the "Found value" vs schema expectations
- Use `--verbose` for additional logging
- Refer to the JSON schema for detailed constraints

## Integration Examples

### With Make

```makefile
validate:
    validate-yaml testcases/*.yml --schema schema.json

watch:
    validate-yaml testcases/*.yml --schema schema.json --watch
```

### With npm/package.json

```json
{
  "scripts": {
    "validate": "validate-yaml testcases/*.yml --schema schema.json",
    "validate:watch": "validate-yaml testcases/*.yml --schema schema.json --watch"
  }
}
```

### With Git Hooks

```bash
#!/bin/bash
# .git/hooks/pre-commit

echo "Validating YAML files..."
validate-yaml testcases/*.yml --schema schema.json

if [ $? -ne 0 ]; then
    echo "Validation failed. Commit aborted."
    exit 1
fi
```

## Related Commands

- `validate-json` - Validate JSON files against schemas
- `editor validate` - the tools's built-in validation
- `./scripts/validate-files.sh` - Shell script for directory-wide validation

## See Also

- [docs/validation.md](validation.md) - Comprehensive validation documentation
- [README.md](../README.md) - Project overview
- [src/bin/validate-yaml.rs](../src/bin/validate-yaml.rs) - Source code
