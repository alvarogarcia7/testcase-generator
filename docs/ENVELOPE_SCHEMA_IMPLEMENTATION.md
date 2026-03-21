# TCMS Envelope Schema Implementation

## Overview

This document describes the implementation of the TCMS envelope schema system, which provides a standardized way to identify and version TCMS documents.

## What Was Implemented

### 1. Envelope Meta-Schema

**File**: `schemas/tcms-envelope.schema.json`

A Draft-07 JSON Schema that defines the envelope pattern for all TCMS documents. It requires two fields:

- `type` (string, enum): One of `test_case`, `test_execution`, `test_verification`, `test_result`, `container_config`, `test_results_container`
- `schema` (string, pattern): Must match pattern `tcms/<type>.schema.v<version>.json`

### 2. Versioned Schemas

Created 6 versioned schemas in `schemas/tcms/` directory:

| Schema File | Type | Description |
|-------------|------|-------------|
| `test-case.schema.v1.json` | `test_case` | Test case definitions with sequences and steps |
| `test-execution.schema.v1.json` | `test_execution` | Execution log entries |
| `test-verification.schema.v1.json` | `test_verification` | Verification results |
| `test-result.schema.v1.json` | `test_result` | Test verification output |
| `container-config.schema.v1.json` | `container_config` | Container configuration metadata |
| `test-results-container.schema.v1.json` | `test_results_container` | Container for multiple test results |

All versioned schemas:
- Compose with the envelope meta-schema using `allOf`
- Constrain `type` to their specific document type using `const`
- Constrain `schema` to their specific version path using `const`
- Include all properties from their corresponding original schema

### 3. Updated Existing Schemas

Modified 6 existing schemas to include optional envelope fields for backward compatibility:

- `schemas/test-case.schema.json`
- `schemas/execution-log.schema.json`
- `schemas/verification-result.schema.json`
- `schemas/verification-output.schema.json`
- `schemas/container_config.schema.json`
- `data/testcase_results_container/schema.json`

These schemas now accept (but do not require) the `type` and `schema` fields, enabling gradual migration to the envelope format.

### 4. Validation Script

**File**: `scripts/validate_envelope_schemas.sh`

A comprehensive validation script that:

- Validates JSON syntax of all schema files
- Checks that versioned schemas properly reference the envelope meta-schema
- Validates envelope field constraints (type and schema constants)
- Creates and validates sample documents against each schema
- Provides colored output and summary statistics
- Supports `--verbose` flag for detailed output

**Features**:
- Automated consistency checking
- Sample document generation and validation
- Comprehensive error reporting
- Exit code 0 on success, 1 on failure

### 5. Makefile Target

Added `validate-envelope-schemas` target to the Makefile:

```bash
make validate-envelope-schemas
```

This target runs the validation script to ensure all envelope schemas are internally consistent.

### 6. Documentation

Created comprehensive documentation:

- `schemas/tcms/README.md`: Complete guide to the envelope schema system
- `docs/ENVELOPE_SCHEMA_IMPLEMENTATION.md`: This implementation guide

## Schema Composition Pattern

The versioned schemas use the `allOf` keyword to compose with the envelope meta-schema:

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
        "requirement": {
          "type": "string"
        }
        // ... additional properties
      },
      "required": [
        "type",
        "schema",
        "requirement"
        // ... additional required fields
      ]
    }
  ]
}
```

This pattern:
1. Inherits the envelope requirements from the meta-schema
2. Constrains `type` to a specific constant value for this document type
3. Constrains `schema` to the specific version identifier
4. Adds document-specific properties and requirements

## Usage Examples

### Example Document with Envelope

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

### Validation

```bash
# Validate all envelope schemas
make validate-envelope-schemas

# Run with verbose output
./scripts/validate_envelope_schemas.sh --verbose
```

## Benefits

1. **Type Safety**: Documents explicitly declare their type and schema version
2. **Validation**: Automated validation ensures consistency across all schemas
3. **Versioning**: Explicit schema versions enable evolution without breaking changes
4. **Interoperability**: Consistent envelope format across all TCMS document types
5. **Discovery**: Tools can inspect `type` and `schema` fields to determine document structure
6. **Migration Path**: Optional envelope fields in existing schemas allow gradual adoption
7. **Schema Evolution**: New versions can be created alongside existing versions

## Migration Strategy

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

### Backward Compatibility

The implementation maintains full backward compatibility:

- Existing documents validate against existing schemas (envelope fields optional)
- New documents can use versioned schemas with required envelope fields
- Tools can support both formats during migration period

## File Structure

```
schemas/
├── tcms-envelope.schema.json        # Envelope meta-schema
├── tcms/                             # Versioned schemas
│   ├── README.md                     # Documentation
│   ├── test-case.schema.v1.json
│   ├── test-execution.schema.v1.json
│   ├── test-verification.schema.v1.json
│   ├── test-result.schema.v1.json
│   ├── container-config.schema.v1.json
│   └── test-results-container.schema.v1.json
├── test-case.schema.json             # Updated with optional envelope fields
├── execution-log.schema.json         # Updated with optional envelope fields
├── verification-result.schema.json   # Updated with optional envelope fields
├── verification-output.schema.json   # Updated with optional envelope fields
└── container_config.schema.json      # Updated with optional envelope fields

scripts/
└── validate_envelope_schemas.sh      # Validation script

data/
└── testcase_results_container/
    └── schema.json                   # Updated with optional envelope fields

Makefile                               # Added validate-envelope-schemas target
```

## Future Enhancements

Potential future improvements:

1. **Automated Migration Tool**: Create a script to add envelope fields to existing documents
2. **Schema Registry**: Implement a central registry for schema versions
3. **Validation in CI/CD**: Add envelope schema validation to continuous integration
4. **Documentation Generation**: Auto-generate schema documentation from envelope metadata
5. **Version Compatibility Matrix**: Document which schema versions are compatible
6. **Deprecation Warnings**: Add warnings for schemas approaching end-of-life

## Testing

The validation script performs comprehensive testing:

- ✓ JSON syntax validation for all schemas
- ✓ Envelope meta-schema reference validation
- ✓ Type constraint validation (const values)
- ✓ Schema constraint validation (const values)
- ✓ Sample document creation and validation
- ✓ $ref resolution for composed schemas

Run tests with:
```bash
make validate-envelope-schemas
```

## Summary

The TCMS envelope schema implementation provides:

- **1 meta-schema** defining envelope requirements
- **6 versioned schemas** following the envelope pattern
- **6 updated schemas** with optional envelope fields for backward compatibility
- **1 validation script** with comprehensive checks
- **1 Makefile target** for easy validation
- **Complete documentation** for users and developers

All schemas are internally consistent, tested, and ready for use.
