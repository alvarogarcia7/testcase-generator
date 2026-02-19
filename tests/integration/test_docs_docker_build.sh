#!/usr/bin/env bash
#
# test_docs_docker_build.sh - Test documentation building in Docker
#
# This script tests that documentation can be built successfully in a Docker environment:
# 1. Builds a Docker image with MkDocs and all dependencies
# 2. Builds the PDF version of the documentation inside Docker
# 3. Builds the HTML version of the documentation inside Docker
#
# All commands must exit with status code 0.
#
# Usage:
#   ./tests/integration/test_docs_docker_build.sh

set -euo pipefail

# Get the script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Source logger library
source "$PROJECT_ROOT/scripts/lib/logger.sh" || {
    echo "[ERROR] Failed to source logger library" >&2
    exit 1
}

# Configuration
IMAGE_NAME="testcase-manager-docs:test"
DOCKERFILE_PATH="$PROJECT_ROOT/Dockerfile"
BUILD_EXIT_CODE=0
PDF_EXIT_CODE=0
HTML_EXIT_CODE=0

# Cleanup function
cleanup() {
    log_info "Cleaning up Docker resources..."
    
    # Remove test image if it exists
    if docker image inspect "$IMAGE_NAME" >/dev/null 2>&1; then
        log_info "Removing test image: $IMAGE_NAME"
        docker rmi "$IMAGE_NAME" >/dev/null 2>&1 || true
        pass "Cleaned up test image"
    fi
}

# Register cleanup on exit
trap cleanup EXIT INT TERM

# Main test execution
main() {
    section "Docker Documentation Build Test"
    log_info "Project root: $PROJECT_ROOT"
    log_info "Docker image: $IMAGE_NAME"
    log_info "Dockerfile: $DOCKERFILE_PATH"
    echo
    
    # Test 1: Build Docker image
    section "Test 1: Build Docker image"
    
    log_info "Building Docker image: $IMAGE_NAME"
    if docker build -f "$DOCKERFILE_PATH" -t "$IMAGE_NAME" "$PROJECT_ROOT" 2>&1 | tee /tmp/docker-build.log; then
        BUILD_EXIT_CODE=0
        pass "Docker build succeeded (exit code: 0)"
    else
        BUILD_EXIT_CODE=$?
        fail "Docker build failed (exit code: $BUILD_EXIT_CODE)"
        echo
        log_error "Build output:"
        tail -20 /tmp/docker-build.log | sed 's/^/  /'
        exit 1
    fi
    echo
    
    # Test 2: Install MkDocs dependencies in Docker
    section "Test 2: Install MkDocs dependencies in Docker"
    
    log_info "Installing Python and MkDocs dependencies..."
    if docker run --rm "$IMAGE_NAME" bash -c "apt-get update && apt-get install -y python3 python3-pip python3-venv && python3 -m pip install --break-system-packages -r requirements.txt" 2>&1 | tee /tmp/docker-mkdocs-install.log | tail -10; then
        pass "MkDocs dependencies installed successfully (exit code: 0)"
    else
        INSTALL_EXIT_CODE=$?
        fail "MkDocs installation failed (exit code: $INSTALL_EXIT_CODE)"
        echo
        log_error "Installation output:"
        tail -20 /tmp/docker-mkdocs-install.log | sed 's/^/  /'
        exit 1
    fi
    echo
    
    # Test 3: Build PDF documentation
    section "Test 3: Build PDF documentation in Docker"
    
    log_info "Running: docker run --rm $IMAGE_NAME bash -c 'ENABLE_PDF_EXPORT=1 mkdocs build'"
    if docker run --rm "$IMAGE_NAME" bash -c "python3 -m pip install --break-system-packages -r requirements.txt > /dev/null && ENABLE_PDF_EXPORT=1 mkdocs build" 2>&1 | tee /tmp/docker-pdf-build.log | tail -10; then
        PDF_EXIT_CODE=0
        pass "PDF build succeeded (exit code: 0)"
    else
        PDF_EXIT_CODE=$?
        fail "PDF build failed (exit code: $PDF_EXIT_CODE)"
        echo
        log_error "PDF build output:"
        tail -20 /tmp/docker-pdf-build.log | sed 's/^/  /'
        exit 1
    fi
    echo
    
    # Test 4: Build HTML documentation
    section "Test 4: Build HTML documentation in Docker"
    
    log_info "Running: docker run --rm $IMAGE_NAME bash -c 'mkdocs build'"
    if docker run --rm "$IMAGE_NAME" bash -c "python3 -m pip install --break-system-packages -r requirements.txt > /dev/null && mkdocs build" 2>&1 | tee /tmp/docker-html-build.log | tail -10; then
        HTML_EXIT_CODE=0
        pass "HTML build succeeded (exit code: 0)"
    else
        HTML_EXIT_CODE=$?
        fail "HTML build failed (exit code: $HTML_EXIT_CODE)"
        echo
        log_error "HTML build output:"
        tail -20 /tmp/docker-html-build.log | sed 's/^/  /'
        exit 1
    fi
    echo
    
    # Verify all exit codes are 0
    section "Test Results Summary"
    
    log_info "Build exit codes:"
    log_info "  Docker build: $BUILD_EXIT_CODE"
    log_info "  PDF build:    $PDF_EXIT_CODE"
    log_info "  HTML build:   $HTML_EXIT_CODE"
    echo
    
    if [ "$BUILD_EXIT_CODE" -eq 0 ] && [ "$PDF_EXIT_CODE" -eq 0 ] && [ "$HTML_EXIT_CODE" -eq 0 ]; then
        pass "All tests passed - all exit codes are 0"
        echo
        log_info "Documentation can be built successfully in Docker"
        return 0
    else
        fail "Some tests failed - exit codes not all 0"
        exit 1
    fi
}

# Run main
main "$@"
