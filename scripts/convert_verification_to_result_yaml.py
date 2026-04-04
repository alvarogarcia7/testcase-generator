#!/usr/bin/env python3.14
"""
Convert verifier JSON output to individual YAML result files.

This script:
1. Reads verifier JSON output (ContainerReport, BatchVerificationReport, or TestCaseVerificationResult)
2. Reads execution logs and test case definitions
3. Enriches results with expected/actual outputs and results
4. Extracts each test case result
5. Writes individual YAML files with 'type: test_result' field and all verification data

The output YAML files include:
- type: test_result (mandatory field)
- schema: tcms/test-result.schema.v1.json
- test_case_id
- description
- sequences (with step_results containing Pass/Fail/NotExecuted variants)
- total_steps, passed_steps, failed_steps, not_executed_steps
- overall_pass
- requirement, item, tc (if present)

For Fail variants, the output includes:
- expected: {success, result, output}
- actual_result: exit code from execution
- actual_output: output from execution
- reason: description of why the step failed
"""

import sys
import json
import argparse
from pathlib import Path
from typing import Any, Optional

try:
    import yaml
    YAML_AVAILABLE = True
except ImportError:
    YAML_AVAILABLE = False
    print("Error: PyYAML is required. Install with: pip3 install pyyaml")
    sys.exit(1)

try:
    from jsonschema import validate, ValidationError, RefResolver
    JSONSCHEMA_AVAILABLE = True
except ImportError:
    JSONSCHEMA_AVAILABLE = False
    print("Warning: jsonschema is not available. Schema validation will be skipped.")
    print("Install with: pip3 install jsonschema")


# Global cache for loaded schemas
_SCHEMA_CACHE = {}


def load_schema(schema_path: Path) -> Optional[dict[str, Any]]:
    """
    Load a JSON schema from file with caching.
    
    Args:
        schema_path: Path to the schema file
        
    Returns:
        Schema dictionary or None if loading failed
    """
    if not JSONSCHEMA_AVAILABLE:
        return None
    
    # Check cache first
    cache_key = str(schema_path)
    if cache_key in _SCHEMA_CACHE:
        return _SCHEMA_CACHE[cache_key]
    
    try:
        with open(schema_path, 'r', encoding='utf-8') as f:
            schema = json.load(f)
        _SCHEMA_CACHE[cache_key] = schema
        return schema
    except Exception as e:
        print(f"Warning: Failed to load schema {schema_path}: {e}")
        return None


def get_schema_base_path() -> Path:
    """
    Get the base path for schemas directory.
    
    Returns:
        Path to schemas directory
    """
    # Assume schemas are in a 'schemas' directory relative to the script
    script_dir = Path(__file__).parent.parent
    return script_dir / "schemas"


def validate_input_json(data: dict[str, Any], verbose: bool = False, enabled: bool = True) -> bool:
    """
    Validate input JSON data against the appropriate schema.
    
    Supports:
    - test_verification schema (tcms/test-verification.schema.v1.json)
    - Legacy verification formats
    
    Args:
        data: The input data to validate
        verbose: Whether to print validation errors
        enabled: Whether validation is enabled
        
    Returns:
        True if validation passed or was skipped, False if validation failed
    """
    if not enabled or not JSONSCHEMA_AVAILABLE:
        return True
    
    schema_base = get_schema_base_path()
    
    # Determine which schema to use based on data structure
    schema_path = None
    
    # Check if it's a new format with type field
    if "type" in data and data.get("type") == "test_verification":
        schema_path = schema_base / "tcms" / "test-verification.schema.v1.json"
    elif "test_results" in data or "test_cases" in data or "test_case_id" in data:
        # Legacy format - use test-verification schema without envelope
        schema_path = schema_base / "tcms" / "test-verification.schema.v1.json"
    
    if not schema_path or not schema_path.exists():
        if verbose:
            print("Info: No schema found for input validation, skipping")
        return True
    
    schema = load_schema(schema_path)
    if not schema:
        return True
    
    try:
        # Create a resolver for $ref handling
        schema_uri = schema_path.as_uri()
        resolver = RefResolver(base_uri=schema_uri, referrer=schema)
        
        validate(instance=data, schema=schema, resolver=resolver)
        if verbose:
            print(f"✓ Input validation passed against {schema_path.name}")
        return True
    except ValidationError as e:
        if verbose:
            print(f"✗ Input validation failed: {e.message}")
            print(f"  Path: {'.'.join(str(p) for p in e.path)}")
        return False
    except Exception as e:
        if verbose:
            print(f"Warning: Input validation error: {e}")
        return True


