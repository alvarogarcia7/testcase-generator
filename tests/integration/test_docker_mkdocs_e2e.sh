#!/usr/bin/env bash
#
# End-to-end integration test for Docker MkDocs setup
#
# This test validates:
# 1. Dockerfile.mkdocs syntax and best practices
# 2. Docker image build via 'make docs-docker-build'
# 3. Image creation and tag verification (testcase-manager-docs:latest)
# 4. Python dependencies installation (mkdocs, mkdocs-material, mkdocs-with-pdf)
# 5. System dependencies for PDF generation (libcairo2, libpango, libffi-dev)
# 6. Non-root user 'mkdocs' creation with correct permissions
# 7. Image size validation (< 1GB)
# 8. ENABLE_PDF_EXPORT environment variable defaults to 0
#
# Usage: ./tests/integration/test_docker_mkdocs_e2e.sh [--no-remove]
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Source logger library
source "$SCRIPT_DIR/../../scripts/lib/logger.sh" || exit 1

# Configuration
DOCKERFILE_PATH="$PROJECT_ROOT/Dockerfile.mkdocs"
IMAGE_NAME="testcase-manager-docs"
IMAGE_TAG="latest"
FULL_IMAGE_NAME="${IMAGE_NAME}:${IMAGE_TAG}"
MAX_IMAGE_SIZE_BYTES=$((1 * 1024 * 1024 * 1024))  # 1GB in bytes

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

section "Docker MkDocs End-to-End Integration Test"
log_info "Project root: $PROJECT_ROOT"
log_info "Dockerfile: $DOCKERFILE_PATH"
log_info "Image name: $FULL_IMAGE_NAME"
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

if [[ ! -f "$DOCKERFILE_PATH" ]]; then
    fail "Dockerfile not found at $DOCKERFILE_PATH"
    exit 1
fi
pass "Dockerfile.mkdocs found"

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Test Dockerfile syntax
section "Test 2: Validate Dockerfile Syntax"

log_info "Checking Dockerfile syntax..."
if docker build --no-cache -f "$DOCKERFILE_PATH" -t temp-dockerfile-test --target 2>&1 | grep -q "Error"; then
    fail "Dockerfile has syntax errors"
    TESTS_FAILED=$((TESTS_FAILED + 1))
else
    pass "Dockerfile syntax is valid"
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi

# Check Dockerfile best practices
log_info "Checking Dockerfile best practices..."
BEST_PRACTICES_PASSED=true

# Check: FROM instruction uses specific version
if grep -q "^FROM python:3.12-slim" "$DOCKERFILE_PATH"; then
    pass "Uses specific Python version (3.12-slim)"
else
    fail "Should use specific Python version instead of 'latest'"
    BEST_PRACTICES_PASSED=false
fi

# Check: WORKDIR is set
if grep -q "^WORKDIR" "$DOCKERFILE_PATH"; then
    pass "WORKDIR is set"
else
    fail "WORKDIR should be set"
    BEST_PRACTICES_PASSED=false
fi

# Check: Cleanup of apt lists
if grep -q "rm -rf /var/lib/apt/lists/\*" "$DOCKERFILE_PATH"; then
    pass "Cleans up apt lists to reduce image size"
else
    fail "Should clean up /var/lib/apt/lists/* to reduce image size"
    BEST_PRACTICES_PASSED=false
fi

# Check: Uses --no-cache-dir for pip
if grep -q "pip install --no-cache-dir" "$DOCKERFILE_PATH"; then
    pass "Uses --no-cache-dir for pip installations"
else
    fail "Should use --no-cache-dir for pip to reduce image size"
    BEST_PRACTICES_PASSED=false
fi

# Check: Non-root user is created
if grep -q "useradd.*mkdocs" "$DOCKERFILE_PATH"; then
    pass "Creates non-root user 'mkdocs'"
else
    fail "Should create non-root user for security"
    BEST_PRACTICES_PASSED=false
fi

# Check: Switches to non-root user
if grep -q "^USER mkdocs" "$DOCKERFILE_PATH"; then
    pass "Switches to non-root user"
else
    fail "Should switch to non-root user"
    BEST_PRACTICES_PASSED=false
fi

# Check: EXPOSE directive for documentation
if grep -q "^EXPOSE 8000" "$DOCKERFILE_PATH"; then
    pass "Exposes port 8000 for documentation server"
else
    fail "Should expose port 8000"
    BEST_PRACTICES_PASSED=false
fi

# Check: ENABLE_PDF_EXPORT environment variable
if grep -q "ENV ENABLE_PDF_EXPORT=0" "$DOCKERFILE_PATH"; then
    pass "Sets ENABLE_PDF_EXPORT environment variable to 0"
