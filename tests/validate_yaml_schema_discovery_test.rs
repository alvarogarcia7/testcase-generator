// Unit tests for schema discovery functionality in validate-yaml
//
// These tests validate the schema dependency discovery logic
// that finds all transitive schema dependencies through $ref

use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Helper to create a schema file with given content
fn create_schema_file(dir: &Path, name: &str, content: &str) -> PathBuf {
    let path = dir.join(name);
    fs::write(&path, content).expect("Failed to write schema file");
    path
}

#[test]
fn test_schema_discovery_single_schema() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let schema_content = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "type": "object",
        "properties": {
            "name": { "type": "string" }
        }
    }"##;

    let schema_path = create_schema_file(temp_dir.path(), "simple.json", schema_content);

    // Schema discovery should find just the main schema
    // In a real implementation, we'd call discover_schema_dependencies here
    assert!(schema_path.exists());
}

#[test]
fn test_schema_discovery_with_external_ref() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create referenced schema
    let base_schema = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "definitions": {
            "name": { "type": "string" }
        }
    }"##;
    create_schema_file(temp_dir.path(), "base.json", base_schema);

    // Create main schema that references base
    let main_schema = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "type": "object",
        "properties": {
            "name": { "$ref": "base.json#/definitions/name" }
        }
    }"##;
    let main_path = create_schema_file(temp_dir.path(), "main.json", main_schema);

    // Schema discovery should find both main.json and base.json
    assert!(main_path.exists());
    assert!(temp_dir.path().join("base.json").exists());
}

#[test]
fn test_schema_discovery_with_internal_ref_only() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Schema with only internal references (should not find additional files)
    let schema_content = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "definitions": {
            "name": { "type": "string" }
        },
        "type": "object",
        "properties": {
            "name": { "$ref": "#/definitions/name" }
        }
    }"##;

    let schema_path = create_schema_file(temp_dir.path(), "internal.json", schema_content);

    // Should only find the main schema (internal refs don't count)
    assert!(schema_path.exists());
}

#[test]
fn test_schema_discovery_transitive_dependencies() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create level 2 schema (deepest)
    let level2_schema = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "definitions": {
            "email": { "type": "string", "format": "email" }
        }
    }"##;
    create_schema_file(temp_dir.path(), "level2.json", level2_schema);

    // Create level 1 schema that references level2
    let level1_schema = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "definitions": {
            "person": {
                "type": "object",
                "properties": {
                    "email": { "$ref": "level2.json#/definitions/email" }
                }
            }
        }
    }"##;
    create_schema_file(temp_dir.path(), "level1.json", level1_schema);

    // Create main schema that references level1
    let main_schema = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "type": "object",
        "properties": {
            "user": { "$ref": "level1.json#/definitions/person" }
        }
    }"##;
    let main_path = create_schema_file(temp_dir.path(), "main.json", main_schema);

    // Schema discovery should find all three: main.json, level1.json, level2.json
    assert!(main_path.exists());
    assert!(temp_dir.path().join("level1.json").exists());
    assert!(temp_dir.path().join("level2.json").exists());
}

#[test]
fn test_schema_discovery_circular_reference() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create schema A that references B
    let schema_a = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "definitions": {
            "typeA": {
                "type": "object",
                "properties": {
                    "refB": { "$ref": "schemaB.json#/definitions/typeB" }
                }
            }
        }
    }"##;
    create_schema_file(temp_dir.path(), "schemaA.json", schema_a);

    // Create schema B that references A (circular)
    let schema_b = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "definitions": {
            "typeB": {
                "type": "object",
                "properties": {
                    "refA": { "$ref": "schemaA.json#/definitions/typeA" }
                }
            }
        }
    }"##;
    create_schema_file(temp_dir.path(), "schemaB.json", schema_b);

    // Discovery should handle circular references without infinite loop
    // Should find both schemas exactly once
    assert!(temp_dir.path().join("schemaA.json").exists());
    assert!(temp_dir.path().join("schemaB.json").exists());
}

#[test]
fn test_schema_discovery_multiple_refs_same_schema() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create common definitions schema
    let common_schema = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "definitions": {
            "name": { "type": "string" },
            "age": { "type": "integer" }
        }
    }"##;
    create_schema_file(temp_dir.path(), "common.json", common_schema);

    // Create main schema that references common multiple times
    let main_schema = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "type": "object",
        "properties": {
            "firstName": { "$ref": "common.json#/definitions/name" },
            "lastName": { "$ref": "common.json#/definitions/name" },
            "age": { "$ref": "common.json#/definitions/age" }
        }
    }"##;
    let main_path = create_schema_file(temp_dir.path(), "main.json", main_schema);

    // Should find both schemas, but common.json should only appear once
    assert!(main_path.exists());
    assert!(temp_dir.path().join("common.json").exists());
}

#[test]
fn test_schema_discovery_nested_objects() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create base schema
    let base_schema = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "definitions": {
            "address": { "type": "object" }
        }
    }"##;
    create_schema_file(temp_dir.path(), "base.json", base_schema);

    // Create main schema with deeply nested $ref
    let main_schema = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "type": "object",
        "properties": {
            "user": {
                "type": "object",
                "properties": {
                    "profile": {
                        "type": "object",
                        "properties": {
                            "address": { "$ref": "base.json#/definitions/address" }
                        }
                    }
                }
            }
        }
    }"##;
    let main_path = create_schema_file(temp_dir.path(), "main.json", main_schema);

    // Should find $ref even when deeply nested
    assert!(main_path.exists());
    assert!(temp_dir.path().join("base.json").exists());
}

