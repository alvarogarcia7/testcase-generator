# Test Case Tagging and Filtering System Implementation

## Overview

Implemented a comprehensive test case tagging and filtering system that supports tag-based test selection, tag inheritance, and dynamic tag evaluation for flexible test suite composition.

## Components Implemented

### 1. Core Tags Module (`src/tags.rs`)

**TagExpression**
- Recursive expression parser supporting AND (`&&`), OR (`||`), NOT (`!`), and parentheses
- Boolean expression evaluation against tag sets
- Complex filtering logic: `(smoke || regression) && !slow`

**TagFilter**
- Include tags: Select tests with any of the specified tags
- Exclude tags: Remove tests with any of the specified tags
- Tag expressions: Apply complex boolean logic
- Combines all three filter types with proper precedence

**TagInheritance**
- Helper functions to compute effective tags
- Test sequences inherit tags from parent test cases
- Aggregates tags from all levels

**DynamicTagEvaluator**
- Automatically computes tags based on test case properties
- Built-in rules:
  - `multi-sequence`: Test has multiple sequences
  - `single-sequence`: Test has exactly one sequence
  - `has-manual-steps`: Test contains manual steps
  - `automated-only`: Test has no manual steps
  - `has-initial-conditions`: Test defines initial conditions
- Extensible: Can add custom dynamic tag rules

**Utility Functions**
- `filter_test_cases()`: Filter a collection of test cases
- `filter_sequences_in_test_case()`: Filter sequences within a test case

### 2. Model Updates (`src/models.rs`)

**TestCase**
- Added `tags: Vec<String>` field
- Serialized as optional (empty list omitted)

**TestSequence**
- Added `tags: Vec<String>` field
- Serialized as optional (empty list omitted)
- Inherits tags from parent test case

### 3. Orchestrator Extensions (`src/orchestrator.rs`)

**New Methods**
- `select_test_cases_with_tags()`: Load and filter specific test cases
- `select_all_test_cases_with_tags()`: Load and filter all test cases
- `list_all_tags()`: Get all unique tags across test suite
- `list_tags_for_test_case()`: Get all tags for a specific test case
- `find_test_cases_by_tag()`: Find test cases with a specific tag

### 4. CLI Enhancements (`src/bin/test-orchestrator.rs`)

**Run Command Options**
- `--include-tags`: Comma-separated list of tags to include
- `--exclude-tags`: Comma-separated list of tags to exclude
- `--tag-expr`: Boolean expression for complex filtering
- `--dynamic-tags`: Enable dynamic tag evaluation

**RunAll Command Options**
- Same tag filtering options as Run command

**New Commands**
- `list-tags`: Display all available tags with counts
- `show-tags <test_case_id>`: Show tags for a specific test case
- `find-by-tag <tag>`: Find test cases with a tag

### 5. Library Exports (`src/lib.rs`)

Exported public API:
- `TagExpression`
- `TagFilter`
- `TagInheritance`
- `DynamicTagEvaluator`
- `filter_test_cases`
- `filter_sequences_in_test_case`

## Features

### Tag-Based Selection

```bash
# Include specific tags
test-orchestrator run-all --include-tags smoke,fast

# Exclude specific tags
test-orchestrator run-all --exclude-tags slow,manual

# Complex expressions
test-orchestrator run-all --tag-expr "(smoke || regression) && !slow"
```

### Tag Inheritance

Test sequences automatically inherit tags from their parent test case:

```yaml
tags: [smoke, authentication]  # Test case level
test_sequences:
  - id: 1
    tags: [fast, positive]     # Sequence level
    # Effective tags: [smoke, authentication, fast, positive]
```

### Dynamic Tag Evaluation

```bash
# Enable dynamic tags
test-orchestrator run-all --dynamic-tags --include-tags automated-only

# Dynamic tags are computed based on test properties
# No need to manually add these tags to YAML files
```

### Tag Management

```bash
# List all available tags
test-orchestrator list-tags

# Show tags for a specific test case
test-orchestrator show-tags TC001

# Find test cases by tag
test-orchestrator find-by-tag smoke
```

## Usage Examples

### Example 1: Run Smoke Tests

```bash
test-orchestrator run-all --include-tags smoke
# Or using make:
make test-filter-smoke
```

### Example 2: Run Fast Automated Tests

```bash
test-orchestrator run-all \
  --tag-expr "fast && automated-only" \
  --dynamic-tags
```

### Example 3: Run High Priority Tests (Excluding Flaky)

```bash
test-orchestrator run-all \
  --include-tags priority-high \
  --exclude-tags flaky
```

### Example 4: Complex Tag Expression

```bash
test-orchestrator run-all \
  --tag-expr "(smoke || regression) && !slow && !broken" \
  --dynamic-tags
```

## Test Coverage

The `src/tags.rs` module includes comprehensive unit tests:

- `test_tag_expression_single_tag`: Single tag matching
- `test_tag_expression_and`: AND operator
- `test_tag_expression_or`: OR operator
- `test_tag_expression_not`: NOT operator
- `test_tag_expression_complex`: Complex nested expressions
- `test_tag_filter_include`: Include tag filtering
- `test_tag_filter_exclude`: Exclude tag filtering
- `test_tag_filter_expression`: Expression-based filtering
- `test_tag_inheritance`: Tag inheritance from test case to sequence
- `test_dynamic_tag_evaluator`: Dynamic tag computation
- `test_filter_test_cases`: Full filtering pipeline

