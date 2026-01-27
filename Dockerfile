# Stage 1: deps - Build dependencies only
FROM rust:1.92-bookworm as deps

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy src/main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    mkdir -p src/bin && \
    echo "fn main() {}" > src/bin/validate-yaml.rs

# Build dependencies (this will be cached)
RUN cargo build --release

# Stage 2: builder - Build the actual application
FROM rust:1.92-bookworm as builder

WORKDIR /app

# Copy dependencies from deps stage
COPY --from=deps /app/target target
COPY --from=deps /usr/local/cargo /usr/local/cargo

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy full source
COPY src ./src
COPY examples ./examples
COPY tests ./tests

# Build the application against cached dependencies
RUN cargo build --release

# Stage 3: runtime - Final lightweight image
FROM debian:bookworm-slim as runtime

# Install runtime dependencies: git
RUN apt-get update && \
    apt-get install -y git && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy only the compiled binaries (not auxiliary build files)
COPY --from=builder /app/target/release/testcase-manager /usr/local/bin/tcm
COPY --from=builder /app/target/release/validate-yaml /usr/local/bin/
COPY --from=builder /app/target/release/validate-json /usr/local/bin/
COPY --from=builder /app/target/release/trm /usr/local/bin/
COPY --from=builder /app/target/release/test-verify /usr/local/bin/
COPY --from=builder /app/target/release/test-executor /usr/local/bin/
COPY --from=builder /app/target/release/editor /usr/local/bin/
COPY --from=builder /app/target/release/test-orchestrator /usr/local/bin/

# Copy data directory
COPY data ./data

# Create README guide at ~/README.md
RUN mkdir -p /root && cat > /root/README.md << 'EOF'
# Test Case Manager - Docker Container Guide

Welcome to the Test Case Manager Docker container! This container includes all the tools needed for managing and executing test cases.

## Available Binaries

All binaries are installed in `/usr/local/bin` and available in your PATH:

### Primary Tool
- **tcm** - Test Case Manager (main interactive tool)
  - Full name: `testcase-manager`
  - Create, edit, and manage test cases interactively
  - Database-backed condition selection with fuzzy search
  - Git integration for version control
  - Usage: `tcm --help`

### Test Execution & Verification
- **test-executor** - Automated test execution from YAML files
  - Generate bash scripts from test cases
  - Execute tests with JSON logging
  - Usage: `test-executor --help`

- **test-verify** - Test verification and reporting
  - Verify test execution logs against test cases
  - Generate JUnit XML reports for CI/CD
  - Batch verification mode
  - Usage: `test-verify --help`

- **test-orchestrator** - Test orchestration and coordination
  - Coordinate complex test workflows
  - Usage: `test-orchestrator --help`

### Validation Tools
- **validate-yaml** - YAML file validation
  - Validate YAML syntax and structure
  - Usage: `validate-yaml <file.yaml>`

- **validate-json** - JSON file validation
  - Validate JSON syntax and structure
  - Usage: `validate-json <file.json>`

### Additional Tools
- **trm** - Test Run Manager
  - Manage test execution runs
  - Usage: `trm --help`

- **editor** - Interactive test case editor
  - Edit test cases with enhanced interface
  - Usage: `editor --help`

## Quick Start

### Create a new test case
```bash
tcm create --id TC_001
```

### Build test sequences interactively
```bash
tcm build-sequences-with-steps
```

### List all test cases
```bash
tcm list
```

### Execute a test case
```bash
test-executor execute testcases/my_test.yml
```

### Verify test execution
```bash
test-verify single --log test_execution_log.json --test-case-id TC_001
```

## Data Directory

Test case data is located at: `/app/data`

## Git Integration

Git is pre-installed for version control integration. Initialize a repository:
```bash
cd /app
git init
git config user.name "Your Name"
git config user.email "your.email@example.com"
```

## Links & Documentation

- **GitHub Repository**: https://github.com/yourusername/testcase-manager
- **Test Case Schema**: `/app/data/testcase_schema.json`
- **Execution Log Schema**: `/app/data/test_execution_log_schema.json`

## Container Information

- **Base Image**: Debian Bookworm Slim
- **Rust Version**: 1.92
- **Working Directory**: `/app`
- **User**: root

## Tips

1. Use `tcm --verbose` for detailed logging
2. All commands support `--help` for usage information
3. Tab completion works for file paths and commands
4. Interactive mode automatically detects TTY and falls back to numbered selection in non-TTY environments

For more detailed documentation, visit the GitHub repository or run `tcm --help`.
EOF

# Set default command
CMD ["tcm"]
