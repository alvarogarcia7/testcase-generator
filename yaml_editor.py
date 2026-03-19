#!/usr/bin/env python3

import sys
import yaml


def transform_result_values(data):
    if isinstance(data, dict):
        for key, value in data.items():
            if key == 'result' and isinstance(value, int):
                # Convert integer result to string
                data[key] = str(value)
            else:
                # Recursively process nested structures
                data[key] = transform_result_values(value)
    elif isinstance(data, list):
        for i, item in enumerate(data):
            data[i] = transform_result_values(item)

    return data


def process_yaml_file(filepath):
    try:
        # Read YAML file
        with open(filepath, 'r') as f:
            data = yaml.safe_load(f)

        if data is None:
            print(f"Warning: {filepath} is empty or contains only comments")
            return True

        # Transform result values
        transformed_data = transform_result_values(data)

        # Write back to the same file
        with open(filepath, 'w') as f:
            yaml.dump(transformed_data, f, default_flow_style=False, sort_keys=False, allow_unicode=True)

        print(f"✓ Processed: {filepath}")
        return True

    except Exception as e:
        print(f"✗ Error processing {filepath}: {e}", file=sys.stderr)
        return False


def main():
    if len(sys.argv) < 2:
        print("Usage: python3 yaml_editor.py <yaml_file> [<yaml_file> ...]")
        print("Converts integer 'result' values to strings in YAML files.")
        sys.exit(1)

    all_successful = True
    for filepath in sys.argv[1:]:
        if not process_yaml_file(filepath):
            all_successful = False

    sys.exit(0 if all_successful else 1)


if __name__ == '__main__':
    main()
