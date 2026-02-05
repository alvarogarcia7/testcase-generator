# Manual Steps Scripts

This directory contains bash scripts generated from YAML test case definitions that include manual steps.

## Generated Scripts

All 10 test case scripts have been successfully generated using the `test-executor generate` command:

1. **TC_MANUAL_SSH_001.sh** - SSH device access verification with manual steps
2. **TC_MANUAL_HARDWARE_002.sh** - Physical hardware setup and verification
3. **TC_MANUAL_UI_003.sh** - User interface verification with manual visual inspection
4. **TC_MANUAL_DEVICE_004.sh** - Device power operations and state transitions
5. **TC_MANUAL_NETWORK_005.sh** - Network configuration and connectivity verification
6. **TC_MANUAL_DATABASE_006.sh** - Database verification with manual SQL query validation
7. **TC_MANUAL_API_007.sh** - API token validation with manual authentication steps
8. **TC_MANUAL_SECURITY_008.sh** - Security certificate verification with manual inspection
9. **TC_MANUAL_BACKUP_009.sh** - Backup restoration process with manual verification steps
10. **TC_MANUAL_MIXED_010.sh** - Mixed automated and manual workflow for complete system validation

## Manual Step Features

Each generated script contains proper manual step handling with:

### 1. Echo Statements
- Step number and description are displayed
- Command to be performed is shown
- Clear information messages guide the user

Example:
```bash
echo "Step 2: Manually SSH into device and verify login"
echo "Command: ssh admin@192.168.1.100"
echo "INFO: This is a manual step. You must perform this action manually."
```

### 2. User Prompts
- Interactive prompts ask user to confirm completion
- Uses `read -p` for user input

Example:
```bash
read -p "Press ENTER to continue..."
```

### 3. Interactive/Non-Interactive Mode Detection
- Checks both `DEBIAN_FRONTEND` environment variable
- Checks TTY status using `-t 0` test
- Automatically skips prompts in non-interactive environments

Example:
```bash
if [[ "${DEBIAN_FRONTEND}" != 'noninteractive' && -t 0 ]]; then
    read -p "Press ENTER to continue..."
else
    echo "Non-interactive mode detected, skipping manual step confirmation."
fi
```

## Usage

To run a generated script:
```bash
bash examples/manual_steps_scripts/TC_MANUAL_SSH_001.sh
```

In non-interactive mode (CI/CD):
```bash
DEBIAN_FRONTEND=noninteractive bash examples/manual_steps_scripts/TC_MANUAL_SSH_001.sh
```

Or via pipe/redirect:
```bash
echo "" | bash examples/manual_steps_scripts/TC_MANUAL_SSH_001.sh
```

## Generation Command

These scripts were generated using:
```bash
./target/release/test-executor generate testcases/examples/manual_steps/<TEST_CASE>.yaml -o examples/manual_steps_scripts/<TEST_CASE>.sh
```
