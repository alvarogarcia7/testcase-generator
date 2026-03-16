---
id: TCMS-11
title: 'MatchStrategy::Precompute'
status: Done
assignee: []
created_date: '2026-03-16 10:15'
updated_date: '2026-03-16 10:15'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
* The Executor prints in the Test Execution JSON the result of evaluating output/result
* The Verifier, when using the Precomputed strategy, reads those fields for evaluation of the step.

Add MatchStrategy::Precomputed variant that reads result_verification_pass and output_verification_pass boolean fields from the JSON execution log entries, bypassing expected-vs-actual comparison. This requires adding the fields to TestStepExecutionEntry and TestExecutionLog, emitting them in the generated bash script JSON output, implementing Precomputed logic in verify_step_new, and adding a --match-strategy CLI flag to the verifier binary.
<!-- SECTION:DESCRIPTION:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
## Description
When MatchStrategy::Precomputed is active, both result_strategy and output_strategy are treated as Precomputed (single strategy controls both fields) rather than allowing mixed strategies like Precomputed-result with Regex-output

 - [x] Add result_verification_pass and output_verification_pass as Optional<bool> fields on TestExecutionLog and TestStepExecutionEntry with serde defaults, so existing JSON logs without these fields remain parseable and missing fields evaluate to failure in Precomputed mode
 - [x] Add a --match-strategy CLI flag to the verifier binary accepting exact/regex/contains/precomputed values, rather than separate --result-strategy and --output-strategy flags, since Precomputed inherently applies to both

## Tasks

 - [x] Add result_verification_pass and output_verification_pass as Option<bool> fields (with #[serde(default)]) to TestStepExecutionEntry in src/models.rs and to TestExecutionLog in src/verification.rs. Update the JSON log parsing in parse_json_log_content and parse_json_log_content_with_test_case_id to propagate these fields from TestStepExecutionEntry into TestExecutionLog. Update the generated bash script in generate_test_script_with_json_output (around lines 1249-1265) to emit "result_verification_pass": $VERIFICATION_RESULT_PASS and "output_verification_pass": $VERIFICATION_OUTPUT_PASS in each JSON entry.

 - [x] Add Precomputed variant to MatchStrategy enum in src/verification.rs. In verify_step_new, when self.result_strategy is Precomputed, check log.result_verification_pass == Some(true) instead of comparing expected vs actual result (missing field = failure). Apply the same for output using log.output_verification_pass. Skip the success field check in Precomputed mode since pass/fail is fully determined by the precomputed fields. Update strategy_name and matches to handle the new variant. Add a --match-strategy CLI argument to src/bin/verifier.rs accepting exact, regex, contains, or precomputed, and pass the selected strategy when constructing the TestVerifier via with_strategies. Add unit tests in tests/verification_test.rs covering Precomputed pass, fail, and missing-field scenarios.
<!-- SECTION:PLAN:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 All tests are passing. Run make test
<!-- DOD:END -->
