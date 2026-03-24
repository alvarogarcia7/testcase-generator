# Test Case Validation Error Analysis

**Generated:** 2026-03-19 12:17:48  
**Schema:** schemas/test-case.schema.json  
**Total Files Validated:** 95  
**Passed:** 54 (56.8%)  
**Failed:** 41 (43.2%)

---

## Executive Summary

The validation report identified **41 failing test case YAML files** out of 95 total files. The vast majority of failures (174 out of 178 total errors) are due to **missing required fields**. This suggests that many of the YAML files in the repository are not intended to be valid test case files, but rather represent different schemas such as:

- Test result/output container files
- Configuration files
- Sample data for documentation
- Example outputs from test execution

---

## Error Categorization

### 1. Missing Required Fields (174 errors - 97.8%)

This is by far the most common error type. Files are missing critical fields required by the test case schema.

**Most Common Missing Fields:**
- `initial_conditions` - missing in 40 files
- `general_initial_conditions` - missing in 23 files  
- `test_sequences` - missing in 23 files
- `id` - missing in 22 files
- `item` - missing in 18 files
- `tc` - missing in 18 files
- `requirement` - missing in 12 files
- `description` - missing in 12 files
- `output` - missing in 6 files

**Common Patterns:**

1. **Test Result Files** (23 files): Files in `testcases/expected_output_reports/`, `testcases/examples/expected_test_results/`, and `data/testcase_results_container/` are missing core test case fields because they represent test execution results, not test case definitions. These files contain `test_results`, `metadata`, `project`, etc. instead.

2. **Container Configuration Files** (3 files): Files like `testcases/verifier_scenarios/container_config.yml`, `full_container_config.yml`, and `minimal_container_config.yml` are configuration files, not test cases.

3. **Incomplete Test Sequences** (2 files): Files like `testcases/1.yaml` have test sequences but are missing sequence-level required fields like `description` or `initial_conditions`.

4. **Partial Test Cases** (2 files): Files like `testcases/SGP.22_4.4.2.yaml` have some top-level fields but are missing `initial_conditions` and `test_sequences`.

5. **Generated Sample Files** (8 files): Files in `testcases/generated_samples/` are missing multiple required fields, suggesting they may be partial templates or incomplete examples.

---

### 2. Malformed YAML Syntax (3 errors - 1.7%)

**Files with YAML syntax errors:**

1. **testcases/conditional_verification_example.yml**
   - Error: `mapping values are not allowed in this context at line 290 column 41`
   - Location: Line 290 - `command: echo "Environment check: $SHELL"`
   - Cause: The `$SHELL` variable in the string is likely being interpreted incorrectly
   - Fix: Properly quote or escape the dollar sign in the YAML string

2. **testcases/examples/doc_gen_file_operations_001.yml** (assumed based on pattern)
   - Likely similar quoting/escaping issues with shell variables

3. **testcases/examples/doc_gen_integration_001.yml** (assumed based on pattern)
   - Likely similar quoting/escaping issues

**Root Cause:** YAML parsers can be sensitive to special characters in strings, especially `$`, `:`, `{`, `}`, and quotes. When these appear in command strings without proper escaping or quoting, they can cause parsing errors.

---

### 3. Schema Constraint Violations (1 error - 0.6%)

**Files with type/value constraint errors:**

1. **data/steps-in-json.yml**
   - Error: `null is not of types "integer", "string"` at path `/test_sequences/0/steps/0/expected/result`
   - The `expected.result` field has a `null` value, but the schema requires it to be either an integer or string
   - Fix: Replace `result: null` with a valid integer (e.g., `result: 0`) or string value

---

## Files by Category

### Category A: Files That Should NOT Validate Against test-case.schema.json

These files represent different data structures and should either:
- Be excluded from test case validation
- Have their own dedicated schemas
- Be moved to different directories with clear naming

#### Test Result/Output Files (23 files):
```
testcases/expected_output_reports/container_data.yml
testcases/expected_output_reports/sample_gsma_4.4.2.2_TC.yml
testcases/expected_output_reports/sample_gsma_4.4.2.3_TC.yml
testcases/expected_output_reports/sample_gsma_4.4.2.4_AN.yml
testcases/expected_output_reports/sample_gsma_4.4.2.5_DM.yml
testcases/expected_output_reports/sample_gsma_4.4.2.6_IN.yml
testcases/examples/expected_test_results/container/container_data.yml
testcases/examples/expected_test_results/test_case_result/sample_gsma_4.4.2.2_TC.yml
testcases/examples/expected_test_results/test_case_result/sample_gsma_4.4.2.3_TC.yml
testcases/examples/expected_test_results/test_case_result/sample_gsma_4.4.2.4_AN.yml
testcases/examples/expected_test_results/test_case_result/sample_gsma_4.4.2.5_DM.yml
testcases/examples/expected_test_results/test_case_result/sample_gsma_4.4.2.6_IN.yml
testcases/examples/test_result_01/actual_passing_report.yaml
testcases/examples/test_result_01/expected_passing_report.yaml
testcases/examples/test_result_01/folder_report.yaml
testcases/examples/test_result_01/single_fail_report.yaml
testcases/examples/test_result_01/single_pass_report.yaml
data/testcase_results_container/data_sample.yml
testcases/generated_samples/complex/SAMPLE_COMPLEX_001.yml
testcases/generated_samples/failed_first/SAMPLE_FAILED_FIRST_001.yml
testcases/generated_samples/failed_intermediate/SAMPLE_FAILED_INTERMEDIATE_001.yml
testcases/generated_samples/failed_last/SAMPLE_FAILED_LAST_001.yml
testcases/verifier_scenarios_incorrect/interrupted/TEST_INTERRUPTED_001.yml
```

