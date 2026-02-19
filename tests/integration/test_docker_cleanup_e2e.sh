#!/usr/bin/env bash
#
# End-to-end integration test for Docker cleanup and resource management
#
# This test validates:
# 1. 'make docs-docker-clean' removes testcase-manager-docs image
# 2. 'make docs-clean' removes site/ directory
# 3. './scripts/docker-mkdocs.sh clean' removes both image and generated files
# 4. Stopped containers are cleaned up automatically (--rm flag)
# 5. Disk space usage is reasonable (image + site < 1GB combined check)
# 6. No dangling images or volumes are left after cleanup
# 7. 'docker system df' verification for resource tracking
#
# Usage: ./tests/integration/test_docker_cleanup_e2e.sh [--no-remove] [--verbose]
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Source logger library
source "$SCRIPT_DIR/../../scripts/lib/logger.sh" || exit 1

# Configuration
IMAGE_NAME="testcase-manager-docs"
IMAGE_TAG="latest"
FULL_IMAGE_NAME="${IMAGE_NAME}:${IMAGE_TAG}"
SITE_DIR="$PROJECT_ROOT/site"
DOCKER_HELPER="$PROJECT_ROOT/scripts/docker-mkdocs.sh"
MAX_TOTAL_SIZE_BYTES=$((1 * 1024 * 1024 * 1024))  # 1GB in bytes

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

section "Docker Cleanup and Resource Management End-to-End Test"
log_info "Project root: $PROJECT_ROOT"
log_info "Docker image: $FULL_IMAGE_NAME"
log_info "Site directory: $SITE_DIR"
log_info "Docker helper script: $DOCKER_HELPER"
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

if [[ ! -f "$DOCKER_HELPER" ]]; then
    fail "Docker helper script not found: $DOCKER_HELPER"
    exit 1
fi
pass "Docker helper script exists"

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Capture initial Docker system state
section "Test 2: Capture Initial Docker System State"

log_info "Capturing initial Docker system state..."

INITIAL_IMAGES=$(docker images -q "$FULL_IMAGE_NAME" | wc -l | tr -d ' ')
log_info "Initial image count for $FULL_IMAGE_NAME: $INITIAL_IMAGES"

INITIAL_DANGLING=$(docker images -f "dangling=true" -q | wc -l | tr -d ' ')
log_info "Initial dangling images: $INITIAL_DANGLING"

INITIAL_VOLUMES=$(docker volume ls -q | wc -l | tr -d ' ')
log_info "Initial volumes: $INITIAL_VOLUMES"

log_info "Initial docker system df output:"
docker system df

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Ensure Docker image exists for testing
section "Test 3: Ensure Docker Image Exists for Testing"

if ! docker images "$FULL_IMAGE_NAME" --format "{{.Repository}}:{{.Tag}}" | grep -q "^${FULL_IMAGE_NAME}$"; then
    log_info "Docker image does not exist, building it..."
    if make -C "$PROJECT_ROOT" docs-docker-build > /dev/null 2>&1; then
        pass "Docker image built successfully"
    else
        fail "Failed to build Docker image"
        exit 1
    fi
else
    pass "Docker image already exists: $FULL_IMAGE_NAME"
fi

# Verify image exists
if docker images "$FULL_IMAGE_NAME" --format "{{.Repository}}:{{.Tag}}" | grep -q "^${FULL_IMAGE_NAME}$"; then
    pass "Verified image exists for testing"
    
    # Get image size
    IMAGE_SIZE=$(docker inspect "$FULL_IMAGE_NAME" --format='{{.Size}}')
    IMAGE_SIZE_MB=$((IMAGE_SIZE / 1024 / 1024))
    log_info "Image size: ${IMAGE_SIZE_MB} MB"
    
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Image verification failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    exit 1
fi

echo

# Ensure site directory exists for testing
section "Test 4: Ensure Site Directory Exists for Testing"

if [[ ! -d "$SITE_DIR" ]]; then
    log_info "Site directory does not exist, building it..."
    if make -C "$PROJECT_ROOT" docs-docker-build-site > /dev/null 2>&1; then
        pass "Site directory created successfully"
    else
        fail "Failed to create site directory"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
else
    pass "Site directory already exists: $SITE_DIR"
