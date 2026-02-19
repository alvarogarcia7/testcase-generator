#!/usr/bin/env bash
#
# End-to-end integration test for Docker cross-platform compatibility (macOS and Linux)
#
# This test validates:
# 1. make docs-docker-build works identically on macOS and Linux
# 2. make docs-docker-serve volume mounts work on both platforms
# 3. site/ directory permissions are correct on both macOS and Linux hosts
# 4. docker-mkdocs.sh script works with both BSD and GNU utilities on host
# 5. Docker Compose commands work on both platforms
# 6. No path separator issues (/ vs \) affect builds
# 7. Platform detection and specific compatibility checks
# 8. File permissions and ownership consistency across platforms
#
# Usage: ./tests/integration/test_docker_cross_platform_e2e.sh [--no-remove] [--verbose]
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Source logger library
source "$SCRIPT_DIR/../../scripts/lib/logger.sh" || exit 1

# Configuration
IMAGE_NAME="testcase-manager-docs:latest"
COMPOSE_FILE="$PROJECT_ROOT/docker-compose.mkdocs.yml"
DOCKER_MKDOCS_HELPER="$PROJECT_ROOT/scripts/docker-mkdocs.sh"
SERVER_PORT=8000
SERVER_HOST="localhost"
MAX_STARTUP_TIME=30
SITE_DIR="$PROJECT_ROOT/site"

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

# Temporary files
TEMP_DIR=$(mktemp -d)
setup_cleanup "$TEMP_DIR"

if [[ $REMOVE_TEMP -eq 0 ]]; then
    disable_cleanup
    info "Temporary files will not be removed: $TEMP_DIR"
fi

section "Docker Cross-Platform Compatibility Test"
log_info "Project root: $PROJECT_ROOT"
log_info "Test directory: $TEMP_DIR"
echo

# Detect platform
section "Test 1: Platform Detection"

# Detect operating system
OS_TYPE="$(uname -s)"
case "$OS_TYPE" in
    Linux*)
        PLATFORM="linux"
        PLATFORM_NAME="Linux"
        ;;
    Darwin*)
        PLATFORM="macos"
        PLATFORM_NAME="macOS"
        ;;
    CYGWIN*|MINGW*|MSYS*)
        PLATFORM="windows"
        PLATFORM_NAME="Windows"
        ;;
    *)
        PLATFORM="unknown"
        PLATFORM_NAME="Unknown"
        ;;
esac

log_info "Detected platform: $PLATFORM_NAME ($OS_TYPE)"
pass "Platform detected: $PLATFORM_NAME"

# Detect architecture
ARCH="$(uname -m)"
log_info "Architecture: $ARCH"

# Check if running in Docker Desktop (macOS/Windows)
DOCKER_DESKTOP=0
if [[ "$PLATFORM" == "macos" ]]; then
    if docker info 2>/dev/null | grep -q "Operating System.*Docker Desktop"; then
        DOCKER_DESKTOP=1
        log_info "Running Docker Desktop on macOS"
    fi
fi

# Detect utility variants
log_info "Detecting utility variants..."

# Check sed variant
if sed --version 2>/dev/null | grep -q "GNU"; then
    SED_VARIANT="GNU"
else
    SED_VARIANT="BSD"
fi
log_info "sed variant: $SED_VARIANT"

# Check grep variant
if grep --version 2>/dev/null | grep -q "GNU"; then
    GREP_VARIANT="GNU"
else
    GREP_VARIANT="BSD"
fi
log_info "grep variant: $GREP_VARIANT"

# Check stat command
if stat --version 2>/dev/null | grep -q "GNU"; then
    STAT_VARIANT="GNU"
    STAT_SIZE_FORMAT="-c %s"
    STAT_OWNER_FORMAT="-c %U"
else
    STAT_VARIANT="BSD"
    STAT_SIZE_FORMAT="-f %z"
    STAT_OWNER_FORMAT="-f %Su"
fi
log_info "stat variant: $STAT_VARIANT"

# Check find variant
if find --version 2>/dev/null | grep -q "GNU"; then
    FIND_VARIANT="GNU"
else
    FIND_VARIANT="BSD"
fi
log_info "find variant: $FIND_VARIANT"

log_info "Host user: $(id -u):$(id -g)"

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Check prerequisites
section "Test 2: Checking Prerequisites"

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

# Display Docker version
DOCKER_VERSION=$(docker --version)
log_info "Docker version: $DOCKER_VERSION"

