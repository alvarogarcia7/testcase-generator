# Schemas Directory Changelog

## [2026-03-31] - Schema Directory Reorganization

### Date of Change
**March 31, 2026** - Comprehensive schema directory reorganization including consolidation of duplicate schemas, relocation of templates, and migration of sample files.

---

## Summary

This reorganization improves the schema directory structure by:
1. **Consolidating duplicate container schemas** into canonical versioned schemas
2. **Relocating template files** to a dedicated `schemas/templates/` directory
3. **Migrating sample data** to `schemas/tcms/samples/` directory
4. **Improving separation of concerns** between schema definitions, templates, and test data

---

## Removed Duplicate Files

### Container Schema Consolidation (Commit: 52f0ea4, Date: 2026-03-31 18:35:54)

The following duplicate and redundant container schema files were **removed** as part of the consolidation effort:

#### Removed: `schemas/tcms/container/` (28 files removed)
- **Schema Files:**
  - `schema.json` - Minimal legacy schema (3 fields only, draft-04)
- **Data/Template Files:**
  - `data.yml`
  - `template.j2`
  - `template_asciidoc.adoc`
  - `test_cases.adoc`
- **Documentation:**
  - `DEPRECATED.md` (155 lines)

**Reason:** Too minimal for modern container needs. Only contained `date`, `product`, and `description` fields. Used outdated JSON Schema draft-04.

---

#### Removed: `schemas/tcms/test_results/` (23 files removed)
- **Schema Files:**
  - `container_schema.json` - Redundant container schema with loose typing
- **Data Files:**
  - `container_data.yml` (343 lines)
  - `sample_gsma_4.4.2.2_TC.yml`
  - `sample_gsma_4.4.2.3_TC.yml`
  - `sample_gsma_4.4.2.4_AN.yml`
  - `sample_gsma_4.4.2.5_DM.yml`
  - `sample_gsma_4.4.2.6_IN.yml`
- **Template Files:**
  - `container_template.j2`
  - `container_template_asciidoc.adoc`
  - `result_template_asciidoc.adoc`
  - `test_container_result.adoc.j2`
  - `test_container_result.md.j2`
- **Documentation:**
  - `DEPRECATED.md` (115 lines)
  - `test_cases.adoc` (615 lines)

**Reason:** Redundant with canonical schema. Had loose typing for `test_results` items (generic `object`). Missing envelope support. Different metadata requirements than versioned schemas.

---

#### Removed: `schemas/tcms/testcase_results_container/` (4 files removed)
- **Schema Files:**
  - `schema.json` - Redundant schema with different encoding approach
- **Data Files:**
  - `data_sample.yml` (253 lines) - **Migrated** to `schemas/tcms/samples/testcase_results_container_sample.yml`
- **Template Files:**
  - `template_asciidoc.adoc` (365 lines)
- **Documentation:**
  - `DEPRECATED.md` (261 lines)

**Reason:** Redundant with canonical schema. Used different expected/actual pair encoding. More sophisticated with complex definitions (`TestStepExecution`, `expectedOnlyPair`, `passFailResult`) but superseded by canonical versioned schema.

---

### Total Files Removed
- **3 duplicate schema directories** completely removed
- **3 redundant container schema files** eliminated
- **4,130 lines of code/documentation** removed from duplicate locations
- **Consolidated into:** Canonical `tcms/test-results-container.schema.v1.json` and working schema `data/testcase_results_container/schema.json`

---

## Moved Template Files

### Template Relocation to `schemas/templates/` (Commit: 1693838, Date: 2026-03-31 18:51:18)

All Jinja2 (`.j2`) and AsciiDoc (`.adoc`) template files were relocated from `schemas/tcms/verification_methods/` subdirectories to a centralized `schemas/templates/` directory with mirrored structure.

**New Location:** `schemas/templates/`

#### Template Files Moved (16 files total)

