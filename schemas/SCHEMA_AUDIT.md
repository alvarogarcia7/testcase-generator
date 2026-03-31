# Schema Audit Report

**Generated:** 2024
**Purpose:** Comprehensive audit of all schema files in the `schemas/` directory, documenting purpose, duplication status, and versioning compliance.

## Executive Summary

### Schema Inventory
- **Total Schema Files:** 19 (after consolidation)
- **Versioned (Envelope-compliant):** 7
- **Legacy (Non-envelope):** 12
- **Unique Schemas:** 18
- **Removed/Consolidated Schemas:** 3

### Key Findings

1. **Envelope System**: The project has successfully migrated to a versioned envelope system (`tcms-envelope.schema.json`) with 7 production-ready v1 schemas in `schemas/tcms/*.schema.v1.json`.

2. **Container Consolidation** (COMPLETED): 
   - ✅ Removed `test_results/container_schema.json` (redundant)
   - ✅ Removed `testcase_results_container/schema.json` (redundant)
   - ✅ Removed `container/schema.json` (minimal legacy)
   - All consolidated into canonical `test-results-container.schema.v1.json`
   - Backward compatible working schema remains at `data/testcase_results_container/schema.json`
   - See [tcms/CONTAINER_SCHEMAS.md](tcms/CONTAINER_SCHEMAS.md) for details

3. **Migration Status**: 5 root-level schemas (`test-case.schema.json`, `container_config.schema.json`, `execution-log.schema.json`, `verification-output.schema.json`, `verification-result.schema.json`) have optional envelope support and appear to be transitional versions.

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

### 3. Container Schema Consolidation (COMPLETED)

#### 3.1 Removed Container Schemas

Three redundant container schemas have been **removed** and consolidated:

| File | Status | Reason |
|------|--------|--------|
| `tcms/test_results/container_schema.json` | ❌ REMOVED | Minimal container, loose typing, no envelope |
| `tcms/testcase_results_container/schema.json` | ❌ REMOVED | Different encoding (expected/actual pairs), no envelope |
| `tcms/container/schema.json` | ❌ REMOVED | Minimal legacy (3 fields only), draft-04 |

**Consolidation Result:**

These have been consolidated into:
- **Canonical Schema:** `tcms/test-results-container.schema.v1.json` (with envelope support)
- **Working Schema:** `data/testcase_results_container/schema.json` (backward compatible, used by verifier)

See [tcms/CONTAINER_SCHEMAS.md](tcms/CONTAINER_SCHEMAS.md) for:
- Detailed comparison of removed schemas
- Migration guidance
- Code and script references
- Working schema vs canonical schema differences

**Analysis of Removed Schemas:**

**`test_results/container_schema.json`** (removed):
- Basic container with `title`, `project`, `test_date`, `test_results`, `metadata`
- `test_results` items were loosely typed as generic `object`
- `metadata` required `environment`, `platform`, `executor` but not execution metrics
- No envelope support

**`testcase_results_container/schema.json`** (removed):
- More sophisticated with definitions: `TestStepExecution`, `expectedOnlyPair`, `passFailResult`
- Different encoding approach using expected/actual pairs
- Supported `requirement`/`item`/`tc` tracking at test result level
- No envelope support

**`container/schema.json`** (removed):
- Only 3 fields: `date`, `product`, `description`
- Used JSON Schema draft-04
- Too minimal for modern container needs

**Current Standard:** `test-results-container.schema.v1.json` supersedes all three:
- Full envelope support
- Comprehensive metadata with execution metrics
- Detailed step results with Pass/Fail/NotExecuted variants
- Optional requirement/item/tc tracking at multiple levels

**Migration Path:** ✅ **COMPLETED**
1. ✅ Removed redundant schema directories
2. ✅ Created consolidation guide: [tcms/CONTAINER_SCHEMAS.md](tcms/CONTAINER_SCHEMAS.md)
3. ✅ Updated documentation to reference canonical and working schemas
4. Code references remain pointing to working schema: `data/testcase_results_container/schema.json`

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

## Duplication Analysis Details (RESOLVED)

### Container Schemas (Consolidated)

The container schema duplication issue has been **resolved** through consolidation. Three redundant schemas were removed:

#### 1. `test_results/container_schema.json` - ❌ REMOVED

**Previous Status:** Redundant legacy container

**Analysis:**
- Had loose typing for test_results items
- Different metadata requirements than v1
- Missing envelope support

**Resolution:** Removed. Use canonical `tcms/test-results-container.schema.v1.json` or working schema `data/testcase_results_container/schema.json`

---

#### 2. `testcase_results_container/schema.json` - ❌ REMOVED

**Previous Status:** Redundant with different encoding

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
- More sophisticated with complex definitions
- Used different expected/actual pair encoding
- Had `requirement`/`item`/`tc` tracking at test result level
- Different step result encoding (pair-based vs tagged enum)

