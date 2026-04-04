#!/usr/bin/env bash
#
# generate_final_report.sh - Generates comprehensive final report for acceptance test suite
#
# DESCRIPTION:
#   This script aggregates results from all 6 validation stage scripts and generates
#   comprehensive reports in HTML, Markdown, and JSON formats. It includes:
#   - Summary statistics for each stage (YAML, scripts, execution, verification, TPDG docs)
#   - Visual status indicators (green/red badges for each stage)
#   - Execution timeline showing duration of each stage
#   - Test coverage matrix showing which test scenario categories were covered
#   - Detailed error messages and failure information
#   - Exportable formats: HTML, Markdown, and JSON
#
# USAGE:
#   ./test-acceptance/generate_final_report.sh [OPTIONS]
#
# OPTIONS:
#   -v, --verbose         Enable verbose output
#   -h, --help            Show this help message
#   -o, --output-dir DIR  Output directory for reports (default: 30_documentation_source/reports/final)
#   --skip-html           Skip HTML report generation
#   --skip-markdown       Skip Markdown report generation
#   --skip-json           Skip JSON report generation
#   --stage1-log FILE     Path to Stage 1 validation log
#   --stage2-log FILE     Path to Stage 2 validation log
#   --stage3-log FILE     Path to Stage 3 validation log
#   --stage4-log FILE     Path to Stage 4 validation log
#   --stage5-log FILE     Path to Stage 5 validation log
#   --stage6-log FILE     Path to Stage 6 validation log
#
# EXIT CODES:
#   0 - Report generation succeeded
#   1 - Report generation failed
#
# OUTPUT:
#   Generates reports in the output directory:
#   - final_report.html - Comprehensive HTML report
#   - final_report.md - Markdown summary report
#   - final_report.json - Detailed JSON data for programmatic analysis
#

set -euo pipefail

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source logger library
source "$REPO_ROOT/scripts/lib/logger.sh" || exit 1

# Configuration
OUTPUT_DIR="$SCRIPT_DIR/30_documentation_source/reports/final"
VERBOSE=0
SKIP_HTML=0
SKIP_MARKDOWN=0
SKIP_JSON=0

# Stage log file paths (default locations)
STAGE1_LOG=""
STAGE2_LOG=""
STAGE3_LOG=""
STAGE4_LOG=""
STAGE5_LOG=""
STAGE6_LOG=""

# Timing data
START_TIME=$(date +%s)

# Usage function
usage() {
    cat << EOF
Usage: $(basename "$0") [OPTIONS]

Generates comprehensive final report for acceptance test suite.

OPTIONS:
    -v, --verbose         Enable verbose output
    -h, --help            Show this help message
    -o, --output-dir DIR  Output directory for reports (default: 30_documentation_source/reports/final)
    --skip-html           Skip HTML report generation
    --skip-markdown       Skip Markdown report generation
    --skip-json           Skip JSON report generation
    --stage1-log FILE     Path to Stage 1 validation log
    --stage2-log FILE     Path to Stage 2 validation log
    --stage3-log FILE     Path to Stage 3 validation log
    --stage4-log FILE     Path to Stage 4 validation log
    --stage5-log FILE     Path to Stage 5 validation log
    --stage6-log FILE     Path to Stage 6 validation log

DESCRIPTION:
    Aggregates results from all 6 validation stages and generates comprehensive
    reports in HTML, Markdown, and JSON formats.

EXIT CODES:
    0 - Report generation succeeded
    1 - Report generation failed

EOF
    exit 0
}

# Parse command-line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -v|--verbose)
                VERBOSE=1
                shift
                ;;
            -h|--help)
                usage
                ;;
            -o|--output-dir)
                OUTPUT_DIR="$2"
                shift 2
                ;;
            --skip-html)
                SKIP_HTML=1
                shift
                ;;
            --skip-markdown)
                SKIP_MARKDOWN=1
                shift
                ;;
            --skip-json)
                SKIP_JSON=1
                shift
                ;;
            --stage1-log)
                STAGE1_LOG="$2"
                shift 2
                ;;
            --stage2-log)
                STAGE2_LOG="$2"
                shift 2
                ;;
            --stage3-log)
                STAGE3_LOG="$2"
                shift 2
                ;;
            --stage4-log)
                STAGE4_LOG="$2"
                shift 2
                ;;
            --stage5-log)
                STAGE5_LOG="$2"
                shift 2
                ;;
            --stage6-log)
                STAGE6_LOG="$2"
                shift 2
                ;;
            *)
                log_error "Unknown option: $1"
                usage
                ;;
        esac
    done
}