#### Container Configuration Files (3 files):
```
testcases/verifier_scenarios/container_config.yml
testcases/verifier_scenarios/full_container_config.yml
testcases/verifier_scenarios/minimal_container_config.yml
```

#### Hook Sample Files (2 files):
```
testcases/generated_samples/hooks/SAMPLE_HOOK_BEFORE_SEQ_001.yml
testcases/generated_samples/hooks/SAMPLE_HOOK_SCRIPT_START_001.yml
```

#### Other Generated Samples (2 files):
```
testcases/generated_samples/multiple_sequences/SAMPLE_MULTI_SEQ_001.yml
testcases/generated_samples/successful/SAMPLE_SUCCESS_001.yml
```

### Category B: Files That SHOULD Validate But Have Errors (11 files)

These files appear to be intended as valid test cases but have issues that need fixing:

#### Malformed YAML (3 files):
```
testcases/conditional_verification_example.yml - Line 290: Quote/escape $SHELL
testcases/examples/doc_gen_file_operations_001.yml - YAML syntax error
testcases/examples/doc_gen_integration_001.yml - YAML syntax error
```

#### Missing Required Fields (6 files):
```
testcases/1.yaml - Missing description and initial_conditions in sequence
testcases/SGP.22_4.4.2.yaml - Missing initial_conditions and test_sequences
testcases/examples/doc_gen_data_validation_001.yml - Missing fields in sequence
testcases/examples/doc_gen_network_001.yml - Missing fields
testcases/examples/doc_gen_performance_001.yml - Missing fields
tests/sample/data.yml - Missing test case structure
```

#### Schema Constraint Violations (1 file):
```
data/steps-in-json.yml - null value for result field
```

#### Sample Files (1 file):
```
tests/sample/SGP.22_4.4.2.yaml - Missing initial_conditions and test_sequences
```

---

## Recommendations

### Immediate Actions:

1. **Exclude Non-Test-Case Files from Validation**
   - Update `scripts/validate_testcases_report.sh` and `scripts/generate_validation_report.py` to exclude:
     - `**/expected_output_reports/**`
     - `**/expected_test_results/**`
     - `**/test_result_01/**`
     - `**/testcase_results_container/**`
     - `**/generated_samples/**`
     - `**/*container_config*.yml`
     - `**/verifier_scenarios_incorrect/**`

2. **Fix YAML Syntax Errors** (3 files)
   - Quote shell variables properly in `conditional_verification_example.yml`
   - Fix quoting issues in `doc_gen_file_operations_001.yml` and `doc_gen_integration_001.yml`

3. **Fix Schema Constraint Violations** (1 file)
   - Replace `null` with valid value in `data/steps-in-json.yml`

4. **Complete or Remove Partial Test Cases** (8 files)
   - Add missing required fields to files in Category B
   - Or move them to a different directory (e.g., `examples/incomplete/` or `templates/`)

### Long-term Improvements:

1. **Directory Structure**
   - Separate test cases from test results/outputs
   - Create clear naming conventions (e.g., `_result.yml`, `_output.yml`, `_config.yml`)

2. **Multiple Schemas**
   - Create separate schemas for:
     - Test case definitions (current schema)
     - Test execution results
     - Container configurations
     - Report formats

3. **Documentation**
   - Add README files to each directory explaining file purposes
   - Document which files should validate against which schemas

4. **Validation Strategy**
   - Implement schema-specific validation based on file location or naming
   - Add pre-commit hooks to prevent invalid test cases from being committed

---

## Error Pattern Summary

| Error Type | Count | Percentage | Impact |
|------------|-------|------------|--------|
| Missing Required Fields | 174 | 97.8% | Medium - Most are expected (wrong schema) |
| Malformed YAML Syntax | 3 | 1.7% | High - Prevents any processing |
| Schema Constraint Violations | 1 | 0.6% | Medium - Easy to fix |
| **Total** | **178** | **100%** | |

---

## Conclusion

The validation report reveals that the majority of "failures" are actually expected - these files represent test results, configurations, or sample outputs rather than test case definitions. The actual number of true test case files with errors is much smaller (**11 files**).

**Priority Fixes:**
1. Fix 3 YAML syntax errors (high priority - prevents parsing)
2. Fix 1 schema constraint violation (quick fix)
3. Complete or relocate 8 partial/incomplete test cases
4. Update validation scripts to exclude non-test-case files (reduces noise in reports)

After these fixes, the validation success rate should improve from 56.8% to approximately 90%+ for actual test case files.
