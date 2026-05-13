#!/usr/bin/env bash
#
# log-build-results.sh - Log Rust build/test results to projector database
#
# Usage:
#   ./scripts/log-build-results.sh [WORKTREE] [MODE]
#
# Arguments:
#   WORKTREE - Git worktree name (default: auto-detect from branch)
#   MODE     - Build mode: 'build', 'test', 'lint', 'release', 'all' (default: 'all')
#
# Examples:
#   ./scripts/log-build-results.sh                    # Auto-detect and run all checks
#   ./scripts/log-build-results.sh main build         # Run only build check on main branch
#   ./scripts/log-build-results.sh f728-in-the-test-acce test  # Run only test on feature branch
#
# Environment Variables:
#   PROJECTOR_BIN - Path to projector binary (default: 'proj' or via uv)
#   PROJECT_NAME  - Projector project name (default: 'testcase-generator')
#

set -euo pipefail

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Configuration
PROJECT_NAME="${PROJECT_NAME:-testcase-generator}"
WORKTREE="${1:-}"
BUILD_MODE="${2:-all}"

# Determine which projector command to use
if command -v proj >/dev/null 2>&1; then
    PROJ_CMD="proj"
elif command -v uv >/dev/null 2>&1; then
    # Try to find projector via uv
    PROJ_CMD="uv run --project ~/repos/projector proj"
else
    echo "Error: Neither 'proj' nor 'uv' found in PATH"
    exit 1
fi

# Auto-detect worktree from current branch if not provided
if [ -z "$WORKTREE" ]; then
    WORKTREE=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")
    if [ "$WORKTREE" = "HEAD" ]; then
        WORKTREE=$(git symbolic-ref --short HEAD 2>/dev/null || echo "unknown")
    fi
fi

# Colors for output
COLOR_RESET='\033[0m'
COLOR_BLUE='\033[34m'
COLOR_GREEN='\033[32m'
COLOR_RED='\033[31m'
COLOR_YELLOW='\033[33m'

log_info() {
    echo -e "${COLOR_BLUE}[INFO]${COLOR_RESET} $1"
}

log_success() {
    echo -e "${COLOR_GREEN}[✓]${COLOR_RESET} $1"
}

log_error() {
    echo -e "${COLOR_RED}[✗]${COLOR_RESET} $1"
}

log_info "Logging build results to projector"
log_info "Project: $PROJECT_NAME"
log_info "Worktree: $WORKTREE"
log_info "Mode: $BUILD_MODE"
echo ""

# Build check results
declare -A results

# Run build check
if [ "$BUILD_MODE" = "build" ] || [ "$BUILD_MODE" = "all" ]; then
    log_info "Running: cargo build --workspace"
    if cargo build --workspace >/dev/null 2>&1; then
        results["build"]="pass"
        log_success "Build passed"
    else
        results["build"]="fail"
        log_error "Build failed"
    fi
fi

# Run test check
if [ "$BUILD_MODE" = "test" ] || [ "$BUILD_MODE" = "all" ]; then
    log_info "Running: cargo test --workspace"
    if cargo test --workspace --all-features >/dev/null 2>&1; then
        results["tests"]="pass"
        log_success "Tests passed"
    else
        results["tests"]="fail"
        log_error "Tests failed"
    fi
fi

# Run lint check
if [ "$BUILD_MODE" = "lint" ] || [ "$BUILD_MODE" = "all" ]; then
    log_info "Running: cargo clippy"
    if cargo clippy --workspace --all-targets --all-features -- -D warnings >/dev/null 2>&1; then
        results["lint"]="pass"
        log_success "Lint check passed"
    else
        results["lint"]="warn"
        log_error "Lint warnings detected"
    fi
fi

# Run release build check
if [ "$BUILD_MODE" = "release" ] || [ "$BUILD_MODE" = "all" ]; then
    log_info "Running: cargo build --workspace --release"
    if cargo build --workspace --release >/dev/null 2>&1; then
        results["release-build"]="pass"
        log_success "Release build passed"
    else
        results["release-build"]="fail"
        log_error "Release build failed"
    fi
fi

# Log results to projector
echo ""
log_info "Logging results to projector database..."

# Build projector CI flags
CI_FLAGS=""
for check in "${!results[@]}"; do
    CI_FLAGS="$CI_FLAGS --ci $check=${results[$check]}"
done

# Log to projector
if $PROJ_CMD log "$PROJECT_NAME" "$WORKTREE" \
    --sha "$(git rev-parse HEAD)" \
    --message "$(git log -1 --pretty=%B)" \
    --author "$(git log -1 --pretty=%an)" \
    $CI_FLAGS 2>/dev/null; then
    log_success "Results logged to projector"
else
    log_error "Failed to log results to projector"
    exit 1
fi

echo ""
log_info "Build result summary for $WORKTREE:"
echo ""

# Show current status
$PROJ_CMD status "$PROJECT_NAME" "$WORKTREE" 2>/dev/null || true

exit 0