**From:** `schemas/tcms/verification_methods/analysis/`  
**To:** `schemas/templates/verification_methods/analysis/`
- `template.j2`
- `template_asciidoc.adoc`

**From:** `schemas/tcms/verification_methods/common_criteria/`  
**To:** `schemas/templates/verification_methods/common_criteria/`
- `template.j2`
- `template_asciidoc.adoc`

**From:** `schemas/tcms/verification_methods/demonstration/`  
**To:** `schemas/templates/verification_methods/demonstration/`
- `template.j2`
- `template_asciidoc.adoc`

**From:** `schemas/tcms/verification_methods/high_assurance/`  
**To:** `schemas/templates/verification_methods/high_assurance/`
- `template.j2`
- `template_asciidoc.adoc`

**From:** `schemas/tcms/verification_methods/inspection/`  
**To:** `schemas/templates/verification_methods/inspection/`
- `template.j2`
- `template_asciidoc.adoc`

**From:** `schemas/tcms/verification_methods/result/`  
**To:** `schemas/templates/verification_methods/result/`
- `template.j2`
- `template_asciidoc.adoc`

**From:** `schemas/tcms/verification_methods/test/`  
**To:** `schemas/templates/verification_methods/test/`
- `template.j2`
- `template_asciidoc.adoc`

**From:** `schemas/tcms/verification_methods/`  
**To:** `schemas/templates/verification_methods/`
- `requirement_aggregation_template.j2`
- `requirement_aggregation_template.adoc`

**Template Count:**
- **8 Jinja2 templates** (`.j2` files)
- **8 AsciiDoc templates** (`.adoc` files)
- **Total:** 16 template files relocated

---

## Moved Sample Files

### Sample Data Relocation to `schemas/tcms/samples/` (Commit: 1693838, Date: 2026-03-31 18:51:18)

Sample data files were migrated from `data/testcase_results_container/` to a centralized `schemas/tcms/samples/` directory for better organization and colocation with related schemas.

**New Location:** `schemas/tcms/samples/`

#### Sample Files Moved (1 file)

**From:** `data/testcase_results_container/data_sample.yml`  
**To:** `schemas/tcms/samples/testcase_results_container_sample.yml`
- **Size:** 105 lines (253 lines in original location before consolidation)
- **Purpose:** Sample test results container data for validation and testing

**Sample Count:**
- **1 YAML sample file** relocated

---

## Updated Canonical Schema Locations

### Current Canonical Schema Structure

After consolidation, the canonical schema locations are:

```
schemas/
├── tcms-envelope.schema.json              # Core envelope meta-schema
├── tcms/
│   ├── test-case.schema.v1.json           # Test case definitions (CURRENT)
│   ├── test-execution.schema.v1.json      # Execution log entries (CURRENT)
│   ├── test-verification.schema.v1.json   # Test verification results (CURRENT)
│   ├── test-result.schema.v1.json         # Test result output (CURRENT)
│   ├── test-results-container.schema.v1.json  # Results container (CURRENT)
│   ├── container-config.schema.v1.json    # Container configuration (CURRENT)
│   ├── verification_methods/
│   │   ├── analysis/schema.json           # Analysis verification method
│   │   ├── common_criteria/schema.json    # Common Criteria (EAL) verification
│   │   ├── demonstration/schema.json      # Demonstration verification method
│   │   ├── high_assurance/schema.json     # DO-178C high-assurance verification
│   │   ├── inspection/schema.json         # Inspection verification method
│   │   ├── result/schema.json             # Generic result reporting
│   │   └── test/schema.json               # Test-based verification method
│   └── samples/
│       └── testcase_results_container_sample.yml  # Sample container data
└── templates/
    └── verification_methods/
        ├── analysis/
        │   ├── template.j2
        │   └── template_asciidoc.adoc
        ├── common_criteria/
        │   ├── template.j2
        │   └── template_asciidoc.adoc
        ├── demonstration/
        │   ├── template.j2
        │   └── template_asciidoc.adoc
        ├── high_assurance/
        │   ├── template.j2
        │   └── template_asciidoc.adoc
        ├── inspection/
        │   ├── template.j2
        │   └── template_asciidoc.adoc
        ├── result/
        │   ├── template.j2
        │   └── template_asciidoc.adoc
        ├── test/
        │   ├── template.j2
        │   └── template_asciidoc.adoc
        ├── requirement_aggregation_template.j2
        └── requirement_aggregation_template.adoc
```

