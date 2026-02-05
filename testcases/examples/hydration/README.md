# Hydration Examples

This directory contains example test cases demonstrating environment variable hydration functionality.

## Overview

These examples show how to use hydration placeholders (`${#VAR_NAME}`) in test cases to create environment-agnostic test definitions that can be hydrated with environment-specific values using export files.

## Test Cases

### TC_HYDRATION_001: API Endpoints with Different Environments

**Purpose**: Demonstrates hydration for API endpoint testing across multiple environments.

**Key Features**:
- Environment-specific API URLs
- Different authentication credentials per environment
- Environment-specific rate limits and timeouts
- Variable protocol handling (http vs https)

**Use Cases**:
- API health checks
- Authentication flow testing
- Rate limit verification
- Response time validation

**Files**:
- `TC_HYDRATION_001.yaml` - Test case template with hydration placeholders
- `TC_HYDRATION_001_dev.export` - Development environment variables
- `TC_HYDRATION_001_staging.export` - Staging environment variables
- `TC_HYDRATION_001_prod.export` - Production environment variables

### TC_HYDRATION_002: Database Connection Strings

**Purpose**: Demonstrates hydration for database connection testing with environment-specific configurations.

**Key Features**:
- Different database hosts per environment
- Environment-specific connection strings
- SSL mode configuration
- Database version verification
- Performance threshold validation

**Use Cases**:
- Database connectivity testing
- Connection string format validation
- SSL/TLS verification
- Query performance testing
- Cache hit ratio monitoring

**Files**:
- `TC_HYDRATION_002.yaml` - Test case template with hydration placeholders
- `TC_HYDRATION_002_dev.export` - Development database configuration
- `TC_HYDRATION_002_staging.export` - Staging database configuration
- `TC_HYDRATION_002_prod.export` - Production database configuration

### TC_HYDRATION_003: Mixed Hydration and Sequence Variables

**Purpose**: Demonstrates combining environment hydration variables with runtime captured variables and sequence-level variables.

**Key Features**:
- Mix of hydration vars, sequence vars, and captured vars
- Dynamic variable composition
- Multi-step workflows with variable chaining
- Variable substitution in multiple contexts

**Use Cases**:
- Complex workflows with authentication tokens
- Data processing pipelines
- Resource creation and management
- Metrics collection and validation

**Files**:
- `TC_HYDRATION_003.yaml` - Test case template with mixed variables
- `TC_HYDRATION_003_dev.export` - Development configuration
- `TC_HYDRATION_003_staging.export` - Staging configuration
- `TC_HYDRATION_003_prod.export` - Production configuration

## Usage Examples

### 1. Generate Export Template

Generate an export file template from a test case's hydration_vars:

```bash
test-executor generate-export TC_HYDRATION_001.yaml --output TC_HYDRATION_001_custom.export
```

### 2. Validate Export File

Validate that an export file contains all required variables:

```bash
test-executor validate-export TC_HYDRATION_001.yaml --export-file TC_HYDRATION_001_dev.export
```

### 3. Hydrate Test Case

Replace hydration placeholders with environment-specific values:

```bash
# Development
test-executor hydrate TC_HYDRATION_001.yaml \
  --export-file TC_HYDRATION_001_dev.export \
  --output TC_HYDRATION_001_dev_hydrated.yaml

# Staging
test-executor hydrate TC_HYDRATION_001.yaml \
  --export-file TC_HYDRATION_001_staging.export \
  --output TC_HYDRATION_001_staging_hydrated.yaml

# Production
test-executor hydrate TC_HYDRATION_001.yaml \
  --export-file TC_HYDRATION_001_prod.export \
  --output TC_HYDRATION_001_prod_hydrated.yaml
```

### 4. Execute Hydrated Test

Execute the hydrated test case:

```bash
test-executor execute TC_HYDRATION_001_dev_hydrated.yaml
```

### 5. Complete Workflow

Combine validation, hydration, and execution:

```bash
# For development environment
test-executor validate-export TC_HYDRATION_001.yaml --export-file TC_HYDRATION_001_dev.export && \
  test-executor hydrate TC_HYDRATION_001.yaml --export-file TC_HYDRATION_001_dev.export --output /tmp/TC_HYDRATION_001_dev.yaml && \
  test-executor execute /tmp/TC_HYDRATION_001_dev.yaml
```

## Environment Comparison

| Environment | API Protocol | Database SSL | Rate Limits | Batch Size | Cache Threshold |
|-------------|--------------|--------------|-------------|------------|-----------------|
| Development | http         | off          | 100/min     | 50         | 75%             |
| Staging     | https        | on           | 500/min     | 500        | 85%             |
| Production  | https        | on           | 1000/min    | 1000       | 95%             |

