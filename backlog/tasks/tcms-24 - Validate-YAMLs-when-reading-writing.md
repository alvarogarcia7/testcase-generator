---
id: TCMS-24
title: Validate YAMLs when reading/writing
status: In Review
assignee: []
created_date: '2026-03-31 17:26'
updated_date: '2026-03-31 17:30'
labels: []
dependencies: []
---

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 All tests are passing. Run make test
<!-- DOD:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
## Summary
This PR implements comprehensive YAML schema validation throughout the test case management system:

Extract validation functions into a reusable library module in testcase-common
Implement automatic schema validation in all YAML loading operations
Update test files to use the new validated YAML loader
Add integration tests covering valid files, missing fields, schema violations, and error reporting
## Changes
testcase-common: New load_and_validate_yaml and parse_and_validate_yaml_string functions
test-executor: Updated to use validated YAML loader
verifier: Updated config loading with schema validation
testcase-ui: Updated editor and prompts modules
Integration tests: Comprehensive coverage of validation scenarios
<!-- SECTION:FINAL_SUMMARY:END -->
