# sccache Configuration Guide

## Overview

This project supports [sccache](https://github.com/mozilla/sccache), a compiler cache that speeds up Rust compilation by caching compilation artifacts. sccache is **entirely optional** - the project builds normally without it.

## Quick Start

```bash
# 1. Install sccache
make install-sccache

# 2. Enable sccache for local builds
USE_SCCACHE=1 source scripts/setup-sccache-env.sh

# 3. Build as normal
cargo build
```

## Configuration

### Environment Variables

The sccache setup uses four environment variables:

1. **USE_SCCACHE**: Master control for sccache enablement
   - Set to `1` to enable sccache
   - Unset or set to `0` to disable (normal cargo compilation)

2. **DOCKER_BUILD**: Determines cache directory location
   - Set to `1` for Docker builds (uses `.sccache/docker`)
   - Unset or set to `0` for local builds (uses `.sccache/host`)

3. **RUSTC_WRAPPER**: Cargo's compiler wrapper (set automatically)
   - Automatically set to `sccache` when `USE_SCCACHE=1`
   - Can be set manually: `export RUSTC_WRAPPER=sccache`

4. **SCCACHE_DIR**: Cache storage location (set automatically)
   - Automatically set based on `DOCKER_BUILD` value
   - Can be set manually: `export SCCACHE_DIR=.sccache/host`

### How It Works

When enabled via the setup script:
1. `USE_SCCACHE=1` triggers the configuration
2. Script sets `RUSTC_WRAPPER=sccache` to tell cargo to use sccache
3. Script sets `SCCACHE_DIR` based on `DOCKER_BUILD` value
4. Cargo automatically invokes sccache for all rustc calls
5. sccache caches compilation results in the specified directory

The configuration is handled through environment variables because cargo's `.cargo/config.toml` format doesn't support runtime conditional evaluation based on environment variables.

## Usage

### Local Development

#### Enable sccache for a single session:
```bash
# Use the setup script (recommended)
USE_SCCACHE=1 source scripts/setup-sccache-env.sh
cargo build
```

#### Enable sccache manually:
```bash
# Set environment variables directly
export RUSTC_WRAPPER=sccache
export SCCACHE_DIR=.sccache/host
cargo build
```

#### Enable sccache permanently:
Add to your `~/.bashrc` or `~/.zshrc`:
```bash
# Option 1: Use setup script in your shell profile
if [ -f /path/to/project/scripts/setup-sccache-env.sh ]; then
    USE_SCCACHE=1 source /path/to/project/scripts/setup-sccache-env.sh
fi

# Option 2: Set variables directly
export RUSTC_WRAPPER=sccache
export SCCACHE_DIR=$HOME/.cache/sccache  # or any preferred location
```

### Docker Builds

#### Using the setup script:
```bash
# Set both USE_SCCACHE and DOCKER_BUILD
USE_SCCACHE=1 DOCKER_BUILD=1 source scripts/setup-sccache-env.sh
cargo build --release
```

#### In a Dockerfile:
```dockerfile
# Install sccache
RUN cargo install sccache

# Enable sccache for Docker builds
ENV USE_SCCACHE=1
ENV DOCKER_BUILD=1

# Source the setup script and build
RUN USE_SCCACHE=1 DOCKER_BUILD=1 source scripts/setup-sccache-env.sh && \
    cargo build --release
```

Or set variables directly:
```dockerfile
ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_DIR=/app/.sccache/docker
RUN cargo build --release
```

## Cache Management

### View Cache Statistics

```bash
make sccache-stats
# or
sccache --show-stats
```

### Clear Cache

```bash
make sccache-clean
# or
sccache --stop-server
```

### Cache Locations

- **Local builds**: `.sccache/host/` (relative to project root)
- **Docker builds**: `.sccache/docker/` (relative to project root)

Both directories are ignored by git (listed in `.gitignore`).

## Installation

sccache must be installed on your system to use this feature:

```bash
# Install via Makefile
make install-sccache

# Or install directly
cargo install sccache

# Or use system package manager
# macOS:
brew install sccache

# Linux:
# Download from https://github.com/mozilla/sccache/releases
```

## Performance Benefits

With a warm cache, sccache can significantly speed up builds:
- **First build**: Normal compilation speed (cache is being populated)
- **Subsequent builds**: 50-90% faster depending on what changed
- **Clean builds**: Much faster when cache is already populated

## Troubleshooting

### Build fails with sccache enabled

Disable sccache and try again:
```bash
unset USE_SCCACHE
cargo clean
cargo build
```

### Cache not being used

Check if sccache is running and accessible:
```bash
sccache --show-stats
```

If sccache is not found:
```bash
make install-sccache
```

### Different cache for Docker and local builds

This is intentional. Docker and local builds may use different Rust toolchains or compilation flags, so they use separate caches to avoid conflicts.

## CI/CD Integration

### GitHub Actions

```yaml
- name: Install sccache
  run: |
    cargo install sccache
    
- name: Build with sccache
  env:
    USE_SCCACHE: 1
    DOCKER_BUILD: 1
  run: cargo build --release
```

### GitLab CI

```yaml
build:
  variables:
    USE_SCCACHE: "1"
    DOCKER_BUILD: "1"
  before_script:
    - cargo install sccache
  script:
    - cargo build --release
```

## Technical Details

### Why Not Use .cargo/config.toml?

Cargo's configuration format doesn't support conditional evaluation of environment variables at runtime. We need to:
1. Conditionally set `RUSTC_WRAPPER` based on `USE_SCCACHE`
2. Conditionally set `SCCACHE_DIR` based on `DOCKER_BUILD`

Since cargo config can't do this, we use environment variables directly, which cargo reads at runtime.

### Alternative: Direct Environment Variables

Instead of using the setup script, you can export the variables directly:

```bash
# For local builds with sccache
export RUSTC_WRAPPER=sccache
export SCCACHE_DIR=.sccache/host

# For Docker builds with sccache
export RUSTC_WRAPPER=sccache
export SCCACHE_DIR=.sccache/docker

# To disable sccache
unset RUSTC_WRAPPER
unset SCCACHE_DIR
```

## References

- [sccache GitHub repository](https://github.com/mozilla/sccache)
- [Cargo documentation on build.rustc-wrapper](https://doc.rust-lang.org/cargo/reference/config.html#buildrustc-wrapper)
- [sccache usage documentation](https://github.com/mozilla/sccache/blob/main/docs/Usage.md)
