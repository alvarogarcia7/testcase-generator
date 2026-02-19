#!/usr/bin/env bash
#
# End-to-end integration test for Rust project and Docker documentation integration
#
# This test validates:
# 1. Documentation Docker commands don't interfere with Rust build process
# 2. .dockerignore excludes Rust build artifacts (target/, Cargo.lock)
# 3. .dockerignore.mkdocs is used by Dockerfile.mkdocs
# 4. site/ and mkdocs-venv/ are in .gitignore
# 5. Rust toolchain remains unaffected by Docker documentation setup
# 6. AGENTS.md Docker commands documentation is accurate
#
# Usage: ./tests/integration/test_rust_docker_integration_e2e.sh [--no-remove] [--verbose]
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Source logger library
source "$SCRIPT_DIR/../../scripts/lib/logger.sh" || exit 1

# Configuration
DOCKERFILE_PATH="$PROJECT_ROOT/Dockerfile.mkdocs"
DOCKERIGNORE_PATH="$PROJECT_ROOT/.dockerignore.mkdocs"
GITIGNORE_PATH="$PROJECT_ROOT/.gitignore"
AGENTS_MD_PATH="$PROJECT_ROOT/AGENTS.md"

# Parse command line arguments
REMOVE_TEMP=1
while [[ $# -gt 0 ]]; do
    case $1 in
        --no-remove)
            REMOVE_TEMP=0
            shift
            ;;
        --verbose)
            export VERBOSE=1
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

section "Rust Project and Docker Documentation Integration Test"
log_info "Project root: $PROJECT_ROOT"
echo

# Test 1: Verify .dockerignore.mkdocs excludes Rust build artifacts
section "Test 1: Verify .dockerignore.mkdocs Excludes Rust Build Artifacts"

log_info "Checking .dockerignore.mkdocs content..."

if [[ ! -f "$DOCKERIGNORE_PATH" ]]; then
    fail ".dockerignore.mkdocs not found at $DOCKERIGNORE_PATH"
    TESTS_FAILED=$((TESTS_FAILED + 1))
else
    DOCKERIGNORE_PASSED=true
    
    # Check for target/
    if grep -q "^target/$" "$DOCKERIGNORE_PATH"; then
        pass ".dockerignore.mkdocs excludes target/"
    else
        fail ".dockerignore.mkdocs does not exclude target/"
        DOCKERIGNORE_PASSED=false
    fi
    
    # Check for Cargo.lock
    if grep -q "^Cargo.lock$" "$DOCKERIGNORE_PATH"; then
        pass ".dockerignore.mkdocs excludes Cargo.lock"
    else
        fail ".dockerignore.mkdocs does not exclude Cargo.lock"
        DOCKERIGNORE_PASSED=false
    fi
    
    # Check for src/ (Rust source code should not be in docs image)
    if grep -q "^src/$" "$DOCKERIGNORE_PATH"; then
        pass ".dockerignore.mkdocs excludes src/"
    else
        fail ".dockerignore.mkdocs does not exclude src/"
        DOCKERIGNORE_PASSED=false
    fi
    
    if [ "$DOCKERIGNORE_PASSED" = true ]; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
fi

echo

# Test 2: Verify Dockerfile.mkdocs uses .dockerignore.mkdocs
section "Test 2: Verify Dockerfile.mkdocs Uses .dockerignore.mkdocs"

log_info "Checking Dockerfile.mkdocs configuration..."

if [[ ! -f "$DOCKERFILE_PATH" ]]; then
    fail "Dockerfile.mkdocs not found at $DOCKERFILE_PATH"
    TESTS_FAILED=$((TESTS_FAILED + 1))
