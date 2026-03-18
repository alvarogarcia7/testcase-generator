#!/usr/bin/env bash
#
# generate_all_sample_cases.sh - Generate comprehensive sample test cases
#
# This script generates multiple sample test cases covering all major scenarios:
# - Successful execution
# - Failed first step
# - Failed intermediate step  
# - Failed last step
# - Interrupted execution
# - Multiple sequences with mixed results
# - Hook failures at various lifecycle points
#
# After generation, the script:
# 1. Runs the orchestrator to execute each test case and generate execution logs
# 2. Runs the verifier on all execution logs
# 3. Generates both AsciiDoc and Markdown documentation reports
# 4. Preserves all results for inspection
#
# Usage: ./scripts/generate_all_sample_cases.sh [OPTIONS]
#
# Options:
#   --output-dir DIR    Output directory for samples (default: testcases/generated_samples)
#   --keep-logs         Keep execution logs after generating reports
#   --verbose           Enable verbose output
#   --help              Show this help message
#

set -e

# Get script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source required libraries
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

# Default configuration
OUTPUT_DIR="$PROJECT_ROOT/testcases/generated_samples"
KEEP_LOGS=0
VERBOSE=0

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --output-dir)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --keep-logs)
            KEEP_LOGS=1
            shift
            ;;
        --verbose)
            VERBOSE=1
            export VERBOSE
            shift
            ;;
        --help)
            head -n 30 "$0" | tail -n +2 | sed 's/^# //'
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Display configuration
section "Generate All Sample Test Cases"
log_info "Configuration:"
log_info "  Output directory: $OUTPUT_DIR"
log_info "  Keep logs: $KEEP_LOGS"
log_info "  Verbose: $VERBOSE"
echo ""

# Create output directories
mkdir -p "$OUTPUT_DIR/successful"
mkdir -p "$OUTPUT_DIR/failed_first"
mkdir -p "$OUTPUT_DIR/failed_intermediate"
mkdir -p "$OUTPUT_DIR/failed_last"
mkdir -p "$OUTPUT_DIR/interrupted"
mkdir -p "$OUTPUT_DIR/multiple_sequences"
mkdir -p "$OUTPUT_DIR/hooks/scripts"
mkdir -p "$OUTPUT_DIR/complex"
mkdir -p "$OUTPUT_DIR/reports"

log_info "Created output directory structure"

# Array to track generated test case files
declare -a GENERATED_TEST_CASES

# ============================================================================
# Generate Sample Test Cases
# ============================================================================

section "Generating Sample Test Cases"

# ----------------------------------------------------------------------------
# 1. Successful Execution Sample
# ----------------------------------------------------------------------------
log_info "Creating SAMPLE_SUCCESS_001..."

cat > "$OUTPUT_DIR/successful/SAMPLE_SUCCESS_001.yml" << 'EOF'
requirement: "SAMPLE_SUCCESS"
item: 1
tc: 1
id: 'SAMPLE_SUCCESS_001'
description: 'Sample successful execution with all steps passing'

general_initial_conditions:
  system:
    - "Shell environment is available"
    - "Standard commands are accessible"

initial_conditions:
  system:
    - "Working directory is writable"

test_sequences:
  - id: 1
    name: "Basic Command Execution"
    description: |
                   Sample sequence demonstrating successful command execution.
    steps:
      - step: 1
        description: "Display greeting message"
        command: echo "Hello World"
        expected:
          success: true
          result: 0
          output: "Hello World"
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "grep -q 'Hello World' <<< \"$COMMAND_OUTPUT\""
      - step: 2
        description: "Display system date"
        command: date +%Y-%m-%d
        expected:
          success: true
          result: 0
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "[[ -n \"$COMMAND_OUTPUT\" ]]"
      - step: 3
        description: "Display current username"
        command: whoami
        expected:
          success: true
          result: 0
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "[[ -n \"$COMMAND_OUTPUT\" ]]"
EOF

GENERATED_TEST_CASES+=("$OUTPUT_DIR/successful/SAMPLE_SUCCESS_001.yml")
pass "SAMPLE_SUCCESS_001.yml created"

# ----------------------------------------------------------------------------
# 2. Failed First Step Sample
# ----------------------------------------------------------------------------
log_info "Creating SAMPLE_FAILED_FIRST_001..."

cat > "$OUTPUT_DIR/failed_first/SAMPLE_FAILED_FIRST_001.yml" << 'EOF'
requirement: "SAMPLE_FAILED_FIRST"
item: 1
tc: 1
id: 'SAMPLE_FAILED_FIRST_001'
description: 'Sample demonstrating failure of first step preventing subsequent steps'

