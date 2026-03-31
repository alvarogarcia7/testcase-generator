# TCMS Envelope Schema System

This directory contains versioned JSON schemas for the Test Case Management System (TCMS) that follow the envelope meta-schema pattern.

## Directory Layout

```
schemas/
├── tcms/                                      # Versioned envelope schemas (current)
│   ├── *.schema.v1.json                       # Schema files (v1)
│   │   ├── test-case.schema.v1.json
│   │   ├── test-execution.schema.v1.json
│   │   ├── test-verification.schema.v1.json
│   │   ├── test-result.schema.v1.json
│   │   ├── container-config.schema.v1.json
│   │   └── test-results-container.schema.v1.json
│   ├── samples/                               # Sample data files
│   │   ├── README.md
│   │   └── testcase_results_container_sample.yml
│   └── verification_methods/                  # Verification method schemas
│       ├── analysis/
│       ├── common_criteria/
│       ├── demonstration/
│       ├── high_assurance/
│       ├── inspection/
│       ├── result/
│       └── test/
├── templates/                                 # Templates (moved from tcms/)
│   ├── verification_methods/
│   │   ├── analysis/
│   │   ├── common_criteria/
│   │   ├── demonstration/
│   │   ├── high_assurance/
│   │   ├── inspection/
│   │   ├── result/
│   │   └── test/
│   └── requirement_aggregation_template.*
├── *.schema.json                              # Legacy schemas (backward compatible)
│   ├── test-case.schema.json
│   ├── execution-log.schema.json
│   ├── verification-result.schema.json
│   ├── verification-output.schema.json
│   └── container_config.schema.json
└── tcms-envelope.schema.json                  # Envelope meta-schema
```

### Key Locations

- **Schema Files**: `schemas/tcms/*.schema.v1.json` - Versioned envelope schemas
- **Sample Data**: `schemas/tcms/samples/` - Test and validation sample files
- **Templates**: `schemas/templates/` - Jinja2 and AsciiDoc templates (previously in `schemas/tcms/verification_methods/`)
- **Legacy Schemas**: `schemas/*.schema.json` - Original schemas with optional envelope support

## Overview

The TCMS envelope schema system provides:

1. **Meta-Schema**: `schemas/tcms-envelope.schema.json` - Defines required `type` and `schema` fields for all TCMS documents
2. **Versioned Schemas**: Located in `schemas/tcms/` - All schemas follow the `tcms/<type>.schema.v<version>.json` naming convention
3. **Backward Compatibility**: Existing schemas in `schemas/` have been updated to include optional envelope fields

## Schema Relationships

### Legacy vs. Versioned Schemas

| Document Type | Legacy Schema (Optional Envelope) | Versioned Schema (Required Envelope) | Relationship |
|---------------|-----------------------------------|--------------------------------------|--------------|
| Test Case | `schemas/test-case.schema.json` | `schemas/tcms/test-case.schema.v1.json` | Legacy accepts envelope fields as optional; versioned requires them |
| Test Execution | `schemas/execution-log.schema.json` | `schemas/tcms/test-execution.schema.v1.json` | Same structure, different envelope requirements |
| Test Verification | `schemas/verification-result.schema.json` | `schemas/tcms/test-verification.schema.v1.json` | Same structure, different envelope requirements |
| Test Result | `schemas/verification-output.schema.json` | `schemas/tcms/test-result.schema.v1.json` | Same structure, different envelope requirements |
| Container Config | `schemas/container_config.schema.json` | `schemas/tcms/container-config.schema.v1.json` | Same structure, different envelope requirements |
| Test Results Container | `data/testcase_results_container/schema.json` | `schemas/tcms/test-results-container.schema.v1.json` | Relocated and updated with envelope |

**Key Differences**:
- **Legacy schemas** (Draft-04): Make `type` and `schema` fields **optional**, allowing gradual migration
- **Versioned schemas** (Draft-07): **Require** `type` and `schema` fields via `allOf` composition with envelope meta-schema
- **Content structure**: Identical property definitions and validation rules
- **Purpose**: Legacy schemas support existing documents; versioned schemas enforce envelope pattern for new documents

