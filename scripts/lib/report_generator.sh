#!/usr/bin/env bash
#
# report_generator.sh - Library for building and invoking test-plan-doc-gen CLI with robust error handling
#
# DESCRIPTION:
#   This library provides functions to build test-plan-doc-gen from a sibling directory,
#   check if the binary is available, invoke it with proper arguments, and validate outputs.
#   
#   KEY FEATURES:
#   - Comprehensive error handling with detailed diagnostics
#   - Exit code validation and interpretation
#   - Automatic retry logic for transient failures (I/O errors)
#   - Output file validation (existence and content checks)
#   - Graceful degradation when tpdg is unavailable
#   - Helpful error messages for common failure scenarios
#   
#   ERROR HANDLING:
#   - Exit code 0: Success
#   - Exit code 1: General error (file not found, parsing error)
#   - Exit code 2: Invalid arguments
#   - Exit code 101: I/O error (retryable)
#   - Exit code 130: Interrupted (Ctrl+C)
#   
#   RETRY BEHAVIOR:
#   - Only I/O errors (exit code 101) are automatically retried
#   - Configurable via TPDG_MAX_RETRIES (default: 3) and TPDG_RETRY_DELAY (default: 2s)
#   - Use invoke_test_plan_doc_gen_with_retry() for automatic retry
#   - Use invoke_test_plan_doc_gen() for single attempt without retry
#
# USAGE:
#   source scripts/lib/report_generator.sh
#   
#   # Build test-plan-doc-gen binary
#   build_test_plan_doc_gen "/path/to/test-plan-doc-gen"
#   
#   # Check if binary is available
#   if check_test_plan_doc_gen_available; then
#       echo "Binary is available"
#   fi
#   
#   # Verify binary works correctly
#   if verify_test_plan_doc_gen_binary; then
#       echo "Binary is functional"
#   fi
#   
#   # Generate documentation (single attempt)
#   if invoke_test_plan_doc_gen --test-case "test.yaml" --output "output.md" --format markdown; then
#       echo "Report generated successfully"
#   else
#       echo "Report generation failed"
#   fi
#   
#   # Generate documentation from container YAML (single attempt)
#   if invoke_test_plan_doc_gen --input "container.yaml" --output "output.md" --format markdown; then
#       echo "Report generated from container successfully"
#   else
#       echo "Report generation from container failed"
#   fi
#   
#   # Generate documentation with automatic retry on transient failures
#   if invoke_test_plan_doc_gen_with_retry --test-case "test.yaml" --output "output.md" --format markdown; then
#       echo "Report generated (possibly after retries)"
#   else
#       echo "Report generation failed permanently"
#   fi
#   
#   # Validate output files exist and have valid content
#   if validate_report_output "output_dir" "report1.md" "report2.adoc"; then
#       echo "All output files are valid"
#   fi
#   
#   # Validate a single file's content
#   if validate_report_file_content "output.md" 100; then
#       echo "File has valid content (>= 100 bytes)"
#   fi
#   
#   # Customize retry behavior
#   TPDG_MAX_RETRIES=5 TPDG_RETRY_DELAY=3 invoke_test_plan_doc_gen_with_retry --test-case "test.yaml" --output "out.md"
#
# FUNCTIONS:
#   build_test_plan_doc_gen <sibling-directory-path>
#       Builds test-plan-doc-gen binary from source in sibling directory
#       Returns 0 on success, 1 on failure
#
#   check_test_plan_doc_gen_available [binary-path]
#       Checks if test-plan-doc-gen binary is available
#       Returns 0 if available, 1 if not
#
#   find_test_plan_doc_gen [sibling-directory-path]
#       Finds test-plan-doc-gen binary in sibling directory or PATH
#       Prints path to binary on stdout, returns 0 if found, 1 if not
#
#   invoke_test_plan_doc_gen <args...>
#       Invokes test-plan-doc-gen CLI with provided arguments
#       Returns exit code from test-plan-doc-gen
#       Includes detailed error handling and diagnostics
#
#   invoke_test_plan_doc_gen_with_retry <args...>
#       Invokes test-plan-doc-gen with automatic retry on transient failures
#       Returns 0 on success, 1 on permanent failure
#
#   validate_report_output <output-directory> <expected-file>...
#       Validates that expected output files were generated
#       Returns 0 if all files exist, 1 if any are missing
#
#   validate_report_file_content <file-path> [min-size-bytes]
#       Validates that a report file exists and has valid content
#       Returns 0 if file is valid, 1 if invalid
#
#   get_tpdg_error_message <exit-code>
#       Returns a human-readable error message for a tpdg exit code
#       Prints error message on stdout
#
#   is_transient_error <exit-code>
#       Checks if an exit code represents a transient error
#       Returns 0 if error is transient, 1 if permanent
#
#   verify_test_plan_doc_gen_binary [binary-path]
#       Verifies that test-plan-doc-gen binary works correctly
#       Returns 0 if binary works, 1 if not
#

