# sccache Configuration Implementation

## Summary

Implemented conditional sccache configuration for the testcase-generator project with separate cache directories for local and Docker builds.

## Implementation Details

### Files Created/Modified

1. **`.cargo/config.toml`** (Modified)
   - Added comprehensive sccache configuration documentation
   - Explains that sccache is controlled via environment variables
   - Documents usage patterns and cache locations
   - References supporting documentation

2. **`scripts/setup-sccache-env.sh`** (Created)
   - Shell script to configure sccache environment variables
   - Conditionally enables sccache based on `USE_SCCACHE` variable
   - Sets `SCCACHE_DIR` based on `DOCKER_BUILD` variable
   - Made executable with proper permissions

3. **`scripts/README_SCCACHE.md`** (Created)
   - Comprehensive documentation for the setup script
   - Usage examples for local and Docker builds
   - Troubleshooting guide
   - Environment variable reference

4. **`docs/SCCACHE_SETUP.md`** (Created/Updated)
   - Quick start guide for sccache
   - Configuration reference
   - Usage examples
   - Integration instructions

## Configuration Approach

### Environment Variables

The implementation uses environment variables instead of `.cargo/config.toml` static configuration because cargo's config format doesn't support runtime conditional evaluation.

**Control Variables:**
- `USE_SCCACHE`: Enable/disable sccache (1=enable, 0=disable)
- `DOCKER_BUILD`: Specify build type (1=Docker, 0=local)

**Cargo Variables (set by script):**
- `RUSTC_WRAPPER`: Set to "sccache" to enable compiler caching
- `SCCACHE_DIR`: Cache directory location

### Cache Directory Structure

```
.sccache/
├── host/      # Local development builds
└── docker/    # Docker container builds
```

Separate cache directories prevent conflicts between different build environments that may use different Rust toolchains or compilation flags.

## Usage

### Local Builds

```bash
# Enable sccache for local builds
USE_SCCACHE=1 source scripts/setup-sccache-env.sh
cargo build
```

**Result:**
- `RUSTC_WRAPPER=sccache`
- `SCCACHE_DIR=.sccache/host`

### Docker Builds

```bash
# Enable sccache for Docker builds
USE_SCCACHE=1 DOCKER_BUILD=1 source scripts/setup-sccache-env.sh
cargo build --release
```

**Result:**
- `RUSTC_WRAPPER=sccache`
- `SCCACHE_DIR=.sccache/docker`

### Manual Configuration

Users can also set environment variables directly without using the script:

```bash
# For local builds
export RUSTC_WRAPPER=sccache
export SCCACHE_DIR=.sccache/host
cargo build

# For Docker builds
export RUSTC_WRAPPER=sccache
export SCCACHE_DIR=.sccache/docker
cargo build
```

## Integration Points

### Makefile

Existing Makefile targets work seamlessly with sccache:
- `make build` - Uses sccache if enabled
- `make test` - Uses sccache if enabled
- `make sccache-stats` - View cache statistics
- `make sccache-clean` - Stop sccache server
- `make install-sccache` - Install sccache binary

### Gitignore

The `.sccache/` directory is already included in `.gitignore`, so cache files won't be committed to the repository.

### Docker Integration

For Docker builds, add environment variables to the Dockerfile:

```dockerfile
# Option 1: Use setup script
RUN USE_SCCACHE=1 DOCKER_BUILD=1 source scripts/setup-sccache-env.sh && \
    cargo build --release

# Option 2: Set variables directly
ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_DIR=/app/.sccache/docker
RUN cargo build --release
```

## Benefits

1. **Optional**: sccache is entirely optional - project builds normally without it
2. **Conditional**: Easy to enable/disable via environment variable
3. **Isolated**: Separate caches prevent conflicts between build types
4. **Transparent**: Works with all existing cargo commands
5. **Documented**: Comprehensive documentation in multiple places
6. **Flexible**: Can be enabled per-session, per-build, or permanently

## Design Rationale

### Why Environment Variables?

Cargo's `.cargo/config.toml` format doesn't support conditional evaluation based on environment variables. We need to:
1. Conditionally set `RUSTC_WRAPPER` based on `USE_SCCACHE`
2. Conditionally set `SCCACHE_DIR` based on `DOCKER_BUILD`

The only way to achieve this is through environment variables that cargo reads at runtime.

### Why Separate Cache Directories?

Docker and local builds may have:
- Different Rust toolchains (different versions)
- Different compilation flags
- Different target architectures
- Different system libraries

Using separate caches prevents:
- Cache corruption from incompatible artifacts
- False cache hits from different build environments
- Confusion when debugging build issues

### Why a Setup Script?

The setup script provides:
- Single source of truth for configuration logic
- Consistent behavior across all users
- Easy-to-use interface via simple variables
- Validation and error messages
- Documentation through code

## Documentation Structure

```
Documentation Hierarchy:
├── .cargo/config.toml          # In-file reference for cargo users
├── scripts/setup-sccache-env.sh # Script implementation
├── scripts/README_SCCACHE.md   # Script-specific documentation
├── docs/SCCACHE_SETUP.md       # Quick start and setup guide
├── docs/SCCACHE.md            # Comprehensive sccache guide (existing)
└── AGENTS.md                   # Build commands reference (existing)
```

Each document serves a specific purpose:
- **config.toml**: Quick reference for developers working with cargo
- **setup-sccache-env.sh**: Implementation of the configuration logic
- **README_SCCACHE.md**: How to use the setup script
- **SCCACHE_SETUP.md**: Setup guide focused on this implementation
- **SCCACHE.md**: Complete sccache documentation
- **AGENTS.md**: Build command reference

## Testing

To test the implementation:

```bash
# Test local build with sccache
USE_SCCACHE=1 source scripts/setup-sccache-env.sh
echo $RUSTC_WRAPPER  # Should output: sccache
echo $SCCACHE_DIR    # Should output: .sccache/host

# Test Docker build with sccache
USE_SCCACHE=1 DOCKER_BUILD=1 source scripts/setup-sccache-env.sh
echo $RUSTC_WRAPPER  # Should output: sccache
echo $SCCACHE_DIR    # Should output: .sccache/docker

# Test disable sccache
source scripts/setup-sccache-env.sh
echo $RUSTC_WRAPPER  # Should output: (empty)
echo $SCCACHE_DIR    # Should output: (empty)
```

## Future Enhancements

Possible future improvements:
1. Add sccache to CI/CD pipelines (GitLab CI)
2. Implement distributed cache (Redis/S3) for team sharing
3. Add cache statistics tracking/reporting
4. Create pre-commit hooks to verify cache health
5. Add cache size monitoring and alerts

## Related Issues

This implementation addresses the requirement to:
- Configure sccache conditionally based on environment
- Support separate caches for local and Docker builds
- Maintain backward compatibility (optional feature)
- Provide clear documentation and usage examples

## Conclusion

The implementation provides a robust, flexible, and well-documented approach to enabling sccache for the testcase-generator project. It supports both local and Docker builds with separate caches, is entirely optional, and integrates seamlessly with existing build workflows.
