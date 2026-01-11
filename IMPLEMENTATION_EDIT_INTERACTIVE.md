# EditInteractive Command Implementation

## Overview
This document describes the implementation of the `EditInteractive` command for the testcase-manager CLI tool, which provides an interactive menu-driven interface for editing test case sections with automatic git commits after each edit.

## Changes Made

### 1. CLI Command Definition (src/cli.rs)
Added new `EditInteractive` variant to the `Commands` enum:
- **Optional Parameters:**
  - `id`: Optional test case ID to edit directly
  - `fuzzy`: Boolean flag to enable fuzzy search for test case selection

### 2. Main Handler (src/main.rs)

#### Primary Handler Function: `handle_edit_interactive()`
- Loads test case via:
  - Direct ID lookup if provided
  - Fuzzy search if `--fuzzy` flag is set
  - Interactive selection menu otherwise
- Opens or initializes git repository
- Enters interactive editing loop with menu presenting 4 editable sections:
  1. Metadata
  2. General Initial Conditions
  3. Initial Conditions
  4. Test Sequences
- Provides "Save and Exit" and "Exit without Saving" options

#### Section-Specific Edit Functions:

**`edit_metadata_section()`**
- Prompts for each metadata field (requirement, item, tc, id, description)
- Pre-populates current values as defaults
- Saves test case after edits
- Creates git commit with message: "Edit metadata section"

**`edit_general_initial_conditions_section()`**
- Displays current general initial conditions in YAML format
- Opens full test case in text editor for editing
- Extracts and saves only the general_initial_conditions changes
- Creates git commit with message: "Edit general initial conditions section"

**`edit_initial_conditions_section()`**
- Displays current initial conditions in YAML format
- Opens full test case in text editor for editing
- Extracts and saves only the initial_conditions changes
- Creates git commit with message: "Edit initial conditions section"

**`edit_test_sequences_section()`**
- Displays current test sequences in YAML format
- Opens full test case in text editor for editing
- Extracts and saves only the test_sequences changes
- Creates git commit with message: "Edit test sequences section"

## Usage Examples

```bash
# Edit with fuzzy search
tcm edit-interactive --fuzzy

# Edit by ID
tcm edit-interactive --id TC001

# Edit by ID from custom path
tcm --path ./testcases edit-interactive --id TC001

# Interactive selection without fuzzy
tcm edit-interactive
```

## Git Integration

The implementation ensures that:
1. A git repository is opened or initialized in the base path
2. Each section edit creates an individual commit
3. Final "Save and Exit" creates a comprehensive commit: "Update test case: {id}"
4. Git author information is read from environment variables:
   - `GIT_AUTHOR_NAME` (default: "Test Case Manager")
   - `GIT_AUTHOR_EMAIL` (default: "testcase@example.com")

## User Flow

1. User invokes `edit-interactive` command
2. Test case is selected/loaded
3. Main menu displays with 6 options
4. User selects a section to edit (1-4)
5. Section-specific edit interface appears
6. Changes are saved and committed to git
7. Returns to main menu
8. User can edit other sections or exit
9. Final save creates comprehensive commit (optional)

## Error Handling

- Failed edits log errors and return to main menu
- User can cancel exit without saving via confirmation prompt
- All git operations use proper error context
- Storage operations handle failures gracefully

## Dependencies

Uses existing functionality from:
- `TestCaseStorage`: Load and save test cases
- `TestCaseFuzzyFinder`: Fuzzy search for test case selection
- `Prompts`: Interactive user input
- `GitManager`: Git operations
- `TestCaseEditor`: Text editor integration
- `print_title`: Formatted console output
