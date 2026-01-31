# Watch Mode Comparison

This document compares the two watch mode implementations available in the Test Case Manager project.

## Overview

There are two ways to enable watch mode for YAML file validation:

1. **validate-yaml --watch** - Built-in watch mode in the `validate-yaml` binary
2. **validate-files.sh --watch** - Shell script-based watch mode with caching

## Quick Comparison

| Feature | validate-yaml --watch | validate-files.sh --watch |
|---------|----------------------|---------------------------|
| **Implementation** | Rust binary with `notify` crate | Shell script with `inotify-tools`/`fswatch` |
| **Platform Support** | Linux, macOS | Linux, macOS |
| **Windows Support** | ❌ Disabled | ❌ Not available |
| **File Selection** | Explicit list on command line | Pattern matching in directory |
| **Monitoring Scope** | Specific files | Entire directory (recursive) |
| **Caching** | In-memory only | Persistent across sessions |
| **Debouncing** | 300ms window | Event-based |
| **Smart Re-validation** | ✅ Full validation when all pass | ❌ Individual file only |
| **Installation** | Cargo build | Requires external tools |
| **Performance** | Native Rust (fast) | Shell script (slower) |
| **Setup Complexity** | None | Requires `inotify-tools`/`fswatch` |

## Detailed Comparison

### 1. validate-yaml --watch

**Type**: Rust binary with built-in file watching

**Command**:
```bash
validate-yaml testcase1.yml testcase2.yml --schema schema.json --watch
```

**Advantages**:
- ✅ No external dependencies (other than Rust build)
- ✅ Fast native performance
- ✅ Smart re-validation (full validation when all changed files pass)
- ✅ Precise file monitoring (only specified files)
- ✅ Debounced event handling (avoids duplicate validations)
- ✅ Color-coded real-time output
- ✅ Cross-platform Rust implementation

**Disadvantages**:
- ❌ Must explicitly list all files on command line
- ❌ No persistent caching between sessions
- ❌ Not available on Windows
- ❌ Cannot watch entire directory with pattern matching

**Best For**:
- Monitoring specific known files during development
- CI/CD workflows with explicit file lists
- Projects where you want zero external dependencies
- When you need the "smart re-validation" feature

**Example Workflow**:
```bash
# Watch specific test cases
validate-yaml testcases/tc_001.yml testcases/tc_002.yml --schema schema.json --watch

# Watch using shell glob (expanded by shell)
validate-yaml testcases/*.yml --schema schema.json --watch
```

### 2. validate-files.sh --watch

**Type**: Shell script with external file watching tools

**Command**:
```bash
./scripts/validate-files.sh --pattern '\.ya?ml$' --validator ./scripts/validate-yaml-wrapper.sh --watch
```

**Advantages**:
- ✅ Watches entire directory recursively
- ✅ Pattern-based file matching (regex)
- ✅ Persistent validation cache across sessions
- ✅ Auto-detects new files matching pattern
- ✅ Cache cleanup for deleted files
- ✅ Flexible validator configuration

**Disadvantages**:
- ❌ Requires external tools (`inotify-tools` on Linux, `fswatch` on macOS)
- ❌ Slower shell script execution
- ❌ No smart re-validation (validates changed files only)
- ❌ More complex setup
- ❌ Not available on Windows

**Best For**:
- Monitoring entire directories for any YAML changes
- Projects with dynamic file creation/deletion
- When you need persistent caching
- Pattern-based file filtering
- Legacy workflows already using the script

**Example Workflow**:
```bash
# Watch all YAML files in testcases/ directory
./scripts/validate-files.sh \
    --pattern '\.ya?ml$' \
    --validator ./scripts/validate-yaml-wrapper.sh \
    --watch

# Watch custom directory
./scripts/validate-files.sh \
    --pattern '\.ya?ml$' \
    --validator ./scripts/validate-yaml-wrapper.sh \
    --watch custom/path/
```

## Feature Deep Dive

### File Selection

**validate-yaml --watch**:
- Explicit file list: `validate-yaml file1.yml file2.yml --schema schema.json --watch`
- Shell glob expansion: `validate-yaml testcases/*.yml --schema schema.json --watch`
- Only watches files specified on command line

**validate-files.sh --watch**:
- Pattern matching: `--pattern '\.ya?ml$'`
- Directory scanning: Monitors entire directory tree
- Automatically picks up new files matching pattern

### Smart Re-validation

**validate-yaml --watch** includes intelligent re-validation:

1. File change detected
2. Validate changed file(s)
3. Display results for changed files
4. **If all changed files pass** → Run full validation on ALL watched files
5. Display complete summary

This ensures that fixing one file doesn't mask issues in other files.

**validate-files.sh --watch** validates only the changed files without triggering full validation.

### Caching

**validate-yaml --watch**:
- In-memory validation results
- Cache cleared when process exits
- No disk persistence