# Extract statistics from stage validation output
extract_stage_stats() {
    local stage_num="$1"
    local log_content="$2"
    
    # Initialize variables for this stage
    eval "STAGE${stage_num}_TOTAL=0"
    eval "STAGE${stage_num}_PASSED=0"
    eval "STAGE${stage_num}_FAILED=0"
    eval "STAGE${stage_num}_SKIPPED=0"
    eval "STAGE${stage_num}_STATUS=UNKNOWN"
    eval "STAGE${stage_num}_ERRORS=''"
    
    # Parse the log content to extract statistics
    # Look for patterns like "Total files: 10", "Passed: 8", "Failed: 2"
    
    local total_line
    total_line=$(echo "$log_content" | grep -i "total" | grep -E "[0-9]+" | head -1 || true)
    if [[ -n "$total_line" ]]; then
        local total
        total=$(echo "$total_line" | grep -oE "[0-9]+" | head -1 || echo "0")
        eval "STAGE${stage_num}_TOTAL=$total"
    fi
    
    # Extract passed count
    local passed_sum=0
    while IFS= read -r line; do
        if [[ -n "$line" ]]; then
            passed_sum=$((passed_sum + line))
        fi
    done < <(echo "$log_content" | grep -i "passed:" | grep -oE "[0-9]+" || true)
    eval "STAGE${stage_num}_PASSED=$passed_sum"
    
    # Extract failed count
    local failed_sum=0
    while IFS= read -r line; do
        if [[ -n "$line" ]]; then
            failed_sum=$((failed_sum + line))
        fi
    done < <(echo "$log_content" | grep -i "failed:" | grep -oE "[0-9]+" || true)
    eval "STAGE${stage_num}_FAILED=$failed_sum"
    
    # Extract skipped count
    local skipped_sum=0
    while IFS= read -r line; do
        if [[ -n "$line" ]]; then
            skipped_sum=$((skipped_sum + line))
        fi
    done < <(echo "$log_content" | grep -i "skipped:" | grep -oE "[0-9]+" || true)
    eval "STAGE${stage_num}_SKIPPED=$skipped_sum"
    
    # Determine overall status
    if echo "$log_content" | grep -qi "all validations passed\|validation result.*passed"; then
        eval "STAGE${stage_num}_STATUS=PASSED"
    elif echo "$log_content" | grep -qi "failed\|error"; then
        eval "STAGE${stage_num}_STATUS=FAILED"
    else
        eval "STAGE${stage_num}_STATUS=UNKNOWN"
    fi
    
    # Extract error messages
    local errors
    errors=$(echo "$log_content" | grep -E "✗|ERROR|FAILED" | head -10 || true)
    if [[ -n "$errors" ]]; then
        eval "STAGE${stage_num}_ERRORS=\"\$errors\""
    fi
}

# Run stage validation and capture output
run_stage_validation() {
    local stage_num="$1"
    local script_name="$2"
    local log_var="$3"
    
    section "Running Stage $stage_num: $script_name"
    
    local stage_log_file
    eval "stage_log_file=\"\$$log_var\""
    
    if [[ -n "$stage_log_file" && -f "$stage_log_file" ]]; then
        log_info "Using provided log file: $stage_log_file"
        local log_content
        log_content=$(cat "$stage_log_file")
        extract_stage_stats "$stage_num" "$log_content"
    else
        log_info "Running $SCRIPT_DIR/$script_name"
        
        local start_time
        start_time=$(date +%s)
        
        local log_content
        local exit_code=0
        log_content=$("$SCRIPT_DIR/$script_name" 2>&1) || exit_code=$?
        
        local end_time
        end_time=$(date +%s)
        local duration=$((end_time - start_time))
        
        eval "STAGE${stage_num}_DURATION=$duration"
        
        extract_stage_stats "$stage_num" "$log_content"
        
        if [[ $exit_code -ne 0 ]]; then
            eval "STAGE${stage_num}_STATUS=FAILED"
            log_warning "Stage $stage_num failed with exit code $exit_code"
        fi
    fi
    
    local status
    eval "status=\$STAGE${stage_num}_STATUS"
    
    if [[ "$status" == "PASSED" ]]; then
        pass "Stage $stage_num completed successfully"
    else
        fail "Stage $stage_num had failures"
    fi
}

# Generate test coverage matrix
generate_coverage_matrix() {
    local test_cases_dir="$SCRIPT_DIR/00_test_cases"
    
    # Initialize category counters
    local success_tests=0
    local failure_tests=0
    local multi_seq_tests=0
    local hook_tests=0
    local variable_tests=0
    local manual_tests=0
    
    # Count test categories
    if [[ -d "$test_cases_dir" ]]; then
        success_tests=$(find "$test_cases_dir" -name "*SUCCESS*.yaml" 2>/dev/null | wc -l | tr -d ' ')
        failure_tests=$(find "$test_cases_dir" -name "*FAIL*.yaml" 2>/dev/null | wc -l | tr -d ' ')
        multi_seq_tests=$(find "$test_cases_dir" -name "*MULTI*.yaml" 2>/dev/null | wc -l | tr -d ' ')
        hook_tests=$(find "$test_cases_dir" -name "*HOOK*.yaml" 2>/dev/null | wc -l | tr -d ' ')
        variable_tests=$(find "$test_cases_dir" -name "*VAR*.yaml" 2>/dev/null | wc -l | tr -d ' ')
        manual_tests=$(find "$test_cases_dir" -name "*MANUAL*.yaml" 2>/dev/null | wc -l | tr -d ' ')
    fi
    
    # Export for use in report generation
    COVERAGE_SUCCESS=$success_tests
    COVERAGE_FAILURE=$failure_tests
    COVERAGE_MULTI_SEQ=$multi_seq_tests
    COVERAGE_HOOKS=$hook_tests
    COVERAGE_VARIABLES=$variable_tests
    COVERAGE_MANUAL=$manual_tests
}

