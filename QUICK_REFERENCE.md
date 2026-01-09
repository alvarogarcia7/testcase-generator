# Test Case Manager - Quick Reference

## Command Line Usage

### Parse Conditions from Database

#### Parse General Initial Conditions
```bash
testcase-manager parse-general-conditions --database data
```

#### Parse Initial Conditions
```bash
testcase-manager parse-initial-conditions --database data
```

### Build Complete Test Case with Steps
```bash
testcase-manager build-sequences-with-steps
```

### Add Steps to Existing Sequence
```bash
testcase-manager add-steps --sequence-id 1
```

### Build Sequences Only (No Steps)
```bash
testcase-manager build-sequences
```

## Database-backed Condition Selection

The database commands extract all unique conditions from existing test case YAML files in a directory, and allow you to add new conditions manually.

### What Gets Extracted

**General Initial Conditions:**
- From `general_initial_conditions[].eUICC[]` fields
- Across all test cases in the database directory

**Initial Conditions:**
- From `initial_conditions.eUICC[]` fields (top-level)
- From `test_sequences[].initial_conditions[].eUICC[]` fields (sequence-level)
- Across all test cases in the database directory

### Interactive Selection Interface

When adding conditions, you have three options:
1. **Search from database (fuzzy search)**
   - Type to filter conditions as you type
   - Use arrow keys to navigate
   - Press Enter to select
   - Press ESC to cancel and return to menu
   
2. **Create new condition (manual entry)**
   - Enter a new condition text directly
   - Useful when the database doesn't have what you need
   - New condition is added to your test case
   
3. **Finish selection**
   - Complete the selection process
   - Shows confirmation before saving

### Current Selection Display
- After each addition, see all selected conditions
- Numbered list for easy review
- Continue adding or finish when ready

### Example Usage

```bash
# Parse general initial conditions from data directory
testcase-manager parse-general-conditions --database data

# Parse initial conditions from custom directory
testcase-manager parse-initial-conditions --database /path/to/testcases
```

### Example Interactive Session

```
=== Current Selection: 0 condition(s) ===
  (none)

=== Add General Initial Condition ===
Options:
  1. Search from database (fuzzy search)
  2. Create new condition (manual entry)
  3. Finish selection

Choice (1/2/3): 1

[Fuzzy finder opens - type to search]
> The profile PROFILE_OPERATIONAL1 is loaded

✓ Added from database: The profile PROFILE_OPERATIONAL1 is loaded on the eUICC.

=== Current Selection: 1 condition(s) ===
  1. The profile PROFILE_OPERATIONAL1 is loaded on the eUICC.

=== Add General Initial Condition ===
Options:
  1. Search from database (fuzzy search)
  2. Create new condition (manual entry)
  3. Finish selection

Choice (1/2/3): 2

Enter new condition: The SIM card is provisioned with test credentials

✓ Added new condition: The SIM card is provisioned with test credentials

=== Current Selection: 2 condition(s) ===
  1. The profile PROFILE_OPERATIONAL1 is loaded on the eUICC.
  2. The SIM card is provisioned with test credentials

=== Add General Initial Condition ===
Options:
  1. Search from database (fuzzy search)
  2. Create new condition (manual entry)
  3. Finish selection

Choice (1/2/3): 3

✓ General initial conditions added to test case

Save to test case? [y/N]: y
✓ General initial conditions saved to: testcases/test_case.yaml

Commit to git? [y/N]: y
✓ Committed: Add general initial conditions
```

## Step Structure

```yaml
- step: 1                    # Auto-generated sequential number
  manual: true               # Optional: true/false
  description: "Step desc"   # Required: what this step does
  command: "ssh"             # Required: command to execute
  expected:                  # Required: expected results
    success: false           # Optional: true/false
    result: "SW=0x9000"      # Required: expected result
    output: "Success"        # Required: expected output
```

## Interactive Prompts Flow

### Step Collection Loop
1. **Step Description**
   - Option: Use fuzzy search for existing descriptions
   - Or: Enter new description