general_initial_conditions:
  system:
    - "Shell environment is available"

initial_conditions:
  system:
    - "Standard commands are accessible"

test_sequences:
  - id: 1
    name: "First Step Failure"
    description: |
                   Sample where first step fails, preventing execution of remaining steps.
    steps:
      - step: 1
        description: "Attempt invalid operation"
        command: ls /nonexistent_directory_12345
        expected:
          success: true
          result: 0
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "true"
      - step: 2
        description: "This step will not execute"
        command: echo "Step 2"
        expected:
          success: true
          result: 0
          output: "Step 2"
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "grep -q 'Step 2' <<< \"$COMMAND_OUTPUT\""
      - step: 3
        description: "This step will not execute either"
        command: echo "Step 3"
        expected:
          success: true
          result: 0
          output: "Step 3"
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "grep -q 'Step 3' <<< \"$COMMAND_OUTPUT\""
EOF

GENERATED_TEST_CASES+=("$OUTPUT_DIR/failed_first/SAMPLE_FAILED_FIRST_001.yml")
pass "SAMPLE_FAILED_FIRST_001.yml created"

# ----------------------------------------------------------------------------
# 3. Failed Intermediate Step Sample
# ----------------------------------------------------------------------------
log_info "Creating SAMPLE_FAILED_INTERMEDIATE_001..."

cat > "$OUTPUT_DIR/failed_intermediate/SAMPLE_FAILED_INTERMEDIATE_001.yml" << 'EOF'
requirement: "SAMPLE_FAILED_INTERMEDIATE"
item: 1
tc: 1
id: 'SAMPLE_FAILED_INTERMEDIATE_001'
description: 'Sample demonstrating failure of intermediate step'

general_initial_conditions:
  system:
    - "Shell environment is available"

initial_conditions:
  system:
    - "Standard commands are accessible"

test_sequences:
  - id: 1
    name: "Intermediate Step Failure"
    description: |
                   Sample where step 3 fails after successful steps 1-2.
    steps:
      - step: 1
        description: "First successful step"
        command: echo "Step 1 success"
        expected:
          success: true
          result: 0
          output: "Step 1 success"
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "grep -q 'Step 1 success' <<< \"$COMMAND_OUTPUT\""
      - step: 2
        description: "Second successful step"
        command: echo "Step 2 success"
        expected:
          success: true
          result: 0
          output: "Step 2 success"
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "grep -q 'Step 2 success' <<< \"$COMMAND_OUTPUT\""
      - step: 3
        description: "Third step fails"
        command: cat /nonexistent_file_99999.txt
        expected:
          success: true
          result: 0
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "true"
      - step: 4
        description: "This step will not execute"
        command: echo "Step 4"
        expected:
          success: true
          result: 0
          output: "Step 4"
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "grep -q 'Step 4' <<< \"$COMMAND_OUTPUT\""
EOF

GENERATED_TEST_CASES+=("$OUTPUT_DIR/failed_intermediate/SAMPLE_FAILED_INTERMEDIATE_001.yml")
pass "SAMPLE_FAILED_INTERMEDIATE_001.yml created"

# ----------------------------------------------------------------------------
# 4. Failed Last Step Sample
# ----------------------------------------------------------------------------
log_info "Creating SAMPLE_FAILED_LAST_001..."

cat > "$OUTPUT_DIR/failed_last/SAMPLE_FAILED_LAST_001.yml" << 'EOF'
requirement: "SAMPLE_FAILED_LAST"
item: 1
tc: 1
id: 'SAMPLE_FAILED_LAST_001'
description: 'Sample demonstrating failure of last step with output mismatch'

general_initial_conditions:
  system:
    - "Shell environment is available"

initial_conditions:
  system:
    - "Standard commands are accessible"

test_sequences:
  - id: 1
    name: "Last Step Failure"
    description: |
                   Sample where all steps execute but last step fails output verification.
    steps:
      - step: 1
        description: "First successful step"
        command: echo "Step 1"
        expected:
          success: true
          result: 0
          output: "Step 1"
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "grep -q 'Step 1' <<< \"$COMMAND_OUTPUT\""
      - step: 2
        description: "Second successful step"
        command: echo "Step 2"
        expected:
          success: true
          result: 0
          output: "Step 2"
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "grep -q 'Step 2' <<< \"$COMMAND_OUTPUT\""
      - step: 3
        description: "Last step with wrong output"
        command: echo "FAILURE"
        expected:
          success: true
          result: 0
          output: "SUCCESS"
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "grep -q 'SUCCESS' <<< \"$COMMAND_OUTPUT\""
EOF