# Generate JSON report
generate_json_report() {
    local output_file="$OUTPUT_DIR/final_report.json"
    
    log_info "Generating JSON report: $output_file"
    
    cat > "$output_file" << EOF
{
  "report_metadata": {
    "generated_at": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
    "report_version": "1.0",
    "total_duration_seconds": $(($(date +%s) - START_TIME))
  },
  "stages": {
    "stage1_yaml_validation": {
      "name": "YAML Validation",
      "total": ${STAGE1_TOTAL:-0},
      "passed": ${STAGE1_PASSED:-0},
      "failed": ${STAGE1_FAILED:-0},
      "skipped": ${STAGE1_SKIPPED:-0},
      "status": "${STAGE1_STATUS:-UNKNOWN}",
      "duration_seconds": ${STAGE1_DURATION:-0}
    },
    "stage2_script_generation": {
      "name": "Script Generation",
      "total": ${STAGE2_TOTAL:-0},
      "passed": ${STAGE2_PASSED:-0},
      "failed": ${STAGE2_FAILED:-0},
      "skipped": ${STAGE2_SKIPPED:-0},
      "status": "${STAGE2_STATUS:-UNKNOWN}",
      "duration_seconds": ${STAGE2_DURATION:-0}
    },
    "stage3_execution": {
      "name": "Script Execution",
      "total": ${STAGE3_TOTAL:-0},
      "passed": ${STAGE3_PASSED:-0},
      "failed": ${STAGE3_FAILED:-0},
      "skipped": ${STAGE3_SKIPPED:-0},
      "status": "${STAGE3_STATUS:-UNKNOWN}",
      "duration_seconds": ${STAGE3_DURATION:-0}
    },
    "stage4_verification": {
      "name": "Verification",
      "total": ${STAGE4_TOTAL:-0},
      "passed": ${STAGE4_PASSED:-0},
      "failed": ${STAGE4_FAILED:-0},
      "skipped": ${STAGE4_SKIPPED:-0},
      "status": "${STAGE4_STATUS:-UNKNOWN}",
      "duration_seconds": ${STAGE4_DURATION:-0}
    },
    "stage5_tpdg_result_docs": {
      "name": "TPDG Result Documentation",
      "total": ${STAGE5_TOTAL:-0},
      "passed": ${STAGE5_PASSED:-0},
      "failed": ${STAGE5_FAILED:-0},
      "skipped": ${STAGE5_SKIPPED:-0},
      "status": "${STAGE5_STATUS:-UNKNOWN}",
      "duration_seconds": ${STAGE5_DURATION:-0}
    },
    "stage6_tpdg_plan_docs": {
      "name": "TPDG Test Plan Documentation",
      "total": ${STAGE6_TOTAL:-0},
      "passed": ${STAGE6_PASSED:-0},
      "failed": ${STAGE6_FAILED:-0},
      "skipped": ${STAGE6_SKIPPED:-0},
      "status": "${STAGE6_STATUS:-UNKNOWN}",
      "duration_seconds": ${STAGE6_DURATION:-0}
    }
  },
  "test_coverage": {
    "success_scenarios": ${COVERAGE_SUCCESS:-0},
    "failure_scenarios": ${COVERAGE_FAILURE:-0},
    "multi_sequence_scenarios": ${COVERAGE_MULTI_SEQ:-0},
    "hook_scenarios": ${COVERAGE_HOOKS:-0},
    "variable_scenarios": ${COVERAGE_VARIABLES:-0},
    "manual_scenarios": ${COVERAGE_MANUAL:-0}
  },
  "summary": {
    "total_tests_executed": $((${STAGE1_TOTAL:-0})),
    "total_passed": $((${STAGE1_PASSED:-0} + ${STAGE2_PASSED:-0} + ${STAGE3_PASSED:-0} + ${STAGE4_PASSED:-0} + ${STAGE5_PASSED:-0} + ${STAGE6_PASSED:-0})),
    "total_failed": $((${STAGE1_FAILED:-0} + ${STAGE2_FAILED:-0} + ${STAGE3_FAILED:-0} + ${STAGE4_FAILED:-0} + ${STAGE5_FAILED:-0} + ${STAGE6_FAILED:-0})),
    "total_skipped": $((${STAGE1_SKIPPED:-0} + ${STAGE2_SKIPPED:-0} + ${STAGE3_SKIPPED:-0} + ${STAGE4_SKIPPED:-0} + ${STAGE5_SKIPPED:-0} + ${STAGE6_SKIPPED:-0})),
    "stages_passed": $(( (${STAGE1_STATUS:-UNKNOWN} == "PASSED" ? 1 : 0) + (${STAGE2_STATUS:-UNKNOWN} == "PASSED" ? 1 : 0) + (${STAGE3_STATUS:-UNKNOWN} == "PASSED" ? 1 : 0) + (${STAGE4_STATUS:-UNKNOWN} == "PASSED" ? 1 : 0) + (${STAGE5_STATUS:-UNKNOWN} == "PASSED" ? 1 : 0) + (${STAGE6_STATUS:-UNKNOWN} == "PASSED" ? 1 : 0) )),
    "stages_failed": $(( (${STAGE1_STATUS:-UNKNOWN} == "FAILED" ? 1 : 0) + (${STAGE2_STATUS:-UNKNOWN} == "FAILED" ? 1 : 0) + (${STAGE3_STATUS:-UNKNOWN} == "FAILED" ? 1 : 0) + (${STAGE4_STATUS:-UNKNOWN} == "FAILED" ? 1 : 0) + (${STAGE5_STATUS:-UNKNOWN} == "FAILED" ? 1 : 0) + (${STAGE6_STATUS:-UNKNOWN} == "FAILED" ? 1 : 0) ))
  }
}
EOF
    
    pass "JSON report generated: $output_file"
}