def validate_output_json(data: dict[str, Any], verbose: bool = False, enabled: bool = True) -> bool:
    """
    Validate output JSON data against test-result.schema.v1.json.
    
    Args:
        data: The output data to validate
        verbose: Whether to print validation errors
        enabled: Whether validation is enabled
        
    Returns:
        True if validation passed or was skipped, False if validation failed
    """
    if not enabled or not JSONSCHEMA_AVAILABLE:
        return True
    
    schema_base = get_schema_base_path()
    schema_path = schema_base / "tcms" / "test-result.schema.v1.json"
    
    if not schema_path.exists():
        if verbose:
            print(f"Warning: Schema not found: {schema_path}")
        return True
    
    schema = load_schema(schema_path)
    if not schema:
        return True
    
    try:
        # Create a resolver for $ref handling
        schema_uri = schema_path.as_uri()
        resolver = RefResolver(base_uri=schema_uri, referrer=schema)
        
        validate(instance=data, schema=schema, resolver=resolver)
        if verbose:
            print(f"✓ Output validation passed against {schema_path.name}")
        return True
    except ValidationError as e:
        if verbose:
            print(f"✗ Output validation failed: {e.message}")
            print(f"  Path: {'.'.join(str(p) for p in e.path)}")
        return False
    except Exception as e:
        if verbose:
            print(f"Warning: Output validation error: {e}")
        return True


def load_execution_log(log_path: Path) -> dict[tuple[int, int], dict[str, Any]]:
    """
    Load execution log from JSON file.
    
    Args:
        log_path: Path to execution log JSON file
        
    Returns:
        Dictionary mapping (sequence_id, step) to execution data
    """
    try:
        with open(log_path, 'r', encoding='utf-8') as f:
            log_data = json.load(f)
        
        # Build a lookup map: (test_sequence, step) -> execution entry
        execution_map = {}
        if isinstance(log_data, list):
            for entry in log_data:
                key = (entry.get("test_sequence"), entry.get("step"))
                execution_map[key] = entry
        
        return execution_map
    except Exception as e:
        print(f"Warning: Failed to load execution log {log_path}: {e}")
        return {}


def load_test_case(testcase_path: Path) -> Optional[dict[str, Any]]:
    """
    Load test case definition from YAML file.
    
    Args:
        testcase_path: Path to test case YAML file
        
    Returns:
        Test case dictionary or None if loading failed
    """
    try:
        with open(testcase_path, 'r', encoding='utf-8') as f:
            testcase_data = yaml.safe_load(f)
        return testcase_data
    except Exception as e:
        print(f"Warning: Failed to load test case {testcase_path}: {e}")
        return None


def get_expected_from_testcase(testcase: dict[str, Any], sequence_id: int, step: int) -> Optional[dict[str, Any]]:
    """
    Extract expected values from test case definition for a specific step.
    
    Args:
        testcase: Test case dictionary
        sequence_id: Sequence ID
        step: Step number
        
    Returns:
        Expected dictionary with 'result', 'output', and optionally 'success'
    """
    if not testcase or "test_sequences" not in testcase:
        return None
    
    for seq in testcase.get("test_sequences", []):
        if seq.get("id") == sequence_id:
            for step_def in seq.get("steps", []):
                if step_def.get("step") == step:
                    expected = step_def.get("expected", {})
                    # Ensure result is a string
                    result_val = expected.get("result")
                    if result_val is not None:
                        result_val = str(result_val)
                    return {
                        "success": expected.get("success"),
                        "result": result_val if result_val is not None else "",
                        "output": expected.get("output", "")
                    }
    return None


