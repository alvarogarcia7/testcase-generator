# Acceptance Test Suite - Workflow Diagram

This document provides visual representations of the acceptance test suite workflow.

## High-Level Pipeline

```
┌─────────────────────────────────────────────────────────────────────┐
│                    run_acceptance_suite.sh                          │
│                   Master Orchestrator Script                        │
└─────────────────────────────────────────────────────────────────────┘
                                  │
                                  ▼
         ┌────────────────────────────────────────────────┐
         │  Stage 1: YAML Validation                      │
         │  • Find all test_cases/**/*.yaml               │
         │  • Validate against schemas/test-case.schema   │
         │  • Track: VALIDATION_PASSED, VALIDATION_FAILED │
         └────────────────────────────────────────────────┘
                                  │
                                  ▼
         ┌────────────────────────────────────────────────┐
         │  Stage 2: Script Generation                    │
         │  • Generate bash scripts with --json-log       │
         │  • Output: scripts/*.sh                        │
         │  • Track: GENERATION_PASSED, GENERATION_FAILED │
         └────────────────────────────────────────────────┘
                                  │
                                  ▼
         ┌────────────────────────────────────────────────┐
         │  Stage 3: Test Execution                       │
         │  • Execute automated tests (skip manual)       │
         │  • Validate JSON logs                          │
         │  • Output: execution_logs/*.json               │
         │  • Track: EXECUTION_PASSED, FAILED, SKIPPED    │
         └────────────────────────────────────────────────┘
                                  │
                                  ▼
         ┌────────────────────────────────────────────────┐
         │  Stage 4: Verification                         │
         │  • Run verifier on execution logs              │
         │  • Generate containers with metadata           │
         │  • Output: verification_results/*_container.yaml│
         │  • Track: VERIFICATION_PASSED, VERIFICATION_FAILED│
         └────────────────────────────────────────────────┘
                                  │
                                  ▼
         ┌────────────────────────────────────────────────┐
         │  Stage 5: Container Validation                 │
         │  • Validate containers against schema          │
         │  • Ensure TPDG compatibility                   │
         │  • Track: CONTAINER_VALIDATION_PASSED, FAILED  │
         └────────────────────────────────────────────────┘
                                  │
                                  ▼
         ┌────────────────────────────────────────────────┐
         │  Stage 6: Documentation Generation             │
         │  • Generate AsciiDoc reports                   │
         │  • Generate Markdown reports                   │
         │  • Output: reports/{asciidoc,markdown}/*       │
         │  • Track: DOCUMENTATION_PASSED, FAILED         │
         └────────────────────────────────────────────────┘
                                  │
                                  ▼
         ┌────────────────────────────────────────────────┐
         │  Summary Report Generation                     │
         │  • Aggregate all statistics                    │
         │  • Generate summary report                     │
         │  • Output: reports/acceptance_suite_summary.txt│
         │  • Display to console                          │
         └────────────────────────────────────────────────┘
                                  │
                                  ▼
                    ┌─────────────────────────────┐
                    │  Exit Code                  │
                    │  0: Success                 │
                    │  1: Failures detected       │
                    └─────────────────────────────┘
```

## Data Flow Diagram

