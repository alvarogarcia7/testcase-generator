#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DRY_RUN=false
LOG_FILE="${SCRIPT_DIR}/migration.log"

log() {
    local timestamp
    timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo "[${timestamp}] $*" | tee -a "${LOG_FILE}"
}

log_error() {
    local timestamp
    timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo "[${timestamp}] ERROR: $*" | tee -a "${LOG_FILE}" >&2
}

usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS]

Migrate test-acceptance directory to organized structure with numbered prefixes.

OPTIONS:
    --dry-run       Show what would be done without making changes
    -h, --help      Show this help message

DIRECTORY STRUCTURE:
    00_test_cases/              - Test case YAML files
    05_scripts/                 - Test execution scripts
    10_test_results/            - Test execution results and logs
        execution_logs/         - Execution log files
    20_verification/            - Verification results
    30_documentation_source/    - Documentation source files and reports

MIGRATION MAPPING:
    test_cases       → 00_test_cases
    scripts_test     → 05_scripts (legacy 'scripts' also supported)
    results/logs     → 10_test_results/execution_logs
    results/*        → 10_test_results/* (other result files)
    verification_results → 20_verification (if exists)
    reports          → 30_documentation_source (if exists)

BACKWARD COMPATIBILITY:
    Symlinks are created from old directory names to new locations.

EOF
    exit 0
}

parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --dry-run)
                DRY_RUN=true
                log "DRY RUN MODE: No changes will be made"
                shift
                ;;
            -h|--help)
                usage
                ;;
            *)
                log_error "Unknown option: $1"
                usage
                ;;
        esac
    done
}

execute() {
    local cmd="$*"
    if [[ "${DRY_RUN}" == true ]]; then
        log "[DRY RUN] Would execute: $cmd"
    else
        log "Executing: $cmd"
        eval "$cmd" || {
            log_error "Command failed: $cmd"
            return 1
        }
    fi
}

create_directory() {
    local dir="$1"
    if [[ -d "$dir" ]]; then
        log "Directory already exists: $dir"
        return 0
    fi
    execute "mkdir -p '$dir'"
}

move_content() {
    local src="$1"
    local dest="$2"
    
    if [[ ! -e "$src" ]]; then
        log "Source does not exist, skipping: $src"
        return 0
    fi
    
    if [[ -L "$src" ]]; then
        log "Source is already a symlink, skipping: $src"
        return 0
    fi
    
    if [[ -e "$dest" ]] && [[ ! -L "$dest" ]]; then
        log "Destination already exists, skipping move: $dest"
        return 0
    fi
    
    local dest_parent
    dest_parent="$(dirname "$dest")"
    create_directory "$dest_parent"
    
    execute "mv '$src' '$dest'"
}

create_symlink() {
    local target="$1"
    local link_name="$2"
    
    if [[ -L "$link_name" ]]; then
        local existing_target
        existing_target="$(readlink "$link_name")"
        if [[ "$existing_target" == "$target" ]]; then
            log "Symlink already exists and points to correct target: $link_name -> $target"
            return 0
        else
            log "Symlink exists but points to different target: $link_name -> $existing_target"
            log "Removing old symlink and creating new one"
            execute "rm '$link_name'"
        fi
    fi
    
    if [[ -e "$link_name" ]]; then
        log_error "Cannot create symlink, path exists and is not a symlink: $link_name"
        return 1
    fi
    
    execute "ln -s '$target' '$link_name'"
}

validate_structure() {
    log "Validating directory structure..."
    local failed=false
    
    local required_dirs=(
        "00_test_cases"
        "05_scripts"
        "10_test_results"
        "10_test_results/execution_logs"
        "20_verification"
        "30_documentation_source"
    )
    
    for dir in "${required_dirs[@]}"; do
        local full_path="${SCRIPT_DIR}/${dir}"
        if [[ -d "$full_path" ]] || [[ -L "$full_path" ]]; then
            log "✓ Directory exists: $dir"
        else
            log_error "✗ Required directory missing: $dir"
            failed=true
        fi
    done
    
    local expected_symlinks=(
        "test_cases:00_test_cases"
        "scripts:05_scripts"
        "execution_logs:10_test_results/execution_logs"
    )
    
    for symlink_spec in "${expected_symlinks[@]}"; do
        IFS=':' read -r link_name target <<< "$symlink_spec"
        local full_link_path="${SCRIPT_DIR}/${link_name}"
        
        if [[ -L "$full_link_path" ]]; then
            local actual_target
            actual_target="$(readlink "$full_link_path")"
            if [[ "$actual_target" == "$target" ]]; then
                log "✓ Symlink exists: $link_name -> $target"
            else
                log_error "✗ Symlink points to wrong target: $link_name -> $actual_target (expected: $target)"
                failed=true
            fi
        elif [[ -e "$full_link_path" ]]; then
            log "⚠ Path exists but is not a symlink: $link_name (migration may be incomplete)"
        else
            log "⚠ Symlink does not exist: $link_name (not critical if old structure doesn't exist)"
        fi
    done
    
    if [[ "$failed" == true ]]; then
        log_error "Validation failed"
        return 1
    else
        log "✓ Validation passed"
        return 0
    fi
}

migrate() {
    log "========================================="
    log "Starting migration to organized structure"
    log "========================================="
    log "Working directory: ${SCRIPT_DIR}"
    
    cd "${SCRIPT_DIR}"
    
    log ""
    log "Step 1: Creating new directory structure"
    log "-----------------------------------------"
    create_directory "00_test_cases"
    create_directory "05_scripts"
    create_directory "10_test_results"
    create_directory "10_test_results/execution_logs"
    create_directory "20_verification"
    create_directory "30_documentation_source"
    
    log ""
    log "Step 2: Moving existing content"
    log "-----------------------------------------"
    
    if [[ -d "test_cases" ]] && [[ ! -L "test_cases" ]]; then
        log "Moving test_cases to 00_test_cases..."
        if [[ -d "00_test_cases" ]] && [[ -n "$(ls -A '00_test_cases' 2>/dev/null)" ]]; then
            log "Target directory 00_test_cases already has content, merging..."
            for item in test_cases/*; do
                if [[ -e "$item" ]]; then
                    local basename
                    basename="$(basename "$item")"
                    if [[ ! -e "00_test_cases/$basename" ]]; then
                        execute "mv '$item' '00_test_cases/$basename'"
                    else
                        log "Item already exists in target, skipping: $basename"
                    fi
                fi
            done
            execute "rmdir test_cases 2>/dev/null || true"
        else
            move_content "test_cases" "00_test_cases"
        fi
    fi
    
    if [[ -d "scripts" ]] && [[ ! -L "scripts" ]]; then
        log "Moving scripts to 05_scripts..."
        if [[ -d "05_scripts" ]] && [[ -n "$(ls -A '05_scripts' 2>/dev/null)" ]]; then
            log "Target directory 05_scripts already has content, merging..."
            for item in scripts/*; do
                if [[ -e "$item" ]]; then
                    local basename
                    basename="$(basename "$item")"
                    if [[ ! -e "05_scripts/$basename" ]]; then
                        execute "mv '$item' '05_scripts/$basename'"
                    else
                        log "Item already exists in target, skipping: $basename"
                    fi
                fi
            done
            execute "rmdir scripts 2>/dev/null || true"
        else
            move_content "scripts" "05_scripts"
        fi
    fi
    
    if [[ -d "scripts_test" ]] && [[ ! -L "scripts_test" ]]; then
        log "Moving scripts_test to 05_scripts..."
        if [[ -d "05_scripts" ]] && [[ -n "$(ls -A '05_scripts' 2>/dev/null)" ]]; then
            log "Target directory 05_scripts already has content, merging..."
            for item in scripts_test/*; do
                if [[ -e "$item" ]]; then
                    local basename
                    basename="$(basename "$item")"
                    if [[ ! -e "05_scripts/$basename" ]]; then
                        execute "mv '$item' '05_scripts/$basename'"
                    else
                        log "Item already exists in target, skipping: $basename"
                    fi
                fi
            done
            execute "rmdir scripts_test 2>/dev/null || true"
        else
            move_content "scripts_test" "05_scripts"
        fi
    fi
    
    if [[ -d "results/logs" ]] && [[ ! -L "results/logs" ]]; then
        log "Moving results/logs to 10_test_results/execution_logs..."
        if [[ -d "10_test_results/execution_logs" ]] && [[ -n "$(ls -A '10_test_results/execution_logs' 2>/dev/null)" ]]; then
            log "Target directory execution_logs already has content, merging..."
            for item in results/logs/*; do
                if [[ -e "$item" ]]; then
                    local basename
                    basename="$(basename "$item")"
                    if [[ ! -e "10_test_results/execution_logs/$basename" ]]; then
                        execute "mv '$item' '10_test_results/execution_logs/$basename'"
                    else
                        log "Item already exists in target, skipping: $basename"
                    fi
                fi
            done
            execute "rmdir results/logs 2>/dev/null || true"
        else
            move_content "results/logs" "10_test_results/execution_logs"
        fi
    fi
    
    if [[ -d "results" ]] && [[ ! -L "results" ]]; then
        log "Moving other results content to 10_test_results..."
        for item in results/*; do
            if [[ -e "$item" ]]; then
                local basename
                basename="$(basename "$item")"
                if [[ "$basename" != "logs" ]]; then
                    if [[ ! -e "10_test_results/$basename" ]]; then
                        execute "mv '$item' '10_test_results/$basename'"
                    else
                        log "Item already exists in target, skipping: $basename"
                    fi
                fi
            fi
        done
        if [[ -z "$(ls -A results 2>/dev/null)" ]]; then
            execute "rmdir results 2>/dev/null || true"
        fi
    fi
    
    if [[ -d "execution_logs" ]] && [[ ! -L "execution_logs" ]]; then
        log "Moving execution_logs to 10_test_results/execution_logs..."
        if [[ -d "10_test_results/execution_logs" ]] && [[ -n "$(ls -A '10_test_results/execution_logs' 2>/dev/null)" ]]; then
            log "Target directory execution_logs already has content, merging..."
            for item in execution_logs/*; do
                if [[ -e "$item" ]]; then
                    local basename
                    basename="$(basename "$item")"
                    if [[ ! -e "10_test_results/execution_logs/$basename" ]]; then
                        execute "mv '$item' '10_test_results/execution_logs/$basename'"
                    else
                        log "Item already exists in target, skipping: $basename"
                    fi
                fi
            done
            execute "rmdir execution_logs 2>/dev/null || true"
        else
            move_content "execution_logs" "10_test_results/execution_logs"
        fi
    fi
    
    if [[ -d "verification_results" ]] && [[ ! -L "verification_results" ]]; then
        log "Moving verification_results to 20_verification..."
        if [[ -d "20_verification" ]] && [[ -n "$(ls -A '20_verification' 2>/dev/null)" ]]; then
            log "Target directory 20_verification already has content, merging..."
            for item in verification_results/*; do
                if [[ -e "$item" ]]; then
                    local basename
                    basename="$(basename "$item")"
                    if [[ ! -e "20_verification/$basename" ]]; then
                        execute "mv '$item' '20_verification/$basename'"
                    else
                        log "Item already exists in target, skipping: $basename"
                    fi
                fi
            done
            execute "rmdir verification_results 2>/dev/null || true"
        else
            move_content "verification_results" "20_verification"
        fi
    fi
    
    if [[ -d "reports" ]] && [[ ! -L "reports" ]]; then
        log "Moving reports to 30_documentation_source..."
        if [[ -d "30_documentation_source" ]] && [[ -n "$(ls -A '30_documentation_source' 2>/dev/null)" ]]; then
            log "Target directory 30_documentation_source already has content, merging..."
            for item in reports/*; do
                if [[ -e "$item" ]]; then
                    local basename
                    basename="$(basename "$item")"
                    if [[ ! -e "30_documentation_source/$basename" ]]; then
                        execute "mv '$item' '30_documentation_source/$basename'"
                    else
                        log "Item already exists in target, skipping: $basename"
                    fi
                fi
            done
            execute "rmdir reports 2>/dev/null || true"
        else
            move_content "reports" "30_documentation_source"
        fi
    fi
    
    log ""
    log "Step 3: Creating backward compatibility symlinks"
    log "-----------------------------------------"
    
    if [[ ! -e "test_cases" ]]; then
        create_symlink "00_test_cases" "test_cases"
    fi
    
    if [[ ! -e "scripts" ]]; then
        create_symlink "05_scripts" "scripts"
    fi
    
    if [[ ! -e "scripts_test" ]] && [[ -d "05_scripts" ]]; then
        create_symlink "05_scripts" "scripts_test"
    fi
    
    if [[ ! -e "execution_logs" ]]; then
        create_symlink "10_test_results/execution_logs" "execution_logs"
    fi
    
    if [[ ! -e "verification_results" ]]; then
        create_symlink "20_verification" "verification_results"
    fi
    
    if [[ ! -e "reports" ]]; then
        create_symlink "30_documentation_source" "reports"
    fi
    
    log ""
    log "Step 4: Validating directory structure"
    log "-----------------------------------------"
    validate_structure
    
    log ""
    log "========================================="
    log "Migration complete!"
    log "========================================="
    log "Log file: ${LOG_FILE}"
}

main() {
    parse_args "$@"
    
    if [[ -f "${LOG_FILE}" ]] && [[ "${DRY_RUN}" == false ]]; then
        local backup_log="${LOG_FILE}.$(date +%Y%m%d_%H%M%S).bak"
        mv "${LOG_FILE}" "${backup_log}"
        log "Previous log backed up to: ${backup_log}"
    fi
    
    if [[ "${DRY_RUN}" == false ]]; then
        log "Starting migration (log file: ${LOG_FILE})"
    fi
    
    migrate
    
    exit 0
}

main "$@"
