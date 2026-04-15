#!/usr/bin/env bash
#
# campaign-run.sh - Run tests for a test campaign
#
# DESCRIPTION:
#   Executes test cases for an active campaign, with optional regex filtering.
#   Generates execution logs and copies test case files to campaign directory.
#
# USAGE:
#   ./scripts/campaign-run.sh [OPTIONS]
#
# OPTIONS:
#   --campaign DIR         Campaign directory (required)
#   --pattern REGEX        Test case filename regex pattern (default: .*)
#   --testcase-dir DIR     Override test case directory from campaign
#   --parallel N           Run N tests in parallel (default: 1)
#   --continue-on-error    Continue running tests even if some fail
#   --skip-verification    Skip verification phase
#   --verbose              Enable verbose output
#   --help                 Show this help message
#
# EXAMPLES:
#   # Run all tests in campaign
#   ./scripts/campaign-run.sh --campaign campaigns/Sprint_23
#
#   # Run only tests matching pattern
#   ./scripts/campaign-run.sh --campaign campaigns/Sprint_23 --pattern "EXAMPLE_.*\.yml"
#
#   # Run tests in parallel
#   ./scripts/campaign-run.sh --campaign campaigns/Sprint_23 --parallel 4
#

set -e

# Get script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source required libraries
source "$SCRIPT_DIR/lib/logger.sh" || exit 1
source "$SCRIPT_DIR/lib/find-binary.sh" || exit 1

# Default configuration
CAMPAIGN_DIR=""
TESTCASE_PATTERN=".*"
TESTCASE_DIR=""
PARALLEL_JOBS=1
CONTINUE_ON_ERROR=0
SKIP_VERIFICATION=0
VERBOSE=0

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --campaign)
            CAMPAIGN_DIR="$2"
            shift 2
            ;;
        --pattern)
            TESTCASE_PATTERN="$2"
            shift 2
            ;;
        --testcase-dir)
            TESTCASE_DIR="$2"
            shift 2
            ;;
        --parallel)
            PARALLEL_JOBS="$2"
            shift 2
            ;;
        --continue-on-error)
            CONTINUE_ON_ERROR=1
            shift
            ;;
        --skip-verification)
            SKIP_VERIFICATION=1
            shift
            ;;
        --verbose)
            VERBOSE=1
            export VERBOSE
            shift
            ;;
        --help)
            head -n 40 "$0" | tail -n +2 | sed 's/^# //'
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Validate required parameters
if [[ -z "$CAMPAIGN_DIR" ]]; then
    log_error "Campaign directory is required (--campaign)"
    echo "Use --help for usage information"
    exit 1
fi

# Validate campaign directory exists
if [[ ! -d "$CAMPAIGN_DIR" ]]; then
    log_error "Campaign directory does not exist: $CAMPAIGN_DIR"
    exit 1
fi

# Check campaign state
CAMPAIGN_STATE="$CAMPAIGN_DIR/metadata/state.txt"
if [[ ! -f "$CAMPAIGN_STATE" ]]; then
    log_error "Campaign state file not found: $CAMPAIGN_STATE"
    log_error "This does not appear to be a valid campaign directory"
    exit 1
fi

STATE=$(cat "$CAMPAIGN_STATE")
if [[ "$STATE" != "ACTIVE" ]]; then
    log_error "Campaign is not active (current state: $STATE)"
    log_error "Cannot run tests on a completed campaign"
    exit 1
fi

# Load campaign metadata
CAMPAIGN_METADATA="$CAMPAIGN_DIR/metadata/campaign.yaml"
if [[ ! -f "$CAMPAIGN_METADATA" ]]; then
    log_error "Campaign metadata not found: $CAMPAIGN_METADATA"
    exit 1
fi

# Get testcase directory from metadata if not specified
if [[ -z "$TESTCASE_DIR" ]]; then
    TESTCASE_DIR=$(grep 'testcase_dir:' "$CAMPAIGN_METADATA" | sed 's/.*testcase_dir: *"\(.*\)".*/\1/' | tr -d '"')
fi

if [[ ! -d "$TESTCASE_DIR" ]]; then
    log_error "Test case directory does not exist: $TESTCASE_DIR"
    exit 1
fi

# Display configuration
section "Running Test Campaign"
log_info "Campaign Configuration:"
log_info "  Campaign directory: $CAMPAIGN_DIR"
log_info "  Test case directory: $TESTCASE_DIR"
log_info "  Pattern filter: $TESTCASE_PATTERN"
log_info "  Parallel jobs: $PARALLEL_JOBS"
log_info "  Continue on error: $CONTINUE_ON_ERROR"
log_info "  Skip verification: $SKIP_VERIFICATION"
log_info "  Verbose: $VERBOSE"
echo ""

