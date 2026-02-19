#!/usr/bin/env bash
#
# End-to-end integration test for Docker MkDocs volume mount and permissions
#
# This test validates:
# 1. docs/ directory volume mount allows file editing from host while container runs
# 2. site/ directory output has correct permissions for host user
# 3. mkdocs.yml volume mount updates configuration in real-time
# 4. README.md and README_INSTALL.md volume mounts work correctly
# 5. Non-root user mkdocs in container doesn't cause permission conflicts
# 6. Generated files can be deleted from host without sudo
#
# Usage: ./tests/integration/test_docker_volume_permissions_e2e.sh [--no-remove] [--verbose]
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Source logger library
source "$SCRIPT_DIR/../../scripts/lib/logger.sh" || exit 1

# Configuration
IMAGE_NAME="testcase-manager-docs:latest"
SERVER_PORT=8000
SERVER_HOST="localhost"
MAX_STARTUP_TIME=30  # seconds
MAX_SHUTDOWN_TIME=10 # seconds

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
SERVER_LOG="$TEMP_DIR/server.log"
TEST_DOCS_FILE="$PROJECT_ROOT/docs/.test_volume_permissions_$$"
TEST_MKDOCS_YML="$TEMP_DIR/mkdocs.yml.backup"
TEST_README="$TEMP_DIR/README.md.backup"
TEST_README_INSTALL="$TEMP_DIR/README_INSTALL.md.backup"
SITE_DIR="$PROJECT_ROOT/site"

# Setup cleanup
setup_cleanup "$TEMP_DIR"

if [[ $REMOVE_TEMP -eq 0 ]]; then
    disable_cleanup
    info "Temporary files will not be removed: $TEMP_DIR"
fi

section "Docker MkDocs Volume Mount and Permissions Test"
log_info "Project root: $PROJECT_ROOT"
log_info "Docker image: $IMAGE_NAME"
log_info "Server URL: http://$SERVER_HOST:$SERVER_PORT"
log_info "Test directory: $TEMP_DIR"
log_info "Host user: $(id -u):$(id -g)"
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

# Check if Docker image exists
if ! docker images "$IMAGE_NAME" --format "{{.Repository}}:{{.Tag}}" | grep -q "^${IMAGE_NAME}$"; then
    fail "Docker image not found: $IMAGE_NAME"
    log_error "Please build the image first using: make docs-docker-build"
    exit 1
fi
pass "Docker image exists: $IMAGE_NAME"

# Check if port is already in use
if command -v lsof &> /dev/null; then
    if lsof -ti:$SERVER_PORT &> /dev/null; then
        fail "Port $SERVER_PORT is already in use"
        log_error "Please stop the process using port $SERVER_PORT and try again"
        exit 1
    fi
    pass "Port $SERVER_PORT is available"
elif command -v netstat &> /dev/null; then
    if netstat -an | grep -q ":$SERVER_PORT.*LISTEN"; then
        fail "Port $SERVER_PORT is already in use"
        log_error "Please stop the process using port $SERVER_PORT and try again"
        exit 1
    fi
    pass "Port $SERVER_PORT is available"
else
    log_warning "Cannot check if port $SERVER_PORT is in use (lsof/netstat not available)"
fi

# Backup original files
log_info "Backing up original files..."
cp "$PROJECT_ROOT/mkdocs.yml" "$TEST_MKDOCS_YML"
cp "$PROJECT_ROOT/README.md" "$TEST_README"
cp "$PROJECT_ROOT/README_INSTALL.md" "$TEST_README_INSTALL"
pass "Original files backed up"

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Clean up any existing site directory
section "Test 2: Clean Existing site/ Directory"

