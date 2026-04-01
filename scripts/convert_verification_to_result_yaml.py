#!/usr/bin/env python3.14
"""
Convert verifier JSON output to individual YAML result files.

This script:
1. Reads verifier JSON output (ContainerReport, BatchVerificationReport, or TestCaseVerificationResult)
2. Extracts each test case result
3. Writes individual YAML files with 'type: result' field and all verification data

The output YAML files include:
- type: result (mandatory field)
- test_case_id
- description
- sequences (with step_results containing Pass/Fail/NotExecuted variants)
- total_steps, passed_steps, failed_steps, not_executed_steps
- overall_pass
- requirement, item, tc (if present)
"""

import sys
import json
import argparse
from pathlib import Path
from typing import Any

try:
    import yaml
    YAML_AVAILABLE = True
except ImportError:
    YAML_AVAILABLE = False
    print("Error: PyYAML is required. Install with: pip3 install pyyaml")
    sys.exit(1)


def parse_step_result(step_result: dict[str, Any]) -> dict[str, Any]:
    """
    Parse a step result from the JSON format.
    
    The JSON format uses enum variants:
    - {"Pass": {"step": 1, "description": "...", ...}}
    - {"Fail": {"step": 1, "description": "...", "expected": {...}, ...}}
    - {"NotExecuted": {"step": 1, "description": "...", ...}}
    
    Args:
        step_result: The step result dictionary from JSON
        
    Returns:
        Parsed step result dictionary with variant name and data
    """
    # Check which variant this is
    if "Pass" in step_result:
        return {"Pass": step_result["Pass"]}
    elif "Fail" in step_result:
        return {"Fail": step_result["Fail"]}
    elif "NotExecuted" in step_result:
        return {"NotExecuted": step_result["NotExecuted"]}
    else:
        # Unknown variant, return as-is
        return step_result


def convert_test_case_to_result(test_case: dict[str, Any]) -> dict[str, Any]:
    """
    Convert a TestCaseVerificationResult to a result YAML structure.
    
    Args:
        test_case: Dictionary containing test case verification result
        
    Returns:
        Dictionary with 'type: result' and all verification data
    """
    result = {
        "type": "result",
        "test_case_id": test_case["test_case_id"],
        "description": test_case["description"],
        "sequences": [],
        "total_steps": test_case["total_steps"],
        "passed_steps": test_case["passed_steps"],
        "failed_steps": test_case["failed_steps"],
        "not_executed_steps": test_case["not_executed_steps"],
        "overall_pass": test_case["overall_pass"]
    }
    
    # Add optional metadata fields if present
    if "requirement" in test_case and test_case["requirement"] is not None:
        result["requirement"] = test_case["requirement"]
    if "item" in test_case and test_case["item"] is not None:
        result["item"] = test_case["item"]
    if "tc" in test_case and test_case["tc"] is not None:
        result["tc"] = test_case["tc"]
    
    # Process sequences
    for sequence in test_case.get("sequences", []):
        seq_result = {
            "sequence_id": sequence["sequence_id"],
            "name": sequence["name"],
            "step_results": [],
            "all_steps_passed": sequence["all_steps_passed"]
        }
        
        # Add optional sequence metadata if present
        if "requirement" in sequence and sequence["requirement"] is not None:
            seq_result["requirement"] = sequence["requirement"]
        if "item" in sequence and sequence["item"] is not None:
            seq_result["item"] = sequence["item"]
        if "tc" in sequence and sequence["tc"] is not None:
            seq_result["tc"] = sequence["tc"]
        
        # Process step results - preserve enum variant structure
        for step_result in sequence.get("step_results", []):
            parsed_step = parse_step_result(step_result)
            seq_result["step_results"].append(parsed_step)
        
        result["sequences"].append(seq_result)
    
    return result


