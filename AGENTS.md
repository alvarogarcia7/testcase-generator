# AGENTS.md - Guide for Development

This document provides guidance for developing and maintaining the TCMS Schemas validator system.

## Project Overview

The TCMS (Test Case Management System) Schemas project provides:
- JSON schema definitions for test plans and test cases
- YAML sample data demonstrating schema compliance
- Automated validation tooling for schema enforcement

## Architecture

### Components

1. **Schema Definitions** (`schemas/`)
   - `test-plan/test-plan.schema.json` - Schema for test plan documents
   - `test-case/test-case.schema.json` - Schema for test case documents

2. **Sample Data** (`samples/`)
   - `test-plan/data.yml` - Example test plan
   - `test-case/gsma_4.4.2.2_TC.yml` - Example test case
   - `test-case/gsma_4.4.2.3_TC.yml` - Example test case

3. **Validation Tools** (root directory)
   - `validator.py` - Main validator script
   - `requirements.txt` - Python dependencies
   - `pyproject.toml` - Project metadata and configuration

4. **CI/CD**
   - `.gitlab-ci.yml` - GitLab pipeline configuration

## Development Tasks

### Adding New Schema Types

When adding a new schema type:

1. Create schema file: `schemas/<type>/<type>.schema.json`
2. Add sample data: `samples/<type>/example.yml`
3. Update `.gitlab-ci.yml` to validate the new samples
4. Test with: `python validator.py samples/<type>/example.yml`

### Modifying Existing Schemas

1. Update the schema file in `schemas/<category>/`
2. Ensure all sample files still validate
3. Run full validation suite before committing

### Running Validation

```bash
# Validate a single file with auto-detected schema
python validator.py samples/test-case/gsma_4.4.2.2_TC.yml

# Validate with explicit schema path
python validator.py samples/test-plan/data.yml schemas/test-plan/test-plan.schema.json
```

### Installation

```bash
# Install dependencies
pip install -r requirements.txt

# Or use the project entry point
pip install -e .
tcms-validate samples/test-case/gsma_4.4.2.2_TC.yml
```

## Schema Structure

### Test Plan Schema
Required fields:
- `date` (string) - Date of test plan
- `product` (string) - Product name
- `description` (string) - Plan description

### Test Case Schema
Required fields:
- `requirement` (string) - Associated requirement
- `item` (string) - Item identifier
- `tc` (string) - Test case reference
- `id` (string) - Unique identifier
- `description` (string) - Test case description
- `general_initial_conditions` (object) - General setup conditions
- `initial_conditions` (object) - Specific setup conditions
- `test_sequences` (array) - Array of test sequences

Each test sequence contains:
- `id`, `name`, `description` - Metadata
- `initial_conditions` - Sequence-specific conditions
- `steps` (array) - Test steps with commands and expected results

## CI/CD Pipeline

The GitLab pipeline automatically:
1. Installs Python 3.10
2. Installs dependencies from `requirements.txt`
3. Validates all sample YAML files
4. Reports success/failure with clear messaging

Pipeline runs on:
- All branch pushes
- All merge requests

## Common Tasks

### Validate All Samples
The `.gitlab-ci.yml` file provides a template. Expand the `script` section to validate additional samples as they're added.

### Add a New Sample File
1. Create the YAML file in the appropriate `samples/<type>/` directory
2. Add validation command to `.gitlab-ci.yml` under the `validate_schemas` job
3. Commit both files

### Debug Validation Failures
Run the validator locally and check:
- YAML syntax errors (reported as "Error parsing YAML")
- Schema violations (reported with path information)
- Missing required fields (reported as "Validation Error")

## Dependencies

- **PyYAML** (6.0.1) - YAML parsing
- **jsonschema** (4.20.0) - JSON schema validation

No other dependencies are required.

## Testing

To test the validator with modified schemas:

```bash
# Install in development mode
pip install -e .

# Run validation
python validator.py samples/test-case/gsma_4.4.2.2_TC.yml
python validator.py samples/test-plan/data.yml
```

## File Structure Reference

```
tcms-schemas/
├── .gitlab-ci.yml           # CI/CD pipeline
├── README.md                # Project overview
├── AGENTS.md                # This file - Development guide
├── requirements.txt         # Python dependencies
├── pyproject.toml          # Project configuration
├── validator.py            # Validation script
├── schemas/                # Schema definitions
│   ├── test-case/
│   │   └── test-case.schema.json
│   └── test-plan/
│       └── test-plan.schema.json
└── samples/                # Sample YAML files
    ├── test-case/
    │   ├── gsma_4.4.2.2_TC.yml
    │   └── gsma_4.4.2.3_TC.yml
    └── test-plan/
        └── data.yml
```

## Maintenance

- Keep dependencies updated to latest stable versions
- Review schema changes before merging PRs
- Ensure new samples pass validation before merging
- Document schema changes in commit messages

## References

- [JSON Schema Documentation](https://json-schema.org/)
- [PyYAML Documentation](https://pyyaml.org/)
- [jsonschema Library](https://python-jsonschema.readthedocs.io/)
