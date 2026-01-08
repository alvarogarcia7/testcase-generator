# Implementation: Database Injection into Prompts

## Overview
Injected the ConditionDatabase into the Prompts struct and modified the TestCaseBuilder to create Prompts instances with the necessary database dependencies. All compilation issues have been resolved and tests remain functional.

## Changes Implemented

### 1. Modified Prompts Structure (src/prompts.rs)

**Before:**
```rust
pub struct Prompts {
    db: ConditionDatabase
}
```

**After:**
```rust
pub struct Prompts {
    db: Option<ConditionDatabase>,
}
```

**Added Constructor Methods:**
```rust
impl Prompts {
    /// Create a new Prompts instance without a database
    pub fn new() -> Self {
        Self { db: None }
    }

    /// Create a new Prompts instance with a database
    pub fn new_with_database(db: ConditionDatabase) -> Self {
        Self { db: Some(db) }
    }
    
    // ... static methods remain unchanged
}
```

**Key Changes:**
- Made `db` an `Option<ConditionDatabase>` to allow Prompts to work with or without a database
- Added `new()` constructor for creating Prompts without database
- Added `new_with_database()` constructor for creating Prompts with database
- All existing static methods (input, confirm, etc.) remain unchanged

### 2. Updated prompt_initial_conditions Method (src/prompts.rs)

**Modified to use database from self:**
```rust
pub fn prompt_initial_conditions(
    &self,
    defaults: Option<&Value>,
    validator: &SchemaValidator,
) -> Result<Value> {
    // ...
    
    // Get device names from database or use defaults
    let default_device_names = if let Some(ref db) = self.db {
        let db_names = db.get_device_names();
        if !db_names.is_empty() {
            db_names.to_vec()
        } else {
            vec!["eUICC".to_string(), "LPA".to_string(), "SM-DP+".to_string()]
        }
    } else {
        vec!["eUICC".to_string(), "LPA".to_string(), "SM-DP+".to_string()]
    };
    
    let device_name = match TestCaseFuzzyFinder::search_strings(
        &default_device_names,
        "Select device name (fuzzy search, or ESC to enter manually): ",
    )? {
        Some(name) => name,
        None => {
            println!("No selection made, entering manually.");
            Self::input("Device name (e.g., eUICC)")?
        }
    };
    
    // ... rest of method
}
```

**Features:**
- Changed signature from `pub fn` to `&self` method
- Accesses `self.db` to get device names
- Falls back to default device names if database is None or empty
- Maintains backward compatibility with default device names

### 3. Enhanced TestCaseBuilder (src/builder.rs)

**Added database field:**
```rust
pub struct TestCaseBuilder {
    base_path: PathBuf,
    validator: SchemaValidator,
    git_manager: Option<GitManager>,
    structure: IndexMap<String, Value>,
    recovery_manager: RecoveryManager,
    db: Option<ConditionDatabase>,  // NEW
}
```

**Updated new() constructor:**
```rust
pub fn new<P: AsRef<Path>>(base_path: P) -> Result<Self> {
    let base_path = base_path.as_ref().to_path_buf();
    let validator = SchemaValidator::new()?;
    let git_manager = GitManager::open(&base_path)
        .or_else(|_| GitManager::init(&base_path))
        .ok();
    let recovery_manager = RecoveryManager::new(&base_path);

    // Try to load database from base_path
    let db = ConditionDatabase::load_from_directory(&base_path).ok();

    Ok(Self {
        base_path,
        validator,
        git_manager,
        structure: IndexMap::new(),
        recovery_manager,
        db,
    })
}
```

**Updated new_with_recovery() constructor:**
```rust
pub fn new_with_recovery<P: AsRef<Path>>(base_path: P) -> Result<Self> {
    // ... existing code ...
    
    // Try to load database from base_path
    let db = ConditionDatabase::load_from_directory(&base_path).ok();

    Ok(Self {
        base_path,
        validator,
        git_manager,
        structure,
        recovery_manager,
        db,
    })
}
```

**Updated add_initial_conditions() method:**
```rust
pub fn add_initial_conditions(&mut self, defaults: Option<&Value>) -> Result<&mut Self> {
    let prompts = if let Some(ref db) = self.db {
        Prompts::new_with_database(db.clone())
    } else {
        Prompts::new()
    };
    let conditions = prompts.prompt_initial_conditions(defaults, &self.validator)
        .context("Failed to prompt for initial conditions")?;

    self.structure
        .insert("initial_conditions".to_string(), conditions);

    Ok(self)
}
```

**Features:**
- TestCaseBuilder now automatically loads database from base_path
- Database is optional (uses .ok() to ignore errors)
- Creates Prompts with database if available, otherwise without
- Passes database through cloning (made ConditionDatabase Clone)

### 4. Made ConditionDatabase Cloneable (src/database.rs)

**Before:**
```rust
pub struct ConditionDatabase {
    general_conditions: Vec<String>,
    initial_conditions: Vec<String>,
    device_names: Vec<String>,
}
```

**After:**
```rust
#[derive(Clone)]
pub struct ConditionDatabase {
    general_conditions: Vec<String>,
    initial_conditions: Vec<String>,
    device_names: Vec<String>,
}
```

**Reason:**
- Needed to clone database when creating Prompts instances
- All fields are Vec<String> which are already Clone
- Simple derive Clone implementation