def write_result_yaml(result: dict[str, Any], output_path: Path) -> None:
    """
    Write a result dictionary to a YAML file.
    
    Args:
        result: Dictionary containing result data
        output_path: Path to write the YAML file
    """
    # Ensure parent directory exists
    output_path.parent.mkdir(parents=True, exist_ok=True)
    
    with open(output_path, 'w', encoding='utf-8') as f:
        yaml.dump(result, f, default_flow_style=False, sort_keys=False, allow_unicode=True)
    
    print(f"✓ Wrote: {output_path}")


def write_multiple_result_files(test_cases: list[dict[str, Any]], output_dir: Path) -> int:
    """
    Write multiple test case results as individual YAML files (one file per test case).
    
    Args:
        test_cases: List of test case verification result dictionaries
        output_dir: Directory to write output YAML files
        
    Returns:
        Number of result files generated
    """
    count = 0
    for test_case in test_cases:
        test_case_id = test_case.get("test_case_id", "unknown")
        
        # Convert to result structure
        result = convert_test_case_to_result(test_case)
        
        # Generate output filename
        output_filename = f"{test_case_id}_result.yaml"
        output_path = output_dir / output_filename
        
        # Write result YAML
        write_result_yaml(result, output_path)
        count += 1
    
    return count


def write_single_result_file(test_cases: list[dict[str, Any]], output_path: Path) -> int:
    """
    Write multiple test case results to a single YAML file as an array.
    
    Args:
        test_cases: List of test case verification result dictionaries
        output_path: Path to write the single YAML file
        
    Returns:
        Number of result documents generated
    """
    results = []
    for test_case in test_cases:
        result = convert_test_case_to_result(test_case)
        results.append(result)
    
    # Ensure parent directory exists
    output_path.parent.mkdir(parents=True, exist_ok=True)
    
    with open(output_path, 'w', encoding='utf-8') as f:
        yaml.dump(results, f, default_flow_style=False, sort_keys=False, allow_unicode=True)
    
    print(f"✓ Wrote: {output_path}")
    return len(results)