def enrich_step_result(
    step_result: dict[str, Any],
    execution_map: dict[tuple[int, int], dict[str, Any]],
    testcase: Optional[dict[str, Any]],
    sequence_id: int
) -> dict[str, Any]:
    """
    Enrich a step result with expected/actual data from execution log and test case.
    
    Args:
        step_result: Original step result from verification
        execution_map: Execution log lookup map
        testcase: Test case definition
        sequence_id: Sequence ID for this step
        
    Returns:
        Enriched step result
    """
    # Check which variant this is
    if "Pass" in step_result:
        pass_data = step_result["Pass"]
        step_num = pass_data.get("step")
        
        # Get execution data
        exec_entry = execution_map.get((sequence_id, step_num))
        
        # For Pass variant, we can optionally add actual data if available
        # but schema doesn't require it for Pass, so we keep it simple
        return step_result
    
    elif "Fail" in step_result:
        fail_data = step_result["Fail"]
        step_num = fail_data.get("step")
        
        # Get execution data
        exec_entry = execution_map.get((sequence_id, step_num))
        
        # Get expected from test case
        expected = get_expected_from_testcase(testcase, sequence_id, step_num) if testcase else None
        
        # Enrich with execution data if available
        if exec_entry:
            # Use actual output from execution log
            if "actual_output" not in fail_data or not fail_data["actual_output"]:
                fail_data["actual_output"] = exec_entry.get("output", "")
            
            # Use actual result (exit code) from execution log
            if "actual_result" not in fail_data or not fail_data["actual_result"]:
                fail_data["actual_result"] = str(exec_entry.get("exit_code", ""))
        
        # Enrich with expected data if available and not already present
        if expected:
            if "expected" not in fail_data or not fail_data["expected"]:
                fail_data["expected"] = expected
            else:
                # Merge expected data, preferring existing values
                for key in ["success", "result", "output"]:
                    if key not in fail_data["expected"] or fail_data["expected"][key] is None:
                        if key in expected and expected[key] is not None:
                            fail_data["expected"][key] = expected[key]
        
        # Ensure all required fields are present for Fail variant
        if "expected" not in fail_data:
            fail_data["expected"] = {"result": "", "output": ""}
        if "actual_result" not in fail_data:
            fail_data["actual_result"] = ""
        if "actual_output" not in fail_data:
            fail_data["actual_output"] = ""
        if "reason" not in fail_data:
            fail_data["reason"] = "Step verification failed"
        
        return {"Fail": fail_data}
    
    elif "NotExecuted" in step_result:
        # NotExecuted steps don't need enrichment
        return step_result
    
    else:
        # Unknown variant, return as-is
        return step_result


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


