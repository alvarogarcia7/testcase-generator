---
id: TCMS-22
title: Script Generation Acceptance E2E Test
status: In Progress
assignee: []
created_date: '2026-03-27 09:41'
updated_date: '2026-04-02 10:30'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Script Generation Acceptance E2E Test

# Organize test-acceptance folder structure with numbered directories

Refactor test-acceptance directory to use a hierarchical numbered folder structure (00_test_cases, 05_scripts, 10_test_results, 20_verification, 30_documentation_source) by updating all validation scripts, orchestrator script, and E2E tests to reference the new paths while maintaining backward compatibility through symlinks.

## Key Decisions

Use numbered prefixes (00_, 05_, 10_, 20_, 30_) for directory organization to enforce processing order and improve visual hierarchy, following common practices in complex test frameworks

Consolidate execution_logs into 10_test_results alongside generated scripts output to simplify the data flow and reduce directory fragmentation

Create a migration script that moves existing content and establishes symlinks from old paths to new paths, allowing gradual transition and preventing breakage of external tooling

## Tasks

Create test-acceptance/migrate_to_organized_structure.sh migration script that: (1) creates the new directory structure (00_test_cases, 05_scripts, 10_test_results, 20_verification, 30_documentation_source), (2) moves existing content from old paths (test_cases → 00_test_cases, scripts → 05_scripts, execution_logs → 10_test_results/execution_logs, verification_results → 20_verification, reports → 30_documentation_source), (3) creates symlinks from old directory names to new locations for backward compatibility, (4) logs all operations with dry-run mode support via --dry-run flag, (5) validates directory structure after migration. Make the script idempotent.

Update all path constants in validation scripts (validate_stage1_yaml.sh, validate_stage2_scripts.sh, validate_stage3_execution.sh, validate_stage4_verification.sh, validate_stage5_tpdg_result_docs.sh, validate_stage6_tpdg_plan_docs.sh, validate_stage7_consolidated_docs.sh) to use new directory names: TEST_CASES_DIR="$SCRIPT_DIR/00_test_cases", SCRIPTS_DIR="$SCRIPT_DIR/05_scripts", EXECUTION_LOGS_DIR="$SCRIPT_DIR/10_test_results/execution_logs", VERIFICATION_RESULTS_DIR="$SCRIPT_DIR/20_verification", REPORTS_DIR="$SCRIPT_DIR/30_documentation_source". Update help text and comments to reflect new structure.

Update test-acceptance/run_acceptance_suite.sh orchestrator script to use new directory paths and add documentation section at the top explaining the numbered directory structure. Update stage descriptions in usage() to reference the new numbered folders. Ensure all mkdir -p calls and directory existence checks use the new paths.

Update test-acceptance/generate_final_report.sh to reference new directory paths and update the report output to use 30_documentation_source/reports/final as the default output directory. Update all log file path defaults to search in new locations.

Update test-acceptance/tests/test_acceptance_suite_e2e.sh E2E test to use new directory structure in create_test_environment function: create 00_test_cases, 05_scripts, 10_test_results/execution_logs, 20_verification, 30_documentation_source directories. Update all path references and test assertions to use new names.

Add test-acceptance/README.md documentation file explaining the numbered directory structure: (1) overview section describing the 5-folder organization, (2) table listing each directory (00_test_cases, 05_scripts, 10_test_results, 20_verification, 30_documentation_source) with description and contents, (3) workflow diagram showing data flow through numbered directories, (4) usage examples for running validation scripts, (5) migration notes explaining symlinks for backward compatibility. Reference the validation scripts as orchestrators.
<!-- SECTION:DESCRIPTION:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 All tests are passing. Run make test
<!-- DOD:END -->