# Generate HTML report
generate_html_report() {
    local output_file="$OUTPUT_DIR/final_report.html"
    
    log_info "Generating HTML report: $output_file"
    
    local total_duration=$(($(date +%s) - START_TIME))
    local overall_status="PASSED"
    
    if [[ "${STAGE1_STATUS:-UNKNOWN}" == "FAILED" ]] || \
       [[ "${STAGE2_STATUS:-UNKNOWN}" == "FAILED" ]] || \
       [[ "${STAGE3_STATUS:-UNKNOWN}" == "FAILED" ]] || \
       [[ "${STAGE4_STATUS:-UNKNOWN}" == "FAILED" ]] || \
       [[ "${STAGE5_STATUS:-UNKNOWN}" == "FAILED" ]] || \
       [[ "${STAGE6_STATUS:-UNKNOWN}" == "FAILED" ]]; then
        overall_status="FAILED"
    fi
    
    cat > "$output_file" << 'HTMLEOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Acceptance Test Suite - Final Report</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            background: #f5f5f5;
            padding: 20px;
        }
        
        .container {
            max-width: 1200px;
            margin: 0 auto;
            background: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            overflow: hidden;
        }
        
        header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 40px;
            text-align: center;
        }
        
        header h1 {
            font-size: 2.5em;
            margin-bottom: 10px;
        }
        
        header .subtitle {
            font-size: 1.2em;
            opacity: 0.9;
        }
        
        .status-badge {
            display: inline-block;
            padding: 8px 20px;
            border-radius: 20px;
            font-weight: bold;
            font-size: 0.9em;
            margin-top: 15px;
        }
        
        .status-passed {
            background: #10b981;
            color: white;
        }
        
        .status-failed {
            background: #ef4444;
            color: white;
        }
        
        .status-unknown {
            background: #6b7280;
            color: white;
        }
        
        .content {
            padding: 40px;
        }
        
        .section {
            margin-bottom: 40px;
        }
        
        .section h2 {
            color: #667eea;
            font-size: 1.8em;
            margin-bottom: 20px;
            padding-bottom: 10px;
            border-bottom: 2px solid #e5e7eb;
        }
        
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }
        
        .stat-card {
            background: #f9fafb;
            border-radius: 8px;
            padding: 20px;
            border-left: 4px solid #667eea;
        }
        
        .stat-card .label {
            font-size: 0.85em;
            color: #6b7280;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            margin-bottom: 8px;
        }
        
        .stat-card .value {
            font-size: 2em;
            font-weight: bold;
            color: #111827;
        }
        
        .stage-card {
            background: #ffffff;
            border: 1px solid #e5e7eb;
            border-radius: 8px;
            padding: 25px;
            margin-bottom: 20px;
            transition: box-shadow 0.3s;
        }
        
        .stage-card:hover {
            box-shadow: 0 4px 12px rgba(0,0,0,0.1);
        }
        
        .stage-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 20px;
        }
        
        .stage-title {
            font-size: 1.3em;
            font-weight: bold;
            color: #111827;
        }
        
        .stage-stats {
            display: grid;
            grid-template-columns: repeat(4, 1fr);
            gap: 15px;
            margin-top: 15px;
        }
        
        .stage-stat {
            text-align: center;
            padding: 10px;
            background: #f9fafb;
            border-radius: 6px;
        }
        
        .stage-stat .label {
            font-size: 0.8em;
            color: #6b7280;
            margin-bottom: 5px;
        }
        
        .stage-stat .value {
            font-size: 1.5em;
            font-weight: bold;
        }
        
        .value.passed {
            color: #10b981;
        }
        
        .value.failed {
            color: #ef4444;
        }
        
        .value.skipped {
            color: #f59e0b;
        }
        
        .timeline {
            margin-top: 30px;
        }
        
        .timeline-item {
            display: flex;
            margin-bottom: 15px;
            align-items: center;
        }
        
        .timeline-label {
            width: 200px;
            font-weight: 500;
            color: #374151;
        }
        
        .timeline-bar-container {
            flex: 1;
            background: #e5e7eb;
            height: 30px;
            border-radius: 15px;
            overflow: hidden;
            position: relative;
        }
        
        .timeline-bar {
            height: 100%;
            background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
            display: flex;
            align-items: center;
            padding: 0 10px;
            color: white;
            font-size: 0.85em;
            font-weight: bold;
        }
        
        .coverage-matrix {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 15px;
        }
        
        .coverage-item {
            background: #f9fafb;
            border-radius: 8px;
            padding: 20px;
            border-left: 4px solid #10b981;
        }
        
        .coverage-item .category {
            font-size: 0.9em;
            color: #6b7280;
            margin-bottom: 8px;
        }
        
        .coverage-item .count {
            font-size: 2em;
            font-weight: bold;
            color: #10b981;
        }
        
        .error-section {
            background: #fef2f2;
            border: 1px solid #fecaca;
            border-radius: 8px;
            padding: 20px;
            margin-top: 15px;
        }
        
        .error-section h4 {
            color: #dc2626;
            margin-bottom: 10px;
        }
        
        .error-list {
            list-style: none;
            padding: 0;
        }
        
        .error-list li {
            padding: 8px;
            background: white;
            margin-bottom: 5px;
            border-radius: 4px;
            font-family: monospace;
            font-size: 0.9em;
            color: #991b1b;
        }
        
        table {
            width: 100%;
            border-collapse: collapse;
            margin-top: 20px;
        }
        
        th, td {
            padding: 12px;
            text-align: left;
            border-bottom: 1px solid #e5e7eb;
        }
        
        th {
            background: #f9fafb;
            font-weight: bold;
            color: #374151;
        }
        
        tr:hover {
            background: #f9fafb;
        }
        
        footer {
            background: #f9fafb;
            padding: 20px;
            text-align: center;
            color: #6b7280;
            font-size: 0.9em;
        }
        
        .icon {
            display: inline-block;
            width: 20px;
            height: 20px;
            margin-right: 5px;
            vertical-align: middle;
        }
        
        .icon-check {
            color: #10b981;
        }
        
        .icon-x {
            color: #ef4444;
        }
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>Acceptance Test Suite</h1>
            <p class="subtitle">Final Validation Report</p>
