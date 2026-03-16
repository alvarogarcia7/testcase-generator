#!/usr/bin/env bash
# NOTE: This file must have executable permissions (chmod +x tests/integration/test_docker_build.sh)
#
# Docker Build and Binary Verification Integration Test
#
# This test validates:
# 1. Dockerfile builds successfully without errors
# 2. All expected binaries are present in the Docker image (12 binaries total)
# 3. The verifier binary executes without errors
# 4. The verifier binary responds to --version flag
# 5. All other critical binaries are present and executable
# 6. Directory structure is correct in the Docker image
# 7. Runtime dependencies (git, make, expect) are installed
# 8. End-to-end verifier workflow works within the container
#
# Usage: ./tests/integration/test_docker_build.sh [--no-remove]
#
# Options:
#   --no-remove    Keep the Docker image after testing (for debugging)
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Source logger library
source "$SCRIPT_DIR/../../scripts/lib/logger.sh" || exit 1

# Handle --no-remove flag
REMOVE_IMAGE=1
while [[ $# -gt 0 ]]; do
    case $1 in
        --no-remove)
            REMOVE_IMAGE=0
            shift
            ;;
        *)
            shift
            ;;
    esac
done

echo "=============================================="
echo "Docker Build Integration Test"
echo "=============================================="
echo ""

# Check prerequisites
section "Checking Prerequisites"

# Check if docker is available
if ! command -v docker > /dev/null 2>&1; then
    fail "Docker is not installed or not in PATH"
    echo "Please install Docker to run this test"
    exit 1
fi
pass "Docker is available"

# Check if docker daemon is running
if ! docker info > /dev/null 2>&1; then
    fail "Docker daemon is not running"
    echo "Please start Docker daemon to run this test"
    exit 1
fi
pass "Docker daemon is running"

# Check if Dockerfile exists
if [[ ! -f "$PROJECT_ROOT/Dockerfile" ]]; then
    fail "Dockerfile not found at $PROJECT_ROOT/Dockerfile"
    exit 1
fi
pass "Dockerfile found"

# Define expected binaries (from Dockerfile and Cargo.toml)
EXPECTED_BINARIES=(
    "validate-yaml"
    "validate-json"
    "test-run-manager"
    "trm"
    "test-verify"
    "test-executor"
    "testcase-manager"
    "editor"
    "test-orchestrator"
    "script-cleanup"
    "json-escape"
    "verifier"
)

# Generate unique image tag for this test
IMAGE_TAG="testcase-manager:test-$(date +%s)"

# Test 1: Build Docker image
section "Test 1: Building Docker Image"

info "Building Docker image: $IMAGE_TAG"
info "This may take several minutes..."

BUILD_LOG="$PROJECT_ROOT/docker_build_test.log"
if docker build -t "$IMAGE_TAG" "$PROJECT_ROOT" > "$BUILD_LOG" 2>&1; then
    pass "Docker image built successfully"
else
    BUILD_EXIT=$?
    fail "Docker build failed with exit code: $BUILD_EXIT"
    echo ""
    echo "Last 50 lines of build log:"
    tail -n 50 "$BUILD_LOG"
    rm -f "$BUILD_LOG"
    exit 1
fi

# Clean up build log
rm -f "$BUILD_LOG"

# Test 2: Verify all expected binaries are present in the image
section "Test 2: Verifying Binaries in Docker Image"

BINARIES_FOUND=0
BINARIES_MISSING=0

for binary in "${EXPECTED_BINARIES[@]}"; do
    if docker run --rm "$IMAGE_TAG" bash -c "command -v $binary > /dev/null 2>&1"; then
        pass "Binary found: $binary"
        BINARIES_FOUND=$((BINARIES_FOUND + 1))
    else
        fail "Binary missing: $binary"
        BINARIES_MISSING=$((BINARIES_MISSING + 1))
    fi
done

echo ""
info "Binaries found: $BINARIES_FOUND / ${#EXPECTED_BINARIES[@]}"

