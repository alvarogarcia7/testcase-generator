# AGENTS.md

## Setup
```bash
# No setup required yet - empty repository
```

## Configuration

### Editor Settings
You can configure which text editor to use by setting environment variables. Copy `.env.example` to `.env` and customize:

```bash
cp .env.example .env
```

Supported environment variables:
- **EDITOR**: Default text editor for general editing operations (e.g., `vim`, `nano`, `emacs`, `code`)
- **VISUAL**: Visual editor for more complex editing tasks (falls back to EDITOR if not set)
- **TESTCASE_EDITOR**: Editor specifically for editing test case files (falls back to VISUAL or EDITOR if not set)

Example `.env` file:
```bash
EDITOR=vim
VISUAL=code
TESTCASE_EDITOR=nano
```

## Commands
- **Build**: make build
- **Lint**: make lint
- **Test**: make test
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

## Tech Stack
- Not yet initialized

## Architecture
- Repository structure to be determined

## Code Style
- Follow language-specific conventions once codebase is established
