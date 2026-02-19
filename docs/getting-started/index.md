# Quick Start

Get started with interactive test case creation.

## Installation

```bash
# Build the project
make build

# Or with cargo
cargo build --release
```

## Basic Usage

### Create a Test Case

```bash
# Start interactive workflow
editor create-interactive

# Or with custom path
editor create-interactive --path ./my-tests
```

### Follow the Prompts

The tool will guide you through:

1. **Metadata** - Requirement, item, TC, ID, description
2. **Initial Conditions** - Setup requirements
3. **Validation** - Automatic schema validation
4. **Git Commits** - Optional version control

## Example

```yaml
requirement: XXX100
item: 1
tc: 4
id: '4.2.2.2.1_test'
description: 'My test description'

initial_conditions:
  setup:
    - "create directory \"/tmp/test\""
    - "set environment variable \"MODE\" to \"test\""
```

## Configuration

```bash
# Set editor
export EDITOR=vim

# Set git author
export GIT_AUTHOR_NAME="Your Name"
export GIT_AUTHOR_EMAIL="your@email.com"
```

## Tips

- Enter numbers without quotes (e.g., `1` not `"1"`)
- Use spaces for indentation, not tabs
- Press Y to accept defaults
- Press N to skip optional sections

## Next Steps

- [Interactive Workflow Guide](../user-guide/interactive-workflow.md)
- [CLI Tools Reference](../cli-tools/)
- [Examples](../examples/)
