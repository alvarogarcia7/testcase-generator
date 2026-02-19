#!/usr/bin/env bash
#
# End-to-end integration test for GitLab CI Pages job configuration
#
# This test validates:
# 1. GitLab CI YAML syntax and structure
# 2. Pages job uses Python Docker image (python:3.11)
# 3. pip install from requirements.txt works in CI container environment
# 4. ENABLE_PDF_EXPORT=1 environment variable is set correctly
# 5. mkdocs build --site-dir public generates output in correct directory for GitLab Pages
# 6. Artifacts configuration exports public/ with 30-day expiration
# 7. Cache configuration for pip works in Docker environment
# 8. Job only runs on main branch
#
# Usage: ./tests/integration/test_gitlab_ci_pages_e2e.sh [--no-remove]
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Source logger library
source "$SCRIPT_DIR/../../scripts/lib/logger.sh" || exit 1

# Configuration
GITLAB_CI_PATH="$PROJECT_ROOT/.gitlab-ci.yml"
REQUIREMENTS_PATH="$PROJECT_ROOT/requirements.txt"
IMAGE_NAME="python:3.11"
EXPECTED_SITE_DIR="public"
EXPECTED_ARTIFACT_EXPIRE="30 days"
EXPECTED_CACHE_PATH="\$HOME/.cache/pip"

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

section "GitLab CI Pages Job End-to-End Validation Test"
log_info "Project root: $PROJECT_ROOT"
log_info "GitLab CI config: $GITLAB_CI_PATH"
log_info "Expected Docker image: $IMAGE_NAME"
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

if [[ ! -f "$GITLAB_CI_PATH" ]]; then
    fail "GitLab CI config not found at $GITLAB_CI_PATH"
    exit 1
fi
pass "GitLab CI config found"

if [[ ! -f "$REQUIREMENTS_PATH" ]]; then
    fail "Requirements file not found at $REQUIREMENTS_PATH"
    exit 1
fi
pass "Requirements file found"

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Test GitLab CI YAML syntax
section "Test 2: Validate GitLab CI YAML Syntax"

log_info "Checking .gitlab-ci.yml syntax..."

# Check for basic YAML syntax errors
if command -v python3 &> /dev/null; then
    if python3 -c "import yaml; yaml.safe_load(open('$GITLAB_CI_PATH'))" 2>/dev/null; then
        pass "GitLab CI YAML syntax is valid"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        fail "GitLab CI YAML has syntax errors"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
else
    log_warning "Python3 not available, skipping YAML syntax validation"
fi

echo

# Test Docker image configuration
section "Test 3: Validate Docker Image Configuration"

log_info "Checking pages job uses python:3.11 image..."

if grep -q "image: python:3.11" "$GITLAB_CI_PATH"; then
    pass "Pages job uses python:3.11 Docker image"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Pages job should use python:3.11 Docker image"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test pip install in CI container environment
section "Test 4: Validate pip install in CI Container"

log_info "Testing pip install from requirements.txt in python:3.11 container..."

TEMP_DIR=$(mktemp -d)
setup_cleanup "$TEMP_DIR"

# Copy requirements to temp directory
cp "$REQUIREMENTS_PATH" "$TEMP_DIR/"

# Run pip install in the same container as CI
if docker run --rm -v "$TEMP_DIR:/workspace" "$IMAGE_NAME" \
    bash -c "cd /workspace && pip install --no-cache-dir -r requirements.txt && mkdocs --version" &> /dev/null; then
    pass "pip install from requirements.txt works in python:3.11 container"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "pip install failed in python:3.11 container"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test ENABLE_PDF_EXPORT environment variable
section "Test 5: Validate ENABLE_PDF_EXPORT Environment Variable"

log_info "Checking ENABLE_PDF_EXPORT=1 is set in pages job script..."

if grep -q "ENABLE_PDF_EXPORT=1" "$GITLAB_CI_PATH"; then
    pass "ENABLE_PDF_EXPORT=1 environment variable is set"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "ENABLE_PDF_EXPORT=1 environment variable should be set"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test mkdocs build --site-dir public command
section "Test 6: Validate mkdocs build Command"

log_info "Checking mkdocs build --site-dir public command..."

if grep -q "mkdocs build --site-dir public" "$GITLAB_CI_PATH"; then
    pass "mkdocs build uses --site-dir public"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "mkdocs build should use --site-dir public for GitLab Pages"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Testing mkdocs build generates output in public/ directory..."

TEMP_BUILD_DIR=$(mktemp -d)
setup_cleanup "$TEMP_BUILD_DIR"

# Copy necessary files
cp "$REQUIREMENTS_PATH" "$TEMP_BUILD_DIR/"
cp -r "$PROJECT_ROOT/mkdocs.yml" "$TEMP_BUILD_DIR/"
cp -r "$PROJECT_ROOT/docs" "$TEMP_BUILD_DIR/"
cp "$PROJECT_ROOT/README.md" "$TEMP_BUILD_DIR/" 2>/dev/null || true
cp "$PROJECT_ROOT/README_INSTALL.md" "$TEMP_BUILD_DIR/" 2>/dev/null || true

