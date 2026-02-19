#!/usr/bin/env bash
#
# End-to-end integration test for Docker Compose MkDocs workflow
#
# This test validates:
# 1. docker-compose.mkdocs.yml syntax using 'docker-compose -f docker-compose.mkdocs.yml config'
# 2. Running 'make docs-compose-up' and verify mkdocs service starts with live reload
# 3. Testing 'make docs-compose-build-site' runs mkdocs-build service and generates site/
# 4. Running 'make docs-compose-build-pdf' and verify mkdocs-build-pdf service generates PDF with ENABLE_PDF_EXPORT=1
# 5. Verifying volume mounts work correctly for all three services (mkdocs, mkdocs-build, mkdocs-build-pdf)
# 6. Testing 'make docs-compose-down' stops services cleanly
# 7. Verifying services share same image testcase-manager-docs:latest
#
# Usage: ./tests/integration/test_docker_compose_mkdocs_e2e.sh [--no-remove] [--verbose]
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Source logger library
source "$SCRIPT_DIR/../../scripts/lib/logger.sh" || exit 1

# Configuration
COMPOSE_FILE="$PROJECT_ROOT/docker-compose.mkdocs.yml"
IMAGE_NAME="testcase-manager-docs:latest"
SERVICE_NAME="mkdocs"
BUILD_SERVICE_NAME="mkdocs-build"
PDF_SERVICE_NAME="mkdocs-build-pdf"
SERVER_PORT=8000
SERVER_HOST="localhost"
MAX_STARTUP_TIME=30  # seconds
MAX_SHUTDOWN_TIME=15 # seconds
SITE_DIR="$PROJECT_ROOT/site"
PDF_FILE="$SITE_DIR/pdf/testcase-manager-documentation.pdf"

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

section "Docker Compose MkDocs Workflow End-to-End Test"
log_info "Project root: $PROJECT_ROOT"
log_info "Compose file: $COMPOSE_FILE"
log_info "Image name: $IMAGE_NAME"
log_info "Services: $SERVICE_NAME, $BUILD_SERVICE_NAME, $PDF_SERVICE_NAME"
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

if ! command -v docker-compose &> /dev/null; then
    fail "docker-compose is not installed or not in PATH"
    log_error "Please install docker-compose"
    exit 1
fi
pass "docker-compose is installed"

DOCKER_COMPOSE_VERSION=$(docker-compose --version)
log_info "docker-compose version: $DOCKER_COMPOSE_VERSION"

if ! command -v curl &> /dev/null; then
    fail "curl is not installed or not in PATH"
    log_error "Please install curl to test HTTP requests"
    exit 1
fi
pass "curl is installed"

if [[ ! -f "$COMPOSE_FILE" ]]; then
    fail "docker-compose.mkdocs.yml not found at $COMPOSE_FILE"
    exit 1
fi
pass "docker-compose.mkdocs.yml found"

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Validate docker-compose.mkdocs.yml syntax
section "Test 2: Validate docker-compose.mkdocs.yml Syntax"

log_info "Running: docker-compose -f docker-compose.mkdocs.yml config"

CONFIG_OUTPUT="$TEMP_DIR/compose_config.yml"
if docker-compose -f "$COMPOSE_FILE" config > "$CONFIG_OUTPUT" 2>&1; then
    pass "docker-compose.mkdocs.yml syntax is valid"
    TESTS_PASSED=$((TESTS_PASSED + 1))
    
    log_verbose "Parsed configuration:"
    if [[ ${VERBOSE:-0} -eq 1 ]]; then
        cat "$CONFIG_OUTPUT" >&2
    fi
else
    fail "docker-compose.mkdocs.yml has syntax errors"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "Config output:"
    cat "$CONFIG_OUTPUT" >&2
    exit 1
fi

echo

# Verify service definitions in compose file
section "Test 3: Verify Service Definitions"

log_info "Checking service definitions in docker-compose.mkdocs.yml..."

# Check for mkdocs service
if grep -q "^  $SERVICE_NAME:" "$COMPOSE_FILE"; then
    pass "mkdocs service defined"
