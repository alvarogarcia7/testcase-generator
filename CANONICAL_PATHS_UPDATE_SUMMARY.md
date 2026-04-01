# Canonical Paths Update Summary

This document summarizes the updates made to ensure all schema, template, and sample file references use canonical paths throughout the codebase.

## Canonical Path Standards

### Schema Files

1. **Legacy Schemas (Backward Compatible)**
   - Location: `schemas/*.schema.json`
   - Example: `schemas/test-case.schema.json`
   - Purpose: Backward compatibility with existing code
   - Envelope: Optional

2. **Versioned Schemas (Production Standard)**
   - Location: `schemas/tcms/*.schema.v1.json`
   - Example: `schemas/tcms/test-case.schema.v1.json`
   - Purpose: Production standard for new development
   - Envelope: Required (`type` and `schema` fields)

3. **Envelope Meta-Schema**
   - Location: `schemas/tcms-envelope.schema.json`
   - Purpose: Defines envelope pattern for versioned schemas

4. **Working Container Schema**
   - Location: `data/testcase_results_container/schema.json`
   - Purpose: Used by verifier tool, backward compatible
   - Recommended alternative: `schemas/tcms/test-results-container.schema.v1.json`

### Sample and Template Files

1. **Sample Data**
   - Canonical Location: `schemas/tcms/samples/`
   - Example: `schemas/tcms/samples/testcase_results_container_sample.yml`

2. **Templates**
   - Canonical Location: `schemas/templates/`
   - Subdirectories: `verification_methods/` for method-specific templates

## Files Updated

### Python Scripts

1. **scripts/generate_validation_report.py**
   - Added comment documenting canonical schema path usage
   - Current: `schemas/test-case.schema.json` (legacy)
   - Alternative: `schemas/tcms/test-case.schema.v1.json` (versioned)

2. **scripts/convert_verification_to_tpdg.py**
   - Updated `--schema` argument help text to reference canonical paths
   - Fixed container schema reference from `tcms/testcase_results_container.schema.v1.json` to `tcms/test-results-container.schema.v1.json`
   - Added comment about canonical schema path for versioned container schema

### Rust Code

3. **crates/testcase-validation/src/lib.rs**
   - Added documentation to `find_schema_file()` function
   - Documents canonical path: `schemas/test-case.schema.json` (legacy)
   - Recommends for new development: `schemas/tcms/test-case.schema.v1.json`

4. **crates/validate-yaml/src/main.rs**
   - Enhanced CLI argument documentation
   - Documents canonical paths for both legacy and versioned schemas
   - Clarifies `schemas_root` default value purpose

5. **crates/validate-json/src/main.rs**
   - Enhanced CLI argument documentation
   - Documents canonical paths for both legacy and versioned schemas
   - Clarifies `schemas_root` default value purpose

6. **crates/testcase-ui/src/validation.rs**
   - Added documentation to `find_schema_file()` function
   - Documents canonical path usage and migration guidance

7. **crates/testcase-manager/src/validation.rs**
   - Added documentation to `SchemaValidator::new()` method
   - Documents canonical path and migration guidance

8. **crates/testcase-common/src/envelope.rs**
   - Enhanced `resolve_schema_from_payload()` function documentation
   - Documents canonical paths resolved by the function

### Test Files

9. **crates/testcase-manager/tests/container_schema_validation_test.rs**
   - Added comprehensive comment block documenting canonical paths
   - Working schema: `data/testcase_results_container/schema.json`
   - Versioned schema: `schemas/tcms/test-results-container.schema.v1.json`
   - Sample data: `schemas/tcms/samples/`

10. **crates/testcase-manager/tests/schema_validation_test.rs**
    - Added documentation to `load_schema()` function
    - Documents canonical paths for legacy and versioned schemas

11. **crates/testcase-manager/tests/data_files_validation.rs**
    - Added comment documenting canonical schema path usage

### Build Configuration

12. **Makefile**
    - Added comment block before `test-e2e-validate-yaml` target
    - Added comments to `validate-all-testcases` target
    - Added comments to `verify-testcases` target
    - Added comments to `validate-output-schemas` target
    - Added comments to `validate-envelope-schemas` target
    - Added comments to `watch-verbose` target

### Shell Scripts

13. **scripts/validate-output-schemas.sh**
    - Enhanced configuration section with canonical path documentation
    - Documents TCMS samples location: `schemas/tcms/samples/`
    - Documents templates location: `schemas/templates/`
    - Documents expected results location

14. **scripts/validate_envelope_schemas.sh**
    - Added comprehensive canonical path documentation
    - Documents versioned schemas location: `schemas/tcms/*.schema.v1.json`
    - Documents envelope meta-schema location
    - Documents samples and templates locations

### Documentation

15. **data/testcase_results_container/README.md**
    - Added "Canonical Schema Paths" section at the top
    - Clarifies working schema vs. versioned schema distinction
    - Documents sample data canonical location
    - Added note about migration to versioned schema

## Key Changes Summary

1. **Consistent Documentation**: All files now document canonical paths for schemas
2. **Migration Guidance**: Comments provide guidance for migrating to versioned schemas
3. **Path Clarity**: Clear distinction between legacy (backward compatible) and versioned (production standard) schemas
4. **Sample Location**: Canonical sample data location is consistently referenced as `schemas/tcms/samples/`
5. **Template Location**: Canonical template location is consistently referenced as `schemas/templates/`

## Canonical Path Reference

```
schemas/
├── *.schema.json                          # Legacy schemas (backward compatible)
├── tcms-envelope.schema.json              # Envelope meta-schema
├── tcms/
│   ├── *.schema.v1.json                  # Versioned schemas (production standard)
│   └── samples/                           # Sample data (canonical location)
│       └── testcase_results_container_sample.yml
└── templates/                             # Templates (canonical location)
    └── verification_methods/

data/
└── testcase_results_container/
    └── schema.json                        # Working schema (used by verifier)
```

## Verification

To verify canonical path usage:

1. Legacy schema: `schemas/test-case.schema.json`
2. Versioned schema: `schemas/tcms/test-case.schema.v1.json`
3. Container schema (working): `data/testcase_results_container/schema.json`
4. Container schema (versioned): `schemas/tcms/test-results-container.schema.v1.json`
5. Samples: `schemas/tcms/samples/`
6. Templates: `schemas/templates/`

All references in the codebase now document these canonical paths consistently.