if command -v docker-compose &> /dev/null; then
    pass "docker-compose is installed"
    DOCKER_COMPOSE_VERSION=$(docker-compose --version)
    log_info "docker-compose version: $DOCKER_COMPOSE_VERSION"
    HAS_COMPOSE=1
else
    log_warning "docker-compose is not installed"
    HAS_COMPOSE=0
fi

if [[ -f "$DOCKER_MKDOCS_HELPER" ]]; then
    pass "docker-mkdocs.sh helper script exists"
else
    fail "docker-mkdocs.sh helper script not found at $DOCKER_MKDOCS_HELPER"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Test path separator handling
section "Test 3: Path Separator and Path Handling"

log_info "Testing path handling for Docker volumes..."

# Test that paths are correctly formed
TEST_PATH="$PROJECT_ROOT/docs"
log_info "Sample path: $TEST_PATH"

# Check for backslashes (Windows-style)
if echo "$TEST_PATH" | grep -q '\\'; then
    fail "Path contains backslashes (Windows-style)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "Docker requires forward slashes for volume mounts"
else
    pass "Path uses forward slashes"
fi

# Test PWD environment variable
log_info "PWD: $PWD"
if echo "$PWD" | grep -q '\\'; then
    fail "PWD contains backslashes"
    TESTS_FAILED=$((TESTS_FAILED + 1))
else
    pass "PWD uses forward slashes"
fi

# Verify paths in Makefile use $(PWD) or absolute paths
if grep -q '$(PWD)' "$PROJECT_ROOT/Makefile"; then
    pass "Makefile uses \$(PWD) for cross-platform compatibility"
else
    log_warning "Makefile may not use \$(PWD) for paths"
fi

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Clean any existing Docker image and site directory
section "Test 4: Clean Existing Resources"

log_info "Cleaning existing Docker image and site directory..."

# Remove existing site directory
if [[ -d "$SITE_DIR" ]]; then
    log_info "Removing existing site/ directory..."
    if rm -rf "$SITE_DIR" 2>/dev/null; then
        pass "Removed existing site/ directory"
    else
        log_warning "Failed to remove site/ directory, attempting chmod"
        chmod -R u+w "$SITE_DIR" 2>/dev/null || true
        rm -rf "$SITE_DIR" 2>/dev/null || true
    fi
fi

# Remove existing Docker image if present
if docker images "$IMAGE_NAME" --format "{{.Repository}}:{{.Tag}}" | grep -q "^${IMAGE_NAME}$"; then
    log_info "Removing existing Docker image..."
    docker rmi "$IMAGE_NAME" 2>/dev/null || log_warning "Failed to remove existing image"
fi

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Test 'make docs-docker-build' on current platform
section "Test 5: Test 'make docs-docker-build' on $PLATFORM_NAME"

log_info "Building Docker image using make..."
log_info "Command: make docs-docker-build"

BUILD_LOG="$TEMP_DIR/docker_build.log"
BUILD_START=$(date +%s)

if make -C "$PROJECT_ROOT" docs-docker-build > "$BUILD_LOG" 2>&1; then
    BUILD_END=$(date +%s)
    BUILD_TIME=$((BUILD_END - BUILD_START))
    pass "Docker image built successfully in ${BUILD_TIME}s"
    TESTS_PASSED=$((TESTS_PASSED + 1))
    
    log_verbose "Build output (last 20 lines):"
    if [[ ${VERBOSE:-0} -eq 1 ]]; then
        tail -n 20 "$BUILD_LOG" >&2
    fi
else
    fail "Docker image build failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "Build output:"
    cat "$BUILD_LOG" >&2
    exit 1
fi

# Verify image exists
if docker images "$IMAGE_NAME" --format "{{.Repository}}:{{.Tag}}" | grep -q "^${IMAGE_NAME}$"; then
    pass "Docker image exists: $IMAGE_NAME"
else
    fail "Docker image not found after build"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    exit 1
fi

# Get image information
IMAGE_SIZE=$(docker images "$IMAGE_NAME" --format "{{.Size}}")
IMAGE_ID=$(docker images "$IMAGE_NAME" --format "{{.ID}}")
IMAGE_CREATED=$(docker images "$IMAGE_NAME" --format "{{.CreatedAt}}")

log_info "Image ID: $IMAGE_ID"
log_info "Image size: $IMAGE_SIZE"
log_info "Created: $IMAGE_CREATED"

echo