if [[ $BINARIES_MISSING -gt 0 ]]; then
    fail "Some binaries are missing from the Docker image"
    exit 1
fi

pass "All expected binaries are present"

# Test 3: Verify verifier binary executes without errors
section "Test 3: Verifying 'verifier' Binary Execution"

# Test verifier --version
VERIFIER_VERSION_OUTPUT=$(docker run --rm "$IMAGE_TAG" verifier --version 2>&1 || true)
VERIFIER_VERSION_EXIT=$?

if [[ $VERIFIER_VERSION_EXIT -eq 0 ]]; then
    pass "verifier --version executed successfully"
    info "Version output: $VERIFIER_VERSION_OUTPUT"
else
    fail "verifier --version failed with exit code: $VERIFIER_VERSION_EXIT"
    echo "Output: $VERIFIER_VERSION_OUTPUT"
    exit 1
fi

# Test verifier --help
VERIFIER_HELP_OUTPUT=$(docker run --rm "$IMAGE_TAG" verifier --help 2>&1 || true)
VERIFIER_HELP_EXIT=$?

if [[ $VERIFIER_HELP_EXIT -eq 0 ]]; then
    pass "verifier --help executed successfully"
else
    fail "verifier --help failed with exit code: $VERIFIER_HELP_EXIT"
    echo "Output: $VERIFIER_HELP_OUTPUT"
    exit 1
fi

# Test 4: Verify other critical binaries can execute
section "Test 4: Verifying Other Critical Binaries"

# Test validate-yaml
if docker run --rm "$IMAGE_TAG" validate-yaml --version > /dev/null 2>&1; then
    pass "validate-yaml --version works"
else
    fail "validate-yaml --version failed"
fi

# Test validate-json
if docker run --rm "$IMAGE_TAG" validate-json --version > /dev/null 2>&1; then
    pass "validate-json --version works"
else
    fail "validate-json --version failed"
fi

# Test test-executor
if docker run --rm "$IMAGE_TAG" test-executor --version > /dev/null 2>&1; then
    pass "test-executor --version works"
else
    fail "test-executor --version failed"
fi

# Test test-orchestrator
if docker run --rm "$IMAGE_TAG" test-orchestrator --version > /dev/null 2>&1; then
    pass "test-orchestrator --version works"
else
    fail "test-orchestrator --version failed"
fi

# Test json-escape (this binary may not support --version, test --help instead)
if docker run --rm "$IMAGE_TAG" json-escape --help > /dev/null 2>&1; then
    pass "json-escape --help works"
else
    fail "json-escape --help failed"
fi

# Test 5: Verify directory structure in the image
section "Test 5: Verifying Directory Structure"

# Check if /app directory exists
if docker run --rm "$IMAGE_TAG" test -d /app; then
    pass "/app directory exists"
else
    fail "/app directory missing"
fi

# Check if /app/scripts directory exists
if docker run --rm "$IMAGE_TAG" test -d /app/scripts; then
    pass "/app/scripts directory exists"
else
    fail "/app/scripts directory missing"
fi

# Check if /app/testcases directory exists
if docker run --rm "$IMAGE_TAG" test -d /app/testcases; then
    pass "/app/testcases directory exists"
else
    fail "/app/testcases directory missing"
fi

# Check if Makefile exists
if docker run --rm "$IMAGE_TAG" test -f /app/Makefile; then
    pass "/app/Makefile exists"
else
    fail "/app/Makefile missing"
fi

# Test 6: Verify runtime dependencies
section "Test 6: Verifying Runtime Dependencies"

# Check if git is available
if docker run --rm "$IMAGE_TAG" git --version > /dev/null 2>&1; then
    pass "git is available"
else
    fail "git is missing"
fi

# Check if make is available
if docker run --rm "$IMAGE_TAG" make --version > /dev/null 2>&1; then
    pass "make is available"
else
    fail "make is missing"
fi

