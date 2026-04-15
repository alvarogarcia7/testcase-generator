#!/usr/bin/env bash
#
# campaign-collect-evidence.sh - Collect all evidence for a test campaign
#
# DESCRIPTION:
#   Aggregates all test artifacts, logs, and results into an evidence package.
#   Creates a comprehensive archive suitable for audit or compliance purposes.
#
# USAGE:
#   ./scripts/campaign-collect-evidence.sh [OPTIONS]
#
# OPTIONS:
#   --campaign DIR         Campaign directory (required)
#   --output FILE          Output archive file (default: <campaign>_evidence.tar.gz)
#   --format FORMAT        Archive format: tar.gz, tar.bz2, zip (default: tar.gz)
#   --include-binaries     Include binary artifacts
#   --checksums            Generate SHA256 checksums for all files
#   --verbose              Enable verbose output
#   --help                 Show this help message
#
# EXAMPLES:
#   # Collect all evidence with default settings
#   ./scripts/campaign-collect-evidence.sh --campaign campaigns/Sprint_23
#
#   # Create ZIP archive with checksums
#   ./scripts/campaign-collect-evidence.sh --campaign campaigns/Sprint_23 \
#       --format zip --checksums
#

set -e

# Get script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source required libraries
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

# Default configuration
CAMPAIGN_DIR=""
OUTPUT_FILE=""
ARCHIVE_FORMAT="tar.gz"
INCLUDE_BINARIES=0
GENERATE_CHECKSUMS=0
VERBOSE=0

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --campaign)
            CAMPAIGN_DIR="$2"
            shift 2
            ;;
        --output)
            OUTPUT_FILE="$2"
            shift 2
            ;;
        --format)
            ARCHIVE_FORMAT="$2"
            shift 2
            ;;
        --include-binaries)
            INCLUDE_BINARIES=1
            shift
            ;;
        --checksums)
            GENERATE_CHECKSUMS=1
            shift
            ;;
        --verbose)
            VERBOSE=1
            export VERBOSE
            shift
            ;;
        --help)
            head -n 35 "$0" | tail -n +2 | sed 's/^# //'
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

# Validate archive format
if [[ ! "$ARCHIVE_FORMAT" =~ ^(tar\.gz|tar\.bz2|zip)$ ]]; then
    log_error "Invalid archive format: $ARCHIVE_FORMAT (must be: tar.gz, tar.bz2, or zip)"
    exit 1
fi

# Load campaign metadata
CAMPAIGN_METADATA="$CAMPAIGN_DIR/metadata/campaign.yaml"
if [[ ! -f "$CAMPAIGN_METADATA" ]]; then
    log_error "Campaign metadata not found: $CAMPAIGN_METADATA"
    exit 1
fi

# Get campaign name
CAMPAIGN_NAME=$(grep 'name:' "$CAMPAIGN_METADATA" | head -1 | sed 's/.*name: *"\(.*\)".*/\1/' | tr -d '"')

# Set default output file if not specified
if [[ -z "$OUTPUT_FILE" ]]; then
    OUTPUT_FILE="$PROJECT_ROOT/${CAMPAIGN_NAME}_evidence.${ARCHIVE_FORMAT}"
fi

# Display configuration
section "Collecting Campaign Evidence"
log_info "Evidence Collection Configuration:"
log_info "  Campaign directory: $CAMPAIGN_DIR"
log_info "  Campaign name: $CAMPAIGN_NAME"
log_info "  Output file: $OUTPUT_FILE"
log_info "  Archive format: $ARCHIVE_FORMAT"
log_info "  Include binaries: $INCLUDE_BINARIES"
log_info "  Generate checksums: $GENERATE_CHECKSUMS"
log_info "  Verbose: $VERBOSE"
echo ""

# Create temporary evidence directory
TEMP_DIR=$(mktemp -d)
setup_cleanup "$TEMP_DIR"

EVIDENCE_DIR="$TEMP_DIR/${CAMPAIGN_NAME}_evidence"
mkdir -p "$EVIDENCE_DIR"

# Copy campaign structure
section "Collecting Evidence Files"

log_info "Copying test cases..."
if [[ -d "$CAMPAIGN_DIR/testcases" ]]; then
    cp -r "$CAMPAIGN_DIR/testcases" "$EVIDENCE_DIR/"
    TESTCASE_COUNT=$(find "$CAMPAIGN_DIR/testcases" -type f \( -name "*.yml" -o -name "*.yaml" \) | wc -l | tr -d ' ')
    pass "Copied $TESTCASE_COUNT test case file(s)"