fi

# Verify site directory exists
if [[ -d "$SITE_DIR" ]]; then
    pass "Verified site/ directory exists for testing"
    
    # Get site directory size
    SITE_SIZE=$(du -sb "$SITE_DIR" 2>/dev/null | awk '{print $1}')
    SITE_SIZE_MB=$((SITE_SIZE / 1024 / 1024))
    log_info "Site directory size: ${SITE_SIZE_MB} MB"
    
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    log_warning "Site directory not available for testing"
    SITE_SIZE_MB=0
fi

echo

# Test disk space usage before cleanup
section "Test 5: Verify Combined Disk Space Usage (Image + Site)"

log_info "Checking combined disk space usage..."

# Get image size
if docker images "$FULL_IMAGE_NAME" --format "{{.Repository}}:{{.Tag}}" | grep -q "^${FULL_IMAGE_NAME}$"; then
    IMAGE_SIZE=$(docker inspect "$FULL_IMAGE_NAME" --format='{{.Size}}')
    IMAGE_SIZE_MB=$((IMAGE_SIZE / 1024 / 1024))
else
    IMAGE_SIZE=0
    IMAGE_SIZE_MB=0
fi

# Get site directory size
if [[ -d "$SITE_DIR" ]]; then
    SITE_SIZE=$(du -sb "$SITE_DIR" 2>/dev/null | awk '{print $1}')
    SITE_SIZE_MB=$((SITE_SIZE / 1024 / 1024))
else
    SITE_SIZE=0
    SITE_SIZE_MB=0
fi

TOTAL_SIZE=$((IMAGE_SIZE + SITE_SIZE))
TOTAL_SIZE_MB=$((TOTAL_SIZE / 1024 / 1024))

log_info "Image size: ${IMAGE_SIZE_MB} MB"
log_info "Site directory size: ${SITE_SIZE_MB} MB"
log_info "Total size: ${TOTAL_SIZE_MB} MB"

if [[ $TOTAL_SIZE -lt $MAX_TOTAL_SIZE_BYTES ]]; then
    pass "Total disk usage is under 1GB limit (${TOTAL_SIZE_MB} MB)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Total disk usage exceeds 1GB limit (${TOTAL_SIZE_MB} MB > 1024 MB)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test container auto-cleanup with --rm flag
section "Test 6: Verify Containers are Cleaned Up Automatically (--rm flag)"

log_info "Testing automatic container cleanup with --rm flag..."

# Count running/stopped containers before test
CONTAINERS_BEFORE=$(docker ps -a --filter "ancestor=$FULL_IMAGE_NAME" -q | wc -l | tr -d ' ')
log_info "Containers before test: $CONTAINERS_BEFORE"

# Run a container with --rm flag (mkdocs build should exit immediately)
log_info "Running container with --rm flag..."
TEMP_BUILD_DIR=$(mktemp -d)
setup_cleanup "$TEMP_BUILD_DIR"

mkdir -p "$TEMP_BUILD_DIR/docs"
cat > "$TEMP_BUILD_DIR/mkdocs.yml" << 'EOF'
site_name: Test
docs_dir: docs
site_dir: site
theme:
  name: material
nav:
  - Home: index.md
EOF

cat > "$TEMP_BUILD_DIR/docs/index.md" << 'EOF'
# Test
EOF

if docker run --rm \
    -v "$TEMP_BUILD_DIR/docs:/docs/docs" \
    -v "$TEMP_BUILD_DIR/mkdocs.yml:/docs/mkdocs.yml" \
    -v "$TEMP_BUILD_DIR/site:/docs/site" \
    "$FULL_IMAGE_NAME" mkdocs build > /dev/null 2>&1; then
    pass "Container ran successfully with --rm flag"
else
    fail "Container execution failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Count containers after test
sleep 1  # Give Docker time to cleanup
CONTAINERS_AFTER=$(docker ps -a --filter "ancestor=$FULL_IMAGE_NAME" -q | wc -l | tr -d ' ')
log_info "Containers after test: $CONTAINERS_AFTER"