HTMLEOF

    # Add overall status badge to HTML
    cat >> "$output_file" << HTMLEOF
            <div class="status-badge status-${overall_status,,}">
                Overall Status: $overall_status
            </div>
        </header>
        
        <div class="content">
            <div class="section">
                <h2>📊 Executive Summary</h2>
                <div class="stats-grid">
                    <div class="stat-card">
                        <div class="label">Total Tests</div>
                        <div class="value">${STAGE1_TOTAL:-0}</div>
                    </div>
                    <div class="stat-card">
                        <div class="label">Total Passed</div>
                        <div class="value" style="color: #10b981;">$((${STAGE1_PASSED:-0} + ${STAGE2_PASSED:-0} + ${STAGE3_PASSED:-0} + ${STAGE4_PASSED:-0} + ${STAGE5_PASSED:-0} + ${STAGE6_PASSED:-0}))</div>
                    </div>
                    <div class="stat-card">
                        <div class="label">Total Failed</div>
                        <div class="value" style="color: #ef4444;">$((${STAGE1_FAILED:-0} + ${STAGE2_FAILED:-0} + ${STAGE3_FAILED:-0} + ${STAGE4_FAILED:-0} + ${STAGE5_FAILED:-0} + ${STAGE6_FAILED:-0}))</div>
                    </div>
                    <div class="stat-card">
                        <div class="label">Total Duration</div>
                        <div class="value">${total_duration}s</div>
                    </div>
                </div>
            </div>
            
            <div class="section">
                <h2>🎯 Stage Results</h2>
HTMLEOF

    # Generate stage cards
    for stage_num in 1 2 3 4 5 6; do
        local stage_name
        case $stage_num in
            1) stage_name="YAML Validation" ;;
            2) stage_name="Script Generation" ;;
            3) stage_name="Script Execution" ;;
            4) stage_name="Verification" ;;
            5) stage_name="TPDG Result Documentation" ;;
            6) stage_name="TPDG Test Plan Documentation" ;;
        esac
        
        eval "local total=\${STAGE${stage_num}_TOTAL:-0}"
        eval "local passed=\${STAGE${stage_num}_PASSED:-0}"
        eval "local failed=\${STAGE${stage_num}_FAILED:-0}"
        eval "local skipped=\${STAGE${stage_num}_SKIPPED:-0}"
        eval "local status=\${STAGE${stage_num}_STATUS:-UNKNOWN}"
        eval "local errors=\${STAGE${stage_num}_ERRORS:-}"
        
        cat >> "$output_file" << STAGEOF
                <div class="stage-card">
                    <div class="stage-header">
                        <div class="stage-title">Stage $stage_num: $stage_name</div>
                        <div class="status-badge status-${status,,}">$status</div>
                    </div>
                    <div class="stage-stats">
                        <div class="stage-stat">
                            <div class="label">Total</div>
                            <div class="value">$total</div>
                        </div>
                        <div class="stage-stat">
                            <div class="label">Passed</div>
                            <div class="value passed">$passed</div>
                        </div>
                        <div class="stage-stat">
                            <div class="label">Failed</div>
                            <div class="value failed">$failed</div>
                        </div>
                        <div class="stage-stat">
                            <div class="label">Skipped</div>
                            <div class="value skipped">$skipped</div>
                        </div>
                    </div>
STAGEOF

        if [[ -n "$errors" && "$status" == "FAILED" ]]; then
            cat >> "$output_file" << ERROREOF
                    <div class="error-section">
                        <h4>⚠️ Error Details</h4>
                        <ul class="error-list">
ERROREOF
            
            echo "$errors" | while IFS= read -r error_line; do
                if [[ -n "$error_line" ]]; then
                    echo "                            <li>$(echo "$error_line" | sed 's/</\&lt;/g; s/>/\&gt;/g')</li>" >> "$output_file"
                fi
            done
            
            cat >> "$output_file" << ERROREOF
                        </ul>
                    </div>
ERROREOF
        fi
        
        echo "                </div>" >> "$output_file"
    done

    cat >> "$output_file" << 'HTMLEOF2'
            </div>
            
            <div class="section">
                <h2>⏱️ Execution Timeline</h2>
                <div class="timeline">
HTMLEOF2

    # Generate timeline bars
    local max_duration=1
    for stage_num in 1 2 3 4 5 6; do
        eval "local duration=\${STAGE${stage_num}_DURATION:-0}"
        if [[ $duration -gt $max_duration ]]; then
            max_duration=$duration
        fi
    done
    
    for stage_num in 1 2 3 4 5 6; do
        local stage_name
        case $stage_num in
            1) stage_name="Stage 1: YAML Validation" ;;
            2) stage_name="Stage 2: Script Generation" ;;
            3) stage_name="Stage 3: Execution" ;;
            4) stage_name="Stage 4: Verification" ;;
            5) stage_name="Stage 5: TPDG Results" ;;
            6) stage_name="Stage 6: TPDG Plans" ;;
        esac
        
        eval "local duration=\${STAGE${stage_num}_DURATION:-0}"
        local percentage=$((duration * 100 / max_duration))
        
        cat >> "$output_file" << TIMELINEOF
                    <div class="timeline-item">
                        <div class="timeline-label">$stage_name</div>
                        <div class="timeline-bar-container">
                            <div class="timeline-bar" style="width: ${percentage}%;">
                                ${duration}s
                            </div>
                        </div>
                    </div>
