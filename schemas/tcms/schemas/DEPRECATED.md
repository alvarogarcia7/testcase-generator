# ⚠️ DEPRECATED SCHEMAS

**This directory contains deprecated schemas that should not be used in new code.**

## Schemas in this directory

### verification_schema.json
- **Status:** ❌ DEPRECATED
- **Reason:** Duplicate of versioned schema without envelope support
- **Use instead:** `schemas/tcms/test-verification.schema.v1.json`
- **Migration:** Add `type: "test_verification"` and `schema: "tcms/test-verification.schema.v1.json"` fields

### verification-schema.json
- **Status:** ❌ DEPRECATED
- **Reason:** Simplified duplicate of versioned schema without envelope support
- **Use instead:** `schemas/tcms/test-verification.schema.v1.json`
- **Migration:** Add envelope fields and expand Fail structure to include expected/actual fields

## Why these schemas are deprecated

1. **No envelope support** - Missing required `type` and `schema` fields
2. **Not versioned** - Cannot evolve without breaking changes
3. **Superseded** - Versioned v1 schemas provide all functionality and more
4. **Maintenance burden** - Multiple schemas serving the same purpose

## Migration Guide

### From verification_schema.json to test-verification.schema.v1.json

**Before:**
```json
{
  "test_case_id": "TC-001",
  "description": "Test case",
  "requirement": "REQ-001",
  "item": 1,
  "tc": 1,
  "sequences": [...],
  "total_steps": 5,
  "passed_steps": 5,
  "failed_steps": 0,
  "not_executed_steps": 0,
  "overall_pass": true
}
```

**After:**
```json
{
  "type": "test_verification",
  "schema": "tcms/test-verification.schema.v1.json",
  "test_case_id": "TC-001",
  "description": "Test case",
  "requirement": "REQ-001",
  "item": 1,
  "tc": 1,
  "sequences": [...],
  "total_steps": 5,
  "passed_steps": 5,
  "failed_steps": 0,
  "not_executed_steps": 0,
  "overall_pass": true
}
```

**Key changes:**
1. Add `"type": "test_verification"` field
2. Add `"schema": "tcms/test-verification.schema.v1.json"` field
3. All other fields remain the same
4. V1 schema supports requirement/item/tc at multiple levels (test, sequence, step)

### From verification-schema.json to test-verification.schema.v1.json

**Before (simplified Fail):**
```json
{
  "Fail": {
    "step": 1,
    "description": "Test step",
    "reason": "Output mismatch"
  }
}
```

**After (complete Fail):**
```json
{
  "Fail": {
    "step": 1,
    "description": "Test step",
    "expected": {
      "result": "0",
      "output": "expected output"
    },
    "actual_result": "1",
    "actual_output": "actual output",
    "reason": "Output mismatch"
  }
}
```

**Key changes:**
1. Add envelope fields (`type`, `schema`)
2. Expand Fail structure to include `expected`, `actual_result`, `actual_output` fields
3. Add optional requirement/item/tc tracking

## Timeline

- **Now:** Schemas marked as deprecated
- **Phase 1:** Update tooling to use v1 schemas
- **Phase 2:** Add warnings when these schemas are detected
- **Phase 3:** Remove deprecated schemas (after 6-month transition period)

## Questions?

See the main schema documentation:
- [schemas/README.md](../../README.md) - Overview
- [schemas/SCHEMA_AUDIT.md](../../SCHEMA_AUDIT.md) - Full audit report
- [schemas/SCHEMA_QUICK_REFERENCE.md](../../SCHEMA_QUICK_REFERENCE.md) - Quick reference