else
    fail "Should set ENABLE_PDF_EXPORT=0 by default"
    BEST_PRACTICES_PASSED=false
fi

# Check: LABEL for metadata
if grep -q "^LABEL" "$DOCKERFILE_PATH"; then
    pass "Includes LABEL metadata"
else
    log_warning "Consider adding LABEL metadata for maintainability"
fi

if [ "$BEST_PRACTICES_PASSED" = true ]; then
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Build Docker image
section "Test 3: Build Docker Image"

log_info "Running: make docs-docker-build"
log_info "This may take several minutes on first build..."

if make -C "$PROJECT_ROOT" docs-docker-build; then
    pass "Docker image built successfully via make"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Failed to build Docker image"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "Build failed. Check the output above for errors."
    exit 1
fi

echo

# Verify image exists
section "Test 4: Verify Image Creation"

log_info "Checking if image exists: $FULL_IMAGE_NAME"
if docker images "$FULL_IMAGE_NAME" --format "{{.Repository}}:{{.Tag}}" | grep -q "^${FULL_IMAGE_NAME}$"; then
    pass "Image $FULL_IMAGE_NAME exists"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Image $FULL_IMAGE_NAME not found"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "Available images:"
    docker images "$IMAGE_NAME"
    exit 1
fi

# Display image information
log_info "Image information:"
docker images "$FULL_IMAGE_NAME" --format "table {{.Repository}}\t{{.Tag}}\t{{.Size}}\t{{.CreatedAt}}"

echo

# Verify image size
section "Test 5: Verify Image Size"

IMAGE_SIZE_BYTES=$(docker images "$FULL_IMAGE_NAME" --format "{{.Size}}" | sed 's/GB$//' | sed 's/MB$//' | awk '{
    if (index($0, "GB")) {
        size = substr($0, 1, length($0)-2)
        print size * 1024 * 1024 * 1024
    } else if (index($0, "MB")) {
        size = substr($0, 1, length($0)-2)
        print size * 1024 * 1024
    } else {
        print $0
    }
}')

# Fallback: get size from docker inspect
if [[ -z "$IMAGE_SIZE_BYTES" ]] || [[ "$IMAGE_SIZE_BYTES" == "0" ]]; then
    IMAGE_SIZE_BYTES=$(docker inspect "$FULL_IMAGE_NAME" --format='{{.Size}}')
fi

IMAGE_SIZE_MB=$((IMAGE_SIZE_BYTES / 1024 / 1024))
IMAGE_SIZE_GB=$((IMAGE_SIZE_BYTES / 1024 / 1024 / 1024))

log_info "Image size: ${IMAGE_SIZE_MB} MB (${IMAGE_SIZE_GB} GB)"

if [[ $IMAGE_SIZE_BYTES -lt $MAX_IMAGE_SIZE_BYTES ]]; then
    pass "Image size is under 1GB limit (${IMAGE_SIZE_MB} MB)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Image size exceeds 1GB limit (${IMAGE_SIZE_MB} MB > 1024 MB)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    log_error "Consider optimizing the image to reduce size"
fi

echo

# Verify Python dependencies
section "Test 6: Verify Python Dependencies"

log_info "Checking installed Python packages..."

# Test mkdocs
if docker run --rm "$FULL_IMAGE_NAME" mkdocs --version &> /dev/null; then
    MKDOCS_VERSION=$(docker run --rm "$FULL_IMAGE_NAME" mkdocs --version 2>&1 | head -1)
    pass "mkdocs is installed: $MKDOCS_VERSION"
else
    fail "mkdocs is not installed or not accessible"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test mkdocs-material
if docker run --rm "$FULL_IMAGE_NAME" python -c "import material" &> /dev/null; then
    MATERIAL_VERSION=$(docker run --rm "$FULL_IMAGE_NAME" python -c "import material; print(material.__version__)" 2>&1)
    pass "mkdocs-material is installed: $MATERIAL_VERSION"
else
    fail "mkdocs-material is not installed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test mkdocs-with-pdf
if docker run --rm "$FULL_IMAGE_NAME" python -c "import mkdocs_with_pdf" &> /dev/null; then
    pass "mkdocs-with-pdf is installed"
else
    fail "mkdocs-with-pdf is not installed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# List all installed packages
log_info "Installed Python packages:"
docker run --rm "$FULL_IMAGE_NAME" pip list 2>&1 | grep -E "(mkdocs|markdown|pymdown)" || true

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Verify system dependencies for PDF
section "Test 7: Verify System Dependencies for PDF"

log_info "Checking system dependencies for PDF generation..."

# Check libcairo2
if docker run --rm "$FULL_IMAGE_NAME" bash -c "dpkg -l | grep -q libcairo2" 2>/dev/null; then
    pass "libcairo2 is installed"
