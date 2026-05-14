# Requirement Coverage String-Based Verification

## Overview

The `req-coverage` tool supports string-based requirement coverage verification, which allows you to verify that test cases accurately cover specific portions of requirement text. This feature enables:

1. **Validation**: Ensures that test case `covers` strings actually exist in the requirement text
2. **Cumulative Coverage Analysis**: Tracks which portions of requirement text are covered across all test cases
3. **Automatic Coverage Determination**: Automatically determines if a requirement is fully or partially covered based on string coverage
4. **Error Detection**: Reports when test cases claim to cover text not found in the requirement

## How It Works

### Basic Workflow

1. **Define Requirements**: Create a requirements definition file (YAML or JSON) containing requirement IDs and their full text
2. **Specify Coverage in Test Cases**: Use the `covers` field in test case YAML to specify the exact substring of the requirement covered
3. **Run Verification**: Execute `req-coverage verify` with the `--requirements-file` option
4. **Review Results**: The tool validates coverage strings, cumulates coverage, and reports errors

### Coverage Verification Logic

When you provide a requirements file, the tool performs the following checks for each test case:

1. **Substring Validation**: Checks if the `covers` string is a substring of the requirement text
2. **Coverage Accumulation**: Collects all `covers` strings from all test cases for each requirement
3. **Full Coverage Determination**: Removes all covered portions from the requirement text and checks if anything remains
4. **Error Reporting**: Logs errors for `covers` strings not found in requirement text

### Example

**Requirement (REQ-001):**
```
The system shall authenticate users with valid credentials and deny access to users with invalid credentials.
```

**Test Case 1:**
```yaml
requirement: REQ-001
requirement_coverage:
  type: partial
  covers: "authenticate users with valid credentials"
```

**Test Case 2:**
```yaml
requirement: REQ-001
requirement_coverage:
  type: partial
  covers: "deny access to users with invalid credentials"
```

**Result:**
- Both `covers` strings are validated against the requirement text ✓
- Cumulative coverage: ["authenticate users with valid credentials", "deny access to users with invalid credentials"]
- Remaining text after removing covered portions: "The system shall and"
- Coverage status: **Partial** (because "The system shall" and "and" are not covered)

To achieve full coverage, you would need additional test cases covering:
```yaml
covers: "The system shall"
covers: "and"
```

## Requirements Definition File Format

### YAML Format

**File: `requirements.yaml`**
```yaml
requirements:
  - id: REQ-001
    text: "The system shall authenticate users with valid credentials and deny access to users with invalid credentials."
    description: "User authentication requirement"
  
  - id: REQ-002
    text: "The system shall allow password reset via email and log all password reset attempts."
    description: "Password reset requirement"
```

**Field Descriptions:**
- `id` (required): Unique requirement identifier
- `text` (required): Full text of the requirement
- `description` (optional): Human-readable description

### JSON Format

**File: `requirements.json`**
```json
{
  "requirements": [
    {
      "id": "REQ-001",
      "text": "The system shall authenticate users with valid credentials and deny access to users with invalid credentials.",
      "description": "User authentication requirement"
    },
    {
      "id": "REQ-002",
      "text": "The system shall allow password reset via email and log all password reset attempts.",
      "description": "Password reset requirement"
    }
  ]
}
```

## Test Case Coverage Specification

### Partial Coverage with String Verification

```yaml
type: test_case
requirement: REQ-001
requirement_coverage:
  type: partial
  covers: "authenticate users with valid credentials"

test_sequences:
  - id: 1
    name: Valid Credential Authentication
    steps:
      - step: 1
        description: Login with valid credentials
        command: login_test valid_user valid_pass
        # ...
```

### Multiple Test Cases for Same Requirement

```yaml
# Test case 1 - Covers authentication aspect
requirement: REQ-001
requirement_coverage:
  type: partial
  covers: "authenticate users with valid credentials"

---
# Test case 2 - Covers denial aspect
requirement: REQ-001
requirement_coverage:
  type: partial
  covers: "deny access to users with invalid credentials"
```

The tool will cumulate these `covers` strings and determine overall coverage.

## Using the Feature

### Command Line Usage

```bash
req-coverage verify \
  --test-cases-folder ./testcases \
  --test-results-folder ./test-results \
  --output coverage.json \
  --requirements-file ./requirements.yaml
```