**Resolution:** Removed. Use canonical `tcms/test-results-container.schema.v1.json` or working schema `data/testcase_results_container/schema.json`

---

#### 3. `container/schema.json` - ❌ REMOVED

**Previous Status:** Minimal legacy schema

**Analysis:**
- Only 3 fields: `date`, `product`, `description`
- Used JSON Schema draft-04
- Too minimal for modern container needs
- Did not match structure of current container schemas

**Resolution:** Removed. Use `tcms/container-config.schema.v1.json` or `tcms/test-results-container.schema.v1.json` depending on use case.

---

**Consolidation Summary:**
- All three redundant schemas removed from `schemas/tcms/`
- Created comprehensive guide: [tcms/CONTAINER_SCHEMAS.md](tcms/CONTAINER_SCHEMAS.md)
- Working schema remains at `data/testcase_results_container/schema.json` for backward compatibility
- All code references updated to point to appropriate schemas

---

## JSON Schema Version Distribution

| Version | Count | Files |
|---------|-------|-------|
| draft-07 | 14 | Most current schemas (v1 envelope schemas and transitional) |
| draft-04 | 5 | Older schemas (verification methods collection, test-case.schema.json) |

**Note:** Count reduced after removing 3 redundant container schemas.

**Recommendation:** Consider migrating draft-04 schemas to draft-07 for consistency, especially `test-case.schema.json` if still in use.

---

## Directory Structure Analysis

```
schemas/
├── *.schema.json                                    # 6 root-level schemas (5 transitional, 1 envelope)
├── tcms-envelope.schema.json                        # Core envelope meta-schema
├── tcms/
│   ├── *.schema.v1.json                            # 7 versioned schemas (CURRENT STANDARD)
│   ├── CONTAINER_SCHEMAS.md                         # Container consolidation guide
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
- Clear separation between versioned (`tcms/*.schema.v1.json`) and transitional schemas
- Container schema consolidation completed (3 redundant directories removed)
- Verification methods in organized subdirectory structure
- Root-level schemas appear to be transitional versions during envelope migration
- New consolidation guide documents removed schemas and migration path

---

## Recommendations

### Completed Actions ✅

1. ✅ **Container Schema Consolidation:**
   - Removed `tcms/container/schema.json`
   - Removed `tcms/test_results/container_schema.json`
   - Removed `tcms/testcase_results_container/schema.json`
   - Created consolidation guide: [tcms/CONTAINER_SCHEMAS.md](tcms/CONTAINER_SCHEMAS.md)
   - Updated all documentation references

### Immediate Actions

1. **Document Transitional Schemas:**
   - Add deprecation notices to root-level schemas (`test-case.schema.json`, `container_config.schema.json`, etc.)
   - Document migration path to v1 equivalents
   - Set deprecation timeline

### Medium-Term Actions

1. **Complete Transitional Schema Migration:**
   - Remove transitional root-level schemas after migration period
   - Update all code references to use v1 schemas
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
- **Avoid:** `verification-result.schema.json`, `verification-output.schema.json` (transitional)

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
| 8 | `tcms/test_results/container_schema.json` | Medium | 07 | ❌ | Potential Dup |
| 9 | `tcms/testcase_results_container/schema.json` | Large | 07 | ❌ | Potential Dup |
| 10 | `tcms/container/schema.json` | Small | 04 | ❌ | Legacy |
| 11 | `test-case.schema.json` | Large | 04 | Optional | Transitional |
| 12 | `container_config.schema.json` | Medium | 07 | Optional | Transitional |
| 13 | `execution-log.schema.json` | Medium | 07 | Optional | Transitional |
| 14 | `verification-output.schema.json` | Large | 07 | Optional | Transitional |
| 15 | `verification-result.schema.json` | Large | 07 | Optional | Transitional |
| 16 | `tcms/verification_methods/test/schema.json` | Medium | 04 | ❌ | Unique |
| 17 | `tcms/verification_methods/analysis/schema.json` | Medium | 04 | ❌ | Unique |
| 18 | `tcms/verification_methods/demonstration/schema.json` | Small | 04 | ❌ | Unique |
| 19 | `tcms/verification_methods/inspection/schema.json` | Small | 04 | ❌ | Unique |
| 20 | `tcms/verification_methods/common_criteria/schema.json` | Large | 04 | ❌ | Unique |
| 21 | `tcms/verification_methods/high_assurance/schema.json` | Large | 04 | ❌ | Unique |
| 22 | `tcms/verification_methods/result/schema.json` | Small | 04 | ❌ | Unique |

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
- **Confirmed for Deprecation:** 1 schema
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
- 🔄 Deprecate 1 confirmed duplicate schema
- 🔄 Evaluate and migrate 2 container duplicates
- 🔄 Document migration path for 5 transitional schemas
- 🔄 Consider envelope migration for verification methods

This audit provides a solid foundation for schema governance, deprecation planning, and future evolution of the TCMS schema ecosystem.
