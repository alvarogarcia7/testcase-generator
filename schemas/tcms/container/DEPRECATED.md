# ⚠️ DEPRECATED SCHEMA

**This directory contains a deprecated schema that should not be used in new code.**

## Schema in this directory

### schema.json
- **Status:** ❌ DEPRECATED
- **Reason:** Minimal legacy schema with only 3 fields, superseded by comprehensive container schemas
- **Use instead:** 
  - `schemas/tcms/container-config.schema.v1.json` - For container configuration
  - `schemas/tcms/test-results-container.schema.v1.json` - For test results containers
- **Migration:** Determine use case and migrate to appropriate v1 schema

## Why this schema is deprecated

1. **Too minimal** - Only has 3 fields: `date`, `product`, `description`
2. **No envelope support** - Missing required `type` and `schema` fields
3. **Not versioned** - Cannot evolve without breaking changes
4. **Outdated** - Uses JSON Schema draft-04
5. **Unclear purpose** - Does not match current container concepts
6. **Superseded** - Both container-config and test-results-container v1 schemas provide comprehensive functionality

## Schema Content

This schema only defines:
```json
{
  "date": "string",
  "product": "string",
  "description": "string"
}
```

This is insufficient for modern container needs.

## Migration Paths

### If you need container configuration

Use `schemas/tcms/container-config.schema.v1.json`:

**Before:**
```json
{
  "date": "2024-01-15",
  "product": "My Product",
  "description": "Test execution container"
}
```

**After:**
```json
{
  "type": "container_config",
  "schema": "tcms/container-config.schema.v1.json",
  "title": "Test execution container",
  "project": "My Product",
  "environment": "Development",
  "platform": "Linux x86_64",
  "executor": "Jenkins"
}
```

**Mapping:**
- `description` → `title`
- `product` → `project`
- `date` → (remove, or add as metadata in actual test results)
- Add envelope fields (`type`, `schema`)
- Add optional metadata fields (`environment`, `platform`, `executor`)

### If you need test results container

Use `schemas/tcms/test-results-container.schema.v1.json`:

**Before:**
```json
{
  "date": "2024-01-15",
  "product": "My Product",
  "description": "Test results"
}
```

**After:**
```json
{
  "type": "test_results_container",
  "schema": "tcms/test-results-container.schema.v1.json",
  "title": "Test results",
  "project": "My Product",
  "test_date": "2024-01-15T10:30:00Z",
  "test_results": [...],
  "metadata": {
    "execution_duration": 120.5,
    "total_test_cases": 10,
    "passed_test_cases": 9,
    "failed_test_cases": 1
  }
}
```

**Mapping:**
- `description` → `title`
- `product` → `project`
- `date` → `test_date` (convert to ISO 8601 format)
- Add envelope fields (`type`, `schema`)
- Add `test_results` array
- Add `metadata` with execution metrics

## Comparison: Old vs New

| Feature | Legacy (container/schema.json) | V1 Container Config | V1 Test Results Container |
|---------|--------------------------------|---------------------|---------------------------|
| Fields | 3 (date, product, description) | 7 (type, schema, title, project, environment, platform, executor) | 7 (type, schema, title, project, test_date, test_results, metadata) |
| Envelope | ❌ No | ✅ Yes | ✅ Yes |
| Versioning | ❌ No | ✅ Yes (v1) | ✅ Yes (v1) |
| JSON Schema | draft-04 | draft-07 | draft-07 |
| Purpose | Unclear | Configuration | Results aggregation |
| Test data | ❌ No | ❌ No | ✅ Yes (test_results array) |
| Metadata | ❌ No | Partial | ✅ Yes (execution metrics) |
| Use case | Legacy/unknown | Configure containers | Store test results |

## Decision Guide

Choose the appropriate replacement based on your use case:

### Use container-config.schema.v1.json if:
- ✅ You need to configure container metadata
- ✅ You're wrapping test results with environment/platform/executor info
- ✅ You don't need to store actual test results in the same document

### Use test-results-container.schema.v1.json if:
- ✅ You need to store multiple test case results
- ✅ You need execution metrics (duration, counts)
- ✅ You're aggregating test results from multiple test cases
- ✅ You need a complete test report document

### Most common scenario:
**Use test-results-container.schema.v1.json** - It includes all metadata from container-config plus test results and execution metrics.

## Timeline

- **Now:** Schema marked as deprecated
- **Phase 1:** Identify all uses of this schema
- **Phase 2:** Migrate to appropriate v1 schema based on use case
- **Phase 3:** Add errors when this schema is detected
- **Phase 4:** Remove deprecated schema (after 6-month transition period)

## Questions?

See the main schema documentation:
- [schemas/README.md](../../README.md) - Overview
- [schemas/SCHEMA_AUDIT.md](../../SCHEMA_AUDIT.md) - Full audit report
- [schemas/SCHEMA_QUICK_REFERENCE.md](../../SCHEMA_QUICK_REFERENCE.md) - Quick reference
