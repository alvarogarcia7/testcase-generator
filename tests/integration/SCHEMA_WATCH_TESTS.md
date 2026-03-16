# Schema Watch Tests

This directory contains integration tests for the `validate-yaml --watch` schema watching functionality.

## Test Files

### test_validate_yaml_schema_watch_e2e.sh

Tests basic schema watching functionality:
- Monitors both YAML files and schema files
- Detects schema file changes
- Re-validates all YAML files when schema changes
- Reports schema file count correctly
- Handles schema modifications that make YAML files invalid
- Handles schema modifications that relax constraints
- Gracefully handles invalid schema syntax

**Usage:**
```bash
./tests/integration/test_validate_yaml_schema_watch_e2e.sh
```

**Test Scenarios:**
1. Modify schema to add new required field
2. Fix YAML files to match new schema
3. Modify schema to relax constraints
4. Test with invalid schema syntax

### test_validate_yaml_transitive_schema_watch_e2e.sh

Tests transitive schema dependency watching:
- Discovers schemas referenced through `$ref`
- Monitors multi-level schema hierarchies (A → B → C)
- Detects changes to any schema in the dependency chain
- Handles circular schema references without infinite loops
- Reports correct schema file count including transitive dependencies

**Usage:**
```bash
./tests/integration/test_validate_yaml_transitive_schema_watch_e2e.sh
```

**Test Scenarios:**
1. Create 3-level schema hierarchy (main → person → base)
2. Modify deepest schema (base) and verify detection
3. Modify middle schema (person) and verify detection
4. Update YAML to violate constraint
5. Relax constraint via schema change
6. Test circular schema references

### test_validate_yaml_watch_e2e.sh

Tests basic YAML file watching (existing test):
- Monitors YAML files for changes
- Re-validates changed files
- Runs full validation when changed files pass
- Reports validation results

## Unit Tests

### tests/validate_yaml_schema_discovery_test.rs

Unit tests for schema discovery logic:
- Single schema (no dependencies)
- External `$ref` references
- Internal `$ref` references (should be ignored)
- Transitive dependencies
- Circular references
- Multiple references to same schema
- Nested objects with `$ref`
- `$ref` in arrays
- Missing referenced files
- Relative path resolution
- Complex multi-level hierarchies
- Fragment extraction from `$ref` URLs

## Running Tests

### Run all schema watch tests:
```bash
# Run basic schema watch test
./tests/integration/test_validate_yaml_schema_watch_e2e.sh

# Run transitive schema watch test
./tests/integration/test_validate_yaml_transitive_schema_watch_e2e.sh

# Run YAML watch test
./tests/integration/test_validate_yaml_watch_e2e.sh
```

### Run with --no-remove to inspect test artifacts:
```bash
./tests/integration/test_validate_yaml_schema_watch_e2e.sh --no-remove
./tests/integration/test_validate_yaml_transitive_schema_watch_e2e.sh --no-remove
```

### Run unit tests:
```bash
cargo test validate_yaml_schema_discovery
```

## Prerequisites

- Rust and Cargo installed
- Built binary: `cargo build`
- Non-Windows platform (watch mode not supported on Windows)
- Temporary directory support

## Test Architecture

### Integration Tests (E2E)
- Create temporary directories with test schemas and YAML files
- Start `validate-yaml --watch` in background
- Modify files and verify detection
- Check watch process logs for expected output
- Clean up background processes and temporary files

### Unit Tests
- Test schema discovery functions in isolation
- Use temporary directories for file creation
- Verify correct behavior with various schema structures
- No process spawning required

## Expected Behavior

### Schema File Changes
When a schema file changes, the watch mode should:
1. Detect the change within debounce window (300ms)
2. Report "Schema file(s) modified" in output
3. Show "Schema changed - re-validating all YAML files"
4. Re-validate ALL YAML files (not just changed ones)
5. Display validation results for all files
6. Return to watching state

### Transitive Schema Changes
When a referenced schema changes:
1. Detect the change even if it's multiple levels deep
2. Re-validate all YAML files
3. Apply new constraints from the modified schema
4. Handle circular references without hanging

### YAML File Changes
When only YAML files change (existing behavior):
1. Validate only the changed files first
2. If all pass, run full validation
3. Report results for changed files only

## Debugging

If tests fail, use `--no-remove` flag to inspect:
- Temporary directory contents
- Watch process logs (`watch_output.log`)
- Schema files used in tests
- YAML files used in tests

Example:
```bash
./tests/integration/test_validate_yaml_schema_watch_e2e.sh --no-remove
# Check the temporary directory path in output
cat /tmp/tmp.XXXXXX/watch_output.log
```

## Notes

- All tests automatically skip on Windows platforms
- Tests use debounce delays (sleep 2-3 seconds) to ensure watch mode detects changes
- Background watch processes are automatically cleaned up via trap handlers
- Tests create isolated temporary directories to avoid conflicts
