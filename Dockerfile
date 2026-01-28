# Stage 1: deps - Build dependencies only
FROM rust:1.92-bookworm as deps

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy src/main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    mkdir -p src/bin && \
    echo "fn main() {}" > src/bin/validate-yaml.rs && \
    mkdir -p src/bin/ && \
    echo "fn main() {}" > "src/bin/validate-yaml.rs" && \
    echo "fn main() {}" > "src/bin/validate-json.rs" && \
    echo "fn main() {}" > "src/bin/test-run-manager.rs" && \
    echo "fn main() {}" > "src/bin/test-verify.rs" && \
    echo "fn main() {}" > "src/bin/test-executor.rs" && \
    mkdir -p src/ && \
    echo "fn main() {}" > "src/main_editor.rs" && \
    echo "fn main() {}" > "src/bin/test-orchestrator.rs" && \
    echo "fn main() {}" > "src/bin/script-cleanup.rs" && \
    mkdir -p examples/ && \
    echo "fn main() {}" > "examples/tty_fallback_demo.rs" && \
    echo "fn main() {}" > "examples/test_verify_demo.rs" && \
    echo "fn main() {}" > "examples/test_verify_integration.rs" && \
    echo "fn main() {}" > "examples/junit_export_example.rs"

# Build dependencies (this will be cached)
RUN cargo build --release

# Stage 2: builder - Build the actual application
FROM rust:1.92-bookworm as builder

WORKDIR /app

# Copy dependencies from deps stage
COPY --from=deps /app/target target
COPY --from=deps /usr/local/cargo /usr/local/cargo

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy full source
COPY src ./src
COPY examples ./examples
COPY tests ./tests

# Build the application against cached dependencies
RUN cargo build --release

# Stage 3: runtime - Final lightweight image
FROM debian:bookworm-slim as runtime

# Install runtime dependencies: git, inotify-tools for watch mode, and make
RUN apt-get update && \
    apt-get install -y git inotify-tools make && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy only the compiled binaries (not auxiliary build files)
COPY --from=builder /app/target/release/testcase-manager /usr/local/bin/tcm
COPY --from=builder /app/target/release/validate-yaml /usr/local/bin/
COPY --from=builder /app/target/release/validate-json /usr/local/bin/
COPY --from=builder /app/target/release/trm /usr/local/bin/
COPY --from=builder /app/target/release/test-verify /usr/local/bin/
COPY --from=builder /app/target/release/test-executor /usr/local/bin/
COPY --from=builder /app/target/release/editor /usr/local/bin/
COPY --from=builder /app/target/release/test-orchestrator /usr/local/bin/

# Copy data directory
COPY data ./data


# Copy scripts directory for watch and validation functionality
COPY scripts ./scripts

# Copy Makefile for convenient commands
COPY Makefile ./Makefile

# Make scripts executable
RUN chmod +x scripts/*.sh && \
    find scripts -type f -name "*.sh" -exec chmod +x {} \;

# Create a helper script for easy watch mode usage
RUN cat > /usr/local/bin/watch-yaml << 'WATCHEOF'
#!/bin/bash
# Helper script to start watch mode easily
cd /app
exec ./scripts/watch-yaml-files.sh "$@"
WATCHEOF

RUN chmod +x /usr/local/bin/watch-yaml

# Create README guide at ~/README.md
RUN mkdir -p /root
COPY README_INSTALL.md /root

# Set default command
CMD ["tcm"]