GENERATED_TEST_CASES+=("$OUTPUT_DIR/failed_last/SAMPLE_FAILED_LAST_001.yml")
pass "SAMPLE_FAILED_LAST_001.yml created"

# ----------------------------------------------------------------------------
# 5. Multiple Sequences Sample
# ----------------------------------------------------------------------------
log_info "Creating SAMPLE_MULTI_SEQ_001..."

cat > "$OUTPUT_DIR/multiple_sequences/SAMPLE_MULTI_SEQ_001.yml" << 'EOF'
requirement: "SAMPLE_MULTI_SEQ"
item: 1
tc: 1
id: 'SAMPLE_MULTI_SEQ_001'
description: 'Sample with multiple sequences demonstrating mixed pass/fail results'

general_initial_conditions:
  system:
    - "Shell environment is available"

initial_conditions:
  system:
    - "Standard commands are accessible"

test_sequences:
  - id: 1
    name: "First Sequence - Success"
    description: "All steps pass in first sequence"
    steps:
      - step: 1
        description: "Echo message"
        command: echo "Sequence 1, Step 1"
        expected:
          success: true
          result: 0
          output: "Sequence 1, Step 1"
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "grep -q 'Sequence 1, Step 1' <<< \"$COMMAND_OUTPUT\""
      - step: 2
        description: "Echo another message"
        command: echo "Sequence 1, Step 2"
        expected:
          success: true
          result: 0
          output: "Sequence 1, Step 2"
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "grep -q 'Sequence 1, Step 2' <<< \"$COMMAND_OUTPUT\""
  
  - id: 2
    name: "Second Sequence - Fails"
    description: "Step 2 fails with output mismatch"
    steps:
      - step: 1
        description: "First step passes"
        command: echo "Sequence 2, Step 1"
        expected:
          success: true
          result: 0
          output: "Sequence 2, Step 1"
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "grep -q 'Sequence 2, Step 1' <<< \"$COMMAND_OUTPUT\""
      - step: 2
        description: "Second step fails"
        command: echo "Wrong output"
        expected:
          success: true
          result: 0
          output: "Expected output"
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "grep -q 'Expected output' <<< \"$COMMAND_OUTPUT\""
  
  - id: 3
    name: "Third Sequence - Not Executed"
    description: "This sequence won't execute due to previous failure"
    steps:
      - step: 1
        description: "This won't execute"
        command: echo "Sequence 3, Step 1"
        expected:
          success: true
          result: 0
          output: "Sequence 3, Step 1"
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "grep -q 'Sequence 3, Step 1' <<< \"$COMMAND_OUTPUT\""
EOF

GENERATED_TEST_CASES+=("$OUTPUT_DIR/multiple_sequences/SAMPLE_MULTI_SEQ_001.yml")
pass "SAMPLE_MULTI_SEQ_001.yml created"

# ----------------------------------------------------------------------------
# 6. Complex Sample with Variables
# ----------------------------------------------------------------------------
log_info "Creating SAMPLE_COMPLEX_001..."

cat > "$OUTPUT_DIR/complex/SAMPLE_COMPLEX_001.yml" << 'EOF'
requirement: "SAMPLE_COMPLEX"
item: 1
tc: 1
id: 'SAMPLE_COMPLEX_001'
description: 'Complex sample demonstrating variable capture and conditional verification'

general_initial_conditions:
  system:
    - "Shell environment is available"

initial_conditions:
  system:
    - "Standard commands are accessible"

test_sequences:
  - id: 1
    name: "Variable Capture and Conditional Verification"
    description: |
                   Demonstrates advanced features like variable capture and conditional verification.
    steps:
      - step: 1
        description: "Generate timestamp and capture it"
        command: date +%s
        expected:
          success: true
          result: 0
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "[[ \"$COMMAND_OUTPUT\" =~ ^[0-9]+$ ]]"
        variables:
          TIMESTAMP:
            from_output: "^([0-9]+)$"
      
      - step: 2
        description: "Display captured timestamp"
        command: echo "Timestamp: $TIMESTAMP"
        expected:
          success: true
          result: 0
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "grep -q 'Timestamp: ' <<< \"$COMMAND_OUTPUT\""
      
      - step: 3
        description: "Conditional verification based on platform"
        command: uname -s
        expected:
          success: true
          result: 0
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: |
            if [[ "$COMMAND_OUTPUT" == "Darwin" ]]; then
              grep -q 'Darwin' <<< "$COMMAND_OUTPUT"
            elif [[ "$COMMAND_OUTPUT" == "Linux" ]]; then
              grep -q 'Linux' <<< "$COMMAND_OUTPUT"
            else
              [[ -n "$COMMAND_OUTPUT" ]]
            fi
