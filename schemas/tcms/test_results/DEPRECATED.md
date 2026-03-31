# ⚠️ DEPRECATED SCHEMA

**This directory contains a deprecated schema that should not be used in new code.**

## Schema in this directory

### container_schema.json
- **Status:** ⚠️ CONSIDER FOR DEPRECATION
- **Reason:** Legacy container without envelope support, loosely typed test_results
- **Use instead:** `schemas/tcms/test-results-container.schema.v1.json`
- **Migration:** Add envelope fields, ensure test_results conform to full schema, update metadata requirements

## Why this schema should be deprecated

1. **No envelope support** - Missing required `type` and `schema` fields
2. **Not versioned** - Cannot evolve without breaking changes
3. **Loose typing** - `test_results` items are generic `object` type
4. **Different metadata model** - Requires environment/platform/executor but not execution metrics
5. **Superseded** - Versioned v1 schema provides comprehensive functionality

## Migration Guide

### From container_schema.json to test-results-container.schema.v1.json

**Before:**
```json
{
  "title": "Test Results",
  "project": "My Project",
  "test_date": "2024-01-15T10:30:00Z",
  "test_results": [
    {
      "test_case_id": "TC-001",
      ...
    }
  ],
  "metadata": {
    "environment": "Development",
    "platform": "Linux x86_64",
    "executor": "Jenkins"
  }
}
```

**After:**
```json
{
  "type": "test_results_container",
  "schema": "tcms/test-results-container.schema.v1.json",
  "title": "Test Results",
  "project": "My Project",
  "test_date": "2024-01-15T10:30:00Z",
  "test_results": [
    {
      "test_case_id": "TC-001",
      "description": "Test case description",
      "sequences": [...],
      "total_steps": 5,
      "passed_steps": 5,
      "failed_steps": 0,
      "not_executed_steps": 0,
      "overall_pass": true
    }
  ],
  "metadata": {
    "environment": "Development",
    "platform": "Linux x86_64",
    "executor": "Jenkins",
    "execution_duration": 120.5,
    "total_test_cases": 1,
    "passed_test_cases": 1,
    "failed_test_cases": 0
  }
}
```

**Key changes:**
1. Add `"type": "test_results_container"` field
2. Add `"schema": "tcms/test-results-container.schema.v1.json"` field
3. Ensure `test_results` items are fully typed with all required fields
4. Add execution metrics to metadata:
   - `execution_duration` (required)
   - `total_test_cases` (required)
   - `passed_test_cases` (required)
   - `failed_test_cases` (required)
5. Environment/platform/executor become optional in v1

## Comparison: Metadata Requirements

| Field | Legacy | v1 Schema |
|-------|--------|-----------|
| `environment` | Required | Optional |
| `platform` | Required | Optional |
| `executor` | Required | Optional |
| `execution_duration` | Optional | **Required** |
| `total_test_cases` | Optional | **Required** |
| `passed_test_cases` | Optional | **Required** |
| `failed_test_cases` | Optional | **Required** |

**Philosophy change:** V1 prioritizes execution metrics (duration, counts) over environment details, making it more suitable for automated reporting and dashboards.

## Timeline

- **Now:** Schema marked for potential deprecation
- **Phase 1:** Assess usage and impact
- **Phase 2:** Update tooling to use v1 schema
- **Phase 3:** Add warnings when this schema is detected
- **Phase 4:** Remove deprecated schema (after transition period)

## Questions?

See the main schema documentation:
- [schemas/README.md](../../README.md) - Overview
- [schemas/SCHEMA_AUDIT.md](../../SCHEMA_AUDIT.md) - Full audit report
- [schemas/SCHEMA_QUICK_REFERENCE.md](../../SCHEMA_QUICK_REFERENCE.md) - Quick reference
