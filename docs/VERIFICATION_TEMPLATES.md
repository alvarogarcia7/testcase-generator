# Verification Expression Templates

This document describes the verification expression template library and how to use it for creating test case verification expressions.

## Overview

The verification template library provides pre-built, tested patterns for common verification scenarios. Instead of manually writing verification expressions for each test step, you can select from a library of templates that cover:

- HTTP status codes
- Exit codes
- String matching patterns
- JSON validation
- Regex patterns

## Template Structure

Each template includes:

- **ID**: Unique identifier for the template
- **Name**: Human-readable name
- **Description**: Detailed explanation of what the template verifies
- **Category**: Organizational category (HTTP Status, Exit Code, etc.)
- **Result Expression**: Shell expression to verify the result
- **Output Expression**: Shell expression to verify the output
- **Examples**: Usage scenarios
- **Variables**: Placeholder variables that can be substituted

## Available Template Categories

### 1. HTTP Status Codes

Templates for verifying HTTP response status codes.

#### http_success - HTTP Success (2xx)
- **Description**: Verify HTTP response has success status code (200-299)
- **Result**: `[[ $HTTP_STATUS -ge 200 && $HTTP_STATUS -lt 300 ]]`
- **Output**: `cat $COMMAND_OUTPUT | grep -q "${OUTPUT}"`
- **Variables**: OUTPUT
- **Example**: Verify API endpoint returns 200 OK

#### http_200 - HTTP 200 OK
- **Description**: Verify HTTP response is exactly 200 OK
- **Result**: `[[ $HTTP_STATUS -eq 200 ]]`
- **Output**: `cat $COMMAND_OUTPUT | grep -q "${OUTPUT}"`
- **Variables**: OUTPUT
- **Example**: Verify successful GET request

#### http_201 - HTTP 201 Created
- **Description**: Verify HTTP response is 201 Created
- **Result**: `[[ $HTTP_STATUS -eq 201 ]]`
- **Output**: `cat $COMMAND_OUTPUT | grep -q "${OUTPUT}"`
- **Variables**: OUTPUT
- **Example**: Verify resource creation

#### http_204 - HTTP 204 No Content
- **Description**: Verify HTTP response is 204 No Content
- **Result**: `[[ $HTTP_STATUS -eq 204 ]]`
- **Output**: `[[ -z "$COMMAND_OUTPUT" ]]`
- **Example**: Verify DELETE operation with no content

#### http_400 - HTTP 400 Bad Request
- **Description**: Verify HTTP response is 400 Bad Request
- **Result**: `[[ $HTTP_STATUS -eq 400 ]]`
- **Output**: `cat $COMMAND_OUTPUT | grep -q "${OUTPUT}"`
- **Variables**: OUTPUT
- **Example**: Verify invalid request handling

#### http_404 - HTTP 404 Not Found
- **Description**: Verify HTTP response is 404 Not Found
- **Result**: `[[ $HTTP_STATUS -eq 404 ]]`
- **Output**: `cat $COMMAND_OUTPUT | grep -q "${OUTPUT}"`
- **Variables**: OUTPUT
- **Example**: Verify resource not found

### 2. Exit Codes

Templates for verifying command exit codes.

#### exit_success - Exit Code 0 (Success)
- **Description**: Verify command exits with success code 0
- **Result**: `[[ $? -eq 0 ]]`
- **Output**: `cat $COMMAND_OUTPUT | grep -q "${OUTPUT}"`
- **Variables**: OUTPUT
- **Example**: Verify successful command execution

#### exit_failure - Exit Code Non-Zero (Failure)
- **Description**: Verify command exits with any non-zero code
- **Result**: `[[ $? -ne 0 ]]`
- **Output**: `cat $COMMAND_OUTPUT | grep -q "${OUTPUT}"`
- **Variables**: OUTPUT
- **Example**: Verify expected failure

#### exit_code_custom - Exit Code (Custom)
- **Description**: Verify command exits with specific exit code
- **Result**: `[[ $? -eq ${EXIT_CODE} ]]`
- **Output**: `cat $COMMAND_OUTPUT | grep -q "${OUTPUT}"`
- **Variables**: EXIT_CODE, OUTPUT
- **Example**: Verify specific exit code like 127 for command not found

