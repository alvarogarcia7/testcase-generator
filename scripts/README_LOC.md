# Lines of Code (LOC) Statistics

This directory contains scripts for computing lines of code statistics for the project using `tokei` (also known as `loc`).

## Overview

The LOC statistics feature provides comprehensive code metrics for the project, breaking down lines of code by:
- **Rust** (*.rs) - Core application code
- **Python** (*.py) - CI/CD and utility scripts
- **Shell Scripts** (*.sh) - Build, test, and deployment scripts
- **Markdown** (*.md) - Documentation files
- **YAML** (*.yml, *.yaml) - Configuration and test case files

## Installation

### Install tokei/loc

```bash
make install-loc
```

This will install `tokei` (the lines of code counter) to `~/.cargo/bin`.

**Manual Installation:**
```bash
cargo install tokei --locked
```

## Usage

### Basic Statistics

Compute overall lines of code statistics:
```bash
make loc
```

This displays:
- Overall project statistics
- Rust code statistics
- Python code statistics
- Shell script statistics
- Documentation statistics (Markdown)
- YAML files statistics
- Summary by language category

### Verbose Output

Get detailed statistics with file-by-file breakdown:
```bash
make loc-verbose
```

### JSON Output

Generate statistics in JSON format (useful for CI/CD):
```bash
make loc-json
```

### YAML Output

Generate statistics in YAML format:
```bash
make loc-yaml
```

### Generate Report File

Save statistics to a report file:
```bash
make loc-report
```

This creates: `reports/loc/loc_statistics.txt`

### Advanced Usage

The underlying script (`scripts/compute-loc.sh`) supports additional options:

```bash
# Custom output file
./scripts/compute-loc.sh --output my_report.txt

# JSON format with custom output
./scripts/compute-loc.sh --format json --output stats.json

# Verbose mode
./scripts/compute-loc.sh --verbose

# Help
./scripts/compute-loc.sh --help
```

## Output Format

### Text Output (Default)

The default output includes:

1. **Overall Project Statistics** - Total lines across all languages
2. **Language-Specific Sections** - Breakdown by Rust, Python, Shell, Markdown, YAML
3. **Summary by Category** - Top languages sorted by code volume

Example:
```
===============================================================================
 Overall Project Statistics
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
-------------------------------------------------------------------------------
 Rust                   67        15432        12345         1234         1853
 Python                 15         2345         1876          234          235
 Shell                  85         5678         4567          456          655
 Markdown              125        12345        10234            0         2111
 YAML                   45         3456         3200           56          200
-------------------------------------------------------------------------------
 Total                 337        39256        32222         1980         5054
===============================================================================
```

### JSON Output

Structured JSON data suitable for programmatic processing:
```json
{
  "Rust": {
    "blanks": 1853,
    "code": 12345,
    "comments": 1234,
    "files": 67
  },
  ...
}
```

### YAML Output

Structured YAML data:
```yaml
Rust:
  blanks: 1853
  code: 12345
  comments: 1234
  files: 67
...
```

## Integration with CI/CD

You can integrate LOC statistics into your CI/CD pipeline:

```yaml
# GitLab CI example
code-metrics:
  script:
    - make install-loc
    - make loc-json > metrics.json
  artifacts:
    reports:
      metrics: metrics.json
```

## Tracked Languages

The scripts specifically track these language categories:

- **Rust** - Main application language
- **Python** - Automation and tooling scripts
- **Shell** - Build, test, and deployment scripts
- **Markdown** - Project documentation
- **YAML** - Configuration and test definitions

## Tools

### install-loc.sh

Installs the `tokei` lines of code counter.

**Options:**
- `--local` - Install to `~/.cargo/bin` (default)
- `--global` - Install globally (requires sudo)
- `--help` - Show help message

**Features:**
- Checks for existing installation
- Prompts before reinstalling
- Verifies installation success
- Shows version and location

### compute-loc.sh

Computes lines of code statistics for the project.

**Options:**
- `--output FILE` - Save output to file
- `--verbose` - Show verbose output with file details
- `--format FORMAT` - Output format (text, json, yaml)
- `--help` - Show help message

**Features:**
- Overall project statistics
- Per-language breakdowns
- Sorted summary by code volume
- Multiple output formats
- Automatic tool detection (tokei/loc)

## Benefits

- **Project Metrics** - Track codebase size and composition
- **Documentation Coverage** - Measure documentation vs. code ratio
- **Language Distribution** - Understand project language mix
- **Trend Analysis** - Monitor codebase growth over time
- **CI/CD Integration** - Automated metrics collection
- **Fast and Accurate** - Uses `tokei`, a high-performance Rust tool

## Troubleshooting

### tokei not found after installation

Ensure `~/.cargo/bin` is in your PATH:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Add to your shell profile (`.bashrc`, `.zshrc`, etc.):
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Installation fails

Check that Rust and Cargo are installed:
```bash
rustc --version
cargo --version
```

If not installed, get Rust from: https://rustup.rs/

### Permission denied

Make sure the scripts are executable:
```bash
chmod +x scripts/install-loc.sh scripts/compute-loc.sh
```

## References

- **tokei**: https://github.com/XAMPPRocky/tokei
- **Documentation**: Run `tokei --help` for detailed options