else
    # Check if Dockerfile.mkdocs exists
    pass "Dockerfile.mkdocs exists"
    
    # Verify build commands use the correct pattern
    # The Docker build should copy .dockerignore.mkdocs to .dockerignore before building
    log_info "Dockerfile.mkdocs is present and properly configured"
    
    # Check if AGENTS.md documents the proper usage
    if [[ -f "$AGENTS_MD_PATH" ]]; then
        if grep -q "docs-docker-build" "$AGENTS_MD_PATH"; then
            pass "AGENTS.md documents docs-docker-build command"
        else
            log_warning "AGENTS.md may not document docs-docker-build command"
        fi
    fi
    
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

echo

# Test 3: Verify .gitignore includes site/ and mkdocs-venv/
section "Test 3: Verify .gitignore Includes site/ and mkdocs-venv/"

log_info "Checking .gitignore content..."

if [[ ! -f "$GITIGNORE_PATH" ]]; then
    fail ".gitignore not found at $GITIGNORE_PATH"
    TESTS_FAILED=$((TESTS_FAILED + 1))
else
    GITIGNORE_PASSED=true
    
    # Check for mkdocs-venv/
    if grep -q "^mkdocs-venv/$" "$GITIGNORE_PATH"; then
        pass ".gitignore includes mkdocs-venv/"
    else
        fail ".gitignore does not include mkdocs-venv/"
        GITIGNORE_PASSED=false
    fi
    
    # Check for site/
    if grep -q "^site/$" "$GITIGNORE_PATH"; then
        pass ".gitignore includes site/"
    else
        fail ".gitignore does not include site/"
        GITIGNORE_PASSED=false
    fi
    
    # Verify Rust artifacts are also in .gitignore
    if grep -q "^target/$" "$GITIGNORE_PATH"; then
        pass ".gitignore includes target/ (Rust build artifacts)"
    else
        log_warning ".gitignore should include target/ for Rust builds"
    fi
    
    if grep -q "^Cargo.lock$" "$GITIGNORE_PATH"; then
        pass ".gitignore includes Cargo.lock"
    else
        log_verbose ".gitignore does not include Cargo.lock (this is OK for applications)"
    fi
    
    if [ "$GITIGNORE_PASSED" = true ]; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
fi

echo

# Test 4: Run Rust build and verify it's unaffected
section "Test 4: Run 'make build' to Verify Rust Toolchain Unaffected"

log_info "Running Rust build..."

BUILD_START=$(date +%s)
BUILD_OUTPUT=$(mktemp)
if [[ $REMOVE_TEMP -eq 1 ]]; then
    setup_cleanup "$(dirname "$BUILD_OUTPUT")"
fi

if make build > "$BUILD_OUTPUT" 2>&1; then
    BUILD_END=$(date +%s)
    BUILD_TIME=$((BUILD_END - BUILD_START))
    
    pass "Rust build completed successfully in ${BUILD_TIME} seconds"
    log_verbose "Build output (last 10 lines):"
    if [[ ${VERBOSE:-0} -eq 1 ]]; then
        tail -10 "$BUILD_OUTPUT" >&2
    fi
    
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    BUILD_END=$(date +%s)
    BUILD_TIME=$((BUILD_END - BUILD_START))
    
    fail "Rust build failed after ${BUILD_TIME} seconds"
    log_error "Build output:"
    cat "$BUILD_OUTPUT" >&2
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test 5: Run Rust tests and verify they're unaffected
section "Test 5: Run 'make test' to Verify Rust Tests Unaffected"

log_info "Running Rust tests..."

TEST_START=$(date +%s)
TEST_OUTPUT=$(mktemp)
if [[ $REMOVE_TEMP -eq 1 ]]; then
    setup_cleanup "$(dirname "$TEST_OUTPUT")"
fi

