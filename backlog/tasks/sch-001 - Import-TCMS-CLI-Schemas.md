---
id: SCH-001
title: Import TCMS CLI Schemas
status: Done
assignee: []
created_date: '2026-02-25 06:51'
updated_date: '2026-02-25 07:00'
labels: []
dependencies: []
ordinal: 1000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
### Schemas Added, from CLI
- **test-case-main.schema.json** (Draft-07): Defines test case structure with requirements, items, sequences, and steps. Supports BDD initial conditions and conditional verification patterns.
- **verification-result.schema.json** (Draft-07): Defines verification result format with Pass/Fail/NotExecuted states and expected vs actual results comparison.
- **execution-log.schema.json** (Draft-07): Defines execution log entry structure with commands, exit codes, outputs, and timestamps.

### CI/CD Pipeline

Implemented single-stage comprehensive validation:

1. **Validate schema versions** - Ensures all schemas use JSON Schema Draft-07
2. **Validate JSON syntax** - Uses jq to verify well-formed JSON
3. **Validate schema structure** - Verifies required fields (type, properties) exist
4. **Validate samples** - Validates YAML samples against corresponding schemas
<!-- SECTION:DESCRIPTION:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 All JSON Schemas are valid
- [x] #2 The pipeline is passing
<!-- DOD:END -->