else
    log_warning "No testcases directory found"
fi

log_info "Copying execution logs..."
if [[ -d "$CAMPAIGN_DIR/execution_logs" ]]; then
    cp -r "$CAMPAIGN_DIR/execution_logs" "$EVIDENCE_DIR/"
    LOG_COUNT=$(find "$CAMPAIGN_DIR/execution_logs" -type f -name "*.json" | wc -l | tr -d ' ')
    pass "Copied $LOG_COUNT execution log(s)"
else
    log_warning "No execution_logs directory found"
fi

log_info "Copying verification results..."
if [[ -d "$CAMPAIGN_DIR/verification_results" ]]; then
    cp -r "$CAMPAIGN_DIR/verification_results" "$EVIDENCE_DIR/"
    VERIFY_COUNT=$(find "$CAMPAIGN_DIR/verification_results" -type f | wc -l | tr -d ' ')
    pass "Copied $VERIFY_COUNT verification result(s)"
else
    log_warning "No verification_results directory found"
fi

log_info "Copying metadata..."
if [[ -d "$CAMPAIGN_DIR/metadata" ]]; then
    cp -r "$CAMPAIGN_DIR/metadata" "$EVIDENCE_DIR/"
    pass "Copied campaign metadata"
else
    log_warning "No metadata directory found"
fi

log_info "Copying reports..."
if [[ -d "$CAMPAIGN_DIR/reports" ]]; then
    cp -r "$CAMPAIGN_DIR/reports" "$EVIDENCE_DIR/"
    pass "Copied reports"
else
    log_verbose "No reports directory found (not required)"
fi

log_info "Copying additional evidence..."
if [[ -d "$CAMPAIGN_DIR/evidence" ]]; then
    if [[ -n "$(ls -A "$CAMPAIGN_DIR/evidence")" ]]; then
        cp -r "$CAMPAIGN_DIR/evidence" "$EVIDENCE_DIR/"
        pass "Copied additional evidence files"
    else
        log_verbose "Evidence directory is empty"
    fi
else
    log_verbose "No evidence directory found (not required)"
fi

# Copy campaign README
if [[ -f "$CAMPAIGN_DIR/README.md" ]]; then
    cp "$CAMPAIGN_DIR/README.md" "$EVIDENCE_DIR/"
    pass "Copied campaign README"
fi

# Generate checksums if requested
if [[ $GENERATE_CHECKSUMS -eq 1 ]]; then
    section "Generating Checksums"
    
    CHECKSUM_FILE="$EVIDENCE_DIR/SHA256SUMS.txt"
    
    log_info "Computing SHA256 checksums for all files..."
    
    pushd "$EVIDENCE_DIR" >/dev/null
    find . -type f ! -name "SHA256SUMS.txt" -exec sha256sum {} \; | sort -k2 > "$CHECKSUM_FILE"
    popd >/dev/null
    
    CHECKSUM_COUNT=$(wc -l < "$CHECKSUM_FILE" | tr -d ' ')
    pass "Generated checksums for $CHECKSUM_COUNT file(s)"
fi

# Create evidence manifest
section "Creating Evidence Manifest"

MANIFEST_FILE="$EVIDENCE_DIR/MANIFEST.txt"
COLLECTION_TIME=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

cat > "$MANIFEST_FILE" << EOF
EVIDENCE PACKAGE MANIFEST
=========================

Campaign: $CAMPAIGN_NAME
Collection Time: $COLLECTION_TIME
Format: $ARCHIVE_FORMAT

Directory Structure:
-------------------
EOF

pushd "$EVIDENCE_DIR" >/dev/null
tree -L 3 >> "$MANIFEST_FILE" 2>/dev/null || find . -type f | sort >> "$MANIFEST_FILE"
popd >/dev/null

cat >> "$MANIFEST_FILE" << EOF

File Counts:
-----------
Test Cases: ${TESTCASE_COUNT:-0}
Execution Logs: ${LOG_COUNT:-0}
Verification Results: ${VERIFY_COUNT:-0}

EOF

if [[ $GENERATE_CHECKSUMS -eq 1 ]]; then
    echo "Checksums: See SHA256SUMS.txt" >> "$MANIFEST_FILE"
fi

pass "Created evidence manifest: MANIFEST.txt"

# Create archive
section "Creating Evidence Archive"

log_info "Creating $ARCHIVE_FORMAT archive..."