# Get the directory of this script
_REPORT_GEN_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source the logging library
if [[ -f "$_REPORT_GEN_SCRIPT_DIR/logger.sh" ]]; then
    source "$_REPORT_GEN_SCRIPT_DIR/logger.sh"
else
    echo "[ERROR] Cannot find logger.sh library" >&2
    exit 1
fi

# Configuration for retry logic
TPDG_MAX_RETRIES="${TPDG_MAX_RETRIES:-3}"
TPDG_RETRY_DELAY="${TPDG_RETRY_DELAY:-2}"

# Exit codes for test-plan-doc-gen (based on typical Rust CLI conventions)
# 0   = Success
# 1   = General error (file not found, parsing error, etc.)
# 2   = Invalid arguments or usage
# 101 = I/O error
# 130 = Interrupted (Ctrl+C)

# Get human-readable error message for tpdg exit code
# Arguments:
#   $1 - Exit code (required)
# Returns:
#   Prints error message on stdout
get_tpdg_error_message() {
    local exit_code="$1"
    
    case "$exit_code" in
        0)
            echo "Success"
            ;;
        1)
            echo "General error (file not found, parsing error, or invalid input)"
            ;;
        2)
            echo "Invalid command-line arguments or usage error"
            ;;
        101)
            echo "I/O error (failed to read input or write output)"
            ;;
        130)
            echo "Interrupted by user (Ctrl+C)"
            ;;
        *)
            echo "Unknown error (exit code: $exit_code)"
            ;;
    esac
}

# Check if an exit code represents a transient error that can be retried
# Arguments:
#   $1 - Exit code (required)
# Returns:
#   0 if error is transient (can retry), 1 if permanent error
is_transient_error() {
    local exit_code="$1"
    
    # Only I/O errors (101) are considered transient
    # Other errors like parsing errors, invalid arguments are permanent
    case "$exit_code" in
        101)
            return 0
            ;;
        *)
            return 1
            ;;
    esac
}

# Build test-plan-doc-gen binary from sibling directory
# Arguments:
#   $1 - Path to test-plan-doc-gen sibling directory (required)
# Returns:
#   0 on success, 1 on failure
build_test_plan_doc_gen() {
    local sibling_dir="$1"
    
    if [[ -z "$sibling_dir" ]]; then
        log_error "build_test_plan_doc_gen: sibling directory path required"
        return 1
    fi
    
    if [[ ! -d "$sibling_dir" ]]; then
        log_error "build_test_plan_doc_gen: directory not found: $sibling_dir"
        log_error "Please ensure test-plan-doc-gen is cloned to the correct location"
        return 1
    fi
    
    log_info "Building test-plan-doc-gen from: $sibling_dir"
    
    # Check if Cargo.toml exists
    if [[ ! -f "$sibling_dir/Cargo.toml" ]]; then
        log_error "build_test_plan_doc_gen: Cargo.toml not found in $sibling_dir"
        log_error "Directory may not be a valid test-plan-doc-gen repository"
        return 1
    fi
    
    # Verify this is actually the test-plan-doc-gen project
    if ! grep -q "test-plan-doc-gen" "$sibling_dir/Cargo.toml" 2>/dev/null; then
        log_warning "build_test_plan_doc_gen: Cargo.toml may not be from test-plan-doc-gen project"
    fi
    
    # Build the binary
    log_verbose "Running: cargo build --release --manifest-path $sibling_dir/Cargo.toml"
    
    local build_output
    local build_exit
    
    build_output=$(cargo build --release --manifest-path "$sibling_dir/Cargo.toml" 2>&1)
    build_exit=$?
    
    # Log build output
    echo "$build_output" | while IFS= read -r line; do
        log_verbose "$line"
    done
    
    if [[ $build_exit -eq 0 ]]; then
        # Verify the binary was actually created (check for both tpdg and test-plan-doc-gen)
        if [[ -f "$sibling_dir/target/release/tpdg" ]]; then
            pass "test-plan-doc-gen built successfully"
            log_debug "Binary location: $sibling_dir/target/release/tpdg"
            return 0
        elif [[ -f "$sibling_dir/target/release/test-plan-doc-gen" ]]; then
            pass "test-plan-doc-gen built successfully"
            log_debug "Binary location: $sibling_dir/target/release/test-plan-doc-gen"
            return 0
        else
            fail "Build succeeded but binary not found at expected location"
            log_error "Expected binary at: $sibling_dir/target/release/tpdg or $sibling_dir/target/release/test-plan-doc-gen"
            return 1
        fi
    else
        fail "Failed to build test-plan-doc-gen (exit code: $build_exit)"
        
        # Try to extract useful error information
        if echo "$build_output" | grep -q "error:"; then
            log_error "Build errors detected:"
            echo "$build_output" | grep "error:" | head -5 | while IFS= read -r line; do
                log_error "  $line"
            done
        fi
        
        return 1
    fi
}