## Variable Types

### 1. Hydration Variables (`${#VAR_NAME}`)

Defined in the `hydration_vars` section and replaced during hydration:

```yaml
hydration_vars:
  API_BASE_URL:
    name: "API_BASE_URL"
    description: "Base URL for the API"
    default_value: "http://localhost:8080"
    required: true
```

Used in test steps:
```yaml
command: "curl ${#API_BASE_URL}/health"
```

### 2. Sequence Variables

Defined in the `variables` section of a test sequence:

```yaml
test_sequences:
  - id: 1
    variables:
      content_type: "application/json"
      user_agent: "TestClient/1.0"
```

Used in test steps:
```yaml
command: "curl -H 'Content-Type: ${content_type}' -H 'User-Agent: ${user_agent}' ..."
```

### 3. Captured Variables

Extracted from command output using regex patterns:

```yaml
capture_vars:
  session_token: "(?<=\"token\":\")[a-zA-Z0-9._-]+"
  user_id: "(?<=\"user_id\":)\\d+"
```

Used in subsequent steps:
```yaml
command: "curl -H 'Authorization: Bearer ${session_token}' /api/users/${user_id}"
```

## Best Practices

### 1. Naming Conventions

- **Hydration vars**: UPPERCASE_WITH_UNDERSCORES (e.g., `API_BASE_URL`)
- **Sequence vars**: lowercase_with_underscores (e.g., `content_type`)
- **Captured vars**: lowercase_with_underscores (e.g., `session_token`)

### 2. Variable Organization

- Use hydration vars for environment-specific configuration
- Use sequence vars for constants shared across steps
- Use captured vars for dynamic runtime values

### 3. Security

- Never commit production credentials to version control
- Use export file templates (`.export.template`) for sensitive environments
- Store production secrets in secure vaults or CI/CD secret management

### 4. Documentation

- Add descriptions to all hydration_vars
- Use meaningful variable names
- Document expected values and formats in export files

## Common Patterns

### Pattern 1: Authentication Flow

```yaml
# Step 1: Authenticate with hydration credentials
command: "curl -X POST ${#API_URL}/login -d '{\"user\":\"${#USERNAME}\",\"pass\":\"${#PASSWORD}\"}'"
capture_vars:
  token: "(?<=\"token\":\")[^\"]+(?=\")"

# Step 2: Use captured token
command: "curl -H 'Authorization: Bearer ${token}' ${#API_URL}/protected"
```

### Pattern 2: Resource Creation and Retrieval

```yaml
# Step 1: Create resource
command: "curl -X POST ${#API_URL}/resources -d '{\"name\":\"${#RESOURCE_NAME}\"}'"
capture_vars:
  resource_id: "(?<=\"id\":)\\d+"

# Step 2: Retrieve created resource
command: "curl ${#API_URL}/resources/${resource_id}"
```

### Pattern 3: Environment-Specific Validation

```yaml
# Verify response time meets environment SLA
command: "curl -w '%{time_total}' ${#API_URL}/endpoint"
verification:
  output: "awk '{exit !($1 < ${#MAX_RESPONSE_TIME})}' <<< \"$COMMAND_OUTPUT\""
```

## Troubleshooting

### Placeholder Not Replaced

**Problem**: `${#VAR_NAME}` still appears in hydrated output

**Solution**:
1. Check variable name spelling in export file
2. Verify variable is listed in `hydration_vars`
3. Ensure export file uses correct format: `export VAR_NAME=value`

### Validation Fails

**Problem**: `validate-export` reports missing variables

**Solution**:
1. Check that all required variables are in export file
2. Verify no typos in variable names (case-sensitive)
3. Ensure proper export syntax with no extra spaces

### Variable Not Captured

**Problem**: Captured variable is empty in subsequent steps

**Solution**:
1. Test regex pattern with `grep -oP` manually
2. Check command output contains expected pattern
3. Verify proper escaping in YAML (use `\\d` not `\d`)

## Additional Resources

- [Environment Variable Hydration Documentation](../../../docs/ENVIRONMENT_VARIABLE_HYDRATION.md)
- [Variable Passing Documentation](../../../docs/VARIABLE_PASSING.md)
- Test Executor CLI: `test-executor --help`

## Quick Reference

```bash
# Generate template
test-executor generate-export <test.yaml> --output <file.export>

# Validate export
test-executor validate-export <test.yaml> --export-file <file.export>

# Hydrate test
test-executor hydrate <test.yaml> --export-file <file.export> --output <hydrated.yaml>

# Execute test
test-executor execute <hydrated.yaml>
```
