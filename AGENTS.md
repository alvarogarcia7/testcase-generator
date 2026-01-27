# AGENTS.md

## Commands
- **Build**: make build
- **Lint**: make lint
- **Test**: make test
- **Test Tag Filtering**: make test-filter-all (tests all tag filtering capabilities)
- **Test Specific Tag Filter**: make test-filter-smoke, make test-filter-fast, etc.
- **Watch Mode**: make watch (monitors testcases/ for changes and auto-validates)
- **Dev Server**: N/A

You must build, test, and lint before committing

## Testing Requirements

**MANDATORY**: All agents must run the full test suite before considering any task complete. Testing is a critical step that cannot be skipped.

### Test Execution
- Run tests using: `cargo test --all-features`
- This ensures comprehensive validation across the entire codebase with all feature flags enabled
- Alternative basic test command: `cargo test`

### Test Requirements
- **All tests must pass** before any code changes can be committed
- If tests fail, investigate and fix the failures before proceeding
- Never commit code with failing tests
- Update or add tests as needed when modifying functionality