# Increment run counter
CAMPAIGN_COUNTER="$CAMPAIGN_DIR/metadata/run_counter.txt"
if [[ -f "$CAMPAIGN_COUNTER" ]]; then
    RUN_NUMBER=$(cat "$CAMPAIGN_COUNTER")
    RUN_NUMBER=$((RUN_NUMBER + 1))
    echo "$RUN_NUMBER" > "$CAMPAIGN_COUNTER"
else
    RUN_NUMBER=1
    echo "$RUN_NUMBER" > "$CAMPAIGN_COUNTER"
fi

RUN_TIMESTAMP=$(date -u +"%Y%m%d_%H%M%S")
RUN_ID="run_${RUN_NUMBER}_${RUN_TIMESTAMP}"

log_info "Run ID: $RUN_ID"
echo ""

# Create run-specific directories
RUN_EXECUTION_DIR="$CAMPAIGN_DIR/execution_logs/$RUN_ID"
RUN_TESTCASE_DIR="$CAMPAIGN_DIR/testcases/$RUN_ID"
mkdir -p "$RUN_EXECUTION_DIR"
mkdir -p "$RUN_TESTCASE_DIR"

# Build required binaries
section "Building Required Binaries"

log_info "Ensuring test-executor binary is built..."
TEST_EXECUTOR=$(find_binary "test-executor")
if [[ -z "$TEST_EXECUTOR" ]]; then
    log_info "Building test-executor..."
    if cargo build --release --bin test-executor 2>&1 | while IFS= read -r line; do log_verbose "$line"; done; then
        TEST_EXECUTOR=$(find_binary_or_exit "test-executor")
        pass "Built test-executor: $TEST_EXECUTOR"
    else
        fail "Failed to build test-executor"
        exit 1
    fi
else
    pass "Found test-executor: $TEST_EXECUTOR"
fi

# Find all test cases matching pattern
section "Discovering Test Cases"

TEST_FILES=()
while IFS= read -r -d '' yaml_file; do
    filename=$(basename "$yaml_file")
    if echo "$filename" | grep -qE "$TESTCASE_PATTERN"; then
        TEST_FILES+=("$yaml_file")
    fi
done < <(find "$TESTCASE_DIR" -type f \( -name "*.yml" -o -name "*.yaml" \) -print0 2>/dev/null)

log_info "Found ${#TEST_FILES[@]} test case(s) matching pattern '$TESTCASE_PATTERN'"