### Root-Level Transitional Schemas (Optional Envelope)

The following root-level schemas remain as **transitional versions** with optional envelope support:
- `test-case.schema.json` (draft-04) → Migrate to `tcms/test-case.schema.v1.json`
- `container_config.schema.json` (draft-07) → Migrate to `tcms/container-config.schema.v1.json`
- `execution-log.schema.json` (draft-07) → Migrate to `tcms/test-execution.schema.v1.json`
- `verification-output.schema.json` (draft-07) → Migrate to `tcms/test-result.schema.v1.json`
- `verification-result.schema.json` (draft-07) → Migrate to `tcms/test-verification.schema.v1.json`

**Note:** These transitional schemas have optional `type` and `schema` envelope fields (not required) and should be migrated to their v1 equivalents.

---

## Migration Notes for Developers

### Understanding the New Directory Structure

The reorganization establishes three distinct categories:

#### 1. **Schemas** (`schemas/` and `schemas/tcms/`)
**Purpose:** JSON Schema definition files for validation

**What belongs here:**
- JSON Schema files (`.schema.json`, `.schema.v1.json`)
- Schema documentation (README, guides)
- Schema audit and reference documents

**What does NOT belong here:**
- Templates (moved to `schemas/templates/`)
- Sample data (moved to `schemas/tcms/samples/`)
- Test fixtures (remain in test directories)

**Key schemas:**
- **Versioned (v1) schemas in `schemas/tcms/`**: Production-ready, envelope-compliant (CURRENT STANDARD)
- **Root-level schemas**: Transitional versions with optional envelope support (DEPRECATED PATH)
- **Verification method schemas**: Domain-specific schemas for different validation approaches

---

#### 2. **Templates** (`schemas/templates/`)
**Purpose:** Jinja2 and AsciiDoc templates for document generation and rendering

**What belongs here:**
- Jinja2 templates (`.j2`) for dynamic content generation
- AsciiDoc templates (`.adoc`) for documentation rendering
- Template-related configuration files

**What does NOT belong here:**
- JSON schemas (remain in `schemas/` or `schemas/tcms/`)
- Actual data or examples (moved to `schemas/tcms/samples/`)

**Structure:**
- Mirrors the verification methods directory structure from schemas
- Each verification method has its own subdirectory with templates
- Top-level `README.md` documents template usage and conventions

---

#### 3. **Samples** (`schemas/tcms/samples/`)
**Purpose:** Sample data files for testing, validation, and documentation

**What belongs here:**
- YAML sample data files (`.yml`, `.yaml`)
- Example test cases and results
- Sample containers and configurations

**What does NOT belong here:**
- JSON schemas (remain in `schemas/tcms/`)
- Templates (moved to `schemas/templates/`)
- Production test data (should be in project-specific locations)

**Structure:**
- Sample files are named descriptively (e.g., `testcase_results_container_sample.yml`)
- Top-level `README.md` documents sample purposes and usage

---

### Code Update Requirements

#### Path Updates Required

If your code references any of the following old paths, update them:

**Template Path Changes:**
```
OLD: schemas/tcms/verification_methods/{method}/template.j2
NEW: schemas/templates/verification_methods/{method}/template.j2

OLD: schemas/tcms/verification_methods/{method}/template_asciidoc.adoc
NEW: schemas/templates/verification_methods/{method}/template_asciidoc.adoc

OLD: schemas/tcms/verification_methods/requirement_aggregation_template.j2
NEW: schemas/templates/verification_methods/requirement_aggregation_template.j2
```

