---
id: TCMS-8
title: Generate output for Reports
status: In Progress
assignee: []
created_date: '2026-02-27 17:31'
updated_date: '2026-03-16 11:01'
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

## Description

Add Container YAML Report Generation to Verifier

Enhance the verifier to generate comprehensive container-format YAML reports matching the structure in container_data.yml, including title, project, test_date, test_results array, and metadata section with execution statistics. Add CLI flags to control title/project fields and automatically compute metadata from BatchVerificationReport.
<!-- SECTION:DESCRIPTION:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
## Tasks

- Create ContainerReport struct with title/project/test_date/test_results/metadata fields instead of modifying BatchVerificationReport serialization, keeping backward compatibility with existing JSON/YAML output formats

- Add --container-output flag to verifier CLI to enable container format generation, using test_results field containing TestCaseVerificationResult structs without the BatchVerificationReport wrapper

- Generate metadata section automatically from BatchVerificationReport aggregated statistics rather than requiring manual input, using environment/platform/executor from optional CLI flags

- Create ContainerReport struct in src/verification.rs with fields: title (String), project (String), test_date (DateTime<Utc>), test_results (Vec<TestCaseVerificationResult>), and nested metadata struct containing environment (Option<String>), platform (Option<String>), executor (Option<String>), execution_duration (f64), total_test_cases (usize), passed_test_cases (usize), and failed_test_cases (usize). Derive Serialize and Deserialize traits. Add from_batch_report constructor that accepts BatchVerificationReport, title, project, and optional metadata fields.

- Add CLI flags to src/bin/verifier.rs: --container-format (boolean to enable container YAML output), --title (default: "Test Execution Results"), --project (default: "Test Case Manager - Verification Results"), --environment, --platform, --executor (all optional String fields). Update generate_output function to check container-format flag and call new generate_container_yaml_report method when enabled, passing through all metadata fields.

- Implement generate_container_yaml_report method in TestVerifier (src/verification.rs) that constructs ContainerReport from BatchVerificationReport using provided title/project/metadata, calculates execution_duration from generated_at timestamp difference if multiple reports exist (otherwise 0.0), serializes to YAML using serde_yaml with proper field ordering matching container_data.yml structure.

- Add integration tests in tests/verification_test.rs that verify container YAML report generation: test ContainerReport::from_batch_report constructor populates all fields correctly, test serialization to YAML matches expected container_data.yml structure with title/project/test_date/test_results/metadata sections, test metadata aggregation calculates correct totals from BatchVerificationReport, test YAML can be deserialized back to ContainerReport.

- Create end-to-end test in tests/report_generation_e2e_test.rs that runs verifier with --container-format flag, validates generated YAML has correct structure matching container_data.yml template, verifies test_results array contains properly formatted TestCaseVerificationResult entries with sequences and step_results, and confirms metadata section has accurate statistics.

- Update docs/report_generation.md to document the new --container-format flag and related CLI options (--title, --project, --environment, --platform, --executor), add examples showing how to generate container YAML reports, explain the container format structure and how it differs from standard JSON/YAML output, and update the workflow sections to show both legacy and container-format approaches.

- Update README.md and AGENTS.md to mention the new container YAML report format option for the verifier, add example command showing --container-format usage with metadata flags.
<!-- SECTION:PLAN:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 All tests are passing. Run make test
<!-- DOD:END -->
