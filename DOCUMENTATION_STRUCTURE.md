# Documentation Structure

This document describes the organization of the Test Case Manager documentation.

## Overview

The documentation has been reorganized into a hierarchical structure with dedicated sections for different use cases and audiences. The main entry point is `docs/index.md`, which serves as the documentation homepage.

## Directory Structure

```
docs/
├── index.md                    # Documentation homepage with navigation
├── getting-started/            # Installation and quick start guides
│   ├── README.md              # Section index
│   ├── index.md               # Quick start guide (formerly QUICK_START.md)
│   ├── docker.md              # Docker installation (formerly DOCKER.md)
│   └── prerequisites.md       # Prerequisites guide (formerly PREREQUISITES.md)
├── user-guide/                 # Core usage and workflows
│   ├── README.md              # Section index
│   ├── interactive-workflow.md # Interactive workflow guide
│   └── validation.md          # Validation guide
├── cli-tools/                  # Command-line tool reference
│   ├── README.md              # Section index with tool overview
│   ├── test-verify-usage.md   # test-verify detailed usage
│   ├── test-verify-workflow.md # test-verify workflow guide
│   ├── test-verify-quick-reference.md # test-verify quick ref
│   ├── validate-yaml.md       # validate-yaml reference
│   └── json-escaping-config.md # JSON escaping configuration
├── features/                   # Advanced features documentation
│   ├── README.md              # Section index
│   ├── bdd-initial-conditions.md # BDD patterns
│   ├── variable-passing.md    # Variable capture and substitution
│   ├── variables-capture-command.md # Advanced variable capture
│   ├── environment-variable-hydration.md # Env var substitution
│   ├── manual-steps-handling.md # Manual step support
│   ├── manual-step-filtering.md # Manual step filtering
│   ├── conditional-verification.md # Conditional verification
│   ├── verification-templates.md # Verification patterns
│   ├── tty-fallback.md        # Non-TTY environment handling
│   ├── recovery.md            # Recovery mechanism
│   ├── watch-mode-comparison.md # Watch mode implementations
│   ├── junit-export.md        # JUnit XML export
│   └── junit-xml-xsd-validation.md # JUnit XSD validation
├── development/                # Developer documentation
│   ├── README.md              # Section index with setup guide
│   ├── coverage.md            # Code coverage testing (formerly COVERAGE.md)
│   ├── gitlab-ci-setup.md     # GitLab CI configuration
│   ├── gitlab-ci-examples.md  # CI/CD examples
│   └── interactive-implementation.md # Implementation details
└── examples/                   # Usage examples and patterns
    └── README.md              # Examples and common workflows
```

## Migration Map

Old documentation files have been relocated to new paths:

### Getting Started
- `docs/QUICK_START.md` → `docs/getting-started/index.md`
- `docs/DOCKER.md` → `docs/getting-started/docker.md`
- `docs/PREREQUISITES.md` → `docs/getting-started/prerequisites.md`

### User Guide
- `docs/interactive_workflow.md` → `docs/user-guide/interactive-workflow.md`
- `docs/validation.md` → `docs/user-guide/validation.md`

### CLI Tools
- `docs/TEST_VERIFY_USAGE.md` → `docs/cli-tools/test-verify-usage.md`
- `docs/TEST_VERIFY_WORKFLOW.md` → `docs/cli-tools/test-verify-workflow.md`
- `docs/TEST_VERIFY_QUICK_REFERENCE.md` → `docs/cli-tools/test-verify-quick-reference.md`
- `docs/VALIDATE_YAML_QUICK_REF.md` → `docs/cli-tools/validate-yaml.md`
- `docs/JSON_ESCAPING_CONFIG.md` → `docs/cli-tools/json-escaping-config.md`

### Features
- `docs/BDD_INITIAL_CONDITIONS.md` → `docs/features/bdd-initial-conditions.md`
- `docs/VARIABLE_PASSING.md` → `docs/features/variable-passing.md`
- `docs/VARIABLES_CAPTURE_COMMAND.md` → `docs/features/variables-capture-command.md`
- `docs/ENVIRONMENT_VARIABLE_HYDRATION.md` → `docs/features/environment-variable-hydration.md`
- `docs/MANUAL_STEPS_HANDLING.md` → `docs/features/manual-steps-handling.md`
- `docs/MANUAL_STEP_FILTERING.md` → `docs/features/manual-step-filtering.md`
- `docs/CONDITIONAL_VERIFICATION.md` → `docs/features/conditional-verification.md`
- `docs/VERIFICATION_TEMPLATES.md` → `docs/features/verification-templates.md`
- `docs/TTY_FALLBACK.md` → `docs/features/tty-fallback.md`
- `docs/RECOVERY.md` → `docs/features/recovery.md`
- `docs/WATCH_MODE_COMPARISON.md` → `docs/features/watch-mode-comparison.md`
- `docs/JUNIT_EXPORT.md` → `docs/features/junit-export.md`
- `docs/JUNIT_XML_XSD_VALIDATION.md` → `docs/features/junit-xml-xsd-validation.md`

### Development
- `docs/COVERAGE.md` → `docs/development/coverage.md`
- `docs/GITLAB_CI_SETUP.md` → `docs/development/gitlab-ci-setup.md`
- `docs/GITLAB_CI_EXAMPLES.md` → `docs/development/gitlab-ci-examples.md`
- `docs/INTERACTIVE_IMPLEMENTATION.md` → `docs/development/interactive-implementation.md`

### Main Documentation
- `README.md` → `docs/index.md` (documentation homepage)

## Section Indices

Each subdirectory contains a `README.md` file that serves as a section index with:
- Overview of the section contents
- Links to documents within the section
- Navigation to related sections
- Quick reference information

## Link Updates

All internal markdown links have been updated to reflect the new paths:
- Relative links updated to use `../` notation for cross-section navigation
- Links to external resources (scripts, tests) updated with appropriate `../` prefixes
- Section index links point to `README.md` files

## Navigation

Users can navigate the documentation in multiple ways:

1. **Top-down**: Start at `docs/index.md` and browse by section
2. **Section indices**: Use `README.md` files in each section for quick access
3. **Cross-references**: Follow links within documents to related topics
4. **Search**: Use file browser or grep to find specific topics

## Benefits

The new structure provides:

- **Clear organization** - Documents grouped by purpose and audience
- **Easy discovery** - Section indices help users find relevant docs
- **Better navigation** - Hierarchical structure with clear paths
- **Scalability** - Easy to add new documents in appropriate sections
- **User-focused** - Getting Started, User Guide, and Examples for end users
- **Developer-focused** - Development section for contributors
- **Reference-focused** - CLI Tools and Features for detailed information