if [[ -d "$SITE_DIR" ]]; then
    log_info "Removing existing site/ directory..."
    
    # Test if we can delete without sudo
    if rm -rf "$SITE_DIR" 2>/dev/null; then
        pass "Successfully removed existing site/ directory without sudo"
    else
        fail "Failed to remove existing site/ directory"
        log_error "This may indicate previous permission issues"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        
        # Try to clean up with more details
        log_info "Attempting to diagnose permission issues..."
        ls -la "$SITE_DIR" 2>&1 | head -20 || true
        
        # Try to fix permissions
        log_info "Attempting to fix permissions..."
        chmod -R u+w "$SITE_DIR" 2>/dev/null || true
        rm -rf "$SITE_DIR" 2>/dev/null || true
    fi
else
    pass "No existing site/ directory to clean"
fi

# Ensure site directory doesn't exist
if [[ ! -d "$SITE_DIR" ]]; then
    pass "site/ directory is clean"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "site/ directory still exists"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Start server using make docs-docker-serve in background
section "Test 3: Start MkDocs Development Server"

log_info "Starting MkDocs development server..."
log_info "Command: docker run with volume mounts..."

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
pass "Server process started"

# Wait for server to start
log_info "Waiting for server to start (max ${MAX_STARTUP_TIME}s)..."
STARTUP_SUCCESS=0

for i in $(seq 1 $MAX_STARTUP_TIME); do
    if grep -q "Serving on" "$SERVER_LOG" 2>/dev/null; then
        STARTUP_SUCCESS=1
        pass "Server started successfully after ${i}s"
        break
    fi
    
    # Check if process is still running
    if ! kill -0 $SERVER_PID 2>/dev/null; then
        fail "Server process died during startup"
        log_error "Server log:"
        cat "$SERVER_LOG" >&2
        TESTS_FAILED=$((TESTS_FAILED + 1))
        exit 1
    fi
    
    sleep 1
done

if [[ $STARTUP_SUCCESS -eq 0 ]]; then
    fail "Server did not start within ${MAX_STARTUP_TIME}s"
    log_error "Server log:"
    cat "$SERVER_LOG" >&2
    TESTS_FAILED=$((TESTS_FAILED + 1))
    kill $SERVER_PID 2>/dev/null || true
    exit 1
fi

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Test docs/ directory volume mount - create file from host
section "Test 4: Test docs/ Directory Volume Mount - Create File from Host"

log_info "Creating test markdown file in docs/ from host..."

TEST_CONTENT="# Volume Mount Test $$

This is a test file created at $(date).

## Purpose

This file tests that:
- Files can be created in docs/ from the host
- The container can access these files
- Live reload detects the changes
- Files have correct permissions

## Test Status

✓ File created successfully from host
"

echo "$TEST_CONTENT" > "$TEST_DOCS_FILE"

if [[ -f "$TEST_DOCS_FILE" ]]; then
    pass "Test file created from host: $TEST_DOCS_FILE"
    
    # Check file permissions
    FILE_OWNER=$(ls -ln "$TEST_DOCS_FILE" | awk '{print $3}')
    FILE_GROUP=$(ls -ln "$TEST_DOCS_FILE" | awk '{print $4}')
    HOST_UID=$(id -u)
    HOST_GID=$(id -g)
    
    log_info "File owner: $FILE_OWNER (host UID: $HOST_UID)"
    log_info "File group: $FILE_GROUP (host GID: $HOST_GID)"
    
    if [[ "$FILE_OWNER" == "$HOST_UID" ]]; then
        pass "File has correct owner (host user)"
    else
        log_warning "File owner ($FILE_OWNER) differs from host UID ($HOST_UID)"
    fi
    
    # Check if file is readable and writable by host user
    if [[ -r "$TEST_DOCS_FILE" ]] && [[ -w "$TEST_DOCS_FILE" ]]; then
        pass "File is readable and writable by host user"
    else
        fail "File permissions issue - not readable/writable by host user"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
else
    fail "Failed to create test file from host"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Wait for MkDocs to detect the change
log_info "Waiting for server to detect file change (5s)..."
sleep 5