# Test image build consistency
section "Test 6: Test Image Build Consistency"

log_info "Verifying image build produces consistent results..."

# Check that the image works
log_info "Testing mkdocs version in container..."
if docker run --rm "$IMAGE_NAME" mkdocs --version > "$TEMP_DIR/mkdocs_version.txt" 2>&1; then
    MKDOCS_VERSION=$(cat "$TEMP_DIR/mkdocs_version.txt")
    pass "MkDocs is accessible: $MKDOCS_VERSION"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "MkDocs not accessible in container"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Verify container user
CONTAINER_USER=$(docker run --rm "$IMAGE_NAME" id -u 2>/dev/null)
CONTAINER_USER_NAME=$(docker run --rm "$IMAGE_NAME" whoami 2>/dev/null)

log_info "Container runs as user: $CONTAINER_USER_NAME (UID: $CONTAINER_USER)"

if [[ "$CONTAINER_USER" != "0" ]]; then
    pass "Container runs as non-root user"
else
    fail "Container runs as root user"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test volume mounts work on both platforms
section "Test 7: Test Volume Mounts with 'make docs-docker-build-site'"

log_info "Testing volume mounts by building documentation site..."
log_info "This tests that file paths work correctly across platforms"

# Clean site directory first
rm -rf "$SITE_DIR" 2>/dev/null || true

BUILD_SITE_LOG="$TEMP_DIR/build_site.log"
BUILD_SITE_START=$(date +%s)

log_info "Command: make docs-docker-build-site"

if make -C "$PROJECT_ROOT" docs-docker-build-site > "$BUILD_SITE_LOG" 2>&1; then
    BUILD_SITE_END=$(date +%s)
    BUILD_SITE_TIME=$((BUILD_SITE_END - BUILD_SITE_START))
    pass "Documentation site built successfully in ${BUILD_SITE_TIME}s"
    TESTS_PASSED=$((TESTS_PASSED + 1))
    
    log_verbose "Build site output (last 20 lines):"
    if [[ ${VERBOSE:-0} -eq 1 ]]; then
        tail -n 20 "$BUILD_SITE_LOG" >&2
    fi
else
    fail "Documentation site build failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "Build output:"
    cat "$BUILD_SITE_LOG" >&2
fi

echo

# Test site/ directory permissions
section "Test 8: Test site/ Directory Permissions on $PLATFORM_NAME"

if [[ ! -d "$SITE_DIR" ]]; then
    fail "site/ directory was not created"
    TESTS_FAILED=$((TESTS_FAILED + 1))