else
    fail "libcairo2 is not installed (required for PDF generation)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Check libpango
if docker run --rm "$FULL_IMAGE_NAME" bash -c "dpkg -l | grep -q 'libpango-1.0-0'" 2>/dev/null; then
    pass "libpango-1.0-0 is installed"
else
    fail "libpango-1.0-0 is not installed (required for PDF generation)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Check libffi-dev
if docker run --rm "$FULL_IMAGE_NAME" bash -c "dpkg -l | grep -q libffi-dev" 2>/dev/null; then
    pass "libffi-dev is installed"
else
    fail "libffi-dev is not installed (required for PDF generation)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Check cairo library files
if docker run --rm "$FULL_IMAGE_NAME" bash -c "ldconfig -p | grep -q libcairo" 2>/dev/null; then
    pass "Cairo libraries are accessible"
else
    log_warning "Cairo libraries may not be accessible"
fi

# Check pango library files
if docker run --rm "$FULL_IMAGE_NAME" bash -c "ldconfig -p | grep -q libpango" 2>/dev/null; then
    pass "Pango libraries are accessible"
else
    log_warning "Pango libraries may not be accessible"
fi

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Verify non-root user
section "Test 8: Verify Non-Root User Configuration"

log_info "Checking user configuration..."

# Check current user in container
CONTAINER_USER=$(docker run --rm "$FULL_IMAGE_NAME" whoami 2>&1)
if [[ "$CONTAINER_USER" == "mkdocs" ]]; then
    pass "Container runs as non-root user 'mkdocs'"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Container should run as user 'mkdocs', but runs as '$CONTAINER_USER'"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Check user ID
USER_ID=$(docker run --rm "$FULL_IMAGE_NAME" id -u 2>&1)
log_info "User ID: $USER_ID"
if [[ "$USER_ID" == "1000" ]]; then
    pass "User ID is 1000 (matches typical host user)"
else
    log_warning "User ID is $USER_ID (expected 1000 for host compatibility)"
fi

# Check user home directory
USER_HOME=$(docker run --rm "$FULL_IMAGE_NAME" bash -c "echo \$HOME" 2>&1)
log_info "User home directory: $USER_HOME"
if [[ "$USER_HOME" == "/home/mkdocs" ]]; then
    pass "User home directory is /home/mkdocs"
else
    log_warning "User home directory is $USER_HOME (expected /home/mkdocs)"
fi

# Check permissions on /docs directory
DOCS_OWNER=$(docker run --rm "$FULL_IMAGE_NAME" stat -c '%U' /docs 2>&1)
if [[ "$DOCS_OWNER" == "mkdocs" ]]; then
    pass "/docs directory is owned by mkdocs user"
else
    fail "/docs directory is owned by '$DOCS_OWNER' (should be 'mkdocs')"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Verify environment variables
section "Test 9: Verify Environment Variables"

log_info "Checking environment variables..."

# Check ENABLE_PDF_EXPORT default value
PDF_EXPORT_VAR=$(docker run --rm "$FULL_IMAGE_NAME" bash -c "echo \$ENABLE_PDF_EXPORT" 2>&1)
if [[ "$PDF_EXPORT_VAR" == "0" ]]; then
    pass "ENABLE_PDF_EXPORT defaults to 0"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "ENABLE_PDF_EXPORT should default to 0, but is '$PDF_EXPORT_VAR'"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test that variable can be overridden
PDF_EXPORT_OVERRIDE=$(docker run --rm -e ENABLE_PDF_EXPORT=1 "$FULL_IMAGE_NAME" bash -c "echo \$ENABLE_PDF_EXPORT" 2>&1)
if [[ "$PDF_EXPORT_OVERRIDE" == "1" ]]; then
    pass "ENABLE_PDF_EXPORT can be overridden (tested with value 1)"
else
    log_warning "ENABLE_PDF_EXPORT override may not work correctly"
fi

echo

# Docker inspect verification
section "Test 10: Docker Inspect Verification"

log_info "Running docker inspect for detailed configuration..."

# Get full inspect output
INSPECT_OUTPUT=$(docker inspect "$FULL_IMAGE_NAME")

# Check exposed ports
EXPOSED_PORTS=$(echo "$INSPECT_OUTPUT" | grep -A 5 '"ExposedPorts"' | grep -o '"[0-9]*/tcp"' | head -1)
if [[ "$EXPOSED_PORTS" == '"8000/tcp"' ]]; then
    pass "Port 8000 is exposed"
else
    log_warning "Port 8000 may not be properly exposed"
fi