TIMELINEOF
    done

    cat >> "$output_file" << HTMLEOF3
                </div>
            </div>
            
            <div class="section">
                <h2>📋 Test Coverage Matrix</h2>
                <div class="coverage-matrix">
                    <div class="coverage-item">
                        <div class="category">Success Scenarios</div>
                        <div class="count">${COVERAGE_SUCCESS:-0}</div>
                    </div>
                    <div class="coverage-item">
                        <div class="category">Failure Scenarios</div>
                        <div class="count">${COVERAGE_FAILURE:-0}</div>
                    </div>
                    <div class="coverage-item">
                        <div class="category">Multi-Sequence Tests</div>
                        <div class="count">${COVERAGE_MULTI_SEQ:-0}</div>
                    </div>
                    <div class="coverage-item">
                        <div class="category">Hook Tests</div>
                        <div class="count">${COVERAGE_HOOKS:-0}</div>
                    </div>
                    <div class="coverage-item">
                        <div class="category">Variable Tests</div>
                        <div class="count">${COVERAGE_VARIABLES:-0}</div>
                    </div>
                    <div class="coverage-item">
                        <div class="category">Manual Step Tests</div>
                        <div class="count">${COVERAGE_MANUAL:-0}</div>
                    </div>
                </div>
            </div>
            
            <div class="section">
                <h2>📈 Detailed Statistics</h2>
                <table>
                    <thead>
                        <tr>
                            <th>Stage</th>
                            <th>Total</th>
                            <th>Passed</th>
                            <th>Failed</th>
                            <th>Skipped</th>
                            <th>Duration</th>
                            <th>Status</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>Stage 1: YAML Validation</td>
                            <td>${STAGE1_TOTAL:-0}</td>
                            <td style="color: #10b981;">${STAGE1_PASSED:-0}</td>
                            <td style="color: #ef4444;">${STAGE1_FAILED:-0}</td>
                            <td style="color: #f59e0b;">${STAGE1_SKIPPED:-0}</td>
                            <td>${STAGE1_DURATION:-0}s</td>
                            <td><span class="status-badge status-${STAGE1_STATUS,,}">${STAGE1_STATUS:-UNKNOWN}</span></td>
                        </tr>
                        <tr>
                            <td>Stage 2: Script Generation</td>
                            <td>${STAGE2_TOTAL:-0}</td>
                            <td style="color: #10b981;">${STAGE2_PASSED:-0}</td>
                            <td style="color: #ef4444;">${STAGE2_FAILED:-0}</td>
                            <td style="color: #f59e0b;">${STAGE2_SKIPPED:-0}</td>
                            <td>${STAGE2_DURATION:-0}s</td>
                            <td><span class="status-badge status-${STAGE2_STATUS,,}">${STAGE2_STATUS:-UNKNOWN}</span></td>
                        </tr>
                        <tr>
                            <td>Stage 3: Script Execution</td>
                            <td>${STAGE3_TOTAL:-0}</td>
                            <td style="color: #10b981;">${STAGE3_PASSED:-0}</td>
                            <td style="color: #ef4444;">${STAGE3_FAILED:-0}</td>
                            <td style="color: #f59e0b;">${STAGE3_SKIPPED:-0}</td>
                            <td>${STAGE3_DURATION:-0}s</td>
                            <td><span class="status-badge status-${STAGE3_STATUS,,}">${STAGE3_STATUS:-UNKNOWN}</span></td>
                        </tr>
                        <tr>
                            <td>Stage 4: Verification</td>
                            <td>${STAGE4_TOTAL:-0}</td>
                            <td style="color: #10b981;">${STAGE4_PASSED:-0}</td>
                            <td style="color: #ef4444;">${STAGE4_FAILED:-0}</td>
                            <td style="color: #f59e0b;">${STAGE4_SKIPPED:-0}</td>
                            <td>${STAGE4_DURATION:-0}s</td>
                            <td><span class="status-badge status-${STAGE4_STATUS,,}">${STAGE4_STATUS:-UNKNOWN}</span></td>
                        </tr>
                        <tr>
                            <td>Stage 5: TPDG Result Docs</td>
                            <td>${STAGE5_TOTAL:-0}</td>
                            <td style="color: #10b981;">${STAGE5_PASSED:-0}</td>
                            <td style="color: #ef4444;">${STAGE5_FAILED:-0}</td>
                            <td style="color: #f59e0b;">${STAGE5_SKIPPED:-0}</td>
                            <td>${STAGE5_DURATION:-0}s</td>
                            <td><span class="status-badge status-${STAGE5_STATUS,,}">${STAGE5_STATUS:-UNKNOWN}</span></td>
                        </tr>
                        <tr>
                            <td>Stage 6: TPDG Test Plan Docs</td>
                            <td>${STAGE6_TOTAL:-0}</td>
                            <td style="color: #10b981;">${STAGE6_PASSED:-0}</td>
                            <td style="color: #ef4444;">${STAGE6_FAILED:-0}</td>
                            <td style="color: #f59e0b;">${STAGE6_SKIPPED:-0}</td>
                            <td>${STAGE6_DURATION:-0}s</td>
                            <td><span class="status-badge status-${STAGE6_STATUS,,}">${STAGE6_STATUS:-UNKNOWN}</span></td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
        
        <footer>
            <p>Generated on $(date "+%Y-%m-%d %H:%M:%S") | Test Case Manager - Acceptance Test Suite</p>
        </footer>
    </div>
</body>
</html>
HTMLEOF3
    
    pass "HTML report generated: $output_file"
}