## Architecture

### Dependency Flow
```
TestCaseBuilder
    ├─ loads ConditionDatabase from base_path
    ├─ creates Prompts::new_with_database(db)
    │   └─ Prompts uses db for device names in prompt_initial_conditions
    └─ calls prompts.prompt_initial_conditions()
```

### Optional Database Pattern
```
TestCaseBuilder::new(path)
    ├─ Try load database: ConditionDatabase::load_from_directory(path).ok()
    │   ├─ Success: db = Some(ConditionDatabase)
    │   └─ Failure: db = None
    │
    └─ Create Prompts
        ├─ if db.is_some(): Prompts::new_with_database(db)
        │   └─ Uses database device names
        └─ if db.is_none(): Prompts::new()
            └─ Uses default device names
```

## Benefits

### 1. Dependency Injection
- Database is properly injected into Prompts
- No global state or static variables
- Clean separation of concerns

### 2. Flexibility
- Prompts can work with or without database
- Graceful fallback to defaults if database unavailable
- No breaking changes to existing code

### 3. Testability
- Prompts::new() can be used in tests without database
- Prompts::new_with_database() allows testing with mock databases
- Static methods remain unchanged for backward compatibility

### 4. Automatic Loading
- TestCaseBuilder automatically loads database from base_path
- No manual database management required by caller
- Database shared across all Prompts instances in builder

### 5. Error Handling
- Database loading errors are silently ignored (.ok())
- Application continues to work even if database unavailable
- Falls back to reasonable defaults

## Compilation Issues Resolved

### Issue 1: Missing Constructors
**Problem:** Prompts had `db` field but no way to create instances
**Solution:** Added `new()` and `new_with_database()` constructors

### Issue 2: Static vs Instance Methods
**Problem:** `prompt_initial_conditions` was trying to access `self.db` but was static
**Solution:** Changed signature to `&self` method, kept other methods static

### Issue 3: Database Not Available in Builder
**Problem:** Builder didn't have database to pass to Prompts
**Solution:** Added `db` field to builder and auto-load in constructors

### Issue 4: ConditionDatabase Not Cloneable
**Problem:** Needed to clone database when creating Prompts
**Solution:** Added `#[derive(Clone)]` to ConditionDatabase

### Issue 5: Incomplete Syntax
**Problem:** Builder had `Prompts(db)` which is invalid syntax
**Solution:** Changed to proper constructor call `Prompts::new_with_database(db.clone())`

## Tests

### No Test Modifications Required
- All tests in src/prompts.rs test TestCaseMetadata, not Prompts struct
- TestCaseMetadata methods don't depend on Prompts instance
- Tests remain unchanged and functional

### Test Coverage
Existing tests cover:
- `test_metadata_to_yaml()` - YAML conversion
- `test_metadata_from_structure()` - Structure parsing
- `test_metadata_from_structure_missing_field()` - Error handling
- `test_metadata_from_structure_invalid_type()` - Type validation
- `test_validate_recursive_single_attribute()` - Attribute validation
- `test_validate_recursive_all_attributes()` - Full validation
- `test_validate_recursive_unknown_attribute()` - Error cases
- `test_validate_all_attributes_recursively()` - Recursive validation
- `test_get_validatable_attributes()` - Attribute listing
- `test_validate_recursive_iterative()` - Iterative validation

All tests continue to pass without modification.

## Usage Example

### Creating Builder (Automatic Database Loading)
```rust
// Database automatically loaded from base_path
let builder = TestCaseBuilder::new("./testcases")?;

// Builder now has db field populated if database exists
// Otherwise db is None and defaults are used
```

### Using in Builder Methods
```rust
pub fn add_initial_conditions(&mut self, defaults: Option<&Value>) -> Result<&mut Self> {
    // Create Prompts with or without database
    let prompts = if let Some(ref db) = self.db {
        Prompts::new_with_database(db.clone())
    } else {
        Prompts::new()
    };
    
    // Use prompts instance method
    let conditions = prompts.prompt_initial_conditions(defaults, &self.validator)?;
    
    // ...
}
```

### Prompts Usage
```rust
// Without database (uses defaults)
let prompts = Prompts::new();
let conditions = prompts.prompt_initial_conditions(None, &validator)?;

// With database (uses database device names)
let db = ConditionDatabase::load_from_directory("./data")?;
let prompts = Prompts::new_with_database(db);
let conditions = prompts.prompt_initial_conditions(None, &validator)?;
```

## Files Modified

- **src/prompts.rs**: Added constructors, modified prompt_initial_conditions signature
- **src/builder.rs**: Added db field, updated constructors, modified add_initial_conditions
- **src/database.rs**: Added Clone derive
- **IMPLEMENTATION_PROMPTS_DB_INJECTION.md**: This documentation

## Summary

Successfully injected ConditionDatabase into Prompts struct with proper dependency injection pattern:
- ✅ Added constructors for Prompts (new, new_with_database)
- ✅ Made db field Optional for flexibility
- ✅ Updated prompt_initial_conditions to use self.db
- ✅ Added db field to TestCaseBuilder with auto-loading
- ✅ Made ConditionDatabase Clone-able
- ✅ Fixed all compilation issues
- ✅ All tests remain functional (no modifications needed)
- ✅ Graceful fallback to defaults when database unavailable
- ✅ Clean dependency injection pattern throughout
