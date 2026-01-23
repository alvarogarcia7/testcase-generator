# Example Script Capture Files

This directory contains two example script capture files that demonstrate realistic terminal output with control characters commonly found in terminal recordings (such as those created by the `script` command).

## Files

### cleanup.txt
A terminal session recording showing a system cleanup operation with:
- **User corrections**: Typos like `lls` corrected to `ls`, `ehco` corrected to `echo`, `df -hh` corrected to `df -h`
- **Backspace characters** (^H / 0x08): Used to show character deletion
- **Bell character** (^G / 0x07): Terminal bell sound after an error
- **ANSI escape sequences** (^[[ / ESC[): 
  - Cursor movement (`^[[A` = up, `^[[B` = down)
  - Line clearing (`^[[K`)
  - Text formatting (`^[[1m` = bold, `^[[32m` = green, `^[[0m` = reset)

### errors.txt
A terminal session recording showing a test suite run with debugging:
- **Extensive backspaces**: Shows long command correction (pythonn → python)
- **Bell character** (^G / 0x07): Terminal alert on connection failure
- **Rich ANSI escape sequences**:
  - Cursor visibility control (`^[[?25l` = hide, `^[[?25h` = show)
  - Line manipulation (`^[[2K` = clear line, `^[[1A` = cursor up)
  - Color codes for test output:
    - `^[[32m` = green (PASS)
    - `^[[31m` = red (FAIL)
    - `^[[33m` = yellow (SKIP/WARNING)
  - Text styling (`^[[1m` = bold, `^[[7m` = inverse)
- **Progress indicators**: Shows dynamic test runner output with live updates
- **User corrections**: URL typo fix (userss → users)

## Control Character Reference

### Backspace (^H or \x08)
```
Binary: 0x08
Effect: Moves cursor one position to the left
Example: "lls^H^Hs" displays as "ls" (backspaces delete the extra 'l')
```

### Bell (^G or \x07)
```
Binary: 0x07
Effect: Triggers terminal bell/beep
Example: "command not found^G" makes a beep sound
```

### ANSI Escape Sequences (^[[ or ESC[)
```
Binary: 0x1B 0x5B (ESC followed by '[')
Common sequences:
  ^[[A          - Cursor up one line
  ^[[B          - Cursor down one line
  ^[[K          - Clear from cursor to end of line
  ^[[2K         - Clear entire line
  ^[[1m         - Bold text
  ^[[0m         - Reset all attributes
  ^[[32m        - Green foreground
  ^[[31m        - Red foreground
  ^[[33m        - Yellow foreground
  ^[[?25l       - Hide cursor
  ^[[?25h       - Show cursor
  ^[[7m         - Reverse video
```

## Converting to Binary Format

The files currently use caret notation (^H, ^G, ^[[) for readability. To convert them to actual binary control characters, run:

```bash
python3 convert_to_binary.py
```

This script will replace:
- `^H` with actual backspace byte (0x08)
- `^G` with actual bell byte (0x07)  
- `^[[` with actual ESC[ bytes (0x1B 0x5B)

## Verification

To verify the control characters after conversion:

### View with visible control characters:
```bash
cat -v cleanup.txt
cat -v errors.txt
```

### View with processed control characters (as terminal would display):
```bash
cat cleanup.txt
cat errors.txt
```

### Check with hexdump:
```bash
hexdump -C cleanup.txt | head -20
hexdump -C errors.txt | head -20
```

### View with `less` (preserves formatting):
```bash
less -R cleanup.txt
less -R errors.txt
```

## Use Cases

These example files are useful for:
1. **Testing terminal output parsers** - Code that needs to strip ANSI sequences
2. **Script cleanup utilities** - Tools that remove control characters from logs
3. **Terminal emulator testing** - Verify correct handling of escape sequences
4. **Log sanitization** - Remove unwanted control characters from log files
5. **CI/CD output processing** - Clean up colorized build output

## Realistic Elements

Both files contain authentic patterns found in real terminal usage:
- Start/end markers from `script` command
- Timestamp headers
- Shell prompts with `$`
- Command typos and corrections
- Error messages
- Background job control (`&`, `[1]`, `kill %1`)
- Mixed successful and failed operations
- Progress indicators with live updates
- ANSI color codes for status (green=success, red=error, yellow=warning)
- Unicode characters (✓, ✗, ⚠) for visual indicators

## Technical Notes

The caret notation format (^H, ^G, ^[[) is:
- **Human-readable** - Easy to see what control characters are present
- **Git-friendly** - Can be diff'd and versioned without binary issues
- **Convertible** - Simple to convert to actual binary bytes when needed

When converted to binary:
- File sizes remain similar (caret notation vs binary)
- Files become true terminal recordings
- Can be played back with `scriptreplay` or similar tools (with timing file)
- Some text editors may not display them correctly

## See Also

- `man script` - Terminal session recording utility
- `man console_codes` - Linux console escape sequences
- `man terminfo` - Terminal capability database
- ANSI/VT100 escape sequence reference: https://en.wikipedia.org/wiki/ANSI_escape_code
