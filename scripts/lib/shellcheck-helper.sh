#!/usr/bin/env bash
# Shellcheck validation helper for integration tests
# Works with logger.sh functions (pass/fail/info) or standalone echo fallback

# Validates a shell script with shellcheck (errors only)
# Usage: validate_with_shellcheck "/path/to/script.sh" "Description"
# Returns: 0 if passed or shellcheck not available, 1 if failed
validate_with_shellcheck() {
    local script_path="$1"
    local description="${2:-$1}"

    if ! command -v shellcheck &> /dev/null; then
        if type info &> /dev/null; then
            info "shellcheck not installed, skipping validation for: $description"
        else
            echo "  shellcheck not installed, skipping validation for: $description"
        fi
        return 0
    fi

    if shellcheck -S error "$script_path" > /dev/null 2>&1; then
        pass "$description passes shellcheck validation"
        return 0
    else
        fail "$description failed shellcheck validation"
        shellcheck -S error "$script_path" 2>&1 | head -20
        return 1
    fi
}