# Find test-plan-doc-gen binary
# Arguments:
#   $1 - Path to test-plan-doc-gen sibling directory (optional)
# Returns:
#   Path to binary on stdout, 0 if found, 1 if not found
find_test_plan_doc_gen() {
    local sibling_dir="${1:-}"
    
    # Check environment variable first
    if [[ -n "${TEST_PLAN_DOC_GEN:-}" ]]; then
        if [[ -x "$TEST_PLAN_DOC_GEN" ]]; then
            echo "$TEST_PLAN_DOC_GEN"
            return 0
        else
            log_warning "TEST_PLAN_DOC_GEN is set but binary is not executable: $TEST_PLAN_DOC_GEN"
        fi
    fi
    
    # Check sibling directory if provided
    if [[ -n "$sibling_dir" ]]; then
        # Check release build (try both tpdg and test-plan-doc-gen)
        if [[ -x "$sibling_dir/target/release/tpdg" ]]; then
            echo "$sibling_dir/target/release/tpdg"
            return 0
        fi
        if [[ -x "$sibling_dir/target/release/test-plan-doc-gen" ]]; then
            echo "$sibling_dir/target/release/test-plan-doc-gen"
            return 0
        fi
        
        # Check debug build (try both tpdg and test-plan-doc-gen)
        if [[ -x "$sibling_dir/target/debug/tpdg" ]]; then
            echo "$sibling_dir/target/debug/tpdg"
            return 0
        fi
        if [[ -x "$sibling_dir/target/debug/test-plan-doc-gen" ]]; then
            echo "$sibling_dir/target/debug/test-plan-doc-gen"
            return 0
        fi
    fi
    
    # Check system PATH (try both tpdg and test-plan-doc-gen)
    if command -v tpdg >/dev/null 2>&1; then
        echo "tpdg"
        return 0
    fi
    if command -v test-plan-doc-gen >/dev/null 2>&1; then
        echo "test-plan-doc-gen"
        return 0
    fi
    
    # Not found
    return 1
}

# Check if test-plan-doc-gen binary is available
# Arguments:
#   $1 - Path to test-plan-doc-gen sibling directory (optional)
# Returns:
#   0 if available, 1 if not available
check_test_plan_doc_gen_available() {
    local sibling_dir="${1:-}"
    local binary_path
    
    binary_path=$(find_test_plan_doc_gen "$sibling_dir")
    local find_result=$?
    
    if [[ $find_result -eq 0 ]] && [[ -n "$binary_path" ]]; then
        log_debug "test-plan-doc-gen found at: $binary_path"
        
#        # Verify the binary is executable
#        if [[ ! -x "$binary_path" ]]; then
#            log_error "test-plan-doc-gen found but is not executable: $binary_path"
#            return 1
#        fi
        
        return 0
    else
        log_debug "test-plan-doc-gen binary not found"
        return 1
    fi
}

