---
id: TCMS-29
title: Create Test Result Summary from Test Results
status: In Progress
assignee: []
created_date: '2026-04-02 08:15'
updated_date: '2026-04-02 10:32'
labels: []
dependencies:
  - TCMS-8
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Should contain:
- schema + version
- test_results
  - expected + actual
  - status: pass/fail/not_executed
  - is_manual
- metadata

# Add single and multiple output modes to convert_verification_to_result_yaml.py

Extend convert_verification_to_result_yaml.py to support both --multiple mode (write one YAML file per test case, current behavior) and --single mode (write all test case results into a single YAML file as an array), making the output mode explicit via command-line flags and maintaining backward compatibility by defaulting to multiple-file mode.

## Key Decisions

Default to --multiple mode when neither flag is specified to preserve backward compatibility with existing scripts and workflows

Store single-mode output as a top-level array of result dictionaries rather than wrapping in a container object, keeping the format simple and consistent with the individual file structure

## Tasks

Add mutually exclusive --multiple and --single command-line flags to scripts/convert_verification_to_result_yaml.py, update the argument parser to accept these flags with --multiple as the default behavior when neither is specified, and update the help text and examples to document both output modes.

Refactor process_verification_json function in scripts/convert_verification_to_result_yaml.py to accept an output mode parameter, split the output logic into two helper functions write_multiple_result_files (current behavior: write one YAML per test case) and write_single_result_file (new behavior: write array of all results to a single YAML), and route to the appropriate function based on mode.

Implement write_single_result_file function in scripts/convert_verification_to_result_yaml.py that converts all test cases to result structures, stores them in a list, and writes to a single output path using yaml.dump with the same formatting options. The single-file output should be a plain YAML array containing all result dictionaries without wrapper objects.

Update main function in scripts/convert_verification_to_result_yaml.py to handle argument validation: when --single mode is used, require --output-dir to be a file path (with .yaml or .yml extension) rather than a directory, and update error messages accordingly. Preserve existing directory path behavior for --multiple mode.

Create comprehensive unit tests in scripts/test_convert_verification_to_result_yaml.py covering: (1) multiple mode with ContainerReport input creates N separate YAML files in output directory, (2) single mode with ContainerReport input creates one YAML file as array with N elements, (3) multiple mode with stdin ContainerReport produces individual files, (4) single mode with stdin produces single array file, (5) validation that single-mode output is parseable as YAML array and each element has type: result field, (6) error when --single is used with directory path instead of file path.

Update scripts/generate_documentation_reports.sh invocation of convert_verification_to_result_yaml.py at line 253 to explicitly pass --multiple flag (though optional due to default behavior), add a comment explaining the output mode choice, and document the single-mode alternative in script header comments.

Update crates/testcase-manager/tests/integration/test_documentation_generation.sh at line 171 to explicitly test both output modes: (1) existing test with --multiple flag, (2) new test section for --single mode that verifies single output file creation, parses the YAML array, and validates it contains the expected number of test case results with correct structure.
<!-- SECTION:DESCRIPTION:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 All tests are passing. Run make test
<!-- DOD:END -->