**Sample Path Changes:**
```
OLD: data/testcase_results_container/data_sample.yml
NEW: schemas/tcms/samples/testcase_results_container_sample.yml
```

**Container Schema Path Changes:**
```
REMOVED: schemas/tcms/container/schema.json
REMOVED: schemas/tcms/test_results/container_schema.json
REMOVED: schemas/tcms/testcase_results_container/schema.json

CANONICAL: schemas/tcms/test-results-container.schema.v1.json
WORKING:   data/testcase_results_container/schema.json  (backward compatible)
```

#### Files Already Updated

The following files were updated as part of this reorganization:

1. **`crates/testcase-manager/tests/container_schema_validation_test.rs`**
   - Updated `SAMPLE_PATH` constant to use new sample location

2. **`scripts/validate-container-output.sh`**
   - Updated example path in usage message

#### Code Search Recommendations

Search your codebase for references to removed or relocated files:

```bash
# Search for old template paths
grep -r "schemas/tcms/verification_methods/.*\.j2" .
grep -r "schemas/tcms/verification_methods/.*\.adoc" .

# Search for old sample paths
grep -r "data/testcase_results_container/data_sample" .

# Search for removed container schemas
grep -r "schemas/tcms/container/schema.json" .
grep -r "schemas/tcms/test_results/container_schema.json" .
grep -r "schemas/tcms/testcase_results_container/schema.json" .
```

---

### Backward Compatibility

#### Container Schema Working Location

The working container schema at `data/testcase_results_container/schema.json` **remains unchanged** to maintain backward compatibility with existing tools (e.g., verifier binary).

**Use cases:**
- **Canonical schema:** `schemas/tcms/test-results-container.schema.v1.json` - For envelope-based validation
- **Working schema:** `data/testcase_results_container/schema.json` - For verifier and legacy tools

See `schemas/tcms/CONTAINER_SCHEMAS.md` for detailed guidance on which schema to use.

#### Verification Method Schemas

Verification method schemas in `schemas/tcms/verification_methods/` remain in their original locations. Only templates were moved.

---

### Migration Checklist for Developers

Use this checklist when updating code to work with the new structure:

- [ ] **Schema References**
  - [ ] Update references to use versioned schemas (`*.schema.v1.json`) in `schemas/tcms/`
  - [ ] Remove references to deleted container schemas in subdirectories
  - [ ] Use canonical container schema (`test-results-container.schema.v1.json`) or working schema (`data/testcase_results_container/schema.json`) as appropriate

- [ ] **Template References**
  - [ ] Update template paths from `schemas/tcms/verification_methods/` to `schemas/templates/verification_methods/`
  - [ ] Update Jinja2 template loader paths in code
  - [ ] Update AsciiDoc template references in documentation generators

- [ ] **Sample Data References**
  - [ ] Update sample file paths from `data/` to `schemas/tcms/samples/`
  - [ ] Update test fixtures that reference sample data
  - [ ] Update documentation examples that reference sample files

- [ ] **Documentation Updates**
  - [ ] Update README files with new paths
  - [ ] Update code comments referencing old locations
  - [ ] Update developer guides and onboarding documentation

- [ ] **Testing**
  - [ ] Run schema validation tests to ensure paths are correct
  - [ ] Run template rendering tests with new paths
  - [ ] Verify sample data loading in tests

---

### Example Migration Scenarios

#### Scenario 1: Loading a Template

**Before:**
```python
template_path = "schemas/tcms/verification_methods/analysis/template.j2"
```

**After:**
```python
template_path = "schemas/templates/verification_methods/analysis/template.j2"
```

---

#### Scenario 2: Validating Container Schema