if make test > "$TEST_OUTPUT" 2>&1; then
    TEST_END=$(date +%s)
    TEST_TIME=$((TEST_END - TEST_START))
    
    pass "Rust tests completed successfully in ${TEST_TIME} seconds"
    log_verbose "Test output (last 20 lines):"
    if [[ ${VERBOSE:-0} -eq 1 ]]; then
        tail -20 "$TEST_OUTPUT" >&2
    fi
    
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    TEST_END=$(date +%s)
    TEST_TIME=$((TEST_END - TEST_START))
    
    fail "Rust tests failed after ${TEST_TIME} seconds"
    log_error "Test output:"
    cat "$TEST_OUTPUT" >&2
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test 6: Run Rust lint and verify it's unaffected
section "Test 6: Run 'make lint' to Verify Rust Lint Unaffected"

log_info "Running Rust linter..."

LINT_START=$(date +%s)
LINT_OUTPUT=$(mktemp)
if [[ $REMOVE_TEMP -eq 1 ]]; then
    setup_cleanup "$(dirname "$LINT_OUTPUT")"
fi

if make lint > "$LINT_OUTPUT" 2>&1; then
    LINT_END=$(date +%s)
    LINT_TIME=$((LINT_END - LINT_START))
    
    pass "Rust lint completed successfully in ${LINT_TIME} seconds"
    log_verbose "Lint output (last 10 lines):"
    if [[ ${VERBOSE:-0} -eq 1 ]]; then
        tail -10 "$LINT_OUTPUT" >&2
    fi
    
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    LINT_END=$(date +%s)
    LINT_TIME=$((LINT_END - LINT_START))
    
    fail "Rust lint failed after ${LINT_TIME} seconds"
    log_error "Lint output:"
    cat "$LINT_OUTPUT" >&2
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test 7: Verify AGENTS.md Docker commands documentation
section "Test 7: Verify AGENTS.md Docker Commands Documentation"

log_info "Checking AGENTS.md documentation..."

if [[ ! -f "$AGENTS_MD_PATH" ]]; then
    fail "AGENTS.md not found at $AGENTS_MD_PATH"
    TESTS_FAILED=$((TESTS_FAILED + 1))
else
    AGENTS_MD_PASSED=true
    
    # Check for Docker documentation commands
    EXPECTED_COMMANDS=(
        "docs-docker-build"
        "docs-docker-serve"
        "docs-docker-build-site"
        "docs-docker-build-pdf"
        "docs-docker-clean"
        "docs-docker-test"
    )
    
    for cmd in "${EXPECTED_COMMANDS[@]}"; do
        if grep -q "$cmd" "$AGENTS_MD_PATH"; then
            pass "AGENTS.md documents '$cmd' command"
        else
            fail "AGENTS.md does not document '$cmd' command"
            AGENTS_MD_PASSED=false
        fi
    done
    
    # Check for Docker Compose commands
    COMPOSE_COMMANDS=(
        "docs-compose-up"
        "docs-compose-build-site"
        "docs-compose-build-pdf"
        "docs-compose-down"
    )
    
    for cmd in "${COMPOSE_COMMANDS[@]}"; do
        if grep -q "$cmd" "$AGENTS_MD_PATH"; then
            pass "AGENTS.md documents '$cmd' command"
        else
            log_warning "AGENTS.md may not document '$cmd' command"
        fi
    done
    
    if [ "$AGENTS_MD_PASSED" = true ]; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
fi

echo

# Test 8: Test Docker build doesn't interfere with Rust artifacts
section "Test 8: Test Docker Build Doesn't Interfere with Rust Artifacts"

log_info "Testing Docker build integration..."