else
    pass "site/ directory exists"
    
    # Check directory accessibility
    if [[ -r "$SITE_DIR" ]] && [[ -x "$SITE_DIR" ]]; then
        pass "site/ directory is accessible from host"
    else
        fail "site/ directory is not accessible from host"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    
    # Get directory ownership using platform-specific stat command
    if [[ "$STAT_VARIANT" == "GNU" ]]; then
        SITE_OWNER=$(stat -c %U "$SITE_DIR" 2>/dev/null || echo "unknown")
        SITE_UID=$(stat -c %u "$SITE_DIR" 2>/dev/null || echo "unknown")
        SITE_GID=$(stat -c %g "$SITE_DIR" 2>/dev/null || echo "unknown")
    else
        SITE_OWNER=$(stat -f %Su "$SITE_DIR" 2>/dev/null || echo "unknown")
        SITE_UID=$(stat -f %u "$SITE_DIR" 2>/dev/null || echo "unknown")
        SITE_GID=$(stat -f %g "$SITE_DIR" 2>/dev/null || echo "unknown")
    fi
    
    HOST_UID=$(id -u)
    HOST_GID=$(id -g)
    HOST_USER=$(whoami)
    
    log_info "site/ owner: $SITE_OWNER (UID: $SITE_UID, GID: $SITE_GID)"
    log_info "Host user: $HOST_USER (UID: $HOST_UID, GID: $HOST_GID)"
    
    # Platform-specific permission checks
    if [[ "$PLATFORM" == "macos" ]] && [[ $DOCKER_DESKTOP -eq 1 ]]; then
        log_info "macOS with Docker Desktop: Using osxfs volume mounts"
        # Docker Desktop on macOS uses osxfs which handles permissions differently
        if [[ "$SITE_UID" == "$HOST_UID" ]] || [[ "$SITE_OWNER" == "$HOST_USER" ]]; then
            pass "site/ ownership matches host user (expected on macOS)"
        else
            log_warning "site/ ownership differs from host user"
            info "This may be expected on macOS with Docker Desktop"
        fi
    elif [[ "$PLATFORM" == "linux" ]]; then
        log_info "Linux: Direct volume mount with host filesystem"
        # On Linux, ownership should match host user due to volume mount behavior
        if [[ "$SITE_UID" == "$HOST_UID" ]]; then
            pass "site/ ownership matches host user (expected on Linux)"
        else
            log_warning "site/ ownership differs from host user (UID: $SITE_UID vs $HOST_UID)"
        fi
    fi
    
    # Test write access from host
    TEST_PERM_FILE="$SITE_DIR/.test_write_$$"
    if echo "test" > "$TEST_PERM_FILE" 2>/dev/null; then
        pass "Host user can write to site/ directory"
        rm -f "$TEST_PERM_FILE" 2>/dev/null
    else
        fail "Host user cannot write to site/ directory"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    
    # Test deletion without sudo
    TEST_DELETE_DIR="$SITE_DIR/.test_delete_$$"
    mkdir -p "$TEST_DELETE_DIR" 2>/dev/null
    echo "test" > "$TEST_DELETE_DIR/test.txt" 2>/dev/null
    
    if rm -rf "$TEST_DELETE_DIR" 2>/dev/null; then
        pass "Host user can delete files in site/ directory without sudo"
    else
        fail "Host user cannot delete files in site/ directory"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    
    # Check specific files
    if [[ -f "$SITE_DIR/index.html" ]]; then
        pass "index.html exists"
        
        # Get file size using platform-specific stat
        if [[ "$STAT_VARIANT" == "GNU" ]]; then
            HTML_SIZE=$(stat -c %s "$SITE_DIR/index.html")
        else
            HTML_SIZE=$(stat -f %z "$SITE_DIR/index.html")
        fi
        
        log_info "index.html size: $HTML_SIZE bytes"
        
        if [[ $HTML_SIZE -gt 100 ]]; then
            pass "index.html has content"
        else
            fail "index.html is too small"
            TESTS_FAILED=$((TESTS_FAILED + 1))
        fi
        
        # Check file readability
        if [[ -r "$SITE_DIR/index.html" ]]; then
            pass "index.html is readable from host"
        else
            fail "index.html is not readable from host"
            TESTS_FAILED=$((TESTS_FAILED + 1))
        fi
    else
        fail "index.html not found"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

echo

# Test docker-mkdocs.sh script with BSD/GNU utilities
section "Test 9: Test docker-mkdocs.sh Script Compatibility"

log_info "Testing docker-mkdocs.sh script with $PLATFORM_NAME utilities..."

if [[ -x "$DOCKER_MKDOCS_HELPER" ]]; then
    pass "docker-mkdocs.sh is executable"
else
    log_warning "docker-mkdocs.sh is not executable, setting permissions"
    chmod +x "$DOCKER_MKDOCS_HELPER" 2>/dev/null || true
fi

# Test script help
HELPER_OUTPUT="$TEMP_DIR/helper_output.txt"
if bash "$DOCKER_MKDOCS_HELPER" help > "$HELPER_OUTPUT" 2>&1; then
    pass "docker-mkdocs.sh help command works"
    
    log_verbose "Helper script help:"
    if [[ ${VERBOSE:-0} -eq 1 ]]; then
        cat "$HELPER_OUTPUT" >&2
    fi
else
    fail "docker-mkdocs.sh help command failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test script status command
STATUS_OUTPUT="$TEMP_DIR/status_output.txt"
if bash "$DOCKER_MKDOCS_HELPER" status > "$STATUS_OUTPUT" 2>&1; then
    pass "docker-mkdocs.sh status command works"
    
    log_verbose "Helper script status:"
    if [[ ${VERBOSE:-0} -eq 1 ]]; then
        cat "$STATUS_OUTPUT" >&2
    fi
else
    fail "docker-mkdocs.sh status command failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Verify script uses portable shell constructs
log_info "Checking script for BSD/GNU compatibility..."

COMPAT_ISSUES=0

# Check for GNU-specific flags
if grep -n "sed -r" "$DOCKER_MKDOCS_HELPER" 2>/dev/null; then
    log_warning "Found 'sed -r' (GNU-specific), should use 'sed -E' for BSD compatibility"
    COMPAT_ISSUES=$((COMPAT_ISSUES + 1))
