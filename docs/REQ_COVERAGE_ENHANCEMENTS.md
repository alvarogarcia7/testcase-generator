# Requirement Coverage Tool - Enhancement Guide

This document describes the enhancements made to the `req-coverage` tool to support requirement coverage specification in test cases and custom HTML templates.

## Enhancement 1: Requirement Coverage in Test Cases

### Overview

Test cases can now explicitly specify how they cover requirements using the `requirement_coverage` field in their YAML definition. This allows for:
- Full vs. partial coverage specification
- Description of what aspects are covered
- Coverage of multiple requirements by a single test case

### YAML Structure

```yaml
requirement: PRIMARY-REQ-ID
requirement_coverage:
  type: full | partial
  covers: "Description of what is covered (optional, for partial)"
  additional_requirements:  # Optional list
    - ADDITIONAL-REQ-1
    - ADDITIONAL-REQ-2
```

### Full Coverage Example

```yaml
type: test_case
requirement: AUTH-001
requirement_coverage:
  type: full

test_sequences:
  - id: 1
    name: Complete Authentication Test
    # ... test steps that cover entire AUTH-001 requirement
```

### Partial Coverage Example

```yaml
type: test_case
requirement: AUTH-002
requirement_coverage:
  type: partial
  covers: "Password reset via email workflow"

test_sequences:
  - id: 1
    name: Email Password Reset
    # ... test steps that only cover email-based password reset
```

### Multiple Requirements Example

```yaml
type: test_case
requirement: AUTH-002
requirement_coverage:
  type: partial
  covers: "Password reset via email"
  additional_requirements:
    - AUTH-003  # Email notification requirement
    - SEC-005   # Security logging requirement

test_sequences:
  - id: 1
    name: Password Reset with Security Logging
    # ... test steps that cover multiple requirements
```

### Behavior

**Without `requirement_coverage` field:**
- Defaults to `type: full`
- Test case is counted as full coverage for the primary requirement
- No additional requirements are covered

**With `requirement_coverage` field:**
- Uses specified type (full or partial)
- Uses `covers` description in reports
- Adds test case to coverage for all `additional_requirements`

### Data Model

**In `crates/testcase-models/src/lib.rs`:**

```rust
pub enum RequirementCoverageType {
    Full,
    Partial,
}

pub struct RequirementCoverageSpec {
    pub coverage_type: RequirementCoverageType,
    pub covers: Option<String>,
    pub additional_requirements: Option<Vec<String>>,
}

pub struct TestCase {
    pub requirement: String,
    pub requirement_coverage: Option<RequirementCoverageSpec>,
    // ... other fields
}
```

## Enhancement 2: Custom HTML Templates

### Overview

The HTML report generator now supports custom templates, allowing organizations to customize the appearance and branding of coverage reports.

### Usage

```bash
req-coverage print \
  --format html \
  --input coverage.json \
  --output ./report/ \
  --template ./my-custom-template.html
```

### Template Placeholders

Templates use double-brace syntax for placeholders:

| Placeholder | Description | Example Value |
|-------------|-------------|---------------|
| `{{GENERATED_AT}}` | Report generation timestamp | `2024-01-15 10:30:00 UTC` |
| `{{TOTAL_REQUIREMENTS}}` | Total number of requirements | `42` |
| `{{FULLY_COVERED}}` | Number of fully covered requirements | `30` |
| `{{PARTIALLY_COVERED}}` | Number of partially covered requirements | `8` |
| `{{UNCOVERED}}` | Number of uncovered requirements | `4` |
| `{{REQUIREMENTS_ROWS}}` | HTML table rows with requirement details | `<tr>...</tr>` |

### Template Structure

```html
<!DOCTYPE html>
<html>
<head>
    <title>My Custom Coverage Report</title>
    <style>
        /* Your custom styles */
    </style>
</head>
<body>
    <h1>Coverage Report</h1>
    <p>Generated: {{GENERATED_AT}}</p>
    
    <div class="stats">
        <div>Total: {{TOTAL_REQUIREMENTS}}</div>
        <div>Fully Covered: {{FULLY_COVERED}}</div>
        <div>Partially Covered: {{PARTIALLY_COVERED}}</div>
        <div>Uncovered: {{UNCOVERED}}</div>
    </div>
    
    <table>
        <thead>
            <tr>
                <th>Requirement</th>
                <th>Type</th>
                <th>Status</th>
                <th>Tests</th>
                <th>Pass/Fail</th>
            </tr>
        </thead>
        <tbody>
            {{REQUIREMENTS_ROWS}}
        </tbody>
    </table>
    
    <script>
        /* Your custom JavaScript */
    </script>
</body>
</html>
```

### Example Template

See `docs/examples/html_template_example.html` for a complete working example with:
- Custom color scheme
- Modified layout
- Different typography
- Custom interactive elements

### Requirements Row Format

The `{{REQUIREMENTS_ROWS}}` placeholder is replaced with HTML like:

