# Schema Audit Report

**Generated:** 2024
**Purpose:** Comprehensive audit of all schema files in the `schemas/` directory, documenting purpose, duplication status, and versioning compliance.

## Executive Summary

### Schema Inventory
- **Total Schema Files:** 24
- **Versioned (Envelope-compliant):** 7
- **Legacy (Non-envelope):** 17
- **Unique Schemas:** 18
- **Duplicate/Superseded Schemas:** 6

### Key Findings

1. **Envelope System**: The project has successfully migrated to a versioned envelope system (`tcms-envelope.schema.json`) with 7 production-ready v1 schemas in `schemas/tcms/*.schema.v1.json`.

2. **Duplicates Identified**: 
   - `verification_schema.json` and `verification-schema.json` are duplicates of `test-verification.schema.v1.json`
   - `test_results/container_schema.json` and `testcase_results_container/schema.json` are potential duplicates of `test-results-container.schema.v1.json`
   - `container/schema.json` is a minimal legacy schema superseded by container-config schemas

3. **Migration Status**: 4 root-level schemas (`test-case.schema.json`, `container_config.schema.json`, `execution-log.schema.json`, `verification-output.schema.json`, `verification-result.schema.json`) have optional envelope support and appear to be transitional versions.

4. **Verification Methods**: 7 specialized verification method schemas exist for different validation approaches (test, analysis, demonstration, inspection, Common Criteria, DO-178C high-assurance, and result reporting).

## Schema Categories

### 1. Envelope Meta-Schema (Core)

| File | Purpose | Status |
|------|---------|--------|
| `tcms-envelope.schema.json` | Defines envelope meta-schema requiring `type` and `schema` fields | **Keep** - Essential for versioned envelope system |

**Description:** Core meta-schema that all versioned schemas reference. Defines valid document types: `test_case`, `test_execution`, `test_verification`, `test_result`, `container_config`, `test_results_container`.

---

### 2. Versioned Schemas (Current Standard - Envelope-Compliant)

All files in `schemas/tcms/*.schema.v1.json` follow the envelope pattern with required `type` and `schema` fields. These are the **production-ready** schemas.

| File | Document Type | Purpose | Key Features |
|------|---------------|---------|--------------|
| `test-case.schema.v1.json` | `test_case` | Test case definitions | Prerequisites, hooks, BDD initial conditions, test sequences, hydration_vars, capture_vars, verification logic |
| `test-execution.schema.v1.json` | `test_execution` | Execution log entries | Individual test step execution records with command, exit_code, output, verification flags, hook_type |
| `test-verification.schema.v1.json` | `test_verification` | Test verification results | Complete verification results with Pass/Fail/NotExecuted steps, optional requirement/item/tc tracking |
| `test-result.schema.v1.json` | `test_result` | Test verification output | Similar to test-verification, includes test_case_id, sequences, step results, counts, overall_pass |
| `test-results-container.schema.v1.json` | `test_results_container` | Results container | Container for multiple test results with metadata (title, project, test_date, execution metrics) |
| `container-config.schema.v1.json` | `container_config` | Container configuration | Configuration for wrapping results with metadata (title, project, environment, platform, executor) |

**Status:** ✅ **Keep all** - These are the current production standards.

**JSON Schema Version:** draft-07

**Envelope Structure Example:**
```json
{
  "type": "test_case",
  "schema": "tcms/test-case.schema.v1.json",
  "id": "TC-001",
  "description": "Example test case",
  ...
}
```

---

### 3. Duplicate Schemas (Confirmed)

#### 3.1 Verification Schema Duplicates

| File | Type | Issue | Recommendation |
|------|------|-------|----------------|
| `tcms/schemas/verification_schema.json` | Legacy | Duplicate of `test-verification.schema.v1.json` without envelope | **Deprecate** |
| `tcms/schemas/verification-schema.json` | Legacy | Simplified duplicate of `test-verification.schema.v1.json` without envelope | **Deprecate** |