## Document Type Mapping

This table maps each document type to its canonical schema location and corresponding sample files:

| Document Type | Type Value | Versioned Schema | Legacy Schema | Sample Files | Notes |
|---------------|------------|------------------|---------------|--------------|-------|
| **Test Case** | `test_case` | `schemas/tcms/test-case.schema.v1.json` | `schemas/test-case.schema.json` | `testcases/*.yml`<br/>`examples/*.yml` | Test case definitions with sequences, steps, prerequisites |
| **Test Execution** | `test_execution` | `schemas/tcms/test-execution.schema.v1.json` | `schemas/execution-log.schema.json` | `data/TEST_VAR_PASSING_001_execution_log.json` | Execution log entries for test steps |
| **Test Verification** | `test_verification` | `schemas/tcms/test-verification.schema.v1.json` | `schemas/verification-result.schema.json` | N/A | Verification results for test cases |
| **Test Result** | `test_result` | `schemas/tcms/test-result.schema.v1.json` | `schemas/verification-output.schema.json` | N/A | Test verification output from executor |
| **Container Config** | `container_config` | `schemas/tcms/container-config.schema.v1.json` | `schemas/container_config.schema.json` | N/A | Container configuration metadata |
| **Test Results Container** | `test_results_container` | `schemas/tcms/test-results-container.schema.v1.json` | `data/testcase_results_container/schema.json` | `schemas/tcms/samples/testcase_results_container_sample.yml` | Container format for multiple test results |

### Sample File Locations

- **Test Cases**: `testcases/` directory contains production test case YAML files
- **Examples**: `examples/` directory contains example and demonstration test cases
- **Test Data**: `data/` directory contains sample data for testing and validation
- **TCMS Samples**: `schemas/tcms/samples/` directory contains schema-specific sample files

## Envelope Meta-Schema

**File**: `schemas/tcms-envelope.schema.json`

The envelope meta-schema (Draft-07) requires two fields in all TCMS documents:

- `type` (string, enum): Document type identifier
  - Valid values: `test_case`, `test_execution`, `test_verification`, `test_result`, `container_config`, `test_results_container`
- `schema` (string, pattern): Schema reference matching `tcms/<type>.schema.v<version>.json`

## Versioned Schemas

All versioned schemas are located in `schemas/tcms/` and compose with the envelope meta-schema using `allOf`:

### Test Case Schema v1
**File**: `schemas/tcms/test-case.schema.v1.json`
- **Type**: `test_case`
- **Schema**: `tcms/test-case.schema.v1.json`
- **Description**: Defines test case structure with sequences, steps, prerequisites, and hooks

### Test Execution Schema v1
**File**: `schemas/tcms/test-execution.schema.v1.json`
- **Type**: `test_execution`
- **Schema**: `tcms/test-execution.schema.v1.json`
- **Description**: Defines execution log entries for test steps

### Test Verification Schema v1
**File**: `schemas/tcms/test-verification.schema.v1.json`
- **Type**: `test_verification`
- **Schema**: `tcms/test-verification.schema.v1.json`
- **Description**: Defines verification results for test cases

### Test Result Schema v1
**File**: `schemas/tcms/test-result.schema.v1.json`
- **Type**: `test_result`
- **Schema**: `tcms/test-result.schema.v1.json`
- **Description**: Defines test verification output from executor

### Container Config Schema v1
**File**: `schemas/tcms/container-config.schema.v1.json`
- **Type**: `container_config`
- **Schema**: `tcms/container-config.schema.v1.json`
- **Description**: Defines container configuration metadata

### Test Results Container Schema v1
**File**: `schemas/tcms/test-results-container.schema.v1.json`
- **Type**: `test_results_container`
- **Schema**: `tcms/test-results-container.schema.v1.json`
- **Description**: Defines container format for multiple test results

## Backward Compatibility

The following existing schemas have been updated to include optional envelope fields:

- `schemas/test-case.schema.json`
- `schemas/execution-log.schema.json`
- `schemas/verification-result.schema.json`
- `schemas/verification-output.schema.json`
- `schemas/container_config.schema.json`
- `data/testcase_results_container/schema.json`