```html
<tr>
    <td class="expandable" data-details="details-0">REQ-001</td>
    <td><span class="coverage-type">Full</span></td>
    <td><span class="status-badge green">Covered (All Passed)</span></td>
    <td>3</td>
    <td>3 / 0</td>
</tr>
<tr>
    <td colspan="5" class="details" id="details-0">
        <div class="test-cases">
            <div class="test-case-item pass">
                <span class="test-case-id">TC-001</span>
                <span class="test-case-description">Test description</span>
            </div>
        </div>
    </td>
</tr>
```

### Styling Classes

The following CSS classes are used in the generated rows:

**Status Badges:**
- `.status-badge.green` - Fully covered, all tests passed
- `.status-badge.red` - Fully covered, some tests failed
- `.status-badge.yellow` - Partially covered, all tests passed
- `.status-badge.orange` - Partially covered, some tests failed
- `.status-badge.gray` - No coverage

**Test Case Items:**
- `.test-case-item.pass` - Passed test
- `.test-case-item.fail` - Failed test
- `.test-case-item.not-executed` - Not executed test

**Interactive Elements:**
- `.expandable` - Clickable requirement row
- `.expanded` - Expanded state
- `.details` - Collapsible details section
- `.details.show` - Visible details

### Best Practices

1. **Preserve Structure**: Keep the basic table structure for requirements rows
2. **Include JavaScript**: Include interactive expand/collapse script if desired
3. **Test Locally**: Test your template with sample data before deploying
4. **Version Control**: Keep templates in version control alongside your tests
5. **Documentation**: Document any custom placeholders or features you add

## Migration Guide

### Updating Existing Test Cases

**Before:**
```yaml
requirement: REQ-001
# No coverage specification - defaults to full coverage
```

**After (Explicit Full Coverage):**
```yaml
requirement: REQ-001
requirement_coverage:
  type: full
```

**After (Partial Coverage):**
```yaml
requirement: REQ-001
requirement_coverage:
  type: partial
  covers: "Login functionality only"
```

**After (Multiple Requirements):**
```yaml
requirement: REQ-001
requirement_coverage:
  type: partial
  covers: "Login with OAuth"
  additional_requirements:
    - REQ-002  # OAuth integration
    - SEC-001  # Security logging
```

### Using Custom Templates

1. **Start with the example template:**
   ```bash
   cp docs/examples/html_template_example.html my-template.html
   ```

2. **Customize the template:**
   - Modify colors, fonts, and layout
   - Add your organization's branding
   - Adjust the dashboard cards
   - Customize the table styling

3. **Test the template:**
   ```bash
   req-coverage verify --test-cases-folder ./testcases --test-results-folder ./results --output coverage.json
   req-coverage print --format html --input coverage.json --output ./report/ --template ./my-template.html
   open ./report/index.html
   ```

4. **Commit the template:**
   ```bash
   git add my-template.html
   git commit -m "Add custom coverage report template"
   ```

## Troubleshooting

### Issue: Additional requirements not appearing in report

**Solution**: Ensure `additional_requirements` is a list (YAML array):
```yaml
# Correct
additional_requirements:
  - REQ-002
  - REQ-003

# Incorrect
additional_requirements: REQ-002
```

### Issue: Template placeholders not replaced

**Solution**: Check that placeholders exactly match the format `{{PLACEHOLDER}}` with:
- Double braces on each side
- No spaces inside braces
- Exact uppercase placeholder names

### Issue: Template JavaScript not working

**Solution**: Ensure your template includes the interactive script for expandable rows:
```javascript
document.querySelectorAll('.expandable').forEach(el => {
    el.addEventListener('click', function() {
        this.classList.toggle('expanded');
        const detailsId = this.getAttribute('data-details');
        const details = document.getElementById(detailsId);
        details.classList.toggle('show');
    });
});
```

## Examples

### Complete Test Case Example

```yaml
type: test_case
schema: tcms/test-case.schema.v1.json
requirement: AUTH-002
item: 1
tc: 1
id: AUTH_002_TC_001
description: Test password reset functionality

requirement_coverage:
  type: partial
  covers: "Password reset via email"
  additional_requirements:
    - AUTH-003  # Email notifications
    - SEC-005   # Security event logging

general_initial_conditions:
  system:
    - Email service is configured
    - User database is accessible

initial_conditions:
  system:
    - Test user exists with valid email

test_sequences:
  - id: 1
    name: Password Reset Flow
    description: Complete password reset via email
    steps:
      - step: 1
        description: Request password reset
        command: curl -X POST /api/password-reset -d '{"email":"test@example.com"}'
        expected:
          result: 0
          output: '{"status":"success"}'
        verification:
          result: '[[ $EXIT_CODE -eq 0 ]]'
          output: grep -q '"status":"success"' <<< "$COMMAND_OUTPUT"
```

### Complete Template Example

See `docs/examples/html_template_example.html` for a full working template with all features demonstrated.

## Additional Resources

- [Product Requirements Document](PRD_REQ_COVERAGE.md)
- [Quick Start Guide](REQ_COVERAGE_QUICK_START.md)
- [Quick Reference](REQ_COVERAGE_REFERENCE.md)
- [Implementation Summary](../REQ_COVERAGE_IMPLEMENTATION.md)