```
test_cases/
│
├── TC_SUCCESS_SIMPLE_001.yaml ──────┐
├── TC_FAILURE_FIRST_001.yaml ───────┤
├── TC_MANUAL_ALL_001.yaml ──────────┤
└── ... (93 test cases) ─────────────┤
                                     │
                          [Stage 1: Validation]
                                     │
                          ┌──────────┴──────────┐
                          ▼                     ▼
                      ✓ Valid               ✗ Invalid
                          │                     │
                          │                [Track Failure]
                          │
                    [Stage 2: Generation]
                          │
                          ▼
                    scripts/
                    ├── TC_SUCCESS_SIMPLE_001.sh
                    ├── TC_FAILURE_FIRST_001.sh
                    ├── TC_MANUAL_ALL_001.sh
                    └── ...
                          │
                    [Stage 3: Execution]
                          │
        ┌─────────────────┼─────────────────┐
        ▼                 ▼                 ▼
   Automated         Manual Test        Failure
   Success           (Skip)             (Track)
        │                                   │
        └───────────┬─────────────────────┬─┘
                    ▼
            execution_logs/
            ├── TC_SUCCESS_SIMPLE_001.json
            ├── TC_FAILURE_FIRST_001.json
            └── ...
                    │
            [Stage 4: Verification]
                    │
                    ▼
        verification_results/
        ├── TC_SUCCESS_SIMPLE_001_container.yaml
        ├── TC_FAILURE_FIRST_001_container.yaml
        └── ...
                    │
        [Stage 5: Container Validation]
                    │
            ┌───────┴───────┐
            ▼               ▼
        ✓ Valid         ✗ Invalid
            │               │
            │          [Track Failure]
            │
    [Stage 6: Documentation]
            │
            ▼
        reports/
        ├── asciidoc/
        │   ├── TC_SUCCESS_SIMPLE_001.adoc
        │   ├── TC_FAILURE_FIRST_001.adoc
        │   └── ...
        ├── markdown/
        │   ├── TC_SUCCESS_SIMPLE_001.md
        │   ├── TC_FAILURE_FIRST_001.md
        │   └── ...
        └── acceptance_suite_summary.txt
```

## Function Call Graph

```
main()
 │
 ├──> verify_binaries()
 │     └──> Checks: validate-yaml, test-executor, verifier, validate-json, TPDG
 │
 ├──> validate_test_cases()
 │     ├──> find test_cases/**/*.yaml
 │     ├──> validate-yaml --schema <schema> <file>
 │     └──> Track: VALIDATION_PASSED, VALIDATION_FAILED
 │
 ├──> generate_test_scripts()
 │     ├──> find test_cases/**/*.yaml
 │     ├──> test-executor generate --json-log --output <script> <yaml>
 │     ├──> chmod +x <script>
 │     └──> Track: GENERATION_PASSED, GENERATION_FAILED
 │
 ├──> execute_test_scripts()
 │     ├──> find scripts/*.sh
 │     ├──> is_manual_test() ──> grep "manual: true"
 │     ├──> Execute: <script> > <log>
 │     ├──> Validate: python3 -m json.tool <log>
 │     └──> Track: EXECUTION_PASSED, EXECUTION_FAILED, EXECUTION_SKIPPED
 │
 ├──> verify_execution_logs()
 │     ├──> find execution_logs/*.json
 │     ├──> verifier --title <title> --project <project> --environment <env>
 │     │              --test-case <yaml> --execution-log <log> --output <container>
 │     └──> Track: VERIFICATION_PASSED, VERIFICATION_FAILED
 │
 ├──> validate_container_yamls()
 │     ├──> find verification_results/*_container.yaml
 │     ├──> validate-yaml --schema <container_schema> <container>
 │     └──> Track: CONTAINER_VALIDATION_PASSED, CONTAINER_VALIDATION_FAILED
 │
 ├──> generate_documentation()
 │     ├──> Check: command -v test-plan-documentation-generator
 │     ├──> find verification_results/*_container.yaml
 │     ├──> TPDG: --input <container> --output <adoc> --format asciidoc
 │     ├──> TPDG: --input <container> --output <md> --format markdown
 │     └──> Track: DOCUMENTATION_PASSED, DOCUMENTATION_FAILED
 │
 └──> generate_summary_report()
       ├──> Aggregate all statistics
       ├──> Generate text report
       ├──> tee to console and file
       └──> Return: 0 (success) or 1 (failure)
```

## Stage Dependencies

