#!/usr/bin/env bash
#
# Simulate GitHub Actions workflow locally
#
# This script simulates the GitHub Actions documentation workflow locally
# for testing and validation before pushing changes to GitHub.
#
# Usage: ./scripts/simulate-github-actions-workflow.sh [options]
#
# Options:
#   --clean         Clean previous simulation artifacts
#   --keep-temp     Keep temporary files for debugging
#   --verbose       Enable verbose output
#   --help          Show this help message
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source logger library
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

# Configuration
GITHUB_RUNNER_IMAGE="python:3.11"
OUTPUT_DIR="$PROJECT_ROOT/site"
TEMP_CONTAINER_NAME="github-actions-workflow-sim-$$"
CLEAN_FIRST=0
KEEP_TEMP=0

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --clean)
            CLEAN_FIRST=1
            shift
            ;;
        --keep-temp)
            KEEP_TEMP=1
            shift
            ;;
        --verbose)
            VERBOSE=1
            shift
            ;;
        --help)
            grep '^#' "$0" | sed 's/^# \?//'
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

section "GitHub Actions Workflow Simulation"
log_info "Project root: $PROJECT_ROOT"
log_info "Docker image: $GITHUB_RUNNER_IMAGE"
log_info "Output directory: $OUTPUT_DIR"
echo

# Clean previous artifacts if requested
if [[ $CLEAN_FIRST -eq 1 ]]; then
    section "Cleaning Previous Artifacts"
    if [[ -d "$OUTPUT_DIR" ]]; then
        log_info "Removing existing site/ directory..."
        rm -rf "$OUTPUT_DIR"
        pass "Cleaned previous artifacts"
    else
        log_info "No previous artifacts to clean"
    fi
    echo
fi

# Check prerequisites
section "Step 1: Checking Prerequisites"

if ! command -v docker &> /dev/null; then
    fail "Docker is not installed or not in PATH"
    log_error "Please install Docker from https://www.docker.com/get-started"
    exit 1
fi
pass "Docker is installed"

if ! docker info &> /dev/null; then
    fail "Docker daemon is not running"
    log_error "Please start Docker and try again"
    exit 1
fi
pass "Docker daemon is running"

REQUIRED_FILES=(
    "$PROJECT_ROOT/.github/workflows/docs.yml"
    "$PROJECT_ROOT/requirements.txt"
    "$PROJECT_ROOT/mkdocs.yml"
)

for file in "${REQUIRED_FILES[@]}"; do
    if [[ ! -f "$file" ]]; then
        fail "Required file not found: $(basename "$file")"
        exit 1
    fi
done
pass "All required files found"

if [[ ! -d "$PROJECT_ROOT/docs" ]]; then
    fail "docs/ directory not found"
    exit 1
fi
pass "docs/ directory found"

echo

# Pull Docker image
section "Step 2: Setting Up Python Environment"

log_info "Pulling $GITHUB_RUNNER_IMAGE..."
if docker pull "$GITHUB_RUNNER_IMAGE" &>/dev/null; then
    pass "Docker image pulled successfully"
else
    fail "Failed to pull Docker image"
    exit 1
fi

echo

# Create temporary working directory
section "Step 3: Preparing Working Directory"

TEMP_DIR=$(mktemp -d)
if [[ $KEEP_TEMP -eq 0 ]]; then
    setup_cleanup "$TEMP_DIR"
fi

log_info "Copying project files to temporary directory..."
cp "$PROJECT_ROOT/requirements.txt" "$TEMP_DIR/"
cp "$PROJECT_ROOT/mkdocs.yml" "$TEMP_DIR/"
cp -r "$PROJECT_ROOT/docs" "$TEMP_DIR/"
cp "$PROJECT_ROOT/README.md" "$TEMP_DIR/" 2>/dev/null || true
cp "$PROJECT_ROOT/README_INSTALL.md" "$TEMP_DIR/" 2>/dev/null || true

pass "Working directory prepared: $TEMP_DIR"

if [[ $KEEP_TEMP -eq 1 ]]; then
    log_info "Temporary directory will be preserved at: $TEMP_DIR"
fi

echo

# Install dependencies
section "Step 4: Installing Dependencies"

log_info "Running: pip install -r requirements.txt"
log_verbose "Command: docker run --rm -v $TEMP_DIR:/workspace $GITHUB_RUNNER_IMAGE bash -c 'cd /workspace && pip install -r requirements.txt'"

START_TIME=$(date +%s)
if [[ ${VERBOSE:-0} -eq 1 ]]; then
    docker run --rm -v "$TEMP_DIR:/workspace" "$GITHUB_RUNNER_IMAGE" \
        bash -c "cd /workspace && pip install -r requirements.txt"
    PIP_EXIT=$?
else
    docker run --rm -v "$TEMP_DIR:/workspace" "$GITHUB_RUNNER_IMAGE" \
        bash -c "cd /workspace && pip install -r requirements.txt" &>/dev/null
    PIP_EXIT=$?
fi
END_TIME=$(date +%s)
INSTALL_TIME=$((END_TIME - START_TIME))

if [[ $PIP_EXIT -eq 0 ]]; then
    pass "Dependencies installed successfully (${INSTALL_TIME}s)"
else
    fail "Failed to install dependencies"
    exit 1
fi

echo

# Build documentation
section "Step 5: Building Documentation"

log_info "Running: mkdocs build"
log_verbose "Command: docker run --rm -v $TEMP_DIR:/workspace $GITHUB_RUNNER_IMAGE bash -c 'cd /workspace && mkdocs build'"