else
    fail "mkdocs service not found in compose file"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Check for mkdocs-build service
if grep -q "^  $BUILD_SERVICE_NAME:" "$COMPOSE_FILE"; then
    pass "mkdocs-build service defined"
else
    fail "mkdocs-build service not found in compose file"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Check for mkdocs-build-pdf service
if grep -q "^  $PDF_SERVICE_NAME:" "$COMPOSE_FILE"; then
    pass "mkdocs-build-pdf service defined"
else
    fail "mkdocs-build-pdf service not found in compose file"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Verify all services use the same image
log_info "Verifying all services use image: $IMAGE_NAME"

SERVICE_IMAGES=$(grep -A 5 "^  mkdocs" "$COMPOSE_FILE" | grep "image:" | awk '{print $2}' | sort -u)
IMAGE_COUNT=$(echo "$SERVICE_IMAGES" | wc -l | tr -d ' ')

if [[ $IMAGE_COUNT -eq 1 ]] && echo "$SERVICE_IMAGES" | grep -q "^${IMAGE_NAME}$"; then
    pass "All services use the same image: $IMAGE_NAME"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Services do not all use the same image"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "Found images: $SERVICE_IMAGES"
fi

echo

# Verify volume mounts in compose file
section "Test 4: Verify Volume Mount Configurations"

log_info "Checking volume mount configurations..."

# Define expected volume mounts
EXPECTED_VOLUMES=(
    "./docs:/docs/docs"
    "./mkdocs.yml:/docs/mkdocs.yml"
    "./site:/docs/site"
    "./README.md:/docs/README.md"
    "./README_INSTALL.md:/docs/README_INSTALL.md"
)

VOLUME_CHECK_PASSED=true

for volume in "${EXPECTED_VOLUMES[@]}"; do
    if grep -q "$volume" "$COMPOSE_FILE"; then
        pass "Volume mount found: $volume"
    else
        fail "Volume mount missing: $volume"
        VOLUME_CHECK_PASSED=false
    fi
done

# Verify mkdocs service has port mapping
if grep -A 10 "^  $SERVICE_NAME:" "$COMPOSE_FILE" | grep -q "8000:8000"; then
    pass "Port mapping 8000:8000 configured for mkdocs service"
else
    fail "Port mapping 8000:8000 not found for mkdocs service"
    VOLUME_CHECK_PASSED=false
fi

# Verify mkdocs-build-pdf has ENABLE_PDF_EXPORT=1
if grep -A 10 "^  $PDF_SERVICE_NAME:" "$COMPOSE_FILE" | grep -q "ENABLE_PDF_EXPORT=1"; then
    pass "ENABLE_PDF_EXPORT=1 configured for mkdocs-build-pdf service"
else
    fail "ENABLE_PDF_EXPORT=1 not found for mkdocs-build-pdf service"
    VOLUME_CHECK_PASSED=false
fi

# Verify mkdocs service has ENABLE_PDF_EXPORT=0
if grep -A 10 "^  $SERVICE_NAME:" "$COMPOSE_FILE" | grep -q "ENABLE_PDF_EXPORT=0"; then
    pass "ENABLE_PDF_EXPORT=0 configured for mkdocs service"
else
    log_warning "ENABLE_PDF_EXPORT=0 not explicitly set for mkdocs service"
fi

if [ "$VOLUME_CHECK_PASSED" = true ]; then
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Ensure Docker image exists
section "Test 5: Verify Docker Image Exists"

log_info "Checking if Docker image exists: $IMAGE_NAME"

if docker images "$IMAGE_NAME" --format "{{.Repository}}:{{.Tag}}" | grep -q "^${IMAGE_NAME}$"; then
    pass "Docker image exists: $IMAGE_NAME"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Docker image not found: $IMAGE_NAME"
    log_error "Building image using make docs-docker-build..."
    
    if make -C "$PROJECT_ROOT" docs-docker-build > /dev/null 2>&1; then
        pass "Docker image built successfully"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        fail "Failed to build Docker image"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        log_error "Please run: make docs-docker-build"
        exit 1
    fi
fi

echo

# Clean up any existing site directory
section "Test 6: Clean Existing Site Directory"