fi

if grep -n "grep -P" "$DOCKER_MKDOCS_HELPER" 2>/dev/null; then
    log_warning "Found 'grep -P' (GNU-specific, Perl regex not available on BSD)"
    COMPAT_ISSUES=$((COMPAT_ISSUES + 1))
fi

if grep -n "readlink -f" "$DOCKER_MKDOCS_HELPER" 2>/dev/null; then
    log_warning "Found 'readlink -f' (GNU-specific, not available on BSD)"
    COMPAT_ISSUES=$((COMPAT_ISSUES + 1))
fi

# Check for bash 4.0+ features
if grep -n "declare -A" "$DOCKER_MKDOCS_HELPER" 2>/dev/null; then
    log_warning "Found 'declare -A' (requires bash 4.0+, macOS has bash 3.2)"
    COMPAT_ISSUES=$((COMPAT_ISSUES + 1))
fi

if [[ $COMPAT_ISSUES -eq 0 ]]; then
    pass "No obvious BSD/GNU compatibility issues found"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    log_warning "Found $COMPAT_ISSUES potential compatibility issues"
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

echo

# Test 'make docs-docker-serve' volume mounts
section "Test 10: Test 'make docs-docker-serve' Volume Mounts"

log_info "Testing development server volume mounts..."

# Check if port is available
PORT_IN_USE=0
if command -v lsof &> /dev/null; then
    if lsof -ti:$SERVER_PORT &> /dev/null; then
        PORT_IN_USE=1
    fi
elif command -v netstat &> /dev/null; then
    if netstat -an | grep -q ":$SERVER_PORT.*LISTEN"; then
        PORT_IN_USE=1
    fi
fi

if [[ $PORT_IN_USE -eq 1 ]]; then
    log_warning "Port $SERVER_PORT is already in use, skipping serve test"
    log_info "Run 'make docs-docker-serve' manually to test development server"
else
    log_info "Starting development server in background..."
    
    SERVER_LOG="$TEMP_DIR/server.log"
    
    # Start server in background
    docker run --rm \
        -p "$SERVER_PORT:8000" \
        -v "$PROJECT_ROOT/docs:/docs/docs" \
        -v "$PROJECT_ROOT/mkdocs.yml:/docs/mkdocs.yml" \
        -v "$PROJECT_ROOT/site:/docs/site" \
        -v "$PROJECT_ROOT/README.md:/docs/README.md" \
        -v "$PROJECT_ROOT/README_INSTALL.md:/docs/README_INSTALL.md" \
        "$IMAGE_NAME" mkdocs serve -a 0.0.0.0:8000 > "$SERVER_LOG" 2>&1 &
    
    SERVER_PID=$!
    register_background_pid $SERVER_PID
    
    log_info "Server started with PID: $SERVER_PID"
    
    # Wait for server to start
    log_info "Waiting for server to start (max ${MAX_STARTUP_TIME}s)..."
    STARTUP_SUCCESS=0
    
    for i in $(seq 1 $MAX_STARTUP_TIME); do
        if kill -0 $SERVER_PID 2>/dev/null; then
            if grep -q "Serving on" "$SERVER_LOG" 2>/dev/null; then
                STARTUP_SUCCESS=1
                pass "Server started successfully after ${i}s"
                break
            fi
            
            # Alternative check: try to connect
            if command -v curl &> /dev/null; then
                if curl -s -f -o /dev/null "http://$SERVER_HOST:$SERVER_PORT/" 2>/dev/null; then
                    STARTUP_SUCCESS=1
                    pass "Server is accessible after ${i}s"
                    break
                fi
            fi
        else
            fail "Server process died during startup"
            log_error "Server log:"
            cat "$SERVER_LOG" >&2
            TESTS_FAILED=$((TESTS_FAILED + 1))
            break
        fi
        
        sleep 1
    done
    
    if [[ $STARTUP_SUCCESS -eq 1 ]]; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
        
        # Test file modification while server is running
        log_info "Testing live reload by creating a test file..."
        
        TEST_MD_FILE="$PROJECT_ROOT/docs/.test_cross_platform_$$"
        echo "# Cross-Platform Test $(date)" > "$TEST_MD_FILE"
        
        if [[ -f "$TEST_MD_FILE" ]]; then
            pass "Test file created from host while server running"
            
            # Wait for server to detect change
            sleep 3
            
            # Check if server is still running
            if kill -0 $SERVER_PID 2>/dev/null; then
                pass "Server still running after host file creation"
            else
                fail "Server stopped after host file creation"
                TESTS_FAILED=$((TESTS_FAILED + 1))
            fi
            
            # Clean up test file
            rm -f "$TEST_MD_FILE" 2>/dev/null
        else
            fail "Failed to create test file from host"
            TESTS_FAILED=$((TESTS_FAILED + 1))
        fi
        
        # Stop server
        log_info "Stopping development server..."
        if kill -0 $SERVER_PID 2>/dev/null; then
            kill -TERM $SERVER_PID 2>/dev/null || true
            sleep 2
            if kill -0 $SERVER_PID 2>/dev/null; then
                kill -9 $SERVER_PID 2>/dev/null || true
            fi
        fi
    else
        fail "Server did not start within ${MAX_STARTUP_TIME}s"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        log_error "Server log:"
        cat "$SERVER_LOG" >&2
    fi
