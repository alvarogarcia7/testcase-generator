#!/usr/bin/env bash
#
# build-dev-docker.sh - Build and verify the development Docker image
#
# This script builds the testcase-manager:dev image on top of the base
# testcase-manager:latest image, then verifies the build.
#

set -e

# Get script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source logger library
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

cd "$PROJECT_ROOT"

section "Building Development Docker Image"

# Step 1: Check if base image exists
log_info "Checking for base image: testcase-manager:latest"
if ! docker image inspect testcase-manager:latest > /dev/null 2>&1; then
    log_warning "Base image testcase-manager:latest not found"
    log_info "Building base image first..."
    
    if ! docker build -t testcase-manager:latest .; then
        log_error "Failed to build base image"
        exit 1
    fi
    
    pass "Base image built successfully"
else
    pass "Base image found"
fi

# Step 2: Build development image
log_info "Building development image: testcase-manager:dev"
if ! docker build -f Dockerfile.dev -t testcase-manager:dev .; then
    log_error "Failed to build development image"
    exit 1
fi

pass "Development image built successfully"

# Step 3: Verify the development image
section "Verifying Development Image"

log_info "Checking image existence..."
if ! docker image inspect testcase-manager:dev > /dev/null 2>&1; then
    fail "Development image not found"
    exit 1
fi
pass "Image exists"

# Check image size
IMAGE_SIZE=$(docker image inspect testcase-manager:dev --format='{{.Size}}' | awk '{printf "%.2f MB", $1/1024/1024}')
log_info "Image size: $IMAGE_SIZE"

# Step 4: Verify development tools are installed
section "Verifying Development Tools"

log_info "Checking for development tools in the image..."

TOOLS_TO_CHECK=(
    "vim"
    "curl"
    "wget"
    "htop"
    "strace"
    "gdb"
    "tmux"
    "tree"
    "jq"
    "inotifywait"
    "make"
)

MISSING_TOOLS=0
for tool in "${TOOLS_TO_CHECK[@]}"; do
    if docker run --rm testcase-manager:dev which "$tool" > /dev/null 2>&1; then
        pass "$tool found"
    else
        fail "$tool not found"
        MISSING_TOOLS=$((MISSING_TOOLS + 1))
    fi
done

if [ $MISSING_TOOLS -gt 0 ]; then
    log_error "$MISSING_TOOLS development tool(s) missing"
    exit 1
fi

# Step 5: Verify development scripts
section "Verifying Development Scripts"

DEV_SCRIPTS=(
    "dev-setup"
    "quick-test"
    "dev-status"
)

MISSING_SCRIPTS=0
for script in "${DEV_SCRIPTS[@]}"; do
    if docker run --rm testcase-manager:dev which "$script" > /dev/null 2>&1; then
        pass "$script found"
    else
        fail "$script not found"
        MISSING_SCRIPTS=$((MISSING_SCRIPTS + 1))
    fi
done

if [ $MISSING_SCRIPTS -gt 0 ]; then
    log_error "$MISSING_SCRIPTS development script(s) missing"
    exit 1
fi

# Step 6: Verify development directories
section "Verifying Development Directories"

DEV_DIRS=(
    "/app/dev-workspace"
    "/app/logs"
    "/app/tmp"
)

MISSING_DIRS=0
for dir in "${DEV_DIRS[@]}"; do
    if docker run --rm testcase-manager:dev test -d "$dir" 2>/dev/null; then
        pass "$dir exists"
    else
        fail "$dir not found"
        MISSING_DIRS=$((MISSING_DIRS + 1))
    fi
done

if [ $MISSING_DIRS -gt 0 ]; then
    log_error "$MISSING_DIRS development directory(ies) missing"
    exit 1
fi

# Step 7: Verify configuration files
section "Verifying Configuration Files"

CONFIG_FILES=(
    "/root/.bashrc"
    "/root/.vimrc"
    "/app/.dev-config"
)

MISSING_CONFIGS=0
for config in "${CONFIG_FILES[@]}"; do
    if docker run --rm testcase-manager:dev test -f "$config" 2>/dev/null; then
        pass "$config exists"
    else
        fail "$config not found"
        MISSING_CONFIGS=$((MISSING_CONFIGS + 1))
    fi
done

if [ $MISSING_CONFIGS -gt 0 ]; then
    log_error "$MISSING_CONFIGS configuration file(s) missing"
    exit 1
fi

# Step 8: Run quick test script
section "Running Quick Test"

log_info "Executing quick-test script in container..."
if docker run --rm testcase-manager:dev quick-test; then
    pass "Quick test passed"
else
    fail "Quick test failed"
    exit 1
fi

# Step 9: Verify base binaries still work
section "Verifying Base Binaries"

BASE_BINARIES=(
    "tcm"
    "test-executor"
    "test-orchestrator"
    "validate-yaml"
)

MISSING_BINARIES=0
for binary in "${BASE_BINARIES[@]}"; do
    if docker run --rm testcase-manager:dev which "$binary" > /dev/null 2>&1; then
        pass "$binary found"
    else
        fail "$binary not found"
        MISSING_BINARIES=$((MISSING_BINARIES + 1))
    fi
done

if [ $MISSING_BINARIES -gt 0 ]; then
    log_error "$MISSING_BINARIES base binary(ies) missing"
    exit 1
fi

# Success!
section "Build Verification Complete"

log_info "Development image: testcase-manager:dev"
log_info "Size: $IMAGE_SIZE"
log_info ""
log_info "To run the development container:"
log_info "  docker run -it --rm testcase-manager:dev"
log_info ""
log_info "To run with volume mount:"
log_info "  docker run -it --rm -v \$(pwd):/app testcase-manager:dev"
log_info ""
log_info "To run dev-setup:"
log_info "  docker run -it --rm testcase-manager:dev dev-setup"

pass "All verifications passed!"

exit 0
