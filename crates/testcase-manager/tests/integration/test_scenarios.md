# Integration Test Scenarios

This document describes the test scenarios covered by the integration tests.

## Test Coverage Matrix

| Scenario | Basic Workflow | Complete Workflow | Description |
|----------|----------------|-------------------|-------------|
| Metadata Entry | ✓ | ✓ | Validates user input for requirement, item, tc, id, description |
| Metadata Validation | ✓ | ✓ | Schema validation of metadata fields |
| Metadata Git Commit | ✓ | ✓ | Commits metadata to git repository |
| General Initial Conditions | - | ✓ | Prompts and validates general initial conditions |
| General IC Git Commit | - | ✓ | Commits general IC to git |
| Initial Conditions | ✓ | ✓ | Device-specific initial conditions with iterative input |
| IC Validation | ✓ | ✓ | Schema validation of initial conditions |
| IC Git Commit | ✓ | ✓ | Commits initial conditions to git |
| Test Sequence Creation | ✓ | ✓ | Creates test sequences with name and description |
| Sequence IC | - | - | Sequence-specific initial conditions (optional) |
| Sequence Validation | ✓ | ✓ | Validates test sequence structure |
| Sequence Git Commit | - | ✓ | Commits each sequence to git |
| Step Collection | - | ✓ | Iterative step collection within sequences |
| Step Validation | - | ✓ | Schema validation of each step |
| Step Git Commit | - | ✓ | Commits each step to git |
| Manual Step Flag | - | ✓ | Tests manual vs automated step differentiation |
| Expected Fields | - | ✓ | Tests expected result/output/success fields |
| Final File Save | ✓ | ✓ | Saves complete YAML file |
| Final Git Commit | ✓ | ✓ | Final commit of complete test case |
| YAML Structure | ✓ | ✓ | Validates output YAML structure |
| Git History | ✓ | ✓ | Verifies commit history and messages |
| Schema Validation | ✓ | ✓ | Validates against JSON schema |
| Cleanup | ✓ | ✓ | Removes test artifacts |

## Scenario Details

### Basic Workflow Test (`e2e_basic_workflow.exp`)

**Purpose**: Quick smoke test of core functionality

**Flow**:
1. Create metadata (minimal fields)
2. Skip general initial conditions
3. Add minimal initial conditions (1 device, 1 condition)
4. Create 1 test sequence (no steps)
5. Skip step collection
6. Save and commit

**Expected Duration**: ~10 seconds (timeout: 60 seconds)

**Technical Notes**:
- Timeout increased to 60 seconds for system compatibility
- Editor and fuzzy search prompts automatically skipped

**Use Cases**:
- Quick validation after code changes
- Pre-commit hook testing
- Continuous integration smoke test

### Complete Workflow Test (`e2e_complete_workflow.exp`)

**Purpose**: Comprehensive end-to-end validation

**Flow**:
1. Create metadata (all fields)
2. Add general initial conditions (via editor)
3. Add initial conditions (device: eUICC, 2 conditions)
4. Create test sequence "Basic Profile Installation"
5. Add 2 steps to sequence:
   - Step 1: "Initialize connection to eUICC"
     - Command: ssh init
     - Expected: success=true, SW=9000, Connection established
   - Step 2: "Download profile package"
     - Command: download profile.pkg
     - Expected: Package downloaded, 100% complete
6. Commit each step individually
7. Save and commit final file

**Expected Duration**: ~30 seconds (timeout: 60 seconds)

**Technical Notes**:
- Timeout increased to 60 seconds for system compatibility
- Editor and fuzzy search prompts automatically skipped by sending "n"
- Comprehensive validation of all workflow steps

**Use Cases**:
- Full regression testing
- Release validation
- Comprehensive workflow verification

## Test Data

### Metadata Test Values

```yaml
requirement: SGP.22_v3.0  # (Complete) / TEST_REQ (Basic)
item: 4                    # (Complete) / 1 (Basic)
tc: 2                      # (Complete) / 1 (Basic)
id: test_e2e_001           # (Complete) / basic_test_001 (Basic)
description: E2E integration test case
```

### Initial Conditions Test Values

```yaml
initial_conditions:
  eUICC:
    - Device in test mode          # (Complete)
    - Profile storage available    # (Complete)
    - Test condition               # (Basic)
```

### Test Sequence Test Values

```yaml
test_sequences:
  - id: 1
    name: Basic Profile Installation  # (Complete) / Test Sequence (Basic)
    description: Test basic profile installation flow
    initial_conditions: []
    steps:
      - step: 1
        description: Initialize connection to eUICC
        command: ssh init
        expected:
          success: true
          result: SW=9000
          output: Connection established
      - step: 2
        description: Download profile package
        command: download profile.pkg
        expected:
          result: Package downloaded
          output: 100% complete
```

## Validation Checks

### File Structure Validation

Each test validates:
- File exists at expected location
- YAML is valid and parseable
- Required top-level fields present:
  - `requirement`
  - `item`
  - `tc`
  - `id`
  - `description`
  - `general_initial_conditions` (Complete only)
  - `initial_conditions`
  - `test_sequences`
- Test sequences contain expected data
- Steps contain expected fields (Complete only)

### Git Validation

Each test validates:
- Git repository initialized
- Minimum number of commits created:
  - Basic: 2+ commits
  - Complete: 5+ commits
- Commit messages contain expected keywords:
  - "Add test case metadata"
  - "Add general initial conditions" (Complete only)
  - "Add initial conditions"
  - "Add test sequence"
  - "Add step" (Complete only)
  - "Complete test case"
- Working directory is clean after completion

### Schema Validation

Each test validates:
- Output file passes JSON schema validation
- All required fields are present
- Data types are correct
- Structure matches expected format

## Error Scenarios

The tests check for proper error handling in these scenarios:

1. **Missing Binary**: Test fails gracefully if binary not found
2. **Timeout**: Test fails if prompts don't appear within timeout
3. **Invalid Input**: Test continues with validation retry prompts
4. **File Creation Failure**: Test detects missing output files
5. **Git Failure**: Test detects if commits weren't created

## Extending Tests

To add new test scenarios:

1. **Create new .exp file**:
   ```bash
   cp tests/integration/e2e_basic_workflow.exp tests/integration/e2e_new_scenario.exp
   ```

2. **Modify test flow**:
   - Update test directory name
   - Modify user input sequences
   - Add specific validations

3. **Add to test runner**:
   ```bash
   # Add to run_all_tests.sh
   run_test "New Scenario Test" "$SCRIPT_DIR/e2e_new_scenario.exp"
   ```

4. **Update documentation**:
   - Add row to coverage matrix
   - Document test data
   - Describe validation checks

## Performance Considerations

- Tests run in isolated temporary directories
- Each test creates a fresh git repository
- Test artifacts are cleaned up automatically
- Tests can run in parallel if needed (use different directories)

## Known Limitations

1. **Editor Interaction**: Tests that require editor interaction (e.g., editing descriptions) use default/skip options
2. **Fuzzy Search**: Tests don't use fuzzy search features (would require skim interaction)
3. **Error Recovery**: Tests don't validate the recovery file mechanism
4. **Network Operations**: Tests don't make actual network calls
5. **Device Interaction**: Tests don't interact with real eUICC devices

## Future Enhancements

- [ ] Add test for recovery file mechanism
- [ ] Add test for invalid input handling
- [ ] Add test for multi-sequence workflows
- [ ] Add test for editing existing test cases
- [ ] Add performance benchmarking
- [ ] Add tests for import/export functionality
- [ ] Add tests for validation command
- [ ] Add tests for list/search commands