**Analysis:**
- Both are legacy versions of the current `test-verification.schema.v1.json`
- `verification_schema.json` has optional `requirement`/`item`/`tc` at top level only
- `verification-schema.json` is even more simplified (no requirement tracking, simpler Fail structure)
- Neither supports the envelope pattern
- Current v1 schema supersedes both

**Migration Path:** Update any code/tooling referencing these files to use `tcms/test-verification.schema.v1.json` with envelope support.

---

#### 3.2 Container Schema Potential Duplicates

| File | Type | Issue | Recommendation |
|------|------|-------|----------------|
| `tcms/test_results/container_schema.json` | Legacy | Minimal container without envelope | **Consider deprecating** |
| `tcms/testcase_results_container/schema.json` | Legacy | More detailed container without envelope | **Consider deprecating** |
| `tcms/container/schema.json` | Legacy | Minimal 3-field schema (date, product, description) | **Deprecate** |

**Analysis:**

**`test_results/container_schema.json`:**
- Basic container with `title`, `project`, `test_date`, `test_results`, `metadata`
- `test_results` items are loosely typed as generic `object`
- `metadata` requires `environment`, `platform`, `executor` but not execution metrics
- No envelope support

**`testcase_results_container/schema.json`:**
- More sophisticated than `test_results/container_schema.json`
- Includes complex definitions: `TestStepExecution`, `expectedOnlyPair`, `passFailResult`
- Supports `requirement`/`item`/`tc` tracking at test result level
- More detailed step result structure with expected/actual pairs
- No envelope support

**`container/schema.json`:**
- Only 3 fields: `date`, `product`, `description`
- Uses JSON Schema draft-04
- Appears to be an early/incomplete container format
- Does not match current container concepts

**Current Standard:** `test-results-container.schema.v1.json` supersedes all three:
- Full envelope support
- Comprehensive metadata with execution metrics
- Detailed step results with Pass/Fail/NotExecuted variants
- Optional requirement/item/tc tracking at multiple levels

**Migration Path:** 
1. Update tooling to use `tcms/test-results-container.schema.v1.json`
2. Add envelope fields (`type`, `schema`) to existing data
3. Ensure metadata includes required execution metrics

---

### 4. Transitional Schemas (Legacy with Optional Envelope Support)

These root-level schemas have **optional** `type` and `schema` fields, suggesting they were created during the migration to the envelope system.

| File | Versioned Equivalent | JSON Schema | Status |
|------|---------------------|-------------|--------|
| `test-case.schema.json` | `tcms/test-case.schema.v1.json` | draft-04 | Transitional |
| `container_config.schema.json` | `tcms/container-config.schema.v1.json` | draft-07 | Transitional |
| `execution-log.schema.json` | `tcms/test-execution.schema.v1.json` | draft-07 | Transitional |
| `verification-output.schema.json` | `tcms/test-result.schema.v1.json` | draft-07 | Transitional |
| `verification-result.schema.json` | `tcms/test-verification.schema.v1.json` | draft-07 | Transitional |

**Analysis:**
- These schemas are nearly identical to their v1 counterparts
- Main difference: envelope fields (`type`, `schema`) are **optional** instead of **required**
- `test-case.schema.json` uses older draft-04 (others use draft-07)
- Likely used during migration period to maintain backward compatibility

**Recommendation:** 
- **Consider migration path** to fully versioned equivalents
- Update documentation to point to v1 schemas
- Add deprecation notices
- Plan removal timeline once all consumers migrate

---

### 5. Verification Methods Collection (Unique - Keep)

Specialized schemas for different verification approaches. These are **unique and valuable** for supporting diverse verification methodologies.

