#!/usr/bin/env bash
#
# docker-dev-quick-start.sh - Quick start for Docker development environment
#
# This script provides an interactive quick start for the development Docker
# environment, helping developers get up and running quickly.
#

set -e

# Get script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source logger library
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

cd "$PROJECT_ROOT"

# Print header
section "Docker Development Environment - Quick Start"

log_info "This script will help you set up and run the development Docker environment."
echo ""

# Step 1: Check if Docker is installed
log_info "Step 1: Checking Docker installation..."
if ! command -v docker > /dev/null 2>&1; then
    fail "Docker is not installed"
    log_error "Please install Docker from https://docs.docker.com/get-docker/"
    exit 1
fi
pass "Docker is installed"

# Check if Docker daemon is running
if ! docker info > /dev/null 2>&1; then
    fail "Docker daemon is not running"
    log_error "Please start Docker and try again"
    exit 1
fi
pass "Docker daemon is running"

echo ""

# Step 2: Check if base image exists
log_info "Step 2: Checking for base image..."
if docker image inspect testcase-manager:latest > /dev/null 2>&1; then
    pass "Base image (testcase-manager:latest) exists"
    BASE_EXISTS=1
else
    log_warning "Base image (testcase-manager:latest) not found"
    BASE_EXISTS=0
fi

echo ""

# Step 3: Check if dev image exists
log_info "Step 3: Checking for development image..."
if docker image inspect testcase-manager:dev > /dev/null 2>&1; then
    pass "Development image (testcase-manager:dev) exists"
    DEV_EXISTS=1
else
    log_warning "Development image (testcase-manager:dev) not found"
    DEV_EXISTS=0
fi

echo ""

# Step 4: Build images if needed
if [ $DEV_EXISTS -eq 0 ]; then
    section "Building Docker Images"
    
    log_info "The development image needs to be built."
    log_info "This will take a few minutes on the first run."
    echo ""
    
    read -p "Do you want to build the development image now? (y/n) " -n 1 -r
    echo ""
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        log_info "Building images..."
        if ./scripts/build-dev-docker.sh; then
            pass "Images built successfully!"
        else
            fail "Failed to build images"
            exit 1
        fi
    else
        log_info "Skipping build. You can build later with:"
        log_info "  make docker-build-dev"
        log_info "  OR"
        log_info "  ./scripts/build-dev-docker.sh"
        exit 0
    fi
    
    echo ""
fi

# Step 5: Show usage options
section "Usage Options"

log_info "The development environment is ready! Here are some ways to use it:"
echo ""

info "Option 1: Interactive Development Session"
echo "  docker run -it --rm -v \$(pwd):/app testcase-manager:dev"
echo ""

info "Option 2: Using Make"
echo "  make docker-run-dev"
echo ""

info "Option 3: Run Specific Command"
echo "  docker run --rm -v \$(pwd):/app testcase-manager:dev <command>"
echo ""

info "Option 4: Run Tests in Container"
echo "  docker run --rm -v \$(pwd):/app testcase-manager:dev run-tests"
echo ""

# Step 6: Ask if user wants to start interactive session
echo ""
read -p "Do you want to start an interactive development session now? (y/n) " -n 1 -r
echo ""
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]; then
    section "Starting Interactive Session"
    
    log_info "Starting development container..."
    log_info "Type 'show-help' inside the container for available commands"
    log_info "Type 'exit' to leave the container"
    echo ""
    
    sleep 1
    
    # Start interactive session with volume mount
    docker run -it --rm -v "$(pwd)":/app testcase-manager:dev
    
    echo ""
    section "Session Ended"
    log_info "You exited the development container"
else
    log_info "You can start a session anytime with:"
    log_info "  make docker-run-dev"
    log_info "  OR"
    log_info "  docker run -it --rm -v \$(pwd):/app testcase-manager:dev"
fi

echo ""
section "Quick Reference"

log_info "Commands:"
echo "  make docker-build-dev      - Build/rebuild development image"
echo "  make docker-run-dev        - Start interactive development session"
echo "  ./scripts/build-dev-docker.sh - Build with verification"
echo ""

log_info "Inside the container, you can use:"
echo "  show-help                  - Show all available commands"
echo "  show-binaries              - List all available binaries"
echo "  validate-file <file>       - Validate a YAML test case"
echo "  generate-script <file>     - Generate script from test case"
echo "  watch-yaml                 - Start watch mode"
echo "  run-tests                  - Run full test suite"
echo "  dev-status                 - Show environment status"
echo ""

log_info "Documentation:"
echo "  cat DOCKER_DEV_SETUP.md    - Complete development Docker guide"
echo "  cat DOCKER_BUILD_INSTRUCTIONS.md - Base Docker guide"
echo "  cat AGENTS.md              - Development guide"
echo ""

pass "Quick start complete!"

exit 0
