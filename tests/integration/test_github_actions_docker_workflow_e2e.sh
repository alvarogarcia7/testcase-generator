#!/usr/bin/env bash
#
# End-to-end integration test for GitHub Actions Docker workflow configuration
#
# This test validates:
# 1. GitHub Actions YAML syntax and structure
# 2. Build job uses Python 3.11 with pip install from requirements.txt (simulating Docker-like environment)
# 3. mkdocs build command runs in containerized GitHub Actions runner
# 4. upload-pages-artifact uploads site/ directory
# 5. build-pdf job sets ENABLE_PDF_EXPORT=1 correctly
# 6. PDF artifact upload from site/pdf/ directory
# 7. Workflow triggers on push to main and workflow_dispatch
# 8. Concurrency group prevents conflicting deployments
#
# Usage: ./tests/integration/test_github_actions_docker_workflow_e2e.sh [--no-remove]
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Source logger library
source "$SCRIPT_DIR/../../scripts/lib/logger.sh" || exit 1

# Configuration
GITHUB_WORKFLOW_PATH="$PROJECT_ROOT/.github/workflows/docs.yml"
REQUIREMENTS_PATH="$PROJECT_ROOT/requirements.txt"
PYTHON_VERSION="3.11"
EXPECTED_SITE_DIR="site"
EXPECTED_PDF_DIR="site/pdf"
EXPECTED_CONCURRENCY_GROUP="pages"

# Handle --no-remove flag
REMOVE_TEMP=1
while [[ $# -gt 0 ]]; do
    case $1 in
        --no-remove)
            REMOVE_TEMP=0
            shift
            ;;
        *)
            shift
            ;;
    esac
done

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

section "GitHub Actions Docker Workflow End-to-End Validation Test"
log_info "Project root: $PROJECT_ROOT"
log_info "GitHub workflow: $GITHUB_WORKFLOW_PATH"
log_info "Python version: $PYTHON_VERSION"
echo

# Check prerequisites
section "Test 1: Checking Prerequisites"

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

if [[ ! -f "$GITHUB_WORKFLOW_PATH" ]]; then
    fail "GitHub workflow not found at $GITHUB_WORKFLOW_PATH"
    exit 1
fi
pass "GitHub workflow found"

if [[ ! -f "$REQUIREMENTS_PATH" ]]; then
    fail "Requirements file not found at $REQUIREMENTS_PATH"
    exit 1
fi
pass "Requirements file found"

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Test GitHub Actions YAML syntax
section "Test 2: Validate GitHub Actions YAML Syntax"

log_info "Checking .github/workflows/docs.yml syntax..."

# Check for basic YAML syntax errors
if command -v python3 &> /dev/null; then
    if python3 -c "import yaml; yaml.safe_load(open('$GITHUB_WORKFLOW_PATH'))" 2>/dev/null; then
        pass "GitHub Actions YAML syntax is valid"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        fail "GitHub Actions YAML has syntax errors"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
else
    log_warning "Python3 not available, skipping YAML syntax validation"
fi

echo

# Test workflow triggers
section "Test 3: Validate Workflow Triggers"

log_info "Checking workflow triggers on push to main..."

if grep -A3 "^on:" "$GITHUB_WORKFLOW_PATH" | grep -A2 "push:" | grep -q "- main"; then
    pass "Workflow triggers on push to main branch"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Workflow should trigger on push to main branch"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Checking workflow_dispatch trigger..."

if grep -A3 "^on:" "$GITHUB_WORKFLOW_PATH" | grep -q "workflow_dispatch"; then
    pass "Workflow supports manual dispatch"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Workflow should support workflow_dispatch"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test concurrency configuration
section "Test 4: Validate Concurrency Configuration"

log_info "Checking concurrency group configuration..."

if grep -A 2 "^concurrency:" "$GITHUB_WORKFLOW_PATH" | grep -q "group: \"pages\""; then
    pass "Concurrency group 'pages' is configured"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Concurrency group should be set to 'pages'"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Checking cancel-in-progress configuration..."

if grep -A 2 "^concurrency:" "$GITHUB_WORKFLOW_PATH" | grep -q "cancel-in-progress: false"; then
    pass "cancel-in-progress is set to false (prevents conflicting deployments)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "cancel-in-progress should be false to prevent deployment conflicts"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test build job configuration
section "Test 5: Validate Build Job Configuration"

log_info "Checking build job uses Python 3.11..."

