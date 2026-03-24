# Auto-Schema Resolution Implementation

## Overview
This implementation adds automatic schema resolution to `validate-yaml` and `validate-json` binaries, making the `--schema` argument optional. When not provided, the tools automatically read the `schema` field from the input file and resolve it to a local schema path.

## Components Implemented

### 1. Core Module: `src/envelope.rs`
**New module** that provides schema auto-resolution functionality:
- `resolve_schema_from_payload()` - Main function that reads a file (YAML or JSON), extracts the `schema` field, and resolves it to a local path
- `resolve_schema_uri()` - Helper that maps schema URIs to local file paths
- Supports both YAML and JSON input formats
- Comprehensive unit tests included

### 2. Binary Updates: `src/bin/validate-yaml.rs`
**Modified** to support auto-resolution:
- `--schema` argument is now **optional** (was required)
- New `--schemas-root` argument (default: `schemas/`) for specifying schema directory
- `resolve_schema_for_file()` function determines whether to use explicit schema or auto-resolve
- `validate_files()` updated to handle per-file schema resolution
- Watch mode updated to work with auto-resolved schemas
- Enhanced output shows resolved schema path for each file

### 3. Binary Updates: `src/bin/validate-json.rs`
**Modified** to support auto-resolution:
- Schema file argument is now **optional** (was required positional argument)
- New `--schemas-root` argument (default: `schemas/`) for specifying schema directory
- Auto-resolves schema from JSON file when not explicitly provided
- Logs resolved schema path at info level

### 4. Verifier Updates: `src/bin/verifier.rs` and `src/verification.rs`
**Modified** to check for type/schema fields:
- Added warnings when `type` or `schema` fields are missing from execution logs
- Logs schema field value when present (debug level)
- Checks both in `parse_json_log_content()` and `parse_json_log_content_with_test_case_id()`
- Non-breaking: warnings only, does not fail validation

### 5. Integration Tests: `tests/integration/test_auto_schema_validation_e2e.sh`
**New comprehensive test suite** covering:
- ✓ validate-yaml with auto-resolution (test_case schema)
- ✓ validate-json with auto-resolution (test_execution schema)
- ✓ validate-yaml with explicit --schema override
- ✓ validate-json with explicit schema override
- ✓ validate-yaml with missing schema field (error case)
- ✓ validate-json with missing schema field (error case)
- ✓ validate-yaml with unknown schema URI (error case)
- ✓ validate-json with unknown schema URI (error case)
- ✓ validate-yaml with multiple files (mixed auto-resolution)
- ✓ validate-yaml with custom schemas-root
- ✓ verifier warns on missing schema field in execution logs
- ✓ verifier logs schema field when present

### 6. Build System: `Makefile`
**Modified** to add new test targets:
- `test-e2e-auto-schema` - Run auto-schema validation tests
- Integrated into main `test-e2e` target

## Usage Examples

### validate-yaml with auto-resolution
```bash
# Auto-resolve schema from 'schema' field in YAML
validate-yaml testcases/TC_001.yaml

# With custom schemas root
validate-yaml testcases/TC_001.yaml --schemas-root ./my-schemas

# Explicit schema (skip auto-resolution)
validate-yaml testcases/TC_001.yaml --schema schemas/tcms/test-case.schema.v1.json
```

### validate-json with auto-resolution
```bash
# Auto-resolve schema from 'schema' field in JSON
validate-json logs/execution_log.json

# With custom schemas root
validate-json logs/execution_log.json --schemas-root ./my-schemas

# Explicit schema (skip auto-resolution)
validate-json logs/execution_log.json schemas/tcms/test-execution.schema.v1.json
```

### Document Format
Documents must include a `schema` field for auto-resolution:

**YAML:**
```yaml
type: test_case
schema: tcms/test-case.schema.v1.json
id: TC_001
description: Test case
```

**JSON:**
```json
{
  "type": "test_execution",
  "schema": "tcms/test-execution.schema.v1.json",
  "test_sequence": 1,
  "step": 1
}
```

## Schema Resolution Logic

1. **Explicit Schema Override**: If `--schema` is provided, use it (no auto-resolution)
2. **Auto-Resolution**: If no `--schema`, read the input file and extract `schema` field
3. **Path Mapping**: Join `schemas_root` + `schema` field value
4. **Validation**: Verify the resolved schema file exists
5. **Error Handling**: Clear error messages for missing fields or files

## Error Cases Handled

1. **Missing schema field**: `Missing 'schema' field in file: <path>`
2. **Schema file not found**: `Schema file not found: <path> (resolved from URI: '<uri>')`
3. **Invalid file format**: Parse errors when reading YAML/JSON
4. **Invalid schema**: JSON schema compilation errors

## Backward Compatibility

✅ **Fully backward compatible**:
- Explicit `--schema` argument still works exactly as before
- Existing scripts and CI/CD pipelines unaffected
- New auto-resolution is opt-in (only when `--schema` is omitted)
- Verifier warnings are informational only, don't break existing functionality

## Testing

Run the test suite:
```bash
make test-e2e-auto-schema
```

Or as part of full integration tests:
```bash
make test-e2e
```

## Files Modified

- `src/envelope.rs` (NEW)
- `src/lib.rs` (added envelope module export)
- `src/bin/validate-yaml.rs` (modified)
- `src/bin/validate-json.rs` (modified)
- `src/verification.rs` (modified)
- `tests/integration/test_auto_schema_validation_e2e.sh` (NEW)
- `Makefile` (modified)

## Implementation Status

✅ All requested functionality implemented:
- ✅ Auto-schema resolution in validate-yaml
- ✅ Auto-schema resolution in validate-json
- ✅ Shared resolve_schema_from_payload function
- ✅ Optional --schema argument (explicit override)
- ✅ --schemas-root argument with default
- ✅ Verifier type/schema field checking with warnings
- ✅ Comprehensive integration tests
- ✅ Makefile target: test-e2e-auto-schema
- ✅ Error handling for missing fields and unknown URIs
- ✅ Unit tests in envelope module
