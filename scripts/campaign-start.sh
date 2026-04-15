#!/usr/bin/env bash
#
# campaign-start.sh - Initialize a new test campaign
#
# DESCRIPTION:
#   Creates a new test campaign directory structure and initializes metadata.
#   A test campaign is an organized collection of test executions with evidence tracking.
#
# USAGE:
#   ./scripts/campaign-start.sh [OPTIONS]
#
# OPTIONS:
#   --name NAME            Campaign name (required)
#   --description DESC     Campaign description (optional)
#   --output-dir DIR       Campaign output directory (default: campaigns/<name>)
#   --testcase-dir DIR     Test case directory to use (default: testcases)
#   --verbose              Enable verbose output
#   --help                 Show this help message
#
# EXAMPLES:
#   # Start a new campaign with default settings
#   ./scripts/campaign-start.sh --name "Sprint_23_Regression"
#
#   # Start a campaign with custom description and location
#   ./scripts/campaign-start.sh --name "Release_Validation" \
#       --description "Full regression suite for v2.0 release" \
#       --output-dir /tmp/campaigns/release_v2
#

set -e

# Get script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source required libraries
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

# Default configuration
CAMPAIGN_NAME=""
CAMPAIGN_DESCRIPTION=""
OUTPUT_DIR=""
TESTCASE_DIR="$PROJECT_ROOT/testcases"
VERBOSE=0

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --name)
            CAMPAIGN_NAME="$2"
            shift 2
            ;;
        --description)
            CAMPAIGN_DESCRIPTION="$2"
            shift 2
            ;;
        --output-dir)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --testcase-dir)
            TESTCASE_DIR="$2"
            shift 2
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
if [[ -z "$CAMPAIGN_NAME" ]]; then
    log_error "Campaign name is required (--name)"
    echo "Use --help for usage information"
    exit 1
fi

# Set default output directory if not specified
if [[ -z "$OUTPUT_DIR" ]]; then
    OUTPUT_DIR="$PROJECT_ROOT/campaigns/$CAMPAIGN_NAME"
fi

# Display configuration
section "Starting Test Campaign"
log_info "Campaign Configuration:"
log_info "  Name: $CAMPAIGN_NAME"
log_info "  Description: ${CAMPAIGN_DESCRIPTION:-<none>}"
log_info "  Output directory: $OUTPUT_DIR"
log_info "  Test case directory: $TESTCASE_DIR"
log_info "  Verbose: $VERBOSE"
echo ""

# Check if campaign already exists
if [[ -d "$OUTPUT_DIR" ]]; then
    log_error "Campaign directory already exists: $OUTPUT_DIR"
    log_error "Please use a different name or remove the existing campaign"
    exit 1
fi

# Create campaign directory structure
section "Creating Campaign Directory Structure"

mkdir -p "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR/testcases"
mkdir -p "$OUTPUT_DIR/execution_logs"
mkdir -p "$OUTPUT_DIR/verification_results"
mkdir -p "$OUTPUT_DIR/evidence"
mkdir -p "$OUTPUT_DIR/reports"
mkdir -p "$OUTPUT_DIR/metadata"

pass "Created campaign directory structure"

# Create campaign metadata file
CAMPAIGN_METADATA="$OUTPUT_DIR/metadata/campaign.yaml"
CAMPAIGN_START_TIME=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

cat > "$CAMPAIGN_METADATA" << EOF
# Test Campaign Metadata
campaign:
  name: "$CAMPAIGN_NAME"
  description: "${CAMPAIGN_DESCRIPTION:-Test campaign created on $CAMPAIGN_START_TIME}"
  start_time: "$CAMPAIGN_START_TIME"
  status: "active"
  
configuration:
  testcase_dir: "$TESTCASE_DIR"
  output_dir: "$OUTPUT_DIR"
  
statistics:
  total_tests_planned: 0
  total_tests_executed: 0
  total_tests_passed: 0
  total_tests_failed: 0
  total_tests_skipped: 0
  
environment:
  hostname: "$(hostname)"
  user: "$(whoami)"
  platform: "$(uname -s)"
  architecture: "$(uname -m)"
EOF

pass "Created campaign metadata: $CAMPAIGN_METADATA"

# Create campaign README
CAMPAIGN_README="$OUTPUT_DIR/README.md"

cat > "$CAMPAIGN_README" << EOF
# Test Campaign: $CAMPAIGN_NAME

## Overview

**Campaign Name:** $CAMPAIGN_NAME  
**Description:** ${CAMPAIGN_DESCRIPTION:-Test campaign created on $CAMPAIGN_START_TIME}  
**Start Time:** $CAMPAIGN_START_TIME  
**Status:** Active  

## Directory Structure

\`\`\`
$OUTPUT_DIR/
├── testcases/              # Test case YAML files executed in this campaign
├── execution_logs/         # JSON execution logs from test runs
├── verification_results/   # Verification results in JSON/YAML format
├── evidence/               # Collected evidence (logs, screenshots, artifacts)
├── reports/                # Generated test reports
└── metadata/               # Campaign metadata and configuration
    ├── campaign.yaml       # Campaign metadata
    └── run_*.yaml          # Individual test run metadata
\`\`\`

## Usage

### Run Tests

Run all tests:
\`\`\`bash
./scripts/campaign-run.sh --campaign "$OUTPUT_DIR"
\`\`\`

Run tests matching a pattern:
\`\`\`bash
./scripts/campaign-run.sh --campaign "$OUTPUT_DIR" --pattern "EXAMPLE.*"
\`\`\`

### Collect Evidence

Collect all evidence for this campaign:
\`\`\`bash
./scripts/campaign-collect-evidence.sh --campaign "$OUTPUT_DIR"
\`\`\`

### Stop Campaign

Mark campaign as complete and generate final reports:
\`\`\`bash
./scripts/campaign-stop.sh --campaign "$OUTPUT_DIR"
\`\`\`

## Test Runs

Test runs will be recorded in \`metadata/run_*.yaml\` files with timestamps and results.

EOF

pass "Created campaign README: $CAMPAIGN_README"

# Create state file for tracking campaign status
CAMPAIGN_STATE="$OUTPUT_DIR/metadata/state.txt"
echo "ACTIVE" > "$CAMPAIGN_STATE"
pass "Created campaign state file"

# Create run counter
CAMPAIGN_COUNTER="$OUTPUT_DIR/metadata/run_counter.txt"
echo "0" > "$CAMPAIGN_COUNTER"
pass "Created run counter"

# Final summary
section "Campaign Successfully Created"
echo ""
info "Campaign Location:"
echo "  📁 $OUTPUT_DIR"
echo ""
info "Next Steps:"
echo "  1. Run tests:       ./scripts/campaign-run.sh --campaign \"$OUTPUT_DIR\""
echo "  2. Collect evidence: ./scripts/campaign-collect-evidence.sh --campaign \"$OUTPUT_DIR\""
echo "  3. Stop campaign:    ./scripts/campaign-stop.sh --campaign \"$OUTPUT_DIR\""
echo ""
pass "Campaign '$CAMPAIGN_NAME' is ready!"

exit 0
