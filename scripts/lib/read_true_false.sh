# Bash helper functions for user prompts
# Prompts user for Y/n input with proper validation
# Returns: 1 for yes, 0 for no
# Supports both interactive and non-interactive modes with TTY detection
read_true_false() {
    local prompt="$1"
    local default="${2:-y}"
    
    # Check if running in non-interactive mode
    if [[ "${DEBIAN_FRONTEND:-}" == 'noninteractive' ]] || ! [ -t 0 ]; then
        # Non-interactive mode: return default
        if [[ "$default" =~ ^[Yy]$ ]]; then
            return 1
        else
            return 0
        fi
    fi
    
    # Interactive mode: prompt user
    while true; do
        if [[ "$default" =~ ^[Yy]$ ]]; then
            read -p "$prompt [Y/n]: " response
        else
            read -p "$prompt [y/N]: " response
        fi
        
        # Empty response uses default
        if [[ -z "$response" ]]; then
            response="$default"
        fi
        
        # Validate response
        case "$response" in
            [Yy]|[Yy][Ee][Ss])
                return 1
                ;;
            [Nn]|[Nn][Oo])
                return 0
                ;;
            *)
                echo "Invalid response. Please enter Y or n." >&2
                ;;
        esac
    done
}
