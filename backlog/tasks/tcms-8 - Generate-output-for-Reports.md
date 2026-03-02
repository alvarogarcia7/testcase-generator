---
id: TCMS-8
title: Generate output for Reports
status: To Do
assignee: []
created_date: '2026-02-27 17:31'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
1. Send the new schemas to TCMS Schemas (~/repos/tcms-schemas)
2. Inspect the expected output files:
testcases/expected_output_reports/container_data.yml
testcases/expected_output_reports/sample_gsma_4.4.2.2_TC.yml
testcases/expected_output_reports/sample_gsma_4.4.2.3_TC.yml
testcases/expected_output_reports/sample_gsma_4.4.2.4_AN.yml
testcases/expected_output_reports/sample_gsma_4.4.2.5_DM.yml
testcases/expected_output_reports/sample_gsma_4.4.2.6_IN.yml
3. Create a new executable: verifier
4. Has modes:
    - Single file: single test case and single execution log
    - Folder: discover (recursively) all test cases and all execution log. The execution log (TEST_VAR_PASSING_001_execution_log.json belongs to the test case with ID=TEST_VAR_PASSING_001).
5. Has parameters:
  - Accepts a execution_log (like TEST_VAR_PASSING_001_execution_log.json)
  - Accepts a test case (like gsma_4.4.2.2_TC.yml)
4. Use the logger:
  - An INFO line for each file processed
  - An ERROR line for each step, test_sequence, test case that is not passing
<!-- SECTION:DESCRIPTION:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 All tests are passing. Run make test
<!-- DOD:END -->
