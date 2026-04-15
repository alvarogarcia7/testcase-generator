#!/usr/bin/env bash
#
# campaign-stop.sh - Stop a test campaign and generate final reports
#
# DESCRIPTION:
#   Marks a campaign as complete, updates statistics, and generates final summary reports.
#   A stopped campaign cannot be modified further.
#
# USAGE:
#   ./scripts/campaign-stop.sh [OPTIONS]
#
# OPTIONS:
#   --campaign DIR         Campaign directory (required)
#   --summary TEXT         Final summary text (optional)
#   --generate-reports     Generate final HTML/PDF reports
#   --collect-evidence     Automatically collect evidence after stopping
#   --verbose              Enable verbose output
#   --help                 Show this help message
#
# EXAMPLES:
#   # Stop campaign with default settings
#   ./scripts/campaign-stop.sh --campaign campaigns/Sprint_23
#
#   # Stop and generate reports
#   ./scripts/campaign-stop.sh --campaign campaigns/Sprint_23 --generate-reports
#

set -e

# Get script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source required libraries
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

# Default configuration
CAMPAIGN_DIR=""
SUMMARY_TEXT=""
GENERATE_REPORTS=0
COLLECT_EVIDENCE=0
VERBOSE=0

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --campaign)
            CAMPAIGN_DIR="$2"
            shift 2
            ;;
        --summary)
            SUMMARY_TEXT="$2"
            shift 2
            ;;
        --generate-reports)
            GENERATE_REPORTS=1
            shift
            ;;
        --collect-evidence)
            COLLECT_EVIDENCE=1
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
if [[ "$STATE" == "COMPLETED" ]]; then
    log_warning "Campaign is already completed"
    log_info "Campaign was previously stopped on: $(stat -c %y "$CAMPAIGN_STATE" 2>/dev/null || stat -f %Sm "$CAMPAIGN_STATE" 2>/dev/null || echo 'unknown')"
    exit 0
fi

# Load campaign metadata
CAMPAIGN_METADATA="$CAMPAIGN_DIR/metadata/campaign.yaml"
if [[ ! -f "$CAMPAIGN_METADATA" ]]; then
    log_error "Campaign metadata not found: $CAMPAIGN_METADATA"
    exit 1
fi

# Get campaign name
CAMPAIGN_NAME=$(grep 'name:' "$CAMPAIGN_METADATA" | head -1 | sed 's/.*name: *"\(.*\)".*/\1/' | tr -d '"')

# Display configuration
section "Stopping Test Campaign"
log_info "Campaign Configuration:"
log_info "  Campaign directory: $CAMPAIGN_DIR"
log_info "  Campaign name: $CAMPAIGN_NAME"
log_info "  Generate reports: $GENERATE_REPORTS"
log_info "  Collect evidence: $COLLECT_EVIDENCE"
log_info "  Verbose: $VERBOSE"
echo ""

# Calculate statistics
section "Calculating Campaign Statistics"

log_info "Analyzing test runs..."

TOTAL_RUNS=0
TOTAL_TESTS_EXECUTED=0
TOTAL_TESTS_SUCCESS=0
TOTAL_TESTS_FAILED=0
TOTAL_TESTS_ERROR=0

if [[ -d "$CAMPAIGN_DIR/metadata" ]]; then
    while IFS= read -r run_file; do
        TOTAL_RUNS=$((TOTAL_RUNS + 1))
        
        # Extract statistics from run metadata
        if grep -q 'total_tests:' "$run_file"; then
            TESTS=$(grep 'total_tests:' "$run_file" | sed 's/.*total_tests: *//')
            TOTAL_TESTS_EXECUTED=$((TOTAL_TESTS_EXECUTED + TESTS))
        fi
        
        if grep -q 'execution_success:' "$run_file"; then
            SUCCESS=$(grep 'execution_success:' "$run_file" | sed 's/.*execution_success: *//')
            TOTAL_TESTS_SUCCESS=$((TOTAL_TESTS_SUCCESS + SUCCESS))
        fi
        
        if grep -q 'execution_failed:' "$run_file"; then
            FAILED=$(grep 'execution_failed:' "$run_file" | sed 's/.*execution_failed: *//')
            TOTAL_TESTS_FAILED=$((TOTAL_TESTS_FAILED + FAILED))
        fi
        
        if grep -q 'execution_error:' "$run_file"; then
            ERROR=$(grep 'execution_error:' "$run_file" | sed 's/.*execution_error: *//')
            TOTAL_TESTS_ERROR=$((TOTAL_TESTS_ERROR + ERROR))
        fi
    done < <(find "$CAMPAIGN_DIR/metadata" -name "run_*.yaml" 2>/dev/null)