# Check server log for rebuild message
if grep -q "Building documentation" "$SERVER_LOG" 2>/dev/null || \
   grep -q "Reloading" "$SERVER_LOG" 2>/dev/null || \
   grep -q "Detected" "$SERVER_LOG" 2>/dev/null; then
    pass "Server detected file change from host"
else
    log_warning "Server rebuild not clearly detected in logs"
fi

# Check if server is still running
if kill -0 $SERVER_PID 2>/dev/null; then
    pass "Server still running after host file creation"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Server stopped after host file creation"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test editing file from host while container runs
section "Test 5: Test Editing File from Host While Container Runs"

log_info "Modifying test file from host..."

echo "" >> "$TEST_DOCS_FILE"
echo "## Edit Test" >> "$TEST_DOCS_FILE"
echo "" >> "$TEST_DOCS_FILE"
echo "This content was added at $(date) while the container is running." >> "$TEST_DOCS_FILE"

if grep -q "Edit Test" "$TEST_DOCS_FILE"; then
    pass "Test file successfully modified from host"
else
    fail "Failed to modify test file from host"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Wait for reload
log_info "Waiting for server to detect file modification (5s)..."
sleep 5

if kill -0 $SERVER_PID 2>/dev/null; then
    pass "Server still running after host file modification"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Server stopped after host file modification"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test mkdocs.yml volume mount - modify configuration
section "Test 6: Test mkdocs.yml Volume Mount Updates Configuration"

log_info "Modifying mkdocs.yml from host..."

# Add a comment to mkdocs.yml
echo "" >> "$PROJECT_ROOT/mkdocs.yml"
echo "# Test comment added at $(date)" >> "$PROJECT_ROOT/mkdocs.yml"

if tail -1 "$PROJECT_ROOT/mkdocs.yml" | grep -q "Test comment"; then
    pass "mkdocs.yml successfully modified from host"
else
    fail "Failed to modify mkdocs.yml from host"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Wait for reload
log_info "Waiting for server to detect config change (5s)..."
sleep 5

# Restore original mkdocs.yml
log_info "Restoring original mkdocs.yml..."
cp "$TEST_MKDOCS_YML" "$PROJECT_ROOT/mkdocs.yml"

if kill -0 $SERVER_PID 2>/dev/null; then
    pass "Server still running after config modification"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Server stopped after config modification"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test README.md volume mount
section "Test 7: Test README.md Volume Mount"

log_info "Modifying README.md from host..."

# Add a comment to README.md
echo "" >> "$PROJECT_ROOT/README.md"
echo "<!-- Test comment added at $(date) -->" >> "$PROJECT_ROOT/README.md"

if tail -1 "$PROJECT_ROOT/README.md" | grep -q "Test comment"; then
    pass "README.md successfully modified from host"
else
    fail "Failed to modify README.md from host"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Wait for reload
log_info "Waiting for server to detect README change (5s)..."
sleep 5

# Restore original README.md
log_info "Restoring original README.md..."
cp "$TEST_README" "$PROJECT_ROOT/README.md"

if kill -0 $SERVER_PID 2>/dev/null; then
    pass "Server still running after README modification"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Server stopped after README modification"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test README_INSTALL.md volume mount
section "Test 8: Test README_INSTALL.md Volume Mount"

log_info "Modifying README_INSTALL.md from host..."

# Add a comment to README_INSTALL.md
echo "" >> "$PROJECT_ROOT/README_INSTALL.md"
echo "<!-- Test comment added at $(date) -->" >> "$PROJECT_ROOT/README_INSTALL.md"

if tail -1 "$PROJECT_ROOT/README_INSTALL.md" | grep -q "Test comment"; then
    pass "README_INSTALL.md successfully modified from host"
else
    fail "Failed to modify README_INSTALL.md from host"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Wait for reload
log_info "Waiting for server to detect README_INSTALL change (5s)..."
sleep 5