**Before:**
```rust
const SCHEMA_PATH: &str = "schemas/tcms/testcase_results_container/schema.json";
const SAMPLE_PATH: &str = "../../data/testcase_results_container/data_sample.yml";
```

**After (using canonical schema):**
```rust
const SCHEMA_PATH: &str = "schemas/tcms/test-results-container.schema.v1.json";
const SAMPLE_PATH: &str = "../../schemas/tcms/samples/testcase_results_container_sample.yml";
```

**After (using working schema for backward compatibility):**
```rust
const SCHEMA_PATH: &str = "data/testcase_results_container/schema.json";
const SAMPLE_PATH: &str = "../../schemas/tcms/samples/testcase_results_container_sample.yml";
```

---

#### Scenario 3: Referencing Verification Method Schemas

**Before (still valid):**
```json
{
  "$ref": "schemas/tcms/verification_methods/analysis/schema.json"
}
```

**After (no change needed):**
```json
{
  "$ref": "schemas/tcms/verification_methods/analysis/schema.json"
}
```

**Note:** Verification method schemas themselves did not move. Only their templates moved to `schemas/templates/`.

---

## Deprecation Notices

### Deprecated: Root-Level Transitional Schemas

The following root-level schemas are **DEPRECATED** and will be removed in a future version:

#### Transitional Schemas (Optional Envelope Support)

| Deprecated Schema | Migration Target | Deprecation Date | Planned Removal |
|-------------------|------------------|------------------|-----------------|
| `test-case.schema.json` | `tcms/test-case.schema.v1.json` | 2026-03-31 | TBD |
| `container_config.schema.json` | `tcms/container-config.schema.v1.json` | 2026-03-31 | TBD |
| `execution-log.schema.json` | `tcms/test-execution.schema.v1.json` | 2026-03-31 | TBD |
| `verification-output.schema.json` | `tcms/test-result.schema.v1.json` | 2026-03-31 | TBD |
| `verification-result.schema.json` | `tcms/test-verification.schema.v1.json` | 2026-03-31 | TBD |

**Why deprecated:**
- These schemas have **optional** envelope fields (`type`, `schema`)
- Versioned v1 schemas in `schemas/tcms/` have **required** envelope fields (production standard)
- Transitional schemas were created during envelope migration period
- All new code should use versioned v1 schemas

**Migration path:**
1. Update schema references to point to `tcms/*.schema.v1.json` equivalents
2. Ensure your data includes required envelope fields: `type` and `schema`
3. Update validation code to expect envelope format
4. Test with versioned schemas before deprecation deadline

**Example migration:**

**Before (deprecated):**
```json
{
  "id": "TC-001",
  "description": "Test case without envelope"
}
```

**After (envelope-compliant v1):**
```json
{
  "type": "test_case",
  "schema": "tcms/test-case.schema.v1.json",
  "id": "TC-001",
  "description": "Test case with envelope"
}
```

---

### Deprecated: Legacy Container Schema Patterns

The following container schema patterns are **REMOVED** and will not be supported:

#### Removed Container Schemas

| Removed Schema | Reason | Removal Date | Migration Target |
|----------------|--------|--------------|------------------|
| `tcms/container/schema.json` | Too minimal (3 fields only) | 2026-03-31 | `tcms/container-config.schema.v1.json` or `tcms/test-results-container.schema.v1.json` |
| `tcms/test_results/container_schema.json` | Loose typing, no envelope | 2026-03-31 | `tcms/test-results-container.schema.v1.json` |
| `tcms/testcase_results_container/schema.json` | Different encoding, superseded | 2026-03-31 | `tcms/test-results-container.schema.v1.json` |

**Why removed:**
- Multiple competing container formats caused confusion
- Loose typing made validation ineffective
- No envelope support for versioning
- Superseded by comprehensive canonical schema

**Migration options:**

