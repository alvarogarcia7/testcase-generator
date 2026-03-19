#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source logger library
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

section "Building Test Case Manager Docker image"

docker build -t testcase-manager:latest .

pass "Build complete!"
info ""
info "To run the container:"
info "  docker run -it --rm testcase-manager:latest"
info ""