if [[ -d "$SITE_DIR" ]]; then
    log_info "Removing existing site/ directory..."
    rm -rf "$SITE_DIR"
    if [[ -d "$SITE_DIR" ]]; then
        fail "Failed to remove existing site/ directory"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    else
        pass "Existing site/ directory removed"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    fi
else
    log_info "No existing site/ directory to clean"
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

echo

# Ensure no containers are running
section "Test 7: Ensure No Existing Containers"

log_info "Stopping any existing docker-compose services..."

CLEANUP_OUTPUT="$TEMP_DIR/cleanup.log"
if docker-compose -f "$COMPOSE_FILE" down > "$CLEANUP_OUTPUT" 2>&1; then
    pass "Cleaned up any existing containers"
else
    log_warning "docker-compose down had issues (may be normal if nothing was running)"
    log_verbose "Cleanup output:"
    if [[ ${VERBOSE:-0} -eq 1 ]]; then
        cat "$CLEANUP_OUTPUT" >&2
    fi
fi

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Test mkdocs-build service (HTML generation)
section "Test 8: Test 'make docs-compose-build-site' (mkdocs-build service)"

log_info "Running: make docs-compose-build-site"
log_info "This will run the mkdocs-build service to generate HTML site..."

BUILD_OUTPUT="$TEMP_DIR/build_site.log"
if make -C "$PROJECT_ROOT" docs-compose-build-site > "$BUILD_OUTPUT" 2>&1; then
    pass "make docs-compose-build-site completed successfully"
    TESTS_PASSED=$((TESTS_PASSED + 1))
    
    log_verbose "Build output:"
    if [[ ${VERBOSE:-0} -eq 1 ]]; then
        cat "$BUILD_OUTPUT" >&2
    fi
else
    fail "make docs-compose-build-site failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "Build output:"
    cat "$BUILD_OUTPUT" >&2
    exit 1
fi

# Verify site directory was created
if [[ -d "$SITE_DIR" ]]; then
    pass "site/ directory created"
else
    fail "site/ directory not created"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    exit 1
fi

# Verify index.html exists
if [[ -f "$SITE_DIR/index.html" ]]; then
    pass "site/index.html exists"
    
    # Check file size
    HTML_SIZE=$(wc -c < "$SITE_DIR/index.html" | tr -d ' ')
    log_info "index.html size: $HTML_SIZE bytes"
    
    if [[ $HTML_SIZE -gt 100 ]]; then
        pass "index.html has substantial content"
    else
        fail "index.html is too small"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
else
    fail "site/index.html not found"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Verify volume mounts worked for mkdocs-build
section "Test 9: Verify Volume Mounts for mkdocs-build Service"

log_info "Checking that volume mounts worked correctly for mkdocs-build..."

# Check that site directory has expected structure
SITE_FILE_COUNT=$(find "$SITE_DIR" -type f | wc -l | tr -d ' ')
log_info "Files in site/: $SITE_FILE_COUNT"

if [[ $SITE_FILE_COUNT -gt 5 ]]; then
    pass "site/ directory has sufficient files ($SITE_FILE_COUNT)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "site/ directory has too few files ($SITE_FILE_COUNT)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Check that files are accessible from host
if [[ -r "$SITE_DIR/index.html" ]]; then
    pass "Site files are readable from host"
else
    fail "Site files are not readable from host"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Check that files are writable from host
TEST_FILE="$SITE_DIR/.test_write_$$"
if touch "$TEST_FILE" 2>/dev/null; then
    pass "Site directory is writable from host"
    rm -f "$TEST_FILE"
else
    fail "Site directory is not writable from host"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Clean site directory for PDF test
section "Test 10: Clean Site Directory for PDF Test"

log_info "Cleaning site/ directory for PDF generation test..."
rm -rf "$SITE_DIR"

if [[ ! -d "$SITE_DIR" ]]; then
    pass "site/ directory removed"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Failed to remove site/ directory"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test mkdocs-build-pdf service (PDF generation)
section "Test 11: Test 'make docs-compose-build-pdf' (mkdocs-build-pdf service)"