## Documentation

### Main Documentation
- `docs/TAG_SYSTEM.md`: Comprehensive user guide
  - Feature overview
  - Tag definition examples
  - Filtering syntax and usage
  - CLI command reference
  - Best practices and conventions
  - Troubleshooting guide

### Example Files
- `testcases/example_tagged_test.yml`: Example test case with tags
- `testcases/gsma_4.4.2.2_TC.yml`: Updated with example tags

## Architecture Decisions

### 1. Tag Storage Format
- **Decision**: Store tags as `Vec<String>` in YAML
- **Rationale**: Simple, readable, easy to edit manually
- **Alternative**: Could use HashSet but YAML serialization more complex

### 2. Expression Parsing
- **Decision**: Custom recursive descent parser
- **Rationale**: No external dependencies, full control, simple grammar
- **Alternative**: Could use pest or nom, but adds complexity

### 3. Filter Precedence
- **Decision**: Exclude → Include → Expression
- **Rationale**: Exclude first (most restrictive), then include, then refine
- **Alternative**: Could be configurable but adds complexity

### 4. Tag Inheritance Model
- **Decision**: Sequences inherit all parent test case tags
- **Rationale**: Natural hierarchy, intuitive behavior
- **Alternative**: Could require explicit inheritance but less convenient

### 5. Dynamic Tag Implementation
- **Decision**: Opt-in with `--dynamic-tags` flag
- **Rationale**: Predictable behavior, no surprises
- **Alternative**: Always enabled, but changes test selection unexpectedly

## Integration Points

### With Existing Systems

1. **Test Case Storage**: Tags loaded automatically during YAML parsing
2. **Test Orchestrator**: Filtering applied before test execution
3. **Test Execution**: No changes to execution logic
4. **Reporting**: Tags available for custom reports (future enhancement)

### Backward Compatibility

- **Fully backward compatible**: Tags are optional
- Empty or missing `tags` field treated as empty list
- Existing test cases work without modification
- `skip_serializing_if = "Vec::is_empty"` keeps YAML clean

## Performance Characteristics

- **Tag Evaluation**: O(n) where n is number of test cases
- **Expression Evaluation**: O(t) where t is number of tags per test
- **Memory**: Minimal overhead, tags stored as strings
- **Filtering**: Single pass through test collection

## Future Enhancements

Potential additions for future versions:

1. **Tag Metadata**: Descriptions, categories, documentation
2. **Tag Validation**: Warn about typos, suggest corrections
3. **Tag Aliases**: Multiple names for the same tag
4. **Tag Inheritance Control**: Override inherited tags
5. **Custom Dynamic Rules**: Load from configuration file
6. **Tag-Based Reports**: Group results by tags
7. **Tag Statistics**: Usage analytics, coverage metrics
8. **Tag Autocomplete**: Shell completion for tag names

## Make Goals

Added comprehensive make goals for testing and using the tagging system:

### Test Goals
- `test-tagged-example`: Validates and tests the example tagged test file
- `test-filter-smoke`: Run smoke tests
- `test-filter-fast`: Run fast tests
- `test-filter-priority-high`: Run priority-high tests
- `test-filter-automated`: Run automated-only tests
- `test-filter-no-slow`: Run tests excluding slow tests
- `test-filter-expression`: Run tests with complex expressions
- `test-filter-all`: Run all tag filter tests

The `test-tagged-example` goal is included in the main `make test` command.

## Files Modified/Created

### Created
- `src/tags.rs` (516 lines): Core tagging system
- `docs/TAG_SYSTEM.md` (403 lines): User documentation
- `testcases/example_tagged_test.yml`: Example file
- `IMPLEMENTATION_TAG_SYSTEM.md`: This file

### Modified
- `src/models.rs`: Added tags fields to TestCase and TestSequence
- `src/lib.rs`: Exported tags module and utilities
- `src/orchestrator.rs`: Added tag filtering methods
- `src/bin/test-orchestrator.rs`: Added CLI options and commands
- `testcases/gsma_4.4.2.2_TC.yml`: Added example tags
- `Makefile`: Added test goals for tagged example and tag filtering
- `AGENTS.md`: Updated with new make goals

## Summary

The test case tagging and filtering system is fully implemented and ready for use. It provides:

1. ✅ **Tag Definition**: Add tags to test cases and sequences in YAML
2. ✅ **Tag Inheritance**: Sequences inherit parent test case tags
3. ✅ **Tag Filtering**: Include/exclude tags and complex expressions
4. ✅ **Dynamic Tags**: Automatic tag computation based on properties
5. ✅ **Tag Management**: CLI commands to list, search, and view tags
6. ✅ **Comprehensive Tests**: Full unit test coverage
7. ✅ **Documentation**: Complete user guide and examples
8. ✅ **Backward Compatible**: No breaking changes to existing code

The system enables flexible test suite composition and makes it easy to run specific subsets of tests based on metadata tags.
