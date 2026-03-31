# Schema Documentation and Audit

This document provides a quick overview and links to comprehensive schema documentation.

## 📖 Documentation Location

All schema documentation is located in the `schemas/` directory:

### 🚀 Start Here
- **[schemas/README.md](schemas/README.md)** - Main schema documentation index
- **[schemas/SCHEMA_QUICK_REFERENCE.md](schemas/SCHEMA_QUICK_REFERENCE.md)** - Quick lookup guide

### 📊 Comprehensive Audit
- **[schemas/SCHEMA_AUDIT.md](schemas/SCHEMA_AUDIT.md)** - Full audit report with:
  - Inventory of all 24 schema files
  - Duplication analysis
  - Migration guidance
  - Recommendations for deprecation
  - Schema selection guide
- **[schemas/SCHEMA_AUDIT.csv](schemas/SCHEMA_AUDIT.csv)** - Spreadsheet format for filtering/analysis

### ✅ Schema Consolidation
- **[schemas/tcms/CONTAINER_SCHEMAS.md](schemas/tcms/CONTAINER_SCHEMAS.md)** - Container schema consolidation guide (3 redundant schemas removed)

## 🎯 Quick Summary

### Production Schemas (Use These!)
All current schemas are in `schemas/tcms/*.schema.v1.json`:
- ✅ **test-case.schema.v1.json** - Test case definitions
- ✅ **test-execution.schema.v1.json** - Execution logs
- ✅ **test-verification.schema.v1.json** - Verification results
- ✅ **test-result.schema.v1.json** - Test results
- ✅ **test-results-container.schema.v1.json** - Results container
- ✅ **container-config.schema.v1.json** - Container configuration

### Key Findings

#### ✅ Container Schema Consolidation (COMPLETED)
Three redundant container schemas have been removed:
1. `tcms/container/schema.json` (REMOVED)
2. `tcms/test_results/container_schema.json` (REMOVED)
3. `tcms/testcase_results_container/schema.json` (REMOVED)

**Use instead:**
- Canonical: `tcms/test-results-container.schema.v1.json` (with envelope)
- Working: `data/testcase_results_container/schema.json` (backward compatible)

See [schemas/tcms/CONTAINER_SCHEMAS.md](schemas/tcms/CONTAINER_SCHEMAS.md) for details.

#### 🔄 Transitional Schemas (Document Migration)
Root-level schemas with optional envelope support:
- `test-case.schema.json`
- `container_config.schema.json`
- `execution-log.schema.json`
- `verification-output.schema.json`
- `verification-result.schema.json`

### Statistics
- **Total schemas:** 22
- **Production (v1):** 7 (32%)
- **Verification methods:** 7 (32%)
- **Transitional:** 5 (23%)
- **Deprecated/Legacy:** 3 (14%)

## 🔍 What Was Audited

The audit covers all JSON schema files in the `schemas/` directory, analyzing:
1. **Purpose** - What each schema is for
2. **Duplication Status** - Whether it duplicates or supersedes another schema
3. **Schema Type** - Legacy (non-envelope) vs Versioned (envelope-compliant)
4. **Recommendations** - Keep, deprecate, or migrate

## 📋 Key Identified Issues

### 1. Container Schema Consolidation ✅ COMPLETED
- **Issue:** Three redundant container schemas
- **Resolution:** All removed and consolidated
  - `test_results/container_schema.json` (REMOVED)
  - `testcase_results_container/schema.json` (REMOVED)
  - `container/schema.json` (REMOVED)
- **Current schemas:**
  - Canonical: `test-results-container.schema.v1.json` (with envelope)
  - Working: `data/testcase_results_container/schema.json` (backward compatible)
- **Documentation:** [schemas/tcms/CONTAINER_SCHEMAS.md](schemas/tcms/CONTAINER_SCHEMAS.md)

### 2. Transitional Schemas
- **Issue:** Root-level schemas with optional envelope fields
- **Impact:** Unclear migration status, may confuse users
- **Action:** Document migration path, set deprecation timeline

## 🎓 Recommendations

### Immediate (Priority 1)
1. ✅ **Deprecate confirmed duplicates** - container/schema.json
2. 📝 **Update documentation** - Point to v1 schemas as standard
3. ⚠️ **Add deprecation warnings** - In code that uses legacy schemas

### Short-term (Priority 2)
4. 🔄 **Assess container duplicates** - Evaluate usage and plan migration
5. 📚 **Create migration guides** - Detailed steps for each deprecated schema
6. 🔧 **Update tooling** - Ensure all tools use v1 schemas

### Long-term (Priority 3)
7. 🗑️ **Remove deprecated schemas** - After transition period (6 months)
8. 📖 **Envelope migration for verification methods** - Consider adding envelope support
9. 🏗️ **Versioning policy** - Establish process for v2, v3, etc.

## 🔗 Related Documentation

- **[AGENTS.md](AGENTS.md)** - Workspace and build system documentation
- **[schemas/README.md](schemas/README.md)** - Main schema documentation
- Individual schema files have inline documentation via `$schema`, `title`, and `description` fields

## 📞 Questions?

For schema-related questions:
1. Start with [schemas/SCHEMA_QUICK_REFERENCE.md](schemas/SCHEMA_QUICK_REFERENCE.md)
2. Review [schemas/SCHEMA_AUDIT.md](schemas/SCHEMA_AUDIT.md) for detailed analysis
3. Check deprecation notices in legacy directories
4. Examine schema files directly in `schemas/tcms/*.schema.v1.json`

---

**Last Updated:** 2024
**Audit Scope:** All 22 schema files in `schemas/` directory
**Status:** ✅ Audit Complete - Documentation Generated
