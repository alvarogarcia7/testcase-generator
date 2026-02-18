# TTY Detection and Fallback Mechanism

## Overview

The fuzzy search functionality in `testcase-manager` automatically detects whether it's running in a TTY (terminal) environment. When a TTY is not available (e.g., in VS Code debug console, redirected input, or CI/CD pipelines), the system gracefully falls back to a numbered selection interface.

## Features

### 1. Automatic TTY Detection

The system uses Rust's `std::io::IsTerminal` trait to detect if stdin is connected to a terminal:

```rust
use std::io::IsTerminal;
std::io::stdin().is_terminal()
```

### 2. Fuzzy Search Mode (TTY Available)

When running in a terminal with TTY support:
- Full fuzzy search interface using `skim`
- Interactive keyboard navigation
- Real-time filtering
- Multi-select with tab/space

### 3. Numbered Selection Mode (No TTY)

When TTY is not available:
- Clear numbered list display
- Simple numeric input
- Input validation with helpful error messages
- Support for both single and multi-selection

## Usage Examples

### Single Selection

**In TTY mode (normal terminal):**
```
Select device name (fuzzy search, or ESC to enter manually):
> [interactive fuzzy search interface]
```

**In non-TTY mode (VS Code debug console):**
```
⚠ TTY not detected (e.g., VS Code debug console)
Fuzzy search unavailable - using numbered selection instead

Select device name (fuzzy search, or ESC to enter manually):

1) eUICC
2) LPA
3) SM-DP+

Enter number (1-3) or 0 to cancel: 1
```

### Multi-Selection

**In non-TTY mode:**
```
⚠ TTY not detected (e.g., VS Code debug console)
Fuzzy search unavailable - using numbered multi-selection instead

Select multiple features:

1) Feature 1: Authentication
2) Feature 2: Authorization
3) Feature 3: Logging
4) Feature 4: Monitoring
5) Feature 5: Testing

Enter numbers separated by spaces (e.g., '1 3 5'), or 0/empty to finish:
> 1 3 5
✓ Selected 3 item(s)
```

## Error Handling

### Invalid Input

The system validates all numeric inputs:

```
Enter number (1-5) or 0 to cancel: 10
❌ Invalid input. Please enter a number between 0 and 5
```

### Cancellation

Users can cancel selection by:
- Entering `0` in the prompt
- Entering an empty line (for multi-select)
- Pressing ESC in fuzzy mode

## Integration Points

The TTY fallback is integrated into all fuzzy search operations:

1. **Device Name Selection** (`src/prompts.rs`)
   - Lines 311-320: Initial conditions device selection
   - Lines 468-481: Database-backed device selection

2. **Condition Selection** (`src/prompts.rs`)
   - Lines 391-414: General initial conditions
   - Lines 488-512: Initial conditions from database

3. **Sequence Name Selection** (`src/builder.rs`)
   - Lines 438-446: Reusing existing sequence names

4. **Step Description Selection** (`src/builder.rs`)
   - Lines 679-687: Reusing existing step descriptions

## Testing

### Run the Demo Example

```bash
# In a normal terminal (will use fuzzy search)
cargo run --example tty_fallback_demo

# Simulate non-TTY environment
echo "1" | cargo run --example tty_fallback_demo
```

### VS Code Debug Console

When debugging in VS Code, the debug console does not provide a TTY, so the numbered selection will automatically activate.

## Implementation Details

### Core Functions

1. **`is_tty()`** - Detects TTY availability
2. **`numbered_selection()`** - Single-select fallback
3. **`numbered_multi_selection()`** - Multi-select fallback

### Key Methods

- `TestCaseFuzzyFinder::search()` - Test case fuzzy search
- `TestCaseFuzzyFinder::search_strings()` - String list search
- `TestCaseFuzzyFinder::multi_select()` - Multi-selection

All methods automatically choose between fuzzy and numbered mode based on TTY detection.

## Benefits

1. **Robustness** - Works in any environment
2. **User-Friendly** - Clear messages about mode selection
3. **Validation** - Comprehensive input validation
4. **Consistency** - Same API regardless of mode
5. **Accessibility** - Numbered mode is easier for screen readers

## Future Enhancements

Potential improvements:
- Range selection (e.g., "1-5" selects items 1 through 5)
- Default selection option
- History/recent selections
- Configurable display format
