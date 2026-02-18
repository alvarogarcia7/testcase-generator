# Environment Variable Hydration

## Overview

Environment variable hydration is a powerful feature that allows you to create reusable test case templates with placeholders that can be dynamically replaced with environment-specific values. This enables you to maintain a single test case definition while easily adapting it for different environments (development, staging, production) or configurations.

## Table of Contents

- [Syntax Overview](#syntax-overview)
- [Export File Format](#export-file-format)
- [Hydration Workflow](#hydration-workflow)
- [CLI Commands](#cli-commands)
- [Complete Examples](#complete-examples)
- [Best Practices](#best-practices)
- [Advanced Use Cases](#advanced-use-cases)

## Syntax Overview

### ${#VAR_NAME} vs ${VAR_NAME}

The hydration system uses **two distinct placeholder syntaxes** for different purposes:

#### 1. `${#VAR_NAME}` - Hydration Placeholders (YAML Level)

Used in **YAML test case files** to mark variables that need to be hydrated before test execution.

**Characteristics:**
- Contains a hash/pound symbol: `${#VAR_NAME}`
- Variable names must be UPPERCASE with underscores and numbers: `[A-Z_][A-Z0-9_]*`
- Used in YAML files before hydration
- Replaced during hydration process
- Left unchanged if variable not found in export file

**Example:**
```yaml
steps:
  - step: 1
    description: "Connect to server"
    command: "ssh ${#USERNAME}@${#SERVER_HOST}:${#PORT}"
    expected:
      output: "${#SUCCESS_MESSAGE}"
    verification:
      result: "[ $EXIT_CODE -eq 0 ]"
      output: "grep -q ${#EXPECTED_PATTERN} $COMMAND_OUTPUT"
```

#### 2. `${VAR_NAME}` - Bash Variables (Script Level)

Used in **generated bash scripts** for standard shell variable substitution.

**Characteristics:**
- No hash symbol: `${VAR_NAME}`
- Standard bash variable syntax
- Evaluated at bash script runtime
- Sourced from export file or defined in script

**Example (after hydration):**
```yaml
steps:
  - step: 1
    description: "Connect to server"
    command: "ssh admin@example.com:22"
    expected:
      output: "Connected successfully"
```

**Generated bash script:**
```bash
#!/bin/bash

# Source environment variables
EXPORT_FILE="TC_001.env"
if [ -f "$EXPORT_FILE" ]; then
    source "$EXPORT_FILE"
fi

# Test execution with bash variables
COMMAND="ssh admin@example.com:22"
eval "$COMMAND"
```

### Automatic Conversion

When generating bash scripts, the test-executor automatically converts hydration placeholders:
- `${#VAR_NAME}` in YAML → `${VAR_NAME}` in bash script
- This ensures proper bash variable substitution at runtime

## Export File Format

### File Structure

Export files use standard bash `export` statement syntax:

```bash
# General format
export VAR_NAME=value
export VAR_NAME="value with spaces"
export VAR_NAME='single quoted value'

# Comments are supported
# Empty lines are ignored
```

### Supported Formats

#### 1. Unquoted Values (Simple)
```bash
export SERVER_HOST=localhost
export PORT=8080
export DEBUG=true
```

Use for: Simple alphanumeric values without special characters or spaces.

#### 2. Double-Quoted Values
```bash
export MESSAGE="Hello World"
export PATH="/usr/local/bin:/usr/bin"
export URL="https://api.example.com/v1?key=abc123"
```

Use for: Values with spaces, special characters, or when you need variable expansion.

**Special character escaping:**
- Double quotes: `\"`
- Dollar signs: `\$`
- Backslashes: `\\`
- Backticks: `` \` ``

#### 3. Single-Quoted Values
```bash
export JSON='{"key": "value"}'
export REGEX='[0-9]+\.[0-9]+'
export COMMAND='ls -la | grep test'
```

Use for: Literal strings where you want to preserve everything as-is (no variable expansion).

### Example Export File

```bash
# Environment: Production
# Generated: 2024-01-15

# Server Configuration
export SERVER_HOST=prod.example.com
export SERVER_PORT=443
export SERVER_PROTOCOL=https

# Authentication
export USERNAME=admin
export API_KEY=sk_prod_1234567890abcdef

# Database Configuration
export DB_HOST=db.prod.example.com
export DB_PORT=5432
export DB_NAME=production_db
export DB_CONNECTION_STRING="postgresql://user:pass@db.prod.example.com:5432/production_db"

# Feature Flags
export ENABLE_LOGGING=true
export DEBUG_MODE=false
export TIMEOUT_SECONDS=30

# Complex Values
export JSON_CONFIG='{"env": "production", "region": "us-east-1"}'
export URL_PATTERN="https://*.example.com/*"
export SUCCESS_REGEX="Status: (200|201|204)"
```

## Hydration Workflow

### Step-by-Step Process

```
┌─────────────────────────────────────┐
│  1. Create Test Case Template       │
│     (with ${#VAR} placeholders)     │
└─────────────────┬───────────────────┘
                  │
                  ▼
┌─────────────────────────────────────┐
│  2. Generate Export Template        │
│     (from hydration_vars)           │
└─────────────────┬───────────────────┘
                  │
                  ▼
┌─────────────────────────────────────┐
│  3. Fill Values in Export File      │
│     (manually or via script)        │
└─────────────────┬───────────────────┘
                  │
                  ▼
┌─────────────────────────────────────┐
│  4. Validate Export File            │
│     (optional but recommended)      │
└─────────────────┬───────────────────┘
                  │
                  ▼
┌─────────────────────────────────────┐
│  5. Hydrate YAML                    │
│     (replace ${#VAR} with values)   │
└─────────────────┬───────────────────┘
                  │
                  ▼
┌─────────────────────────────────────┐
│  6. Generate & Execute Tests        │
│     (using hydrated YAML)           │
└─────────────────────────────────────┘
```

### Detailed Workflow

#### 1. Create Test Case Template

Define your test case with `${#VAR_NAME}` placeholders and a `hydration_vars` section:

```yaml
requirement: "REQ001"
item: 1
tc: 1
id: "TC_LOGIN_001"
description: "Test user login functionality"

test_sequences:
  - id: 1
    name: "Login Test"
    description: "Verify user can login"
    steps:
      - step: 1
        description: "Send login request"
        command: "curl -X POST ${#API_URL}/login -d '{\"username\":\"${#USERNAME}\",\"password\":\"${#PASSWORD}\"}'"
        expected:
          result: "0"
          output: "${#SUCCESS_RESPONSE}"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "grep -q ${#SUCCESS_PATTERN} $COMMAND_OUTPUT"

# Define hydration variables with metadata
hydration_vars:
  API_URL:
    name: "API_URL"
    description: "Base API URL for the environment"
    default_value: "http://localhost:8080"
    required: true
  USERNAME:
    name: "USERNAME"
    description: "Test user username"
    default_value: "testuser"
    required: true
  PASSWORD:
    name: "PASSWORD"
    description: "Test user password"
    default_value: "password123"
    required: true
  SUCCESS_RESPONSE:
    name: "SUCCESS_RESPONSE"
    description: "Expected success response"
    default_value: "Login successful"
    required: true
  SUCCESS_PATTERN:
    name: "SUCCESS_PATTERN"
    description: "Pattern to match in output"
    default_value: "token"
    required: true
```

#### 2. Generate Export Template

```bash
test-executor generate-export TC_LOGIN_001.yaml --output dev.env
```

This creates:

```bash
export API_URL=http://localhost:8080
export PASSWORD=password123
export SUCCESS_PATTERN=token
export SUCCESS_RESPONSE=Login successful
export USERNAME=testuser
```

#### 3. Customize for Each Environment

**dev.env** (Development):
```bash
export API_URL=http://localhost:8080
export USERNAME=dev_user
export PASSWORD=dev_pass123
export SUCCESS_RESPONSE=Login successful
export SUCCESS_PATTERN=token
```

**staging.env** (Staging):
```bash
export API_URL=https://staging-api.example.com
export USERNAME=staging_user
export PASSWORD=staging_pass456
export SUCCESS_RESPONSE=Login successful
export SUCCESS_PATTERN=token
```

**prod.env** (Production):
```bash
export API_URL=https://api.example.com
export USERNAME=prod_user
export PASSWORD=prod_secure_789
export SUCCESS_RESPONSE=Login successful
export SUCCESS_PATTERN=token
```

#### 4. Validate Export Files (Optional)

```bash
# Validate development environment
test-executor validate-export TC_LOGIN_001.yaml --export-file dev.env

# Validate staging environment
test-executor validate-export TC_LOGIN_001.yaml --export-file staging.env

# Validate production environment
test-executor validate-export TC_LOGIN_001.yaml --export-file prod.env
```

Expected output:
```
✓ Export file is valid: all required variables are present
```

Or if validation fails:
```
✗ Export file validation failed
  Required variables missing: PASSWORD, API_URL
  Optional variables missing: TIMEOUT
```

#### 5. Hydrate Test Cases

```bash
# Hydrate for development
test-executor hydrate TC_LOGIN_001.yaml \
  --export-file dev.env \
  --output TC_LOGIN_001_dev.yaml

# Hydrate for staging
test-executor hydrate TC_LOGIN_001.yaml \
  --export-file staging.env \
  --output TC_LOGIN_001_staging.yaml

# Hydrate for production
test-executor hydrate TC_LOGIN_001.yaml \
  --export-file prod.env \
  --output TC_LOGIN_001_prod.yaml
```

#### 6. Execute Tests

```bash
# Execute development tests
test-executor execute TC_LOGIN_001_dev.yaml

# Execute staging tests
test-executor execute TC_LOGIN_001_staging.yaml

# Execute production tests (with caution!)
test-executor execute TC_LOGIN_001_prod.yaml
```

## CLI Commands

### 1. generate-export

Generate an export file template from test case hydration_vars declarations.

**Syntax:**
```bash
test-executor generate-export <YAML_FILE> [--output <OUTPUT_FILE>]
```

**Arguments:**
- `<YAML_FILE>`: Path to test case YAML file with `hydration_vars` section
- `--output, -o <OUTPUT_FILE>`: Optional output file path (defaults to stdout)

**Examples:**
```bash
# Output to stdout
test-executor generate-export testcase.yaml

# Save to file
test-executor generate-export testcase.yaml --output vars.env

# Generate for multiple environments
test-executor generate-export testcase.yaml --output dev.env
test-executor generate-export testcase.yaml --output staging.env
test-executor generate-export testcase.yaml --output prod.env
```

**Behavior:**
- Extracts all variables defined in `hydration_vars` section
- Creates export statements with default values (if provided)
- Variables are sorted alphabetically
- Output format: `export VAR_NAME=default_value`

### 2. hydrate

Hydrate a test case YAML file with values from an export file.

**Syntax:**
```bash
test-executor hydrate <YAML_FILE> --export-file <EXPORT_FILE> [--output <OUTPUT_FILE>]
```

**Arguments:**
- `<YAML_FILE>`: Path to test case YAML file with `${#VAR}` placeholders
- `--export-file, -e <EXPORT_FILE>`: Path to export file with variable values
- `--output, -o <OUTPUT_FILE>`: Optional output file path (defaults to stdout)

**Examples:**
```bash
# Output to stdout
test-executor hydrate testcase.yaml --export-file dev.env

# Save to file
test-executor hydrate testcase.yaml --export-file dev.env --output testcase_dev.yaml

# Pipeline with validation
test-executor hydrate testcase.yaml --export-file prod.env --output testcase_prod.yaml && \
  test-executor execute testcase_prod.yaml
```

**Behavior:**
- Loads variables from export file
- Scans YAML content for `${#VAR_NAME}` patterns
- Replaces matching placeholders with values
- Leaves unmatched placeholders unchanged
- Preserves YAML structure and formatting

### 3. validate-export

Validate that an export file contains all required variables from test case.

**Syntax:**
```bash
test-executor validate-export <YAML_FILE> --export-file <EXPORT_FILE>
```

**Arguments:**
- `<YAML_FILE>`: Path to test case YAML file with `hydration_vars` section
- `--export-file, -e <EXPORT_FILE>`: Path to export file to validate

**Examples:**
```bash
# Validate export file
test-executor validate-export testcase.yaml --export-file dev.env

# Use in CI/CD pipeline (exits with non-zero on failure)
test-executor validate-export testcase.yaml --export-file prod.env || exit 1
```

**Exit Codes:**
- `0`: All required variables present
- `1`: Missing required variables

**Output Examples:**

Success (all required variables):
```
✓ Export file is valid: all required variables are present
```

Success (missing optional only):
```
✓ Export file is valid: all required variables are present
  Optional variables missing: TIMEOUT, DEBUG_MODE
```

Failure:
```
✗ Export file validation failed
  Required variables missing: API_KEY, SERVER_HOST
  Optional variables missing: RETRY_COUNT
```

### 4. generate

Generate a bash script from a test case YAML file.

**Syntax:**
```bash
test-executor generate <YAML_FILE> [--output <OUTPUT_FILE>] [--json-log]
```

**Arguments:**
- `<YAML_FILE>`: Path to test case YAML file
- `--output, -o <OUTPUT_FILE>`: Optional output file path (defaults to stdout)
- `--json-log`: Generate execution log JSON file alongside bash script

**Hydration Support:**
- Automatically detects `${#VAR}` placeholders in test case
- Generates script with export file sourcing
- Converts `${#VAR}` to `${VAR}` in generated script
- Creates environment-specific scripts when combined with hydration

**Examples:**
```bash
# Generate script with hydration support
test-executor generate testcase_dev.yaml --output test_dev.sh

# Generate with JSON log template
test-executor generate testcase_dev.yaml --output test_dev.sh --json-log

# Execute generated script
chmod +x test_dev.sh
./test_dev.sh
```

### 5. execute

Execute a test case directly (hydration must be done beforehand).

**Syntax:**
```bash
test-executor execute <YAML_FILE>
```

**Arguments:**
- `<YAML_FILE>`: Path to hydrated test case YAML file

**Examples:**
```bash
# Execute hydrated test case
test-executor execute testcase_dev.yaml

# Full workflow
test-executor hydrate testcase.yaml --export-file dev.env --output testcase_dev.yaml
test-executor execute testcase_dev.yaml
```

## Complete Examples

### Example 1: Simple API Testing

#### Test Case Template (api_health_check.yaml)

```yaml
requirement: "API-001"
item: 1
tc: 1
id: "TC_API_HEALTH_001"
description: "API health check across environments"

general_initial_conditions: {}
initial_conditions: {}

test_sequences:
  - id: 1
    name: "Health Check"
    description: "Verify API is responsive"
    initial_conditions: {}
    steps:
      - step: 1
        description: "Check API health endpoint"
        command: "curl -s -o /dev/null -w '%{http_code}' ${#API_URL}/health"
        expected:
          result: "0"
          output: "200"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[ \"$COMMAND_OUTPUT\" = \"200\" ]"
      
      - step: 2
        description: "Check API version endpoint"
        command: "curl -s ${#API_URL}/version"
        expected:
          result: "0"
          output: "${#EXPECTED_VERSION}"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[[ \"$COMMAND_OUTPUT\" =~ ${#VERSION_PATTERN} ]]"

hydration_vars:
  API_URL:
    name: "API_URL"
    description: "Base API URL"
    default_value: "http://localhost:8080"
    required: true
  EXPECTED_VERSION:
    name: "EXPECTED_VERSION"
    description: "Expected API version"
    default_value: "v1.0.0"
    required: true
  VERSION_PATTERN:
    name: "VERSION_PATTERN"
    description: "Version pattern regex"
    default_value: "v[0-9]+\\.[0-9]+\\.[0-9]+"
    required: false
```

#### Generate Export Templates

```bash
test-executor generate-export api_health_check.yaml --output dev.env
test-executor generate-export api_health_check.yaml --output staging.env
test-executor generate-export api_health_check.yaml --output prod.env
```

#### Customize for Each Environment

**dev.env:**
```bash
export API_URL=http://localhost:8080
export EXPECTED_VERSION=v1.0.0-dev
export VERSION_PATTERN=v[0-9]+\.[0-9]+\.[0-9]+-dev
```

**staging.env:**
```bash
export API_URL=https://staging-api.example.com
export EXPECTED_VERSION=v1.0.0-rc1
export VERSION_PATTERN=v[0-9]+\.[0-9]+\.[0-9]+-rc[0-9]+
```

**prod.env:**
```bash
export API_URL=https://api.example.com
export EXPECTED_VERSION=v1.0.0
export VERSION_PATTERN=v[0-9]+\.[0-9]+\.[0-9]+
```

#### Validate and Execute

```bash
# Development
test-executor validate-export api_health_check.yaml --export-file dev.env
test-executor hydrate api_health_check.yaml --export-file dev.env --output api_health_dev.yaml
test-executor execute api_health_dev.yaml

# Staging
test-executor validate-export api_health_check.yaml --export-file staging.env
test-executor hydrate api_health_check.yaml --export-file staging.env --output api_health_staging.yaml
test-executor execute api_health_staging.yaml

# Production
test-executor validate-export api_health_check.yaml --export-file prod.env
test-executor hydrate api_health_check.yaml --export-file prod.env --output api_health_prod.yaml
test-executor execute api_health_prod.yaml
```

### Example 2: Database Testing

#### Test Case Template (database_connection.yaml)

```yaml
requirement: "DB-001"
item: 1
tc: 1
id: "TC_DB_CONNECTION_001"
description: "Database connection and query test"

general_initial_conditions: {}
initial_conditions:
  database:
    - "Database server must be running"
    - "Test user credentials must be configured"

test_sequences:
  - id: 1
    name: "Connection Test"
    description: "Verify database connectivity"
    initial_conditions: {}
    steps:
      - step: 1
        description: "Test database connection"
        command: "psql -h ${#DB_HOST} -p ${#DB_PORT} -U ${#DB_USER} -d ${#DB_NAME} -c 'SELECT 1;'"
        expected:
          result: "0"
          output: "1"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "grep -q '1 row' $COMMAND_OUTPUT"
      
      - step: 2
        description: "Check database version"
        command: "psql -h ${#DB_HOST} -p ${#DB_PORT} -U ${#DB_USER} -d ${#DB_NAME} -c 'SELECT version();'"
        expected:
          result: "0"
          output: "${#EXPECTED_DB_VERSION}"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "grep -q ${#DB_VERSION_PATTERN} $COMMAND_OUTPUT"
      
      - step: 3
        description: "Query test table"
        command: "psql -h ${#DB_HOST} -p ${#DB_PORT} -U ${#DB_USER} -d ${#DB_NAME} -c 'SELECT COUNT(*) FROM ${#TEST_TABLE};'"
        expected:
          result: "0"
          output: "count"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "grep -q '[0-9]\\+' $COMMAND_OUTPUT"

hydration_vars:
  DB_HOST:
    name: "DB_HOST"
    description: "Database host"
    default_value: "localhost"
    required: true
  DB_PORT:
    name: "DB_PORT"
    description: "Database port"
    default_value: "5432"
    required: true
  DB_USER:
    name: "DB_USER"
    description: "Database user"
    default_value: "testuser"
    required: true
  DB_NAME:
    name: "DB_NAME"
    description: "Database name"
    default_value: "testdb"
    required: true
  EXPECTED_DB_VERSION:
    name: "EXPECTED_DB_VERSION"
    description: "Expected PostgreSQL version"
    default_value: "PostgreSQL 14"
    required: false
  DB_VERSION_PATTERN:
    name: "DB_VERSION_PATTERN"
    description: "Database version pattern"
    default_value: "PostgreSQL [0-9]\\+"
    required: false
  TEST_TABLE:
    name: "TEST_TABLE"
    description: "Test table name"
    default_value: "users"
    required: true
```

#### Environment-Specific Export Files

**dev.env:**
```bash
export DB_HOST=localhost
export DB_PORT=5432
export DB_USER=dev_user
export DB_NAME=dev_testdb
export EXPECTED_DB_VERSION=PostgreSQL 14
export DB_VERSION_PATTERN=PostgreSQL 14
export TEST_TABLE=dev_users
```

**staging.env:**
```bash
export DB_HOST=staging-db.example.com
export DB_PORT=5432
export DB_USER=staging_user
export DB_NAME=staging_testdb
export EXPECTED_DB_VERSION=PostgreSQL 15
export DB_VERSION_PATTERN=PostgreSQL 15
export TEST_TABLE=staging_users
```

**prod.env:**
```bash
export DB_HOST=prod-db.example.com
export DB_PORT=5432
export DB_USER=prod_readonly
export DB_NAME=production_db
export EXPECTED_DB_VERSION=PostgreSQL 15
export DB_VERSION_PATTERN=PostgreSQL 15
export TEST_TABLE=users
```

### Example 3: Multi-Environment CI/CD Pipeline

#### Test Case Template (ci_integration.yaml)

```yaml
requirement: "CI-001"
item: 1
tc: 1
id: "TC_CI_DEPLOY_001"
description: "CI/CD deployment verification"

general_initial_conditions: {}
initial_conditions:
  deployment:
    - "Application must be deployed"
    - "Load balancer must be configured"

test_sequences:
  - id: 1
    name: "Deployment Verification"
    description: "Verify successful deployment"
    initial_conditions: {}
    steps:
      - step: 1
        description: "Check application is running"
        command: "curl -f ${#APP_URL}/health"
        expected:
          result: "0"
          output: "healthy"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "grep -q 'status.*healthy' $COMMAND_OUTPUT"
      
      - step: 2
        description: "Verify correct version deployed"
        command: "curl -s ${#APP_URL}/version | jq -r '.version'"
        expected:
          result: "0"
          output: "${#EXPECTED_VERSION}"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[ \"$COMMAND_OUTPUT\" = \"${#EXPECTED_VERSION}\" ]"
      
      - step: 3
        description: "Check database connectivity from app"
        command: "curl -s ${#APP_URL}/db-status | jq -r '.status'"
        expected:
          result: "0"
          output: "connected"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[ \"$COMMAND_OUTPUT\" = \"connected\" ]"
      
      - step: 4
        description: "Verify environment configuration"
        command: "curl -s ${#APP_URL}/config | jq -r '.environment'"
        expected:
          result: "0"
          output: "${#ENVIRONMENT_NAME}"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[ \"$COMMAND_OUTPUT\" = \"${#ENVIRONMENT_NAME}\" ]"

hydration_vars:
  APP_URL:
    name: "APP_URL"
    description: "Application base URL"
    default_value: "http://localhost:3000"
    required: true
  EXPECTED_VERSION:
    name: "EXPECTED_VERSION"
    description: "Expected deployed version"
    default_value: "1.0.0"
    required: true
  ENVIRONMENT_NAME:
    name: "ENVIRONMENT_NAME"
    description: "Environment identifier"
    default_value: "development"
    required: true
```

#### CI/CD Pipeline Script (gitlab-ci.yml example)

```yaml
stages:
  - test
  - deploy
  - verify

# Development environment
test:dev:
  stage: test
  script:
    - test-executor validate-export ci_integration.yaml --export-file envs/dev.env
    - test-executor hydrate ci_integration.yaml --export-file envs/dev.env --output ci_dev.yaml
    - test-executor execute ci_dev.yaml
  only:
    - develop

# Staging environment
test:staging:
  stage: verify
  script:
    - test-executor validate-export ci_integration.yaml --export-file envs/staging.env
    - test-executor hydrate ci_integration.yaml --export-file envs/staging.env --output ci_staging.yaml
    - test-executor execute ci_staging.yaml
  only:
    - main

# Production environment
test:prod:
  stage: verify
  script:
    - test-executor validate-export ci_integration.yaml --export-file envs/prod.env
    - test-executor hydrate ci_integration.yaml --export-file envs/prod.env --output ci_prod.yaml
    - test-executor execute ci_prod.yaml
  only:
    - tags
  when: manual
```

#### Environment Files in Repository

```
project/
├── testcases/
│   └── ci_integration.yaml
├── envs/
│   ├── dev.env
│   ├── staging.env
│   └── prod.env
└── .gitlab-ci.yml
```

**envs/dev.env:**
```bash
export APP_URL=https://dev-app.example.com
export EXPECTED_VERSION=1.0.0-dev
export ENVIRONMENT_NAME=development
```

**envs/staging.env:**
```bash
export APP_URL=https://staging-app.example.com
export EXPECTED_VERSION=1.0.0-rc1
export ENVIRONMENT_NAME=staging
```

**envs/prod.env:**
```bash
export APP_URL=https://app.example.com
export EXPECTED_VERSION=1.0.0
export ENVIRONMENT_NAME=production
```

## Best Practices

### 1. Variable Naming Conventions

**Use UPPERCASE with underscores:**
```yaml
✓ GOOD:
  ${#SERVER_HOST}
  ${#API_KEY}
  ${#DB_CONNECTION_STRING}
  ${#TIMEOUT_SECONDS}

✗ BAD:
  ${#serverHost}      # lowercase not supported
  ${#api-key}         # hyphens not supported
  ${#db.connection}   # dots not supported
```

**Use descriptive, hierarchical names:**
```bash
# Group related variables with prefixes
export DB_HOST=localhost
export DB_PORT=5432
export DB_USER=admin
export DB_NAME=testdb

export API_URL=https://api.example.com
export API_KEY=secret123
export API_TIMEOUT=30

export AWS_REGION=us-east-1
export AWS_ACCESS_KEY=AKIAIOSFODNN7EXAMPLE
export AWS_SECRET_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
```

### 2. Organize Export Files

**Directory structure:**
```
project/
├── testcases/
│   ├── api_tests.yaml
│   ├── db_tests.yaml
│   └── integration_tests.yaml
├── envs/
│   ├── common.env           # Shared variables
│   ├── dev/
│   │   ├── api.env
│   │   ├── db.env
│   │   └── combined.env
│   ├── staging/
│   │   ├── api.env
│   │   ├── db.env
│   │   └── combined.env
│   └── prod/
│       ├── api.env
│       ├── db.env
│       └── combined.env
└── scripts/
    ├── generate_exports.sh
    └── validate_all.sh
```

**Combine multiple export files:**
```bash
#!/bin/bash
# scripts/generate_combined_env.sh

ENV=$1  # dev, staging, or prod

# Combine common and environment-specific variables
cat envs/common.env envs/${ENV}/api.env envs/${ENV}/db.env > envs/${ENV}/combined.env
```

### 3. Security Best Practices

**Never commit sensitive values:**
```bash
# .gitignore
envs/prod/*.env
envs/staging/*.env
*.secret.env
*_credentials.env
```

**Use templates for sensitive data:**
```bash
# envs/prod/api.env.template (committed to git)
export API_URL=https://api.example.com
export API_KEY=<REPLACE_WITH_ACTUAL_KEY>
export DB_PASSWORD=<REPLACE_WITH_ACTUAL_PASSWORD>

# envs/prod/api.env (NOT committed, created from template)
export API_URL=https://api.example.com
export API_KEY=sk_prod_actual_secret_key
export DB_PASSWORD=super_secure_password_123
```

**Use CI/CD secrets:**
```yaml
# GitLab CI example
test:prod:
  stage: verify
  script:
    # Create export file from CI/CD variables
    - echo "export API_KEY=$PROD_API_KEY" > prod.env
    - echo "export DB_PASSWORD=$PROD_DB_PASSWORD" >> prod.env
    - test-executor hydrate testcase.yaml --export-file prod.env --output testcase_prod.yaml
    - test-executor execute testcase_prod.yaml
```

### 4. Use hydration_vars Section

**Always define variables in test case:**
```yaml
hydration_vars:
  SERVER_HOST:
    name: "SERVER_HOST"
    description: "Server hostname or IP address"
    default_value: "localhost"
    required: true
  
  PORT:
    name: "PORT"
    description: "Server port number"
    default_value: "8080"
    required: false
  
  TIMEOUT:
    name: "TIMEOUT"
    description: "Connection timeout in seconds"
    default_value: "30"
    required: false
```

**Benefits:**
- Self-documenting test cases
- Automatic template generation
- Validation support
- Default values for optional variables

### 5. Validation in CI/CD

**Always validate before execution:**
```bash
#!/bin/bash
# scripts/test_with_validation.sh

TESTCASE=$1
ENV_FILE=$2
OUTPUT=$3

# Validate export file
if ! test-executor validate-export "$TESTCASE" --export-file "$ENV_FILE"; then
    echo "❌ Validation failed: $ENV_FILE"
    exit 1
fi

# Hydrate test case
if ! test-executor hydrate "$TESTCASE" --export-file "$ENV_FILE" --output "$OUTPUT"; then
    echo "❌ Hydration failed"
    exit 1
fi

# Execute test
if ! test-executor execute "$OUTPUT"; then
    echo "❌ Test execution failed"
    exit 1
fi

echo "✅ All tests passed"
```

### 6. Documentation

**Document variables in export files:**
```bash
# Environment: Production
# Updated: 2024-01-15
# Owner: DevOps Team
# Contact: devops@example.com

# ==========================================
# Server Configuration
# ==========================================
# Primary application server
export SERVER_HOST=prod.example.com

# HTTPS port
export SERVER_PORT=443

# Server protocol (http/https)
export SERVER_PROTOCOL=https

# ==========================================
# Authentication
# ==========================================
# Admin user for test execution
export USERNAME=test_admin

# Rotate monthly (last rotated: 2024-01-01)
export API_KEY=sk_prod_1234567890abcdef

# ==========================================
# Database Configuration
# ==========================================
# Read-only replica for testing
export DB_HOST=prod-replica.db.example.com
export DB_PORT=5432
export DB_NAME=production_db

# Read-only user (no write permissions)
export DB_USER=readonly_test_user
```

### 7. Version Control Strategy

**Track templates, not secrets:**
```bash
# Commit to git:
✓ testcases/*.yaml          # Test case templates
✓ envs/*.env.template        # Export file templates
✓ scripts/generate_exports.sh
✓ scripts/validate_all.sh
✓ .gitignore

# Do NOT commit:
✗ envs/prod/*.env            # Production credentials
✗ envs/staging/*.env         # Staging credentials
✗ *.secret.env               # Any secrets
✗ *_credentials.env          # Credential files
```

### 8. Testing Strategy

**Test with multiple environments:**
```bash
#!/bin/bash
# scripts/test_all_environments.sh

TESTCASE="testcases/api_test.yaml"

for ENV in dev staging prod; do
    echo "Testing $ENV environment..."
    
    # Generate export template if missing
    if [ ! -f "envs/${ENV}/api.env" ]; then
        test-executor generate-export "$TESTCASE" --output "envs/${ENV}/api.env.template"
        echo "⚠️  Please fill in values in envs/${ENV}/api.env.template"
        continue
    fi
    
    # Validate
    if ! test-executor validate-export "$TESTCASE" --export-file "envs/${ENV}/api.env"; then
        echo "❌ Validation failed for $ENV"
        continue
    fi
    
    # Hydrate
    test-executor hydrate "$TESTCASE" \
        --export-file "envs/${ENV}/api.env" \
        --output "testcases/api_test_${ENV}.yaml"
    
    # Execute
    if test-executor execute "testcases/api_test_${ENV}.yaml"; then
        echo "✅ $ENV tests passed"
    else
        echo "❌ $ENV tests failed"
    fi
    
    echo ""
done
```

## Advanced Use Cases

### 1. Dynamic Export File Generation

Generate export files programmatically from external sources:

```python
#!/usr/bin/env python3
# scripts/generate_from_vault.py

import os
import sys
import hvac  # HashiCorp Vault client

def generate_export_file(environment, output_file):
    # Connect to Vault
    client = hvac.Client(url=os.environ['VAULT_ADDR'])
    client.token = os.environ['VAULT_TOKEN']
    
    # Read secrets
    secrets = client.secrets.kv.v2.read_secret_version(
        path=f'testcases/{environment}'
    )['data']['data']
    
    # Generate export file
    with open(output_file, 'w') as f:
        f.write(f"# Generated from Vault: {environment}\n")
        f.write(f"# Timestamp: {datetime.now().isoformat()}\n\n")
        
        for key, value in sorted(secrets.items()):
            # Quote values with special characters
            if any(c in value for c in [' ', '"', '$', '\\']):
                escaped = value.replace('\\', '\\\\').replace('"', '\\"')
                f.write(f'export {key}="{escaped}"\n')
            else:
                f.write(f'export {key}={value}\n')
    
    print(f"✅ Generated: {output_file}")

if __name__ == '__main__':
    environment = sys.argv[1]
    output_file = sys.argv[2]
    generate_export_file(environment, output_file)
```

**Usage:**
```bash
# Generate from Vault
python scripts/generate_from_vault.py prod envs/prod/api.env

# Validate and execute
test-executor validate-export testcases/api_test.yaml --export-file envs/prod/api.env
test-executor hydrate testcases/api_test.yaml --export-file envs/prod/api.env --output api_test_prod.yaml
test-executor execute api_test_prod.yaml
```

### 2. Templating with Multiple Export Files

Combine multiple export files for complex configurations:

```bash
#!/bin/bash
# scripts/combine_exports.sh

ENV=$1
OUTPUT_FILE="envs/${ENV}/combined.env"

echo "# Combined export file for $ENV" > "$OUTPUT_FILE"
echo "# Generated: $(date)" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Combine in order of precedence (later files override earlier ones)
cat envs/common.env >> "$OUTPUT_FILE" 2>/dev/null
cat envs/${ENV}/base.env >> "$OUTPUT_FILE" 2>/dev/null
cat envs/${ENV}/secrets.env >> "$OUTPUT_FILE" 2>/dev/null

echo "✅ Combined export file: $OUTPUT_FILE"
```

### 3. Conditional Hydration

Use shell scripts to conditionally hydrate based on environment:

```bash
#!/bin/bash
# scripts/smart_hydrate.sh

TESTCASE=$1
ENV=${2:-dev}

# Select export file based on environment
case $ENV in
    dev)
        EXPORT_FILE="envs/dev.env"
        OUTPUT="testcases/test_dev.yaml"
        ;;
    staging)
        EXPORT_FILE="envs/staging.env"
        OUTPUT="testcases/test_staging.yaml"
        ;;
    prod)
        EXPORT_FILE="envs/prod.env"
        OUTPUT="testcases/test_prod.yaml"
        # Require confirmation for production
        read -p "⚠️  Execute in PRODUCTION? (yes/no): " CONFIRM
        if [ "$CONFIRM" != "yes" ]; then
            echo "Aborted"
            exit 1
        fi
        ;;
    *)
        echo "Unknown environment: $ENV"
        exit 1
        ;;
esac

# Validate, hydrate, execute
test-executor validate-export "$TESTCASE" --export-file "$EXPORT_FILE" && \
test-executor hydrate "$TESTCASE" --export-file "$EXPORT_FILE" --output "$OUTPUT" && \
test-executor execute "$OUTPUT"
```

### 4. Parallel Multi-Environment Testing

Test across all environments simultaneously:

```bash
#!/bin/bash
# scripts/parallel_test.sh

TESTCASE=$1

# Function to test one environment
test_environment() {
    local env=$1
    local export_file="envs/${env}.env"
    local output="testcases/test_${env}.yaml"
    
    echo "🔄 Testing $env..."
    
    if test-executor validate-export "$TESTCASE" --export-file "$export_file" && \
       test-executor hydrate "$TESTCASE" --export-file "$export_file" --output "$output" && \
       test-executor execute "$output"; then
        echo "✅ $env: PASSED"
        return 0
    else
        echo "❌ $env: FAILED"
        return 1
    fi
}

# Export function for parallel execution
export -f test_environment

# Test all environments in parallel
parallel -j 3 test_environment ::: dev staging prod

# Check results
echo ""
echo "All environments tested"
```

## Troubleshooting

### Common Issues

**Issue 1: Placeholder not replaced**
```
Problem: ${#VAR_NAME} still appears in hydrated output
Solution: 
  - Check variable name spelling in export file
  - Verify variable name is UPPERCASE
  - Ensure export file is being loaded correctly
  - Use validate-export to check for missing variables
```

**Issue 2: Validation fails with "missing required variables"**
```
Problem: validate-export reports missing variables that exist in export file
Solution:
  - Check for typos in variable names (case-sensitive)
  - Verify export file syntax: export VAR_NAME=value
  - Ensure no extra spaces: "export VAR=value" not "export VAR = value"
  - Check file encoding (should be UTF-8)
```

**Issue 3: Special characters cause issues**
```
Problem: Values with special characters break hydration
Solution:
  - Use double quotes: export VAR="value with spaces"
  - Escape special characters: \" \$ \\ \`
  - For literal strings, use single quotes: export VAR='literal$value'
```

**Issue 4: Generated script doesn't source export file**
```
Problem: Bash script doesn't load environment variables
Solution:
  - Check that YAML contains ${#VAR} placeholders
  - Verify test case has hydration_vars section
  - Generated scripts look for <TC_ID>.env file by default
  - Ensure export file matches naming convention: TC_XXXX.env
```

## See Also

- [Variables and Data Passing Documentation](VARIABLE_PASSING.md) - For runtime variable capture and substitution
- [Test Executor Usage](../README.md#test-executor) - Complete test-executor command reference
- [BDD Initial Conditions](BDD_INITIAL_CONDITIONS.md) - For test prerequisites
- [CI/CD Examples](GITLAB_CI_EXAMPLES.md) - Integration with CI/CD pipelines
