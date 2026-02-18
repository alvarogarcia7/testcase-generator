# JSON Escaping Configuration Guide

## Table of Contents

1. [Overview](#overview)
2. [The json-escape Binary](#the-json-escape-binary)
3. [Configuration Options](#configuration-options)
4. [Method Selection Guide](#method-selection-guide)
5. [Script Generation Behavior](#script-generation-behavior)
6. [Migration from Python/Perl](#migration-from-pythonperl)
7. [Installation and PATH Setup](#installation-and-path-setup)
8. [Troubleshooting](#troubleshooting)

## Overview

JSON escaping is a critical component of the test case manager's script generation system. When generating bash scripts from YAML test case definitions, the system needs to safely handle special characters, quotes, control characters, and newlines in command outputs that will be written to JSON execution log files.

The test case manager provides three methods for JSON escaping:
- **Rust Binary** (`rust_binary`): Use the compiled `json-escape` binary (fastest, most reliable)
- **Shell Fallback** (`shell_fallback`): Pure shell implementation using `sed` and `awk` (portable, no dependencies)
- **Auto Detection** (`auto`): Automatically detect and use the best available method (recommended)

## The json-escape Binary

### Purpose

The `json-escape` binary is a high-performance Rust utility that reads text from stdin and performs proper JSON string escaping according to the JSON specification (RFC 8259). It ensures that special characters, control characters, quotes, and newlines are correctly escaped so the output can be safely embedded in JSON strings.

### Key Features

- **Standards-compliant**: Implements JSON string escaping per RFC 8259
- **Fast and reliable**: Written in Rust for optimal performance and memory safety
- **Control character handling**: Properly escapes all control characters including `\n`, `\r`, `\t`, `\b`, `\f`, and others
- **Unicode support**: Handles Unicode characters correctly with `\uXXXX` encoding for control characters
- **Test mode**: Built-in validation to verify output is valid JSON
- **Verbose logging**: Optional detailed logging for debugging

### Usage

#### Basic Usage

```bash
# Escape text from stdin
echo "Hello \"World\"" | json-escape
# Output: Hello \"World\"

# Escape multi-line text
printf "Line 1\nLine 2\nLine 3" | json-escape
# Output: Line 1\nLine 2\nLine 3
```

#### Test Mode

Validate that escaped output is valid JSON:

```bash
echo "Test with quotes and newlines" | json-escape --test
# Validates and outputs escaped string
```

#### Verbose Mode

Enable detailed logging:

```bash
echo "Debug this" | json-escape --verbose
# Shows INFO logs about reading, escaping, and output
```

### Building the Binary

```bash
# Build in debug mode
make build-json-escape

# Build in release mode (optimized)
cargo build --release --bin json-escape

# Run directly
make run-json-escape
```

### Command-Line Options

- `-t, --test`: Test mode - validate that escaped output is valid JSON when wrapped in quotes
- `-v, --verbose`: Enable verbose logging (shows INFO-level logs)
- `-h, --help`: Display help information
- `-V, --version`: Display version information

### Escaping Behavior

The `json-escape` binary implements these escape rules:

| Input Character | Output Sequence | Description |
|----------------|-----------------|-------------|
| `"` (quote) | `\"` | Escaped quote |
| `\` (backslash) | `\\` | Escaped backslash |
| `\n` (newline) | `\n` | Newline escape sequence |
| `\r` (carriage return) | `\r` | Carriage return escape sequence |
| `\t` (tab) | `\t` | Tab escape sequence |
| `\b` (backspace) | `\b` | Backspace escape sequence |
| `\f` (form feed) | `\f` | Form feed escape sequence |
| Control characters (0x00-0x1F) | `\uXXXX` | Unicode escape sequence |
| All other characters | Unchanged | Preserved as-is |

### Integration with Test Executor

When the `test-executor` binary generates bash scripts from YAML test cases, it uses `json-escape` to ensure command outputs are properly escaped before writing to JSON log files:

```bash
# Generated script fragment
COMMAND_OUTPUT=$(some_command)
OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
echo "  {\"test_sequence\": 1, \"step\": 1, \"output\": \"$OUTPUT_ESCAPED\", ...}" >> log.json
```

## Configuration Options

JSON escaping configuration is stored in `~/.testcase-manager/config.toml` under the `[script_generation.json_escaping]` section.

### Complete Configuration Schema

```toml
[script_generation.json_escaping]
# Method selection (required)
# Options: "rust_binary", "shell_fallback", "auto"
# Default: "auto"
method = "auto"

# Enable/disable JSON escaping (optional)
# Set to false to disable escaping entirely (not recommended)
# Default: true
enabled = true

# Custom binary path (optional)
# Specify full path to json-escape binary
# Default: null (auto-detect from standard locations)
binary_path = "/usr/local/bin/json-escape"
```

### Configuration Field Reference

#### `method` (string, required)

Controls which JSON escaping implementation to use.

**Values:**
- `"rust_binary"` - Always use the `json-escape` binary (fails if not found)
- `"shell_fallback"` - Always use shell-based escaping (`sed`/`awk`)
- `"auto"` - Detect `json-escape` binary and fall back to shell if not found (recommended)

**Default:** `"auto"`

#### `enabled` (boolean, optional)

Master switch for JSON escaping functionality.

**Values:**
- `true` - Enable JSON escaping (recommended)
- `false` - Disable JSON escaping entirely (may cause malformed JSON logs)

**Default:** `true`

**Warning:** Disabling JSON escaping can result in invalid JSON log files when command outputs contain special characters, quotes, or newlines.

#### `binary_path` (string or null, optional)

Full filesystem path to the `json-escape` binary.

**When to use:**
- Binary installed in non-standard location
- Using a specific version of the binary
- Running in containerized or sandboxed environments
- Binary not in system PATH

**Default:** `null` (auto-detection enabled)

**Auto-detection search order:**
1. `./target/release/json-escape` (local release build)
2. `./target/debug/json-escape` (local debug build)
3. System PATH (uses `command -v json-escape`)

**Examples:**
```toml
# Absolute path
binary_path = "/opt/testcase-manager/bin/json-escape"

# Home directory path
binary_path = "~/.local/bin/json-escape"

# Relative path (from project root)
binary_path = "./bin/json-escape"

# Docker mount path
binary_path = "/app/bin/json-escape"
```

## Method Selection Guide

### When to Use Each Method

#### `auto` (Recommended)

**Use when:**
- You want the best available method automatically
- The binary may or may not be installed
- You're distributing scripts to multiple environments
- You want maximum portability with performance when possible

**Behavior:**
1. Checks if `json-escape` binary is available
2. Uses binary if found (fast, reliable)
3. Falls back to shell implementation if not found
4. Always generates working scripts

**Generated script example:**
```bash
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g; s/\r/\\r/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi
```

**Pros:**
- Maximum portability
- Performance when binary available
- No hard dependency on binary
- No script failures

**Cons:**
- Larger generated scripts (includes both paths)
- Runtime detection overhead

#### `rust_binary` (Best Performance)

**Use when:**
- You control the execution environment
- The binary is always installed
- You want maximum performance
- You need consistent escaping behavior
- You're running in CI/CD with controlled images

**Behavior:**
- Always uses the `json-escape` binary
- Fails if binary not found or not in PATH
- Smallest generated scripts
- Fastest execution

**Generated script example:**
```bash
OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
```

**Pros:**
- Fastest execution
- Smallest script size
- Most reliable escaping
- Explicit dependency

**Cons:**
- Requires binary installation
- Script fails if binary missing
- Less portable

#### `shell_fallback` (Maximum Portability)

**Use when:**
- You cannot install the binary
- You need scripts that work anywhere
- You're running on restricted systems
- You prioritize portability over performance
- The binary is unavailable or prohibited

**Behavior:**
- Always uses `sed` and `awk` for escaping
- Works on any POSIX-compatible system
- BSD and GNU `sed`/`awk` compatible
- No external dependencies

**Generated script example:**
```bash
# Shell fallback: escape backslashes, quotes, tabs, carriage returns, and convert newlines to \n
OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g; s/\r/\\r/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
```

**Pros:**
- Works everywhere (POSIX-compatible systems)
- No binary dependency
- BSD/GNU compatible
- Maximum portability

**Cons:**
- Slower than binary
- Less comprehensive escaping (doesn't handle all control characters)
- Longer script generation time

### Comparison Matrix

| Feature | `auto` | `rust_binary` | `shell_fallback` |
|---------|--------|---------------|------------------|
| **Performance** | High (if binary found) | Highest | Medium |
| **Portability** | Highest | Lowest | High |
| **Dependencies** | Optional binary | Required binary | None |
| **Script Size** | Larger | Smallest | Medium |
| **Escaping Completeness** | Depends | Most complete | Basic |
| **Failure Handling** | Graceful fallback | Hard failure | Always works |
| **Best For** | General use | Controlled environments | Restricted systems |

### Configuration Examples

#### Example 1: Development Environment (Recommended)

```toml
[script_generation.json_escaping]
method = "auto"
enabled = true
# binary_path not set - uses auto-detection
```

**Rationale:** During development, the binary may be in `target/debug` or `target/release`. Auto-detection finds it automatically and falls back to shell if you're testing on a clean environment.

#### Example 2: Production CI/CD

```toml
[script_generation.json_escaping]
method = "rust_binary"
enabled = true
binary_path = "/usr/local/bin/json-escape"
```

**Rationale:** In CI/CD, you control the Docker image or runner. Install the binary in a known location and use it directly for best performance.

#### Example 3: Distributed Scripts

```toml
[script_generation.json_escaping]
method = "shell_fallback"
enabled = true
```

**Rationale:** When distributing scripts to customers or external users who may not have the binary, use shell fallback for guaranteed compatibility.

#### Example 4: Docker Container

```toml
[script_generation.json_escaping]
method = "rust_binary"
enabled = true
binary_path = "/app/bin/json-escape"
```

**Rationale:** In a Docker container, the binary is built into the image at a known path. Use the binary directly.

#### Example 5: Sandboxed Environment

```toml
[script_generation.json_escaping]
method = "shell_fallback"
enabled = true
```

**Rationale:** In restricted or sandboxed environments where you cannot install binaries, shell fallback ensures scripts still work.

## Script Generation Behavior

### How JSON Escaping Fits Into Script Generation

When `test-executor` generates bash scripts from YAML test cases, it includes JSON escaping code to handle command outputs that will be written to JSON execution log files.

### Script Generation Workflow

1. **Parse YAML test case** - Load test sequences and steps
2. **Generate script header** - Set up bash script with error handling
3. **For each test step:**
   - Execute command and capture output
   - **Apply JSON escaping** to command output
   - Verify results using verification expressions
   - Write escaped output to JSON log file
4. **Generate script footer** - Clean up and exit

### Generated Script Structure

```bash
#!/usr/bin/env bash
set -euo pipefail

# Initialize JSON log file
echo "[" > execution_log.json

# Sequence 1, Step 1
echo "=== Test Sequence: 1, Step: 1 ===" | tee -a "$LOG_FILE"
COMMAND_OUTPUT=$(echo "Test command")
EXIT_CODE=$?

# Escape output for JSON (method depends on configuration)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g; s/\r/\\r/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi

# Verify results
RESULT_VERIFIED=false
OUTPUT_VERIFIED=false
if [ $EXIT_CODE -eq 0 ]; then
    RESULT_VERIFIED=true
fi
if [ "$COMMAND_OUTPUT" = "expected" ]; then
    OUTPUT_VERIFIED=true
fi

# Write to JSON log
echo "  {\"test_sequence\": 1, \"step\": 1, \"command\": \"echo \\\"Test command\\\"\", \"exit_code\": $EXIT_CODE, \"output\": \"$OUTPUT_ESCAPED\", \"timestamp\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\"}," >> execution_log.json

# More steps...

# Finalize JSON log
echo "]" >> execution_log.json
```

### Method-Specific Generation

#### With `method = "rust_binary"`

```bash
# Escape output for JSON (BSD/GNU compatible)
OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
```

- Direct binary invocation
- No detection overhead
- Fails gracefully with empty string on error

#### With `method = "shell_fallback"`

```bash
# Escape output for JSON (BSD/GNU compatible)
# Shell fallback: escape backslashes, quotes, tabs, carriage returns, and convert newlines to \n
OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g; s/\r/\\r/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
```

- Pure shell implementation
- Works on BSD and GNU systems
- Compatible with bash 3.2+

#### With `method = "auto"`

```bash
# Escape output for JSON (BSD/GNU compatible)
if command -v json-escape >/dev/null 2>&1; then
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
else
    # Shell fallback: escape backslashes, quotes, tabs, carriage returns, and convert newlines to \n
    OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g; s/\r/\\r/g' | awk '{printf "%s%s", (NR>1?"\\n":""), $0}')
fi
```

- Runtime detection with `command -v`
- Binary preferred, shell fallback
- Ensures script always works

### When Escaping is Disabled

If `enabled = false`, the raw output is used directly:

```bash
# JSON escaping disabled
OUTPUT_ESCAPED="$COMMAND_OUTPUT"
```

**Warning:** This can produce invalid JSON if the output contains:
- Unescaped quotes (`"`)
- Unescaped backslashes (`\`)
- Literal newlines
- Control characters

## Migration from Python/Perl

If you previously used Python or Perl for JSON escaping in test scripts, the `json-escape` binary provides a drop-in replacement with better performance and no runtime dependencies.

### Python Migration

#### Before (Python)

```bash
OUTPUT_ESCAPED=$(echo "$COMMAND_OUTPUT" | python3 -c 'import sys, json; print(json.dumps(sys.stdin.read())[1:-1])')
```

**Issues:**
- Requires Python 3 installation
- Slower due to interpreter startup
- Additional system dependency

#### After (json-escape)

```bash
OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
```

**Benefits:**
- No Python dependency
- Faster execution (native binary)
- Smaller memory footprint
- Fail-safe with fallback

### Perl Migration

#### Before (Perl)

```bash
OUTPUT_ESCAPED=$(echo "$COMMAND_OUTPUT" | perl -pe 's/\\/\\\\/g; s/"/\\"/g; s/\n/\\n/g; s/\r/\\r/g; s/\t/\\t/g')
```

**Issues:**
- Requires Perl installation
- Not available on all systems
- Regex complexity

#### After (json-escape)

```bash
OUTPUT_ESCAPED=$(printf '%s' "$COMMAND_OUTPUT" | json-escape 2>/dev/null || echo "")
```

**Benefits:**
- No Perl dependency
- More comprehensive escaping
- Standards-compliant
- Simpler command

### Migration Checklist

- [ ] Build and install `json-escape` binary
- [ ] Update configuration to use `method = "auto"` or `method = "rust_binary"`
- [ ] Test generated scripts with edge cases (quotes, newlines, control characters)
- [ ] Remove Python/Perl dependencies from deployment documentation
- [ ] Update CI/CD pipelines to include `json-escape` binary
- [ ] Verify JSON log files are valid after migration

## Installation and PATH Setup

### Building from Source

#### Prerequisites

- Rust toolchain (1.70.0 or later)
- Cargo package manager

#### Build Commands

```bash
# Clone the repository
git clone <repository-url>
cd testcase-manager

# Build all binaries (includes json-escape)
make build

# Build json-escape specifically
make build-json-escape

# Build optimized release version
cargo build --release --bin json-escape
```

#### Binary Locations

- Debug build: `./target/debug/json-escape`
- Release build: `./target/release/json-escape`

### Installation Options

#### Option 1: Local Installation (User)

Install to user's local bin directory:

```bash
# Build release binary
cargo build --release --bin json-escape

# Create local bin directory if it doesn't exist
mkdir -p ~/.local/bin

# Copy binary
cp target/release/json-escape ~/.local/bin/

# Add to PATH (add to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.local/bin:$PATH"

# Verify installation
json-escape --version
```

#### Option 2: System Installation (Root)

Install system-wide (requires sudo):

```bash
# Build release binary
cargo build --release --bin json-escape

# Install to /usr/local/bin
sudo cp target/release/json-escape /usr/local/bin/

# Verify installation
json-escape --version
```

#### Option 3: Custom Installation Path

Install to a custom location:

```bash
# Build release binary
cargo build --release --bin json-escape

# Create custom directory
mkdir -p /opt/testcase-manager/bin

# Copy binary
cp target/release/json-escape /opt/testcase-manager/bin/

# Configure in config.toml
cat >> ~/.testcase-manager/config.toml << EOF
[script_generation.json_escaping]
method = "rust_binary"
binary_path = "/opt/testcase-manager/bin/json-escape"
EOF
```

#### Option 4: Docker Installation

Install in Docker image:

```dockerfile
# Dockerfile
FROM rust:1.70 as builder
WORKDIR /build
COPY . .
RUN cargo build --release --bin json-escape

FROM ubuntu:22.04
COPY --from=builder /build/target/release/json-escape /usr/local/bin/
RUN json-escape --version
```

### PATH Configuration

#### Bash

Add to `~/.bashrc`:

```bash
# Add json-escape to PATH
export PATH="$HOME/.local/bin:$PATH"
```

Reload configuration:

```bash
source ~/.bashrc
```

#### Zsh

Add to `~/.zshrc`:

```zsh
# Add json-escape to PATH
export PATH="$HOME/.local/bin:$PATH"
```

Reload configuration:

```zsh
source ~/.zshrc
```

#### Fish

Add to `~/.config/fish/config.fish`:

```fish
# Add json-escape to PATH
set -gx PATH $HOME/.local/bin $PATH
```

Reload configuration:

```fish
source ~/.config/fish/config.fish
```

### Verification

Verify the installation:

```bash
# Check if binary is in PATH
command -v json-escape

# Display version
json-escape --version

# Test basic functionality
echo "Hello \"World\"" | json-escape

# Test with --test flag
echo "Test output" | json-escape --test
```

## Troubleshooting

### Binary Not Found

#### Symptoms

```
bash: json-escape: command not found
```

Or generated scripts fail with:

```
ERROR: json-escape binary not found
```

#### Solutions

1. **Verify binary exists:**
   ```bash
   ls -la target/release/json-escape
   ```

2. **Check PATH:**
   ```bash
   echo $PATH
   which json-escape
   ```

3. **Add to PATH:**
   ```bash
   export PATH="$HOME/.local/bin:$PATH"
   ```

4. **Configure explicit path in config.toml:**
   ```toml
   [script_generation.json_escaping]
   binary_path = "/full/path/to/json-escape"
   ```

5. **Use fallback method:**
   ```toml
   [script_generation.json_escaping]
   method = "auto"  # or "shell_fallback"
   ```

### Binary Detection Fails in Auto Mode

#### Symptoms

Scripts use shell fallback even though binary is installed.

#### Diagnosis

```bash
# Test if shell can find binary
command -v json-escape

# Test binary execution
json-escape --version

# Check file permissions
ls -la $(which json-escape)
```

#### Solutions

1. **Ensure binary is executable:**
   ```bash
   chmod +x ~/.local/bin/json-escape
   ```

2. **Verify PATH in script environment:**
   ```bash
   # Add to generated script for debugging
   echo $PATH
   command -v json-escape
   ```

3. **Use explicit binary path:**
   ```toml
   [script_generation.json_escaping]
   method = "rust_binary"
   binary_path = "/full/path/to/json-escape"
   ```

### Invalid JSON Output

#### Symptoms

```
Error: invalid JSON in execution log
```

Or JSON parsers fail on log files.

#### Diagnosis

1. **Check generated log file:**
   ```bash
   cat execution_log.json | jq .
   ```

2. **Verify escaping is enabled:**
   ```bash
   grep "enabled = true" ~/.testcase-manager/config.toml
   ```

3. **Test escaping directly:**
   ```bash
   echo 'Test "with" quotes' | json-escape
   ```

#### Solutions

1. **Enable escaping if disabled:**
   ```toml
   [script_generation.json_escaping]
   enabled = true
   ```

2. **Use binary method for more complete escaping:**
   ```toml
   [script_generation.json_escaping]
   method = "rust_binary"
   ```

3. **Test with edge cases:**
   ```bash
   printf 'Line1\nLine2\n"quotes"\ttabs' | json-escape --test
   ```

### Performance Issues

#### Symptoms

Slow script execution, especially with large outputs.

#### Solutions

1. **Use rust_binary method:**
   ```toml
   [script_generation.json_escaping]
   method = "rust_binary"
   ```

2. **Build release binary:**
   ```bash
   cargo build --release --bin json-escape
   ```

3. **Avoid shell_fallback for large outputs:**
   - Shell fallback uses `sed` and `awk` which are slower
   - Binary is significantly faster for large texts

### Permission Denied

#### Symptoms

```
bash: /path/to/json-escape: Permission denied
```

#### Solutions

1. **Make binary executable:**
   ```bash
   chmod +x /path/to/json-escape
   ```

2. **Check file ownership:**
   ```bash
   ls -la /path/to/json-escape
   chown $USER:$USER /path/to/json-escape
   ```

3. **Verify SELinux/AppArmor:**
   ```bash
   # Check SELinux context
   ls -Z /path/to/json-escape
   
   # Restore context if needed
   restorecon -v /path/to/json-escape
   ```

### Configuration Not Loaded

#### Symptoms

Scripts use default behavior instead of configured method.

#### Diagnosis

```bash
# Check config file exists
cat ~/.testcase-manager/config.toml

# Verify config syntax
cargo run --bin test-executor -- --help
```

#### Solutions

1. **Create config directory:**
   ```bash
   mkdir -p ~/.testcase-manager
   ```

2. **Create config file:**
   ```bash
   cat > ~/.testcase-manager/config.toml << EOF
   [script_generation.json_escaping]
   method = "auto"
   enabled = true
   EOF
   ```

3. **Verify TOML syntax:**
   - Ensure proper section headers `[script_generation.json_escaping]`
   - Use quotes for string values: `method = "auto"`
   - Use boolean literals: `enabled = true` (not `"true"`)

### Build Failures

#### Symptoms

```
error: could not compile `testcase-manager`
```

#### Solutions

1. **Update Rust toolchain:**
   ```bash
   rustup update
   ```

2. **Clean and rebuild:**
   ```bash
   cargo clean
   cargo build --bin json-escape
   ```

3. **Check minimum Rust version:**
   ```bash
   rustc --version  # Should be 1.70.0 or later
   ```

### Testing JSON Escaping

#### Manual Testing

```bash
# Test basic escaping
echo 'Hello "World"' | json-escape

# Test newlines
printf 'Line 1\nLine 2\nLine 3' | json-escape

# Test control characters
printf 'Tab\there\tBackspace\b\b' | json-escape

# Test with validation
echo 'Complex "test" with\nnewlines\tand\ttabs' | json-escape --test

# Test verbose mode
echo 'Debug output' | json-escape --verbose
```

#### Integration Testing

```bash
# Generate and execute a test script
test-executor generate tests/sample/test.yml --output /tmp/test.sh
bash /tmp/test.sh

# Verify JSON log is valid
cat execution_log.json | jq .

# Check specific fields
cat execution_log.json | jq '.[0].output'
```

## Additional Resources

- **Binary Source Code**: `src/bin/json-escape.rs`
- **Configuration Schema**: `src/config.rs`
- **Test Executor**: `src/executor.rs`
- **Example Configuration**: `examples/config/testcase-manager-config.toml`
- **AGENTS.md**: Project build and test instructions
- **README.md**: Project overview and usage guide

## Summary

The JSON escaping system provides flexible, high-performance JSON string escaping for generated test scripts:

- **Three methods**: `rust_binary`, `shell_fallback`, `auto`
- **Recommended method**: `auto` for development, `rust_binary` for production
- **Configuration file**: `~/.testcase-manager/config.toml`
- **Binary location**: `target/release/json-escape`
- **Auto-detection**: Searches local builds and system PATH
- **Fallback**: Pure shell implementation works everywhere
- **Migration**: Drop-in replacement for Python/Perl implementations

For most users, the default `auto` method provides the best balance of performance and portability.