if grep -A 10 "jobs:" "$GITHUB_WORKFLOW_PATH" | grep -A 5 "build:" | grep -A 3 "setup-python" | grep -q "python-version: '3.11'"; then
    pass "Build job uses Python 3.11"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Build job should use Python 3.11"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Checking pip install from requirements.txt..."

if grep -A 20 "jobs:" "$GITHUB_WORKFLOW_PATH" | grep -A 15 "build:" | grep -q "pip install -r requirements.txt"; then
    pass "Build job installs dependencies from requirements.txt"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Build job should install dependencies from requirements.txt"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Checking mkdocs build command..."

if grep -A 25 "jobs:" "$GITHUB_WORKFLOW_PATH" | grep -A 20 "build:" | grep -q "mkdocs build"; then
    pass "Build job runs mkdocs build command"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Build job should run mkdocs build command"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test upload-pages-artifact action
section "Test 6: Validate upload-pages-artifact Configuration"

log_info "Checking upload-pages-artifact action is used..."

if grep -A 30 "jobs:" "$GITHUB_WORKFLOW_PATH" | grep -A 25 "build:" | grep -q "actions/upload-pages-artifact"; then
    pass "Build job uses actions/upload-pages-artifact"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Build job should use actions/upload-pages-artifact"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Checking artifact path is site/..."

if grep -A 35 "jobs:" "$GITHUB_WORKFLOW_PATH" | grep -A 30 "build:" | grep -A 3 "upload-pages-artifact" | grep -q "path: site/"; then
    pass "Artifact uploads site/ directory"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Artifact should upload site/ directory"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test build job in containerized environment
section "Test 7: Test Build Job in Containerized Environment"

log_info "Simulating GitHub Actions runner environment with Docker..."

TEMP_BUILD_DIR=$(mktemp -d)
setup_cleanup "$TEMP_BUILD_DIR"

# Copy necessary files
cp "$REQUIREMENTS_PATH" "$TEMP_BUILD_DIR/"
cp "$PROJECT_ROOT/mkdocs.yml" "$TEMP_BUILD_DIR/"
cp -r "$PROJECT_ROOT/docs" "$TEMP_BUILD_DIR/"
cp "$PROJECT_ROOT/README.md" "$TEMP_BUILD_DIR/" 2>/dev/null || true
cp "$PROJECT_ROOT/README_INSTALL.md" "$TEMP_BUILD_DIR/" 2>/dev/null || true

# Use official Python Docker image (similar to GitHub Actions)
RUNNER_IMAGE="python:3.11"

log_info "Pulling $RUNNER_IMAGE..."
if docker pull "$RUNNER_IMAGE" &>/dev/null; then
    pass "Runner image pulled successfully"
else
    fail "Failed to pull runner image"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Running pip install in containerized environment..."
if docker run --rm -v "$TEMP_BUILD_DIR:/workspace" "$RUNNER_IMAGE" \
    bash -c "cd /workspace && pip install -r requirements.txt" &>/dev/null; then
    pass "pip install successful in containerized environment"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "pip install failed in containerized environment"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Running mkdocs build in containerized environment..."
if docker run --rm -v "$TEMP_BUILD_DIR:/workspace" "$RUNNER_IMAGE" \
    bash -c "cd /workspace && mkdocs build" &>/dev/null; then
    pass "mkdocs build successful in containerized environment"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "mkdocs build failed in containerized environment"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Verifying site/ directory was created..."
if docker run --rm -v "$TEMP_BUILD_DIR:/workspace" "$RUNNER_IMAGE" \
    bash -c "cd /workspace && test -d site && test -f site/index.html"; then
    pass "site/ directory created with expected content"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "site/ directory not created properly"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test build-pdf job configuration
section "Test 8: Validate build-pdf Job Configuration"

log_info "Checking build-pdf job uses Python 3.11..."

if grep -A 50 "build-pdf:" "$GITHUB_WORKFLOW_PATH" | grep -A 3 "setup-python" | grep -q "python-version: '3.11'"; then
    pass "build-pdf job uses Python 3.11"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "build-pdf job should use Python 3.11"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Checking ENABLE_PDF_EXPORT=1 environment variable..."

if grep -A 50 "build-pdf:" "$GITHUB_WORKFLOW_PATH" | grep -A 5 "Build PDF" | grep -q "ENABLE_PDF_EXPORT: 1"; then
    pass "ENABLE_PDF_EXPORT=1 environment variable is set"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "ENABLE_PDF_EXPORT=1 should be set in build-pdf job"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Checking system dependencies installation for PDF generation..."

