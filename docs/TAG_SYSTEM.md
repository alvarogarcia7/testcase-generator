# Test Case Tagging and Filtering System

## Overview

The Test Case Tagging and Filtering System provides a flexible and powerful way to organize, select, and filter test cases based on metadata tags. This system supports tag-based test selection, tag inheritance from test cases to sequences, and dynamic tag evaluation for flexible test suite composition.

## Features

- **Tag-Based Selection**: Filter test cases using simple tag lists or complex expressions
- **Tag Inheritance**: Test sequences inherit tags from their parent test case
- **Dynamic Tag Evaluation**: Automatically compute tags based on test case properties
- **Complex Filtering**: Use boolean logic (AND, OR, NOT) to create sophisticated filters
- **Tag Management**: CLI commands to list, search, and manage tags

## Defining Tags

### Test Case Tags

Add tags directly to test cases in YAML files:

```yaml
requirement: "REQ001"
item: 1
tc: 1
id: "TC001"
description: "User authentication test"
tags:
  - smoke
  - authentication
  - priority-high
  - fast
test_sequences: []
```

### Test Sequence Tags

Add tags to individual test sequences:

```yaml
test_sequences:
  - id: 1
    name: "Happy Path Test"
    description: "Test successful authentication"
    tags:
      - positive
      - fast
    steps: []
  - id: 2
    name: "Error Handling Test"
    description: "Test authentication errors"
    tags:
      - negative
      - edge-case
    steps: []
```

### Tag Inheritance

Sequences automatically inherit tags from their parent test case:

- Test case has tags: `[smoke, authentication]`
- Sequence has tags: `[fast, positive]`
- Effective sequence tags: `[smoke, authentication, fast, positive]`

## Dynamic Tags

Dynamic tags are automatically evaluated based on test case properties:

### Built-in Dynamic Tags

- `multi-sequence`: Test case has multiple test sequences
- `single-sequence`: Test case has exactly one test sequence
- `has-manual-steps`: Test case contains manual steps
- `automated-only`: Test case has no manual steps
- `has-initial-conditions`: Test case defines initial conditions

Enable dynamic tags with the `--dynamic-tags` flag.

## Filtering Test Cases

### Include Tags

Run only test cases that have at least one of the specified tags:

```bash
# Run all smoke tests
test-orchestrator run-all --include-tags smoke

# Run smoke OR regression tests
test-orchestrator run-all --include-tags smoke,regression
```

### Exclude Tags

Exclude test cases that have any of the specified tags:

```bash
# Run all tests except slow ones
test-orchestrator run-all --exclude-tags slow

# Exclude manual and broken tests
test-orchestrator run-all --exclude-tags manual,broken
```

### Tag Expressions

Use complex boolean expressions for advanced filtering:

```bash
# Run smoke tests that are not slow
test-orchestrator run-all --tag-expr "smoke && !slow"

# Run smoke OR regression tests that are fast
test-orchestrator run-all --tag-expr "(smoke || regression) && fast"

# Run high priority tests excluding manual ones
test-orchestrator run-all --tag-expr "priority-high && !has-manual-steps" --dynamic-tags
```

### Expression Syntax

- `tag`: Match tests with this tag
- `tag1 && tag2`: Match tests with both tags (AND)
- `tag1 || tag2`: Match tests with either tag (OR)
- `!tag`: Match tests without this tag (NOT)
- `(expr)`: Group expressions for precedence

### Combining Filters

You can combine multiple filter types:

```bash
# Include smoke tests, exclude slow ones, and apply expression
test-orchestrator run-all \
  --include-tags smoke \
  --exclude-tags slow \
  --tag-expr "priority-high || critical"
```

Filter evaluation order:
1. Exclude tags (eliminates tests first)
2. Include tags (selects from remaining)
3. Tag expressions (further refines selection)

## Tag Management Commands

### List All Tags

Display all tags across all test cases:

```bash
test-orchestrator list-tags
```

Output:
```
=== Available Tags ===

Total tags: 8

  automated-only (2 test case(s))
  fast (3 test case(s))
  integration (1 test case(s))
  multi-sequence (1 test case(s))
  priority-high (2 test case(s))
  regression (1 test case(s))
  smoke (2 test case(s))
```

### Show Tags for Test Case

View all tags for a specific test case (including inherited and dynamic):

```bash
test-orchestrator show-tags TC001
```

Output:
```
=== Tags for Test Case: TC001 ===

Tags:
  - automated-only
  - authentication
  - fast
  - multi-sequence
  - priority-high
  - smoke
```

### Find Test Cases by Tag

Search for test cases with a specific tag:

```bash
test-orchestrator find-by-tag smoke
```

Output:
```
=== Test Cases with Tag: smoke ===

Found 2 test case(s):

  TC001 - User authentication test
  TC002 - API endpoint test
```

## Usage Examples

### Using Make Goals

The repository includes convenient make goals for testing tag filtering:

