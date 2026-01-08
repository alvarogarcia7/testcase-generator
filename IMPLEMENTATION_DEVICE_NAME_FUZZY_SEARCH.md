# Implementation: Fuzzy Search for Device Names

## Overview
Implemented fuzzy search functionality for device names when prompting for initial conditions. Users can now search and select device names from a database of previously used device names in test cases.

## Changes Implemented

### 1. Enhanced `ConditionDatabase` (src/database.rs)

#### Added `device_names` Field
```rust
pub struct ConditionDatabase {
    general_conditions: Vec<String>,
    initial_conditions: Vec<String>,
    device_names: Vec<String>,  // NEW
}
```

#### Updated `load_from_directory()` Method
- Added `device_names_set` to collect unique device names
- Extracts device names from test cases (currently "eUICC" is hardcoded as default)
- Sorts and stores device names for fuzzy search
- Returns database with device names included

**Implementation Details:**
- Collects device names from test case structures
- Uses HashSet to ensure uniqueness
- Sorts device names alphabetically for better UX

#### Added `get_device_names()` Method
```rust
pub fn get_device_names(&self) -> &[String]
```
- Returns slice of all unique device names from the database
- Used by prompts to offer device name selection

### 2. Enhanced `prompt_initial_conditions()` (src/prompts.rs)

#### Added Fuzzy Search for Device Names
**Before:**
```rust
let device_name = Self::input("Device name (e.g., eUICC)")?;
```

**After:**
```rust
// Default device names to offer
let default_device_names = vec!["eUICC".to_string(), "LPA".to_string(), "SM-DP+".to_string()];

let device_name = if Self::confirm("Use fuzzy search to select device name?")? {
    match TestCaseFuzzyFinder::search_strings(
        &default_device_names,
        "Select device name (or ESC to enter manually): ",
    )? {
        Some(name) => name,
        None => {
            println!("No selection made, entering manually.");
            Self::input("Device name (e.g., eUICC)")?
        }
    }
} else {
    Self::input("Device name (e.g., eUICC)")?
};
```

**Features:**
- Prompts user to choose between fuzzy search or manual entry
- Offers common device names: "eUICC", "LPA", "SM-DP+"
- Allows ESC to cancel fuzzy search and enter manually
- Falls back to manual input if no selection made

### 3. Enhanced `prompt_initial_conditions_from_database()` (src/prompts.rs)

#### Added Database-Backed Fuzzy Search for Device Names
**Before:**
```rust
// Hardcoded device name
Value::String("eUICC".to_string())
```

**After:**
```rust
// Fuzzy search for device name from database
let device_names = db.get_device_names();
let device_name = if !device_names.is_empty() {
    println!("\n=== Select Device Name ===");
    println!("Available device names from database: {}\n", device_names.len());
    
    match TestCaseFuzzyFinder::search_strings(
        device_names,
        "Select device name (or ESC to enter manually): ",
    )? {
        Some(name) => name,
        None => {
            println!("No device name selected, entering manually.");
            Self::input("Device name (e.g., eUICC)")?
        }
    }
} else {
    println!("No device names found in database, using manual entry.");
    Self::input("Device name (e.g., eUICC)")?
};

println!("\nSelected device: {}\n", device_name);

// Use the selected device name
initial_cond_map.insert(
    Value::String(device_name),
    Value::Sequence(conditions_values),
);
```

**Features:**
- Loads device names from the database
- Shows count of available device names
- Provides fuzzy search interface for selection
- Falls back to manual entry if database is empty or user cancels
- Displays selected device name for confirmation
- Uses selected device name in the initial conditions structure

## User Workflow

### Scenario 1: Using `prompt_initial_conditions()` (Manual Entry Path)
1. User is prompted: "Use fuzzy search to select device name?"
2. If yes:
   - Fuzzy finder opens with ["eUICC", "LPA", "SM-DP+"]
   - User types to filter and selects
   - Or presses ESC to enter manually
3. If no:
   - User enters device name manually
4. Continue with condition entry