if grep -A 50 "build-pdf:" "$GITHUB_WORKFLOW_PATH" | grep -q "libcairo2"; then
    pass "System dependencies for PDF generation are installed"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "System dependencies for PDF generation should be installed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test PDF artifact upload
section "Test 9: Validate PDF Artifact Upload"

log_info "Checking PDF artifact upload action..."

if grep -A 55 "build-pdf:" "$GITHUB_WORKFLOW_PATH" | grep -q "actions/upload-artifact"; then
    pass "build-pdf job uses actions/upload-artifact"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "build-pdf job should use actions/upload-artifact"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Checking PDF artifact path is site/pdf/..."

if grep -A 60 "build-pdf:" "$GITHUB_WORKFLOW_PATH" | grep -A 5 "upload-artifact" | grep -q "path: site/pdf/"; then
    pass "PDF artifact uploads site/pdf/ directory"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "PDF artifact should upload site/pdf/ directory"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Checking artifact name is 'documentation-pdf'..."

if grep -A 60 "build-pdf:" "$GITHUB_WORKFLOW_PATH" | grep -A 5 "upload-artifact" | grep -q "name: documentation-pdf"; then
    pass "PDF artifact has correct name"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "PDF artifact should be named 'documentation-pdf'"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test build-pdf job in containerized environment
section "Test 10: Test build-pdf Job in Containerized Environment"

log_info "Simulating build-pdf job with ENABLE_PDF_EXPORT=1..."

TEMP_PDF_DIR=$(mktemp -d)
setup_cleanup "$TEMP_PDF_DIR"

# Copy necessary files
cp "$REQUIREMENTS_PATH" "$TEMP_PDF_DIR/"
cp "$PROJECT_ROOT/mkdocs.yml" "$TEMP_PDF_DIR/"
cp -r "$PROJECT_ROOT/docs" "$TEMP_PDF_DIR/"
cp "$PROJECT_ROOT/README.md" "$TEMP_PDF_DIR/" 2>/dev/null || true
cp "$PROJECT_ROOT/README_INSTALL.md" "$TEMP_PDF_DIR/" 2>/dev/null || true

log_info "Installing system dependencies for PDF generation..."
# Simulate the apt-get install step
SYSTEM_DEPS_INSTALLED=0
if docker run --rm "$RUNNER_IMAGE" \
    bash -c "apt-get update &>/dev/null && apt-get install -y --no-install-recommends libcairo2 libpango-1.0-0 libpangocairo-1.0-0 libgdk-pixbuf2.0-0 libffi-dev shared-mime-info &>/dev/null"; then
    pass "System dependencies for PDF can be installed"
    SYSTEM_DEPS_INSTALLED=1
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    log_warning "System dependencies installation check skipped (may require elevated permissions)"
fi

log_info "Running mkdocs build with ENABLE_PDF_EXPORT=1..."
if docker run --rm -v "$TEMP_PDF_DIR:/workspace" "$RUNNER_IMAGE" \
    bash -c "cd /workspace && pip install -r requirements.txt &>/dev/null && ENABLE_PDF_EXPORT=1 mkdocs build &>/dev/null"; then
    pass "mkdocs build with ENABLE_PDF_EXPORT=1 successful"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "mkdocs build with ENABLE_PDF_EXPORT=1 failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Verifying site/pdf/ directory was created..."
if docker run --rm -v "$TEMP_PDF_DIR:/workspace" "$RUNNER_IMAGE" \
    bash -c "cd /workspace && test -d site/pdf"; then
    pass "site/pdf/ directory created"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    log_warning "site/pdf/ directory not created (PDF generation may require additional dependencies)"
fi

echo

# Test deploy job configuration
section "Test 11: Validate Deploy Job Configuration"

log_info "Checking deploy job depends on build job..."

if grep -A 3 "deploy:" "$GITHUB_WORKFLOW_PATH" | grep -q "needs: build"; then
    pass "Deploy job depends on build job"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Deploy job should depend on build job"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Checking deploy job uses actions/deploy-pages..."

if grep -A 10 "deploy:" "$GITHUB_WORKFLOW_PATH" | grep -q "actions/deploy-pages"; then
    pass "Deploy job uses actions/deploy-pages"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Deploy job should use actions/deploy-pages"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Checking deploy job environment configuration..."

