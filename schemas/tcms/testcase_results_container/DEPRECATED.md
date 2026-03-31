# ⚠️ DEPRECATED SCHEMA

**This directory contains a deprecated schema that should not be used in new code.**

## Schema in this directory

### schema.json
- **Status:** ⚠️ CONSIDER FOR DEPRECATION
- **Reason:** Legacy container without envelope support, different step result encoding
- **Use instead:** `schemas/tcms/test-results-container.schema.v1.json`
- **Migration:** Add envelope fields, convert step results from pair-based to tagged enum format

## Why this schema should be deprecated

1. **No envelope support** - Missing required `type` and `schema` fields
2. **Not versioned** - Cannot evolve without breaking changes
3. **Different encoding** - Uses expected/actual pairs instead of externally tagged enums
4. **Complex definitions** - Has TestStepExecution, expectedOnlyPair, passFailResult (different approach than v1)
5. **Superseded** - Versioned v1 schema provides comprehensive functionality with clearer structure

## Migration Guide

### From schema.json to test-results-container.schema.v1.json

This migration is more complex due to different step result encoding.

#### Step Results Encoding Change

**Before (pair-based):**
```json
{
  "Pass": {
    "step": 1,
    "description": "Test step",
    "result": {
      "expected": "0",
      "actual": "0"
    },
    "output": {
      "expected": "hello",
      "actual": "hello"
    }
  }
}
```

**After (tagged enum with expected in Fail only):**
```json
{
  "Pass": {
    "step": 1,
    "description": "Test step"
  }
}
```

**Before (Fail with pairs):**
```json
{
  "Fail": {
    "step": 2,
    "description": "Test step",
    "reason": "Mismatch",
    "result": {
      "expected": "0",
      "actual": "1"
    },
    "output": {
      "expected": "success",
      "actual": "error"
    }
  }
}
```

**After (Fail with separate expected/actual):**
```json
{
  "Fail": {
    "step": 2,
    "description": "Test step",
    "expected": {
      "result": "0",
      "output": "success"
    },
    "actual_result": "1",
    "actual_output": "error",
    "reason": "Mismatch"
  }
}
```

**Before (NotExecuted with expected only):**
```json
{
  "NotExecuted": {
    "step": 3,
    "description": "Test step",
    "result": {
      "expected": "0"
    },
    "output": {
      "expected": "output"
    }
  }
}
```

**After (NotExecuted simplified):**
```json
{
  "NotExecuted": {
    "step": 3,
    "description": "Test step"
  }
}
```

#### Complete Migration Example

**Before:**
```json
{
  "title": "Test Results",
  "project": "My Project",
  "test_date": "2024-01-15T10:30:00Z",
  "test_results": [
    {
      "requirement": "REQ-001",
      "item": 1,
      "tc": 1,
      "test_case_id": "TC-001",
      "description": "Test case",
      "sequences": [
        {
          "sequence_id": 1,
          "name": "Sequence 1",
          "step_results": [
            {
              "Pass": {
                "step": 1,
                "description": "Step 1",
                "result": {
                  "expected": "0",
                  "actual": "0"
                },
                "output": {
                  "expected": "ok",
                  "actual": "ok"
                }
              }
            }
          ],
          "all_steps_passed": true
        }
      ],
      "total_steps": 1,
      "passed_steps": 1,
      "failed_steps": 0,
      "not_executed_steps": 0,
      "overall_pass": true
    }
  ],
  "metadata": {
    "environment": "Dev",
    "platform": "Linux",
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
      "description": "Test case",
      "requirement": "REQ-001",
      "item": 1,
      "tc": 1,
      "sequences": [
        {
          "sequence_id": 1,
          "name": "Sequence 1",
          "step_results": [
            {
              "Pass": {
                "step": 1,
                "description": "Step 1"
              }
            }
          ],
          "all_steps_passed": true
        }
      ],
      "total_steps": 1,
      "passed_steps": 1,
      "failed_steps": 0,
      "not_executed_steps": 0,
      "overall_pass": true
    }
  ],
  "metadata": {
    "environment": "Dev",
    "platform": "Linux",
    "executor": "Jenkins",
    "execution_duration": 120.5,
    "total_test_cases": 1,
    "passed_test_cases": 1,
    "failed_test_cases": 0
  }
}
```

**Key changes:**
1. Add envelope fields (`type`, `schema`)
2. Convert Pass step results: Remove `result`/`output` pairs (only needed for Fail)
3. Convert Fail step results: Flatten to `expected` object + `actual_result`/`actual_output` strings
4. Convert NotExecuted: Remove `result`/`output` fields entirely
5. Add execution metrics to metadata
6. Move requirement/item/tc to test result level (v1 supports multi-level)

## Encoding Comparison

| Aspect | Legacy (Pairs) | V1 (Tagged Enum) |
|--------|----------------|------------------|
| Pass | Includes expected/actual pairs | Only step and description |
| Fail | Pairs in result/output | expected object + actual_result/actual_output |
| NotExecuted | Expected-only pairs | Only step and description |
| Philosophy | Always show expected/actual | Only show expected/actual on failure |
| Verbosity | More verbose (pairs always) | Cleaner (pairs only when needed) |

## Benefits of V1 Encoding

1. **Clearer intent** - Pass means "as expected", no need to repeat values
2. **Less redundant** - Don't store expected/actual when they match
3. **Simpler** - NotExecuted doesn't need expected values
4. **Consistent** - All relevant information in Fail where it matters
5. **Smaller** - Reduced JSON size for passed tests

## Timeline

- **Now:** Schema marked for potential deprecation
- **Phase 1:** Assess usage and impact
- **Phase 2:** Create automated migration tool for step result encoding
- **Phase 3:** Update tooling to use v1 schema
- **Phase 4:** Add warnings when this schema is detected
- **Phase 5:** Remove deprecated schema (after transition period)

## Questions?

See the main schema documentation:
- [schemas/README.md](../../README.md) - Overview
- [schemas/SCHEMA_AUDIT.md](../../SCHEMA_AUDIT.md) - Full audit report
- [schemas/SCHEMA_QUICK_REFERENCE.md](../../SCHEMA_QUICK_REFERENCE.md) - Quick reference