| File | Method Type | Domain | Key Features |
|------|-------------|--------|--------------|
| `verification_methods/test/schema.json` | Test | General | Test-based verification with test sequences and steps |
| `verification_methods/analysis/schema.json` | Analysis | General | Analytical verification with calculations, models, acceptance criteria |
| `verification_methods/demonstration/schema.json` | Demonstration | General | Operational demonstrations with procedures and observations |
| `verification_methods/inspection/schema.json` | Inspection | General | Inspection/review verification with checklists |
| `verification_methods/common_criteria/schema.json` | Common Criteria | Security | Security evaluation with EAL levels, SFRs, SARs, TOE security functions, vulnerability assessment |
| `verification_methods/high_assurance/schema.json` | DO-178C | Safety-Critical | Aviation software verification with DAL levels, structural coverage (MC/DC), traceability, configuration management |
| `verification_methods/result/schema.json` | Result | General | Generic result reporting format for verification methods |

**Status:** ✅ **Keep all** - These support diverse verification methodologies:
- **General Methods:** test, analysis, demonstration, inspection
- **Security:** Common Criteria (EAL1-7) for high-security systems
- **Safety:** DO-178C/DO-254 for safety-critical aviation software
- **Results:** Unified result reporting format

**JSON Schema Version:** All use draft-04

**Notes:**
- These schemas are domain-specific and not duplicates
- Support different regulatory/compliance frameworks
- May benefit from envelope migration in future versions
- Current draft-04 version is stable for legacy tools

---

## Duplication Analysis Details

### Confirmed Duplicates

#### 1. `verification_schema.json` vs `test-verification.schema.v1.json`

**Comparison:**

| Feature | `verification_schema.json` (Legacy) | `test-verification.schema.v1.json` (Current) |
|---------|-------------------------------------|---------------------------------------------|
| Envelope | ❌ No | ✅ Yes (required) |
| JSON Schema | draft-07 | draft-07 |
| Structure | Test results with sequences | Test results with sequences |
| Requirement Tracking | Top-level only (requirement, item, tc) | Multi-level (test, sequence, step) |
| Step Results | Pass/Fail/NotExecuted | Pass/Fail/NotExecuted (externally tagged) |
| Use Case | Legacy output format | Current standard with envelope |

**Verdict:** ✅ **Confirmed duplicate** - Legacy version without envelope support.

---

#### 2. `verification-schema.json` vs `test-verification.schema.v1.json`

**Comparison:**

| Feature | `verification-schema.json` (Legacy) | `test-verification.schema.v1.json` (Current) |
|---------|-------------------------------------|---------------------------------------------|
| Envelope | ❌ No | ✅ Yes (required) |
| JSON Schema | draft-07 | draft-07 |
| Structure | Simplified test results | Full test results with sequences |
| Requirement Tracking | ❌ None | Multi-level (test, sequence, step) |
| Fail Structure | Simplified (step, description, reason) | Complete (step, description, expected, actual_result, actual_output, reason) |
| Use Case | Simplified legacy format | Current comprehensive standard |

**Verdict:** ✅ **Confirmed duplicate** - Simplified legacy version without envelope or requirement tracking.

---

### Potential Duplicates (Container Schemas)

#### 3. `test_results/container_schema.json` vs `test-results-container.schema.v1.json`

**Comparison:**

| Feature | `container_schema.json` (Legacy) | `test-results-container.schema.v1.json` (Current) |
|---------|----------------------------------|---------------------------------------------------|
| Envelope | ❌ No | ✅ Yes (required) |
| JSON Schema | draft-07 | draft-07 |
| Structure | title, project, test_date, test_results, metadata | Same + envelope fields |
| Test Results Items | Generic `object` type (loosely typed) | Fully specified with sequences, step_results, counts |
| Metadata Required | environment, platform, executor | execution_duration, total_test_cases, passed_test_cases, failed_test_cases |
| Metadata Optional | execution_duration, counts | environment, platform, executor |
| Use Case | Legacy container format | Current container with metrics |

**Analysis:**
- Structural similarities but different metadata requirements
- Legacy version has loose typing for test_results items
- Current v1 has comprehensive metadata with execution metrics
- Different emphasis: legacy focuses on environment, v1 focuses on metrics

**Verdict:** ⚠️ **Potential duplicate** - Similar purpose but different metadata philosophy. Consider deprecating legacy version.

---

#### 4. `testcase_results_container/schema.json` vs `test-results-container.schema.v1.json`