**Options:**
- `--requirements-file`: Path to YAML or JSON file containing requirement definitions (optional)

### Without Requirements File

If you don't provide `--requirements-file`, the tool operates in the original mode:
- No string validation is performed
- Coverage type is taken from the test case's `requirement_coverage.type` field
- No cumulative coverage analysis

### With Requirements File

When `--requirements-file` is provided:
- All `covers` strings are validated against requirement text
- Coverage type is automatically determined based on cumulative string coverage
- Errors are reported for invalid `covers` strings
- Coverage report includes requirement text, covered portions, and errors

## Coverage Report Output

### JSON Report Structure

```json
{
  "generated_at": "2024-01-20T10:30:00Z",
  "total_requirements": 2,
  "fully_covered_requirements": 1,
  "partially_covered_requirements": 1,
  "uncovered_requirements": 0,
  "requirements": [
    {
      "requirement_id": "REQ-001",
      "coverage_type": "full",
      "test_cases": [
        {
          "test_case_id": "TC-001",
          "status": "pass",
          "covers": "authenticate users with valid credentials",
          "description": "Test valid credentials"
        },
        {
          "test_case_id": "TC-002",
          "status": "pass",
          "covers": "deny access to users with invalid credentials",
          "description": "Test invalid credentials"
        }
      ],
      "status": "covered_pass",
      "requirement_text": "The system shall authenticate users with valid credentials and deny access to users with invalid credentials.",
      "covered_portions": [
        "authenticate users with valid credentials",
        "deny access to users with invalid credentials"
      ],
      "coverage_errors": null
    },
    {
      "requirement_id": "REQ-002",
      "coverage_type": "partial",
      "test_cases": [
        {
          "test_case_id": "TC-003",
          "status": "pass",
          "covers": "password reset via email",
          "description": "Test email password reset"
        }
      ],
      "status": "partial_covered_pass",
      "requirement_text": "The system shall allow password reset via email and log all password reset attempts.",
      "covered_portions": [
        "password reset via email"
      ],
      "coverage_errors": null
    }
  ]
}
```

### HTML Report Features

The HTML report displays additional information when using requirement definitions:

1. **Requirement Text**: Shows the full text of the requirement in a blue box
2. **Covered Portions**: Lists all `covers` strings from test cases in a green box
3. **Coverage Errors**: Displays any validation errors in a red box
4. **Automatic Coverage Type**: Coverage type is determined by the tool based on string coverage

**Example HTML Output:**

```
Requirement ID: REQ-001
Coverage Type: Full
Status: Covered (All Passed)

Requirement Text:
  The system shall authenticate users with valid credentials and deny access to users with invalid credentials.

Covered Portions:
  • authenticate users with valid credentials
  • deny access to users with invalid credentials

Test Cases:
  TC-001 (Pass): Test valid credentials
    Covers: authenticate users with valid credentials
  TC-002 (Pass): Test invalid credentials
    Covers: deny access to users with invalid credentials
```

## Error Handling

### Error Case 1: String Not Found in Requirement

**Test Case:**
```yaml
requirement: REQ-001
requirement_coverage:
  type: partial
  covers: "invalid substring that doesn't exist"
```

**Result:**
- Error logged: "Test case TC-XXX claims to cover 'invalid substring that doesn't exist' which is not found in requirement REQ-001"
- Coverage error added to the report
- Test case is still counted but flagged with error

### Error Case 2: Requirement Definition Not Found

**Test Case:**
```yaml
requirement: REQ-999
```

**Result:**
- Error logged: "Requirement definition not found for ID: REQ-999"
- Coverage error added to the report
- Normal coverage analysis continues without string verification

## Best Practices

### 1. Write Clear Requirement Text

✓ **Good:**
```yaml
text: "The system shall authenticate users with valid credentials and deny access to users with invalid credentials."
```

✗ **Avoid:**
```yaml
text: "System does auth stuff and denies bad logins, etc."
```

### 2. Use Exact Substrings in Covers

✓ **Good:**
```yaml
covers: "authenticate users with valid credentials"
```

✗ **Avoid:**
```yaml
covers: "auth valid creds"  # Won't match requirement text
```

### 3. Cover All Requirement Text

For full coverage, ensure all parts of the requirement text are covered:

