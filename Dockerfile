# Stage 1: builder - Build the actual application
FROM rust:1.92-bookworm AS builder

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
#RUN --mount=type=cache,target=/usr/local/cargo/registry \
#    --mount=type=cache,target=/app/target \
RUN \
    cargo build --all --locked --release --target-dir ./target

WORKDIR /app

# Copy source code and perform the final build
COPY src ./src
COPY examples ./examples
COPY tests ./tests

# Build the application against cached dependencies
# The previous RUN command will be reused if only Cargo.toml/Cargo.lock are unchanged
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --all --locked --release --target-dir ./target && \
for bin in $(ls -F /app/target/release | grep -E ".*\*" | cut -d"*" -f1); do \
      if [ -n "$bin" ]; then \
        cp "/app/target/release/$bin" /usr/local/bin/ && chmod +x "/usr/local/bin/$bin"; \
      fi; \
    done

# Install runtime dependencies: git, inotify-tools for watch mode, and make
RUN apt-get update && \
    apt-get install -y git inotify-tools make && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN \
ls -lah /usr/local/bin/testcase-manager > /dev/null && \
ls -lah /usr/local/bin/validate-yaml > /dev/null && \
ls -lah /usr/local/bin/validate-json > /dev/null && \
ls -lah /usr/local/bin/trm > /dev/null && \
ls -lah /usr/local/bin/test-verify > /dev/null && \
ls -lah /usr/local/bin/test-executor > /dev/null && \
ls -lah /usr/local/bin/editor > /dev/null && \
ls -lah /usr/local/bin/test-orchestrator > /dev/null



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
COPY README_INSTALL_AUTOMATED.md /app
COPY README_INSTALL.md /app
RUN cat README_INSTALL.md README_INSTALL_AUTOMATED.md >> README_INSTALL_2.md && mv README_INSTALL_2.md README_INSTALL.md && cp README_INSTALL.md /app/README.md && cp README_INSTALL.md /root/README.md

# Set default command
CMD ["tcm"]
