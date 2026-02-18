# Features

Test Case Manager includes powerful features for advanced test case management and automation.

## Core Features

### BDD & Natural Language

- **[BDD Initial Conditions](bdd-initial-conditions.md)** - Write human-readable test conditions that automatically convert to executable bash commands
  - 23 built-in step patterns
  - Custom pattern support
  - Mix BDD and plain text

### Variables & Data Flow

- **[Variable Passing](variable-passing.md)** - Capture and pass data between test steps
  - Capture values from command output using regex
  - Variable substitution in commands
  - Scope rules and variable management

- **[Variables Capture Command](variables-capture-command.md)** - Advanced variable capture techniques
  - Regex pattern matching
  - Multiple capture groups
  - Output parsing

- **[Environment Variable Hydration](environment-variable-hydration.md)** - Dynamic environment variable substitution
  - Load environment variables from shell
  - Hydrate test case YAML with actual values
  - Environment-specific test execution

### Test Execution

- **[Manual Steps Handling](manual-steps-handling.md)** - Support for manual test steps
  - Interactive prompts for human verification
  - Non-interactive mode handling
  - Mixed automated/manual workflows

- **[Manual Step Filtering](manual-step-filtering.md)** - Control manual step execution
  - Filter manual steps in automated runs
  - Selective execution modes

- **[Conditional Verification](conditional-verification.md)** - Advanced verification logic
  - Conditional expressions in bash
  - Complex verification patterns
  - Exit code and output validation

- **[Verification Templates](verification-templates.md)** - Reusable verification patterns
  - Common verification templates
  - Best practices and examples

### User Experience

- **[TTY Fallback](tty-fallback.md)** - Graceful handling of non-TTY environments
  - Automatic detection of terminal availability
  - Numbered selection fallback
  - Works in CI/CD, VS Code debug console, etc.

- **[Recovery](recovery.md)** - Automatic progress saving and resumption
  - Save state after each operation
  - Resume interrupted workflows
  - Error tracking and inline annotations

### Monitoring & Continuous Integration

- **[Watch Mode Comparison](watch-mode-comparison.md)** - Different watch mode implementations
  - Script-based watch mode
  - Binary-based watch mode
  - Feature comparison and selection guide

- **[JUnit Export](junit-export.md)** - JUnit XML output for CI/CD integration
  - Standard JUnit XML format
  - Test results reporting
  - CI/CD pipeline integration

- **[JUnit XML XSD Validation](junit-xml-xsd-validation.md)** - Schema validation for JUnit XML
  - XSD validation
  - Format verification
  - Standards compliance

## Feature Categories

### Test Definition
- BDD Initial Conditions
- Prerequisites
- Conditional Verification
- Verification Templates

### Test Execution
- Variable Passing
- Environment Variable Hydration
- Manual Steps Handling
- Manual Step Filtering

### User Interface
- TTY Fallback
- Recovery Mechanism
- Interactive Workflows

### CI/CD Integration
- JUnit Export
- Watch Mode
- GitLab CI Examples

## Getting Started with Features

1. Start with [BDD Initial Conditions](bdd-initial-conditions.md) to write readable test conditions
2. Learn [Variable Passing](variable-passing.md) to handle dynamic data
3. Explore [Recovery](recovery.md) to understand workflow resilience
4. Check [JUnit Export](junit-export.md) for CI/CD integration

For practical examples, see the [Examples](../examples/) section.