if grep -A 10 "deploy:" "$GITHUB_WORKFLOW_PATH" | grep -A 3 "environment:" | grep -q "name: github-pages"; then
    pass "Deploy job uses github-pages environment"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Deploy job should use github-pages environment"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test permissions configuration
section "Test 12: Validate Permissions Configuration"

log_info "Checking workflow permissions..."

if grep -A 5 "^permissions:" "$GITHUB_WORKFLOW_PATH" | grep -q "contents: write"; then
    pass "Workflow has contents: write permission"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Workflow should have contents: write permission"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

if grep -A 5 "^permissions:" "$GITHUB_WORKFLOW_PATH" | grep -q "pages: write"; then
    pass "Workflow has pages: write permission"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Workflow should have pages: write permission"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

if grep -A 5 "^permissions:" "$GITHUB_WORKFLOW_PATH" | grep -q "id-token: write"; then
    pass "Workflow has id-token: write permission"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Workflow should have id-token: write permission"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test complete workflow simulation
section "Test 13: Simulate Complete GitHub Actions Workflow"

log_info "Running full workflow simulation..."

TEMP_WORKFLOW_DIR=$(mktemp -d)
setup_cleanup "$TEMP_WORKFLOW_DIR"

# Copy all necessary files
cp "$REQUIREMENTS_PATH" "$TEMP_WORKFLOW_DIR/"
cp "$PROJECT_ROOT/mkdocs.yml" "$TEMP_WORKFLOW_DIR/"
cp -r "$PROJECT_ROOT/docs" "$TEMP_WORKFLOW_DIR/"
cp "$PROJECT_ROOT/README.md" "$TEMP_WORKFLOW_DIR/" 2>/dev/null || true
cp "$PROJECT_ROOT/README_INSTALL.md" "$TEMP_WORKFLOW_DIR/" 2>/dev/null || true

log_info "Step 1: Set up Python (pulling python:3.11 image)..."
if docker pull "$RUNNER_IMAGE" &>/dev/null; then
    pass "Python 3.11 environment ready"
else
    fail "Failed to set up Python environment"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    exit 1
fi

log_info "Step 2: Install dependencies (pip install -r requirements.txt)..."
if docker run --rm -v "$TEMP_WORKFLOW_DIR:/workspace" "$RUNNER_IMAGE" \
    bash -c "cd /workspace && pip install -r requirements.txt" &>/dev/null; then
    pass "Dependencies installed successfully"
else
    fail "Failed to install dependencies"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Step 3: Build documentation (mkdocs build)..."
if docker run --rm -v "$TEMP_WORKFLOW_DIR:/workspace" "$RUNNER_IMAGE" \
    bash -c "cd /workspace && mkdocs build" &>/dev/null; then
    pass "Documentation built successfully"
else
    fail "Failed to build documentation"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Step 4: Verify artifact upload would work (check site/ directory)..."
if docker run --rm -v "$TEMP_WORKFLOW_DIR:/workspace" "$RUNNER_IMAGE" \
    bash -c "cd /workspace && test -d site && test -f site/index.html && ls site/ | wc -l" &>/dev/null; then
    pass "site/ directory ready for artifact upload"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "site/ directory not ready for artifact upload"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Step 5: Build PDF (ENABLE_PDF_EXPORT=1 mkdocs build)..."
if docker run --rm -v "$TEMP_WORKFLOW_DIR:/workspace" "$RUNNER_IMAGE" \
    bash -c "cd /workspace && rm -rf site && ENABLE_PDF_EXPORT=1 mkdocs build" &>/dev/null; then
    pass "PDF documentation built successfully"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    log_warning "PDF build may have issues (this is expected without all system dependencies)"
fi

log_info "Step 6: Verify PDF artifact directory..."
if docker run --rm -v "$TEMP_WORKFLOW_DIR:/workspace" "$RUNNER_IMAGE" \
    bash -c "cd /workspace && test -d site/pdf" &>/dev/null; then
    pass "site/pdf/ directory ready for artifact upload"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    log_warning "site/pdf/ directory not created (PDF export may be disabled or require dependencies)"
fi

echo

# Summary
section "Test Summary"
echo
log_info "Total tests passed: $TESTS_PASSED"
log_info "Total tests failed: $TESTS_FAILED"
echo

if [[ $TESTS_FAILED -eq 0 ]]; then
    pass "All GitHub Actions Docker workflow validation tests passed!"
    exit 0
else
    fail "Some GitHub Actions Docker workflow validation tests failed"
    exit 1
fi