# Generate Markdown report
generate_markdown_report() {
    local output_file="$OUTPUT_DIR/final_report.md"
    
    log_info "Generating Markdown report: $output_file"
    
    local overall_status="✅ PASSED"
    if [[ "${STAGE1_STATUS:-UNKNOWN}" == "FAILED" ]] || \
       [[ "${STAGE2_STATUS:-UNKNOWN}" == "FAILED" ]] || \
       [[ "${STAGE3_STATUS:-UNKNOWN}" == "FAILED" ]] || \
       [[ "${STAGE4_STATUS:-UNKNOWN}" == "FAILED" ]] || \
       [[ "${STAGE5_STATUS:-UNKNOWN}" == "FAILED" ]] || \
       [[ "${STAGE6_STATUS:-UNKNOWN}" == "FAILED" ]]; then
        overall_status="❌ FAILED"
    fi
    
    cat > "$output_file" << MDEOF
# Acceptance Test Suite - Final Report

**Generated:** $(date "+%Y-%m-%d %H:%M:%S")  
**Overall Status:** $overall_status

---

## Executive Summary

| Metric | Value |
|--------|-------|
| Total Tests | ${STAGE1_TOTAL:-0} |
| Total Passed | $((${STAGE1_PASSED:-0} + ${STAGE2_PASSED:-0} + ${STAGE3_PASSED:-0} + ${STAGE4_PASSED:-0} + ${STAGE5_PASSED:-0} + ${STAGE6_PASSED:-0})) |
| Total Failed | $((${STAGE1_FAILED:-0} + ${STAGE2_FAILED:-0} + ${STAGE3_FAILED:-0} + ${STAGE4_FAILED:-0} + ${STAGE5_FAILED:-0} + ${STAGE6_FAILED:-0})) |
| Total Skipped | $((${STAGE1_SKIPPED:-0} + ${STAGE2_SKIPPED:-0} + ${STAGE3_SKIPPED:-0} + ${STAGE4_SKIPPED:-0} + ${STAGE5_SKIPPED:-0} + ${STAGE6_SKIPPED:-0})) |
| Total Duration | $(($(date +%s) - START_TIME))s |

---

## Stage Results

### Stage 1: YAML Validation
**Status:** ${STAGE1_STATUS:-UNKNOWN}

| Total | Passed | Failed | Skipped | Duration |
|-------|--------|--------|---------|----------|
| ${STAGE1_TOTAL:-0} | ${STAGE1_PASSED:-0} | ${STAGE1_FAILED:-0} | ${STAGE1_SKIPPED:-0} | ${STAGE1_DURATION:-0}s |

### Stage 2: Script Generation
**Status:** ${STAGE2_STATUS:-UNKNOWN}

| Total | Passed | Failed | Skipped | Duration |
|-------|--------|--------|---------|----------|
| ${STAGE2_TOTAL:-0} | ${STAGE2_PASSED:-0} | ${STAGE2_FAILED:-0} | ${STAGE2_SKIPPED:-0} | ${STAGE2_DURATION:-0}s |

### Stage 3: Script Execution
**Status:** ${STAGE3_STATUS:-UNKNOWN}

| Total | Passed | Failed | Skipped | Duration |
|-------|--------|--------|---------|----------|
| ${STAGE3_TOTAL:-0} | ${STAGE3_PASSED:-0} | ${STAGE3_FAILED:-0} | ${STAGE3_SKIPPED:-0} | ${STAGE3_DURATION:-0}s |

### Stage 4: Verification
**Status:** ${STAGE4_STATUS:-UNKNOWN}

| Total | Passed | Failed | Skipped | Duration |
|-------|--------|--------|---------|----------|
| ${STAGE4_TOTAL:-0} | ${STAGE4_PASSED:-0} | ${STAGE4_FAILED:-0} | ${STAGE4_SKIPPED:-0} | ${STAGE4_DURATION:-0}s |

### Stage 5: TPDG Result Documentation
**Status:** ${STAGE5_STATUS:-UNKNOWN}

| Total | Passed | Failed | Skipped | Duration |
|-------|--------|--------|---------|----------|
| ${STAGE5_TOTAL:-0} | ${STAGE5_PASSED:-0} | ${STAGE5_FAILED:-0} | ${STAGE5_SKIPPED:-0} | ${STAGE5_DURATION:-0}s |

### Stage 6: TPDG Test Plan Documentation
**Status:** ${STAGE6_STATUS:-UNKNOWN}

| Total | Passed | Failed | Skipped | Duration |
|-------|--------|--------|---------|----------|
| ${STAGE6_TOTAL:-0} | ${STAGE6_PASSED:-0} | ${STAGE6_FAILED:-0} | ${STAGE6_SKIPPED:-0} | ${STAGE6_DURATION:-0}s |

---

## Test Coverage Matrix

| Category | Count |
|----------|-------|
| Success Scenarios | ${COVERAGE_SUCCESS:-0} |
| Failure Scenarios | ${COVERAGE_FAILURE:-0} |
| Multi-Sequence Tests | ${COVERAGE_MULTI_SEQ:-0} |
| Hook Tests | ${COVERAGE_HOOKS:-0} |
| Variable Tests | ${COVERAGE_VARIABLES:-0} |
| Manual Step Tests | ${COVERAGE_MANUAL:-0} |

---

## Execution Timeline

| Stage | Duration |
|-------|----------|
| Stage 1: YAML Validation | ${STAGE1_DURATION:-0}s |
| Stage 2: Script Generation | ${STAGE2_DURATION:-0}s |
| Stage 3: Script Execution | ${STAGE3_DURATION:-0}s |
| Stage 4: Verification | ${STAGE4_DURATION:-0}s |
| Stage 5: TPDG Result Docs | ${STAGE5_DURATION:-0}s |
| Stage 6: TPDG Test Plan Docs | ${STAGE6_DURATION:-0}s |

---

## Detailed Statistics

| Stage | Total | Passed | Failed | Skipped | Duration | Status |
|-------|-------|--------|--------|---------|----------|--------|
| Stage 1: YAML Validation | ${STAGE1_TOTAL:-0} | ${STAGE1_PASSED:-0} | ${STAGE1_FAILED:-0} | ${STAGE1_SKIPPED:-0} | ${STAGE1_DURATION:-0}s | ${STAGE1_STATUS:-UNKNOWN} |
| Stage 2: Script Generation | ${STAGE2_TOTAL:-0} | ${STAGE2_PASSED:-0} | ${STAGE2_FAILED:-0} | ${STAGE2_SKIPPED:-0} | ${STAGE2_DURATION:-0}s | ${STAGE2_STATUS:-UNKNOWN} |
| Stage 3: Script Execution | ${STAGE3_TOTAL:-0} | ${STAGE3_PASSED:-0} | ${STAGE3_FAILED:-0} | ${STAGE3_SKIPPED:-0} | ${STAGE3_DURATION:-0}s | ${STAGE3_STATUS:-UNKNOWN} |
| Stage 4: Verification | ${STAGE4_TOTAL:-0} | ${STAGE4_PASSED:-0} | ${STAGE4_FAILED:-0} | ${STAGE4_SKIPPED:-0} | ${STAGE4_DURATION:-0}s | ${STAGE4_STATUS:-UNKNOWN} |
| Stage 5: TPDG Result Docs | ${STAGE5_TOTAL:-0} | ${STAGE5_PASSED:-0} | ${STAGE5_FAILED:-0} | ${STAGE5_SKIPPED:-0} | ${STAGE5_DURATION:-0}s | ${STAGE5_STATUS:-UNKNOWN} |
| Stage 6: TPDG Test Plan Docs | ${STAGE6_TOTAL:-0} | ${STAGE6_PASSED:-0} | ${STAGE6_FAILED:-0} | ${STAGE6_SKIPPED:-0} | ${STAGE6_DURATION:-0}s | ${STAGE6_STATUS:-UNKNOWN} |

---

*Report generated by Test Case Manager - Acceptance Test Suite*
MDEOF
    
    pass "Markdown report generated: $output_file"
}

