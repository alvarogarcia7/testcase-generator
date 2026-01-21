# Script Library

This directory contains shared shell script libraries used by multiple scripts in the project.

## Available Libraries

### find-binary.sh

Provides functions to locate Rust binaries in the target directory (release or debug builds) with fallback to system PATH.

**Functions:**

- `find_binary <binary-name> [env-var-name]`
  - Finds a binary by checking environment variable (if provided), target/release, target/debug, and system PATH
  - Returns the path to the binary or empty string if not found
  
- `find_binary_or_exit <binary-name> [env-var-name]`
  - Same as `find_binary` but exits with an error message if not found
  
- `ensure_binary_built <binary-name>`
  - Checks if binary exists, builds it with cargo if missing
  - Returns 0 if binary exists or was built successfully, 1 otherwise

**Usage Example:**

```bash
#!/usr/bin/env bash
set -euo pipefail

# Source the library
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib/find-binary.sh"

# Find binary or exit if not found
VALIDATE_YAML=$(find_binary_or_exit "validate-yaml" "VALIDATE_YAML_BIN")

# Or ensure binary is built
ensure_binary_built "validate-yaml" || exit 1

# Use the binary
"$VALIDATE_YAML" file.yaml schema.json
```

## Scripts Using These Libraries

- `scripts/validate-yaml-wrapper.sh` - Uses `find_binary_or_exit()` to locate validate-yaml binary
- `scripts/watch-yaml-files.sh` - Uses `ensure_binary_built()` to ensure binary exists before watching

## Adding New Libraries

When creating new shared libraries:

1. Create the library file in `scripts/lib/`
2. Add clear documentation at the top of the file
3. Update this README with the new library description
4. Follow the naming convention: `<purpose>-<description>.sh`
5. Make functions reusable and focused on a single responsibility
