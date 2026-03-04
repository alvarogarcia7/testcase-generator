#!/usr/bin/env bash
#
# report_generator.sh - Library for building and invoking test-plan-doc-gen CLI
#
# DESCRIPTION:
#   This library provides functions to build test-plan-doc-gen from a sibling directory,
#   check if the binary is available, invoke it with proper arguments, and validate outputs.
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
#   # Generate documentation
#   invoke_test_plan_doc_gen --container "container.yaml" --test-case "test_case.yaml" --output "output_dir"
#   
#   # Validate output files
#   validate_report_output "output_dir" "expected_file.md"
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
#
#   validate_report_output <output-directory> <expected-file>...
#       Validates that expected output files were generated
#       Returns 0 if all files exist, 1 if any are missing
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
        return 1
    fi
    
    log_info "Building test-plan-doc-gen from: $sibling_dir"
    
    # Check if Cargo.toml exists
    if [[ ! -f "$sibling_dir/Cargo.toml" ]]; then
        log_error "build_test_plan_doc_gen: Cargo.toml not found in $sibling_dir"
        return 1
    fi
    
    # Build the binary
    log_verbose "Running: cargo build --release --manifest-path $sibling_dir/Cargo.toml"
    
    if cargo build --release --manifest-path "$sibling_dir/Cargo.toml" 2>&1 | while IFS= read -r line; do
        log_verbose "$line"
    done; then
        pass "test-plan-doc-gen built successfully"
        return 0
    else
        fail "Failed to build test-plan-doc-gen"
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
        fi
    fi
    
    # Check sibling directory if provided
    if [[ -n "$sibling_dir" ]]; then
        # Check release build
        if [[ -x "$sibling_dir/target/release/test-plan-doc-gen" ]]; then
            echo "$sibling_dir/target/release/test-plan-doc-gen"
            return 0
        fi
        
        # Check debug build
        if [[ -x "$sibling_dir/target/debug/test-plan-doc-gen" ]]; then
            echo "$sibling_dir/target/debug/test-plan-doc-gen"
            return 0
        fi
    fi
    
    # Check system PATH
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
        return 0
    else
        log_debug "test-plan-doc-gen binary not found"
        return 1
    fi
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
        return 1
    fi
    
    log_info "Invoking test-plan-doc-gen: $binary_path $*"
    
    # Execute test-plan-doc-gen with all provided arguments
    if "$binary_path" "$@"; then
        pass "test-plan-doc-gen completed successfully"
        return 0
    else
        local exit_code=$?
        fail "test-plan-doc-gen failed with exit code: $exit_code"
        return $exit_code
    fi
}

# Validate that expected output files were generated
# Arguments:
#   $1 - Output directory path (required)
#   $@ - Expected file paths relative to output directory (required, at least one)
# Returns:
#   0 if all files exist, 1 if any are missing
validate_report_output() {
    local output_dir="$1"
    shift
    
    if [[ -z "$output_dir" ]]; then
        log_error "validate_report_output: output directory required"
        return 1
    fi
    
    if [[ ! -d "$output_dir" ]]; then
        log_error "validate_report_output: output directory not found: $output_dir"
        return 1
    fi
    
    if [[ $# -eq 0 ]]; then
        log_error "validate_report_output: at least one expected file required"
        return 1
    fi
    
    local all_found=0
    local missing_count=0
    
    log_info "Validating output files in: $output_dir"
    
    for expected_file in "$@"; do
        local full_path="$output_dir/$expected_file"
        
        if [[ -f "$full_path" ]]; then
            pass "Found: $expected_file"
            log_debug "File size: $(stat -f%z "$full_path" 2>/dev/null || stat -c%s "$full_path" 2>/dev/null || echo "unknown") bytes"
        else
            fail "Missing: $expected_file"
            ((missing_count++))
            all_found=1
        fi
    done
    
    if [[ $all_found -eq 0 ]]; then
        pass "All expected output files found"
        return 0
    else
        fail "Missing $missing_count output file(s)"
        return 1
    fi
}

# Export functions for use in other scripts
export -f build_test_plan_doc_gen
export -f check_test_plan_doc_gen_available
export -f find_test_plan_doc_gen
export -f invoke_test_plan_doc_gen
export -f validate_report_output
