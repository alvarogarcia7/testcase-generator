#!/usr/bin/env bash
# Script to detect locally changed crates and their reverse dependencies
# Outputs a space-separated list of crate names that need to be built
# Handles both committed and uncommitted changes in the working directory

set -euo pipefail

# Color output for better readability
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $*" >&2
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $*" >&2
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*" >&2
}

# Check required tools
if ! command -v jq &> /dev/null; then
    log_error "jq is required but not installed. Please install jq."
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    log_error "cargo is required but not installed."
    exit 1
fi

# Parse arguments
BASE_REF="${1:-HEAD}"

log_info "Detecting changes between $BASE_REF and working directory (including uncommitted changes)"

# Get list of changed files
# Combine committed changes (BASE_REF to HEAD) and uncommitted changes (staged + unstaged)
CHANGED_FILES=""

# If BASE_REF is HEAD, only check uncommitted changes
if [[ "$BASE_REF" == "HEAD" ]]; then
    # Get unstaged changes
    UNSTAGED=$(git diff --name-only 2>/dev/null || echo "")
    # Get staged changes
    STAGED=$(git diff --cached --name-only 2>/dev/null || echo "")
    CHANGED_FILES=$(printf '%s\n%s\n' "$UNSTAGED" "$STAGED" | grep -v '^$' | sort -u || echo "")
else
    # Get changes between BASE_REF and HEAD
    COMMITTED=$(git diff --name-only "$BASE_REF...HEAD" 2>/dev/null || echo "")
    # Get unstaged changes
    UNSTAGED=$(git diff --name-only 2>/dev/null || echo "")
    # Get staged changes
    STAGED=$(git diff --cached --name-only 2>/dev/null || echo "")
    CHANGED_FILES=$(printf '%s\n%s\n%s\n' "$COMMITTED" "$UNSTAGED" "$STAGED" | grep -v '^$' | sort -u || echo "")
fi

if [[ -z "$CHANGED_FILES" ]]; then
    log_warn "No changed files detected"
    exit 0
fi

log_info "Changed files:"
echo "$CHANGED_FILES" | sed 's/^/  /' >&2

# Check if workspace root Cargo.toml changed (build everything)
if echo "$CHANGED_FILES" | grep -q '^Cargo\.toml$'; then
    log_info "Workspace root Cargo.toml changed - building all crates"
    cargo metadata --format-version 1 --no-deps | jq -r '.packages[].name' | tr '\n' ' '
    exit 0
fi

# Extract changed crates from file paths
CHANGED_CRATES=()
while IFS= read -r file; do
    # Match files in crates/*/ pattern
    if [[ "$file" =~ ^crates/([^/]+)/ ]]; then
        crate_dir="${BASH_REMATCH[1]}"
        # Verify this is an actual crate directory
        if [[ -f "crates/$crate_dir/Cargo.toml" ]]; then
            CHANGED_CRATES+=("$crate_dir")
        fi
    fi
done <<< "$CHANGED_FILES"

# Remove duplicates from changed crates
CHANGED_CRATES=($(printf '%s\n' "${CHANGED_CRATES[@]}" | sort -u))

if [[ ${#CHANGED_CRATES[@]} -eq 0 ]]; then
    log_info "No crate changes detected (changed files are outside crates/)"
    exit 0
fi

log_info "Changed crates (by directory): ${CHANGED_CRATES[*]}"

# Get cargo metadata with dependencies
METADATA=$(cargo metadata --format-version 1)

# Map crate directories to crate names
declare -A DIR_TO_NAME
while IFS=$'\t' read -r name manifest_path; do
    # Extract directory name from manifest path
    # manifest_path format: /path/to/crates/crate-name/Cargo.toml
    if [[ "$manifest_path" =~ crates/([^/]+)/Cargo\.toml ]]; then
        dir_name="${BASH_REMATCH[1]}"
        DIR_TO_NAME["$dir_name"]="$name"
    fi
done < <(echo "$METADATA" | jq -r '.packages[] | [.name, .manifest_path] | @tsv')

# Convert changed crate directories to crate names
CHANGED_CRATE_NAMES=()
for dir in "${CHANGED_CRATES[@]}"; do
    if [[ -n "${DIR_TO_NAME[$dir]:-}" ]]; then
        CHANGED_CRATE_NAMES+=("${DIR_TO_NAME[$dir]}")
    else
        log_warn "Could not find crate name for directory: $dir"
    fi
done

if [[ ${#CHANGED_CRATE_NAMES[@]} -eq 0 ]]; then
    log_warn "No valid crate names found"
    exit 0
fi

log_info "Changed crates (by name): ${CHANGED_CRATE_NAMES[*]}"

# Build dependency graph: map each crate to its direct dependencies
declare -A CRATE_DEPS
while IFS=$'\t' read -r pkg_name dep_name; do
    if [[ -z "${CRATE_DEPS[$pkg_name]:-}" ]]; then
        CRATE_DEPS["$pkg_name"]="$dep_name"
    else
        CRATE_DEPS["$pkg_name"]="${CRATE_DEPS[$pkg_name]} $dep_name"
    fi
done < <(echo "$METADATA" | jq -r '.packages[] | .name as $pkg | .dependencies[] | select(.path != null) | [$pkg, .name] | @tsv')

# Get all workspace package names
ALL_PACKAGES=($(echo "$METADATA" | jq -r '.packages[].name'))

# Find reverse dependencies: crates that depend on changed crates
# Use iterative approach to find all transitive reverse dependencies
AFFECTED_CRATES=("${CHANGED_CRATE_NAMES[@]}")
declare -A SEEN
for crate in "${CHANGED_CRATE_NAMES[@]}"; do
    SEEN["$crate"]=1
done

# Iterate until no new affected crates are found
CHANGED=1
while [[ $CHANGED -eq 1 ]]; do
    CHANGED=0
    for pkg in "${ALL_PACKAGES[@]}"; do
        # Skip if already in affected list
        if [[ -n "${SEEN[$pkg]:-}" ]]; then
            continue
        fi
        
        # Check if this package depends on any affected crate
        deps="${CRATE_DEPS[$pkg]:-}"
        if [[ -n "$deps" ]]; then
            for dep in $deps; do
                if [[ -n "${SEEN[$dep]:-}" ]]; then
                    # This package depends on an affected crate
                    AFFECTED_CRATES+=("$pkg")
                    SEEN["$pkg"]=1
                    CHANGED=1
                    break
                fi
            done
        fi
    done
done

# Sort and output unique affected crates
AFFECTED_CRATES_SORTED=($(printf '%s\n' "${AFFECTED_CRATES[@]}" | sort -u))

log_info "Affected crates (including reverse dependencies): ${AFFECTED_CRATES_SORTED[*]}"
log_info "Total crates to build: ${#AFFECTED_CRATES_SORTED[@]}"

# Output space-separated list (to stdout, not stderr)
printf '%s' "${AFFECTED_CRATES_SORTED[*]}"