START_TIME=$(date +%s)
if [[ ${VERBOSE:-0} -eq 1 ]]; then
    docker run --rm -v "$TEMP_DIR:/workspace" "$GITHUB_RUNNER_IMAGE" \
        bash -c "cd /workspace && mkdocs build"
    BUILD_EXIT=$?
else
    docker run --rm -v "$TEMP_DIR:/workspace" "$GITHUB_RUNNER_IMAGE" \
        bash -c "cd /workspace && mkdocs build" &>/dev/null
    BUILD_EXIT=$?
fi
END_TIME=$(date +%s)
BUILD_TIME=$((END_TIME - START_TIME))

if [[ $BUILD_EXIT -eq 0 ]]; then
    pass "Documentation built successfully (${BUILD_TIME}s)"
else
    fail "Failed to build documentation"
    exit 1
fi

echo

# Verify artifacts
section "Step 6: Verifying Artifacts"

log_info "Checking site/ directory contents..."

if [[ ! -d "$TEMP_DIR/site" ]]; then
    fail "site/ directory was not created"
    exit 1
fi
pass "site/ directory created"

if [[ ! -f "$TEMP_DIR/site/index.html" ]]; then
    fail "index.html not found in site/"
    exit 1
fi
pass "index.html found"

# Count total files
TOTAL_FILES=$(find "$TEMP_DIR/site" -type f 2>/dev/null | wc -l | tr -d ' ')
TOTAL_SIZE=$(du -sh "$TEMP_DIR/site" 2>/dev/null | awk '{print $1}')

log_info "Total files generated: $TOTAL_FILES"
log_info "Total size: $TOTAL_SIZE"

echo

# Copy artifacts to project directory
section "Step 7: Copying Artifacts"

log_info "Copying artifacts to $OUTPUT_DIR..."

if [[ -d "$OUTPUT_DIR" ]]; then
    log_warning "Removing existing site/ directory"
    rm -rf "$OUTPUT_DIR"
fi

cp -r "$TEMP_DIR/site" "$OUTPUT_DIR"
pass "Artifacts copied to project directory"

echo

# Simulate build-pdf job
section "Step 8: Building PDF Documentation"

log_info "Running: ENABLE_PDF_EXPORT=1 mkdocs build"
log_verbose "Command: docker run --rm -v $TEMP_DIR:/workspace $GITHUB_RUNNER_IMAGE bash -c 'cd /workspace && rm -rf site && ENABLE_PDF_EXPORT=1 mkdocs build'"

# Clean previous build
rm -rf "$TEMP_DIR/site"

START_TIME=$(date +%s)
if [[ ${VERBOSE:-0} -eq 1 ]]; then
    docker run --rm -v "$TEMP_DIR:/workspace" "$GITHUB_RUNNER_IMAGE" \
        bash -c "cd /workspace && ENABLE_PDF_EXPORT=1 mkdocs build"
    PDF_EXIT=$?
else
    docker run --rm -v "$TEMP_DIR:/workspace" "$GITHUB_RUNNER_IMAGE" \
        bash -c "cd /workspace && ENABLE_PDF_EXPORT=1 mkdocs build" &>/dev/null
    PDF_EXIT=$?
fi
END_TIME=$(date +%s)
PDF_TIME=$((END_TIME - START_TIME))

if [[ $PDF_EXIT -eq 0 ]]; then
    pass "PDF documentation built successfully (${PDF_TIME}s)"
else
    log_warning "PDF build failed (this is expected without system dependencies)"
fi

if [[ -d "$TEMP_DIR/site/pdf" ]]; then
    pass "PDF documentation directory found"
    
    PDF_COUNT=$(find "$TEMP_DIR/site/pdf" -name "*.pdf" 2>/dev/null | wc -l | tr -d ' ')
    if [[ $PDF_COUNT -gt 0 ]]; then
        pass "PDF files generated: $PDF_COUNT"
        
        # Update site directory with PDF version
        log_info "Updating site/ with PDF documentation..."
        rm -rf "$OUTPUT_DIR"
        cp -r "$TEMP_DIR/site" "$OUTPUT_DIR"
        pass "Site updated with PDF documentation"
    else
        log_warning "PDF directory exists but no PDF files found"
    fi
else
    log_warning "PDF documentation directory not found (PDF export may require system dependencies)"
fi

echo

# Summary
section "Simulation Summary"

pass "GitHub Actions workflow simulated successfully!"
echo
log_info "Workflow steps completed:"
log_info "  ✓ Python 3.11 environment set up"
log_info "  ✓ Dependencies installed (${INSTALL_TIME}s)"
log_info "  ✓ Documentation built (${BUILD_TIME}s)"
log_info "  ✓ Artifacts verified and copied"
if [[ $PDF_EXIT -eq 0 ]]; then
    log_info "  ✓ PDF documentation built (${PDF_TIME}s)"
else
    log_info "  ⚠ PDF documentation skipped (requires system dependencies)"
fi
echo
log_info "Output directory: $OUTPUT_DIR"
log_info "You can view the generated site by running:"
log_info "  python3 -m http.server 8000 --directory $OUTPUT_DIR"
echo
log_info "Or open directly in browser:"
log_info "  open $OUTPUT_DIR/index.html"
echo

if [[ $KEEP_TEMP -eq 1 ]]; then
    log_info "Temporary files preserved at: $TEMP_DIR"
fi

# Show concurrency information
section "Workflow Configuration"
log_info "Concurrency group: pages"
log_info "Cancel in progress: false (prevents conflicting deployments)"
log_info "Triggers: push to main, workflow_dispatch"
echo

exit 0