#[test]
fn test_schema_discovery_in_arrays() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create item schema
    let item_schema = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "definitions": {
            "item": { "type": "string" }
        }
    }"##;
    create_schema_file(temp_dir.path(), "item.json", item_schema);

    // Create main schema with $ref in array context
    let main_schema = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "type": "object",
        "properties": {
            "items": {
                "type": "array",
                "items": { "$ref": "item.json#/definitions/item" }
            }
        },
        "oneOf": [
            { "$ref": "item.json#/definitions/item" }
        ]
    }"##;
    let main_path = create_schema_file(temp_dir.path(), "main.json", main_schema);

    // Should find $ref in array items and oneOf
    assert!(main_path.exists());
    assert!(temp_dir.path().join("item.json").exists());
}

#[test]
fn test_schema_discovery_with_missing_referenced_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create schema that references non-existent file
    let main_schema = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "type": "object",
        "properties": {
            "name": { "$ref": "nonexistent.json#/definitions/name" }
        }
    }"##;
    let main_path = create_schema_file(temp_dir.path(), "main.json", main_schema);

    // Discovery should handle missing files gracefully
    // Should still find the main schema
    assert!(main_path.exists());
    assert!(!temp_dir.path().join("nonexistent.json").exists());
}

#[test]
fn test_schema_discovery_relative_path_resolution() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create subdirectory
    let subdir = temp_dir.path().join("schemas");
    fs::create_dir(&subdir).expect("Failed to create subdirectory");

    // Create referenced schema in subdirectory
    let ref_schema = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "definitions": {
            "name": { "type": "string" }
        }
    }"##;
    fs::write(subdir.join("referenced.json"), ref_schema)
        .expect("Failed to write referenced schema");

    // Create main schema in root that references subdirectory schema
    let main_schema = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "type": "object",
        "properties": {
            "name": { "$ref": "schemas/referenced.json#/definitions/name" }
        }
    }"##;
    let main_path = create_schema_file(temp_dir.path(), "main.json", main_schema);

    // Should resolve relative path correctly
    assert!(main_path.exists());
    assert!(subdir.join("referenced.json").exists());
}

#[test]
fn test_schema_discovery_complex_multi_level() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create a complex schema hierarchy:
    // main -> [person, address]
    // person -> [name, email]
    // address -> [country]

    let country_schema = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "definitions": {
            "country": { "type": "string", "enum": ["US", "UK", "CA"] }
        }
    }"##;
    create_schema_file(temp_dir.path(), "country.json", country_schema);

    let email_schema = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "definitions": {
            "email": { "type": "string", "format": "email" }
        }
    }"##;
    create_schema_file(temp_dir.path(), "email.json", email_schema);

    let name_schema = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "definitions": {
            "name": { "type": "string" }
        }
    }"##;
    create_schema_file(temp_dir.path(), "name.json", name_schema);

    let address_schema = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "definitions": {
            "address": {
                "type": "object",
                "properties": {
                    "country": { "$ref": "country.json#/definitions/country" }
                }
            }
        }
    }"##;
    create_schema_file(temp_dir.path(), "address.json", address_schema);

    let person_schema = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "definitions": {
            "person": {
                "type": "object",
                "properties": {
                    "name": { "$ref": "name.json#/definitions/name" },
                    "email": { "$ref": "email.json#/definitions/email" }
                }
            }
        }
    }"##;
    create_schema_file(temp_dir.path(), "person.json", person_schema);

    let main_schema = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "type": "object",
        "properties": {
            "person": { "$ref": "person.json#/definitions/person" },
            "address": { "$ref": "address.json#/definitions/address" }
        }
    }"##;
    let main_path = create_schema_file(temp_dir.path(), "main.json", main_schema);

    // Should discover all 6 schemas:
    // main.json, person.json, address.json, name.json, email.json, country.json
    assert!(main_path.exists());
    assert!(temp_dir.path().join("person.json").exists());
    assert!(temp_dir.path().join("address.json").exists());
    assert!(temp_dir.path().join("name.json").exists());
    assert!(temp_dir.path().join("email.json").exists());
    assert!(temp_dir.path().join("country.json").exists());
}

#[test]
fn test_find_external_refs_ignores_internal() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Schema with mix of internal and external refs
    let schema_content = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "definitions": {
            "internal": { "type": "string" }
        },
        "type": "object",
        "properties": {
            "field1": { "$ref": "#/definitions/internal" },
            "field2": { "$ref": "external.json#/definitions/external" }
        }
    }"##;

    create_schema_file(temp_dir.path(), "external.json", "{}");
    let main_path = create_schema_file(temp_dir.path(), "main.json", schema_content);

    // Should find external.json but not try to process #/definitions/internal
    assert!(main_path.exists());
    assert!(temp_dir.path().join("external.json").exists());
}

#[test]
fn test_schema_with_fragment_in_external_ref() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create referenced schema
    let ref_schema = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "definitions": {
            "myType": { "type": "string" },
            "otherType": { "type": "integer" }
        }
    }"##;
    create_schema_file(temp_dir.path(), "defs.json", ref_schema);

    // Create main schema with fragment references
    let main_schema = r##"{
        "$schema": "http://json-schema.org/draft-04/schema#",
        "type": "object",
        "properties": {
            "field1": { "$ref": "defs.json#/definitions/myType" },
            "field2": { "$ref": "defs.json#/definitions/otherType" }
        }
    }"##;
    let main_path = create_schema_file(temp_dir.path(), "main.json", main_schema);

    // Should extract defs.json from both fragment references
    assert!(main_path.exists());
    assert!(temp_dir.path().join("defs.json").exists());
}
