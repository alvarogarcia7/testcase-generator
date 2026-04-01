# Templates Directory

This directory contains all Jinja2 and AsciiDoc templates used for generating documentation and reports.

## Directory Structure

```
templates/
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
    ├── requirement_aggregation_template.adoc
    └── requirement_aggregation_template.j2
```

## Template Types

### Jinja2 Templates (*.j2)
Jinja2 templates for generating structured documentation in various formats.

### AsciiDoc Templates (*.adoc)
AsciiDoc templates for generating documentation in AsciiDoc format.

## Verification Methods Templates

Templates are organized by verification method type:
- **analysis**: Analysis verification method templates
- **common_criteria**: Common Criteria verification method templates
- **demonstration**: Demonstration verification method templates
- **high_assurance**: High Assurance verification method templates
- **inspection**: Inspection verification method templates
- **result**: Result verification method templates
- **test**: Test verification method templates

## Notes

All templates were previously located in `schemas/tcms/verification_methods/` subdirectories and have been consolidated here for better organization and separation of concerns from schema files.
