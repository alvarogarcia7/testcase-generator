# TCMS Schema Documentation

This directory contains JSON schemas for the Test Case Management System (TCMS).

## 📚 Documentation Files

### Quick Start
- **[SCHEMA_QUICK_REFERENCE.md](SCHEMA_QUICK_REFERENCE.md)** - Fast lookup guide for schema selection and usage

### Comprehensive Analysis
- **[SCHEMA_AUDIT.md](SCHEMA_AUDIT.md)** - Full audit report with duplication analysis, migration guidance, and recommendations
- **[SCHEMA_AUDIT.csv](SCHEMA_AUDIT.csv)** - Spreadsheet format of all schemas for easy filtering and analysis

### Deprecation and Consolidation
- **[tcms/CONTAINER_SCHEMAS.md](tcms/CONTAINER_SCHEMAS.md)** - Container schema consolidation guide (explains removal of redundant schemas)

## 🎯 Current Production Schemas (v1)

All production schemas are located in `schemas/tcms/*.schema.v1.json` and follow the envelope pattern.

### Core
- `tcms-envelope.schema.json` - Meta-schema defining envelope pattern

### Document Types
1. **test_case** - `tcms/test-case.schema.v1.json` - Test case definitions
2. **test_execution** - `tcms/test-execution.schema.v1.json` - Execution log entries
3. **test_verification** - `tcms/test-verification.schema.v1.json` - Verification results
4. **test_result** - `tcms/test-result.schema.v1.json` - Test results (alternative to test_verification)
5. **test_results_container** - `tcms/test-results-container.schema.v1.json` - Container for multiple results
6. **container_config** - `tcms/container-config.schema.v1.json` - Container metadata configuration

## 🔬 Verification Methods

Specialized schemas for different verification methodologies in `tcms/verification_methods/`:

- **test** - Test-based verification
- **analysis** - Analytical verification with calculations
- **demonstration** - Operational demonstrations
- **inspection** - Inspection/review verification
- **common_criteria** - Security evaluation (EAL1-7)
- **high_assurance** - DO-178C aviation safety verification
- **result** - Generic result reporting

## ⚠️ Schema Status Summary

### Active Schemas
- **7 versioned (v1)** - Current production standard with envelope support
- **7 verification methods** - Domain-specific methodologies

### Removed Schemas (Consolidated)
- **3 redundant container schemas** - Removed and consolidated into canonical v1 schema
  - `tcms/container/schema.json` (minimal legacy - 3 fields only)
  - `tcms/test_results/container_schema.json` (loose typing)
  - `tcms/testcase_results_container/schema.json` (different encoding)
- **See:** [tcms/CONTAINER_SCHEMAS.md](tcms/CONTAINER_SCHEMAS.md) for details and migration guidance

### Transitional Schemas
- **5 root-level schemas** - Have optional envelope support, migrate to v1
  - `test-case.schema.json`
  - `container_config.schema.json`
  - `execution-log.schema.json`
  - `verification-output.schema.json`
  - `verification-result.schema.json`

## 📖 Quick Schema Selection

### I need to...
- **Define test cases** → `tcms/test-case.schema.v1.json`
- **Record execution logs** → `tcms/test-execution.schema.v1.json`
- **Store single test results** → `tcms/test-verification.schema.v1.json` or `tcms/test-result.schema.v1.json`
- **Aggregate multiple results** → `tcms/test-results-container.schema.v1.json`
- **Configure container metadata** → `tcms/container-config.schema.v1.json`
- **Use specific verification method** → `tcms/verification_methods/{type}/schema.json`

## 🔄 Envelope Pattern

All v1 schemas follow the envelope pattern with required fields:

```json
{
  "type": "test_case",
  "schema": "tcms/test-case.schema.v1.json",
  ...
}
```

Valid types: `test_case`, `test_execution`, `test_verification`, `test_result`, `container_config`, `test_results_container`

## 📊 Statistics

- **Total schemas:** 19 (3 redundant schemas removed)
- **Production (v1):** 7 (37%)
- **Verification methods:** 7 (37%)
- **Transitional:** 5 (26%)
- **Removed/Consolidated:** 3 container schemas

## 🔍 Schema Consolidation