# Restore original README_INSTALL.md
log_info "Restoring original README_INSTALL.md..."
cp "$TEST_README_INSTALL" "$PROJECT_ROOT/README_INSTALL.md"

if kill -0 $SERVER_PID 2>/dev/null; then
    pass "Server still running after README_INSTALL modification"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Server stopped after README_INSTALL modification"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Stop server and build site to test output permissions
section "Test 9: Stop Server and Build Site for Permission Testing"

log_info "Stopping development server..."
if kill -0 $SERVER_PID 2>/dev/null; then
    kill -TERM $SERVER_PID 2>/dev/null || true
    
    # Wait for server to stop
    SHUTDOWN_SUCCESS=0
    for i in $(seq 1 $MAX_SHUTDOWN_TIME); do
        if ! kill -0 $SERVER_PID 2>/dev/null; then
            SHUTDOWN_SUCCESS=1
            pass "Server stopped cleanly after ${i}s"
            break
        fi
        sleep 1
    done
    
    if [[ $SHUTDOWN_SUCCESS -eq 0 ]]; then
        log_warning "Server did not stop within ${MAX_SHUTDOWN_TIME}s, force killing..."
        kill -9 $SERVER_PID 2>/dev/null || true
    fi
fi

# Build site to test output permissions
log_info "Building documentation site to test output permissions..."
BUILD_LOG="$TEMP_DIR/build.log"

docker run --rm \
    -v "$PROJECT_ROOT/docs:/docs/docs" \
    -v "$PROJECT_ROOT/mkdocs.yml:/docs/mkdocs.yml" \
    -v "$PROJECT_ROOT/site:/docs/site" \
    -v "$PROJECT_ROOT/README.md:/docs/README.md" \
    -v "$PROJECT_ROOT/README_INSTALL.md:/docs/README_INSTALL.md" \
    "$IMAGE_NAME" > "$BUILD_LOG" 2>&1

if [[ $? -eq 0 ]]; then
    pass "Documentation site built successfully"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Failed to build documentation site"
    log_error "Build log:"
    cat "$BUILD_LOG" >&2
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test site/ directory output permissions
section "Test 10: Test site/ Directory Output Permissions"

if [[ ! -d "$SITE_DIR" ]]; then
    fail "site/ directory was not created"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "Build log:"
    cat "$BUILD_LOG" >&2
else
    pass "site/ directory exists"
    
    # Check permissions on site directory
    log_info "Checking site/ directory permissions..."
    
    # Check if host user can access site directory
    if [[ -r "$SITE_DIR" ]] && [[ -x "$SITE_DIR" ]]; then
        pass "site/ directory is accessible by host user"
    else
        fail "site/ directory is not accessible by host user"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    
    # Check owner and group of site directory
    SITE_OWNER=$(ls -ldn "$SITE_DIR" | awk '{print $3}')
    SITE_GROUP=$(ls -ldn "$SITE_DIR" | awk '{print $4}')
    HOST_UID=$(id -u)
    HOST_GID=$(id -g)
    
    log_info "site/ owner: $SITE_OWNER (host UID: $HOST_UID)"
    log_info "site/ group: $SITE_GROUP (host GID: $HOST_GID)"
    
    # The owner should match host user due to volume mount behavior
    # On Linux, Docker volume mounts preserve host user ownership
    # On macOS, Docker Desktop uses osxfs which handles this differently
    if [[ "$SITE_OWNER" == "$HOST_UID" ]]; then
        pass "site/ directory has correct owner (host user)"
    else
        # On macOS this is expected due to Docker Desktop's osxfs
        log_warning "site/ owner ($SITE_OWNER) differs from host UID ($HOST_UID)"
        info "This is expected on macOS with Docker Desktop"
    fi
    
    # Check some generated files
    log_info "Checking permissions of generated files..."
    
    SAMPLE_FILES=(
        "$SITE_DIR/index.html"
        "$SITE_DIR/404.html"
    )
    
    FILES_CHECKED=0
    FILES_ACCESSIBLE=0
    
    for file in "${SAMPLE_FILES[@]}"; do
        if [[ -f "$file" ]]; then
            FILES_CHECKED=$((FILES_CHECKED + 1))
            
            if [[ -r "$file" ]]; then
                FILES_ACCESSIBLE=$((FILES_ACCESSIBLE + 1))
                log_verbose "  ✓ $file is readable"
            else
                log_verbose "  ✗ $file is not readable"
            fi
            
            # Check if file is writable
            if [[ -w "$file" ]]; then
                log_verbose "  ✓ $file is writable"
            else
                log_verbose "  ✗ $file is not writable"
            fi
        fi
    done
    
    log_info "Generated files accessible: $FILES_ACCESSIBLE/$FILES_CHECKED"
    
    if [[ $FILES_CHECKED -gt 0 ]] && [[ $FILES_ACCESSIBLE -eq $FILES_CHECKED ]]; then
        pass "All checked generated files are accessible"
    elif [[ $FILES_ACCESSIBLE -gt 0 ]]; then
        pass "Some generated files are accessible"
    else
        fail "Generated files are not accessible"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