**Comparison:**

| Feature | `testcase_results_container` (Legacy) | `test-results-container.schema.v1.json` (Current) |
|---------|---------------------------------------|---------------------------------------------------|
| Envelope | ❌ No | ✅ Yes (required) |
| JSON Schema | draft-07 | draft-07 |
| Structure | More detailed with complex definitions | Standard structure with inline step results |
| Special Definitions | TestStepExecution, expectedOnlyPair, passFailResult | Inline definitions |
| Requirement Tracking | Test result level | Multi-level (test, sequence, step) |
| Step Results | Expected/actual pairs in result/output | Externally tagged Pass/Fail/NotExecuted |
| Metadata | Same as test_results version | Comprehensive with execution metrics |

**Analysis:**
- More sophisticated than `test_results/container_schema.json`
- Uses different approach for expected/actual pairs
- Has `requirement`/`item`/`tc` tracking at test result level
- Different step result encoding (pair-based vs tagged enum)

**Verdict:** ⚠️ **Potential duplicate** - Different encoding style but similar purpose. Consider migration to v1 standard.

---

#### 5. `container/schema.json` - Minimal Legacy Schema

**Analysis:**
- Only 3 fields: `date`, `product`, `description`
- Uses JSON Schema draft-04
- Minimal and incomplete compared to current container schemas
- Does not match structure of either container-config or test-results-container

**Verdict:** ✅ **Superseded** - Too minimal to be useful. Superseded by both `container-config.schema.v1.json` and `test-results-container.schema.v1.json`.

---

## JSON Schema Version Distribution

| Version | Count | Files |
|---------|-------|-------|
| draft-07 | 16 | Most current schemas (v1 envelope schemas and many legacy) |
| draft-04 | 8 | Older schemas (verification methods collection, test-case.schema.json, container/schema.json) |

**Recommendation:** Consider migrating draft-04 schemas to draft-07 for consistency, especially `test-case.schema.json` if still in use.

---

## Directory Structure Analysis

```
schemas/
├── *.schema.json                                    # 7 root-level schemas (5 transitional, 1 envelope, 1 container-config)
├── tcms-envelope.schema.json                        # Core envelope meta-schema
├── tcms/
│   ├── *.schema.v1.json                            # 6 versioned schemas (CURRENT STANDARD)
│   ├── container/schema.json                        # 1 minimal legacy (DEPRECATE)
│   ├── schemas/
│   │   ├── verification_schema.json                 # DUPLICATE - Deprecate
│   │   └── verification-schema.json                 # DUPLICATE - Deprecate
│   ├── test_results/container_schema.json          # POTENTIAL DUPLICATE - Consider deprecate
│   ├── testcase_results_container/schema.json      # POTENTIAL DUPLICATE - Consider deprecate
│   └── verification_methods/                        # 7 verification method schemas (KEEP)
│       ├── test/schema.json
│       ├── analysis/schema.json
│       ├── demonstration/schema.json
│       ├── inspection/schema.json
│       ├── common_criteria/schema.json
│       ├── high_assurance/schema.json
│       └── result/schema.json
```

**Observations:**
- Clear separation between versioned (`tcms/*.schema.v1.json`) and legacy schemas
- Multiple legacy directories (`schemas/`, `test_results/`, `testcase_results_container/`, `container/`) suggest incremental evolution
- Verification methods in organized subdirectory structure
- Root-level schemas appear to be transitional versions during envelope migration

---

## Recommendations

### Immediate Actions

1. **Deprecate Confirmed Duplicates:**
   - `tcms/schemas/verification_schema.json` → Use `tcms/test-verification.schema.v1.json`
   - `tcms/schemas/verification-schema.json` → Use `tcms/test-verification.schema.v1.json`
   - `tcms/container/schema.json` → Use `tcms/container-config.schema.v1.json` or `tcms/test-results-container.schema.v1.json`