if [[ ${#TEST_FILES[@]} -eq 0 ]]; then
    log_warning "No test cases found matching pattern"
    exit 0
fi

echo ""
for test_file in "${TEST_FILES[@]}"; do
    log_verbose "  - $(basename "$test_file")"
done
echo ""

# Execute test cases
section "Executing Test Cases"

EXECUTION_SUCCESS=0
EXECUTION_FAILED=0
EXECUTION_ERROR=0

for test_file in "${TEST_FILES[@]}"; do
    test_basename=$(basename "$test_file" .yml)
    test_basename=$(basename "$test_basename" .yaml)
    
    log_info "[$((EXECUTION_SUCCESS + EXECUTION_FAILED + EXECUTION_ERROR + 1))/${#TEST_FILES[@]}] Executing: $test_basename"
    
    # Copy test case to campaign directory
    cp "$test_file" "$RUN_TESTCASE_DIR/"
    
    # Determine output log path
    log_file="$RUN_EXECUTION_DIR/${test_basename}_execution_log.json"
    
    # Run test executor
    log_verbose "Command: $TEST_EXECUTOR generate $test_file"
    
    # Generate and execute test script
    if $TEST_EXECUTOR generate "$test_file" 2>&1 | while IFS= read -r line; do log_verbose "$line"; done; then
        # Look for generated script
        generated_script="${test_file%.yml}.sh"
        generated_script="${generated_script%.yaml}.sh"
        
        if [[ -f "$generated_script" ]]; then
            log_verbose "Executing generated script: $generated_script"
            
            # Execute with JSON logging
            if bash "$generated_script" --json-log "$log_file" >/dev/null 2>&1; then
                pass "  Execution completed: $test_basename"
                EXECUTION_SUCCESS=$((EXECUTION_SUCCESS + 1))
            else
                EXIT_CODE=$?
                if [[ $EXIT_CODE -eq 1 ]]; then
                    pass "  Execution completed with failures: $test_basename"
                    EXECUTION_FAILED=$((EXECUTION_FAILED + 1))
                else
                    log_warning "  Execution error (exit code: $EXIT_CODE): $test_basename"
                    EXECUTION_ERROR=$((EXECUTION_ERROR + 1))
                fi
            fi
        else
            log_warning "  Generated script not found: $generated_script"
            EXECUTION_ERROR=$((EXECUTION_ERROR + 1))
        fi
    else
        log_warning "  Failed to generate script: $test_basename"
        EXECUTION_ERROR=$((EXECUTION_ERROR + 1))
    fi
    
    # Check if we should continue on error
    if [[ $CONTINUE_ON_ERROR -eq 0 ]] && [[ $EXECUTION_ERROR -gt 0 ]]; then
        fail "Stopping due to execution error (use --continue-on-error to continue)"
        break
    fi
done

echo ""
log_info "Execution Summary:"
log_info "  Total: ${#TEST_FILES[@]}"
log_info "  Success: $EXECUTION_SUCCESS"
log_info "  Failed: $EXECUTION_FAILED"
log_info "  Error: $EXECUTION_ERROR"
echo ""

# Run verification if not skipped
if [[ $SKIP_VERIFICATION -eq 0 ]]; then
    section "Running Verification"
    
    log_info "Building verifier binary..."
    VERIFIER=$(find_binary "verifier")
    if [[ -z "$VERIFIER" ]]; then
        log_info "Building verifier..."
        if cargo build --release --bin verifier 2>&1 | while IFS= read -r line; do log_verbose "$line"; done; then
            VERIFIER=$(find_binary_or_exit "verifier")
            pass "Built verifier: $VERIFIER"
        else
            fail "Failed to build verifier"
            exit 1
        fi
    else
        pass "Found verifier: $VERIFIER"
    fi
    
    VERIFICATION_OUTPUT_JSON="$CAMPAIGN_DIR/verification_results/${RUN_ID}_verification.json"
    VERIFICATION_OUTPUT_YAML="$CAMPAIGN_DIR/verification_results/${RUN_ID}_verification.yaml"
    
    log_info "Running verifier on execution logs..."
    log_verbose "Command: $VERIFIER --folder $RUN_EXECUTION_DIR --format json --output $VERIFICATION_OUTPUT_JSON"
    
    if $VERIFIER \
        --folder "$RUN_EXECUTION_DIR" \
        --format json \
        --output "$VERIFICATION_OUTPUT_JSON" \
        --test-case-dir "$RUN_TESTCASE_DIR" 2>&1 | while IFS= read -r line; do
            log_verbose "$line"
        done; then
        pass "Verification completed successfully"
    else
        VERIFIER_EXIT=$?
        if [[ $VERIFIER_EXIT -eq 1 ]]; then
            pass "Verification completed (some tests failed)"
        else
            log_warning "Verifier failed with unexpected exit code: $VERIFIER_EXIT"
        fi
    fi
    
    if [[ -f "$VERIFICATION_OUTPUT_JSON" ]]; then
        pass "Verification results: $VERIFICATION_OUTPUT_JSON"
        
        # Also generate YAML format
        if $VERIFIER \
            --folder "$RUN_EXECUTION_DIR" \
            --format yaml \
            --output "$VERIFICATION_OUTPUT_YAML" \
            --test-case-dir "$RUN_TESTCASE_DIR" >/dev/null 2>&1; then
            pass "YAML verification results: $VERIFICATION_OUTPUT_YAML"
        fi
    fi
else
    section "Verification Skipped"
    log_info "Verification was skipped (--skip-verification)"
fi

# Create run metadata
section "Recording Run Metadata"

RUN_METADATA="$CAMPAIGN_DIR/metadata/${RUN_ID}.yaml"
RUN_END_TIME=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

cat > "$RUN_METADATA" << EOF
# Test Run Metadata
run:
  id: "$RUN_ID"
  number: $RUN_NUMBER
  timestamp: "$RUN_TIMESTAMP"
  start_time: "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
  end_time: "$RUN_END_TIME"
  
configuration:
  testcase_pattern: "$TESTCASE_PATTERN"
  parallel_jobs: $PARALLEL_JOBS
  continue_on_error: $CONTINUE_ON_ERROR
  skip_verification: $SKIP_VERIFICATION
  
results:
  total_tests: ${#TEST_FILES[@]}
  execution_success: $EXECUTION_SUCCESS
  execution_failed: $EXECUTION_FAILED
  execution_error: $EXECUTION_ERROR
  
paths:
  testcases: "$RUN_TESTCASE_DIR"
  execution_logs: "$RUN_EXECUTION_DIR"
  verification_json: "$VERIFICATION_OUTPUT_JSON"
  verification_yaml: "$VERIFICATION_OUTPUT_YAML"
EOF

pass "Created run metadata: $RUN_METADATA"

# Final summary
section "Test Run Complete"
echo ""
info "Run Summary:"
echo "  Run ID: $RUN_ID"
echo "  Tests executed: ${#TEST_FILES[@]}"
echo "  Success: $EXECUTION_SUCCESS"
echo "  Failed: $EXECUTION_FAILED"
echo "  Error: $EXECUTION_ERROR"
echo ""
info "Output Locations:"
echo "  Test cases: $RUN_TESTCASE_DIR"
echo "  Execution logs: $RUN_EXECUTION_DIR"
if [[ $SKIP_VERIFICATION -eq 0 ]]; then
    echo "  Verification: $VERIFICATION_OUTPUT_JSON"
fi
echo ""
pass "Test run completed successfully!"

exit 0