```
Stage 1 (Validation)
    │
    ├─ Required for: Stage 2
    ├─ Can skip: No (required)
    └─ Depends on: Test case YAML files

Stage 2 (Generation)
    │
    ├─ Required for: Stage 3
    ├─ Can skip: Yes (--skip-generation)
    ├─ Depends on: Stage 1 (valid YAMLs)
    └─ Uses: test-executor binary

Stage 3 (Execution)
    │
    ├─ Required for: Stage 4
    ├─ Can skip: Yes (--skip-execution)
    ├─ Depends on: Stage 2 (generated scripts)
    └─ Produces: execution_logs/*.json

Stage 4 (Verification)
    │
    ├─ Required for: Stage 5, Stage 6
    ├─ Can skip: Yes (--skip-verification)
    ├─ Depends on: Stage 3 (execution logs)
    └─ Produces: verification_results/*_container.yaml

Stage 5 (Container Validation)
    │
    ├─ Required for: Ensuring TPDG compatibility
    ├─ Can skip: Yes (auto-skip if Stage 4 skipped)
    ├─ Depends on: Stage 4 (container YAMLs)
    └─ Uses: validate-yaml binary

Stage 6 (Documentation)
    │
    ├─ Required for: Final reports
    ├─ Can skip: Yes (--skip-documentation or no TPDG)
    ├─ Depends on: Stage 4 (container YAMLs)
    └─ Produces: reports/{asciidoc,markdown}/*
```

## Decision Flow

```
                        [Start]
                           │
                           ▼
                [Parse Command Line Args]
                           │
        ┌──────────────────┼──────────────────┐
        ▼                  ▼                  ▼
   --verbose=1      --include-manual=1    --skip-*=1
        │                  │                  │
        └──────────────────┴──────────────────┘
                           │
                           ▼
                [Verify Binaries Exist]
                           │
                  ┌────────┴────────┐
                  ▼                 ▼
              All Found         Missing
                  │                 │
                  │            [Exit 1]
                  ▼
         [Execute Stage 1]
                  │
          ┌───────┴───────┐
          ▼               ▼
      Success         Failure
          │               │
          │        [Track Failures]
          │               │
          └───────┬───────┘
                  ▼
         [Skip Generation?]
          ┌───────┴───────┐
          ▼               ▼
         Yes             No
          │               │
          │        [Execute Stage 2]
          │               │
          │       ┌───────┴───────┐
          │       ▼               ▼
          │   Success         Failure
          │       │               │
          │       │        [Track Failures]
          │       │               │
          └───────┴───────┬───────┘
                          ▼
                 [Skip Execution?]
                  ┌───────┴───────┐
                  ▼               ▼
                 Yes             No
                  │               │
                  │        [Execute Stage 3]
                  │               │
                  │         [Manual Test?]
                  │        ┌──────┴──────┐
                  │        ▼             ▼
                  │      Yes            No
                  │        │             │
                  │  [Include Manual?]  │
                  │   ┌────┴────┐       │
                  │   ▼         ▼       │
                  │  Yes       No       │
                  │   │         │       │
                  │ [Run]    [Skip]   [Run]
                  │   │         │       │
                  └───┴─────────┴───────┘
                          │
                     [Continue...]
                          │
                          ▼
                  [All Stages Done]
                          │
                          ▼
              [Generate Summary Report]
                          │
                  ┌───────┴───────┐
                  ▼               ▼
          Any Failures?          No
                Yes               │
                  │               │
             [Exit 1]        [Exit 0]
```

## Statistics Tracking Flow

```
For each test case:
    │
    ├─ Stage 1: Validation
    │   ├─ Success → VALIDATION_PASSED++
    │   └─ Failure → VALIDATION_FAILED++, append to failures.txt
    │
    ├─ Stage 2: Generation
    │   ├─ Success → GENERATION_PASSED++
    │   └─ Failure → GENERATION_FAILED++, append to failures.txt
    │
    ├─ Stage 3: Execution
    │   ├─ Automated Success → EXECUTION_PASSED++
    │   ├─ Automated Failure → EXECUTION_FAILED++, append to failures.txt
    │   └─ Manual (no --include-manual) → EXECUTION_SKIPPED++, append to manual_tests.txt
    │
    ├─ Stage 4: Verification
    │   ├─ Success → VERIFICATION_PASSED++
    │   └─ Failure → VERIFICATION_FAILED++, append to failures.txt
    │
    ├─ Stage 5: Container Validation
    │   ├─ Success → CONTAINER_VALIDATION_PASSED++
    │   └─ Failure → CONTAINER_VALIDATION_FAILED++, append to failures.txt
    │
    └─ Stage 6: Documentation
        ├─ Both formats success → DOCUMENTATION_PASSED++
        └─ Any format failure → DOCUMENTATION_FAILED++, append to failures.txt

Final Summary:
    total_failures = sum of all *_FAILED counters
    exit_code = (total_failures > 0) ? 1 : 0
```

