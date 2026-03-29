---
id: TCMS-18
title: Split application into different crates
status: In Progress
assignee: []
created_date: '2026-03-25 07:36'
updated_date: '2026-03-25 07:38'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Split binaries into autonomous crates with dependency management

Restructure the monolithic testcase-manager crate into a Cargo workspace with separate crates for each binary, extracting shared functionality into reusable library crates. Update CI/CD pipelines to build only changed crates and their dependents using dependency analysis.
<!-- SECTION:DESCRIPTION:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
## Key Decisions

Use a Cargo workspace with each binary as a separate crate rather than keeping all binaries in src/bin/, enabling independent versioning and reducing compilation coupling

Extract shared code into domain-specific library crates (testcase-models, testcase-validation, testcase-verification, testcase-execution, testcase-storage) instead of a single monolithic library, improving modularity and allowing selective compilation

Implement CI/CD change detection using git diff and cargo-metadata to build only affected crates and their dependents, rather than always building the entire workspace

## Tasks

Create workspace structure: move workspace declaration to root Cargo.toml with members for all crates, create crates/ directory, and set up initial workspace dependencies using [workspace.dependencies] for shared dependencies like serde, anyhow, clap, chrono, etc.

Create testcase-models crate in crates/testcase-models/: move src/models.rs as lib.rs, add Cargo.toml with workspace dependencies (serde, chrono, indexmap), and export all model types (TestCase, TestSequence, Step, Expected, ActualResult, Prerequisite, etc.).

Create testcase-common crate in crates/testcase-common/: move src/yaml_utils.rs, src/envelope.rs, src/config.rs as modules, add Cargo.toml depending on testcase-models and workspace deps (serde_yaml, serde_json, anyhow, toml), re-export yaml_utils::log_yaml_parse_error, envelope::resolve_schema_from_payload, and Config types.

Create testcase-validation crate in crates/testcase-validation/: move src/validation.rs as lib.rs, src/dependency_validator.rs, src/dependency_resolver.rs, src/junit_xml_validator.rs as modules, add Cargo.toml depending on testcase-models, testcase-common and workspace deps (jsonschema, roxmltree), export SchemaValidator, DependencyValidator, validate_cross_file_dependencies, DependencyResolver, and validate_junit_xml.

Create testcase-verification crate in crates/testcase-verification/: move src/verification.rs as lib.rs, src/verification_templates.rs, src/log_cleaner.rs as modules, add Cargo.toml depending on testcase-models, testcase-common, bash-eval and workspace deps (regex, strip-ansi-escapes, serde_json), export TestVerifier, BatchVerificationReport, ContainerReport, MatchStrategy, StepVerificationResultEnum, and LogCleaner.

Create testcase-execution crate in crates/testcase-execution/: move src/executor.rs as lib.rs, src/hydration.rs, src/bdd_parser.rs as modules, add Cargo.toml depending on testcase-models, testcase-common, bash-eval and workspace deps (regex, chrono, tempfile), export TestExecutor, VarHydrator, BddStepRegistry, and parse_bdd_statement.

Create testcase-storage crate in crates/testcase-storage/: move src/storage.rs as lib.rs, src/database.rs, src/test_run_storage.rs, src/parser.rs, src/sample.rs, src/recovery.rs as modules, add Cargo.toml depending on testcase-models, testcase-common, testcase-validation and workspace deps (walkdir, serde_yaml, serde_json), export TestCaseStorage, TestRunStorage, TestCaseParser, TestCaseFilter, ConditionDatabase, SampleData, and RecoveryManager.

Create testcase-orchestration crate in crates/testcase-orchestration/: move src/orchestrator.rs as lib.rs, add Cargo.toml depending on testcase-models, testcase-execution, testcase-storage and workspace deps, export TestOrchestrator, RetryPolicy, and WorkerConfig.

Create testcase-ui crate in crates/testcase-ui/: move src/ui.rs, src/fuzzy.rs, src/prompts.rs, src/oracle.rs, src/editor.rs, src/creator.rs, src/builder.rs, src/complex_structure_editor.rs as modules, add Cargo.toml depending on testcase-models, testcase-storage and workspace deps (skim, dialoguer, edit), export print_title, TestCaseFuzzyFinder, Prompts, Oracle, TestCaseEditor, TestCaseCreator, TestCaseBuilder, ComplexStructureEditor.

Create testcase-git crate in crates/testcase-git/: move src/git.rs as lib.rs, add Cargo.toml depending on testcase-models and workspace deps (git2, chrono), export GitManager and CommitInfo.

Create testcase-cli crate in crates/testcase-cli/: move src/cli.rs as lib.rs, add Cargo.toml depending on testcase-models and workspace deps (clap), export Cli, Commands, and GitCommands.

Create validate-yaml binary crate in crates/validate-yaml/: move src/bin/validate-yaml.rs to src/main.rs, add Cargo.toml depending on testcase-validation, testcase-models, testcase-common and workspace deps (clap, notify, walkdir), configure [[bin]] section with name validate-yaml.

Create validate-json binary crate in crates/validate-json/: move src/bin/validate-json.rs to src/main.rs, add Cargo.toml depending on testcase-validation, testcase-common and workspace deps (clap), configure [[bin]] section with name validate-json.

Create test-executor binary crate in crates/test-executor/: move src/bin/test-executor.rs to src/main.rs, add Cargo.toml depending on testcase-execution, testcase-storage, testcase-models, testcase-validation and workspace deps (clap), configure [[bin]] section with name test-executor.

