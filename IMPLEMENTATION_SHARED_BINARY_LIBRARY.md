# Shared Binary Finding Library Implementation

## Summary

Extracted the function to find binaries in the target directory from `scripts/watch-yaml-files.sh` into a shared library file `scripts/lib/find-binary.sh`. Updated all scripts that search for binaries to use this shared library for consistency and maintainability.

## Files Created

### 1. `scripts/lib/find-binary.sh`
Shared library providing three functions for locating Rust binaries:

**Functions:**
- `find_binary <binary-name> [env-var-name]`
  - Searches for binary in: environment variable → target/release → target/debug → system PATH
  - Returns path to binary or empty string if not found
  
- `find_binary_or_exit <binary-name> [env-var-name]`
  - Same as find_binary but exits with error message if not found
  - Useful for scripts that require the binary to proceed
  
- `ensure_binary_built <binary-name>`
  - Checks if binary exists, automatically builds it with cargo if missing
  - Returns 0 on success, 1 on failure

### 2. `scripts/lib/README.md`
Documentation for the script library directory explaining:
- Available libraries and their functions
- Usage examples
- Scripts using the libraries
- Guidelines for adding new libraries

## Files Modified

### Scripts Directory

#### `scripts/watch-yaml-files.sh`
- **Before:** Inline code checking target/release and target/debug
- **After:** Sources `find-binary.sh` and uses `ensure_binary_built()` function
- **Lines changed:** Reduced from 5 lines of binary checking to 1 function call

#### `scripts/validate-yaml-wrapper.sh`
- **Before:** Inline code checking VALIDATE_YAML_BIN env var, target/release, target/debug, and PATH
- **After:** Sources `find-binary.sh` and uses `find_binary_or_exit()` function
- **Lines changed:** Reduced from 16 lines to 5 lines

### Tests Directory

#### `tests/integration/smoke_test.sh`
- **Before:** Hardcoded path to `target/debug/testcase-manager`
- **After:** Uses `find_binary()` to check both release and debug builds
- **Benefit:** Works with both debug and release builds

#### `tests/integration/check_environment.sh`
- **Before:** Checked both target/debug and target/release with separate if statements
- **After:** Uses `find_binary()` function
- **Benefit:** Cleaner code, consistent with other scripts

#### `tests/integration/run_e2e_test.sh`
- **Before:** Hardcoded path to `target/debug/testcase-manager`
- **After:** Uses `find_binary()` to locate binary
- **Benefit:** More flexible, works with release builds

#### `tests/integration/run_all_tests.sh`
- **Before:** Hardcoded path to `target/debug/testcase-manager`
- **After:** Uses `find_binary()` function
- **Benefit:** Consistent binary finding across all test scripts

#### `tests/integration/ci_test.sh`
- **Before:** Hardcoded path to `target/debug/testcase-manager`
- **After:** Uses `find_binary()` function
- **Benefit:** CI can now use release builds if available

## Benefits

### 1. Code Reusability
- Common binary finding logic is now in one place
- No duplication of the search logic across multiple scripts
- Easier to maintain and update

### 2. Consistency
- All scripts use the same search order
- Uniform error messages across all scripts
- Predictable behavior

### 3. Flexibility
- Scripts can now work with both debug and release builds
- Support for environment variable overrides
- System PATH fallback for installed binaries

### 4. Maintainability
- Single point of change for binary finding logic
- Well-documented functions with clear usage
- Easier to add new binary finding strategies

### 5. Testing
- Integration tests no longer hardcode debug builds
- Can test with release builds for better performance
- More robust CI/CD pipeline support

## Usage Examples

### Example 1: Find or exit
```bash
source "$PROJECT_ROOT/scripts/lib/find-binary.sh"
VALIDATE_YAML=$(find_binary_or_exit "validate-yaml" "VALIDATE_YAML_BIN")
"$VALIDATE_YAML" file.yaml schema.json
```

### Example 2: Ensure built
```bash
source "$PROJECT_ROOT/scripts/lib/find-binary.sh"
ensure_binary_built "validate-yaml" || exit 1
# Binary is now guaranteed to exist
```

### Example 3: Conditional check
```bash
source "$PROJECT_ROOT/scripts/lib/find-binary.sh"
BINARY=$(find_binary "testcase-manager")
if [[ -z "$BINARY" ]]; then
    echo "Binary not found, please build first"
    exit 1
fi
echo "Using: $BINARY"
```

## Migration Summary

**Total files created:** 2
- `scripts/lib/find-binary.sh` - Shared library
- `scripts/lib/README.md` - Documentation

**Total files modified:** 7
- `scripts/watch-yaml-files.sh` - Use ensure_binary_built()
- `scripts/validate-yaml-wrapper.sh` - Use find_binary_or_exit()
- `tests/integration/smoke_test.sh` - Use find_binary()
- `tests/integration/check_environment.sh` - Use find_binary()
- `tests/integration/run_e2e_test.sh` - Use find_binary()
- `tests/integration/run_all_tests.sh` - Use find_binary()
- `tests/integration/ci_test.sh` - Use find_binary()

**Lines of code reduced:** ~50 lines of duplicated logic replaced with function calls

## Technical Details

### Search Order
1. Environment variable (if provided)
2. `target/release/<binary-name>` (optimized build)
3. `target/debug/<binary-name>` (debug build)
4. System PATH via `command -v` (installed binary)

### Error Handling
- `find_binary`: Returns empty string, lets caller handle error
- `find_binary_or_exit`: Exits with code 1 and helpful error message
- `ensure_binary_built`: Attempts to build if missing, returns exit code

### Shell Compatibility
- Uses bash-specific features (BASH_SOURCE, arrays)
- Requires bash (not POSIX sh)
- Works on Linux and macOS

## Future Enhancements

Potential improvements to the library:
- Add `find_all_binaries()` to get all available binaries
- Add version checking functions
- Support for custom search paths
- Caching of binary locations
- Support for multiple binary names (aliases)