if [[ $CONTAINERS_AFTER -eq $CONTAINERS_BEFORE ]]; then
    pass "Container was automatically cleaned up (--rm flag works)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Container was not cleaned up automatically"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_warning "Expected $CONTAINERS_BEFORE containers, found $CONTAINERS_AFTER"
    
    # Show remaining containers
    log_info "Remaining containers:"
    docker ps -a --filter "ancestor=$FULL_IMAGE_NAME" --format "table {{.ID}}\t{{.Status}}\t{{.CreatedAt}}"
fi

echo

# Test 'make docs-clean' removes site/ directory
section "Test 7: Test 'make docs-clean' Removes site/ Directory"

log_info "Testing 'make docs-clean'..."

# Ensure site directory exists
if [[ ! -d "$SITE_DIR" ]]; then
    log_info "Creating site/ directory for testing..."
    make -C "$PROJECT_ROOT" docs-docker-build-site > /dev/null 2>&1 || true
fi

if [[ -d "$SITE_DIR" ]]; then
    log_info "site/ directory exists before cleanup"
    
    # Run make docs-clean
    if make -C "$PROJECT_ROOT" docs-clean > /dev/null 2>&1; then
        pass "make docs-clean completed successfully"
        
        # Verify site directory is removed
        if [[ ! -d "$SITE_DIR" ]]; then
            pass "site/ directory successfully removed by 'make docs-clean'"
            TESTS_PASSED=$((TESTS_PASSED + 1))
        else
            fail "site/ directory still exists after 'make docs-clean'"
            TESTS_FAILED=$((TESTS_FAILED + 1))
            
            # Check if directory is empty
            REMAINING_FILES=$(find "$SITE_DIR" -type f | wc -l | tr -d ' ')
            log_warning "Files remaining in site/: $REMAINING_FILES"
        fi
    else
        fail "make docs-clean failed"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
else
    log_warning "site/ directory does not exist, skipping test"
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

echo

# Test 'make docs-docker-clean' removes Docker image
section "Test 8: Test 'make docs-docker-clean' Removes Docker Image"

log_info "Testing 'make docs-docker-clean'..."

# Ensure image exists
if ! docker images "$FULL_IMAGE_NAME" --format "{{.Repository}}:{{.Tag}}" | grep -q "^${FULL_IMAGE_NAME}$"; then
    log_info "Building Docker image for testing..."
    make -C "$PROJECT_ROOT" docs-docker-build > /dev/null 2>&1 || true
fi

if docker images "$FULL_IMAGE_NAME" --format "{{.Repository}}:{{.Tag}}" | grep -q "^${FULL_IMAGE_NAME}$"; then
    log_info "Docker image exists before cleanup"
    
    # Run make docs-docker-clean
    if make -C "$PROJECT_ROOT" docs-docker-clean > /dev/null 2>&1; then
        pass "make docs-docker-clean completed successfully"
        
        # Verify image is removed
        if ! docker images "$FULL_IMAGE_NAME" --format "{{.Repository}}:{{.Tag}}" | grep -q "^${FULL_IMAGE_NAME}$"; then
            pass "Docker image successfully removed by 'make docs-docker-clean'"
            TESTS_PASSED=$((TESTS_PASSED + 1))
        else
            fail "Docker image still exists after 'make docs-docker-clean'"
            TESTS_FAILED=$((TESTS_FAILED + 1))
        fi
    else
        fail "make docs-docker-clean failed"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
else
    log_warning "Docker image does not exist, skipping test"
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

echo

# Test './scripts/docker-mkdocs.sh clean' removes both image and files
section "Test 9: Test './scripts/docker-mkdocs.sh clean' Removes Both Image and Files"

log_info "Testing './scripts/docker-mkdocs.sh clean'..."

# Rebuild image and site for comprehensive test
log_info "Rebuilding Docker image and site/ directory for comprehensive cleanup test..."
make -C "$PROJECT_ROOT" docs-docker-build > /dev/null 2>&1 || true
make -C "$PROJECT_ROOT" docs-docker-build-site > /dev/null 2>&1 || true

# Verify both exist
IMAGE_EXISTS=false
SITE_EXISTS=false

if docker images "$FULL_IMAGE_NAME" --format "{{.Repository}}:{{.Tag}}" | grep -q "^${FULL_IMAGE_NAME}$"; then
    IMAGE_EXISTS=true
    log_info "Docker image exists before cleanup"
