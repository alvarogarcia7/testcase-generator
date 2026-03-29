---
id: TCMS-17
title: Add Schema and Type to each file payload
status: In Progress
assignee: []
created_date: '2026-03-20 06:48'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
All JSON and YAML input/output files should have a
1) 'type' field: test_execution, test_result, test_verification, test_case, etc
2) 'schema': 'tcms/test_execution.schema.json@v1', etc.

A) Create a json schema to validate these fields on all files.
B) When receiving a yaml or json, read these properties, verify the payload against the schema given.
<!-- SECTION:DESCRIPTION:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 All tests are passing. Run make test
<!-- DOD:END -->