fi

log_info "Campaign Statistics:"
log_info "  Total runs: $TOTAL_RUNS"
log_info "  Total tests executed: $TOTAL_TESTS_EXECUTED"
log_info "  Total tests success: $TOTAL_TESTS_SUCCESS"
log_info "  Total tests failed: $TOTAL_TESTS_FAILED"
log_info "  Total tests error: $TOTAL_TESTS_ERROR"
echo ""

# Update campaign metadata
section "Updating Campaign Metadata"

STOP_TIME=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

# Create a backup of the original metadata
cp "$CAMPAIGN_METADATA" "$CAMPAIGN_METADATA.backup"

# Update metadata with final statistics
cat > "$CAMPAIGN_METADATA" << EOF
# Test Campaign Metadata
campaign:
  name: "$CAMPAIGN_NAME"
  description: "$(grep 'description:' "$CAMPAIGN_METADATA.backup" | sed 's/.*description: *"\(.*\)".*/\1/' | tr -d '"')"
  start_time: "$(grep 'start_time:' "$CAMPAIGN_METADATA.backup" | sed 's/.*start_time: *"\(.*\)".*/\1/' | tr -d '"')"
  stop_time: "$STOP_TIME"
  status: "completed"
  
configuration:
  testcase_dir: "$(grep 'testcase_dir:' "$CAMPAIGN_METADATA.backup" | sed 's/.*testcase_dir: *"\(.*\)".*/\1/' | tr -d '"')"
  output_dir: "$CAMPAIGN_DIR"
  
statistics:
  total_runs: $TOTAL_RUNS
  total_tests_executed: $TOTAL_TESTS_EXECUTED
  total_tests_success: $TOTAL_TESTS_SUCCESS
  total_tests_failed: $TOTAL_TESTS_FAILED
  total_tests_error: $TOTAL_TESTS_ERROR
  pass_rate: $(awk "BEGIN {if ($TOTAL_TESTS_EXECUTED > 0) printf \"%.2f\", ($TOTAL_TESTS_SUCCESS / $TOTAL_TESTS_EXECUTED) * 100; else print 0}")
  
environment:
  hostname: "$(hostname)"
  user: "$(whoami)"
  platform: "$(uname -s)"
  architecture: "$(uname -m)"

summary:
  text: "${SUMMARY_TEXT:-Campaign completed on $STOP_TIME}"
EOF

pass "Updated campaign metadata"

# Update state file
echo "COMPLETED" > "$CAMPAIGN_STATE"
pass "Marked campaign as COMPLETED"

# Generate final summary report
section "Generating Final Summary Report"

SUMMARY_REPORT="$CAMPAIGN_DIR/reports/CAMPAIGN_SUMMARY.md"
mkdir -p "$CAMPAIGN_DIR/reports"

cat > "$SUMMARY_REPORT" << EOF
# Test Campaign Summary: $CAMPAIGN_NAME

## Campaign Overview

**Status:** ✅ COMPLETED  
**Start Time:** $(grep 'start_time:' "$CAMPAIGN_METADATA" | sed 's/.*start_time: *"\(.*\)".*/\1/' | tr -d '"')  
**Stop Time:** $STOP_TIME  

## Summary

${SUMMARY_TEXT:-Campaign completed successfully.}

## Statistics

| Metric | Count |
|--------|-------|
| Total Test Runs | $TOTAL_RUNS |
| Total Tests Executed | $TOTAL_TESTS_EXECUTED |
| Tests Passed | $TOTAL_TESTS_SUCCESS |
| Tests Failed | $TOTAL_TESTS_FAILED |
| Tests with Errors | $TOTAL_TESTS_ERROR |
| Pass Rate | $(awk "BEGIN {if ($TOTAL_TESTS_EXECUTED > 0) printf \"%.2f%%\", ($TOTAL_TESTS_SUCCESS / $TOTAL_TESTS_EXECUTED) * 100; else print \"N/A\"}")  |