# Main execution
main() {
    section "Acceptance Test Suite - Final Report Generation"
    
    parse_args "$@"
    
    # Create output directory
    mkdir -p "$OUTPUT_DIR"
    log_info "Output directory: $OUTPUT_DIR"
    
    # Run all stage validations
    run_stage_validation 1 "validate_stage1_yaml.sh" "STAGE1_LOG"
    run_stage_validation 2 "validate_stage2_scripts.sh" "STAGE2_LOG"
    run_stage_validation 3 "validate_stage3_execution.sh" "STAGE3_LOG"
    run_stage_validation 4 "validate_stage4_verification.sh" "STAGE4_LOG"
    run_stage_validation 5 "validate_stage5_tpdg_result_docs.sh" "STAGE5_LOG"
    run_stage_validation 6 "validate_stage6_tpdg_plan_docs.sh" "STAGE6_LOG"
    
    # Generate test coverage matrix
    section "Generating Test Coverage Matrix"
    generate_coverage_matrix
    
    # Generate reports
    section "Generating Reports"
    
    if [[ $SKIP_JSON -eq 0 ]]; then
        generate_json_report
    else
        log_info "Skipping JSON report generation (--skip-json)"
    fi
    
    if [[ $SKIP_HTML -eq 0 ]]; then
        generate_html_report
    else
        log_info "Skipping HTML report generation (--skip-html)"
    fi
    
    if [[ $SKIP_MARKDOWN -eq 0 ]]; then
        generate_markdown_report
    else
        log_info "Skipping Markdown report generation (--skip-markdown)"
    fi
    
    # Summary
    section "Report Generation Complete"
    
    local end_time
    end_time=$(date +%s)
    local total_duration=$((end_time - START_TIME))
    
    log_info "Reports generated in: $OUTPUT_DIR"
    log_info "Total execution time: ${total_duration}s"
    
    if [[ $SKIP_HTML -eq 0 ]]; then
        info "HTML Report: $OUTPUT_DIR/final_report.html"
    fi
    
    if [[ $SKIP_MARKDOWN -eq 0 ]]; then
        info "Markdown Report: $OUTPUT_DIR/final_report.md"
    fi
    
    if [[ $SKIP_JSON -eq 0 ]]; then
        info "JSON Report: $OUTPUT_DIR/final_report.json"
    fi
    
    # Determine overall exit code
    local overall_failed=0
    
    if [[ "${STAGE1_STATUS:-UNKNOWN}" == "FAILED" ]] || \
       [[ "${STAGE2_STATUS:-UNKNOWN}" == "FAILED" ]] || \
       [[ "${STAGE3_STATUS:-UNKNOWN}" == "FAILED" ]] || \
       [[ "${STAGE4_STATUS:-UNKNOWN}" == "FAILED" ]] || \
       [[ "${STAGE5_STATUS:-UNKNOWN}" == "FAILED" ]] || \
       [[ "${STAGE6_STATUS:-UNKNOWN}" == "FAILED" ]]; then
        overall_failed=1
        fail "Some stages failed - see reports for details"
        return 1
    else
        pass "All stages passed successfully!"
        return 0
    fi
}

# Run main function
main "$@"