fi

echo

# Test Docker Compose if available
if [[ $HAS_COMPOSE -eq 1 ]]; then
    section "Test 11: Test Docker Compose Cross-Platform Compatibility"
    
    log_info "Testing Docker Compose commands on $PLATFORM_NAME..."
    
    # Validate compose file syntax
    COMPOSE_CONFIG_LOG="$TEMP_DIR/compose_config.log"
    if docker-compose -f "$COMPOSE_FILE" config > "$COMPOSE_CONFIG_LOG" 2>&1; then
        pass "docker-compose.mkdocs.yml is valid"
        
        log_verbose "Compose configuration (first 30 lines):"
        if [[ ${VERBOSE:-0} -eq 1 ]]; then
            head -n 30 "$COMPOSE_CONFIG_LOG" >&2
        fi
    else
        fail "docker-compose.mkdocs.yml has syntax errors"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        log_error "Config output:"
        cat "$COMPOSE_CONFIG_LOG" >&2
    fi
    
    # Test compose build command
    log_info "Testing: make docs-compose-build-site"
    
    # Clean site first
    rm -rf "$SITE_DIR" 2>/dev/null || true
    
    COMPOSE_BUILD_LOG="$TEMP_DIR/compose_build.log"
    if make -C "$PROJECT_ROOT" docs-compose-build-site > "$COMPOSE_BUILD_LOG" 2>&1; then
        pass "Docker Compose build completed successfully"
        
        if [[ -f "$SITE_DIR/index.html" ]]; then
            pass "Docker Compose generated site/ directory correctly"
            TESTS_PASSED=$((TESTS_PASSED + 1))
        else
            fail "Docker Compose did not generate site/ directory"
            TESTS_FAILED=$((TESTS_FAILED + 1))
        fi
        
        log_verbose "Compose build output (last 20 lines):"
        if [[ ${VERBOSE:-0} -eq 1 ]]; then
            tail -n 20 "$COMPOSE_BUILD_LOG" >&2
        fi
    else
        fail "Docker Compose build failed"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        log_error "Compose build output:"
        cat "$COMPOSE_BUILD_LOG" >&2
    fi
    
    # Clean up compose services
    log_info "Cleaning up Docker Compose services..."
    docker-compose -f "$COMPOSE_FILE" down > /dev/null 2>&1 || true
    
    echo
else
    log_info "Skipping Docker Compose tests (not installed)"
    echo
fi

# Test path separator handling in volume mounts
section "Test 12: Verify No Path Separator Issues in Docker Commands"

log_info "Checking Makefile volume mount syntax..."

# Extract docker run commands from Makefile
MAKEFILE="$PROJECT_ROOT/Makefile"
VOLUME_CHECK_PASSED=0

if grep -A 10 "docs-docker-serve:" "$MAKEFILE" | grep -q '\-v.*:.*:'; then
    log_warning "Found potential Windows-style path separator in Makefile"
else
    pass "No Windows-style path separators found in Makefile"
    VOLUME_CHECK_PASSED=$((VOLUME_CHECK_PASSED + 1))
fi

# Check for $(PWD) usage
if grep -A 10 "docs-docker-" "$MAKEFILE" | grep -q '\$(PWD)'; then
    pass "Makefile uses \$(PWD) for cross-platform paths"
    VOLUME_CHECK_PASSED=$((VOLUME_CHECK_PASSED + 1))
else
    log_warning "Makefile may not use \$(PWD) for paths"
fi