def convert_test_case_to_result(
    test_case: dict[str, Any],
    execution_map: dict[tuple[int, int], dict[str, Any]],
    testcase: Optional[dict[str, Any]]
) -> dict[str, Any]:
    """
    Convert a TestCaseVerificationResult to a test_result YAML structure.
    
    Args:
        test_case: Dictionary containing test case verification result
        execution_map: Execution log lookup map
        testcase: Test case definition (optional)
        
    Returns:
        Dictionary with 'type: test_result' and all verification data
    """
    result = {
        "type": "test_result",
        "schema": "tcms/test-result.schema.v1.json",
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
        
        # Process step results - preserve enum variant structure and enrich with execution data
        for step_result in sequence.get("step_results", []):
            parsed_step = parse_step_result(step_result)
            enriched_step = enrich_step_result(
                parsed_step,
                execution_map,
                testcase,
                sequence["sequence_id"]
            )
            seq_result["step_results"].append(enriched_step)
        
        result["sequences"].append(seq_result)
    
    return result


def write_result_yaml(result: dict[str, Any], output_path: Path, verbose: bool = False, validate: bool = True) -> None:
    """
    Write a result dictionary to a YAML file.
    
    Args:
        result: Dictionary containing result data
        output_path: Path to write the YAML file
        verbose: Whether to print verbose validation messages
        validate: Whether to validate output against schema
    """
    # Validate output before writing
    if not validate_output_json(result, verbose=verbose, enabled=validate):
        print(f"Warning: Output validation failed for {output_path}, but writing anyway")
    
    # Ensure parent directory exists
    output_path.parent.mkdir(parents=True, exist_ok=True)
    
    with open(output_path, 'w', encoding='utf-8') as f:
        yaml.dump(result, f, default_flow_style=False, sort_keys=False, allow_unicode=True)
    
    print(f"✓ Wrote: {output_path}")


def write_multiple_result_files(
    test_cases: list[dict[str, Any]],
    output_dir: Path,
    execution_logs_dir: Optional[Path],
    testcases_dir: Optional[Path],
    verbose: bool = False,
    validate: bool = True
) -> int:
    """
    Write multiple test case results as individual YAML files (one file per test case).
    
    Args:
        test_cases: List of test case verification result dictionaries
        output_dir: Directory to write output YAML files
        execution_logs_dir: Directory containing execution log JSON files
        testcases_dir: Directory containing test case YAML files
        verbose: Whether to print verbose validation messages
        validate: Whether to validate output against schema
        
    Returns:
        Number of result files generated
    """
    count = 0
    for test_case in test_cases:
        test_case_id = test_case.get("test_case_id", "unknown")
        
        # Load execution log for this test case
        execution_map = {}
        if execution_logs_dir:
            log_path = execution_logs_dir / f"{test_case_id}_execution_log.json"
            if log_path.exists():
                execution_map = load_execution_log(log_path)
            else:
                print(f"Warning: Execution log not found: {log_path}")
        
        # Load test case definition
        testcase = None
        if testcases_dir:
            # Try common extensions
            for ext in ['.yaml', '.yml']:
                tc_path = testcases_dir / f"{test_case_id}{ext}"
                if tc_path.exists():
                    testcase = load_test_case(tc_path)
                    break
            if testcase is None:
                print(f"Warning: Test case file not found for: {test_case_id}")
        
        # Convert to result structure with enrichment
        result = convert_test_case_to_result(test_case, execution_map, testcase)
        
        # Generate output filename
        output_filename = f"{test_case_id}_result.yaml"
        output_path = output_dir / output_filename
        
        # Write result YAML
        write_result_yaml(result, output_path, verbose=verbose, validate=validate)
        count += 1
    
    return count


def write_single_result_file(
    test_cases: list[dict[str, Any]],
    output_path: Path,
    execution_logs_dir: Optional[Path],
    testcases_dir: Optional[Path],
    verbose: bool = False,
    validate: bool = True
) -> int:
    """
    Write multiple test case results to a single YAML file as an array.
    
    Args:
        test_cases: List of test case verification result dictionaries
        output_path: Path to write the single YAML file
        execution_logs_dir: Directory containing execution log JSON files
        testcases_dir: Directory containing test case YAML files
        verbose: Whether to print verbose validation messages
        validate: Whether to validate output against schema
        
    Returns:
        Number of result documents generated
    """
    results = []
    for test_case in test_cases:
        test_case_id = test_case.get("test_case_id", "unknown")
        
        # Load execution log for this test case
        execution_map = {}
        if execution_logs_dir:
            log_path = execution_logs_dir / f"{test_case_id}_execution_log.json"
            if log_path.exists():
                execution_map = load_execution_log(log_path)
            else:
                print(f"Warning: Execution log not found: {log_path}")
        
        # Load test case definition
        testcase = None
        if testcases_dir:
            # Try common extensions
            for ext in ['.yaml', '.yml']:
                tc_path = testcases_dir / f"{test_case_id}{ext}"
                if tc_path.exists():
                    testcase = load_test_case(tc_path)
                    break
            if testcase is None:
                print(f"Warning: Test case file not found for: {test_case_id}")
        
        # Convert to result structure with enrichment
        result = convert_test_case_to_result(test_case, execution_map, testcase)
        
        # Validate each result before adding to array
        if not validate_output_json(result, verbose=verbose, enabled=validate):
            print(f"Warning: Output validation failed for {test_case_id}, but including anyway")
        
        results.append(result)
    
    # Ensure parent directory exists
    output_path.parent.mkdir(parents=True, exist_ok=True)
    
    with open(output_path, 'w', encoding='utf-8') as f:
        yaml.dump(results, f, default_flow_style=False, sort_keys=False, allow_unicode=True)
    
    print(f"✓ Wrote: {output_path}")
    return len(results)


def process_verification_json(
    input_path: Path,
    output_path: Path,
    execution_logs_dir: Optional[Path],
    testcases_dir: Optional[Path],
    output_mode: str = "multiple",
    verbose: bool = False,
    validate: bool = True
) -> int:
    """
    Process a verification JSON file and generate result YAML files.
    
    Args:
        input_path: Path to the input JSON file
        output_path: In multiple mode: directory to write output YAML files. In single mode: file path for output YAML file.
        execution_logs_dir: Directory containing execution log JSON files
        testcases_dir: Directory containing test case YAML files
        output_mode: Output mode - "multiple" for individual files, "single" for one file
        verbose: Whether to print verbose output
        validate: Whether to validate input/output against schemas
        
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
    
    # Validate input JSON against schema
    if not validate_input_json(data, verbose=verbose, enabled=validate):
        print("Warning: Input validation failed, but continuing processing")
    
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
        return write_single_result_file(test_cases, output_path, execution_logs_dir, testcases_dir, verbose, validate)
    else:
        # In multiple mode, output_path is a directory
        return write_multiple_result_files(test_cases, output_path, execution_logs_dir, testcases_dir, verbose, validate)


def main() -> int:
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Convert verifier JSON output to YAML result files with enrichment from execution logs and test cases",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Convert to multiple individual files (default) with enrichment
  %(prog)s verification.json -o results/ --execution-logs logs/ --testcases testcases/
  %(prog)s verification.json -o results/ --multiple --execution-logs logs/ --testcases testcases/

  # Convert to a single YAML file as a plain array
  %(prog)s verification.json -o results.yaml --single --execution-logs logs/ --testcases testcases/

  # Convert with verbose output
  %(prog)s verification.json -o results/ -v --execution-logs logs/ --testcases testcases/

  # Convert from stdin
  cat verification.json | %(prog)s -o results/ --execution-logs logs/ --testcases testcases/

Input JSON format:
  The script accepts:
  1. ContainerReport: {"test_results": [...], "metadata": {...}, ...}
  2. BatchVerificationReport: {"test_cases": [...], "total_test_cases": N, ...}
  3. TestCaseVerificationResult: {"test_case_id": "...", "description": "...", ...}

Execution Logs:
  --execution-logs should point to a directory containing execution log JSON files.
  Files should be named: {test_case_id}_execution_log.json
  Each file should contain an array of execution entries with fields:
    - test_sequence, step, command, exit_code, output, result_verification_pass, output_verification_pass

Test Cases:
  --testcases should point to a directory containing test case YAML files.
  Files should be named: {test_case_id}.yaml or {test_case_id}.yml
  Each file should follow the tcms/test-case.schema.v1.json schema.

Output modes:
  --multiple (default): Each test case generates a separate file named {test_case_id}_result.yaml
                        Requires --output-dir to be a directory path
  --single: All test cases are written to a single YAML file as a plain YAML array
            Requires --output-dir to be a file path with .yaml or .yml extension

Output YAML format:
  Each result document contains:
  - type: test_result
  - schema: tcms/test-result.schema.v1.json
  - test_case_id, description
  - sequences with step_results (Pass/Fail/NotExecuted variants)
  - total_steps, passed_steps, failed_steps, not_executed_steps
  - overall_pass
  - requirement, item, tc (if present)
  
  For Fail variants, the output includes:
  - expected: {success, result, output} (from test case definition)
  - actual_result: exit code (from execution log)
  - actual_output: output (from execution log)
  - reason: description of why the step failed

Schema Validation:
  The script automatically validates:
  - Input JSON against tcms/test-verification.schema.v1.json
  - Output YAML against tcms/test-result.schema.v1.json
  
  Validation is performed when jsonschema is installed.
  Use --no-validate-schemas to disable validation.
  Use -v/--verbose to see detailed validation messages.
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
    
    parser.add_argument(
        '--execution-logs',
        type=Path,
        required=True,
        help='Directory containing execution log JSON files (required). Files should be named {test_case_id}_execution_log.json'
    )
    
    parser.add_argument(
        '--testcases',
        type=Path,
        required=True,
        help='Directory containing test case YAML files (required). Files should be named {test_case_id}.yaml or {test_case_id}.yml'
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
    
    parser.add_argument(
        '--validate-schemas',
        action='store_true',
        default=True,
        help='Validate input and output against JSON schemas (default: enabled)'
    )
    
    parser.add_argument(
        '--no-validate-schemas',
        dest='validate_schemas',
        action='store_false',
        help='Disable schema validation'
    )
    
    args = parser.parse_args()
    
    # Validate execution logs directory
    if not args.execution_logs.exists():
        print(f"✗ Error: Execution logs directory not found: {args.execution_logs}")
        return 1
    if not args.execution_logs.is_dir():
        print(f"✗ Error: Execution logs path is not a directory: {args.execution_logs}")
        return 1
    
    # Validate test cases directory
    if not args.testcases.exists():
        print(f"✗ Error: Test cases directory not found: {args.testcases}")
        return 1
    if not args.testcases.is_dir():
        print(f"✗ Error: Test cases path is not a directory: {args.testcases}")
        return 1
    
    # Determine output mode: default to multiple if neither flag is specified
    mode = "single" if args.single else "multiple"
    
    # Validate --output-dir based on mode
    if mode == "single":
        # In single mode, require output-dir to be a file path with .yaml or .yml extension
        output_path = args.output_dir
        if output_path.suffix.lower() not in ['.yaml', '.yml']:
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
            
            count = process_verification_json(
                tmp_path,
                args.output_dir,
                args.execution_logs,
                args.testcases,
                mode,
                args.verbose,
                args.validate_schemas
            )
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
        count = process_verification_json(
            args.input,
            args.output_dir,
            args.execution_logs,
            args.testcases,
            mode,
            args.verbose,
            args.validate_schemas
        )
    
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