```bash
# Test all tag filtering capabilities
make test-filter-all

# Run specific filtered test suites
make test-filter-smoke          # Run smoke tests
make test-filter-fast           # Run fast tests
make test-filter-priority-high  # Run priority-high tests
make test-filter-automated      # Run automated-only tests
make test-filter-no-slow        # Run all tests except slow ones
make test-filter-expression     # Run with complex expression

# Test the tagged example file
make test-tagged-example
```

### Run Smoke Tests

```bash
test-orchestrator run-all --include-tags smoke
# Or using make:
make test-filter-smoke
```

### Run Fast Regression Tests

```bash
test-orchestrator run-all \
  --include-tags regression \
  --tag-expr "fast && !slow"
# Or using make for fast tests:
make test-filter-fast
```

### Run High Priority Tests (Automated Only)

```bash
test-orchestrator run-all \
  --tag-expr "priority-high && automated-only" \
  --dynamic-tags
# Or using make for priority-high tests:
make test-filter-priority-high
# Or for automated tests:
make test-filter-automated
```

### Run Critical Tests Excluding Known Issues

```bash
test-orchestrator run-all \
  --include-tags critical \
  --exclude-tags flaky,known-issue
```

### Run Specific Tests with Tag Filtering

```bash
# Run specific test cases but filter their sequences
test-orchestrator run TC001 TC002 TC003 \
  --include-tags fast \
  --dynamic-tags
```

## Best Practices

### Tag Naming Conventions

1. **Use lowercase**: `smoke`, `regression`, not `Smoke`, `REGRESSION`
2. **Use hyphens for multi-word**: `priority-high`, `api-test`
3. **Be consistent**: Choose a convention and stick to it
4. **Be descriptive**: Tags should be self-explanatory

### Recommended Tag Categories

#### Test Type
- `smoke`: Quick sanity tests
- `regression`: Tests for known issues
- `integration`: Integration tests
- `unit`: Unit tests
- `e2e`: End-to-end tests

#### Priority
- `priority-high`
- `priority-medium`
- `priority-low`
- `critical`: Must pass tests
- `blocker`: Blocks releases

#### Performance
- `fast`: Quick tests (< 1 minute)
- `slow`: Long-running tests
- `performance`: Performance benchmarks

#### Test Area
- `authentication`
- `authorization`
- `api`
- `ui`
- `database`

#### Status
- `stable`: Reliable tests
- `flaky`: Intermittent failures
- `wip`: Work in progress
- `broken`: Known to fail
- `manual`: Requires manual intervention

### Organizing Test Suites

Use tag combinations to create virtual test suites:

```bash
# Quick CI suite
--tag-expr "smoke && fast && !flaky"

# Pre-deployment suite
--tag-expr "(smoke || regression) && !broken && !wip"

# Full regression suite
--include-tags regression --exclude-tags flaky,broken

# Nightly test suite
--tag-expr "(regression || integration) && !manual"
```

## Implementation Details

### Tag Storage

Tags are stored directly in test case YAML files and are loaded into memory when test cases are parsed.

### Tag Evaluation

The system evaluates tags in this order:

1. **Static Tags**: Tags defined in YAML files
2. **Inherited Tags**: Tags from parent test cases
3. **Dynamic Tags**: Automatically computed tags (if enabled)

### Performance Considerations

- Tag filtering occurs before test execution
- Tag evaluation is performed once during test case loading
- Dynamic tag evaluation has minimal overhead
- Large test suites benefit from efficient tag-based filtering

## API Reference

### TagFilter

```rust
use testcase_manager::tags::TagFilter;

let filter = TagFilter::new()
    .with_include_tags(vec!["smoke".to_string()])
    .with_exclude_tags(vec!["slow".to_string()]);
```

### TagExpression

```rust
use testcase_manager::tags::TagExpression;

let expr = TagExpression::parse("smoke && !slow")?;
```

### DynamicTagEvaluator

```rust
use testcase_manager::tags::DynamicTagEvaluator;

let evaluator = DynamicTagEvaluator::with_default_rules();
let dynamic_tags = evaluator.evaluate(&test_case);
```

## Troubleshooting

### No Tests Selected

If filtering returns no tests:

1. List all available tags: `test-orchestrator list-tags`
2. Check tag spelling and capitalization
3. Verify test cases have the expected tags
4. Use `show-tags` to see effective tags for a test case
5. Try broader filters first, then narrow down

### Expression Parse Errors

If tag expressions fail to parse:

1. Check for balanced parentheses
2. Ensure proper spacing around operators
3. Verify tag names don't contain spaces
4. Use quotes around the entire expression: `--tag-expr "expr"`

### Unexpected Results

If filtering gives unexpected results:

1. Enable dynamic tags if using computed tags: `--dynamic-tags`
2. Check tag inheritance (sequences inherit from test cases)
3. Verify filter precedence (exclude → include → expression)
4. Test expressions incrementally to isolate issues

## Future Enhancements

Potential future additions:

- Tag metadata (descriptions, categories)
- Tag aliases and synonyms
- Tag validation and suggestions
- Tag-based test reports
- Tag usage analytics
- Custom dynamic tag rules from config
- Tag-based test prioritization