# Validate that a report file has valid content
# Arguments:
#   $1 - File path (required)
#   $2 - Minimum size in bytes (optional, default: 10)
# Returns:
#   0 if file is valid, 1 if invalid
validate_report_file_content() {
    local file_path="$1"
    local min_size="${2:-10}"
    
    if [[ -z "$file_path" ]]; then
        log_error "validate_report_file_content: file path required"
        return 1
    fi
    
    if [[ ! -f "$file_path" ]]; then
        log_error "validate_report_file_content: file not found: $file_path"
        return 1
    fi
    
    # Check file size (using portable stat command)
    local file_size
    file_size=$(stat -f%z "$file_path" 2>/dev/null || stat -c%s "$file_path" 2>/dev/null || echo "0")
    
    if [[ "$file_size" -lt "$min_size" ]]; then
        log_error "validate_report_file_content: file is too small ($file_size bytes, minimum: $min_size bytes)"
        log_error "File may be empty or truncated: $file_path"
        return 1
    fi
    
    log_debug "File size validation passed: $file_size bytes"
    
    # Check for common file format markers based on extension
    local extension="${file_path##*.}"
    
    case "$extension" in
        md|markdown)
            # Markdown files should have some text content
            if ! grep -q '[a-zA-Z]' "$file_path" 2>/dev/null; then
                log_error "validate_report_file_content: Markdown file appears to be empty or invalid"
                return 1
            fi
            log_debug "Markdown content validation passed"
            ;;
        adoc|asciidoc)
            # AsciiDoc files should have some text content
            if ! grep -q '[a-zA-Z]' "$file_path" 2>/dev/null; then
                log_error "validate_report_file_content: AsciiDoc file appears to be empty or invalid"
                return 1
            fi
            log_debug "AsciiDoc content validation passed"
            ;;
        *)
            # For other formats, just check for any non-whitespace content
            if ! grep -q '[^[:space:]]' "$file_path" 2>/dev/null; then
                log_error "validate_report_file_content: file appears to contain only whitespace"
                return 1
            fi
            log_debug "Generic content validation passed"
            ;;
    esac
    
    return 0
}

