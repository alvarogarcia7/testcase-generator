#!/usr/bin/env python3
"""
YAML Schema Validator for TCMS Schemas

Validates YAML files against JSON schemas.
Dependencies: PyYAML, jsonschema, click
"""

import json
import sys
from pathlib import Path
from typing import Any

try:
    import click
except ImportError:
    print("Error: click library is required. Install with: pip install click")
    sys.exit(1)

try:
    import yaml
except ImportError:
    print("Error: PyYAML library is required. Install with: pip install pyyaml")
    sys.exit(1)

try:
    from jsonschema import SchemaError, ValidationError, validate
except ImportError:
    print("Error: jsonschema library is required. Install with: pip install jsonschema")
    sys.exit(1)


class YamlSchemaValidator:
    @staticmethod
    def load_json_schema(path_: Path) -> tuple[bool, Any]:
        """Load and parse a JSON schema file."""
        try:
            with open(path_) as json_schema_file:
                json_schema = json.load(json_schema_file)
                return True, json_schema
        except FileNotFoundError:
            print(f"Error: Schema file '{path_}' not found")
            return False, None
        except json.JSONDecodeError as e:
            print(f"Error parsing JSON schema: {e}")
            return False, None
        except Exception as e:
            print(f"Error reading JSON schema file: {e}")
            return False, None

    def __init__(self, yaml_file: Path, schema_object: dict[str, Any]):
        """
        Initialize validator with YAML file and schema object.

        Args:
            yaml_file: Path to the YAML file to validate
            schema_object: Parsed JSON schema dictionary
        """
        self.yaml_file = yaml_file
        self.data = None
        self.schema = schema_object

    def load_yaml(self) -> bool:
        """Load YAML file."""
        try:
            with open(self.yaml_file) as f:
                self.data = yaml.safe_load(f)
            return True
        except FileNotFoundError:
            print(f"Error: File '{self.yaml_file}' not found")
            return False
        except yaml.YAMLError as e:
            print(f"Error parsing YAML: {e}")
            return False
        except Exception as e:
            print(f"Error reading YAML file: {e}")
            return False

    def validate(self) -> bool:
        """Validate loaded YAML data against schema."""
        if not self.data:
            print("Error: No data loaded. Call load_yaml() first.")
            return False
        try:
            validate(instance=self.data, schema=self.schema)
            return True
        except ValidationError as e:
            print(f"Validation Error: {e.message}")
            path_str = ' -> '.join(str(p) for p in e.path) if e.path else 'root'
            print(f"Failed at: {path_str}")
            if e.schema_path:
                schema_path_str = ' -> '.join(str(p) for p in e.schema_path)
                print(f"Schema path: {schema_path_str}")
            return False
        except SchemaError as e:
            print(f"Schema Error: {e.message}")
            return False

    def validate_and_report(self) -> bool:
        """Load YAML and validate with formatted output."""
        print(f"Validating: {self.yaml_file}")
        print("-" * 80)
        if not self.load_yaml():
            print("\n✗ Validation failed\n")
            return False
        if self.validate():
            print("✓ Validation successful")
            print(f"\nFile '{self.yaml_file}' is valid according to the schema.\n")
            return True
        else:
            print("\n✗ Validation failed\n")
            return False


def infer_schema_path(yaml_file: Path) -> Path:
    """
    Infer the schema file path based on the YAML file location.

    Example: samples/test-case/gsma_4.4.2.2_TC.yml -> schemas/test-case/test-case.schema.json
    """
    parts = yaml_file.parts
    if 'samples' in parts:
        idx = parts.index('samples')
        category = parts[idx + 1]
        return Path(f"schemas/{category}/{category}.schema.json")
    return None


@click.command()
@click.argument('yaml_file', type=click.Path(exists=True))
@click.argument('schema_file', type=click.Path(exists=True), required=False)
def main(yaml_file: str, schema_file: str | None):
    """Validate YAML files against JSON schemas.

    Args:
        yaml_file: Path to the YAML file to validate
        schema_file: Optional path to the JSON schema file
    """
    yaml_path = Path(yaml_file)

    # Infer schema path if not provided
    if schema_file is None:
        schema_path = infer_schema_path(yaml_path)
        if not schema_path:
            click.echo(f"Error: Could not infer schema path for '{yaml_path}'", err=True)
            click.echo("Please provide schema file as second argument", err=True)
            sys.exit(1)
    else:
        schema_path = Path(schema_file)

    # Load schema
    success, schema = YamlSchemaValidator.load_json_schema(schema_path)
    if not success:
        sys.exit(1)

    # Validate YAML
    validator = YamlSchemaValidator(yaml_path, schema)
    if validator.validate_and_report():
        sys.exit(0)
    else:
        sys.exit(1)


if __name__ == "__main__":
    main()
