---
id: TCMS-26
title: Generation of shell scripts must support JSON-invalid output
status: In Progress
assignee: []
created_date: '2026-04-01 07:52'
updated_date: '2026-04-01 07:56'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
# Use jq for robust JSON output capture in generated scripts

Replace the current JSON escaping approach (COMMAND_OUTPUT variable + json-escape binary or sed/awk fallback) with jq-based direct capture that pipes command output through jq -Rs . to produce valid JSON strings, eliminating shell variable assignment issues with special characters and simplifying the escaping logic.

## Key Decisions

Use jq with the -Rs flags (read raw input as string) to convert arbitrary command output directly into JSON-escaped strings, replacing the current two-step approach of capturing to COMMAND_OUTPUT and then escaping

Keep json-escape binary and shell fallback as a secondary option controlled by config for environments where jq is not available, introducing a new JsonEscapingMethod::Jq variant that is checked first in Auto mode

Store command output in temporary files and use jq to read them rather than piping directly, avoiding bash variable assignment issues when output contains special characters like quotes or backslashes

## Tasks

Add JsonEscapingMethod::Jq variant to crates/testcase-common/src/config.rs enum and update Auto mode logic to prioritize jq > json-escape binary > shell fallback based on availability. Update JsonEscapingConfig with jq_path option similar to binary_path.

Refactor generate_json_escaping_code in crates/testcase-execution/src/executor.rs to implement jq-based escaping: write COMMAND_OUTPUT to a temporary file, then use OUTPUT_ESCAPED=$(jq -Rs . < tmpfile) to read and escape it. Implement detection of jq availability in Auto mode using command -v jq. Keep existing json-escape and shell fallback code as alternative branches.

Update generate_test_script_with_json_output in crates/testcase-execution/src/executor.rs to write command output directly to temp files before JSON escaping: change COMMAND_OUTPUT=$({ eval ...; } 2>&1 | tee "$LOG_FILE") to write to a temp capture file first, then use jq to read it. Handle cleanup of temp files in the script cleanup trap.

Add comprehensive unit tests in crates/testcase-execution/src/executor.rs covering: (1) generated script contains jq command when JsonEscapingMethod::Jq is configured, (2) generated script contains fallback logic in Auto mode checking for jq then json-escape then shell, (3) script correctly handles temp file creation and cleanup in cleanup trap, (4) edge cases with empty output and binary output.

Create E2E test script crates/testcase-manager/tests/integration/test_jq_json_escaping_e2e.sh that: (1) creates test case YAML with commands producing special characters (quotes, backslashes, Unicode, newlines, control characters, binary data), (2) generates scripts with JsonEscapingMethod::Jq, executes them, validates JSON logs parse correctly via jq, (3) tests with jq removed from PATH to verify fallback to json-escape, (4) compares output between jq, json-escape, and shell fallback methods for consistency. Wire into mk/incremental.mk E2E mapping under test-executor crate.

Add Rust integration tests in crates/testcase-manager/tests/jq_json_escaping_test.rs that: (1) build TestCase with commands containing special characters, (2) set config to JsonEscapingMethod::Jq and generate scripts programmatically, (3) execute and parse JSON logs via serde_json, (4) assert command and output fields roundtrip correctly, (5) verify jq fallback behavior when binary is unavailable.

Update test_script_generation_acceptance_e2e.sh test case 9 (special characters) to explicitly test jq-based escaping by configuring JsonEscapingMethod::Jq, and add a new test case 10 that validates Auto mode detection and fallback chain (jq → json-escape → shell) works correctly.

Update AGENTS.md documentation to add a section on JSON escaping methods explaining: (1) jq is now the preferred method for JSON output capture due to robustness, (2) Auto mode prioritizes jq > json-escape > shell fallback, (3) how to configure json_escaping.method in config TOML, (4) troubleshooting JSON parsing errors with invalid output characters, (5) requirements for jq availability in production environments.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Output that is not valid JSON is safely stored in JSON file. The output is a valid JSON file
- [ ] #2 Output that is valid JSON is still safely stored in JSON file (unmodified). The output is a valid JSON file
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- JQ is present in the test machine, use it.
- Example characters: printing borders for a table in CLI
<!-- SECTION:NOTES:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 All tests are passing. Run make test
<!-- DOD:END -->
