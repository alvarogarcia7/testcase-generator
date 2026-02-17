
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
- **Test Case Schema**: `/app/schemas/test-case.schema.json`
- **Execution Log Schema**: `/app/schemas/execution-log.schema.json`

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