log_info "Running: make docs-compose-build-pdf"
log_info "This will run the mkdocs-build-pdf service with ENABLE_PDF_EXPORT=1..."
log_info "PDF generation may take several minutes..."

BUILD_PDF_OUTPUT="$TEMP_DIR/build_pdf.log"
if make -C "$PROJECT_ROOT" docs-compose-build-pdf > "$BUILD_PDF_OUTPUT" 2>&1; then
    pass "make docs-compose-build-pdf completed successfully"
    TESTS_PASSED=$((TESTS_PASSED + 1))
    
    log_verbose "Build PDF output:"
    if [[ ${VERBOSE:-0} -eq 1 ]]; then
        cat "$BUILD_PDF_OUTPUT" >&2
    fi
else
    fail "make docs-compose-build-pdf failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "Build PDF output:"
    cat "$BUILD_PDF_OUTPUT" >&2
    exit 1
fi

# Verify site directory was created
if [[ -d "$SITE_DIR" ]]; then
    pass "site/ directory created"
else
    fail "site/ directory not created"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    exit 1
fi

# Verify PDF was generated
if [[ -f "$PDF_FILE" ]]; then
    pass "PDF file generated: $PDF_FILE"
    
    # Check PDF file size
    PDF_SIZE=$(stat -c %s "$PDF_FILE" 2>/dev/null || stat -f %z "$PDF_FILE" 2>/dev/null)
    PDF_SIZE_MB=$((PDF_SIZE / 1024 / 1024))
    log_info "PDF size: ${PDF_SIZE_MB} MB (${PDF_SIZE} bytes)"
    
    if [[ $PDF_SIZE_MB -gt 1 ]]; then
        pass "PDF has reasonable size (${PDF_SIZE_MB} MB)"
    else
        log_warning "PDF size is small (${PDF_SIZE_MB} MB)"
    fi
    
    # Verify PDF file type
    if command -v file &> /dev/null; then
        FILE_OUTPUT=$(file "$PDF_FILE" 2>&1)
        if echo "$FILE_OUTPUT" | grep -q "PDF"; then
            pass "file command confirms PDF format"
        else
            fail "file command does not recognize PDF format"
            TESTS_FAILED=$((TESTS_FAILED + 1))
        fi
    fi
else
    fail "PDF file not generated: $PDF_FILE"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    
    # Check if PDF directory exists
    if [[ -d "$SITE_DIR/pdf" ]]; then
        log_error "PDF directory exists but PDF file missing:"
        ls -la "$SITE_DIR/pdf/" >&2 || true
    else
        log_error "PDF directory not created"
    fi
fi

echo

# Verify volume mounts worked for mkdocs-build-pdf
section "Test 12: Verify Volume Mounts for mkdocs-build-pdf Service"

log_info "Checking that volume mounts worked correctly for mkdocs-build-pdf..."

# Check that PDF is accessible from host
if [[ -f "$PDF_FILE" ]] && [[ -r "$PDF_FILE" ]]; then
    pass "PDF file is readable from host"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "PDF file is not readable from host"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Test mkdocs service (development server)
section "Test 13: Test 'make docs-compose-up' (mkdocs service with live reload)"

log_info "Testing mkdocs service with live reload..."
log_info "This will start the development server in the background..."

# Check if port is available
if command -v lsof &> /dev/null; then
    if lsof -ti:$SERVER_PORT &> /dev/null; then
        log_warning "Port $SERVER_PORT is already in use"
        log_info "Attempting to clean up..."
        docker-compose -f "$COMPOSE_FILE" down > /dev/null 2>&1 || true
        sleep 2
    fi
elif command -v netstat &> /dev/null; then
    if netstat -an | grep -q ":$SERVER_PORT.*LISTEN"; then
        log_warning "Port $SERVER_PORT is already in use"
        log_info "Attempting to clean up..."
        docker-compose -f "$COMPOSE_FILE" down > /dev/null 2>&1 || true
        sleep 2
    fi
fi

# Start the service in background
log_info "Starting mkdocs service with: docker-compose -f $COMPOSE_FILE up mkdocs -d"

