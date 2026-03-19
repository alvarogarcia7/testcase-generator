# Lines of Code Statistics - Quick Reference

## Installation

```bash
make install-loc
```

## Basic Usage

```bash
# Compute statistics (default text output)
make loc

# Verbose output with file details
make loc-verbose

# JSON output
make loc-json

# YAML output
make loc-yaml

# Generate report file
make loc-report
```

## Output Location

Reports are saved to: `reports/loc/loc_statistics.txt`

## What's Measured

- **Rust** - Application code (*.rs)
- **Python** - Scripts and tooling (*.py)
- **Shell** - Build/test scripts (*.sh)
- **Markdown** - Documentation (*.md)
- **YAML** - Config and test cases (*.yml, *.yaml)

## Statistics Breakdown

Each language section shows:
- **Files** - Number of files
- **Lines** - Total lines (code + comments + blanks)
- **Code** - Lines of actual code
- **Comments** - Comment lines
- **Blanks** - Blank lines

## Advanced Usage

```bash
# Custom output file
./scripts/compute-loc.sh --output custom_report.txt

# JSON with custom file
./scripts/compute-loc.sh --format json --output stats.json

# Help
./scripts/compute-loc.sh --help
```

## Tool Information

- Uses **tokei** (https://github.com/XAMPPRocky/tokei)
- Fast, accurate Rust-based line counter
- Installed to `~/.cargo/bin/tokei`
- Also known as `loc` in some contexts

## Troubleshooting

### Command not found

Ensure `~/.cargo/bin` is in PATH:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### Installation fails

Check Rust is installed:
```bash
rustc --version
cargo --version
```

Install from: https://rustup.rs/

## See Also

- Full documentation: [scripts/README_LOC.md](../scripts/README_LOC.md)
- Tool help: `tokei --help`