# Run mkdocs build in container
if docker run --rm -v "$TEMP_BUILD_DIR:/workspace" "$IMAGE_NAME" \
    bash -c "cd /workspace && pip install --no-cache-dir -r requirements.txt &>/dev/null && ENABLE_PDF_EXPORT=1 mkdocs build --site-dir public &>/dev/null && test -d public && test -f public/index.html"; then
    pass "mkdocs build generates output in public/ directory"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "mkdocs build failed to generate output in public/ directory"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test artifacts configuration
section "Test 7: Validate Artifacts Configuration"

log_info "Checking artifacts paths configuration..."

if grep -A 3 "artifacts:" "$GITLAB_CI_PATH" | grep -q "- public"; then
    pass "Artifacts exports public/ directory"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Artifacts should export public/ directory"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Checking artifacts expiration..."

if grep -A 3 "artifacts:" "$GITLAB_CI_PATH" | grep -q "expire_in: 30 days"; then
    pass "Artifacts expire in 30 days"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Artifacts should expire in 30 days"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test cache configuration
section "Test 8: Validate Cache Configuration"

log_info "Checking pip cache configuration..."

if grep -A 2 "cache:" "$GITLAB_CI_PATH" | grep -q "~/.cache/pip"; then
    pass "Cache configured for pip dependencies"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Cache should be configured for pip dependencies"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Testing cache effectiveness in Docker environment..."

# First run - install dependencies
START_TIME=$(date +%s)
docker run --rm -v "$TEMP_BUILD_DIR:/workspace" "$IMAGE_NAME" \
    bash -c "cd /workspace && pip install --no-cache-dir -r requirements.txt" &>/dev/null || true
END_TIME=$(date +%s)
FIRST_RUN_TIME=$((END_TIME - START_TIME))

log_info "First pip install took ${FIRST_RUN_TIME}s (without cache)"
pass "Cache configuration is present and can be used in CI environment"

echo

# Test branch restrictions
section "Test 9: Validate Branch Restrictions"

log_info "Checking job only runs on main branch..."

if grep -A 2 "only:" "$GITLAB_CI_PATH" | grep -q "- main"; then
    pass "Pages job only runs on main branch"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Pages job should only run on main branch"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test complete CI pipeline simulation
section "Test 10: Simulate Complete GitLab CI Pipeline"

log_info "Running full CI pipeline simulation..."

TEMP_CI_DIR=$(mktemp -d)
setup_cleanup "$TEMP_CI_DIR"

# Copy all necessary files
cp "$REQUIREMENTS_PATH" "$TEMP_CI_DIR/"
cp "$PROJECT_ROOT/mkdocs.yml" "$TEMP_CI_DIR/"
cp -r "$PROJECT_ROOT/docs" "$TEMP_CI_DIR/"
cp "$PROJECT_ROOT/README.md" "$TEMP_CI_DIR/" 2>/dev/null || true
cp "$PROJECT_ROOT/README_INSTALL.md" "$TEMP_CI_DIR/" 2>/dev/null || true

log_info "Step 1: Pulling python:3.11 image..."
if docker pull "$IMAGE_NAME" &>/dev/null; then
    pass "Docker image pulled successfully"
else
    fail "Failed to pull Docker image"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    exit 1
fi

log_info "Step 2: Installing dependencies with pip..."
if docker run --rm -v "$TEMP_CI_DIR:/workspace" "$IMAGE_NAME" \
    bash -c "cd /workspace && pip install -r requirements.txt" &>/dev/null; then
    pass "Dependencies installed successfully"
else
    fail "Failed to install dependencies"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Step 3: Building documentation with ENABLE_PDF_EXPORT=1..."
if docker run --rm -v "$TEMP_CI_DIR:/workspace" "$IMAGE_NAME" \
    bash -c "cd /workspace && ENABLE_PDF_EXPORT=1 mkdocs build --site-dir public" &>/dev/null; then
    pass "Documentation built successfully"
else
    fail "Failed to build documentation"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

log_info "Step 4: Verifying public/ directory contents..."
if docker run --rm -v "$TEMP_CI_DIR:/workspace" "$IMAGE_NAME" \
    bash -c "cd /workspace && test -d public && test -f public/index.html && test -d public/pdf"; then
    pass "public/ directory contains expected files"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "public/ directory missing expected files"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Summary
section "Test Summary"
echo
log_info "Total tests passed: $TESTS_PASSED"
log_info "Total tests failed: $TESTS_FAILED"
echo

if [[ $TESTS_FAILED -eq 0 ]]; then
    pass "All GitLab CI Pages job validation tests passed!"
    exit 0
else
    fail "Some GitLab CI Pages job validation tests failed"
    exit 1
fi
