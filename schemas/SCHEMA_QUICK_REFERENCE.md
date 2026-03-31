# Schema Quick Reference Guide

**Quick lookup for schema selection and usage**

## Current Production Schemas (Use These!)

All versioned schemas in `schemas/tcms/*.schema.v1.json` with envelope support.

### Core Envelope
```
tcms-envelope.schema.json
```
- Meta-schema that defines envelope pattern
- Referenced by all v1 schemas
- Requires: `type`, `schema` fields

### Test Case Definition
```
schemas/tcms/test-case.schema.v1.json
```
- **Type:** `test_case`
- **Schema:** `tcms/test-case.schema.v1.json`
- **Purpose:** Define test cases with sequences, steps, initial conditions
- **Features:** Prerequisites, hooks, BDD, hydration_vars, capture_vars

### Test Execution Log
```
schemas/tcms/test-execution.schema.v1.json
```
- **Type:** `test_execution`
- **Schema:** `tcms/test-execution.schema.v1.json`
- **Purpose:** Record individual test step executions
- **Features:** Command, exit_code, output, timestamp, hook_type, verification flags

### Test Verification Result
```
schemas/tcms/test-verification.schema.v1.json
```
- **Type:** `test_verification`
- **Schema:** `tcms/test-verification.schema.v1.json`
- **Purpose:** Store verification results for single test case
- **Features:** Pass/Fail/NotExecuted steps, requirement tracking, counts

### Test Result (Alternative)
```
schemas/tcms/test-result.schema.v1.json
```
- **Type:** `test_result`
- **Schema:** `tcms/test-result.schema.v1.json`
- **Purpose:** Similar to test-verification (choose based on preference)
- **Features:** Pass/Fail/NotExecuted steps, requirement tracking, counts

### Test Results Container
```
schemas/tcms/test-results-container.schema.v1.json
```
- **Type:** `test_results_container`
- **Schema:** `tcms/test-results-container.schema.v1.json`
- **Purpose:** Aggregate multiple test results with metadata
- **Features:** Title, project, test_date, execution metrics, test_results array

### Container Configuration
```
schemas/tcms/container-config.schema.v1.json
```
- **Type:** `container_config`
- **Schema:** `tcms/container-config.schema.v1.json`
- **Purpose:** Configure metadata for wrapping test results
- **Features:** Title, project, environment, platform, executor

---

## Verification Methods (Domain-Specific)

Located in `schemas/tcms/verification_methods/*/schema.json`

### Test Method
```
schemas/tcms/verification_methods/test/schema.json
```
- **Type:** `test`
- **Purpose:** Test-based verification with test sequences

### Analysis Method
```
schemas/tcms/verification_methods/analysis/schema.json
```
- **Type:** `analysis`
- **Purpose:** Analytical verification with calculations and models

### Demonstration Method
```
schemas/tcms/verification_methods/demonstration/schema.json
```
- **Type:** `demonstration`
- **Purpose:** Operational demonstrations with procedures

### Inspection Method
```
schemas/tcms/verification_methods/inspection/schema.json
```
- **Type:** `inspection`
- **Purpose:** Inspection/review verification with checklists

### Common Criteria Method
```
schemas/tcms/verification_methods/common_criteria/schema.json
```
- **Type:** `common_criteria`
- **Purpose:** Security evaluation (EAL1-7) with SFRs, SARs, vulnerability assessment

### High Assurance Method (DO-178C)
```
schemas/tcms/verification_methods/high_assurance/schema.json
```
- **Type:** `high_assurance`
- **Purpose:** Aviation safety verification (DAL A-E) with structural coverage

### Result Method
```
schemas/tcms/verification_methods/result/schema.json
```
- **Type:** `result`
- **Purpose:** Generic result reporting for verification methods

---

## Deprecated Schemas (Do Not Use)

### ❌ Verification Duplicates
- `tcms/schemas/verification_schema.json` → Use `test-verification.schema.v1.json`
- `tcms/schemas/verification-schema.json` → Use `test-verification.schema.v1.json`

### ⚠️ Container Duplicates
- `tcms/test_results/container_schema.json` → Use `test-results-container.schema.v1.json`
- `tcms/testcase_results_container/schema.json` → Use `test-results-container.schema.v1.json`
- `tcms/container/schema.json` → Use `container-config.schema.v1.json`