```yaml
# Requirement: "The system shall validate input and sanitize output."

# Test 1
covers: "validate input"

# Test 2
covers: "sanitize output"

# Missing: "The system shall" and "and"
```

### 4. Organize Requirements by Atomicity

Break down complex requirements into atomic parts:

✗ **Avoid:**
```yaml
text: "The system shall do A, B, C, D, and E with conditions X, Y, Z."
```

✓ **Better:**
```yaml
- id: REQ-001
  text: "The system shall do A with condition X."
- id: REQ-002
  text: "The system shall do B with condition Y."
```

### 5. Version Control Requirements

Keep requirements definitions in version control alongside test cases:
```
project/
├── requirements.yaml
├── testcases/
│   ├── test_req_001.yaml
│   └── test_req_002.yaml
└── test-results/
```

## Advanced Usage

### Covering Multiple Requirements

A single test case can cover portions of multiple requirements:

```yaml
requirement: AUTH-001
requirement_coverage:
  type: partial
  covers: "verify username and password"
  additional_requirements:
    - SEC-005  # Security logging
```

This test case will:
1. Verify "verify username and password" exists in AUTH-001 text
2. Add this test case to SEC-005's coverage (but no string verification for SEC-005)

### Programmatic Access

Parse the JSON coverage report to build custom tools:

```python
import json

with open('coverage.json') as f:
    report = json.load(f)

for req in report['requirements']:
    if req.get('coverage_errors'):
        print(f"Errors in {req['requirement_id']}:")
        for error in req['coverage_errors']:
            print(f"  - {error}")
```

### CI/CD Integration

```yaml
# GitLab CI example
coverage-check:
  script:
    - cargo build --release -p req-coverage
    - ./target/release/req-coverage verify 
        --test-cases-folder testcases 
        --test-results-folder results 
        --output coverage.json
        --requirements-file requirements.yaml
    - |
      # Fail if there are coverage errors
      if jq '.requirements[].coverage_errors | select(. != null)' coverage.json | grep -q .; then
        echo "Coverage errors found!"
        exit 1
      fi
```

## Troubleshooting

### Issue: Covers string not found in requirement

**Error:** `Test case TC-001 claims to cover 'xyz' which is not found in requirement REQ-001`

**Solution:** 
- Check that the `covers` string is an exact substring (case-sensitive)
- Verify the requirement text in `requirements.yaml`
- Ensure there are no typos or extra/missing spaces

### Issue: Coverage shows partial when it should be full

**Symptom:** All parts seem covered but status is "partial"

**Cause:** Some text is not covered (punctuation, conjunctions, etc.)

**Solution:**
- Review the "Covered Portions" in the HTML report
- Check which parts of the requirement text are missing
- Add test cases to cover connecting words if needed

### Issue: Requirement definition not found

**Error:** `Requirement definition not found for ID: REQ-XXX`

**Solution:**
- Verify the requirement ID exists in `requirements.yaml`
- Check for typos in the requirement ID
- Ensure the requirements file is being loaded correctly

## Schema Reference

### RequirementDefinition

```typescript
interface RequirementDefinition {
  id: string;                    // Unique requirement ID
  text: string;                  // Full requirement text
  description?: string;          // Optional description
}
```

### RequirementDefinitions

```typescript
interface RequirementDefinitions {
  requirements: RequirementDefinition[];
}
```

### Coverage Report Extensions

```typescript
interface RequirementCoverageItem {
  requirement_id: string;
  coverage_type: "full" | "partial";
  test_cases: TestCaseResult[];
  status: CoverageStatus;
  
  // New fields when using requirements file
  requirement_text?: string;         // Full requirement text
  covered_portions?: string[];       // All covers strings
  coverage_errors?: string[];        // Validation errors
}
```

## Examples

See example files in `crates/req-coverage/templates/`:
- `requirements.example.yaml` - YAML format example
- `requirements.example.json` - JSON format example

## Related Documentation

- [README.md](../crates/req-coverage/README.md) - General usage guide
- [PRD_REQ_COVERAGE.md](PRD_REQ_COVERAGE.md) - Product requirements
- [REQ_COVERAGE_ENHANCEMENTS.md](REQ_COVERAGE_ENHANCEMENTS.md) - Feature enhancements