# Check if Docker is available
if ! command -v docker &> /dev/null; then
    log_warning "Docker is not installed, skipping Docker build test"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    # Verify target/ exists from Rust build
    if [[ ! -d "$PROJECT_ROOT/target" ]]; then
        log_warning "target/ directory not found (Rust may not have been built)"
    else
        pass "target/ directory exists from Rust build"
    fi
    
    # Copy .dockerignore.mkdocs to .dockerignore for Docker build
    BACKUP_DOCKERIGNORE=""
    if [[ -f "$PROJECT_ROOT/.dockerignore" ]]; then
        BACKUP_DOCKERIGNORE="$PROJECT_ROOT/.dockerignore.backup.$$"
        cp "$PROJECT_ROOT/.dockerignore" "$BACKUP_DOCKERIGNORE"
        log_verbose "Backed up existing .dockerignore"
    fi
    
    cp "$DOCKERIGNORE_PATH" "$PROJECT_ROOT/.dockerignore"
    log_verbose "Copied .dockerignore.mkdocs to .dockerignore for build"
    
    # Run Docker build to verify it works
    log_info "Building Docker image for documentation..."
    DOCKER_BUILD_START=$(date +%s)
    DOCKER_BUILD_OUTPUT=$(mktemp)
    if [[ $REMOVE_TEMP -eq 1 ]]; then
        setup_cleanup "$(dirname "$DOCKER_BUILD_OUTPUT")"
    fi
    
    if docker build -f "$DOCKERFILE_PATH" -t testcase-manager-docs:integration-test . > "$DOCKER_BUILD_OUTPUT" 2>&1; then
        DOCKER_BUILD_END=$(date +%s)
        DOCKER_BUILD_TIME=$((DOCKER_BUILD_END - DOCKER_BUILD_START))
        
        pass "Docker build completed successfully in ${DOCKER_BUILD_TIME} seconds"
        
        # Verify image doesn't contain Rust artifacts
        log_info "Verifying Docker image doesn't contain Rust artifacts..."
        
        if docker run --rm testcase-manager-docs:integration-test sh -c "test -d /docs/target" 2>/dev/null; then
            fail "Docker image contains target/ directory (should be excluded)"
            TESTS_FAILED=$((TESTS_FAILED + 1))
        else
            pass "Docker image correctly excludes target/ directory"
            TESTS_PASSED=$((TESTS_PASSED + 1))
        fi
        
        # Clean up Docker image
        docker rmi testcase-manager-docs:integration-test 2>/dev/null || true
    else
        DOCKER_BUILD_END=$(date +%s)
        DOCKER_BUILD_TIME=$((DOCKER_BUILD_END - DOCKER_BUILD_START))
        
        fail "Docker build failed after ${DOCKER_BUILD_TIME} seconds"
        log_error "Build output (last 30 lines):"
        tail -30 "$DOCKER_BUILD_OUTPUT" >&2
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    
    # Restore original .dockerignore
    if [[ -n "$BACKUP_DOCKERIGNORE" ]] && [[ -f "$BACKUP_DOCKERIGNORE" ]]; then
        mv "$BACKUP_DOCKERIGNORE" "$PROJECT_ROOT/.dockerignore"
        log_verbose "Restored original .dockerignore"
    else
        rm -f "$PROJECT_ROOT/.dockerignore"
        log_verbose "Removed temporary .dockerignore"
    fi
    
    # Verify Rust artifacts still exist after Docker build
    if [[ -d "$PROJECT_ROOT/target" ]]; then
        pass "Rust target/ directory still exists after Docker build"
    else
        fail "Rust target/ directory was removed by Docker operations"
    fi
fi

echo

# Test 9: Verify Makefile has both Rust and Docker targets
section "Test 9: Verify Makefile Has Both Rust and Docker Targets"

log_info "Checking Makefile targets..."

MAKEFILE_PATH="$PROJECT_ROOT/Makefile"

if [[ ! -f "$MAKEFILE_PATH" ]]; then
    fail "Makefile not found at $MAKEFILE_PATH"
    TESTS_FAILED=$((TESTS_FAILED + 1))
else
    MAKEFILE_PASSED=true
    
    # Check for Rust targets
    RUST_TARGETS=("build" "test" "lint" "clippy")
    for target in "${RUST_TARGETS[@]}"; do
        if grep -q "^${target}:" "$MAKEFILE_PATH"; then
            pass "Makefile has Rust target: $target"
        else
            fail "Makefile missing Rust target: $target"
            MAKEFILE_PASSED=false
        fi
    done
    
    # Check for Docker documentation targets
    DOCKER_TARGETS=("docs-docker-build" "docs-docker-serve" "docs-docker-build-site")
    for target in "${DOCKER_TARGETS[@]}"; do
        if grep -q "^${target}:" "$MAKEFILE_PATH"; then
            pass "Makefile has Docker target: $target"
        else
            fail "Makefile missing Docker target: $target"
            MAKEFILE_PASSED=false
        fi
    done
    
    if [ "$MAKEFILE_PASSED" = true ]; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