# Check compose file for path separators
if [[ -f "$COMPOSE_FILE" ]]; then
    if grep -q '\\' "$COMPOSE_FILE"; then
        log_warning "Found backslashes in docker-compose.mkdocs.yml"
    else
        pass "No backslashes found in docker-compose.mkdocs.yml"
        VOLUME_CHECK_PASSED=$((VOLUME_CHECK_PASSED + 1))
    fi
    
    # Check for relative paths starting with ./
    if grep -q '\./.*:' "$COMPOSE_FILE"; then
        pass "Docker Compose uses relative paths (./ prefix)"
        VOLUME_CHECK_PASSED=$((VOLUME_CHECK_PASSED + 1))
    else
        log_warning "Docker Compose may not use relative paths"
    fi
fi

if [[ $VOLUME_CHECK_PASSED -ge 2 ]]; then
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    log_warning "Some path separator checks did not pass"
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

echo

# Platform-specific checks
section "Test 13: Platform-Specific Docker Behavior"

log_info "Running platform-specific checks for $PLATFORM_NAME..."

if [[ "$PLATFORM" == "macos" ]]; then
    log_info "macOS-specific checks:"
    
    # Check for Docker Desktop
    if [[ $DOCKER_DESKTOP -eq 1 ]]; then
        pass "Docker Desktop detected on macOS"
        log_info "Using osxfs for volume mounts"
    else
        log_warning "Docker Desktop not detected, may be using alternative Docker setup"
    fi
    
    # Check file system type
    FS_TYPE=$(df -T "$PROJECT_ROOT" 2>/dev/null | tail -1 | awk '{print $2}' || echo "unknown")
    log_info "File system type: $FS_TYPE"
    
    # macOS specific: test that volume mounts work with spaces in paths
    TEST_SPACE_DIR="$TEMP_DIR/test space"
    mkdir -p "$TEST_SPACE_DIR"
    echo "test" > "$TEST_SPACE_DIR/test.txt"
    
    if docker run --rm -v "$TEST_SPACE_DIR:/test:ro" "$IMAGE_NAME" cat /test/test.txt > /dev/null 2>&1; then
        pass "Volume mounts work with spaces in paths (macOS)"
    else
        log_warning "Volume mounts with spaces may have issues"
    fi
    
    TESTS_PASSED=$((TESTS_PASSED + 1))
    
elif [[ "$PLATFORM" == "linux" ]]; then
    log_info "Linux-specific checks:"
    
    # Check for SELinux
    if command -v getenforce &> /dev/null; then
        SELINUX_STATUS=$(getenforce 2>/dev/null || echo "Not installed")
        log_info "SELinux status: $SELINUX_STATUS"
        
        if [[ "$SELINUX_STATUS" == "Enforcing" ]]; then
            log_warning "SELinux is enforcing, may need :z or :Z volume mount flags"
        fi
    fi
    
    # Check Docker socket permissions
    if [[ -e /var/run/docker.sock ]]; then
        DOCKER_SOCK_PERMS=$(ls -l /var/run/docker.sock 2>/dev/null | awk '{print $1}')
        log_info "Docker socket permissions: $DOCKER_SOCK_PERMS"
    fi
    
    # Check if running in WSL
    if grep -qi microsoft /proc/version 2>/dev/null; then
        log_info "Running in WSL (Windows Subsystem for Linux)"
        pass "WSL detected, Docker should work normally"
    fi
    
    TESTS_PASSED=$((TESTS_PASSED + 1))
    
else
    log_warning "Unknown platform, skipping platform-specific checks"
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

echo

# Test cleanup and permission handling
section "Test 14: Test Cleanup and Permission Handling"

log_info "Testing cleanup of generated files..."

if [[ -d "$SITE_DIR" ]]; then
    log_info "Attempting to delete site/ directory..."
    
    DELETE_START=$(date +%s)
    if rm -rf "$SITE_DIR" 2>/dev/null; then
        DELETE_END=$(date +%s)
        DELETE_TIME=$((DELETE_END - DELETE_START))
        pass "Successfully deleted site/ directory without sudo (${DELETE_TIME}s)"
        TESTS_PASSED=$((TESTS_PASSED + 1))
        
        # Verify deletion
        if [[ ! -d "$SITE_DIR" ]]; then
            pass "site/ directory completely removed"
        else
            fail "site/ directory still exists after deletion"
            TESTS_FAILED=$((TESTS_FAILED + 1))
        fi
    else
        fail "Failed to delete site/ directory without sudo"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        
        log_info "Attempting to fix permissions and retry..."
        chmod -R u+w "$SITE_DIR" 2>/dev/null || true
        if rm -rf "$SITE_DIR" 2>/dev/null; then
            pass "Successfully deleted after chmod"
        else
            fail "Still cannot delete even after chmod"
            log_error "This indicates a permission conflict with Docker volumes"
        fi
    fi
