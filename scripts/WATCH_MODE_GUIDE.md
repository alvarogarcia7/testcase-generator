# Watch Mode Guide for validate-files.sh

## Overview

Watch mode enables continuous monitoring of a directory for file changes, with automatic validation triggered on modified files. This is ideal for development workflows where you want instant feedback on file changes.

## Quick Start

### 1. Install Platform-Specific Tools

**Linux:**
```bash
sudo apt-get install inotify-tools
```

**macOS:**
```bash
brew install fswatch
```

### 2. Run Watch Mode

Basic usage (monitors `testcases/` directory):
```bash
./scripts/validate-files.sh \
    --pattern '\.ya?ml$' \
    --validator ./scripts/validate-yaml-wrapper.sh \
    --watch
```

Custom directory:
```bash
./scripts/validate-files.sh \
    --pattern '\.ya?ml$' \
    --validator ./scripts/validate-yaml-wrapper.sh \
    --watch path/to/custom/dir/
```

With verbose logging:
```bash
./scripts/validate-files.sh \
    --pattern '\.ya?ml$' \
    --validator ./scripts/validate-yaml-wrapper.sh \
    --watch \
    --verbose
```

## Features

### Initial Validation
When watch mode starts, it performs a complete validation of all files matching the pattern. This provides a baseline validation status before monitoring begins.

### Real-Time Monitoring
The script monitors the specified directory recursively for:
- File modifications (`modify`)
- New file creation (`create`)
- File deletion (`delete`)
- File moves/renames (`move`)

### Instant Validation
When a file change is detected:
1. The script checks if the file matches the specified regex pattern
2. If it matches, validation is triggered immediately (with a small delay to ensure the file is fully written)
3. Results are displayed in real-time with color-coded output:
   - **Green** (✓ PASSED): File validation successful
   - **Red** (✗ FAILED): File validation failed (with error details)

### Persistent Cache
The validation cache persists across watch sessions, providing:
- Fast re-validation using two-layer caching (mtime + content hash)
- Efficient skipping of unchanged files
- Automatic cache updates when files change
- Cache cleanup when files are deleted

### Pattern Matching
Only files matching the specified regex pattern are validated. This allows you to:
- Watch a directory containing multiple file types
- Only validate specific file extensions (e.g., `\.ya?ml$` for YAML files)
- Use complex patterns for fine-grained control

## Usage Examples

### Example 1: YAML Validation with Schema
```bash
# Set the schema file
export SCHEMA_FILE=data/schema.json

# Start watch mode for YAML files
./scripts/validate-files.sh \
    --pattern '\.ya?ml$' \
    --validator ./scripts/validate-yaml-wrapper.sh \
    --watch testcases/
```

### Example 2: Custom Validator
```bash
# Watch JSON files with a custom validator
./scripts/validate-files.sh \
    --pattern '\.json$' \
    --validator ./scripts/my-json-validator.sh \
    --watch data/
```

### Example 3: Multiple File Types
```bash
# Watch for both YAML and YML files
./scripts/validate-files.sh \
    --pattern '\.(yaml|yml)$' \
    --validator ./scripts/validate-yaml-wrapper.sh \
    --watch
```

### Example 4: Custom Cache Directory
```bash
# Use a custom cache directory
./scripts/validate-files.sh \
    --pattern '\.ya?ml$' \
    --validator ./scripts/validate-yaml-wrapper.sh \
    --cache-dir /tmp/my-cache \
    --watch
```

## How It Works

### Linux (inotifywait)
```
inotifywait -m -r -e modify,create,delete,move --format '%w%f' <directory>
```
- Monitors directory recursively (`-r`)
- Watches for specific events (`-e modify,create,delete,move`)
- Outputs full file path (`--format '%w%f'`)
- Continuous monitoring mode (`-m`)

### macOS (fswatch)
```
fswatch -0 -r -e ".*" -i <pattern> <directory>
```
- Monitors directory recursively (`-r`)
- Uses null-separated output (`-0`)
- Excludes all files first (`-e ".*"`)
- Then includes only files matching pattern (`-i <pattern>`)

