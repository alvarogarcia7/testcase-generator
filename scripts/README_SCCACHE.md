# sccache Setup Script

## Overview

The `setup-sccache-env.sh` script configures environment variables for using sccache (Shared Compiler Cache) with Rust/cargo builds. This script enables conditional sccache configuration based on build type (local vs Docker).

## Purpose

sccache accelerates Rust compilation by caching compilation results. This script:
- Enables/disables sccache based on `USE_SCCACHE` environment variable
- Sets appropriate cache directory based on `DOCKER_BUILD` environment variable
- Provides separate caches for local and Docker builds to prevent conflicts

## Usage

### Source the Script

**Important**: This script must be **sourced** (not executed) to set environment variables in your current shell:

```bash
source scripts/setup-sccache-env.sh
```

### Enable sccache for Local Builds

```bash
USE_SCCACHE=1 source scripts/setup-sccache-env.sh
cargo build
```

Output:
```
sccache enabled: RUSTC_WRAPPER=sccache, SCCACHE_DIR=.sccache/host
```

### Enable sccache for Docker Builds

```bash
USE_SCCACHE=1 DOCKER_BUILD=1 source scripts/setup-sccache-env.sh
cargo build --release
```

Output:
```
sccache enabled: RUSTC_WRAPPER=sccache, SCCACHE_DIR=.sccache/docker
```

### Disable sccache

```bash
source scripts/setup-sccache-env.sh
```

Output:
```
sccache disabled
```

## Environment Variables

### Input Variables

- **USE_SCCACHE**: Control whether sccache is enabled
  - `1` = Enable sccache
  - `0` or unset = Disable sccache (default)

- **DOCKER_BUILD**: Specify build environment
  - `1` = Docker build (uses `.sccache/docker`)
  - `0` or unset = Local build (uses `.sccache/host`, default)

### Output Variables

When `USE_SCCACHE=1`, the script sets:

- **RUSTC_WRAPPER**: Set to `sccache` (tells cargo to use sccache)
- **SCCACHE_DIR**: Cache directory path
  - Local builds: `.sccache/host`
  - Docker builds: `.sccache/docker`

When `USE_SCCACHE=0` or unset, the script unsets:
- `RUSTC_WRAPPER`
- `SCCACHE_DIR`

## Cache Directories

The script uses separate cache directories to prevent conflicts:

- **`.sccache/host/`**: Local development builds
  - Used when building on your development machine
  - May use different Rust toolchains or compilation flags

- **`.sccache/docker/`**: Docker container builds  
  - Used when building inside Docker containers
  - Separate to avoid toolchain/environment conflicts

Both directories are gitignored (`.gitignore` includes `.sccache/`).

## Integration with Cargo

Once environment variables are set, cargo automatically uses sccache:

```bash
# Setup sccache
USE_SCCACHE=1 source scripts/setup-sccache-env.sh

# All cargo commands now use sccache
cargo build
cargo test
cargo clippy
cargo build --release
```

No changes to cargo commands required - sccache works transparently.

## Examples

### Example 1: Single Build with sccache

```bash
$ USE_SCCACHE=1 source scripts/setup-sccache-env.sh
sccache enabled: RUSTC_WRAPPER=sccache, SCCACHE_DIR=.sccache/host

$ cargo build
   Compiling testcase-generator v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 45.2s

$ make sccache-stats
Compile requests                    234
Cache hits                           12
Cache misses                        222
```

### Example 2: Development Session

```bash
# Enable sccache once
$ USE_SCCACHE=1 source scripts/setup-sccache-env.sh
sccache enabled: RUSTC_WRAPPER=sccache, SCCACHE_DIR=.sccache/host

# First build (populates cache)
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 45.2s

# Make a small change
$ echo "// comment" >> src/main.rs

# Second build (uses cache)
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 3.1s

# Check cache statistics
$ make sccache-stats
Cache hit rate: 85%
```

### Example 3: Docker Build

```bash
# In Dockerfile
RUN USE_SCCACHE=1 DOCKER_BUILD=1 source scripts/setup-sccache-env.sh && \
    cargo build --release
```

### Example 4: Disable sccache

```bash
$ source scripts/setup-sccache-env.sh
sccache disabled

$ cargo build
# Uses normal rustc (no caching)
```

## Troubleshooting

### Script has no effect

**Problem**: Environment variables not set after running script.

**Solution**: Use `source` or `.` instead of executing the script:
```bash
# Wrong
./scripts/setup-sccache-env.sh

# Correct
source scripts/setup-sccache-env.sh
```

### sccache not found

**Problem**: `sccache` binary not installed.

**Solution**: Install sccache first:
```bash
make install-sccache
# or
cargo install sccache
```

### Wrong cache directory used

**Problem**: Script uses wrong cache directory.

**Solution**: Check `DOCKER_BUILD` variable:
```bash
# For local builds (uses .sccache/host)
USE_SCCACHE=1 source scripts/setup-sccache-env.sh

# For Docker builds (uses .sccache/docker)
USE_SCCACHE=1 DOCKER_BUILD=1 source scripts/setup-sccache-env.sh
```

### Cache not working

**Problem**: No cache hits despite using sccache.

**Solution**: Verify environment variables:
```bash
$ echo $RUSTC_WRAPPER
sccache

$ echo $SCCACHE_DIR
.sccache/host

$ make sccache-stats
```

## Related Documentation

- **`docs/SCCACHE.md`**: Comprehensive sccache documentation
- **`docs/SCCACHE_SETUP.md`**: Detailed setup and configuration guide  
- **`.cargo/config.toml`**: Cargo configuration with sccache documentation
- **`AGENTS.md`**: Build commands including `make install-sccache`

## See Also

- `make install-sccache` - Install sccache binary
- `make sccache-stats` - View cache statistics
- `make sccache-clean` - Stop sccache server and reset stats
