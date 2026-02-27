# AGENTS.md

## Feature Overview

This project is a YAML-based test harness that converts declarative test case definitions into executable bash scripts. Key features include:

- **Declarative Test Cases**: Define test sequences, steps, and expectations in YAML
- **Variable Capture**: Extract values from command output using regex patterns or commands
- **Conditional Verification**: Support for if/then/else logic in verification expressions
- **Prerequisites**: Define manual and automatic prerequisites with verification commands
- **Environment Variables**: Hydration support with required/optional environment variables
- **Test Execution Lifecycle Hooks**: Optional hooks for custom setup, teardown, logging, and resource management at eight different lifecycle points
- **Shell Script Generation**: Generate portable bash 3.2+ compatible scripts from test cases
- **Comprehensive Validation**: Built-in schema validation and test execution verification

### Test Execution Lifecycle Hooks

Hooks provide optional extensibility points throughout the test execution lifecycle. **Hooks are entirely optional** - test cases work perfectly without them. When defined, hooks enable:

- **Custom Setup/Teardown**: Initialize and clean up resources at various lifecycle points
- **Logging and Monitoring**: Track test execution with custom logging at any stage
- **Resource Management**: Create temporary directories, files, and external resources
- **Integration with External Systems**: Connect to databases, APIs, or monitoring systems
- **Context-Aware Operations**: Access test execution context (sequences, steps, variables)
- **Error Handling**: Choose between strict (fail on error) or lenient (continue on error) modes

**Eight Hook Types Available:**
1. `script_start` - Once at script beginning (global initialization)
2. `setup_test` - Once after script_start (test-wide setup)
3. `before_sequence` - Before each test sequence (sequence initialization)
4. `after_sequence` - After each test sequence (sequence cleanup)
5. `before_step` - Before each test step (step preparation)
6. `after_step` - After each test step (step validation)
7. `teardown_test` - Once before script_end (test-wide cleanup)
8. `script_end` - Once at script end (final logging/cleanup)

