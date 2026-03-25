# Workspace Structure

This project uses a Cargo workspace to organize multiple related crates.

## Directory Structure

```
.
├── Cargo.toml                 # Workspace root configuration
├── crates/
│   ├── bash-eval/             # Bash evaluation and shell script generation library
│   │   ├── Cargo.toml
│   │   └── src/
│   └── testcase-manager/      # Main test case management application
│       ├── Cargo.toml
│       ├── src/
│       │   ├── bin/           # Binary executables
│       │   └── lib.rs         # Library code
│       ├── examples/          # Example code
│       └── tests/             # Integration tests
├── testcases/                 # Test case YAML files (project root)
├── schemas/                   # JSON schemas (project root)
├── scripts/                   # Build and utility scripts (project root)
└── data/                      # Sample data (project root)
```

## Workspace Members

### bash-eval

A library crate that provides bash script evaluation and generation functionality.

**Dependencies:**
- serde (workspace)
- anyhow (workspace)
- log (workspace)
- regex (workspace)

### testcase-manager

The main application crate containing all binaries, tests, and examples.

**Key Binaries:**
- `validate-yaml` - YAML validation against schemas
- `validate-json` - JSON validation
- `trm` - Test Run Manager
- `test-verify` - Test verification tool
- `test-executor` - Test execution engine
- `test-orchestrator` - Test orchestration
- `verifier` - Verification tool
- `script-cleanup` - Script cleanup utility
- `json-escape` - JSON escaping utility
- `test-plan-documentation-generator-compat` - Documentation generator compatibility layer
- `json-to-yaml` - JSON to YAML conversion

**Dependencies:** See `crates/testcase-manager/Cargo.toml`

## Workspace Dependencies

The root `Cargo.toml` defines shared dependencies in `[workspace.dependencies]`:

### Core Serialization
- `serde` - Serialization framework
- `serde_json` - JSON support
- `serde_yaml` - YAML support
- `serde_path_to_error` - Better error reporting

### CLI and UI
- `clap` - Command-line argument parsing
- `skim` - Fuzzy finder
- `dialoguer` - Interactive prompts
- `edit` - Text editor integration

### Error Handling
- `anyhow` - Flexible error handling
- `log` - Logging facade
- `env_logger` - Logger implementation
- `regex` - Regular expressions

### Date and Time
- `chrono` - Date and time handling

### Data Structures
- `indexmap` - Ordered map implementation

### Git Integration
- `git2` - Git repository operations

### Validation
- `jsonschema` - JSON schema validation

### Configuration
- `dotenvy` - Environment variable loading
- `toml` - TOML configuration parsing

### File System
- `tempfile` - Temporary file handling
- `notify` - File system watching
- `walkdir` - Directory traversal

### XML and Text Processing
- `roxmltree` - XML parsing
- `strip-ansi-escapes` - ANSI escape sequence handling

### HTTP (dev dependency)
- `reqwest` - HTTP client

## Building

```bash
# Build all workspace members
cargo build --all

# Build in release mode
cargo build --all --release

# Build specific crate
cargo build -p bash-eval
cargo build -p testcase-manager

# Build specific binary
cargo build --bin validate-yaml
```

## Testing

```bash
# Run all tests in workspace
cargo test --all

# Run tests for specific crate
cargo test -p bash-eval
cargo test -p testcase-manager

# Run integration tests
make test-e2e
```

## Coverage

```bash
# Run coverage analysis
make coverage

# Generate HTML coverage report
make coverage-html

# Run E2E tests with coverage
make coverage-e2e
```

## Migration Notes

The workspace structure was created by:

1. Creating `crates/` directory
2. Moving `bash-eval/` to `crates/bash-eval/`
3. Moving `src/`, `examples/`, and `tests/` to `crates/testcase-manager/`
4. Updating `Cargo.toml` files to use workspace dependencies
5. Updating build scripts and CI configuration

Common directories like `testcases/`, `schemas/`, `scripts/`, and `data/` remain at the project root for easy access.

## Benefits

- **Shared Dependencies**: Common dependencies defined once in workspace root
- **Consistent Versions**: All crates use the same dependency versions
- **Better Organization**: Clear separation between library and application code
- **Parallel Builds**: Cargo can build workspace members in parallel
- **Easier Maintenance**: Dependency updates managed centrally