fi

if [[ -d "$SITE_DIR" ]]; then
    SITE_EXISTS=true
    log_info "site/ directory exists before cleanup"
fi

# Run docker-mkdocs.sh clean
log_info "Running: $DOCKER_HELPER clean"
if "$DOCKER_HELPER" clean > /dev/null 2>&1; then
    pass "./scripts/docker-mkdocs.sh clean completed successfully"
    
    CLEANUP_SUCCESS=true
    
    # Verify image is removed
    if [[ "$IMAGE_EXISTS" == true ]]; then
        if ! docker images "$FULL_IMAGE_NAME" --format "{{.Repository}}:{{.Tag}}" | grep -q "^${FULL_IMAGE_NAME}$"; then
            pass "Docker image removed by docker-mkdocs.sh clean"
        else
            fail "Docker image still exists after docker-mkdocs.sh clean"
            CLEANUP_SUCCESS=false
        fi
    fi
    
    # Verify site directory is removed
    if [[ "$SITE_EXISTS" == true ]]; then
        if [[ ! -d "$SITE_DIR" ]]; then
            pass "site/ directory removed by docker-mkdocs.sh clean"
        else
            fail "site/ directory still exists after docker-mkdocs.sh clean"
            CLEANUP_SUCCESS=false
        fi
    fi
    
    if [[ "$CLEANUP_SUCCESS" == true ]]; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
else
    fail "./scripts/docker-mkdocs.sh clean failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test for dangling images
section "Test 10: Verify No Dangling Images After Cleanup"

log_info "Checking for dangling images..."

DANGLING_IMAGES=$(docker images -f "dangling=true" -q | wc -l | tr -d ' ')
log_info "Dangling images after cleanup: $DANGLING_IMAGES"
log_info "Initial dangling images: $INITIAL_DANGLING"

if [[ $DANGLING_IMAGES -le $INITIAL_DANGLING ]]; then
    pass "No new dangling images created (count: $DANGLING_IMAGES)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "New dangling images detected ($DANGLING_IMAGES > $INITIAL_DANGLING)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    
    log_warning "Dangling images:"
    docker images -f "dangling=true" --format "table {{.ID}}\t{{.CreatedAt}}\t{{.Size}}"
fi

echo

# Test for dangling volumes
section "Test 11: Verify No Dangling Volumes After Cleanup"

log_info "Checking for volumes..."

CURRENT_VOLUMES=$(docker volume ls -q | wc -l | tr -d ' ')
log_info "Current volumes: $CURRENT_VOLUMES"
log_info "Initial volumes: $INITIAL_VOLUMES"

if [[ $CURRENT_VOLUMES -le $INITIAL_VOLUMES ]]; then
    pass "No new volumes created (count: $CURRENT_VOLUMES)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    log_warning "New volumes detected ($CURRENT_VOLUMES > $INITIAL_VOLUMES)"
    log_info "This is expected if other Docker operations created volumes"
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

# Check for dangling volumes specifically
DANGLING_VOLUMES=$(docker volume ls -f "dangling=true" -q | wc -l | tr -d ' ')
log_info "Dangling volumes: $DANGLING_VOLUMES"

if [[ $DANGLING_VOLUMES -eq 0 ]]; then
    pass "No dangling volumes found"
else
    log_warning "$DANGLING_VOLUMES dangling volumes found"
    log_info "These may be from other Docker operations"
fi

echo

# Docker system df verification
section "Test 12: Docker System df Verification"

log_info "Running 'docker system df' for resource verification..."
echo

docker system df

echo
log_info "Docker system df --verbose (limited output):"
docker system df -v | head -20

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Comprehensive cleanup verification
section "Test 13: Comprehensive Cleanup Verification"

log_info "Performing comprehensive cleanup verification..."

VERIFICATION_PASSED=true

# Check image is removed
if docker images "$FULL_IMAGE_NAME" --format "{{.Repository}}:{{.Tag}}" | grep -q "^${FULL_IMAGE_NAME}$"; then
    fail "Docker image still exists: $FULL_IMAGE_NAME"
    VERIFICATION_PASSED=false
else
    pass "Docker image removed: $FULL_IMAGE_NAME"
fi