## Test Runs

EOF

# List all test runs
if [[ -d "$CAMPAIGN_DIR/metadata" ]]; then
    RUN_COUNT=0
    while IFS= read -r run_file; do
        RUN_COUNT=$((RUN_COUNT + 1))
        RUN_ID=$(grep 'id:' "$run_file" | head -1 | sed 's/.*id: *"\(.*\)".*/\1/' | tr -d '"')
        RUN_NUMBER=$(grep 'number:' "$run_file" | sed 's/.*number: *//')
        RUN_TIMESTAMP=$(grep 'timestamp:' "$run_file" | sed 's/.*timestamp: *"\(.*\)".*/\1/' | tr -d '"')
        RUN_TESTS=$(grep 'total_tests:' "$run_file" | sed 's/.*total_tests: *//')
        
        cat >> "$SUMMARY_REPORT" << RUNEOF
### Run #$RUN_NUMBER - $RUN_ID

- **Timestamp:** $RUN_TIMESTAMP
- **Tests Executed:** $RUN_TESTS
- **Status:** See \`metadata/${RUN_ID}.yaml\` for details

RUNEOF
    done < <(find "$CAMPAIGN_DIR/metadata" -name "run_*.yaml" 2>/dev/null | sort)
    
    if [[ $RUN_COUNT -eq 0 ]]; then
        echo "No test runs found." >> "$SUMMARY_REPORT"
    fi
else
    echo "No test runs found." >> "$SUMMARY_REPORT"
fi

cat >> "$SUMMARY_REPORT" << EOF

## Evidence and Artifacts

Campaign artifacts are organized in the following directories:

- **testcases/** - Test case YAML files executed in this campaign
- **execution_logs/** - JSON execution logs from all test runs
- **verification_results/** - Verification results in JSON/YAML format
- **metadata/** - Campaign metadata and individual run information
- **reports/** - Generated test reports and summaries
- **evidence/** - Additional evidence files

## Conclusion

Campaign completed with $TOTAL_TESTS_SUCCESS successful tests out of $TOTAL_TESTS_EXECUTED total tests.

EOF

pass "Generated campaign summary: $SUMMARY_REPORT"

# Generate reports if requested
if [[ $GENERATE_REPORTS -eq 1 ]]; then
    section "Generating Final Reports"
    log_info "Report generation is not yet implemented"
    log_info "Summary report has been created at: $SUMMARY_REPORT"
fi

# Collect evidence if requested
if [[ $COLLECT_EVIDENCE -eq 1 ]]; then
    section "Collecting Evidence"
    log_info "Running evidence collection..."
    
    if [[ -x "$SCRIPT_DIR/campaign-collect-evidence.sh" ]]; then
        "$SCRIPT_DIR/campaign-collect-evidence.sh" --campaign "$CAMPAIGN_DIR" --checksums
    else
        log_warning "campaign-collect-evidence.sh not found or not executable"
        log_info "You can collect evidence manually with:"
        log_info "  ./scripts/campaign-collect-evidence.sh --campaign \"$CAMPAIGN_DIR\" --checksums"
    fi
fi

# Final summary
section "Campaign Stopped Successfully"
echo ""
info "Campaign Summary:"
echo "  Name: $CAMPAIGN_NAME"
echo "  Status: COMPLETED"
echo "  Stop Time: $STOP_TIME"
echo ""
info "Statistics:"
echo "  Test Runs: $TOTAL_RUNS"
echo "  Tests Executed: $TOTAL_TESTS_EXECUTED"
echo "  Tests Passed: $TOTAL_TESTS_SUCCESS"
echo "  Tests Failed: $TOTAL_TESTS_FAILED"
echo "  Tests Error: $TOTAL_TESTS_ERROR"
if [[ $TOTAL_TESTS_EXECUTED -gt 0 ]]; then
    echo "  Pass Rate: $(awk "BEGIN {printf \"%.2f%%\", ($TOTAL_TESTS_SUCCESS / $TOTAL_TESTS_EXECUTED) * 100}")"
fi
echo ""
info "Reports:"
echo "  Summary: $SUMMARY_REPORT"
echo ""
pass "Campaign '$CAMPAIGN_NAME' has been stopped!"

exit 0