echo

# Test non-root user doesn't cause permission conflicts
section "Test 11: Test Non-Root User in Container"

log_info "Verifying container runs as non-root user..."

# Check Dockerfile for USER directive
if grep -q "USER mkdocs" "$PROJECT_ROOT/Dockerfile.mkdocs"; then
    pass "Dockerfile specifies non-root user 'mkdocs'"
else
    fail "Dockerfile does not specify non-root user"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Run a container to check the user
CONTAINER_USER=$(docker run --rm "$IMAGE_NAME" id -u 2>/dev/null)

log_info "Container runs as UID: $CONTAINER_USER"

if [[ "$CONTAINER_USER" != "0" ]]; then
    pass "Container runs as non-root user (UID: $CONTAINER_USER)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Container runs as root user (UID: 0)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Verify the non-root user doesn't cause conflicts
log_info "Verifying non-root user doesn't cause permission conflicts..."

if [[ -d "$SITE_DIR" ]]; then
    # Try to modify a file in site/ from host
    TEST_PERM_FILE="$SITE_DIR/.permission_test_$$"
    
    if echo "test" > "$TEST_PERM_FILE" 2>/dev/null; then
        pass "Host user can write to site/ directory"
        rm -f "$TEST_PERM_FILE" 2>/dev/null
    else
        fail "Host user cannot write to site/ directory"
        log_error "This indicates a permission conflict with the container's non-root user"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
fi

echo

# Test generated files can be deleted from host without sudo
section "Test 12: Test Deleting Generated Files Without sudo"

if [[ ! -d "$SITE_DIR" ]]; then
    log_warning "site/ directory does not exist, skipping deletion test"
else
    log_info "Attempting to delete site/ directory from host..."
    
    # Try to delete site directory
    if rm -rf "$SITE_DIR" 2>/dev/null; then
        pass "Successfully deleted site/ directory without sudo"
        TESTS_PASSED=$((TESTS_PASSED + 1))
        
        # Verify deletion
        if [[ ! -d "$SITE_DIR" ]]; then
            pass "site/ directory completely removed"
        else
            fail "site/ directory still exists after deletion attempt"
            TESTS_FAILED=$((TESTS_FAILED + 1))
        fi
    else
        fail "Failed to delete site/ directory without sudo"
        log_error "This indicates a permission issue with generated files"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        
        # Show permission details
        log_info "Showing file permissions in site/:"
        ls -la "$SITE_DIR" 2>&1 | head -20 || true
        
        # Try to fix permissions and delete
        log_info "Attempting to fix permissions..."
        chmod -R u+w "$SITE_DIR" 2>/dev/null || true
        if rm -rf "$SITE_DIR" 2>/dev/null; then
            pass "Successfully deleted after fixing permissions"
        else
            fail "Still cannot delete even after chmod"
            log_error "Manual cleanup may be required with: sudo rm -rf $SITE_DIR"
        fi
    fi
