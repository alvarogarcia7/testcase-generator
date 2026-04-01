# Container Schema Consolidation

This document explains the consolidation of container schemas and their current locations.

## Schema History

Previously, there were multiple redundant container schemas:
- `schemas/tcms/test_results/container_schema.json` (REMOVED)
- `schemas/tcms/testcase_results_container/schema.json` (REMOVED)
- `schemas/tcms/container/schema.json` (REMOVED)

These have been **removed** as they were superseded by the canonical versioned schema.

## Current Active Schemas

### 1. Canonical V1 Schema (with envelope)

**Location:** `schemas/tcms/test-results-container.schema.v1.json`

**Purpose:** Official versioned schema for test results containers with full TCMS envelope support

**Use case:** New integrations, when envelope fields (`type`, `schema`) are required

**Features:**
- ✅ Full envelope support with `type` and `schema` fields
- ✅ Versioned (v1) for evolution
- ✅ Comprehensive validation
- ✅ Uses JSON Schema draft-07
- ✅ References `tcms-envelope.schema.json`

**Example:**
```json
{
  "type": "test_results_container",
  "schema": "tcms/test-results-container.schema.v1.json",
  "title": "Test Results",
  "project": "My Project",
  "test_date": "2024-01-15T10:30:00Z",
  "test_results": [...],
  "metadata": {
    "execution_duration": 120.5,
    "total_test_cases": 1,
    "passed_test_cases": 1,
    "failed_test_cases": 0
  }
}
```

### 2. Working Schema (without envelope requirement)

**Locations:**
- `data/testcase_results_container/schema.json` (used by verifier binary default)
- `testcases/examples/expected_test_results/container/container_schema.json` (used by validation scripts)

**Purpose:** Backward-compatible schema for existing tools and workflows that don't require envelope fields

**Use case:** 
- Verifier binary output validation
- Integration tests
- Scripts that validate container outputs
- Tools that haven't migrated to envelope format yet

**Features:**
- ✅ Same validation rules as canonical v1 (except envelope)
- ✅ Envelope fields (`type`, `schema`) are optional, not required
- ✅ Compatible with existing verifier outputs
- ✅ Comprehensive validation for test results structure
- ✅ Uses JSON Schema draft-07

**Example:**
```json
{
  "title": "Test Results",
  "project": "My Project",
  "test_date": "2024-01-15T10:30:00Z",
  "test_results": [...],
  "metadata": {
    "execution_duration": 120.5,
    "total_test_cases": 1,
    "passed_test_cases": 1,
    "failed_test_cases": 0
  }
}
```

## Key Differences

| Feature | Canonical V1 | Working Schema |
|---------|--------------|----------------|
| Location | `schemas/tcms/test-results-container.schema.v1.json` | `data/testcase_results_container/schema.json` |
| Envelope required | ✅ Yes (type, schema) | ❌ No (optional) |
| Versioning | ✅ v1 | ❌ Not versioned |
| Use envelope.schema | ✅ Yes (allOf) | ❌ No |
| Test results validation | ✅ Full | ✅ Full |
| Metadata validation | ✅ Full | ✅ Full |
| Used by | New tools, canonical reference | Verifier binary, validation scripts |

## Migration Path

### For new code/tools
Use the **canonical v1 schema** at `schemas/tcms/test-results-container.schema.v1.json`

### For existing tools
Continue using the **working schema** at:
- `data/testcase_results_container/schema.json` (verifier)
- `testcases/examples/expected_test_results/container/container_schema.json` (validation scripts)

### Future consolidation
Once all tools support the envelope format:
1. Update verifier to generate outputs with envelope fields
2. Migrate working schemas to reference canonical v1
3. Deprecate non-envelope format

## Code References

### Rust Code
- `crates/verifier/src/main.rs` - Uses `data/testcase_results_container/schema.json` as default
- `crates/testcase-manager/tests/container_schema_validation_test.rs` - Tests with `data/testcase_results_container/schema.json`
- `crates/testcase-manager/src/bin/verifier.rs` - Verifier binary references

### Scripts
- `scripts/validate-container-output.sh` - Uses `data/testcase_results_container/schema.json`
- `scripts/validate-output-schemas.sh` - Uses `testcases/examples/expected_test_results/container/container_schema.json`
- Test acceptance scripts in `test-acceptance/` directory

## Documentation References

Updated documentation:
- `schemas/README.md` - Main schema documentation
- `schemas/SCHEMA_AUDIT.md` - Schema audit report
- `schemas/SCHEMA_QUICK_REFERENCE.md` - Quick reference guide
- `SCHEMA_DOCUMENTATION.md` - Root schema documentation
- `data/testcase_results_container/README.md` - Working schema documentation

## Removed Schemas Analysis

### schemas/tcms/test_results/container_schema.json
**Status:** ❌ REMOVED (redundant)
- Reason: Minimal, loose typing (test_results items were generic objects)
- Missing: Envelope support, versioning, detailed validation
- Superseded by: Canonical v1 schema

### schemas/tcms/testcase_results_container/schema.json
**Status:** ❌ REMOVED (redundant) 
- Reason: Different encoding approach (expected/actual pairs vs tagged enums)
- Missing: Envelope support, versioning
- Different from v1: Used pairs in Pass/Fail results instead of simplified tagged enums
- Superseded by: Canonical v1 schema with clearer encoding

### schemas/tcms/container/schema.json
**Status:** ❌ REMOVED (minimal legacy)
- Reason: Too minimal (only 3 fields: date, product, description)
- Missing: Test results, metadata, envelope, versioning
- Outdated: Used JSON Schema draft-04
- Superseded by: Either canonical v1 or container-config.schema.v1.json depending on use case

## Questions?

For schema-related questions, see:
- [schemas/README.md](../README.md) - Schema overview
- [schemas/SCHEMA_AUDIT.md](SCHEMA_AUDIT.md) - Detailed audit
- [schemas/SCHEMA_QUICK_REFERENCE.md](SCHEMA_QUICK_REFERENCE.md) - Quick reference
