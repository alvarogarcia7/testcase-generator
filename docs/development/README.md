# Development

Documentation for developers contributing to the project.

## Setup

```bash
# Build
make build

# Test
make test

# Lint
make lint
```

## Code Quality

- **[Coverage Testing](coverage.md)** - Code coverage guide
  - Minimum 50% unit test coverage
  - Minimum 70% E2E coverage
  - HTML reports

## CI/CD Integration

- **[GitLab CI Setup](gitlab-ci-setup.md)** - Pipeline configuration
- **[GitLab CI Examples](gitlab-ci-examples.md)** - Example configurations

## Requirements

- Minimum coverage: 50% (unit), 70% (E2E)
- All tests must pass
- Scripts must be compatible with bash 3.2+