2. **Manual Flag**
   - "Is this a manual step?" (yes/no)
   - Defaults to false if not specified

3. **Command**
   - Enter the command to execute

4. **Expected Results**
   - "Include 'success' field?" (yes/no)
   - If yes: "Success value (true/false)?"
   - "Expected result" (e.g., "SW=0x9000")
   - "Expected output" (e.g., "Operation successful")

5. **Validation & Save**
   - Step validated automatically
   - File saved to disk

6. **Git Commit**
   - "Commit this step to git?" (yes/no)
   - Message: "Add step #N to sequence #M: description"

7. **Continue?**
   - "Add another step to this sequence?" (yes/no)

## Key Features

### Automatic Step Numbering
- Steps numbered 1, 2, 3, ...
- No manual tracking needed
- Continues from last step

### Fuzzy Search
- Press Enter to search existing step descriptions
- Type to filter results
- ESC to cancel and enter manually

### Schema Validation
- All fields validated before saving
- Clear error messages if validation fails
- Required fields enforced

### Git Integration
- Commit after each step (optional)
- Full audit trail
- Descriptive commit messages

### File Persistence
- Saved after each step
- No data loss
- Can resume if interrupted

## Common Patterns

### Standard Step (Auto Success)
```
Description: Execute login command
Manual: no
Command: ssh
Include success: no
Result: SW=0x9000
Output: Login successful
```

### Manual Step with Explicit Success
```
Description: Verify UI shows login screen
Manual: yes
Command: manual_check
Include success: yes
Success: true
Result: UI_DISPLAYED
Output: Login screen visible
```

### Error Case
```
Description: Attempt invalid login
Manual: no
Command: ssh
Include success: yes
Success: false
Result: SW=0x6A88
Output: Authentication failed
```

## Tips

1. **Reuse Conditions**: Use database parsing (option 1) to select from existing conditions for consistency
2. **Create New When Needed**: Use manual entry (option 2) when the database doesn't have the exact condition you need
3. **Review Before Saving**: The current selection display lets you verify all conditions before committing
4. **Reuse Descriptions**: Use fuzzy search to maintain consistency across test cases
5. **Commit Often**: Commit after important steps for audit trail
6. **Optional Fields**: Only add 'manual' and 'success' when needed
7. **Clear Descriptions**: Make descriptions searchable and descriptive
8. **Expected Fields**: Always provide clear result and output values
9. **Database Updates**: Newly added conditions are available in the database for future test cases
10. **Cancel Anytime**: Press ESC in fuzzy search to cancel and return to menu without selecting

## Troubleshooting

### Validation Failed
- Check required fields: step, description, command, expected
- Ensure expected has result and output
- Verify all fields are correct types

### Fuzzy Search Empty
- No existing steps yet in any sequence
- Enter description manually
- After first step, fuzzy search will work

### Commit Failed
- Check git is initialized
- Verify file permissions
- Ensure file is saved

### Wrong Step Number
- Step numbers auto-increment from highest existing
- Delete last step if wrong
- Add steps in order

## Example Session

```
=== Add Step #1 ===

Use fuzzy search for description? [y/N]: n
Step description: Initialize test environment
Is this a manual step? [y/N]: n
Command: ssh
Include 'success' field? [y/N]: n
Expected result: SW=0x9000
Expected output: Environment initialized

✓ Step validated and added

Commit this step to git? [y/N]: y
✓ Committed: Add step #1 to sequence #1: Initialize test environment

Add another step to this sequence? [y/N]: y

=== Add Step #2 ===

Use fuzzy search for description? [y/N]: n
Step description: Verify initialization
Is this a manual step? [y/N]: n
Command: ssh
Include 'success' field? [y/N]: n
Expected result: SW=0x9000
Expected output: Initialization verified

✓ Step validated and added

Commit this step to git? [y/N]: y
✓ Committed: Add step #2 to sequence #1: Verify initialization

Add another step to this sequence? [y/N]: n

✓ All steps added to sequence
```
