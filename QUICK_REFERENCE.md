# Step Collection Loop - Quick Reference

## Command Line Usage

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

1. **Reuse Descriptions**: Use fuzzy search to maintain consistency
2. **Commit Often**: Commit after important steps for audit trail
3. **Optional Fields**: Only add 'manual' and 'success' when needed
4. **Clear Descriptions**: Make descriptions searchable and descriptive
5. **Expected Fields**: Always provide clear result and output values

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
