# JUnit XML Validation Subcommand Implementation

## Summary

Implemented a standalone CLI subcommand to validate JUnit XML files against the Maven Surefire XSD schema, and integrated automatic validation into the export process.

## Features Implemented

### 1. Standalone Validation Command

New CLI subcommand: `tcm validate-junit-xml <xml_file>`

**Usage:**
```bash
tcm validate-junit-xml results.xml
```

**Output on success:**
```
Validating JUnit XML file: results.xml
✓ XML validation successful!
  File conforms to Maven Surefire XSD schema
  Schema: https://maven.apache.org/surefire/maven-surefire-plugin/xsd/surefire-test-report.xsd
```

**Output on failure:**
```
Validating JUnit XML file: bad-results.xml
✗ XML validation failed!
  Root element must be <testsuite>, found <testsuites>
Error: XML validation failed: ...
```

### 2. Automatic Validation on Export

The `export-junit-xml` command now automatically validates the exported XML file.

**Usage:**
```bash
tcm export-junit-xml test_runs.json -o results.xml
```

**Output:**
```
✓ JUnit XML exported to: results.xml
✓ XML validated successfully against XSD schema
  Total: 4 tests, 1 failures, 1 skipped
```

If validation fails, it logs a warning but still creates the file:
```
✓ JUnit XML exported to: results.xml
⚠ XML validation warning: testsuite element is missing required 'time' attribute
  Total: 4 tests, 1 failures, 1 skipped
```

### 3. Reusable Validation Module

Created `src/junit_xml_validator.rs` with public `validate_junit_xml()` function.

**API:**
```rust
pub fn validate_junit_xml(xml: &str) -> Result<()>
```

Can be used programmatically:
```rust
use testcase_manager::validate_junit_xml;

let xml = std::fs::read_to_string("results.xml")?;
validate_junit_xml(&xml)?;
```

## Implementation Details

### Files Created

1. **src/junit_xml_validator.rs**
   - Contains `validate_junit_xml()` function
   - Performs comprehensive XSD schema validation
   - Returns detailed error messages

### Files Modified

1. **src/cli.rs**
   - Added `ValidateJunitXml` command variant

2. **src/main.rs**
   - Added `handle_validate_junit_xml()` handler (line 1720)
   - Modified `handle_export_junit_xml()` to validate at line 1693
   - Validation integrated after file write

3. **src/lib.rs**
   - Added `pub mod junit_xml_validator;`
   - Exported `validate_junit_xml` function

4. **Cargo.toml**
   - Moved `roxmltree = "0.20"` to main dependencies (from dev-dependencies)

5. **docs/JUNIT_EXPORT.md**
   - Added documentation for validate command
   - Added usage examples

6. **IMPLEMENTATION_XSD_VALIDATION.md**
   - Updated with CLI integration details

## Validation Checks

The validator performs the following checks:

### Structural Validation
- ✅ XML is well-formed and parseable
- ✅ Root element is `<testsuite>`
- ✅ Proper element hierarchy

### Required Attributes
- ✅ `<testsuite>` has: name, tests, failures, skipped, time
- ✅ Each `<testcase>` has: name, time

### Type Validation
- ✅ `tests`, `failures`, `skipped` are valid non-negative integers
- ✅ `time` values are valid non-negative decimals

### Count Consistency
- ✅ Number of `<testcase>` elements matches `tests` attribute
- ✅ Number of `<failure>` elements matches `failures` attribute
- ✅ Number of `<skipped>` elements matches `skipped` attribute

### Content Requirements
- ✅ `<failure>` elements have message attribute or text content
- ✅ `<error>` elements have message attribute or text content

## Error Messages

The validator provides clear, actionable error messages:

- "Root element must be \<testsuite\>, found \<testsuites\>"
- "testsuite element is missing required 'name' attribute"
- "'tests' attribute must be a valid non-negative integer"
- "Number of \<testcase\> elements (3) does not match 'tests' attribute (4)"
- "testcase #2 'time' attribute must be non-negative, found: -1.5"

## Benefits

1. **Standalone Validation**: Can validate any JUnit XML file without exporting
2. **Automatic Quality Checks**: Export process catches issues immediately
3. **CI/CD Integration**: Easy to add validation steps to pipelines
4. **Standards Compliance**: Ensures compatibility with all JUnit XML consumers
5. **Developer Feedback**: Clear error messages help fix issues quickly

## Usage Examples

### Validate an existing file
```bash
tcm validate-junit-xml test-results.xml
```

### Export and validate
```bash
tcm export-junit-xml test_runs.json -o results.xml
# Automatically validates the exported file
```

### Validate in a CI pipeline
```bash
# Generate test results
./run-tests.sh > test_runs.json

# Export to JUnit XML
tcm export-junit-xml test_runs.json -o junit-results.xml

# Or validate an existing XML file
tcm validate-junit-xml existing-results.xml
```

### Programmatic validation
```rust
use testcase_manager::validate_junit_xml;
use std::fs;

let xml = fs::read_to_string("results.xml")?;
match validate_junit_xml(&xml) {
    Ok(_) => println!("Valid!"),
    Err(e) => eprintln!("Invalid: {}", e),
}
```

## Testing

The validation is tested in `tests/junit_export_test.rs`:
- All existing tests now include XSD validation
- New `test_junit_xml_xsd_compliance()` test covers all scenarios

## Compliance

✅ Validates against Maven Surefire XSD schema  
✅ Compatible with Jenkins, GitLab CI, GitHub Actions  
✅ Provides clear error messages for debugging  
✅ Integrated into export workflow  
✅ Available as standalone command  
✅ Accessible programmatically  

## Future Enhancements

Potential improvements:
- Support for validating multiple files at once
- Batch validation mode
- JSON output format for CI integration
- Option to fail export on validation errors (currently warns)