# Invoke test-plan-doc-gen CLI with provided arguments
# Arguments:
#   $@ - All arguments to pass to test-plan-doc-gen
# Returns:
#   Exit code from test-plan-doc-gen
invoke_test_plan_doc_gen() {
    local binary_path
    
    # Try to find the binary
    binary_path=$(find_test_plan_doc_gen)
    
    if [[ -z "$binary_path" ]]; then
        log_error "test-plan-doc-gen binary not found"
        log_error "Please build it first using build_test_plan_doc_gen()"
        log_error "Or set TEST_PLAN_DOC_GEN environment variable to the binary path"
        return 1
    fi
    
    log_verbose "Binary path: $binary_path"
    log_verbose "Full command: $binary_path $*"
    log_info "Invoking test-plan-doc-gen..."
    
    # Validate input files exist before calling tpdg
    local checking_container=0
    local checking_testcase=0
    local container_files=()
    local testcase_files=()
    
    for arg in "$@"; do
        if [[ "$arg" == "--container" ]] || [[ "$arg" == "--input" ]]; then
            checking_container=1
            checking_testcase=0
            continue
        elif [[ "$arg" == "--test-case" ]]; then
            checking_container=0
            checking_testcase=1
            continue
        elif [[ "$arg" == "--format" ]] || [[ "$arg" == "--output" ]]; then
            checking_container=0
            checking_testcase=0
            continue
        fi
        
        if [[ $checking_container -eq 1 ]] && [[ ! "$arg" =~ ^-- ]]; then
            container_files+=("$arg")
        elif [[ $checking_testcase -eq 1 ]] && [[ ! "$arg" =~ ^-- ]]; then
            testcase_files+=("$arg")
        fi
    done
    
    # Validate container files
    for file in "${container_files[@]}"; do
        if [[ ! -f "$file" ]] && [[ ! -d "$file" ]]; then
            log_error "Container file/directory does not exist: $file"
            return 1
        fi
        log_verbose "Container input: $file ($(if [[ -f "$file" ]]; then echo "file"; else echo "directory"; fi))"
    done
    
    # Validate test case files
    for file in "${testcase_files[@]}"; do
        if [[ ! -f "$file" ]] && [[ ! -d "$file" ]]; then
            log_error "Test case file/directory does not exist: $file"
            return 1
        fi
        log_verbose "Test case input: $file ($(if [[ -f "$file" ]]; then echo "file"; else echo "directory"; fi))"
    done
    
    # Capture both stdout and stderr
    local tpdg_output
    local tpdg_exit
    
    tpdg_output=$("$binary_path" "$@" 2>&1)
    tpdg_exit=$?
    
    # Log the output at verbose level
    if [[ -n "$tpdg_output" ]]; then
        log_verbose "=== tpdg output (exit code: $tpdg_exit) ==="
        echo "$tpdg_output" | while IFS= read -r line; do
            log_verbose "$line"
        done
        log_verbose "=== end tpdg output ==="
    fi
    
    if [[ $tpdg_exit -eq 0 ]]; then
        pass "test-plan-doc-gen completed successfully"
        
        # Validate output file if --output argument was provided
        local output_file=""
        local next_is_output=0
        
        for arg in "$@"; do
            if [[ $next_is_output -eq 1 ]]; then
                output_file="$arg"
                break
            fi
            if [[ "$arg" == "--output" ]] || [[ "$arg" == "-o" ]]; then
                next_is_output=1
            fi
        done
        
        if [[ -n "$output_file" ]]; then
            log_debug "Validating output file: $output_file"
            
            if validate_report_file_content "$output_file"; then
                log_debug "Output file validation passed: $output_file"
            else
                log_warning "Output file validation failed, but tpdg reported success"
                log_warning "File may be incomplete or corrupted: $output_file"
                # Don't fail here as tpdg reported success - just warn
            fi
        fi
        
        return 0
    else
        local error_msg
        error_msg=$(get_tpdg_error_message "$tpdg_exit")
        
        fail "test-plan-doc-gen failed with exit code: $tpdg_exit"
        log_error "Error: $error_msg"
        
        # Provide contextual error information
        case "$tpdg_exit" in
            1)
                log_error "Common causes:"
                log_error "  - Input file not found or not readable"
                log_error "  - YAML parsing error in input file"
                log_error "  - Invalid YAML structure or missing required fields"
                
                # Try to extract specific error from output
                if echo "$tpdg_output" | grep -qi "no such file"; then
                    log_error "  → Input file does not exist"
                elif echo "$tpdg_output" | grep -qi "permission denied"; then
                    log_error "  → Permission denied reading input file"
                elif echo "$tpdg_output" | grep -qi "yaml"; then
                    log_error "  → YAML parsing error detected"
                fi
                ;;
            2)
                log_error "Common causes:"
                log_error "  - Missing required arguments (--input/--container, --test-case, --output)"
                log_error "  - Invalid argument format or combination"
                log_error "  - Conflicting options provided"
                log_error ""
                log_error "Please check the command-line arguments"
                ;;
            101)
                log_error "I/O error occurred - this may be a transient issue"
                log_error "Common causes:"
                log_error "  - Disk full or quota exceeded"
                log_error "  - Output directory not writable"
                log_error "  - Network filesystem temporarily unavailable"
                log_error ""
                log_error "Suggestion: retry the operation"
                ;;
            130)
                log_error "Operation was interrupted by user (Ctrl+C)"
                ;;
        esac
        
        # Error details are in verbose output above
        log_error ""
        log_error "Run with --verbose flag to see detailed error output from test-plan-doc-gen"
        
        return $tpdg_exit
    fi
}

# Invoke test-plan-doc-gen with automatic retry on transient failures
# Arguments:
#   $@ - All arguments to pass to test-plan-doc-gen
# Returns:
#   0 on success, 1 on permanent failure
invoke_test_plan_doc_gen_with_retry() {
    local max_retries="$TPDG_MAX_RETRIES"
    local retry_delay="$TPDG_RETRY_DELAY"
    local attempt=1
    
    log_debug "invoke_test_plan_doc_gen_with_retry: max_retries=$max_retries, retry_delay=$retry_delay"
    
    while [[ $attempt -le $max_retries ]]; do
        if [[ $attempt -gt 1 ]]; then
            log_info "Retry attempt $attempt of $max_retries"
        fi
        
        # Try to invoke test-plan-doc-gen
        invoke_test_plan_doc_gen "$@"
        local exit_code=$?
        
        if [[ $exit_code -eq 0 ]]; then
            # Success
            return 0
        fi
        
        # Check if error is transient
        if is_transient_error "$exit_code"; then
            if [[ $attempt -lt $max_retries ]]; then
                log_warning "Transient error detected (exit code: $exit_code)"
                log_info "Retrying in ${retry_delay}s... (attempt $(( attempt + 1 )) of $max_retries)"
                sleep "$retry_delay"
                ((attempt++))
            else
                log_error "Maximum retry attempts reached ($max_retries)"
                log_error "Giving up after transient error (exit code: $exit_code)"
                return 1
            fi
        else
            # Permanent error, don't retry
            log_error "Permanent error detected (exit code: $exit_code)"
            log_error "Not retrying"
            return 1
        fi
    done
    
    # Should not reach here, but just in case
    log_error "Retry logic exhausted unexpectedly"
    return 1
}