case "$ARCHIVE_FORMAT" in
    tar.gz)
        tar -czf "$OUTPUT_FILE" -C "$TEMP_DIR" "$(basename "$EVIDENCE_DIR")" 2>&1 | while IFS= read -r line; do
            log_verbose "$line"
        done
        ;;
    tar.bz2)
        tar -cjf "$OUTPUT_FILE" -C "$TEMP_DIR" "$(basename "$EVIDENCE_DIR")" 2>&1 | while IFS= read -r line; do
            log_verbose "$line"
        done
        ;;
    zip)
        pushd "$TEMP_DIR" >/dev/null
        zip -r "$OUTPUT_FILE" "$(basename "$EVIDENCE_DIR")" >/dev/null 2>&1
        popd >/dev/null
        ;;
esac

if [[ ! -f "$OUTPUT_FILE" ]]; then
    fail "Failed to create archive: $OUTPUT_FILE"
    exit 1
fi

ARCHIVE_SIZE=$(du -h "$OUTPUT_FILE" | cut -f1)
pass "Created archive: $OUTPUT_FILE ($ARCHIVE_SIZE)"

# Generate archive checksum
if [[ $GENERATE_CHECKSUMS -eq 1 ]]; then
    log_info "Computing archive checksum..."
    ARCHIVE_CHECKSUM=$(sha256sum "$OUTPUT_FILE" | cut -d' ' -f1)
    echo "$ARCHIVE_CHECKSUM  $(basename "$OUTPUT_FILE")" > "${OUTPUT_FILE}.sha256"
    pass "Archive SHA256: $ARCHIVE_CHECKSUM"
    pass "Checksum saved to: ${OUTPUT_FILE}.sha256"
fi

# Generate evidence collection report
section "Generating Evidence Report"

REPORT_FILE="${OUTPUT_FILE%.${ARCHIVE_FORMAT}}_report.txt"

cat > "$REPORT_FILE" << EOF
EVIDENCE COLLECTION REPORT
==========================

Campaign Information:
--------------------
Name: $CAMPAIGN_NAME
Campaign Directory: $CAMPAIGN_DIR

Collection Details:
------------------
Collection Time: $COLLECTION_TIME
Collected By: $(whoami)
Hostname: $(hostname)

Evidence Package:
----------------
Archive File: $OUTPUT_FILE
Archive Size: $ARCHIVE_SIZE
Archive Format: $ARCHIVE_FORMAT
EOF

if [[ $GENERATE_CHECKSUMS -eq 1 ]]; then
    cat >> "$REPORT_FILE" << EOF
Archive SHA256: $ARCHIVE_CHECKSUM
Checksum File: ${OUTPUT_FILE}.sha256
EOF
fi

cat >> "$REPORT_FILE" << EOF

Contents Summary:
----------------
Test Cases: ${TESTCASE_COUNT:-0} files
Execution Logs: ${LOG_COUNT:-0} files
Verification Results: ${VERIFY_COUNT:-0} files

Included Directories:
--------------------
- testcases/           Test case YAML files
- execution_logs/      JSON execution logs
- verification_results/ Verification results (JSON/YAML)
- metadata/            Campaign metadata and run information
- reports/             Generated test reports (if any)
- evidence/            Additional evidence files (if any)

Files in Archive:
----------------
EOF

case "$ARCHIVE_FORMAT" in
    tar.gz|tar.bz2)
        tar -tzf "$OUTPUT_FILE" >> "$REPORT_FILE" 2>/dev/null || echo "(file list unavailable)" >> "$REPORT_FILE"
        ;;
    zip)
        unzip -l "$OUTPUT_FILE" >> "$REPORT_FILE" 2>/dev/null || echo "(file list unavailable)" >> "$REPORT_FILE"
        ;;
esac

pass "Generated evidence report: $REPORT_FILE"

# Final summary
section "Evidence Collection Complete"
echo ""
info "Evidence Package Summary:"
echo "  Campaign: $CAMPAIGN_NAME"
echo "  Archive: $OUTPUT_FILE"
echo "  Size: $ARCHIVE_SIZE"
echo "  Format: $ARCHIVE_FORMAT"
if [[ $GENERATE_CHECKSUMS -eq 1 ]]; then
    echo "  SHA256: $ARCHIVE_CHECKSUM"
fi
echo ""
info "Generated Files:"
echo "  📦 Archive: $OUTPUT_FILE"
if [[ $GENERATE_CHECKSUMS -eq 1 ]]; then
    echo "  🔒 Checksum: ${OUTPUT_FILE}.sha256"
fi
echo "  📄 Report: $REPORT_FILE"
echo ""
pass "Evidence collection completed successfully!"

exit 0
