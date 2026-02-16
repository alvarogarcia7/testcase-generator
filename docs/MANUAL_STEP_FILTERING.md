# Manual Step Filtering

This document explains the manual step detection and filtering capabilities in the test case management system, including CLI commands, orchestrator options, and practical use cases.

## Table of Contents

- [Overview](#overview)
- [Manual Step Detection Logic](#manual-step-detection-logic)
- [CLI Commands for Listing and Filtering](#cli-commands-for-listing-and-filtering)
- [Orchestrator Filtering Options](#orchestrator-filtering-options)
- [Use Cases](#use-cases)
- [Examples with Test Cases](#examples-with-test-cases)

## Overview

The test case management system supports mixed test scenarios containing both automated and manual steps. Manual steps require human intervention (e.g., physical device interaction, visual verification, GUI operations), while automated steps can be executed programmatically.

The filtering system allows you to:
- List all test cases with manual step indicators
- Filter test cases to show only those with manual steps
- Filter test cases to show only those with automated steps
- Generate statistics about manual vs. automated content
- Execute only automated test cases in CI/CD pipelines

## Manual Step Detection Logic

### Step-Level Detection

A test step is considered **manual** when the `manual` field is explicitly set to `true`:

```yaml
steps:
  - step: 1
    manual: true  # This is a manual step
    description: "Manually verify LED indicator is blinking"
    command: "Observe device LED status"
    expected:
      success: true
      result: 0
      output: "LED is blinking green"
```

A step is considered **automated** (non-manual) when:
- The `manual` field is absent (defaults to automated)
- The `manual` field is explicitly set to `false`

```yaml
steps:
  - step: 2
    # No manual field - automated by default
    description: "Check network connectivity"
    command: "ping -c 3 192.168.1.1"
    expected:
      success: true
      result: 0
      output: "3 packets transmitted, 3 received"
```

### Test Case-Level Detection

The system provides three methods on `TestCase` to analyze manual step content:

**`has_manual_steps()`** - Returns `true` if any step in any sequence has `manual: true`
```rust
pub fn has_manual_steps(&self) -> bool {
    self.test_sequences
        .iter()
        .any(|sequence| sequence.steps.iter().any(|step| step.manual == Some(true)))
}
```

**`has_automated_steps()`** - Returns `true` if any step in any sequence is not manual
```rust
pub fn has_automated_steps(&self) -> bool {
    self.test_sequences
        .iter()
        .any(|sequence| sequence.steps.iter().any(|step| step.manual != Some(true)))
}
```

**`get_manual_step_count()`** - Returns the total count of manual steps across all sequences
```rust
pub fn get_manual_step_count(&self) -> usize {
    self.test_sequences
        .iter()
        .flat_map(|sequence| &sequence.steps)
        .filter(|step| step.manual == Some(true))
        .count()
}
```

### Filter Criteria

Three filter types are available via the `TestCaseFilter` enum:

- **`TestCaseFilter::All`** - Include all test cases regardless of step types
- **`TestCaseFilter::ManualOnly`** - Include only test cases that have at least one manual step
- **`TestCaseFilter::AutomatedOnly`** - Include only test cases that have at least one automated step

**Note:** Test cases with both manual and automated steps will appear in both `ManualOnly` and `AutomatedOnly` filters.

## CLI Commands for Listing and Filtering

### test-executor list

The `test-executor` command provides a `list` subcommand for discovering and filtering test cases.

#### Basic Usage

```bash
# List all test cases in the default testcases directory
test-executor list

# List test cases from a specific directory
test-executor list /path/to/testcases
```

#### Filtering Options

**`--manual-only`** - Show only test cases containing manual steps
```bash
test-executor list --manual-only
```

**`--automated-only`** - Show only test cases containing automated steps
```bash
test-executor list --automated-only
```

**`--show-stats`** - Display statistics about test case composition
```bash
test-executor list --show-stats
```

#### Output Format

The list command displays test cases with manual step indicators:

```
Test Cases:

  TC_MANUAL_SSH_001 [M:3] - Test case for SSH device access verification with manual steps
  TC_MANUAL_MIXED_010 [M:2] - Test case with mixed automated and manual workflow for complete system validation
  TC_AUTO_001 - Fully automated network connectivity test
```

The `[M:n]` indicator shows the count of manual steps in the test case.

#### Statistics Output

When using `--show-stats`, additional information is displayed:

```
Statistics:
  Total test cases: 10
  Test cases with manual steps: 5
  Test cases with automated steps: 8
  Total manual steps: 15
```

#### Examples

```bash
# List only test cases with manual steps from examples directory
test-executor list testcases/examples/manual_steps --manual-only

# List automated test cases with statistics
test-executor list --automated-only --show-stats

# List all test cases in a specific directory with detailed stats
test-executor list testcases/ci --show-stats
```

## Orchestrator Filtering Options

### test-orchestrator run

The orchestrator supports filtering test cases before execution using the same filter logic.

#### Filter Flags

**`--filter-manual`** - Execute only test cases with manual steps
```bash
test-orchestrator run --filter-manual [TEST_CASE_IDS]
```

**`--filter-automated`** - Execute only test cases with automated steps
```bash
test-orchestrator run --filter-automated [TEST_CASE_IDS]
```

These flags are mutually exclusive and will conflict if used together.

#### Programmatic Filtering

When using the orchestrator API, you can filter test cases before execution:

```rust
use testcase_manager::{TestCaseFilter, TestCaseFilterer, TestCaseStorage};

let storage = TestCaseStorage::new("testcases")?;
let all_cases = storage.load_all_test_cases()?;

let filterer = TestCaseFilterer::new();

// Filter for automated-only test cases
let automated_cases = filterer.filter_test_cases(
    all_cases.clone(),
    TestCaseFilter::AutomatedOnly
);

// Execute only automated cases
orchestrator.execute_tests(automated_cases, config, verbose)?;
```

## Use Cases

### 1. CI/CD Automated-Only Runs

**Scenario:** Run only automated tests in a CI/CD pipeline, excluding tests requiring manual intervention.

**Solution:** Use the `--filter-automated` flag or `AutomatedOnly` filter to ensure only fully automated or partially automated tests are executed.

```bash
# In your CI/CD pipeline script
test-orchestrator run-all --filter-automated --workers 8 --save --report
```

**Benefits:**
- No pipeline failures due to manual steps
- Faster execution without manual intervention
- Clear separation of automated and manual testing phases

### 2. Manual Testing Sessions

**Scenario:** During manual QA sessions, testers need to identify and execute all tests requiring human interaction.

**Solution:** Use `--manual-only` to list and execute only tests with manual steps.

```bash
# List all test cases requiring manual intervention
test-executor list --manual-only --show-stats

# Execute specific manual test cases
test-orchestrator run TC_MANUAL_SSH_001 TC_MANUAL_DEVICE_004 --workers 1
```

**Benefits:**
- Efficient test session planning
- Clear visibility of manual workload
- Focused manual testing efforts

### 3. Pre-Release Validation

**Scenario:** Before release, execute a complete test suite including both automated and manual tests in phases.

**Solution:** Run automated tests first, then manual tests separately.

```bash
# Phase 1: Automated tests (can run overnight)
test-orchestrator run-all --filter-automated --workers 16 --save --report

# Phase 2: Manual tests (during business hours with QA team)
test-orchestrator run-all --filter-manual --workers 2 --save --report
```

### 4. Test Coverage Analysis

**Scenario:** Analyze the ratio of automated vs. manual test coverage across the test suite.

**Solution:** Use the `--show-stats` flag to gather metrics.

```bash
# Analyze entire suite
test-executor list --show-stats

# Analyze specific module
test-executor list testcases/modules/networking --show-stats
```

**Output:**
```
Statistics:
  Total test cases: 45
  Test cases with manual steps: 12
  Test cases with automated steps: 40
  Total manual steps: 28
```

### 5. Developer Quick Validation

**Scenario:** Developers want to quickly validate their changes without waiting for manual steps.

**Solution:** Run automated-only tests during development.

```bash
# Quick validation of changes
test-orchestrator run --filter-automated --fuzzy --workers 4
```

## Examples with Test Cases

### Example Test Cases from `testcases/examples/manual_steps/`

#### TC_MANUAL_SSH_001 - SSH Device Access (Mixed Steps)

**File:** `TC_MANUAL_SSH_001.yaml`

**Manual Steps:** 3 out of 5 total steps

**Step Breakdown:**
- Step 1: **Automated** - Check device network connectivity with ping
- Step 2: **Manual** - SSH into device and verify login
- Step 3: **Manual** - Execute uptime command on remote device
- Step 4: **Automated** - Check SSH service status locally
- Step 5: **Manual** - Log out from SSH session

**Filtering Behavior:**
- Appears in `--manual-only` filter (has manual steps)
- Appears in `--automated-only` filter (has automated steps)
- Manual step count: 3

**Usage Example:**
```bash
# List this test case
test-executor list testcases/examples/manual_steps | grep TC_MANUAL_SSH_001
# Output: TC_MANUAL_SSH_001 [M:3] - Test case for SSH device access verification with manual steps

# Execute only automated steps (manual steps would be skipped in execution)
test-orchestrator run TC_MANUAL_SSH_001 --filter-automated
```

#### TC_MANUAL_MIXED_010 - End-to-End System Validation (Mixed Steps)

**File:** `TC_MANUAL_MIXED_010.yaml`

**Manual Steps:** 2 out of 5 total steps

**Step Breakdown:**
- Step 1: **Automated** - System health check
- Step 2: **Manual** - Start application service from GUI
- Step 3: **Automated** - Verify service startup
- Step 4: **Manual** - Test application functionality through UI
- Step 5: **Automated** - Log analysis

**Filtering Behavior:**
- Appears in `--manual-only` filter (has manual steps)
- Appears in `--automated-only` filter (has automated steps)
- Manual step count: 2

**Usage Example:**
```bash
# List this test case with statistics
test-executor list testcases/examples/manual_steps --show-stats

# This test case represents a realistic workflow mixing automation and manual verification
# In CI/CD: Only steps 1, 3, and 5 would execute
# In manual testing: All 5 steps would be performed by the tester
```

#### Other Example Test Cases

The `testcases/examples/manual_steps/` directory contains additional examples:

- **TC_MANUAL_HARDWARE_002** - Physical hardware interaction tests
- **TC_MANUAL_UI_003** - User interface verification tests
- **TC_MANUAL_DEVICE_004** - Device-specific manual procedures
- **TC_MANUAL_NETWORK_005** - Network configuration with manual verification
- **TC_MANUAL_DATABASE_006** - Database operations requiring manual checks
- **TC_MANUAL_API_007** - API testing with manual validation
- **TC_MANUAL_SECURITY_008** - Security testing with manual verification
- **TC_MANUAL_BACKUP_009** - Backup and recovery procedures

### Filtering All Examples

```bash
# List all manual step examples
test-executor list testcases/examples/manual_steps --manual-only --show-stats

# Output:
# Test Cases:
#
#   TC_MANUAL_API_007 [M:2] - API testing with manual validation
#   TC_MANUAL_BACKUP_009 [M:3] - Backup and recovery procedures
#   TC_MANUAL_DATABASE_006 [M:2] - Database operations requiring manual checks
#   TC_MANUAL_DEVICE_004 [M:4] - Device-specific manual procedures
#   TC_MANUAL_HARDWARE_002 [M:3] - Physical hardware interaction tests
#   TC_MANUAL_MIXED_010 [M:2] - Test case with mixed automated and manual workflow
#   TC_MANUAL_NETWORK_005 [M:2] - Network configuration with manual verification
#   TC_MANUAL_SECURITY_008 [M:3] - Security testing with manual verification
#   TC_MANUAL_SSH_001 [M:3] - Test case for SSH device access verification
#   TC_MANUAL_UI_003 [M:2] - User interface verification tests
#
# Statistics:
#   Total test cases: 10
#   Test cases with manual steps: 10
#   Test cases with automated steps: 10
#   Total manual steps: 26
```

## Best Practices

1. **Clear Step Marking:** Always explicitly mark manual steps with `manual: true` for clarity
2. **Descriptive Manual Steps:** Provide detailed descriptions for manual steps to guide testers
3. **Verification Instructions:** Include clear expected outcomes for manual verification
4. **CI/CD Integration:** Use `--filter-automated` in automated pipelines
5. **Test Organization:** Consider organizing test cases by automation level in directory structure
6. **Coverage Tracking:** Regularly review statistics to monitor automation progress
7. **Mixed Workflows:** Design test cases with logical flow between manual and automated steps

## Related Documentation

- [MANUAL_STEPS_HANDLING.md](MANUAL_STEPS_HANDLING.md) - Detailed manual steps implementation
- [TEST_VERIFY_USAGE.md](TEST_VERIFY_USAGE.md) - Test verification workflows
- [GITLAB_CI_EXAMPLES.md](GITLAB_CI_EXAMPLES.md) - CI/CD integration examples

## Summary

The manual step filtering system provides flexible test case organization and execution control. By detecting manual steps at the step and test case level, the system enables:

- Efficient test discovery and planning
- Automated CI/CD execution of fully automated tests
- Clear separation of manual and automated testing workflows
- Comprehensive statistics and reporting

Use the `test-executor list` command for discovery and `test-orchestrator run` with filter flags for controlled execution based on your testing needs.
