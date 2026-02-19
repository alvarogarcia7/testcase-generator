# Documentation

CLI tools for test management in YAML format with interactive workflows, validation, and automation capabilities.

## Quick Links

- [Quick Start Guide](getting-started/index.md)
- [Interactive Workflow](user-guide/interactive-workflow.md)
- [Validate YAML](cli-tools/validate-yaml.md)

## Features

- **Interactive Creation**: Build test cases with guided prompts
- **Schema Validation**: Validate test cases with watch mode
- **Fuzzy Search**: Search through test cases and components
- **Recovery**: Automatically save and resume progress
- **BDD Patterns**: Human-readable conditions that convert to commands
- **Variables**: Capture and pass data between steps

## Installation

### Docker

```bash
# Build the image
./scripts/build-docker.sh

# Run interactively
docker run -it --rm testcase-manager:latest
```

### From Source

```bash
# Build all binaries
make build

# Run tests
make test
```

## Quick Example

```yaml
test_sequences:
  - id: 1
    name: Basic Test
    steps:
      - step: 1
        description: "Check system"
        command: "echo 'Hello World'"
        expected:
          result: "0"
          output: "Hello World"
```

## Documentation Sections

- **[Getting Started](getting-started/)** - Installation and quick start
- **[User Guide](user-guide/)** - Core workflows and concepts
- **[CLI Tools](cli-tools/)** - Command-line tool reference
- **[Features](features/)** - Advanced features and capabilities
- **[Examples](examples/)** - Real-world usage patterns
