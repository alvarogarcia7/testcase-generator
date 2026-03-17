#!/usr/bin/env python3
"""
Convert JSON to YAML without requiring PyYAML library.
Properly preserves arrays and objects.
"""

import json
import sys
from pathlib import Path


def to_yaml_lines(obj, indent_level=0):
    """Convert Python object to YAML lines (list of strings)"""
    ind = "  " * indent_level
    next_ind = "  " * (indent_level + 1)

    if obj is None:
        return ["null"]
    elif isinstance(obj, bool):
        return ["true" if obj else "false"]
    elif isinstance(obj, (int, float)):
        return [str(obj)]
    elif isinstance(obj, str):
        # Handle multiline strings
        if "\n" in obj:
            return ["|-"] + [next_ind + line for line in obj.split("\n")]
        # Escape special chars
        if any(c in obj for c in ['"', "'", ':', '@', '`', '\\']):
            return [json.dumps(obj)]
        return [obj]
    elif isinstance(obj, list):
        if not obj:
            return ["[]"]

        lines = []
        for i, item in enumerate(obj):
            item_lines = to_yaml_lines(item, indent_level + 1)

            if isinstance(item, dict):
                # Dict in array
                if i == 0:
                    lines.append("-")
                    # Add dict fields indented
                    for key, val in item.items():
                        val_lines = to_yaml_lines(val, indent_level + 2)
                        if len(val_lines) == 1:
                            lines.append(f"{next_ind}{key}: {val_lines[0]}")
                        else:
                            lines.append(f"{next_ind}{key}:")
                            for vline in val_lines:
                                lines.append(f"{next_ind}  {vline}")
                else:
                    lines.append("-")
                    # Add dict fields indented
                    for key, val in item.items():
                        val_lines = to_yaml_lines(val, indent_level + 2)
                        if len(val_lines) == 1:
                            lines.append(f"{next_ind}{key}: {val_lines[0]}")
                        else:
                            lines.append(f"{next_ind}{key}:")
                            for vline in val_lines:
                                lines.append(f"{next_ind}  {vline}")
            elif isinstance(item, list):
                # Nested list
                lines.append("-")
                for vline in item_lines:
                    lines.append(f"{next_ind}{vline}")
            else:
                # Scalar value
                lines.append(f"- {item_lines[0]}")

        return lines

    elif isinstance(obj, dict):
        if not obj:
            return ["{}"]

        lines = []
        for key, val in obj.items():
            val_lines = to_yaml_lines(val, indent_level + 1)

            if len(val_lines) == 1:
                # Single line value
                lines.append(f"{key}: {val_lines[0]}")
            else:
                # Multi-line value
                lines.append(f"{key}:")
                for vline in val_lines:
                    lines.append(f"{next_ind}{vline}")

        return lines
    else:
        return [str(obj)]


def main():
    if len(sys.argv) < 2:
        print("Usage: convert_json_to_yaml_v3.py <input.json> [-o <output_dir>]")
        sys.exit(1)

    input_file = sys.argv[1]
    output_dir = "."

    # Parse arguments
    i = 2
    while i < len(sys.argv):
        if sys.argv[i] in ("-o", "--output"):
            if i + 1 < len(sys.argv):
                output_dir = sys.argv[i + 1]
                i += 2
            else:
                print("Error: -o/--output requires an argument", file=sys.stderr)
                sys.exit(1)
        else:
            i += 1

    input_path = Path(input_file)
    if not input_path.exists():
        print(f"Error: Input file not found: {input_file}", file=sys.stderr)
        sys.exit(1)

    # Read JSON
    try:
        with open(input_file, 'r') as f:
            data = json.load(f)
    except json.JSONDecodeError as e:
        print(f"Error: Invalid JSON: {e}", file=sys.stderr)
        sys.exit(1)

    # Create output directory
    output_path = Path(output_dir)
    output_path.mkdir(parents=True, exist_ok=True)

    # Handle both single test case and batch report formats
    test_cases = []
    if isinstance(data, dict):
        if "test_cases" in data:
            # Batch report format - extract individual test cases
            test_cases = data.get("test_cases", [])
        elif "test_case_id" in data:
            # Single test case format
            test_cases = [data]
        else:
            print("Error: No test_cases or test_case_id found in JSON", file=sys.stderr)
            sys.exit(1)
    else:
        print("Error: JSON must be an object", file=sys.stderr)
        sys.exit(1)

    if not test_cases:
        print("Error: No test cases found in JSON", file=sys.stderr)
        sys.exit(1)

    # Convert each test case to YAML
    for test_case in test_cases:
        if "test_case_id" not in test_case:
            print("Warning: Skipping test case without test_case_id", file=sys.stderr)
            continue

        test_case_id = test_case["test_case_id"]

        # Prepare YAML data with type: result
        yaml_data = {"type": "result"}
        yaml_data.update(test_case)

        # Convert to YAML
        yaml_lines = to_yaml_lines(yaml_data, 0)
        yaml_content = "\n".join(yaml_lines) + "\n"

        # Write YAML file
        output_file = output_path / f"{test_case_id}_result.yaml"
        try:
            with open(output_file, 'w') as f:
                f.write(yaml_content)
            print(f"Generated: {output_file}")
        except IOError as e:
            print(f"Error: Failed to write output file: {e}", file=sys.stderr)
            sys.exit(1)


if __name__ == "__main__":
    main()