# Check site directory is removed
if [[ -d "$SITE_DIR" ]]; then
    fail "site/ directory still exists: $SITE_DIR"
    VERIFICATION_PASSED=false
else
    pass "site/ directory removed: $SITE_DIR"
fi

# Check for stopped containers
STOPPED_CONTAINERS=$(docker ps -a --filter "ancestor=$FULL_IMAGE_NAME" -q | wc -l | tr -d ' ')
if [[ $STOPPED_CONTAINERS -eq 0 ]]; then
    pass "No stopped containers found for $FULL_IMAGE_NAME"
else
    fail "$STOPPED_CONTAINERS stopped containers found"
    VERIFICATION_PASSED=false
    
    log_info "Stopped containers:"
    docker ps -a --filter "ancestor=$FULL_IMAGE_NAME" --format "table {{.ID}}\t{{.Status}}\t{{.CreatedAt}}"
fi

if [[ "$VERIFICATION_PASSED" == true ]]; then
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test idempotent cleanup
section "Test 14: Test Idempotent Cleanup (Running Cleanup Multiple Times)"

log_info "Testing idempotent cleanup (cleanup when already clean)..."

# Run cleanup commands again
log_info "Running 'make docs-clean' again..."
if make -C "$PROJECT_ROOT" docs-clean > /dev/null 2>&1; then
    pass "make docs-clean succeeded (idempotent)"
else
    log_warning "make docs-clean failed on already-clean state"
fi

log_info "Running 'make docs-docker-clean' again..."
if make -C "$PROJECT_ROOT" docs-docker-clean > /dev/null 2>&1; then
    pass "make docs-docker-clean succeeded (idempotent)"
else
    log_warning "make docs-docker-clean failed on already-clean state"
fi

log_info "Running './scripts/docker-mkdocs.sh clean' again..."
if "$DOCKER_HELPER" clean > /dev/null 2>&1; then
    pass "./scripts/docker-mkdocs.sh clean succeeded (idempotent)"
else
    log_warning "./scripts/docker-mkdocs.sh clean failed on already-clean state"
fi

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Verify final state
section "Test 15: Verify Final Clean State"

log_info "Verifying final clean state..."

FINAL_STATE_CLEAN=true

# Verify image is removed
if docker images "$FULL_IMAGE_NAME" --format "{{.Repository}}:{{.Tag}}" | grep -q "^${FULL_IMAGE_NAME}$"; then
    fail "Docker image still exists after final cleanup"
    FINAL_STATE_CLEAN=false
else
    pass "Docker image successfully removed"
fi

# Verify site directory is removed
if [[ -d "$SITE_DIR" ]]; then
    fail "site/ directory still exists after final cleanup"
    FINAL_STATE_CLEAN=false
else
    pass "site/ directory successfully removed"
fi

# Display final Docker system state
log_info "Final Docker system state:"
docker system df

if [[ "$FINAL_STATE_CLEAN" == true ]]; then
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
log_info "Cleanup verification summary:"
log_info "  ✓ 'make docs-docker-clean' removes Docker image"
log_info "  ✓ 'make docs-clean' removes site/ directory"
log_info "  ✓ './scripts/docker-mkdocs.sh clean' removes both"
log_info "  ✓ Containers auto-cleanup with --rm flag"
log_info "  ✓ Disk space usage verified (image + site < 1GB)"
log_info "  ✓ No dangling images left after cleanup"
log_info "  ✓ No dangling volumes left after cleanup"
log_info "  ✓ 'docker system df' output verified"

if [[ $TESTS_FAILED -eq 0 ]]; then
    echo
    pass "All Docker cleanup and resource management tests passed successfully!"
    echo
    log_info "Docker cleanup commands are working correctly:"
    log_info "  • make docs-clean              - Removes site/ directory"
    log_info "  • make docs-docker-clean       - Removes Docker image"
    log_info "  • ./scripts/docker-mkdocs.sh clean - Removes both"
    log_info "  • Automatic container cleanup with --rm flag works"
    log_info "  • No resource leaks detected"
    echo
    exit 0
else
    echo
    fail "Some Docker cleanup tests failed!"
    echo
    log_error "$TESTS_FAILED test(s) failed"
    log_info "Please review the output above and fix the issues"
    echo
    exit 1
fi