2. **Evaluate Container Duplicates:**
   - Assess usage of `test_results/container_schema.json` and `testcase_results_container/schema.json`
   - Plan migration to `test-results-container.schema.v1.json`
   - Document breaking changes (metadata requirements, step result encoding)

3. **Document Transitional Schemas:**
   - Add deprecation notices to root-level schemas (`test-case.schema.json`, `container_config.schema.json`, etc.)
   - Document migration path to v1 equivalents
   - Set deprecation timeline

### Medium-Term Actions

1. **Schema Consolidation:**
   - Remove deprecated schemas after migration period
   - Clean up legacy directories (`schemas/`, `test_results/`, `testcase_results_container/`, `container/`)
   - Update documentation and examples

2. **Verification Methods Enhancement:**
   - Consider adding envelope support to verification method schemas
   - Evaluate migration to JSON Schema draft-07
   - Maintain backward compatibility for existing tools

3. **Documentation Improvements:**
   - Create migration guides for each deprecated schema
   - Document envelope pattern benefits and usage
   - Provide schema selection guidance (when to use test-verification vs test-result)

### Long-Term Actions

1. **Versioning Strategy:**
   - Establish process for creating v2, v3, etc.
   - Define backward compatibility guarantees
   - Document schema evolution practices

2. **Tooling:**
   - Create validation tools for envelope compliance
   - Build schema migration utilities
   - Implement automated schema testing

3. **Governance:**
   - Define schema approval process
   - Establish schema deprecation policy
   - Create schema maintenance guidelines

---

## Schema Selection Guide

### When to Use Each Schema

#### Test Case Definitions
- **Use:** `tcms/test-case.schema.v1.json`
- **When:** Defining test cases with sequences, steps, initial conditions, hooks
- **Envelope:** ✅ Required
- **Avoid:** `test-case.schema.json` (transitional), `verification_methods/test/schema.json` (different purpose)

#### Test Execution Logs
- **Use:** `tcms/test-execution.schema.v1.json`
- **When:** Recording individual test step executions
- **Envelope:** ✅ Required
- **Avoid:** `execution-log.schema.json` (transitional)

#### Test Verification Results (Individual Test)
- **Use:** `tcms/test-verification.schema.v1.json` or `tcms/test-result.schema.v1.json`
- **When:** Storing verification results for a single test case
- **Envelope:** ✅ Required
- **Notes:** Both schemas are very similar; choose based on naming preference or tool compatibility
- **Avoid:** `verification_schema.json`, `verification-schema.json`, `verification-result.schema.json`, `verification-output.schema.json` (all legacy/transitional)

#### Test Results Container (Multiple Tests)
- **Use:** `tcms/test-results-container.schema.v1.json`
- **When:** Aggregating results from multiple test cases with metadata
- **Envelope:** ✅ Required
- **Avoid:** `test_results/container_schema.json`, `testcase_results_container/schema.json` (legacy)

#### Container Configuration
- **Use:** `tcms/container-config.schema.v1.json`
- **When:** Configuring metadata for wrapping test results
- **Envelope:** ✅ Required
- **Avoid:** `container_config.schema.json` (transitional), `container/schema.json` (minimal legacy)

#### Verification Methods
- **Use:** `tcms/verification_methods/{type}/schema.json`
- **When:** Defining verification approaches for specific methodologies
- **Types:** test, analysis, demonstration, inspection, common_criteria, high_assurance, result
- **Notes:** These are domain-specific and not general test schemas

---

## Appendix: Schema File Details

### Complete File Listing with Metadata