# Validate that expected output files were generated
# Arguments:
#   $1 - Output directory path (required)
#   $@ - Expected file paths relative to output directory (required, at least one)
# Returns:
#   0 if all files exist and have valid content, 1 if any are missing or invalid
validate_report_output() {
    local output_dir="$1"
    shift
    
    if [[ -z "$output_dir" ]]; then
        log_error "validate_report_output: output directory required"
        return 1
    fi
    
    if [[ ! -d "$output_dir" ]]; then
        log_error "validate_report_output: output directory not found: $output_dir"
        log_error "Directory may not have been created or path is incorrect"
        return 1
    fi
    
    if [[ $# -eq 0 ]]; then
        log_error "validate_report_output: at least one expected file required"
        return 1
    fi
    
    local all_found=0
    local missing_count=0
    local invalid_count=0
    
    log_info "Validating output files in: $output_dir"
    
    for expected_file in "$@"; do
        local full_path="$output_dir/$expected_file"
        
        if [[ -f "$full_path" ]]; then
            # File exists, now validate content
            if validate_report_file_content "$full_path"; then
                pass "Found and validated: $expected_file"
                
                local file_size
                file_size=$(stat -f%z "$full_path" 2>/dev/null || stat -c%s "$full_path" 2>/dev/null || echo "unknown")
                log_debug "File size: $file_size bytes"
            else
                fail "Invalid content: $expected_file"
                ((invalid_count++))
                all_found=1
            fi
        else
            fail "Missing: $expected_file"
            ((missing_count++))
            all_found=1
        fi
    done
    
    if [[ $all_found -eq 0 ]]; then
        pass "All expected output files found and validated"
        return 0
    else
        if [[ $missing_count -gt 0 ]]; then
            fail "Missing $missing_count output file(s)"
        fi
        if [[ $invalid_count -gt 0 ]]; then
            fail "Invalid $invalid_count output file(s)"
        fi
        
        log_error "Output validation failed"
        log_error "Possible causes:"
        log_error "  - test-plan-doc-gen failed to generate some files"
        log_error "  - Output files were truncated or corrupted"
        log_error "  - Incorrect output path or permissions issue"
        
        return 1
    fi
}

# Verify test-plan-doc-gen binary functionality
# Arguments:
#   $1 - Binary path (optional, will search if not provided)
# Returns:
#   0 if binary works, 1 if not
verify_test_plan_doc_gen_binary() {
    local binary_path="${1:-}"
    
    if [[ -z "$binary_path" ]]; then
        binary_path=$(find_test_plan_doc_gen)
        if [[ -z "$binary_path" ]]; then
            log_error "verify_test_plan_doc_gen_binary: binary not found"
            return 1
        fi
    fi
    
    if [[ ! -x "$binary_path" ]]; then
        log_error "verify_test_plan_doc_gen_binary: not executable: $binary_path"
        return 1
    fi
    
    log_debug "Verifying test-plan-doc-gen binary: $binary_path"
    
    # Try to get version or help output
    local test_output
    test_output=$("$binary_path" --help 2>&1)
    local test_exit=$?
    
    if [[ $test_exit -eq 0 ]] || [[ $test_exit -eq 2 ]]; then
        # Exit code 0 or 2 (usage error) is expected for --help
        if echo "$test_output" | grep -qi "test-plan-doc-gen\|tpdg\|usage\|--help"; then
            log_debug "Binary verification passed"
            return 0
        else
            log_warning "Binary responded but output is unexpected"
            log_debug "Output: $test_output"
            return 1
        fi
    else
        log_error "Binary verification failed (exit code: $test_exit)"
        return 1
    fi
}

# Export functions for use in other scripts
export -f build_test_plan_doc_gen
export -f check_test_plan_doc_gen_available
export -f find_test_plan_doc_gen
export -f invoke_test_plan_doc_gen
export -f invoke_test_plan_doc_gen_with_retry
export -f validate_report_output
export -f validate_report_file_content
export -f get_tpdg_error_message
export -f is_transient_error
export -f verify_test_plan_doc_gen_binary