Create test-verify binary crate in crates/test-verify/: move src/bin/test-verify.rs to src/main.rs, add Cargo.toml depending on testcase-verification, testcase-storage, testcase-models and workspace deps (clap), configure [[bin]] section with name test-verify.

Create verifier binary crate in crates/verifier/: move src/bin/verifier.rs to src/main.rs, add Cargo.toml depending on testcase-verification, testcase-storage, testcase-models and workspace deps (clap, jsonschema, serde_json, serde_yaml), configure [[bin]] section with name verifier.

Create test-orchestrator binary crate in crates/test-orchestrator/: move src/bin/test-orchestrator.rs to src/main.rs, add Cargo.toml depending on testcase-orchestration, testcase-storage, testcase-ui and workspace deps (clap), configure [[bin]] section with name test-orchestrator.

Create test-run-manager binary crate in crates/test-run-manager/: move src/bin/test-run-manager.rs to src/main.rs, add Cargo.toml depending on testcase-storage, testcase-ui, testcase-models and workspace deps (clap, chrono), configure [[bin]] section with name trm.

Create script-cleanup binary crate in crates/script-cleanup/: move src/bin/script-cleanup.rs to src/main.rs, add Cargo.toml depending on testcase-verification and workspace deps (clap), configure [[bin]] section with name script-cleanup.

Create json-escape binary crate in crates/json-escape/: move src/bin/json-escape.rs to src/main.rs, add Cargo.toml with only workspace deps (clap, serde_json, anyhow), configure [[bin]] section with name json-escape.

Create json-to-yaml binary crate in crates/json-to-yaml/: move src/bin/json-to-yaml.rs to src/main.rs, add Cargo.toml with only workspace deps (clap, serde_json, serde_yaml, anyhow), configure [[bin]] section with name json-to-yaml.

Create test-plan-documentation-generator-compat binary crate in crates/tpdg-compat/: move src/bin/test-plan-documentation-generator-compat.rs to src/main.rs, add Cargo.toml with only workspace deps (clap, serde, serde_yaml, anyhow), configure [[bin]] section with name test-plan-documentation-generator-compat.

Create editor binary crate in crates/editor/: move src/main_editor.rs to src/main.rs, add Cargo.toml depending on testcase-ui, testcase-cli, testcase-git, testcase-storage, testcase-models, testcase-common and workspace deps (clap), configure [[bin]] section with name editor.

Update root Cargo.toml: remove old package section and bin declarations, keep only workspace configuration, move bash-eval into crates/bash-eval/, update workspace.members to include all new crates under crates/*, configure workspace.dependencies for all shared dependencies with versions from the original Cargo.toml.

Update Makefile: change build targets from cargo build --all to workspace-aware builds, update binary paths from target/{debug,release}/binary-name to reflect workspace structure, add per-crate build targets (e.g., build-validate-yaml, build-verifier, build-test-executor), keep test and lint targets building entire workspace.

Create scripts/ci-detect-changes.sh: script that uses git diff --name-only $CI_MERGE_REQUEST_DIFF_BASE_SHA...$CI_COMMIT_SHA to find changed files, maps them to crate names using path patterns (crates/*/), uses cargo metadata --format-version 1 | jq to extract reverse dependencies, outputs space-separated list of crate names to build, handles both library and binary crates, includes special case for workspace root Cargo.toml changes (builds everything).

Update .github/workflows/workspace.yml: add change detection step using scripts/ci-detect-changes.sh before build job, store affected crates list in environment variable AFFECTED_CRATES, modify build step to use cargo build -p $crate for each affected crate instead of cargo build --all, update clippy and test steps to target only affected crates, ensure full workspace build on main/master branch.

Update .gitlab-ci.yml: add change detection in before_script of rust:build-test-lint job using scripts/ci-detect-changes.sh with GitLab CI variables ($CI_MERGE_REQUEST_DIFF_BASE_SHA, $CI_COMMIT_SHA), modify build, clippy, and test commands to iterate over AFFECTED_CRATES list, add artifact collection for only affected binaries, implement fallback to full workspace build when change detection fails or on protected branches.

Update integration test scripts in tests/integration/: modify test scripts that invoke binaries to use workspace-aware paths (find binaries in target/debug/ or target/release/ regardless of workspace structure), update scripts that parse cargo output to handle workspace formatting, verify all E2E tests pass with new crate structure.

Update examples/ directory: modify examples/tty_fallback_demo.rs, examples/test_verify_demo.rs, examples/test_verify_integration.rs to import from new crate structure (testcase-verification, testcase-ui, testcase-models), update Cargo.toml [[example]] declarations to reference workspace crates, verify examples compile and run.

Create crates/README.md: document the crate architecture with dependency graph showing library crates (testcase-models → testcase-common → testcase-validation/testcase-verification/testcase-execution → testcase-orchestration) and binary crates depending on libraries, explain the autonomous microservice design where each binary crate is self-contained, document how to add new binaries or shared libraries to the workspace.

Update AGENTS.md: document new workspace structure with crates/ organization, update build commands to reflect workspace builds, add section explaining selective compilation in CI/CD when dependencies change, document per-crate development workflow (cargo build -p crate-name, cargo test -p crate-name), add troubleshooting section for workspace-related issues.
<!-- SECTION:PLAN:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 All tests are passing. Run make test
<!-- DOD:END -->
