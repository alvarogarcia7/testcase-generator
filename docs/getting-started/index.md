# Quick Start Guide - Interactive Test Case Creation

## Installation

```bash
# Build the project
make build

# Or directly with cargo
cargo build --release
```

## Basic Usage

### 1. Create a Test Case Interactively

```bash
# Start the interactive workflow
testcase-manager create-interactive

# Or with a custom path
testcase-manager create-interactive --path ./my-tests
```

### 2. Follow the Prompts

#### Step 1: Enter Metadata

```
=== Test Case Metadata ===

Requirement: XXX100
Item: 1
TC: 4
ID: 4.2.2.2.1_test
Description: My test description
```

#### Step 2: Validate and Commit

```
=== Validating Metadata ===
✓ Metadata is valid

Commit metadata to git? [Y/n]: y
✓ Committed: Add test case metadata
```

#### Step 3: Add General Initial Conditions

```
Add general initial conditions? [Y/n]: y

=== General Initial Conditions ===

[Your editor will open with a template]
```

Edit the template:
```yaml
- eUICC:
    - "Condition 1"
    - "Condition 2"
```

Save and close the editor.

```
✓ Valid structure
✓ General initial conditions added

Commit general initial conditions to git? [Y/n]: y
✓ Committed: Add general initial conditions
```

#### Step 4: Add Initial Conditions

```
Add initial conditions? [Y/n]: y

=== Initial Conditions ===

[Your editor will open with a template]
```

Edit the template:
```yaml
eUICC:
  - "Initial condition 1"
  - "Initial condition 2"
```

Save and close the editor.

```
✓ Valid structure
✓ Initial conditions added

Commit initial conditions to git? [Y/n]: y
✓ Committed: Add initial conditions
```

#### Step 5: Done!

```
╔═══════════════════════════════════════════════╗
║          Test Case Created Successfully       ║
╚═══════════════════════════════════════════════╝

Saved to: ./testcases/4.2.2.2.1_test.yaml
```

## Example Run

```bash
# Run the example to see it in action
cargo run --example interactive_workflow
```

## Configuration

### Set Your Editor

```bash
# Use vim
export EDITOR=vim

# Or use nano
export EDITOR=nano

# Or use VS Code
export EDITOR="code --wait"
```

### Set Git Author

```bash
export GIT_AUTHOR_NAME="Your Name"
export GIT_AUTHOR_EMAIL="your.email@example.com"
```

## Tips

1. **Integer values**: Enter numbers without quotes (e.g., `1` not `"1"`)
2. **YAML syntax**: Use spaces for indentation, not tabs
3. **Validation errors**: Read carefully and retry
4. **Keep defaults**: Press Y when defaults are shown if they're correct
5. **Skip sections**: Press N when asked "Add general initial conditions?" to skip

## Troubleshooting

### "Invalid type, expected integer"
- Make sure to enter numbers without quotes
- Correct: `1`
- Wrong: `"1"`

### "Editor not found"
- Set the EDITOR environment variable
- `export EDITOR=vim` or `export EDITOR=nano`

### "Git commit failed"
- Set git author information:
  ```bash
  export GIT_AUTHOR_NAME="Your Name"
  export GIT_AUTHOR_EMAIL="your@email.com"
  ```

### "YAML parse error"
- Check indentation (use spaces, not tabs)
- Check list syntax (use `- ` for list items)
- Check for trailing spaces

## Next Steps

- Read the [full documentation](../user-guide/interactive-workflow.md)
- Check out the [implementation details](../development/interactive-implementation.md)
- Explore other commands: `testcase-manager --help`