1. **For container configuration** (wrapping metadata):
   - Use: `schemas/tcms/container-config.schema.v1.json`
   - Purpose: Configuration for wrapping results with metadata

2. **For test results containers** (aggregating multiple test results):
   - Use: `schemas/tcms/test-results-container.schema.v1.json` (canonical, envelope-based)
   - Use: `data/testcase_results_container/schema.json` (working, backward compatible)

**See also:**
- `schemas/tcms/CONTAINER_SCHEMAS.md` - Detailed comparison and migration guidance
- `schemas/SCHEMA_AUDIT.md` - Complete schema analysis and recommendations

---

### Deprecated: Draft-04 JSON Schemas

Some verification method schemas still use **JSON Schema draft-04** (2013):

| Schema | Current Draft | Recommended |
|--------|---------------|-------------|
| Verification method schemas | draft-04 | Migrate to draft-07 |
| `test-case.schema.json` (root) | draft-04 | Use v1 equivalent (draft-07) |

**Why migrate to draft-07:**
- draft-07 (2018) is the current stable version
- Better validation features and error messages
- Consistent with all v1 envelope schemas
- Improved tooling support

**Timeline:** No immediate removal planned, but migration recommended for consistency.

---

### Deprecation Timeline

| Phase | Date | Actions |
|-------|------|---------|
| **Phase 1: Documentation** | 2026-03-31 | ✅ Add deprecation notices to docs and README files |
| **Phase 2: Warnings** | TBD | Add runtime warnings when deprecated schemas are used |
| **Phase 3: Migration Period** | TBD | 6-12 month migration period with support for both old and new |
| **Phase 4: Removal** | TBD | Remove deprecated schemas from repository |

**Support commitments:**
- Deprecated schemas will remain available during migration period
- Migration guides and tools will be provided
- Breaking changes will be communicated via release notes
- Backward compatibility maintained for working schema at `data/testcase_results_container/schema.json`

---

## Benefits of Reorganization

### Improved Organization
- **Clear separation of concerns**: Schemas, templates, and samples in dedicated directories
- **Easier navigation**: Logical grouping makes finding files intuitive
- **Reduced duplication**: Eliminated 3 redundant container schemas (4,130 lines)

### Simplified Maintenance
- **Single source of truth**: One canonical schema location for each type
- **Centralized templates**: All templates in one location for easier updates
- **Consistent structure**: Mirrored directory structure maintains logical organization

### Better Developer Experience
- **Clear purpose**: Each directory has a README explaining its role
- **Migration guidance**: Comprehensive documentation for updating code
- **Backward compatibility**: Working schemas remain for legacy tools

### Enhanced Validation
- **Canonical versioned schemas**: Production-ready v1 schemas with envelope support
- **Consistent validation**: Eliminate confusion from multiple schema versions
- **Clear deprecation path**: Well-documented migration from transitional schemas

---

## Related Documentation

- **`schemas/README.md`** - Main schemas directory documentation with decision tree
- **`schemas/tcms/README.md`** - TCMS schema organization and migration guide
- **`schemas/tcms/CONTAINER_SCHEMAS.md`** - Detailed container schema consolidation guide
- **`schemas/SCHEMA_AUDIT.md`** - Comprehensive audit of all schema files
- **`schemas/SCHEMA_QUICK_REFERENCE.md`** - Quick reference for schema selection
- **`schemas/REORGANIZATION.md`** - Technical details of template and sample relocation
- **`schemas/templates/README.md`** - Template directory documentation
- **`schemas/tcms/samples/README.md`** - Sample data directory documentation

---

## Questions or Issues?

If you encounter issues during migration or have questions about the new structure:

1. Check the relevant README files in each directory
2. Review the schema audit and consolidation guides
3. Search for similar usage patterns in recently updated files
4. Open an issue if documentation is unclear or missing

---

**Note:** This changelog documents the March 31, 2026 reorganization. Future schema changes will be added to this file with appropriate dates and version information.