See the [Hooks](#hooks) section for detailed documentation and examples.

## Commands
- **Build**: make build
- **Lint**: make lint
- **Test**: make test
- **Coverage**: make coverage (run unit tests with coverage analysis, 50% threshold)
- **Coverage E2E**: make coverage-e2e (run unit + e2e tests with coverage analysis, 70% threshold)
- **Coverage HTML**: make coverage-html (generate HTML coverage report)
- **Coverage HTML E2E**: make coverage-html-e2e (generate HTML coverage report with e2e tests)
- **Coverage Report**: make coverage-report (display coverage summary)
- **Coverage Report E2E**: make coverage-report-e2e (display coverage summary with e2e tests)
- **Install Coverage Tools**: make install-coverage-tools (install cargo-llvm-cov and related tools)
- **Verify Scripts**: make verify-scripts (verify syntax of all shell scripts)
- **Watch Mode**: make watch (monitors testcases/ for changes and auto-validates)
- **Dev Server**: N/A

You must build, test, lint, and verify coverage before committing

## Binaries

The project includes several binary utilities:

- **json-escape**: A utility that reads from stdin and performs JSON string escaping. Supports a test mode (`--test`) to validate that escaped output is valid JSON when wrapped in quotes, and verbose mode (`--verbose`) for detailed logging.
  - Build: `make build-json-escape`
  - Run: `make run-json-escape` or `cargo run --bin json-escape`
  - Usage: `echo "text" | json-escape`

## Shell Script Compatibility

**MANDATORY**: All shell scripts and generated bash scripts must be compatible with both BSD and GNU variants of command-line tools, and must work with bash 3.2+ (the default on macOS).

### Key Requirements:
- Scripts must work on macOS (BSD) and Linux (GNU) without modification
- Scripts must be compatible with bash 3.2+ (macOS ships with bash 3.2 by default)
- Avoid GNU-specific flags or options that don't exist in BSD variants
- Avoid bash 4.0+ features like associative arrays (`declare -A`)
- Test commands like `sed`, `grep`, `awk`, `find`, etc. must use portable syntax
- When using regex, ensure patterns are compatible with both POSIX and GNU extended regex
- Use POSIX-compliant shell constructs where possible

### Logging Library

**MANDATORY**: All shell scripts must use the centralized logging library for consistent output formatting.

**Location**: `scripts/lib/logger.sh`

**Usage**:
```bash
#!/usr/bin/env bash
set -e

# Get script directory and source logger
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

# Use logging functions
log_info "Informational message"
log_warning "Warning message"
log_error "Error message"
log_debug "Debug message (only shown if VERBOSE=1)"
log_verbose "Verbose message (only shown if VERBOSE=1)"

# Use color-coded test helpers
pass "Test passed"
fail "Test failed"
info "Information"
section "Section Header"
```

**Available Functions**:
- `log_info "message"` - Standard informational message
- `log_warning "message"` - Warning message
- `log_error "message"` - Error message (outputs to stderr)
- `log_debug "message"` - Debug message (only shown when VERBOSE=1)
- `log_verbose "message"` - Verbose message (only shown when VERBOSE=1)
- `pass "message"` - Success message with green checkmark (✓)
- `fail "message"` - Failure message with red X (✗)
- `info "message"` - Info message with blue info symbol (ℹ)
- `section "title"` - Section header with yellow highlighting

**Cleanup Management**:
The logger library also provides cleanup management for temporary files and background processes:
- `setup_cleanup "/path/to/temp/dir"` - Register temporary directory for cleanup
- `register_background_pid $PID` - Register background process for cleanup
- `disable_cleanup` - Disable automatic cleanup (for debugging)
- `enable_cleanup` - Re-enable automatic cleanup

**Benefits**:
- Consistent formatting across all scripts
- Color-coded output for better readability
- Automatic cleanup of temporary resources
- Easy integration with CI/CD pipelines

### Common Pitfalls:
- `grep -P` (Perl regex) is GNU-only - use `sed -n` with capture groups instead
- `sed -r` is GNU-only - use `sed -E` for BSD/macOS compatibility
- `date` formatting differs between BSD and GNU
- `readlink -f` is GNU-only - use alternative methods for BSD
- `declare -A` (associative arrays) requires bash 4.0+ - use eval with dynamic variable names for bash 3.2+

### Testing:
- Test generated scripts on both macOS and Linux when possible
- Use portable regex patterns that work with both implementations
- Verify scripts work with bash 3.2 (default on macOS)
- Verify script syntax using `make verify-scripts`

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

## Hooks

Hooks provide optional extensibility points in the test execution lifecycle, enabling custom setup, teardown, logging, and resource management. **Hooks are entirely optional** - all test cases function normally without defining any hooks.

### Overview

Hooks allow you to inject custom scripts at eight different points in the test execution lifecycle:

1. **script_start** - Executes once at the very beginning of the generated test script
2. **setup_test** - Executes once after script_start, before any test sequences run
3. **before_sequence** - Executes before each test sequence starts
4. **after_sequence** - Executes after each test sequence completes
5. **before_step** - Executes before each test step
6. **after_step** - Executes after each test step completes
7. **teardown_test** - Executes once after all test sequences, before script_end
8. **script_end** - Executes once at the very end of the test script

### Configuration

Hooks are defined in the test case YAML under the `hooks` key:

```yaml
hooks:
  script_start:
    command: "scripts/script_start.sh"
    on_error: "fail"
  setup_test:
    command: "scripts/setup_test.sh"
  before_sequence:
    command: "scripts/before_sequence.sh"
  after_sequence:
    command: "scripts/after_sequence.sh"
  before_step:
    command: "scripts/before_step.sh"
  after_step:
    command: "scripts/after_step.sh"
    on_error: "continue"
  teardown_test:
    command: "scripts/teardown_test.sh"
  script_end:
    command: "scripts/script_end.sh"
```

#### Hook Configuration Fields

- **command** (required): Path to the script or command to execute. Paths are relative to the test case YAML file location, or can be absolute paths.
- **on_error** (optional): Error handling behavior when the hook fails
  - `fail` (default): Test execution stops immediately if the hook fails
  - `continue`: Hook errors are logged but test execution continues

### Available Environment Variables

Hooks have access to the test execution context through environment variables:

#### All Hooks
- Standard environment variables from the test execution context
- Any environment variables defined in the test case's `hydration_vars`

#### before_sequence and after_sequence
- `TEST_SEQUENCE_ID`: The sequence ID (e.g., "1", "2")
- `TEST_SEQUENCE_NAME`: The sequence name

#### before_step and after_step
- `TEST_SEQUENCE_ID`: The sequence ID
- `TEST_STEP_NUMBER`: The step number
- `TEST_STEP_DESCRIPTION`: The step description
- All sequence-scoped variables defined in the test sequence
- All captured variables from previous steps (in after_step)

#### after_step only
- `STEP_EXIT_CODE`: The exit code of the step command
- `COMMAND_OUTPUT`: The output from the step command

### Common Use Cases

#### 1. Test Environment Setup

Create temporary directories and initialize resources:

```yaml
hooks:
  setup_test:
    command: "scripts/setup_environment.sh"
  teardown_test:
    command: "scripts/cleanup_environment.sh"
```

**setup_environment.sh:**
```bash
#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/lib/logger.sh" || exit 1

# Create test workspace
TEST_WORKSPACE="/tmp/test_workspace_$$"
mkdir -p "$TEST_WORKSPACE"
echo "$TEST_WORKSPACE" > /tmp/test_workspace_path.txt

log_info "Created test workspace: $TEST_WORKSPACE"

# Initialize test database
log_info "Initializing test database..."
sqlite3 "$TEST_WORKSPACE/test.db" "CREATE TABLE tests (id INTEGER, name TEXT);"

log_info "Test environment setup complete"
```

**cleanup_environment.sh:**
```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/lib/logger.sh" || exit 1

# Read workspace path
if [ -f /tmp/test_workspace_path.txt ]; then
    TEST_WORKSPACE=$(cat /tmp/test_workspace_path.txt)
    if [ -d "$TEST_WORKSPACE" ]; then
        rm -rf "$TEST_WORKSPACE"
        log_info "Removed test workspace: $TEST_WORKSPACE"
    fi
    rm -f /tmp/test_workspace_path.txt
fi

log_info "Test environment cleanup complete"
```

#### 2. Custom Logging

Track test execution with detailed logging:

```yaml
hooks:
  script_start:
    command: "scripts/log_start.sh"
  before_sequence:
    command: "scripts/log_sequence_start.sh"
  after_step:
    command: "scripts/log_step_result.sh"
  script_end:
    command: "scripts/log_completion.sh"
```

**log_start.sh:**
```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/lib/logger.sh" || exit 1

START_TIME=$(date +%s)
echo "$START_TIME" > /tmp/test_start_time.txt

log_info "========================================="
log_info "Test Execution Started: $(date)"
log_info "========================================="
```

**log_sequence_start.sh:**
```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/lib/logger.sh" || exit 1

SEQUENCE_ID="${TEST_SEQUENCE_ID:-unknown}"
SEQUENCE_NAME="${TEST_SEQUENCE_NAME:-unknown}"

section "Sequence $SEQUENCE_ID: $SEQUENCE_NAME"
log_info "Starting test sequence: $SEQUENCE_NAME (ID: $SEQUENCE_ID)"
```

**log_step_result.sh:**
```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/lib/logger.sh" || exit 1

STEP_NUMBER="${TEST_STEP_NUMBER:-unknown}"
EXIT_CODE="${STEP_EXIT_CODE:-unknown}"

if [ "$EXIT_CODE" = "0" ]; then
    pass "Step $STEP_NUMBER completed successfully"
else
    fail "Step $STEP_NUMBER failed with exit code: $EXIT_CODE"
fi
```

**log_completion.sh:**
```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/lib/logger.sh" || exit 1

if [ -f /tmp/test_start_time.txt ]; then
    START_TIME=$(cat /tmp/test_start_time.txt)
    END_TIME=$(date +%s)
    DURATION=$((END_TIME - START_TIME))
    
    log_info "========================================="
    log_info "Test Execution Completed: $(date)"
    log_info "Total Duration: ${DURATION}s"
    log_info "========================================="
    
    rm -f /tmp/test_start_time.txt
fi
```

#### 3. Resource Cleanup

Ensure proper cleanup even when tests fail:

```yaml
hooks:
  before_sequence:
    command: "scripts/allocate_resources.sh"
  after_sequence:
    command: "scripts/release_resources.sh"
    on_error: "continue"  # Always try to clean up
```

**allocate_resources.sh:**
```bash
#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/lib/logger.sh" || exit 1

SEQUENCE_ID="${TEST_SEQUENCE_ID:-1}"
RESOURCE_DIR="/tmp/test_resources_seq_${SEQUENCE_ID}"

mkdir -p "$RESOURCE_DIR"
echo "$RESOURCE_DIR" > "/tmp/resource_dir_seq_${SEQUENCE_ID}.txt"

# Allocate test resources
log_info "Allocated resources for sequence $SEQUENCE_ID: $RESOURCE_DIR"

# Create lock file to track active resources
echo "$$" > "$RESOURCE_DIR/lock"
```

**release_resources.sh:**
```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/lib/logger.sh" || exit 1

SEQUENCE_ID="${TEST_SEQUENCE_ID:-1}"
RESOURCE_FILE="/tmp/resource_dir_seq_${SEQUENCE_ID}.txt"

if [ -f "$RESOURCE_FILE" ]; then
    RESOURCE_DIR=$(cat "$RESOURCE_FILE")
    if [ -d "$RESOURCE_DIR" ]; then
        rm -rf "$RESOURCE_DIR"
        log_info "Released resources for sequence $SEQUENCE_ID"
    fi
    rm -f "$RESOURCE_FILE"
fi
```

#### 4. Integration with External Systems

Connect to external monitoring or reporting systems:

```yaml
hooks:
  script_start:
    command: "scripts/notify_test_start.sh"
    on_error: "continue"  # Don't fail if monitoring unavailable
  after_step:
    command: "scripts/report_step_metrics.sh"
    on_error: "continue"
  script_end:
    command: "scripts/notify_test_complete.sh"
    on_error: "continue"
```

**notify_test_start.sh:**
```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/lib/logger.sh" || exit 1

TEST_ID="${TEST_CASE_ID:-unknown}"
MONITORING_URL="${MONITORING_ENDPOINT:-http://localhost:8080/api/tests}"

# Send test start notification
curl -s -X POST "$MONITORING_URL/start" \
    -H "Content-Type: application/json" \
    -d "{\"test_id\":\"$TEST_ID\",\"status\":\"started\",\"timestamp\":\"$(date -u +%Y-%m-%dT%H:%M:%SZ)\"}" \
    > /dev/null 2>&1 || log_warning "Failed to notify monitoring system"

log_info "Notified monitoring system: test started"
```

**report_step_metrics.sh:**
```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/lib/logger.sh" || exit 1

STEP_NUMBER="${TEST_STEP_NUMBER:-unknown}"
EXIT_CODE="${STEP_EXIT_CODE:-unknown}"
METRICS_DB="${METRICS_DATABASE:-/tmp/test_metrics.db}"

# Record step metrics to database
if command -v sqlite3 > /dev/null 2>&1; then
    sqlite3 "$METRICS_DB" \
        "INSERT INTO step_metrics (step_number, exit_code, timestamp) \
         VALUES ($STEP_NUMBER, $EXIT_CODE, datetime('now'));" \
        2>/dev/null || log_verbose "Metrics database not available"
fi

log_verbose "Reported metrics for step $STEP_NUMBER"
```

**notify_test_complete.sh:**
```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/lib/logger.sh" || exit 1

TEST_ID="${TEST_CASE_ID:-unknown}"
MONITORING_URL="${MONITORING_ENDPOINT:-http://localhost:8080/api/tests}"

# Calculate test duration
if [ -f /tmp/test_start_time.txt ]; then
    START_TIME=$(cat /tmp/test_start_time.txt)
    END_TIME=$(date +%s)
    DURATION=$((END_TIME - START_TIME))
else
    DURATION=0
fi

# Send completion notification
curl -s -X POST "$MONITORING_URL/complete" \
    -H "Content-Type: application/json" \
    -d "{\"test_id\":\"$TEST_ID\",\"status\":\"completed\",\"duration\":$DURATION,\"timestamp\":\"$(date -u +%Y-%m-%dT%H:%M:%SZ)\"}" \
    > /dev/null 2>&1 || log_warning "Failed to notify monitoring system"

log_info "Notified monitoring system: test completed (${DURATION}s)"
```

### Best Practices

1. **Use the Logger Library**: All hook scripts should use the centralized `scripts/lib/logger.sh` for consistent output formatting
2. **Handle Errors Gracefully**: Use `on_error: "continue"` for cleanup hooks to ensure they always run
3. **Shell Compatibility**: Hook scripts must be compatible with bash 3.2+ (BSD and GNU variants)
4. **Resource Tracking**: Use temporary files to track resources created by hooks for proper cleanup
5. **Minimal Side Effects**: Hooks should be lightweight and not significantly impact test execution time
6. **Environment Variable Access**: Use `${VAR:-default}` syntax to provide defaults for optional variables
7. **Idempotent Operations**: Design hooks to be safely re-runnable when possible

### Example: Complete Test Case with Hooks

See `testcases/examples/hooks/TC_HOOKS_001.yaml` for a comprehensive example demonstrating all eight hook types with:
- Resource management (temporary directories)
- Logging integration (centralized logger library)
- Variable access (sequence and step context)
- Error handling (both fail and continue modes)
- Timing and duration tracking

Full documentation and example hook scripts are available in `testcases/examples/hooks/README.md`.

## Coverage Testing

**MANDATORY**: Code coverage testing is required to ensure comprehensive test coverage across the codebase.

### Installation

Install coverage tools using the provided installation script:

```bash
make install-coverage-tools
```

Or manually install `cargo-llvm-cov`:

```bash
cargo install cargo-llvm-cov
```

For more details on coverage tool installation, see `scripts/README_COVERAGE_TOOLS.md`.

### Coverage Commands

- **Run unit tests with coverage**: `make coverage`
  - Executes unit tests with coverage analysis enabled
  - Minimum threshold: 50% line coverage
  - Excludes: fuzzy.rs, prompts.rs, main_editor.rs

- **Run all tests with coverage (including e2e)**: `make coverage-e2e`
  - Executes unit tests and e2e integration tests with coverage analysis
  - Minimum threshold: 70% line coverage
  - Excludes: fuzzy.rs, prompts.rs, main_editor.rs

- **Generate HTML coverage report**: `make coverage-html`
  - Creates an interactive HTML report showing line-by-line coverage (unit tests only)
  - Opens automatically in your default browser
  - Useful for identifying untested code paths

- **Generate HTML coverage report with e2e**: `make coverage-html-e2e`
  - Creates an interactive HTML report including e2e test coverage
  - Opens automatically in your default browser

- **Display coverage summary**: `make coverage-report`
  - Shows a summary of coverage statistics in the terminal (unit tests only)
  - Provides quick overview of coverage percentages

- **Display coverage summary with e2e**: `make coverage-report-e2e`
  - Shows a summary of coverage statistics including e2e tests
  - Provides quick overview of coverage percentages

### Coverage Exclusions

The following files are excluded from coverage analysis:
- `src/fuzzy.rs` - Interactive fuzzy finder UI components
- `src/prompts.rs` - Interactive prompt UI components
- `src/main_editor.rs` - Main editor binary entry point

### Coverage Requirements

- **Minimum coverage threshold (unit tests)**: 50% line coverage
- **Minimum coverage threshold (unit + e2e tests)**: 70% line coverage
- Coverage must be maintained or improved with each commit
- New code should strive for higher coverage (80%+) when possible
- Review coverage reports to identify critical untested paths

### Pre-Commit Workflow

Before committing any code changes, complete the following steps in order:

1. **Build**: `make build` - Ensure code compiles without errors
2. **Lint**: `make lint` - Fix any style or quality issues
3. **Test**: `make test` - Verify all tests pass
4. **Coverage**: `make coverage-e2e` - Verify coverage meets 70% threshold with e2e tests

All steps must complete successfully before committing changes.


<!-- BACKLOG.MD MCP GUIDELINES START -->

<CRITICAL_INSTRUCTION>

## BACKLOG WORKFLOW INSTRUCTIONS

This project uses Backlog.md MCP for all task and project management activities.

**CRITICAL GUIDANCE**

- If your client supports MCP resources, read `backlog://workflow/overview` to understand when and how to use Backlog for this project.
- If your client only supports tools or the above request fails, call `backlog.get_workflow_overview()` tool to load the tool-oriented overview (it lists the matching guide tools).

- **First time working here?** Read the overview resource IMMEDIATELY to learn the workflow
- **Already familiar?** You should have the overview cached ("## Backlog.md Overview (MCP)")
- **When to read it**: BEFORE creating tasks, or when you're unsure whether to track work

These guides cover:
- Decision framework for when to create tasks
- Search-first workflow to avoid duplicates
- Links to detailed guides for task creation, execution, and finalization
- MCP tools reference

You MUST read the overview resource to understand the complete workflow. The information is NOT summarized here.

</CRITICAL_INSTRUCTION>

<!-- BACKLOG.MD MCP GUIDELINES END -->