### Container Schemas
Three redundant container schemas were identified and removed:
- `tcms/container/schema.json`
- `tcms/test_results/container_schema.json`
- `tcms/testcase_results_container/schema.json`

These have been **consolidated** into:
- **Canonical:** `tcms/test-results-container.schema.v1.json` (with envelope)
- **Working:** `data/testcase_results_container/schema.json` (backward compatible, used by verifier)

See [tcms/CONTAINER_SCHEMAS.md](tcms/CONTAINER_SCHEMAS.md) for details and migration guidance.

## 🚀 Usage Example

### Creating a Test Case
```json
{
  "type": "test_case",
  "schema": "tcms/test-case.schema.v1.json",
  "requirement": "REQ-001",
  "item": 1,
  "tc": 1,
  "id": "TC-001",
  "description": "Example test case",
  "general_initial_conditions": {},
  "initial_conditions": {},
  "test_sequences": [
    {
      "id": 1,
      "name": "Test Sequence 1",
      "description": "First test sequence",
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

### Creating a Results Container
```json
{
  "type": "test_results_container",
  "schema": "tcms/test-results-container.schema.v1.json",
  "title": "Test Run Results",
  "project": "My Project",
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

## 📋 Directory Structure

```
schemas/
├── README.md                          # This file
├── SCHEMA_AUDIT.md                    # Comprehensive audit report
├── SCHEMA_AUDIT.csv                   # Spreadsheet format
├── SCHEMA_QUICK_REFERENCE.md          # Quick lookup guide
├── tcms-envelope.schema.json          # Envelope meta-schema
├── tcms/
│   ├── *.schema.v1.json              # 7 versioned schemas (PRODUCTION)
│   ├── CONTAINER_SCHEMAS.md           # Container schema consolidation guide
│   └── verification_methods/          # 7 verification method schemas (ACTIVE)
│       ├── test/
│       ├── analysis/
│       ├── demonstration/
│       ├── inspection/
│       ├── common_criteria/
│       ├── high_assurance/
│       └── result/
└── *.schema.json                      # Root-level transitional schemas
```

## 🎓 Best Practices

1. ✅ **Always use v1 schemas** for new code
2. ✅ **Always include envelope fields** (`type`, `schema`)
3. ✅ **Use JSON Schema draft-07** for new schemas
4. ❌ **Avoid legacy schemas** without envelope support
5. 📚 **Document schema selection** in your code
6. 🔄 **Plan migration** from transitional schemas
7. ✅ **Validate against schemas** before processing

## 🛠️ Validation

### Using ajv (Node.js)
```javascript
const Ajv = require('ajv');
const ajv = new Ajv();
const schema = require('./schemas/tcms/test-case.schema.v1.json');
const valid = ajv.validate(schema, data);
if (!valid) console.log(ajv.errors);
```

### Using jsonschema (Python)
```python
import jsonschema
import json

with open('schemas/tcms/test-case.schema.v1.json') as f:
    schema = json.load(f)

jsonschema.validate(instance=data, schema=schema)
```

## 🔗 Related Tools

- **validate-yaml** - YAML test case validator (uses test-case schema)
- **test-executor** - Test execution engine (generates test-execution logs)
- **verifier** - Test verification tool (generates test-verification/test-result outputs)
- **test-orchestrator** - Test orchestration (uses container schemas)

## 📞 Support

For schema-related questions:
1. Check [SCHEMA_QUICK_REFERENCE.md](SCHEMA_QUICK_REFERENCE.md)
2. Review [SCHEMA_AUDIT.md](SCHEMA_AUDIT.md)
3. Examine schema files directly
4. Consult tool-specific documentation

## 🗺️ Roadmap

### Completed
- ✅ Envelope pattern implementation
- ✅ Version 1 schema migration
- ✅ Schema audit and documentation
- ✅ Container schema consolidation (removed 3 redundant schemas)

### In Progress
- 🔄 Migration guides for transitional schemas
- 🔄 Tooling updates to use v1 schemas

### Planned
- 📅 Envelope support for verification methods
- 📅 Migration to JSON Schema draft-07 for all schemas
- 📅 Automated schema testing and validation tools
- 📅 Schema versioning policy and v2 planning

## 📜 License

See project LICENSE file for schema licensing information.

---

**Last Updated:** 2024
**Schema Version:** v1
**Total Schemas:** 19 (after consolidation)