### 3. String Matching

Templates for string comparison and matching.

#### string_exact - Exact String Match
- **Description**: Verify output exactly matches expected string
- **Result**: `[[ "$RESULT" == "${EXPECTED}" ]]`
- **Output**: `[[ "$COMMAND_OUTPUT" == "${OUTPUT}" ]]`
- **Variables**: EXPECTED, OUTPUT
- **Example**: Verify exact output match

#### string_contains - String Contains
- **Description**: Verify output contains expected substring
- **Result**: `[[ "$RESULT" == *"${EXPECTED}"* ]]`
- **Output**: `cat $COMMAND_OUTPUT | grep -q "${OUTPUT}"`
- **Variables**: EXPECTED, OUTPUT
- **Example**: Verify output contains specific text

#### string_starts_with - String Starts With
- **Description**: Verify output starts with expected prefix
- **Result**: `[[ "$RESULT" == "${EXPECTED}"* ]]`
- **Output**: `cat $COMMAND_OUTPUT | grep -q "^${OUTPUT}"`
- **Variables**: EXPECTED, OUTPUT
- **Example**: Verify output prefix

#### string_ends_with - String Ends With
- **Description**: Verify output ends with expected suffix
- **Result**: `[[ "$RESULT" == *"${EXPECTED}" ]]`
- **Output**: `cat $COMMAND_OUTPUT | grep -q "${OUTPUT}$"`
- **Variables**: EXPECTED, OUTPUT
- **Example**: Verify output suffix

#### string_empty - Empty String
- **Description**: Verify output is empty
- **Result**: `[[ -z "$RESULT" ]]`
- **Output**: `[[ -z "$COMMAND_OUTPUT" ]]`
- **Example**: Verify no output produced

#### string_not_empty - Non-Empty String
- **Description**: Verify output is not empty
- **Result**: `[[ -n "$RESULT" ]]`
- **Output**: `[[ -n "$COMMAND_OUTPUT" ]]`
- **Example**: Verify some output was produced

### 4. JSON Validation

Templates for validating JSON responses.

#### json_valid - Valid JSON
- **Description**: Verify output is valid JSON
- **Result**: `echo "$RESULT" | jq empty`
- **Output**: `cat $COMMAND_OUTPUT | jq empty`
- **Example**: Verify API returns valid JSON

#### json_field_exists - JSON Field Exists
- **Description**: Verify JSON output contains a specific field
- **Result**: `echo "$RESULT" | jq -e '.${FIELD}' > /dev/null`
- **Output**: `cat $COMMAND_OUTPUT | jq -e '.${FIELD}' > /dev/null`
- **Variables**: FIELD
- **Example**: Verify JSON has 'status' field

#### json_field_value - JSON Field Value
- **Description**: Verify JSON field has expected value
- **Result**: `echo "$RESULT" | jq -e '.${FIELD} == "${VALUE}"' > /dev/null`
- **Output**: `cat $COMMAND_OUTPUT | jq -e '.${FIELD} == "${VALUE}"' > /dev/null`
- **Variables**: FIELD, VALUE
- **Example**: Verify JSON field 'status' equals 'success'

#### json_array_length - JSON Array Length
- **Description**: Verify JSON array has expected length
- **Result**: `echo "$RESULT" | jq -e '.${FIELD} | length == ${LENGTH}' > /dev/null`
- **Output**: `cat $COMMAND_OUTPUT | jq -e 'length == ${LENGTH}' > /dev/null`
- **Variables**: FIELD, LENGTH
- **Example**: Verify array has 3 elements

### 5. Regex Patterns

Templates for pattern matching with regular expressions.

#### regex_match - Regex Pattern Match
- **Description**: Verify output matches regular expression
- **Result**: `echo "$RESULT" | grep -qE "${PATTERN}"`
- **Output**: `cat $COMMAND_OUTPUT | grep -qE "${PATTERN}"`
- **Variables**: PATTERN
- **Example**: Verify output matches pattern