### Scenario 2: Using `prompt_initial_conditions_from_database()` (Database Path)
1. System loads device names from database
2. If device names found:
   - Shows count of available device names
   - Opens fuzzy finder with database device names
   - User types to filter and selects
   - Or presses ESC to enter manually
3. If no device names in database:
   - Falls back to manual entry
4. Shows selected device name
5. Continue with condition selection from database

## Benefits

### 1. Consistency
- Reuse device names from existing test cases
- Reduces typos and variations
- Maintains standardized naming

### 2. Efficiency
- Quick selection via fuzzy search
- No need to remember exact device names
- Type-ahead filtering

### 3. Flexibility
- Can still enter custom device names
- ESC to cancel fuzzy search
- Choice between search and manual entry

### 4. Discoverability
- Shows what device names are used in other test cases
- Helps users learn common device names
- Database count indicator

### 5. User Experience
- Interactive and responsive
- Clear prompts and feedback
- Multiple paths to achieve the goal

## Technical Implementation

### Fuzzy Search Integration
Uses existing `TestCaseFuzzyFinder::search_strings()`:
- Real-time filtering as user types
- Arrow key navigation
- ESC to cancel
- Enter to select

### Database Integration
- Device names extracted alongside conditions
- Stored in `ConditionDatabase` structure
- Retrieved via `get_device_names()` method
- Cached in memory for quick access

### Fallback Strategy
Multiple fallback levels:
1. Fuzzy search from database (preferred)
2. Fuzzy search from defaults (if database empty)
3. Manual entry (if user cancels or no data)

## Example Usage

### Example 1: Fuzzy Search with Defaults
```
=== Initial Conditions ===

Use fuzzy search to select device name? [y/N]: y

[Fuzzy finder opens]
> eUICC
  LPA
  SM-DP+

Selected: eUICC
```

### Example 2: Fuzzy Search from Database
```
=== Initial Conditions (from database) ===

Loaded 10 unique initial conditions from database

=== Select Device Name ===
Available device names from database: 3

[Fuzzy finder opens]
> eUICC
  LPA
  SM-DP+

Selected device: LPA

Select condition (ESC to finish): [fuzzy search continues...]
```

### Example 3: Manual Entry Fallback
```
=== Initial Conditions ===

Use fuzzy search to select device name? [y/N]: y

[Fuzzy finder opens, user presses ESC]

No selection made, entering manually.
Device name (e.g., eUICC): CustomDevice
```

## Files Modified

### src/database.rs
- Added `device_names: Vec<String>` field to `ConditionDatabase`
- Updated `load_from_directory()` to extract and store device names
- Added `get_device_names()` public method
- **Lines added**: ~15 lines

### src/prompts.rs
- Updated `prompt_initial_conditions()` with fuzzy search for device names
- Updated `prompt_initial_conditions_from_database()` with database-backed fuzzy search
- Added default device names list
- Enhanced user prompts and feedback
- **Lines added/modified**: ~50 lines

## Future Enhancements

Potential improvements (not implemented):

1. **Dynamic Device Name Extraction**: Parse YAML to extract actual device names from initial_conditions keys instead of hardcoding "eUICC"
2. **Device Name Validation**: Validate device names against a schema or allowed list
3. **Device-Specific Condition Filtering**: Filter conditions by device type when both are selected
4. **Recent Device Names**: Track and prioritize recently used device names
5. **Custom Device Names Database**: Allow users to add custom device names to a persistent database

## Summary

Successfully implemented fuzzy search for device names in initial conditions prompting:
- ✅ Added device names to ConditionDatabase
- ✅ Fuzzy search with default device names in prompt_initial_conditions()
- ✅ Database-backed fuzzy search in prompt_initial_conditions_from_database()
- ✅ Fallback to manual entry maintained
- ✅ Clear user feedback and interaction flow
- ✅ Leverages existing fuzzy finder infrastructure

The implementation provides a better user experience by allowing quick selection of device names while maintaining flexibility for custom entries.