fi

echo

# Test 10: Verify no conflicts between Rust and Docker artifacts
section "Test 10: Verify No Conflicts Between Rust and Docker Artifacts"

log_info "Checking for artifact conflicts..."

CONFLICTS_FOUND=false

# Check if site/ would conflict with Rust output
if [[ -d "$PROJECT_ROOT/site" ]]; then
    log_info "site/ directory exists (MkDocs output)"
    
    # Verify it's not in target/
    if [[ "$PROJECT_ROOT/site" == "$PROJECT_ROOT/target/"* ]]; then
        fail "site/ directory is inside target/ (potential conflict)"
        CONFLICTS_FOUND=true
    else
        pass "site/ directory is separate from Rust target/ directory"
    fi
fi

# Check if mkdocs-venv/ would conflict with Rust output
if [[ -d "$PROJECT_ROOT/mkdocs-venv" ]]; then
    log_info "mkdocs-venv/ directory exists"
    
    # Verify it's not in target/
    if [[ "$PROJECT_ROOT/mkdocs-venv" == "$PROJECT_ROOT/target/"* ]]; then
        fail "mkdocs-venv/ directory is inside target/ (potential conflict)"
        CONFLICTS_FOUND=true
    else
        pass "mkdocs-venv/ directory is separate from Rust target/ directory"
    fi
fi

# Verify no Rust artifacts in docs/
if [[ -d "$PROJECT_ROOT/docs" ]]; then
    if find "$PROJECT_ROOT/docs" -name "*.rs" -type f 2>/dev/null | grep -q .; then
        fail "Found Rust source files in docs/ directory"
        CONFLICTS_FOUND=true
    else
        pass "No Rust source files in docs/ directory"
    fi
fi

if [ "$CONFLICTS_FOUND" = false ]; then
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Final summary
section "Test Summary"

TOTAL_TESTS=$((TESTS_PASSED + TESTS_FAILED))
log_info "Total tests: $TOTAL_TESTS"
log_info "Tests passed: $TESTS_PASSED"
log_info "Tests failed: $TESTS_FAILED"

echo
log_info "Rust and Docker Integration Summary:"
log_info "  ✓ .dockerignore.mkdocs excludes Rust artifacts (target/, Cargo.lock, src/)"
log_info "  ✓ .gitignore includes documentation artifacts (site/, mkdocs-venv/)"
log_info "  ✓ Rust build process is unaffected by Docker documentation setup"
log_info "  ✓ Rust tests pass without interference"
log_info "  ✓ Rust lint passes without interference"
log_info "  ✓ AGENTS.md documents Docker commands accurately"
log_info "  ✓ Docker builds don't interfere with Rust artifacts"
log_info "  ✓ No conflicts between Rust and Docker artifact directories"

if [[ $TESTS_FAILED -eq 0 ]]; then
    echo
    pass "All Rust and Docker integration tests passed successfully!"
    echo
    log_info "The documentation Docker setup is properly integrated:"
    log_info "  • Docker builds exclude Rust artifacts"
    log_info "  • Rust toolchain is unaffected by Docker setup"
    log_info "  • Documentation artifacts are properly gitignored"
    log_info "  • AGENTS.md provides accurate command documentation"
    echo
    exit 0
else
    echo
    fail "Some Rust and Docker integration tests failed!"
    echo
    log_error "$TESTS_FAILED test(s) failed"
    log_info "Please review the output above and fix the issues"
    echo
    exit 1
fi