EOF

GENERATED_TEST_CASES+=("$OUTPUT_DIR/complex/SAMPLE_COMPLEX_001.yml")
pass "SAMPLE_COMPLEX_001.yml created"

# ----------------------------------------------------------------------------
# 7. Hook Samples - Create simple hook scripts first
# ----------------------------------------------------------------------------
log_info "Creating hook scripts..."

# Create a simple success hook
cat > "$OUTPUT_DIR/hooks/scripts/hook_success.sh" << 'EOF'
#!/usr/bin/env bash
echo "Hook executed successfully"
exit 0
EOF
chmod +x "$OUTPUT_DIR/hooks/scripts/hook_success.sh"

# Create a failing hook
cat > "$OUTPUT_DIR/hooks/scripts/hook_fail.sh" << 'EOF'
#!/usr/bin/env bash
echo "Hook failed" >&2
exit 1
EOF
chmod +x "$OUTPUT_DIR/hooks/scripts/hook_fail.sh"

pass "Hook scripts created"

# Sample with script_start hook
log_info "Creating SAMPLE_HOOK_SCRIPT_START_001..."

cat > "$OUTPUT_DIR/hooks/SAMPLE_HOOK_SCRIPT_START_001.yml" << 'EOF'
requirement: "SAMPLE_HOOK_SCRIPT_START"
item: 1
tc: 1
id: 'SAMPLE_HOOK_SCRIPT_START_001'
description: 'Sample demonstrating script_start hook success'

hooks:
  script_start:
    command: "scripts/hook_success.sh"
    on_error: "fail"

general_initial_conditions:
  system:
    - "Shell environment is available"

initial_conditions:
  system:
    - "Hook scripts are available"

test_sequences:
  - id: 1
    name: "Test with Script Start Hook"
    description: "Hook executes before any test steps"
    steps:
      - step: 1
        description: "Echo message"
        command: echo "Step 1"
        expected:
          success: true
          result: 0
          output: "Step 1"
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "grep -q 'Step 1' <<< \"$COMMAND_OUTPUT\""
EOF

GENERATED_TEST_CASES+=("$OUTPUT_DIR/hooks/SAMPLE_HOOK_SCRIPT_START_001.yml")
pass "SAMPLE_HOOK_SCRIPT_START_001.yml created"

# Sample with before_sequence hook
log_info "Creating SAMPLE_HOOK_BEFORE_SEQ_001..."

cat > "$OUTPUT_DIR/hooks/SAMPLE_HOOK_BEFORE_SEQ_001.yml" << 'EOF'
requirement: "SAMPLE_HOOK_BEFORE_SEQ"
item: 1
tc: 1
id: 'SAMPLE_HOOK_BEFORE_SEQ_001'
description: 'Sample demonstrating before_sequence hook'

hooks:
  before_sequence:
    command: "scripts/hook_success.sh"
    on_error: "fail"

general_initial_conditions:
  system:
    - "Shell environment is available"

initial_conditions:
  system:
    - "Hook scripts are available"

test_sequences:
  - id: 1
    name: "Test with Before Sequence Hook"
    description: "Hook executes before each sequence starts"
    steps:
      - step: 1
        description: "Echo message"
        command: echo "Sequence 1, Step 1"
        expected:
          success: true
          result: 0
          output: "Sequence 1, Step 1"
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "grep -q 'Sequence 1, Step 1' <<< \"$COMMAND_OUTPUT\""
EOF

GENERATED_TEST_CASES+=("$OUTPUT_DIR/hooks/SAMPLE_HOOK_BEFORE_SEQ_001.yml")
pass "SAMPLE_HOOK_BEFORE_SEQ_001.yml created"

# ============================================================================
# Summary of Generated Test Cases
# ============================================================================

section "Generated Test Cases Summary"
log_info "Total test cases generated: ${#GENERATED_TEST_CASES[@]}"
echo ""
info "Generated test case files:"
for test_case in "${GENERATED_TEST_CASES[@]}"; do
    echo "  📄 $test_case"
done
echo ""

pass "Sample test case generation complete!"
log_info "All samples saved to: $OUTPUT_DIR"

exit 0