**validate-files.sh --watch**:
- Persistent cache stored in `.validation-cache/`
- Cache survives across sessions
- Faster subsequent validations
- Auto-cleanup for deleted files

### Performance

**validate-yaml --watch**:
- Native Rust performance
- Direct file system event handling via `notify` crate
- Low memory footprint
- Instant validation

**validate-files.sh --watch**:
- Shell script overhead
- Spawns validation process per file
- Higher latency
- More disk I/O (cache files)

### Platform Support

Both implementations:
- ✅ Linux (full support)
- ✅ macOS (full support)
- ❌ Windows (not available)

**Windows Alternatives**:
1. Use WSL (Windows Subsystem for Linux)
2. Use PowerShell FileSystemWatcher
3. Use scheduled tasks for periodic validation
4. Use editor's built-in validation features

## Decision Guide

### Choose validate-yaml --watch when:

- ✅ You know exactly which files to monitor
- ✅ You want zero external dependencies
- ✅ You need the fastest performance
- ✅ You want smart re-validation behavior
- ✅ You're using it in CI/CD pipelines
- ✅ You prefer native binaries over shell scripts

### Choose validate-files.sh --watch when:

- ✅ You want to monitor an entire directory
- ✅ You need pattern-based file matching
- ✅ You want persistent caching
- ✅ Files are frequently created/deleted
- ✅ You already have `inotify-tools`/`fswatch` installed
- ✅ You're using existing scripts/workflows

## Migration Guide

### From validate-files.sh to validate-yaml

**Before**:
```bash
./scripts/validate-files.sh \
    --pattern '\.ya?ml$' \
    --validator ./scripts/validate-yaml-wrapper.sh \
    --watch testcases/
```

**After**:
```bash
# Option 1: Explicit file list
validate-yaml testcases/file1.yml testcases/file2.yml --schema schema.json --watch

# Option 2: Shell glob (files expanded by shell)
validate-yaml testcases/*.yml --schema schema.json --watch

# Option 3: Find + xargs for complex patterns
find testcases/ -name "*.yml" -o -name "*.yaml" | \
    xargs validate-yaml --schema schema.json --watch
```

### From validate-yaml to validate-files.sh

**Before**:
```bash
validate-yaml testcases/*.yml --schema schema.json --watch
```

**After**:
```bash
./scripts/validate-files.sh \
    --pattern '\.ya?ml$' \
    --validator ./scripts/validate-yaml-wrapper.sh \
    --watch testcases/
```

## Examples

### Scenario 1: Development Workflow

**Goal**: Monitor 3 specific test case files during development

**Solution**: Use `validate-yaml --watch`
```bash
validate-yaml \
    testcases/tc_001.yml \
    testcases/tc_002.yml \
    testcases/tc_003.yml \
    --schema schema.json \
    --watch
```

### Scenario 2: Full Directory Monitoring

**Goal**: Watch all YAML files in testcases/ directory, including new files

**Solution**: Use `validate-files.sh --watch`
```bash
./scripts/validate-files.sh \
    --pattern '\.ya?ml$' \
    --validator ./scripts/validate-yaml-wrapper.sh \
    --watch testcases/
```

### Scenario 3: CI/CD Pipeline

**Goal**: Validate all test cases on file change (no watch, just validation)

**Solution**: Use `validate-yaml` without watch
```bash
validate-yaml testcases/*.yml --schema schema.json
```

### Scenario 4: Multi-directory Monitoring

**Goal**: Monitor YAML files in multiple directories

**Solution with validate-yaml**:
```bash
validate-yaml testcases/*.yml examples/*.yml docs/*.yml --schema schema.json --watch
```

**Solution with validate-files.sh**:
```bash
# Watch each directory in separate terminal
./scripts/validate-files.sh --pattern '\.ya?ml$' --validator validate.sh --watch testcases/
./scripts/validate-files.sh --pattern '\.ya?ml$' --validator validate.sh --watch examples/
./scripts/validate-files.sh --pattern '\.ya?ml$' --validator validate.sh --watch docs/
```

## Recommendations

### For Most Users

**Recommended**: `validate-yaml --watch`
- Simpler to use
- Better performance
- No external dependencies
- Smart re-validation
- Modern Rust implementation

### For Legacy Projects

**Recommended**: Keep `validate-files.sh --watch` if:
- Already integrated into workflows
- Depend on persistent caching
- Need pattern-based directory monitoring
- Have complex file structures

### For Windows Users

**Recommended**: Use standard validation without watch
```bash
validate-yaml testcases/*.yml --schema schema.json
```

Or use WSL with either watch mode implementation.

## See Also

- [docs/VALIDATE_YAML_QUICK_REF.md](VALIDATE_YAML_QUICK_REF.md) - validate-yaml quick reference
- [docs/validation.md](validation.md) - Comprehensive validation documentation
- [scripts/WATCH_MODE_GUIDE.md](../scripts/WATCH_MODE_GUIDE.md) - validate-files.sh guide
- [README.md](../README.md) - Project overview
