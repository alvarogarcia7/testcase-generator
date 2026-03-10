# Stage 1: builder - Build the actual application
FROM rust:1.92-bookworm AS builder

WORKDIR /app

# Install coverage tools for CI/CD
RUN rustup component add llvm-tools-preview && \
    cargo install cargo-llvm-cov

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy scripts directory early so include_str! macros can find the files
COPY scripts ./scripts

# Copy schemas directory early so tests can find schema files
COPY schemas ./schemas

# Create dummy src/main.rs to build dependencies
RUN mkdir -p src/bin examples && \
    echo "fn main() {}" > src/main.rs && \
    echo "fn main() {}" > src/main_editor.rs && \
    for bin in validate-yaml validate-json test-run-manager test-verify test-executor json-escape verifier test-orchestrator script-cleanup; do \
      echo "fn main() {}" > "src/bin/${bin}.rs"; \
    done && \
    for example in tty_fallback_demo test_verify_demo test_verify_integration junit_export_example; do \
      echo "fn main() {}" > "examples/${example}.rs"; \
    done

# Copy source code
COPY src ./src
COPY examples ./examples
COPY tests ./tests
COPY data ./data
COPY testcases ./testcases

WORKDIR /app


# Install runtime dependencies: git, inotify-tools for watch mode, and make
RUN apt-get update && \
    apt-get install -y \
      git \
      inotify-tools \
      expect \
      iputils-ping \
      shellcheck \
      make && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy Makefile for convenient commands
COPY Makefile ./Makefile

# Build release binaries first
RUN cargo build --all --all-features --release && \
    for bin in target/release/*; do \
      if [ -f "$bin" ] && [ -x "$bin" ]; then \
        cp "$bin" /usr/local/bin/ && chmod +x "/usr/local/bin/$(basename $bin)"; \
      fi; \
    done

# Verify binaries were installed correctly
RUN \
for bin in target/release/*; do \
  if [ -f "$bin" ] && [ -x "$bin" ]; then \
    bin_name="$(basename $bin)"; \
    ls -lah "/usr/local/bin/$bin_name" > /dev/null || (echo "Binary $bin_name not found in /usr/local/bin" && exit 1); \
  fi; \
done

# Verify shellcheck is installed
RUN shellcheck --version

# Run tests to ensure everything compiles and passes
# Run unit tests only (skip integration tests that require binaries in specific PATH)
RUN cargo test --lib --all-features && \
    cargo test --doc --all-features

# Make scripts executable
RUN chmod +x scripts/*.sh && \
    find scripts -type f -name "*.sh" -exec chmod +x {} \;

# Create a helper script for easy watch mode usage
#RUN cat > /usr/local/bin/watch-yaml << 'WATCHEOF'
##!/bin/bash
## Helper script to start watch mode easily
#cd /app
#exec ./scripts/watch-yaml-files.sh "$@"
#WATCHEOF

RUN chmod +x /usr/local/bin/watch-yaml

# Create README guide at ~/README.md
COPY README_INSTALL_AUTOMATED.md /app
COPY README_INSTALL.md /app
RUN cat README_INSTALL.md README_INSTALL_AUTOMATED.md >> README_INSTALL_2.md && mv README_INSTALL_2.md README_INSTALL.md && cp README_INSTALL.md /app/README.md && cp README_INSTALL.md /root/README.md

# Set default command
CMD ["tcm"]
