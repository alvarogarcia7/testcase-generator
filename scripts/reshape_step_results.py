#!/usr/bin/env python3
"""
Reshape step_results in test result container YAML files.

Transforms step_results from:
  - Pass:
      description: ...
      step: N

To:
  - step: N
    status: pass
    description: ...
"""

import sys
import yaml
import glob
from pathlib import Path


def normalize_status(status_key):
    """Convert status key to lowercase for new format."""
    status_map = {
        'Pass': 'pass',
        'Fail': 'fail',
        'NotExecuted': 'not_executed',
    }
    return status_map.get(status_key, status_key.lower())


def reshape_step_results(step_results):
    """
    Transform step_results from old format to new format.

    Old format:
      - Pass:
          description: ...
          step: N
          ...

    New format:
      - step: N
        status: pass
        description: ...
    """
    if not step_results or not isinstance(step_results, list):
        return step_results

    reshaped = []
    for item in step_results:
        if not isinstance(item, dict):
            reshaped.append(item)
            continue

        # Find the status key (Pass, Fail, NotExecuted)
        status_key = None
        status_data = None

        for key in item.keys():
            if key in ['Pass', 'Fail', 'NotExecuted']:
                status_key = key
                status_data = item[key]
                break

        if status_key and isinstance(status_data, dict):
            # Extract required fields
            step_num = status_data.get('step')
            description = status_data.get('description')

            # Create new format
            new_item = {
                'step': step_num,
                'status': normalize_status(status_key),
                'description': description,
            }
            reshaped.append(new_item)
        else:
            # If structure doesn't match, keep original
            reshaped.append(item)

    return reshaped


def reshape_file(filepath):
    """Reshape a single YAML file."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            data = yaml.safe_load(f)

        if not data or 'test_results' not in data:
            print(f"⚠ Skipping {filepath}: no test_results found")
            return False

        test_results = data['test_results']

        # Handle sequences within test_results
        if 'sequences' in test_results and isinstance(test_results['sequences'], list):
            for sequence in test_results['sequences']:
                if 'step_results' in sequence:
                    sequence['step_results'] = reshape_step_results(sequence['step_results'])

        # Add type and schema fields if not present
        if 'type' not in data:
            data['type'] = 'test_case_verification_result'
        if 'schema' not in data:
            data['schema'] = 'tcms/test-case-verification-result.schema.v1.json'

        # Write back to file with proper key ordering
        with open(filepath, 'w', encoding='utf-8') as f:
            # Build ordered output with type and schema first
            output_lines = []
            output_lines.append(f"type: {data['type']}")
            output_lines.append(f"schema: {data['schema']}")

            # Remove type and schema from data for dumping
            data_copy = {k: v for k, v in data.items() if k not in ['type', 'schema']}

            # Dump the rest and append
            rest_yaml = yaml.dump(data_copy, default_flow_style=False, sort_keys=False, allow_unicode=True)
            output_lines.append(rest_yaml)

            f.write('\n'.join(output_lines))

        print(f"✓ Updated {filepath}")
        return True

    except Exception as e:
        print(f"✗ Error processing {filepath}: {e}", file=sys.stderr)
        return False


def main():
    """Find and reshape all container YAML files."""
    # Find all container.yaml files
    container_files = glob.glob('**//*_container.yaml', recursive=True)

    if not container_files:
        print("No container YAML files found")
        return 1

    print(f"Found {len(container_files)} container YAML files")
    print()

    success_count = 0
    for filepath in sorted(container_files):
        if reshape_file(filepath):
            success_count += 1

    print()
    print(f"Summary: {success_count}/{len(container_files)} files successfully updated")

    return 0 if success_count == len(container_files) else 1


if __name__ == '__main__':
    sys.exit(main())
