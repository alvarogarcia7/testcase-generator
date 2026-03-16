# Schema Watch Testing Documentation

This document describes the comprehensive test suite for the `validate-yaml --watch` schema watching functionality.

## Overview

The schema watching feature allows `validate-yaml` to monitor not only YAML input files but also JSON schema files (including transitive dependencies). When any schema file changes, all YAML files are re-validated to ensure they still comply with the updated schema.

## Test Coverage

### 1. Integration Tests (E2E)

Located in `tests/integration/`:

#### test_validate_yaml_schema_watch_e2e.sh
**Purpose:** Test basic schema file watching

**Coverage:**
- Schema file monitoring is enabled
- Schema file count is reported correctly
- Schema changes trigger re-validation of all YAML files
- Schema changes that make YAML invalid are detected
- Schema changes that relax constraints work correctly
- Invalid schema syntax is handled gracefully
- Watch process doesn't crash on schema errors

**Test Cases:**
1. **Initial Setup:** Verify watch mode starts and reports schema count
2. **Add Required Field:** Modify schema to require new field, verify YAML files fail
3. **Fix YAML Files:** Update YAML to match schema, verify they pass
4. **Relax Constraints:** Remove required field, verify YAML still passes
5. **Invalid Schema:** Test with malformed schema, verify graceful handling

#### test_validate_yaml_transitive_schema_watch_e2e.sh
**Purpose:** Test transitive schema dependency watching

**Coverage:**
- Multi-level schema references are discovered (A → B → C)
- All schemas in dependency chain are monitored
- Changes to any schema in chain trigger re-validation
- Circular references don't cause infinite loops
- Schema count includes all transitive dependencies
- Relative path resolution works correctly

**Test Cases:**
1. **3-Level Hierarchy:** Create main → person → base schema chain
2. **Modify Level 2:** Change base schema, verify detection
3. **Modify Level 1:** Change person schema, verify detection
4. **YAML Validation:** Update YAML to fail constraint
5. **Relax Constraint:** Fix schema, verify YAML passes
6. **Circular Reference:** Create circular refs, verify no hang

#### test_validate_yaml_watch_e2e.sh (Existing)
**Purpose:** Test YAML file watching

**Coverage:**
- YAML file changes trigger re-validation
- Only changed files are validated initially
- Full validation runs if changed files pass
- Multiple files can change simultaneously
- Invalid YAML is detected and reported

### 2. Unit Tests

Located in `tests/validate_yaml_schema_discovery_test.rs`:

**Coverage:**
- Schema discovery with no external dependencies
- External `$ref` detection
- Internal `$ref` handling (should be ignored)
- Transitive dependency discovery
- Circular reference handling
- Multiple refs to same schema (deduplication)
- Nested object structures with `$ref`
- Array contexts with `$ref`
- Missing referenced files (graceful handling)
- Relative path resolution
- Fragment extraction from `$ref` URLs
- Complex multi-level hierarchies

**Test Functions:**
1. `test_schema_discovery_single_schema` - No dependencies
2. `test_schema_discovery_with_external_ref` - Simple external ref
3. `test_schema_discovery_with_internal_ref_only` - Internal only
4. `test_schema_discovery_transitive_dependencies` - Multi-level
5. `test_schema_discovery_circular_reference` - Circular refs
6. `test_schema_discovery_multiple_refs_same_schema` - Deduplication
7. `test_schema_discovery_nested_objects` - Deep nesting
8. `test_schema_discovery_in_arrays` - Array contexts
9. `test_schema_discovery_with_missing_referenced_file` - Error handling
10. `test_schema_discovery_relative_path_resolution` - Path handling
11. `test_schema_discovery_complex_multi_level` - Complex hierarchies
12. `test_find_external_refs_ignores_internal` - Ref filtering
13. `test_schema_with_fragment_in_external_ref` - Fragment handling

## Test Execution

### Run All Tests

```bash
# Build first
cargo build

# Run unit tests
cargo test validate_yaml_schema_discovery

# Run integration tests
./tests/integration/test_validate_yaml_schema_watch_e2e.sh
./tests/integration/test_validate_yaml_transitive_schema_watch_e2e.sh
./tests/integration/test_validate_yaml_watch_e2e.sh
```

### Debug Mode

Keep temporary files for inspection:
```bash
./tests/integration/test_validate_yaml_schema_watch_e2e.sh --no-remove
./tests/integration/test_validate_yaml_transitive_schema_watch_e2e.sh --no-remove
```

## Implementation Details Tested

### Schema Discovery Algorithm

The tests verify the following implementation aspects:

1. **Discovery Process:**
   - Start with main schema file
   - Parse JSON to find all `$ref` values
   - Extract file paths from external refs (not starting with `#`)
   - Recursively process referenced schemas
   - Track processed schemas to avoid duplicates/loops

2. **Path Resolution:**
   - Resolve relative paths from current schema's directory
   - Handle fragments in `$ref` URLs (e.g., `schema.json#/definitions/type`)
   - Skip non-existent files gracefully

3. **Watching Behavior:**
   - Watch all discovered schemas with file system watcher
   - Detect modifications to any schema file
   - Trigger full re-validation on schema changes
   - Maintain separate tracking for YAML vs schema changes

### Edge Cases Covered

1. **No Dependencies:** Single schema file with no external refs
2. **Internal Refs Only:** `$ref` pointing to `#/definitions/...`
3. **Missing Files:** External `$ref` to non-existent file
4. **Circular Refs:** Schema A refs B, B refs A
5. **Self-Reference:** Schema refs itself
6. **Deep Nesting:** `$ref` buried in nested objects
7. **Array Contexts:** `$ref` in `items`, `oneOf`, `anyOf`, etc.
8. **Multiple Refs:** Same schema referenced multiple times
9. **Path Types:** Relative paths, subdirectories, fragments

## Validation Criteria

### Integration Tests Pass When:
- Watch process starts successfully
- Correct number of schema files reported
- Schema changes detected within 3 seconds
- All YAML files re-validated on schema change
- Process continues running after errors
- Background process cleaned up properly

### Unit Tests Pass When:
- Expected schema files discovered
- Correct number of schemas returned
- Circular refs handled without infinite loops
- Missing files don't cause crashes
- Internal refs ignored
- External refs processed

## Platform Notes

- **Windows:** Watch mode not supported (tests skip automatically)
- **macOS/Linux:** Full support with `notify` crate
- **Debouncing:** 300ms delay to batch rapid changes

## Future Enhancements

Potential areas for additional testing:
- HTTP/HTTPS schema references (if supported)
- Very large schema files (performance)
- Rapid successive schema changes (stress test)
- Schema validation errors (malformed JSON)
- Permission errors (unreadable schema files)
- Symlink handling
- Network file systems

## Troubleshooting

### Test Failures

1. **Process Dies:** Check watch_output.log in temp directory
2. **Changes Not Detected:** Increase sleep duration in test
3. **Wrong Schema Count:** Verify expected dependencies
4. **Background Process Hangs:** Kill manually: `pkill -f validate-yaml`

### Common Issues

- **Build Required:** Run `cargo build` before integration tests
- **Platform Check:** Tests auto-skip on Windows
- **Temp Directory:** Use `--no-remove` to debug temp files
- **Timing Issues:** Increase sleep delays if CI is slow

## See Also

- `tests/integration/SCHEMA_WATCH_TESTS.md` - Detailed test descriptions
- `src/bin/validate-yaml.rs` - Implementation
- Main README for usage examples