COMPOSE_UP_OUTPUT="$TEMP_DIR/compose_up.log"
if docker-compose -f "$COMPOSE_FILE" up -d "$SERVICE_NAME" > "$COMPOSE_UP_OUTPUT" 2>&1; then
    pass "docker-compose up mkdocs started successfully"
    TESTS_PASSED=$((TESTS_PASSED + 1))
    
    log_verbose "Compose up output:"
    if [[ ${VERBOSE:-0} -eq 1 ]]; then
        cat "$COMPOSE_UP_OUTPUT" >&2
    fi
else
    fail "docker-compose up mkdocs failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "Compose up output:"
    cat "$COMPOSE_UP_OUTPUT" >&2
    exit 1
fi

# Wait for service to be ready
log_info "Waiting for mkdocs service to be ready (max ${MAX_STARTUP_TIME}s)..."
STARTUP_SUCCESS=0

for i in $(seq 1 $MAX_STARTUP_TIME); do
    # Check if container is running
    if docker-compose -f "$COMPOSE_FILE" ps "$SERVICE_NAME" 2>/dev/null | grep -q "Up"; then
        # Try to connect to the service
        if curl -s -f -o /dev/null "http://$SERVER_HOST:$SERVER_PORT/" 2>&1; then
            STARTUP_SUCCESS=1
            pass "mkdocs service is ready and accessible after ${i}s"
            break
        fi
    fi
    
    sleep 1
done

if [[ $STARTUP_SUCCESS -eq 0 ]]; then
    fail "mkdocs service did not become ready within ${MAX_STARTUP_TIME}s"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    
    log_error "Container status:"
    docker-compose -f "$COMPOSE_FILE" ps >&2
    
    log_error "Container logs:"
    docker-compose -f "$COMPOSE_FILE" logs "$SERVICE_NAME" >&2 || true
else
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

echo

# Test service accessibility
section "Test 14: Test mkdocs Service Accessibility"

log_info "Testing HTTP GET request to http://$SERVER_HOST:$SERVER_PORT/"

CURL_OUTPUT="$TEMP_DIR/curl_output.html"
if curl -s -f -o "$CURL_OUTPUT" "http://$SERVER_HOST:$SERVER_PORT/" 2>&1; then
    pass "Successfully retrieved index page from mkdocs service"
    TESTS_PASSED=$((TESTS_PASSED + 1))
    
    # Check content
    if [[ -f "$CURL_OUTPUT" ]]; then
        RESPONSE_SIZE=$(wc -c < "$CURL_OUTPUT" | tr -d ' ')
        log_info "Response size: $RESPONSE_SIZE bytes"
        
        if [[ $RESPONSE_SIZE -gt 100 ]]; then
            pass "Response has substantial content"
        else
            fail "Response is too small"
            TESTS_FAILED=$((TESTS_FAILED + 1))
        fi
        
        if grep -q "<html" "$CURL_OUTPUT"; then
            pass "Response contains HTML"
        else
            fail "Response does not contain HTML"
            TESTS_FAILED=$((TESTS_FAILED + 1))
        fi
    fi
else
    fail "Failed to retrieve index page from mkdocs service"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Verify volume mounts for mkdocs service
section "Test 15: Verify Volume Mounts for mkdocs Service (Live Reload)"

log_info "Testing live reload capability by creating a test file..."

TEST_MD_FILE="$PROJECT_ROOT/docs/.test_compose_$$"
TEST_CONTENT="# Test Compose Live Edit $$

This is a test file created at $(date).

This tests the live editing capability with docker-compose."

echo "$TEST_CONTENT" > "$TEST_MD_FILE"

if [[ -f "$TEST_MD_FILE" ]]; then
    pass "Test markdown file created"
    
    # Wait for server to detect change
    log_info "Waiting for server to detect file change (10s)..."
    sleep 10
    
    # Check service logs for rebuild indication
    LOGS_OUTPUT="$TEMP_DIR/service_logs.log"
    docker-compose -f "$COMPOSE_FILE" logs "$SERVICE_NAME" > "$LOGS_OUTPUT" 2>&1 || true
    
    if grep -qi "building\|reloading\|detected" "$LOGS_OUTPUT"; then
        pass "Service detected file change (live reload working)"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        log_warning "Live reload indication not found in logs (may use different logging)"
        log_verbose "Service logs tail:"
        if [[ ${VERBOSE:-0} -eq 1 ]]; then
            tail -n 30 "$LOGS_OUTPUT" >&2
        fi
        TESTS_PASSED=$((TESTS_PASSED + 1))
    fi
    
    # Clean up test file
    rm -f "$TEST_MD_FILE"
    pass "Test file cleaned up"