These schemas now accept (but do not require) `type` and `schema` fields, allowing for gradual migration to the envelope format.

## Schema Composition Pattern

Versioned schemas compose with the envelope meta-schema using the `allOf` keyword:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://tcms.example.com/schemas/tcms/test-case.schema.v1.json",
  "title": "TCMS Test Case Schema v1",
  "allOf": [
    {
      "$ref": "../tcms-envelope.schema.json"
    },
    {
      "type": "object",
      "properties": {
        "type": {
          "const": "test_case"
        },
        "schema": {
          "const": "tcms/test-case.schema.v1.json"
        },
        // ... other properties
      },
      "required": [
        "type",
        "schema",
        // ... other required fields
      ]
    }
  ]
}
```

This pattern:
1. Inherits envelope requirements from the meta-schema
2. Constrains `type` to a specific constant value
3. Constrains `schema` to the specific version identifier
4. Adds document-specific properties and requirements

## Migration Guide

### Overview

Migrating from legacy schemas to versioned envelope schemas involves adding two required fields and updating schema references in your code and validation pipelines.

### Step 1: Update Document Structure

Add envelope fields to your documents:

**Before (Legacy format):**
```json
{
  "requirement": "REQ-001",
  "item": 1,
  "tc": 1,
  "id": "example-test",
  "description": "Example test case",
  "general_initial_conditions": {},
  "initial_conditions": {},
  "test_sequences": []
}
```

**After (Envelope format):**
```json
{
  "type": "test_case",
  "schema": "tcms/test-case.schema.v1.json",
  "requirement": "REQ-001",
  "item": 1,
  "tc": 1,
  "id": "example-test",
  "description": "Example test case",
  "general_initial_conditions": {},
  "initial_conditions": {},
  "test_sequences": []
}
```

### Step 2: Update Schema References in Code

Update your validation code to reference versioned schemas:

**Before:**
```rust
// Rust example
let schema_path = "schemas/test-case.schema.json";
```

```python
# Python example
schema_path = "schemas/test-case.schema.json"
```

**After:**
```rust
// Rust example
let schema_path = "schemas/tcms/test-case.schema.v1.json";
```

```python
# Python example
schema_path = "schemas/tcms/test-case.schema.v1.json"
```

### Step 3: Update Schema Validation Logic

If your code dynamically selects schemas based on document type, update the mapping:

**Before:**
```rust
// Rust example
fn get_schema_path(doc_type: &str) -> &str {
    match doc_type {
        "test_case" => "schemas/test-case.schema.json",
        "execution" => "schemas/execution-log.schema.json",
        _ => panic!("Unknown type"),
    }
}
```

**After:**
```rust
// Rust example
fn get_schema_path(doc_type: &str, version: u32) -> String {
    format!("schemas/tcms/{}.schema.v{}.json", 
            doc_type.replace("_", "-"), version)
}