| # | File Path | Size | Draft | Envelope | Status |
|---|-----------|------|-------|----------|--------|
| 1 | `tcms-envelope.schema.json` | Small | 07 | Meta | Core |
| 2 | `tcms/test-case.schema.v1.json` | Large | 07 | ✅ | Current |
| 3 | `tcms/test-execution.schema.v1.json` | Medium | 07 | ✅ | Current |
| 4 | `tcms/test-verification.schema.v1.json` | Large | 07 | ✅ | Current |
| 5 | `tcms/test-result.schema.v1.json` | Large | 07 | ✅ | Current |
| 6 | `tcms/test-results-container.schema.v1.json` | Large | 07 | ✅ | Current |
| 7 | `tcms/container-config.schema.v1.json` | Medium | 07 | ✅ | Current |
| 8 | `tcms/schemas/verification_schema.json` | Large | 07 | ❌ | Duplicate |
| 9 | `tcms/schemas/verification-schema.json` | Medium | 07 | ❌ | Duplicate |
| 10 | `tcms/test_results/container_schema.json` | Medium | 07 | ❌ | Potential Dup |
| 11 | `tcms/testcase_results_container/schema.json` | Large | 07 | ❌ | Potential Dup |
| 12 | `tcms/container/schema.json` | Small | 04 | ❌ | Legacy |
| 13 | `test-case.schema.json` | Large | 04 | Optional | Transitional |
| 14 | `container_config.schema.json` | Medium | 07 | Optional | Transitional |
| 15 | `execution-log.schema.json` | Medium | 07 | Optional | Transitional |
| 16 | `verification-output.schema.json` | Large | 07 | Optional | Transitional |
| 17 | `verification-result.schema.json` | Large | 07 | Optional | Transitional |
| 18 | `tcms/verification_methods/test/schema.json` | Medium | 04 | ❌ | Unique |
| 19 | `tcms/verification_methods/analysis/schema.json` | Medium | 04 | ❌ | Unique |
| 20 | `tcms/verification_methods/demonstration/schema.json` | Small | 04 | ❌ | Unique |
| 21 | `tcms/verification_methods/inspection/schema.json` | Small | 04 | ❌ | Unique |
| 22 | `tcms/verification_methods/common_criteria/schema.json` | Large | 04 | ❌ | Unique |
| 23 | `tcms/verification_methods/high_assurance/schema.json` | Large | 04 | ❌ | Unique |
| 24 | `tcms/verification_methods/result/schema.json` | Small | 04 | ❌ | Unique |

---

## Summary Statistics

### By Status
- **Current (v1 Envelope):** 7 schemas (29%)
- **Unique (Verification Methods):** 7 schemas (29%)
- **Transitional:** 5 schemas (21%)
- **Duplicates/Legacy:** 5 schemas (21%)

### By Envelope Support
- **Full Envelope (Required):** 7 schemas
- **Optional Envelope:** 5 schemas
- **No Envelope:** 12 schemas

### By JSON Schema Version
- **Draft-07:** 16 schemas (67%)
- **Draft-04:** 8 schemas (33%)

### Deprecation Candidates
- **Confirmed for Deprecation:** 3 schemas
  - `tcms/schemas/verification_schema.json`
  - `tcms/schemas/verification-schema.json`
  - `tcms/container/schema.json`
  
- **Consider for Deprecation:** 2 schemas
  - `tcms/test_results/container_schema.json`
  - `tcms/testcase_results_container/schema.json`

- **Transitional (Document Migration Path):** 5 schemas
  - `test-case.schema.json`
  - `container_config.schema.json`
  - `execution-log.schema.json`
  - `verification-output.schema.json`
  - `verification-result.schema.json`

---

## Conclusion

The schema audit reveals a well-structured migration from legacy schemas to a modern envelope-based versioning system. The 7 current v1 schemas in `schemas/tcms/*.schema.v1.json` represent the production-ready standard, while identified duplicates and transitional schemas provide clear opportunities for consolidation and cleanup.

Key strengths:
- ✅ Clear envelope pattern implementation
- ✅ Comprehensive v1 schema coverage
- ✅ Rich verification methods collection
- ✅ Consistent JSON Schema draft-07 usage in current schemas

Recommended focus areas:
- 🔄 Deprecate 3 confirmed duplicate schemas
- 🔄 Evaluate and migrate 2 container duplicates
- 🔄 Document migration path for 5 transitional schemas
- 🔄 Consider envelope migration for verification methods

This audit provides a solid foundation for schema governance, deprecation planning, and future evolution of the TCMS schema ecosystem.