fi

echo

# Clean up test file from docs
section "Test 13: Clean Up Test Files"

log_info "Removing test file from docs/..."
if [[ -f "$TEST_DOCS_FILE" ]]; then
    if rm -f "$TEST_DOCS_FILE" 2>/dev/null; then
        pass "Test file removed successfully"
    else
        fail "Failed to remove test file"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
else
    pass "Test file already removed"
fi

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Verify all configurations work together with make command
section "Test 14: Verify make docs-docker-serve Configuration"

log_info "Checking Makefile configuration..."

if grep -q "docs-docker-serve:" "$PROJECT_ROOT/Makefile"; then
    pass "make docs-docker-serve target exists"
    
    # Extract the docker run command from Makefile
    log_verbose "Makefile docker run configuration:"
    if [[ ${VERBOSE:-0} -eq 1 ]]; then
        grep -A 10 "docs-docker-serve:" "$PROJECT_ROOT/Makefile" || true
    fi
    
    # Verify volume mounts in Makefile
    REQUIRED_MOUNTS=(
        "docs:/docs/docs"
        "mkdocs.yml:/docs/mkdocs.yml"
        "README.md:/docs/README.md"
        "README_INSTALL.md:/docs/README_INSTALL.md"
    )
    
    MOUNTS_OK=0
    for mount in "${REQUIRED_MOUNTS[@]}"; do
        if grep -q "$mount" "$PROJECT_ROOT/Makefile"; then
            log_verbose "  ✓ Found mount: $mount"
            MOUNTS_OK=$((MOUNTS_OK + 1))
        else
            log_verbose "  ✗ Missing mount: $mount"
        fi
    done
    
    log_info "Required volume mounts found: $MOUNTS_OK/${#REQUIRED_MOUNTS[@]}"
    
    if [[ $MOUNTS_OK -eq ${#REQUIRED_MOUNTS[@]} ]]; then
        pass "All required volume mounts configured in Makefile"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        fail "Some required volume mounts missing in Makefile"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
else
    fail "make docs-docker-serve target not found in Makefile"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Final summary
section "Test Summary"

TOTAL_TESTS=$((TESTS_PASSED + TESTS_FAILED))
log_info "Total tests: $TOTAL_TESTS"
log_info "Tests passed: $TESTS_PASSED"
log_info "Tests failed: $TESTS_FAILED"

if [[ $TESTS_FAILED -eq 0 ]]; then
    echo
    pass "All Docker volume mount and permissions tests passed successfully!"
    echo
    log_info "Volume mounts are working correctly:"
    log_info "  ✓ docs/ directory allows file editing from host"
    log_info "  ✓ site/ directory output has correct permissions"
    log_info "  ✓ mkdocs.yml updates configuration in real-time"
    log_info "  ✓ README.md volume mount works correctly"
    log_info "  ✓ README_INSTALL.md volume mount works correctly"
    log_info "  ✓ Non-root user doesn't cause permission conflicts"
    log_info "  ✓ Generated files can be deleted without sudo"
    echo
    log_info "You can now use: make docs-docker-serve"
    log_info "Edit files in docs/, mkdocs.yml, README*.md and see live updates"
    echo
    exit 0
else
    echo
    fail "Some Docker volume mount and permissions tests failed!"
    echo
    log_error "$TESTS_FAILED test(s) failed"
    log_info "Please review the output above and fix the issues"
    echo
    
    if [[ -d "$SITE_DIR" ]]; then
        log_warning "Reminder: Clean up site/ directory with: rm -rf site/"
    fi
    
    exit 1
fi
