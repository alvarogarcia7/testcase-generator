# Schema Directory Reorganization

This document describes the reorganization of templates and sample data files in the schemas directory.

## Changes Made

### 1. Templates Relocated to `schemas/templates/`

All Jinja2 (`.j2`) and AsciiDoc (`.adoc`) template files have been moved from their original locations to a centralized templates directory with mirrored structure.

**New Location**: `schemas/templates/`

#### Files Moved:

From `schemas/tcms/verification_methods/analysis/`:
- `template.j2` → `schemas/templates/verification_methods/analysis/template.j2`
- `template_asciidoc.adoc` → `schemas/templates/verification_methods/analysis/template_asciidoc.adoc`

From `schemas/tcms/verification_methods/common_criteria/`:
- `template.j2` → `schemas/templates/verification_methods/common_criteria/template.j2`
- `template_asciidoc.adoc` → `schemas/templates/verification_methods/common_criteria/template_asciidoc.adoc`

From `schemas/tcms/verification_methods/demonstration/`:
- `template.j2` → `schemas/templates/verification_methods/demonstration/template.j2`
- `template_asciidoc.adoc` → `schemas/templates/verification_methods/demonstration/template_asciidoc.adoc`

From `schemas/tcms/verification_methods/high_assurance/`:
- `template.j2` → `schemas/templates/verification_methods/high_assurance/template.j2`
- `template_asciidoc.adoc` → `schemas/templates/verification_methods/high_assurance/template_asciidoc.adoc`

From `schemas/tcms/verification_methods/inspection/`:
- `template.j2` → `schemas/templates/verification_methods/inspection/template.j2`
- `template_asciidoc.adoc` → `schemas/templates/verification_methods/inspection/template_asciidoc.adoc`

From `schemas/tcms/verification_methods/result/`:
- `template.j2` → `schemas/templates/verification_methods/result/template.j2`
- `template_asciidoc.adoc` → `schemas/templates/verification_methods/result/template_asciidoc.adoc`

From `schemas/tcms/verification_methods/test/`:
- `template.j2` → `schemas/templates/verification_methods/test/template.j2`
- `template_asciidoc.adoc` → `schemas/templates/verification_methods/test/template_asciidoc.adoc`

From `schemas/tcms/verification_methods/`:
- `requirement_aggregation_template.j2` → `schemas/templates/verification_methods/requirement_aggregation_template.j2`
- `requirement_aggregation_template.adoc` → `schemas/templates/verification_methods/requirement_aggregation_template.adoc`

**Total**: 16 template files moved (8 Jinja2 + 8 AsciiDoc)

### 2. Sample Data Relocated to `schemas/tcms/samples/`

Sample data files have been moved to a centralized samples directory for better organization.

**New Location**: `schemas/tcms/samples/`

#### Files Moved:

From `data/testcase_results_container/`:
- `data_sample.yml` → `schemas/tcms/samples/testcase_results_container_sample.yml`

**Total**: 1 sample file moved

### 3. Schema Files Remain in Place

All JSON schema files remain in their canonical locations:

- `schemas/tcms/*.schema.v1.json` - Unchanged
- `schemas/tcms/verification_methods/*/schema.json` - Unchanged
- `data/testcase_results_container/schema.json` - Unchanged (this is a separate schema location)

## Code Updates

The following files were updated to reflect the new paths:

1. **`crates/testcase-manager/tests/container_schema_validation_test.rs`**
   - Updated `SAMPLE_PATH` constant from `../../data/testcase_results_container/data_sample.yml` to `../../schemas/tcms/samples/testcase_results_container_sample.yml`

2. **`scripts/validate-container-output.sh`**
   - Updated example path in usage message from `data/testcase_results_container/data_sample.yml` to `schemas/tcms/samples/testcase_results_container_sample.yml`

## New Directory Structure

```
schemas/
├── templates/
│   ├── README.md
│   └── verification_methods/
│       ├── analysis/
│       │   ├── template.j2
│       │   └── template_asciidoc.adoc
│       ├── common_criteria/
│       │   ├── template.j2
│       │   └── template_asciidoc.adoc
│       ├── demonstration/
│       │   ├── template.j2
│       │   └── template_asciidoc.adoc
│       ├── high_assurance/
│       │   ├── template.j2
│       │   └── template_asciidoc.adoc
│       ├── inspection/
│       │   ├── template.j2
│       │   └── template_asciidoc.adoc
│       ├── result/
│       │   ├── template.j2
│       │   └── template_asciidoc.adoc
│       ├── test/
│       │   ├── template.j2
│       │   └── template_asciidoc.adoc
│       ├── requirement_aggregation_template.adoc
│       └── requirement_aggregation_template.j2
└── tcms/
    ├── samples/
    │   ├── README.md
    │   └── testcase_results_container_sample.yml
    └── verification_methods/
        ├── analysis/
        │   └── schema.json
        ├── common_criteria/
        │   └── schema.json
        ├── demonstration/
        │   └── schema.json
        ├── high_assurance/
        │   └── schema.json
        ├── inspection/
        │   └── schema.json
        ├── result/
        │   └── schema.json
        └── test/
            └── schema.json
```

## Benefits

1. **Separation of Concerns**: Templates are now clearly separated from schema definitions
2. **Easier Maintenance**: All templates in one location makes updates simpler
3. **Better Organization**: Sample data is consolidated in a dedicated directory
4. **Consistent Structure**: Mirrored directory structure maintains logical organization
5. **Clear Purpose**: README files document the purpose of each directory

## Migration Notes

- All references to old template and sample paths should be updated to use the new locations
- The directory structure maintains the same hierarchy for easy navigation
- No schema files were moved to ensure backward compatibility with validation tools
