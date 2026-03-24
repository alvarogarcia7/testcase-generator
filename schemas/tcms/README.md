# TCMS Envelope Schema System

This directory contains versioned JSON schemas for the Test Case Management System (TCMS) that follow the envelope meta-schema pattern.

## Overview

The TCMS envelope schema system provides:

1. **Meta-Schema**: `schemas/tcms-envelope.schema.json` - Defines required `type` and `schema` fields for all TCMS documents
2. **Versioned Schemas**: Located in `schemas/tcms/` - All schemas follow the `tcms/<type>.schema.v<version>.json` naming convention
3. **Backward Compatibility**: Existing schemas in `schemas/` have been updated to include optional envelope fields

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

## Migration Guide

### For New Documents

Use versioned schemas from `schemas/tcms/` and include envelope fields:

```json
{
  "type": "test_case",
  "schema": "tcms/test-case.schema.v1.json",
  // ... rest of document
}
```

### For Existing Documents

Existing documents without envelope fields will continue to validate against schemas in `schemas/`. To migrate:

1. Add `type` field with appropriate document type
2. Add `schema` field with versioned schema reference
3. Validate against versioned schema in `schemas/tcms/`

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