## Workflow Integration

### Development Workflow
1. Start watch mode in a terminal window
2. Edit files in your editor
3. Save changes
4. Get instant validation feedback in the watch terminal
5. Fix any errors and save again
6. Repeat until all validations pass

### Continuous Integration
While watch mode is primarily for development, the same validation script can be used in CI:
```bash
# CI: Run one-time validation (no watch mode)
./scripts/validate-files.sh \
    --pattern '\.ya?ml$' \
    --validator ./scripts/validate-yaml-wrapper.sh
```

## Troubleshooting

### Tool Not Found
**Error:** `inotifywait not found` or `fswatch not found`

**Solution:** Install the required tool for your platform:
- Linux: `sudo apt-get install inotify-tools`
- macOS: `brew install fswatch`

### Directory Not Found
**Error:** `Watch directory does not exist: <path>`

**Solution:** Verify the directory path exists:
```bash
ls -la testcases/  # or your custom directory
```

### Pattern Not Matching
**Issue:** Files are changing but not being validated

**Solution:** Test your regex pattern:
```bash
# Test pattern matching
echo "testfile.yaml" | grep -E '\.ya?ml$'
echo "testfile.yml" | grep -E '\.ya?ml$'
```

### Validation Errors
**Issue:** Files fail validation unexpectedly

**Solution:** Run the validator directly to see detailed errors:
```bash
./scripts/validate-yaml-wrapper.sh path/to/file.yaml
```

## Performance Considerations

### Cache Hit Rate
Watch mode uses the same caching system as normal mode:
- First validation: Full hash calculation and validation
- Subsequent validations: Fast mtime check or hash-based verification
- Cache persists across watch sessions

### File System Load
- Modern file watchers (inotify, fswatch) are efficient and low-overhead
- The script includes a small delay (0.1s) after file changes to ensure files are fully written
- Only files matching the pattern trigger validation

### Large Directories
For very large directories:
- Consider using more specific patterns to reduce the number of files monitored
- Use custom watch directories to focus on specific subdirectories
- The recursive monitoring is efficient but may impact performance on massive file trees

## Exit and Cleanup

### Stop Watch Mode
Press `Ctrl+C` to stop watch mode gracefully.

### Cache Cleanup
The validation cache directory (`.validation-cache/`) persists after exit:
- Allows fast re-validation on next run
- Can be safely deleted if needed: `rm -rf .validation-cache/`
- Automatically cleaned up for deleted files during watch mode

### Manual Cache Clear
To start with a fresh cache:
```bash
rm -rf .validation-cache/
./scripts/validate-files.sh --pattern '...' --validator '...' --watch
```

## Advanced Usage

### Combining with Other Tools

#### With tmux/screen
```bash
# Start watch mode in a detached session
tmux new-session -d -s validation './scripts/validate-files.sh --pattern "\.ya?ml$" --validator ./scripts/validate-yaml-wrapper.sh --watch'

# Attach to see validation output
tmux attach -t validation
```

#### With Make
```makefile
# Makefile target
.PHONY: watch
watch:
	./scripts/validate-files.sh \
		--pattern '\.ya?ml$' \
		--validator ./scripts/validate-yaml-wrapper.sh \
		--watch testcases/
```

Then run: `make watch`

### Custom Validators
Create custom validators for different file types:

```bash
#!/usr/bin/env bash
# my-custom-validator.sh

FILE="$1"

# Your custom validation logic
if [[ ! -f "$FILE" ]]; then
    echo "File not found: $FILE"
    exit 1
fi

# Example: Check file size
SIZE=$(stat -f%z "$FILE" 2>/dev/null || stat -c%s "$FILE")
if [[ $SIZE -gt 1000000 ]]; then
    echo "File too large: $FILE ($SIZE bytes)"
    exit 1
fi

# Example: Run a linter
my_linter "$FILE" || exit 1

exit 0
```

Then use with watch mode:
```bash
./scripts/validate-files.sh \
    --pattern '\.txt$' \
    --validator ./my-custom-validator.sh \
    --watch
```