#### regex_email - Email Address
- **Description**: Verify output contains valid email address
- **Result**: `echo "$RESULT" | grep -qE "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}"`
- **Output**: `cat $COMMAND_OUTPUT | grep -qE "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}"`
- **Example**: Verify output contains email address

#### regex_ipv4 - IPv4 Address
- **Description**: Verify output contains valid IPv4 address
- **Result**: `echo "$RESULT" | grep -qE "([0-9]{1,3}\.){3}[0-9]{1,3}"`
- **Output**: `cat $COMMAND_OUTPUT | grep -qE "([0-9]{1,3}\.){3}[0-9]{1,3}"`
- **Example**: Verify output contains IP address

#### regex_uuid - UUID
- **Description**: Verify output contains valid UUID
- **Result**: `echo "$RESULT" | grep -qiE "[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}"`
- **Output**: `cat $COMMAND_OUTPUT | grep -qiE "[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}"`
- **Example**: Verify output contains UUID

#### regex_hexadecimal - Hexadecimal Pattern
- **Description**: Verify output matches hexadecimal pattern
- **Result**: `echo "$RESULT" | grep -qiE "^(0x)?[0-9a-f]+$"`
- **Output**: `cat $COMMAND_OUTPUT | grep -qiE "(0x)?[0-9a-f]+"`
- **Example**: Verify hex value like 0x9000

## Using Templates Interactively

When creating test steps interactively, the system will prompt you to use verification templates:

```
=== Verification Expressions ===
Use a verification template? [y/N]: y

Available template categories:
> Exit Codes
  HTTP Status Codes
  JSON Validation
  Regex Patterns
  String Matching

Select template category (ESC to see all):
```

After selecting a category, you'll see templates in that category:

```
Select verification template (ESC to skip):
> Exit Code 0 (Success) - Verify command exits with success code 0
  Exit Code Non-Zero (Failure) - Verify command exits with any non-zero code
  Exit Code (Custom) - Verify command exits with specific exit code
```

After selecting a template, you'll see its details:

```
✓ Selected template: Exit Code 0 (Success)
  Description: Verify command exits with success code 0

  This template uses the following variables:
    - ${OUTPUT}

  Examples:
    • Verify successful command execution

  Result expression: [[ $? -eq 0 ]]
  Output expression: cat $COMMAND_OUTPUT | grep -q "${OUTPUT}"

Use template as-is? [y/N]:
```

You can either use the template as-is or edit the expressions before applying.

## Programmatic Usage

### Creating and Using Templates

```rust
use testcase_manager::{VerificationTemplate, TemplateCategory, VerificationTemplateLibrary};
use std::collections::HashMap;

// Create a library of templates
let library = VerificationTemplateLibrary::new();

// Get a specific template
let template = library.get_template("exit_success").unwrap();

// Use template without substitutions
let verification = template.expand_default();
println!("Result: {}", verification.result);
println!("Output: {}", verification.output);

// Use template with variable substitutions
let mut substitutions = HashMap::new();
substitutions.insert("OUTPUT".to_string(), "Success".to_string());
let verification = template.expand(&substitutions);
```

### Getting Templates by Category

```rust
use testcase_manager::{VerificationTemplateLibrary, TemplateCategory};

let library = VerificationTemplateLibrary::new();

// Get all HTTP status templates
let http_templates = library.get_templates_by_category(&TemplateCategory::HttpStatus);

for template in http_templates {
    println!("{}: {}", template.name, template.description);
}
```

### Creating Custom Templates

```rust
use testcase_manager::{VerificationTemplate, TemplateCategory};

let custom_template = VerificationTemplate::new(
    "custom_check",
    "Custom Verification",
    "My custom verification pattern",
    TemplateCategory::Custom,
    "[[ $CUSTOM_VAR -eq ${EXPECTED} ]]",
    "cat $COMMAND_OUTPUT | grep -q \"${TEXT}\"",
)
.with_variable("EXPECTED")
.with_variable("TEXT")
.with_example("Check custom variable equals expected value");

// Add to library
let mut library = VerificationTemplateLibrary::new();
library.add_template(custom_template);
```

