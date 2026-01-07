# Test Case Manager

A comprehensive CLI tool for managing test cases in YAML format with fuzzy search, interactive prompts, editor integration, and git operations.

## Features

- **YAML-based test cases**: Store test cases in a structured, human-readable format
- **Interactive CLI**: Create and edit test cases with guided prompts
- **Fuzzy search**: Quickly find test cases using skim fuzzy finder
- **Editor integration**: Edit test cases in your preferred text editor
- **Git integration**: Version control for test cases with built-in git operations
- **Validation**: JSON Schema validation for test case files
- **Import/Export**: Batch operations with test suite files

## Installation

```bash
cargo build --release
```

## Usage

### Initialize a repository

```bash
# Initialize test case repository
testcase-manager init

# Initialize with git
testcase-manager init --git
```

### Create a test case

```bash
# Interactive creation (legacy)
testcase-manager create

# With parameters
testcase-manager create --id TC001 --title "Login Test"

# Open in editor
testcase-manager create --edit

# Interactive workflow with guided prompts and validation
testcase-manager create-interactive
```

### Interactive Test Case Creation Workflow

The `create-interactive` command provides a guided workflow with:
- Interactive prompts for metadata (requirement, item, tc, id, description)
- Schema validation after each section
- Git commits after each major section
- Editor integration for general initial conditions
- Interactive device selection and condition entry for initial conditions
- Default values that can be kept or edited

```bash
# Start interactive workflow
testcase-manager create-interactive

# With custom path
testcase-manager create-interactive --path ./my-testcases
```

See [Interactive Workflow Documentation](docs/interactive_workflow.md) for details.

### List test cases

```bash
# List all
testcase-manager list

# Filter by tag
testcase-manager list --tag "login"

# Filter by status
testcase-manager list --status "active"

# Verbose output
testcase-manager list --verbose
```

### Edit a test case

```bash
# Edit by ID
testcase-manager edit TC001

# Use fuzzy finder
testcase-manager edit --fuzzy
```

### View a test case

```bash
# View by ID
testcase-manager view TC001

# Use fuzzy finder
testcase-manager view --fuzzy
```

### Search test cases

```bash
testcase-manager search
```

### Validate test cases

```bash
# Validate specific file
testcase-manager validate --file TC001.yaml

# Validate all
testcase-manager validate --all
```

### Export/Import

```bash
# Export to test suite
testcase-manager export --output suite.yaml

# Export with tag filter
testcase-manager export --output suite.yaml --tags "smoke,critical"

# Import from test suite
testcase-manager import suite.yaml
```

### Git operations

```bash
# Add files to staging
testcase-manager git add TC001 TC002
testcase-manager git add --all

# Commit changes
testcase-manager git commit --message "Add new test cases"

# Check status
testcase-manager git status

# View log
testcase-manager git log --limit 5
```

## Test Case Schema

Test cases follow this structure:

```yaml
id: TC001
title: User Login Test
description: Verify user can login with valid credentials
priority: high
status: active
type: functional
tags:
  - login
  - authentication
author: John Doe
created_at: 2024-01-01T00:00:00Z
updated_at: 2024-01-01T00:00:00Z

sequences:
  - id: SEQ001
    name: Main Login Flow
    description: Standard login procedure
    steps:
      - id: STEP001
        description: Navigate to login page
        action: navigate
        target: /login
      - id: STEP002
        description: Enter username
        action: type
        target: "#username"
        value: testuser
      - id: STEP003
        description: Enter password
        action: type
        target: "#password"
        value: password123
      - id: STEP004
        description: Click login button
        action: click
        target: "#login-btn"
      - id: STEP005
        description: Verify successful login
        action: verify
        expected: Dashboard page is displayed

preconditions:
  - description: User account exists
    setup_steps:
      - Create test user in database

cleanup:
  - description: Clean up test data
    cleanup_steps:
      - Delete test user session

environments:
  - name: staging
    url: https://staging.example.com
    variables:
      API_KEY: test-key-123

related_tests:
  - TC002
  - TC003
```

## License

MIT
