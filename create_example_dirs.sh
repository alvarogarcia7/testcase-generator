#!/bin/bash

# Source logger library
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/scripts/lib/logger.sh" || exit 1

mkdir -p example-data/run
mkdir -p example-data/run-all
mkdir -p example-data/verify
log_info "Directories created successfully"