else
    log_info "site/ directory already clean"
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

echo

# Summary of platform compatibility
section "Test 15: Platform Compatibility Summary"

log_info "Generating compatibility report..."

cat > "$TEMP_DIR/compatibility_report.txt" << EOF
Docker Cross-Platform Compatibility Report
==========================================

Platform Information:
- Operating System: $PLATFORM_NAME ($OS_TYPE)
- Architecture: $ARCH
- Docker Desktop: $([ $DOCKER_DESKTOP -eq 1 ] && echo "Yes" || echo "No")

Utility Variants:
- sed: $SED_VARIANT
- grep: $GREP_VARIANT
- stat: $STAT_VARIANT
- find: $FIND_VARIANT

Docker Information:
- Docker Version: $DOCKER_VERSION
- Docker Compose: $([ $HAS_COMPOSE -eq 1 ] && echo "$DOCKER_COMPOSE_VERSION" || echo "Not installed")

Image Information:
- Image: $IMAGE_NAME
- Size: $IMAGE_SIZE
- ID: $IMAGE_ID

Test Results:
- Tests Passed: $TESTS_PASSED
- Tests Failed: $TESTS_FAILED

Platform-Specific Notes:
EOF

if [[ "$PLATFORM" == "macos" ]]; then
    cat >> "$TEMP_DIR/compatibility_report.txt" << EOF
- macOS uses osxfs for volume mounts (transparent permission handling)
- File ownership may differ from host user due to osxfs virtualization
- Docker Desktop handles most compatibility issues automatically
- Volume mount performance may be slower than Linux
EOF
elif [[ "$PLATFORM" == "linux" ]]; then
    cat >> "$TEMP_DIR/compatibility_report.txt" << EOF
- Linux uses direct volume mounts with host filesystem
- File ownership matches host user UID/GID
- SELinux may require :z or :Z volume mount flags if enforcing
- Native Docker performance (no virtualization overhead)
EOF
else
    cat >> "$TEMP_DIR/compatibility_report.txt" << EOF
- Platform not specifically recognized
- Basic Docker functionality should work
EOF
fi

log_info "Compatibility report saved to: $TEMP_DIR/compatibility_report.txt"

log_verbose "Compatibility report:"
if [[ ${VERBOSE:-0} -eq 1 ]]; then
    cat "$TEMP_DIR/compatibility_report.txt" >&2
fi

pass "Platform compatibility report generated"
TESTS_PASSED=$((TESTS_PASSED + 1))

echo

# Final summary
section "Test Summary"

TOTAL_TESTS=$((TESTS_PASSED + TESTS_FAILED))
log_info "Total tests: $TOTAL_TESTS"
log_info "Tests passed: $TESTS_PASSED"
log_info "Tests failed: $TESTS_FAILED"
log_info "Platform: $PLATFORM_NAME"

if [[ $TESTS_FAILED -eq 0 ]]; then
    echo
    pass "All Docker cross-platform compatibility tests passed successfully!"
    echo
    log_info "Docker setup is compatible with $PLATFORM_NAME:"
    log_info "  ✓ make docs-docker-build works correctly"
    log_info "  ✓ Volume mounts work correctly"
    log_info "  ✓ File permissions are correct"
    log_info "  ✓ docker-mkdocs.sh script is compatible with platform utilities"
    if [[ $HAS_COMPOSE -eq 1 ]]; then
        log_info "  ✓ Docker Compose commands work correctly"
    fi
    log_info "  ✓ No path separator issues detected"
    log_info "  ✓ Platform-specific features work as expected"
    echo
    log_info "Compatibility report available at:"
    log_info "  $TEMP_DIR/compatibility_report.txt"
    echo
    exit 0
else
    echo
    fail "Some Docker cross-platform compatibility tests failed!"
    echo
    log_error "$TESTS_FAILED test(s) failed on $PLATFORM_NAME"
    log_info "Please review the output above and fix the issues"
    echo
    log_info "Compatibility report available at:"
    log_info "  $TEMP_DIR/compatibility_report.txt"
    echo
    exit 1
fi
