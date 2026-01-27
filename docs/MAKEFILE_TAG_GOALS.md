# Makefile Tag Filter Goals

Quick reference for tag filtering make goals.

## Test Goals

### test-tagged-example
Tests the example tagged test case file.

```bash
make test-tagged-example
```

This goal:
- Validates `testcases/example_tagged_test.yml` against the schema
- Tests tag listing functionality
- Tests tag filtering functionality
- Is automatically run as part of `make test`

### test-filter-smoke
Runs all tests tagged with `smoke`.

```bash
make test-filter-smoke
```

Equivalent to:
```bash
cargo run --bin test-orchestrator run-all --include-tags smoke
```

### test-filter-fast
Runs all tests tagged with `fast`.

```bash
make test-filter-fast
```

Equivalent to:
```bash
cargo run --bin test-orchestrator run-all --include-tags fast
```

### test-filter-priority-high
Runs all tests tagged with `priority-high`.

```bash
make test-filter-priority-high
```

Equivalent to:
```bash
cargo run --bin test-orchestrator run-all --include-tags priority-high
```

### test-filter-automated
Runs all automated tests (using dynamic tags).

```bash
make test-filter-automated
```

Equivalent to:
```bash
cargo run --bin test-orchestrator run-all --dynamic-tags --include-tags automated-only
```

### test-filter-no-slow
Runs all tests excluding those tagged with `slow`.

```bash
make test-filter-no-slow
```

Equivalent to:
```bash
cargo run --bin test-orchestrator run-all --exclude-tags slow
```

### test-filter-expression
Runs tests using a complex boolean expression.

```bash
make test-filter-expression
```

Equivalent to:
```bash
cargo run --bin test-orchestrator run-all --tag-expr "(smoke || regression) && !slow"
```

### test-filter-all
Runs all tag filter test goals sequentially.

```bash
make test-filter-all
```

This runs:
1. `test-filter-smoke`
2. `test-filter-fast`
3. `test-filter-priority-high`
4. `test-filter-automated`
5. `test-filter-no-slow`
6. `test-filter-expression`

## Usage in CI/CD

These goals are designed to be used in continuous integration pipelines:

```yaml
# Example GitHub Actions workflow
jobs:
  smoke-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - run: make test-filter-smoke

  fast-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - run: make test-filter-fast

  full-suite:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - run: make test-filter-all
```

## Custom Filter Goals

You can easily add your own custom filter goals to the Makefile:

```makefile
test-filter-mycustom: build
	@echo "Running my custom filtered tests..."
	cargo run --bin test-orchestrator run-all --include-tags mycustom,mytag
.PHONY: test-filter-mycustom
```

## Integration with Main Test Suite

The `test-tagged-example` goal is automatically included in `make test`:

```makefile
test:
	${MAKE} test-unit
	${MAKE} test-e2e
	${MAKE} test-tagged-example    # Automatically runs
	#${MAKE} verify-testcases
.PHONY: test
```

This ensures that the tagging system is validated as part of the standard test suite.