// Or read from document's schema field:
fn get_schema_from_document(doc: &Document) -> &str {
    &doc.schema // Returns "tcms/test-case.schema.v1.json"
}
```

### Step 4: Migration Strategy for Existing Documents

Choose one of these migration approaches:

#### Option A: Bulk Migration (Recommended for new projects)

Migrate all documents at once:

```bash
# Example script to add envelope fields
for file in testcases/*.yml; do
  # Add type and schema fields at the top of YAML
  echo "Processing $file..."
  # Use yq, jq, or custom script
done
```

#### Option B: Gradual Migration (Recommended for production systems)

1. **Phase 1**: Update legacy schemas to accept optional envelope fields (already done)
2. **Phase 2**: Update tooling to write envelope fields for new documents
3. **Phase 3**: Gradually migrate existing documents during updates
4. **Phase 4**: Switch to versioned schemas once all documents are migrated

#### Option C: Dual Support (For transition period)

Support both formats simultaneously:

```rust
// Rust example
fn validate_document(doc: &Document) -> Result<()> {
    let schema_path = if doc.has_envelope_fields() {
        // Use versioned schema
        format!("schemas/tcms/{}.schema.v1.json", 
                doc.type_name().replace("_", "-"))
    } else {
        // Use legacy schema
        get_legacy_schema_path(doc)
    };
    
    validate_against_schema(doc, &schema_path)
}
```

### Step 5: Update Test and Validation Pipelines

Update your CI/CD and validation scripts:

**Before:**
```bash
# Old validation command
validate-yaml --schema schemas/test-case.schema.json testcases/*.yml
```

**After:**
```bash
# New validation command (with envelope support)
validate-yaml --schema schemas/tcms/test-case.schema.v1.json testcases/*.yml

# Or use the envelope validation script
make validate-envelope-schemas
```

### Type and Schema Field Reference

When adding envelope fields, use these exact values:

| Document Type | `type` Field Value | `schema` Field Value |
|---------------|-------------------|----------------------|
| Test Case | `test_case` | `tcms/test-case.schema.v1.json` |
| Test Execution | `test_execution` | `tcms/test-execution.schema.v1.json` |
| Test Verification | `test_verification` | `tcms/test-verification.schema.v1.json` |
| Test Result | `test_result` | `tcms/test-result.schema.v1.json` |
| Container Config | `container_config` | `tcms/container-config.schema.v1.json` |
| Test Results Container | `test_results_container` | `tcms/test-results-container.schema.v1.json` |

### Common Migration Pitfalls

1. **Incorrect type values**: Use underscores (e.g., `test_case`), not hyphens
2. **Wrong schema format**: Must be `tcms/<name>.schema.v<version>.json`, not a file path
3. **Schema version mismatch**: Ensure document content matches the schema version
4. **Missing required fields**: Both `type` and `schema` are required in versioned schemas
5. **Case sensitivity**: Type and schema values are case-sensitive

### Validation After Migration

After migrating documents, validate them:

```bash
# Validate envelope schemas
make validate-envelope-schemas

# Validate individual document
cargo run -p validate-yaml -- \
  --schema schemas/tcms/test-case.schema.v1.json \
  testcases/example.yml

# Run full test suite
make test
```

### Rollback Strategy

If issues arise during migration:

1. **Keep legacy schemas**: Legacy schemas remain available at `schemas/*.schema.json`
2. **Remove envelope fields**: Documents without envelope fields validate against legacy schemas
3. **Version control**: Use git to revert changes if needed

## Validation

### Validate Envelope Schemas

```bash
make validate-envelope-schemas
```

This runs `scripts/validate_envelope_schemas.sh` which:
1. Validates JSON syntax of all schemas
2. Checks that versioned schemas properly reference the envelope meta-schema
3. Validates envelope field constraints (type and schema constants)
4. Tests sample documents against each versioned schema

### Verbose Output

```bash
./scripts/validate_envelope_schemas.sh --verbose
```

## Example Document

Example test case with envelope fields:

```json
{
  "type": "test_case",
  "schema": "tcms/test-case.schema.v1.json",
  "requirement": "REQ-001",
  "item": 1,
  "tc": 1,
  "id": "example-test",
  "description": "Example test case",
  "general_initial_conditions": {},
  "initial_conditions": {},
  "test_sequences": []
}
```

## Versioning Strategy

Schema versions follow semantic versioning:

- **v1**: Initial version with envelope support
- **v2**: Future breaking changes (e.g., removing fields, changing required fields)
- **v1.1**: Future non-breaking changes (e.g., adding optional fields)

New versions are created as separate files (e.g., `test-case.schema.v2.json`) to maintain backward compatibility.

## Benefits

1. **Type Safety**: Documents explicitly declare their type and schema version
2. **Validation**: Automated validation ensures documents conform to envelope requirements
3. **Versioning**: Explicit schema versions enable evolution without breaking existing documents
4. **Interoperability**: Consistent envelope format across all TCMS document types
5. **Discovery**: Tools can inspect `type` and `schema` fields to determine document structure
6. **Migration Path**: Optional envelope fields in existing schemas allow gradual adoption
7. **Better Organization**: Clear separation between schemas (tcms/), templates (templates/), and samples (tcms/samples/)