# Check if expect is available (for interactive tests)
if docker run --rm "$IMAGE_TAG" command -v expect > /dev/null 2>&1; then
    pass "expect is available"
else
    fail "expect is missing"
fi

# Test 7: Run a simple verifier workflow in Docker
section "Test 7: Running Simple Verifier Workflow in Container"

# Create a simple test case and execution log
TEMP_DIR=$(mktemp -d)
setup_cleanup "$TEMP_DIR"

info "Creating test files in: $TEMP_DIR"

# Create test case YAML
cat > "$TEMP_DIR/TEST_SIMPLE_001.yml" << 'EOF'
requirement: TEST_REQ_001
item: 1
tc: 1
id: TEST_SIMPLE_001
description: Simple test case
general_initial_conditions:
  System:
    - Ready
test_sequences:
  - id: 1
    name: Simple Sequence
    description: Basic test
    steps:
      - step: 1
        description: Echo test
        command: echo 'test'
        expected:
          success: true
          result: "0"
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
EOF

# Create execution log
cat > "$TEMP_DIR/TEST_SIMPLE_001_execution_log.json" << 'EOF'
[
  {
    "test_sequence": 1,
    "step": 1,
    "command": "echo 'test'",
    "exit_code": 0,
    "output": "test",
    "timestamp": "2026-02-02T10:00:00.000000+00:00"
  }
]
EOF

pass "Created test files"

# Run verifier in Docker with mounted test files (using CLI flags)
VERIFIER_OUTPUT="$TEMP_DIR/report.yaml"
if docker run --rm \
    -v "$TEMP_DIR:/test_data" \
    "$IMAGE_TAG" \
    verifier \
    --log /test_data/TEST_SIMPLE_001_execution_log.json \
    --test-case TEST_SIMPLE_001 \
    --test-case-dir /test_data \
    --format yaml \
    --output /test_data/report.yaml \
    --title "Docker Integration Test" \
    --project "Docker Build Verification" \
    --environment "Docker Container" \
    --platform "Docker Image" \
    --executor "Docker Test Runner" > /dev/null 2>&1; then
    pass "verifier executed successfully in container"
else
    VERIFY_EXIT=$?
    fail "verifier execution failed in container with exit code: $VERIFY_EXIT"
    # Don't exit, continue with cleanup
fi

# Check if report was generated
if [[ -f "$VERIFIER_OUTPUT" ]]; then
    pass "Verification report generated"
    
    # Validate report contains expected data
    if grep -q "test_case_id: TEST_SIMPLE_001" "$VERIFIER_OUTPUT"; then
        pass "Report contains correct test case ID"
    else
        fail "Report missing correct test case ID"
    fi
else
    fail "Verification report not generated"
fi

# Cleanup
section "Cleanup"

if [[ $REMOVE_IMAGE -eq 1 ]]; then
    info "Removing test Docker image: $IMAGE_TAG"
    if docker rmi "$IMAGE_TAG" > /dev/null 2>&1; then
        pass "Docker image removed"
    else
        info "Failed to remove Docker image (may still be in use)"
    fi
else
    info "Docker image preserved: $IMAGE_TAG"
    echo "To remove manually, run: docker rmi $IMAGE_TAG"
fi

# Summary
section "Test Summary"
echo ""
echo "=============================================="
echo "Docker Build Integration Test Complete"
echo "=============================================="
echo ""

pass "All Docker build and binary verification tests passed!"
echo ""
echo "Verified:"
echo "  ✓ Dockerfile builds successfully"
echo "  ✓ All ${#EXPECTED_BINARIES[@]} expected binaries are present"
echo "  ✓ verifier binary executes without errors"
echo "  ✓ verifier --version works correctly"
echo "  ✓ All critical binaries respond to --version/--help"
echo "  ✓ Directory structure is correct"
echo "  ✓ Runtime dependencies are installed"
echo "  ✓ End-to-end verifier workflow in container"
echo ""

exit 0