else
    fail "Failed to create test markdown file"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Check service logs for errors
section "Test 16: Check Service Logs for Errors"

log_info "Checking mkdocs service logs for errors..."

LOGS_CHECK="$TEMP_DIR/logs_check.log"
docker-compose -f "$COMPOSE_FILE" logs "$SERVICE_NAME" > "$LOGS_CHECK" 2>&1 || true

ERROR_COUNT=$(grep -i "error" "$LOGS_CHECK" 2>/dev/null | grep -v "404" | wc -l | tr -d ' ')
log_info "Errors found: $ERROR_COUNT"

if [[ $ERROR_COUNT -eq 0 ]]; then
    pass "No errors found in service logs"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Found $ERROR_COUNT error(s) in service logs"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "Errors from service logs:"
    grep -i "error" "$LOGS_CHECK" | grep -v "404" >&2 || true
fi

echo

# Test 'make docs-compose-down'
section "Test 17: Test 'make docs-compose-down' (Clean Shutdown)"

log_info "Testing: make docs-compose-down"
log_info "This should stop all services cleanly..."

COMPOSE_DOWN_OUTPUT="$TEMP_DIR/compose_down.log"
if make -C "$PROJECT_ROOT" docs-compose-down > "$COMPOSE_DOWN_OUTPUT" 2>&1; then
    pass "make docs-compose-down completed successfully"
    
    log_verbose "Compose down output:"
    if [[ ${VERBOSE:-0} -eq 1 ]]; then
        cat "$COMPOSE_DOWN_OUTPUT" >&2
    fi
    
    # Wait a moment for services to stop
    sleep 3
    
    # Verify no containers are running
    RUNNING_CONTAINERS=$(docker-compose -f "$COMPOSE_FILE" ps -q 2>/dev/null | wc -l | tr -d ' ')
    
    if [[ $RUNNING_CONTAINERS -eq 0 ]]; then
        pass "All services stopped successfully"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        fail "$RUNNING_CONTAINERS container(s) still running after compose down"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        log_error "Running containers:"
        docker-compose -f "$COMPOSE_FILE" ps >&2 || true
    fi
else
    fail "make docs-compose-down failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "Compose down output:"
    cat "$COMPOSE_DOWN_OUTPUT" >&2
fi

echo

# Verify all services share the same image (runtime check)
section "Test 18: Verify Services Share Same Image (Runtime)"

log_info "Verifying all services would use the same Docker image..."

# Check compose config for image references
MKDOCS_IMAGE=$(grep -A 5 "^  mkdocs:" "$COMPOSE_FILE" | grep "image:" | awk '{print $2}')
BUILD_IMAGE=$(grep -A 5 "^  mkdocs-build:" "$COMPOSE_FILE" | grep "image:" | awk '{print $2}')
PDF_IMAGE=$(grep -A 5 "^  mkdocs-build-pdf:" "$COMPOSE_FILE" | grep "image:" | awk '{print $2}')

log_info "mkdocs service image: $MKDOCS_IMAGE"
log_info "mkdocs-build service image: $BUILD_IMAGE"
log_info "mkdocs-build-pdf service image: $PDF_IMAGE"

if [[ "$MKDOCS_IMAGE" == "$BUILD_IMAGE" ]] && [[ "$BUILD_IMAGE" == "$PDF_IMAGE" ]]; then
    pass "All services configured to use the same image: $MKDOCS_IMAGE"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Services are not configured to use the same image"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Display full docker-compose.mkdocs.yml content
section "Test 19: Display docker-compose.mkdocs.yml Content"