## Best Practices

1. **Choose the Right Template**: Select templates that match your verification needs. For example, use HTTP status templates for API testing, exit code templates for CLI commands.

2. **Use Variables**: Templates with variables (like `${OUTPUT}`) are more flexible. You can substitute values as needed.

3. **Edit When Needed**: Don't hesitate to edit template expressions to fit your exact needs. Templates are starting points.

4. **Combine Templates**: For complex verifications, you might need multiple checks. Consider using multiple templates or combining their expressions.

5. **Create Custom Templates**: For frequently used patterns not in the library, create custom templates and add them to your local library.

6. **Document Verification Logic**: When using custom or modified templates, document why you chose that verification approach in step descriptions.

## Examples

### Example 1: Verifying API Success Response

```yaml
- step: 1
  description: "Call user API endpoint"
  command: "curl -X GET https://api.example.com/users"
  expected:
    result: "200"
    output: "user data"
  verification:
    result: "[[ $HTTP_STATUS -eq 200 ]]"
    output: "cat $COMMAND_OUTPUT | grep -q \"user data\""
```

Template used: `http_200` (HTTP 200 OK)

### Example 2: Verifying Command Exit Code

```yaml
- step: 2
  description: "Run deployment script"
  command: "./deploy.sh"
  expected:
    result: "0"
    output: "Deployment successful"
  verification:
    result: "[[ $? -eq 0 ]]"
    output: "cat $COMMAND_OUTPUT | grep -q \"Deployment successful\""
```

Template used: `exit_success` (Exit Code 0)

### Example 3: Verifying JSON Response Field

```yaml
- step: 3
  description: "Get user profile JSON"
  command: "curl https://api.example.com/profile"
  expected:
    result: "has_email_field"
    output: "valid_json"
  verification:
    result: "echo \"$RESULT\" | jq -e '.email' > /dev/null"
    output: "cat $COMMAND_OUTPUT | jq empty"
```

Template used: `json_field_exists` (JSON Field Exists) with variable `FIELD=email`

### Example 4: Verifying String Contains Pattern

```yaml
- step: 4
  description: "Check log file for error"
  command: "cat /var/log/app.log"
  expected:
    result: "contains_error"
    output: "Error: Connection timeout"
  verification:
    result: "[[ \"$RESULT\" == *\"Error\"* ]]"
    output: "cat $COMMAND_OUTPUT | grep -q \"Error: Connection timeout\""
```

Template used: `string_contains` (String Contains)

### Example 5: Verifying Hex Pattern (Smart Card)

```yaml
- step: 5
  description: "Send APDU command to smart card"
  command: "send_apdu SELECT_FILE"
  expected:
    result: "SW=0x9000"
    output: "Success"
  verification:
    result: "echo \"$RESULT\" | grep -qiE \"(0x)?[0-9a-f]+\""
    output: "cat $COMMAND_OUTPUT | grep -q \"Success\""
```

Template used: `regex_hexadecimal` (Hexadecimal Pattern)

## Environment Variables Used in Templates

Templates reference standard environment variables that should be set by the test execution framework:

- `$?` - Exit code of the last command
- `$RESULT` - The result value being verified
- `$COMMAND_OUTPUT` - Full output from the command execution
- `$HTTP_STATUS` - HTTP status code (for HTTP requests)

Additional variables can be defined as needed for your specific test environment.

## Troubleshooting

### Template Not Found
If a template ID is not recognized, check the library initialization. You can list all available templates:

```rust
let library = VerificationTemplateLibrary::new();
let ids = library.get_template_ids();
println!("Available templates: {:?}", ids);
```

### Variable Substitution Not Working
Ensure variable names match exactly (case-sensitive). Variables should be in format `${VAR_NAME}`.

### Verification Expression Failing
Test your verification expressions manually in a shell first. Make sure environment variables are set correctly during test execution.

## Further Reading

- See `QUICK_REFERENCE.md` for general test case creation guidance
- See `examples/` directory for complete test case examples using templates
- Refer to the API documentation for detailed method signatures