def process_verification_json(input_path: Path, output_path: Path, output_mode: str = "multiple", verbose: bool = False) -> int:
    """
    Process a verification JSON file and generate result YAML files.
    
    Args:
        input_path: Path to the input JSON file
        output_path: In multiple mode: directory to write output YAML files. In single mode: file path for output YAML file.
        output_mode: Output mode - "multiple" for individual files, "single" for one file
        verbose: Whether to print verbose output
        
    Returns:
        Number of result files/documents generated
    """
    if verbose:
        print(f"Reading: {input_path}")
    
    # Read input JSON
    try:
        with open(input_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
    except json.JSONDecodeError as e:
        print(f"✗ Error: Failed to parse JSON: {e}")
        return 0
    except Exception as e:
        print(f"✗ Error: Failed to read file: {e}")
        return 0
    
    # Determine the format: ContainerReport, BatchVerificationReport, or single TestCaseVerificationResult
    test_cases = []
    
    if "test_results" in data:
        # This is a ContainerReport (new container format)
        if verbose:
            print(f"Processing ContainerReport with {len(data['test_results'])} test case(s)")
        test_cases = data["test_results"]
    elif "test_cases" in data:
        # This is a BatchVerificationReport (legacy format)
        if verbose:
            print(f"Processing BatchVerificationReport with {len(data['test_cases'])} test case(s)")
        test_cases = data["test_cases"]
    elif "test_case_id" in data:
        # This is a single TestCaseVerificationResult
        if verbose:
            print("Processing single TestCaseVerificationResult")
        test_cases = [data]
    else:
        print("✗ Error: Unknown JSON structure. Expected ContainerReport, BatchVerificationReport, or TestCaseVerificationResult")
        return 0
    
    # Route to appropriate output function based on mode
    if output_mode == "single":
        # In single mode, output_path is already the full file path
        return write_single_result_file(test_cases, output_path)
    else:
        # In multiple mode, output_path is a directory
        return write_multiple_result_files(test_cases, output_path)


def main() -> int:
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Convert verifier JSON output to YAML result files",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Convert to multiple individual files (default)
  %(prog)s verification.json -o results/
  %(prog)s verification.json -o results/ --multiple

  # Convert to a single YAML file as a plain array
  %(prog)s verification.json -o results.yaml --single

  # Convert with verbose output
  %(prog)s verification.json -o results/ -v

  # Convert from stdin
  cat verification.json | %(prog)s -o results/

Input JSON format:
  The script accepts:
  1. ContainerReport: {"test_results": [...], "metadata": {...}, ...}
  2. BatchVerificationReport: {"test_cases": [...], "total_test_cases": N, ...}
  3. TestCaseVerificationResult: {"test_case_id": "...", "description": "...", ...}

Output modes:
  --multiple (default): Each test case generates a separate file named {test_case_id}_result.yaml
                        Requires --output-dir to be a directory path
  --single: All test cases are written to a single YAML file as a plain YAML array
            Requires --output-dir to be a file path with .yaml or .yml extension

Output YAML format:
  Each result document contains:
  - type: result
  - test_case_id, description
  - sequences with step_results (Pass/Fail/NotExecuted variants)
  - total_steps, passed_steps, failed_steps, not_executed_steps
  - overall_pass
  - requirement, item, tc (if present)
        """
    )
    
    parser.add_argument(
        'input',
        nargs='?',
        type=Path,
        help='Input JSON file (ContainerReport, BatchVerificationReport, or TestCaseVerificationResult). If omitted, reads from stdin.'
    )
    
    parser.add_argument(
        '-o', '--output-dir',
        type=Path,
        required=True,
        help='Output directory for result YAML files (in --multiple mode) or output file path with .yaml/.yml extension (in --single mode)'
    )
    
    # Mutually exclusive group for output mode
    output_mode = parser.add_mutually_exclusive_group()
    output_mode.add_argument(
        '--multiple',
        action='store_true',
        help='Generate multiple individual YAML files, one per test case (default)'
    )
    output_mode.add_argument(
        '--single',
        action='store_true',
        help='Generate a single YAML file containing all test cases as a plain YAML array'
    )
    
    parser.add_argument(
        '-v', '--verbose',
        action='store_true',
        help='Enable verbose output'
    )
    
    args = parser.parse_args()
    
    # Determine output mode: default to multiple if neither flag is specified
    mode = "single" if args.single else "multiple"
    
    # Validate --output-dir based on mode
    if mode == "single":
        # In single mode, require output-dir to be a file path with .yaml or .yml extension
        output_path = args.output_dir
        if not (output_path.suffix.lower() in ['.yaml', '.yml']):
            print(f"✗ Error: In --single mode, --output-dir must be a file path with .yaml or .yml extension, got: {output_path}")
            return 1
    else:
        # In multiple mode, output-dir should be a directory
        # (we don't validate it exists yet, as it will be created if needed)
        pass
    
    # Handle stdin input
    if args.input is None:
        if args.verbose:
            print("Reading from stdin...")
        
        # Read from stdin and write to a temporary file
        import tempfile
        try:
            stdin_content = sys.stdin.read()
            with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False, encoding='utf-8') as tmp:
                tmp.write(stdin_content)
                tmp_path = Path(tmp.name)
            
            count = process_verification_json(tmp_path, args.output_dir, mode, args.verbose)
            tmp_path.unlink()  # Clean up temporary file
        except Exception as e:
            print(f"✗ Error reading from stdin: {e}")
            return 1
    else:
        # Validate input file
        if not args.input.exists():
            print(f"✗ Error: Input file not found: {args.input}")
            return 1
        
        if not args.input.is_file():
            print(f"✗ Error: Input path is not a file: {args.input}")
            return 1
        
        # Process input file
        count = process_verification_json(args.input, args.output_dir, mode, args.verbose)
    
    # Summary
    if count > 0:
        if mode == "single":
            print(f"\n✓ Successfully generated single YAML array file with {count} result(s): {args.output_dir}")
        else:
            print(f"\n✓ Successfully generated {count} result YAML file(s) in: {args.output_dir}")
        return 0
    else:
        print("\n✗ No result files generated")
        return 1


if __name__ == '__main__':
    sys.exit(main())