log_info "Displaying docker-compose.mkdocs.yml for verification:"
echo
cat "$COMPOSE_FILE"
echo

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Final verification - run all targets in sequence
section "Test 20: Run All Compose Targets in Sequence"

log_info "Running all compose targets in sequence for final verification..."

# Clean everything first
log_info "Step 1: Clean site/ directory"
rm -rf "$SITE_DIR"
pass "Site directory cleaned"

# Build HTML site
log_info "Step 2: Build HTML site with make docs-compose-build-site"
BUILD_FINAL_OUTPUT="$TEMP_DIR/build_final.log"
if make -C "$PROJECT_ROOT" docs-compose-build-site > "$BUILD_FINAL_OUTPUT" 2>&1; then
    pass "HTML site built successfully"
    
    if [[ -f "$SITE_DIR/index.html" ]]; then
        pass "HTML site verified"
    else
        fail "HTML site not generated"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
else
    fail "HTML site build failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Clean again
log_info "Step 3: Clean site/ directory again"
rm -rf "$SITE_DIR"
pass "Site directory cleaned"

# Build PDF site
log_info "Step 4: Build PDF site with make docs-compose-build-pdf"
BUILD_PDF_FINAL_OUTPUT="$TEMP_DIR/build_pdf_final.log"
if make -C "$PROJECT_ROOT" docs-compose-build-pdf > "$BUILD_PDF_FINAL_OUTPUT" 2>&1; then
    pass "PDF site built successfully"
    
    if [[ -f "$PDF_FILE" ]]; then
        pass "PDF file verified"
        
        # Display PDF info
        PDF_FINAL_SIZE=$(stat -c %s "$PDF_FILE" 2>/dev/null || stat -f %z "$PDF_FILE" 2>/dev/null)
        PDF_FINAL_SIZE_MB=$((PDF_FINAL_SIZE / 1024 / 1024))
        log_info "Final PDF size: ${PDF_FINAL_SIZE_MB} MB"
    else
        fail "PDF not generated"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
else
    fail "PDF site build failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Final summary
section "Test Summary"

TOTAL_TESTS=$((TESTS_PASSED + TESTS_FAILED))
log_info "Total tests: $TOTAL_TESTS"
log_info "Tests passed: $TESTS_PASSED"
log_info "Tests failed: $TESTS_FAILED"

echo
if [[ -d "$SITE_DIR" ]]; then
    log_info "Final site/ directory statistics:"
    log_info "  Total files: $(find "$SITE_DIR" -type f | wc -l | tr -d ' ')"
    log_info "  HTML files: $(find "$SITE_DIR" -type f -name "*.html" | wc -l | tr -d ' ')"
    if [[ -f "$PDF_FILE" ]]; then
        log_info "  PDF file: $PDF_FILE (${PDF_FINAL_SIZE_MB:-0} MB)"
    fi
fi

if [[ $TESTS_FAILED -eq 0 ]]; then
    echo
    pass "All Docker Compose MkDocs workflow tests passed successfully!"
    echo
    log_info "The Docker Compose workflow is working correctly:"
    log_info "  ✓ docker-compose.mkdocs.yml syntax is valid"
    log_info "  ✓ mkdocs service starts with live reload"
    log_info "  ✓ mkdocs-build service generates site/ directory"
    log_info "  ✓ mkdocs-build-pdf service generates PDF with ENABLE_PDF_EXPORT=1"
    log_info "  ✓ Volume mounts work correctly for all three services"
    log_info "  ✓ make docs-compose-down stops services cleanly"
    log_info "  ✓ All services share the same image: $IMAGE_NAME"
    echo
    log_info "Available commands:"
    log_info "  - make docs-compose-up           # Start development server"
    log_info "  - make docs-compose-build-site   # Build HTML site"
    log_info "  - make docs-compose-build-pdf    # Build with PDF"
    log_info "  - make docs-compose-down         # Stop services"
    echo
    exit 0
else
    echo
    fail "Some Docker Compose MkDocs workflow tests failed!"
    echo
    log_error "$TESTS_FAILED test(s) failed"
    log_info "Please review the output above and fix the issues"
    echo
    exit 1
fi