# Check working directory
WORKDIR=$(echo "$INSPECT_OUTPUT" | grep '"WorkingDir"' | cut -d'"' -f4)
log_info "Working directory: $WORKDIR"
if [[ "$WORKDIR" == "/docs" ]]; then
    pass "Working directory is /docs"
else
    log_warning "Working directory is $WORKDIR (expected /docs)"
fi

# Check default command
CMD=$(echo "$INSPECT_OUTPUT" | grep -A 3 '"Cmd"' | tail -n 2 | tr -d ' \n"[],' | grep -o 'mkdocsbuild')
if [[ "$CMD" == "mkdocsbuild" ]]; then
    pass "Default command is 'mkdocs build'"
else
    log_info "Default command is: $CMD"
fi

# Check user configuration
USER_CONFIG=$(echo "$INSPECT_OUTPUT" | grep '"User"' | cut -d'"' -f4 | head -1)
log_info "User configuration: $USER_CONFIG"
if [[ "$USER_CONFIG" == "mkdocs" ]]; then
    pass "User is configured as 'mkdocs'"
else
    log_warning "User configuration is '$USER_CONFIG' (expected 'mkdocs')"
fi

# Check labels
log_info "Image labels:"
echo "$INSPECT_OUTPUT" | grep -A 10 '"Labels"' | grep '"' | head -5 || log_info "  No labels found"

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Test image functionality
section "Test 11: Test Image Functionality"

log_info "Testing basic image functionality..."

# Create temporary directory for build test
TEMP_DIR=$(mktemp -d)
setup_cleanup "$TEMP_DIR"

if [[ $REMOVE_TEMP -eq 0 ]]; then
    disable_cleanup
    info "Temporary files will not be removed: $TEMP_DIR"
fi

# Create minimal test documentation
mkdir -p "$TEMP_DIR/docs"
cat > "$TEMP_DIR/mkdocs.yml" << 'EOF'
site_name: Test Documentation
docs_dir: docs
site_dir: site
theme:
  name: material
nav:
  - Home: index.md
EOF

cat > "$TEMP_DIR/docs/index.md" << 'EOF'
# Test Documentation

This is a test page.
EOF

log_info "Testing documentation build..."
if docker run --rm \
    -v "$TEMP_DIR/docs:/docs/docs" \
    -v "$TEMP_DIR/mkdocs.yml:/docs/mkdocs.yml" \
    -v "$TEMP_DIR/site:/docs/site" \
    "$FULL_IMAGE_NAME" mkdocs build > /dev/null 2>&1; then
    pass "Documentation build works correctly"
    
    # Verify output
    if [[ -f "$TEMP_DIR/site/index.html" ]]; then
        pass "Generated index.html exists"
    else
        fail "Generated index.html not found"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    
    if grep -q "Test Documentation" "$TEMP_DIR/site/index.html"; then
        pass "Generated HTML contains expected content"
    else
        fail "Generated HTML missing expected content"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    fail "Documentation build failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

echo

# Display images for verification
section "Test 12: Display Image Information"

log_info "Running: docker images $FULL_IMAGE_NAME"
docker images "$FULL_IMAGE_NAME"

echo
log_info "Running: docker inspect $FULL_IMAGE_NAME (summary)"
docker inspect "$FULL_IMAGE_NAME" --format='
Image: {{.RepoTags}}
Size: {{.Size}} bytes
Architecture: {{.Architecture}}
OS: {{.Os}}
Created: {{.Created}}
User: {{.Config.User}}
WorkingDir: {{.Config.WorkingDir}}
Exposed Ports: {{.Config.ExposedPorts}}
Environment:
{{range .Config.Env}}  {{.}}
{{end}}
Labels:
{{range $key, $value := .Config.Labels}}  {{$key}}: {{$value}}
{{end}}'

TESTS_PASSED=$((TESTS_PASSED + 1))
echo

# Final summary
section "Test Summary"

TOTAL_TESTS=$((TESTS_PASSED + TESTS_FAILED))
log_info "Total tests: $TOTAL_TESTS"
log_info "Tests passed: $TESTS_PASSED"
log_info "Tests failed: $TESTS_FAILED"

if [[ $TESTS_FAILED -eq 0 ]]; then
    echo
    pass "All Docker MkDocs tests passed successfully!"
    echo
    log_info "Image $FULL_IMAGE_NAME is ready to use"
    log_info "You can now use the following commands:"
    log_info "  - make docs-docker-serve      # Start development server"
    log_info "  - make docs-docker-build-site # Build static site"
    log_info "  - make docs-docker-build-pdf  # Build with PDF"
    echo
    exit 0
else
    echo
    fail "Some Docker MkDocs tests failed!"
    echo
    log_error "$TESTS_FAILED test(s) failed"
    log_info "Please review the output above and fix the issues"
    echo
    exit 1
fi