## Command Line Options Impact

```
Option                   Impact
──────────────────────   ──────────────────────────────────
--verbose                Sets VERBOSE=1
                         Enables log_verbose() and log_debug()
                         Shows command outputs on failure

--include-manual         Sets INCLUDE_MANUAL=1
                         Manual tests executed instead of skipped
                         Increases EXECUTION_PASSED/FAILED instead of SKIPPED

--skip-generation        Sets SKIP_GENERATION=1
                         Stage 2 prints "SKIPPED" and returns 0
                         Uses existing scripts/ directory
                         GENERATION_PASSED/FAILED remain 0

--skip-execution         Sets SKIP_EXECUTION=1
                         Stage 3 prints "SKIPPED" and returns 0
                         Uses existing execution_logs/ directory
                         EXECUTION_PASSED/FAILED/SKIPPED remain 0

--skip-verification      Sets SKIP_VERIFICATION=1
                         Stage 4 prints "SKIPPED" and returns 0
                         Stage 5 auto-skipped
                         Uses existing verification_results/ directory
                         VERIFICATION_PASSED/FAILED remain 0
                         CONTAINER_VALIDATION_PASSED/FAILED remain 0

--skip-documentation     Sets SKIP_DOCUMENTATION=1
                         Stage 6 prints "SKIPPED" and returns 0
                         DOCUMENTATION_PASSED/FAILED remain 0

-h, --help              Shows usage information and exits 0
```

## Error Handling Flow

```
[Binary Missing]
    │
    └──> fail "binary not found"
         log_info "Build instructions"
         Exit 1

[Stage Failure]
    │
    ├──> Track failure (increment counter)
    ├──> Log to failure tracking file
    ├──> Show error (with verbose details if --verbose)
    └──> Continue to next stage (best effort)

[Missing Input Files]
    │
    └──> log_warning "No files found"
         log_info "Suggestion"
         Return 1 (stage fails, but continue)

[TPDG Not Available]
    │
    └──> log_warning "TPDG not found"
         log_info "Install instructions"
         Skip documentation stage
         Return 0 (not an error)

[Invalid JSON]
    │
    └──> log_warning "Invalid JSON"
         Track as execution failure
         Continue to next file

[Summary Generation]
    │
    └──> Always succeeds
         Aggregates all statistics
         Returns 1 if any failures detected
```

## File Naming Conventions

```
Input:
    test_cases/**/*.yaml
    Example: TC_SUCCESS_SIMPLE_001.yaml

Generated Scripts:
    scripts/<basename>.sh
    Example: TC_SUCCESS_SIMPLE_001.sh

Execution Logs:
    execution_logs/<basename>.json
    Example: TC_SUCCESS_SIMPLE_001.json

Container YAMLs:
    verification_results/<basename>_container.yaml
    Example: TC_SUCCESS_SIMPLE_001_container.yaml

Documentation:
    reports/asciidoc/<basename>.adoc
    reports/markdown/<basename>.md
    Example: TC_SUCCESS_SIMPLE_001.adoc
             TC_SUCCESS_SIMPLE_001.md
```

## See Also

- [ACCEPTANCE_SUITE.md](ACCEPTANCE_SUITE.md) - Complete documentation
- [ORCHESTRATOR_IMPLEMENTATION.md](ORCHESTRATOR_IMPLEMENTATION.md) - Implementation details
- [QUICKSTART.md](QUICKSTART.md) - Quick start guide
- [README.md](README.md) - Overview