### 🔄 Transitional Schemas (Optional Envelope)
- `test-case.schema.json` → Use `tcms/test-case.schema.v1.json`
- `container_config.schema.json` → Use `tcms/container-config.schema.v1.json`
- `execution-log.schema.json` → Use `tcms/test-execution.schema.v1.json`
- `verification-output.schema.json` → Use `tcms/test-result.schema.v1.json`
- `verification-result.schema.json` → Use `tcms/test-verification.schema.v1.json`

---

## Usage Examples

### Test Case with Envelope
```json
{
  "type": "test_case",
  "schema": "tcms/test-case.schema.v1.json",
  "requirement": "REQ-001",
  "item": 1,
  "tc": 1,
  "id": "TC-001",
  "description": "Test basic functionality",
  "general_initial_conditions": {},
  "initial_conditions": {},
  "test_sequences": [
    {
      "id": 1,
      "name": "Basic Test Sequence",
      "description": "Tests core functionality",
      "initial_conditions": {},
      "steps": [
        {
          "step": 1,
          "description": "Execute command",
          "command": "echo 'hello'",
          "expected": {
            "result": "0",
            "output": "hello"
          }
        }
      ]
    }
  ]
}
```

### Test Results Container with Envelope
```json
{
  "type": "test_results_container",
  "schema": "tcms/test-results-container.schema.v1.json",
  "title": "Test Execution Results",
  "project": "My Project",
  "test_date": "2024-01-15T10:30:00Z",
  "test_results": [
    {
      "test_case_id": "TC-001",
      "description": "Test basic functionality",
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

---

## Schema Selection Flowchart

```
Need to define a schema?
│
├─ Defining test cases?
│  └─ Use: tcms/test-case.schema.v1.json
│
├─ Recording execution logs?
│  └─ Use: tcms/test-execution.schema.v1.json
│
├─ Storing single test results?
│  ├─ Use: tcms/test-verification.schema.v1.json
│  └─ Or:  tcms/test-result.schema.v1.json
│
├─ Aggregating multiple test results?
│  └─ Use: tcms/test-results-container.schema.v1.json
│
├─ Configuring container metadata?
│  └─ Use: tcms/container-config.schema.v1.json
│
└─ Using specific verification methodology?
   ├─ Test → tcms/verification_methods/test/schema.json
   ├─ Analysis → tcms/verification_methods/analysis/schema.json
   ├─ Demonstration → tcms/verification_methods/demonstration/schema.json
   ├─ Inspection → tcms/verification_methods/inspection/schema.json
   ├─ Security (Common Criteria) → tcms/verification_methods/common_criteria/schema.json
   └─ Safety (DO-178C) → tcms/verification_methods/high_assurance/schema.json
```

---

## JSON Schema Versions

- **Current schemas:** JSON Schema draft-07
- **Verification methods:** JSON Schema draft-04
- **Legacy schemas:** Mixed draft-04 and draft-07

---

## Key Principles

1. ✅ **Always use versioned schemas** (`*.schema.v1.json`) for new code
2. ✅ **Always include envelope fields** (`type`, `schema`) in documents
3. ✅ **Use draft-07** JSON Schema for new schemas
4. ❌ **Avoid legacy schemas** in `tcms/schemas/`, `tcms/test_results/`, `tcms/testcase_results_container/`, `tcms/container/`
5. ❌ **Avoid root-level transitional schemas** (those without `tcms/` prefix and without `v1` version)

---

## Migration Checklist

Migrating from legacy schema to v1:

- [ ] Identify current schema in use
- [ ] Find equivalent v1 schema from table above
- [ ] Add `type` field with appropriate document type
- [ ] Add `schema` field with `tcms/{type}.schema.v{version}.json` pattern
- [ ] Update any custom fields to match v1 schema structure
- [ ] Test validation with v1 schema
- [ ] Update documentation and references
- [ ] Deploy changes
- [ ] Remove old schema references

---

## Support

For questions about schema usage:
1. Check this quick reference
2. Review full audit report in `SCHEMA_AUDIT.md`
3. Examine schema files in `schemas/tcms/*.schema.v1.json`
4. Consult envelope meta-schema in `schemas/tcms-envelope.schema.json`

---

## Legend

- ✅ **Use** - Current production standard
- ⚠️ **Consider deprecating** - Legacy but may have active users
- ❌ **Deprecated** - Do not use in new code
- 🔄 **Transitional** - Migration path available
