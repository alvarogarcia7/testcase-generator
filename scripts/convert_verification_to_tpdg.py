#!/usr/bin/env python3.14
"""
Convert verification results to TPDG (Test Plan Data Generator) container format.

This script supports two modes:

1. Direct conversion from verifier JSON (--input):
   - Reads verifier JSON output (ContainerReport, BatchVerificationReport, or TestCaseVerificationResult)
   - Converts to TPDG container YAML format

2. Test case + execution log mode (--test-case-dir + --logs-dir):
   - Scans for test case YAML files (type: test_case)
   - Matches with execution log JSON files
   - Builds verification results from scratch
   - Converts to TPDG container YAML format

The TPDG container format includes:
- type: test_results_container
- test_results: array of test case results with Pass/Fail/NotExecuted step variants
- metadata: test execution metadata
"""

import sys
import json
import argparse
from pathlib import Path
from typing import Any, Optional
from datetime import datetime

try:
    import yaml
    YAML_AVAILABLE = True
except ImportError:
    YAML_AVAILABLE = False
    print("Error: PyYAML is required. Install with: pip3 install pyyaml")
    sys.exit(1)


def parse_test_case_yaml(yaml_path: Path) -> Optional[dict[str, Any]]:
    """
    Parse a test case YAML file and extract relevant metadata.
    
    Args:
        yaml_path: Path to the test case YAML file
        
    Returns:
        Dictionary with test case metadata or None if not a test_case
    """
    try:
        with open(yaml_path, 'r', encoding='utf-8') as f:
            data = yaml.safe_load(f)
        
        # Only process files with type: test_case
        if not isinstance(data, dict) or data.get('type') != 'test_case':
            return None
        
        # Extract metadata
        result = {
            'id': data.get('id'),
            'description': data.get('description', ''),
            'requirement': data.get('requirement'),
            'item': data.get('item'),
            'tc': data.get('tc'),
            'test_sequences': []
        }
        
        # Extract test sequences with steps and expected values
        for seq in data.get('test_sequences', []):
            seq_data = {
                'id': seq.get('id'),
                'name': seq.get('name', ''),
                'steps': []
            }
            
            for step in seq.get('steps', []):
                step_data = {
                    'step': step.get('step'),
                    'description': step.get('description', ''),
                    'expected': step.get('expected', {})
                }
                seq_data['steps'].append(step_data)
            
            result['test_sequences'].append(seq_data)
        
        return result
    except Exception as e:
        print(f"Warning: Failed to parse {yaml_path}: {e}")
        return None


def parse_execution_log(log_path: Path) -> list[dict[str, Any]]:
    """
    Parse an execution log JSON file.
    
    Args:
        log_path: Path to the execution log JSON file
        
    Returns:
        List of execution log entries
    """
    try:
        with open(log_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        if isinstance(data, list):
            return data
        return []
    except Exception as e:
        print(f"Warning: Failed to parse {log_path}: {e}")
        return []


def read_actual_log_file(logs_dir: Path, test_case_id: str, sequence_id: int, step_num: int) -> Optional[str]:
    """
    Read the actual output log file for a specific step.
    
    Args:
        logs_dir: Directory containing log files
        test_case_id: Test case ID
        sequence_id: Sequence ID
        step_num: Step number
        
    Returns:
        Content of the actual log file or None if not found
    """
    actual_log_path = logs_dir / f"{test_case_id}_sequence-{sequence_id}_step-{step_num}.actual.log"
    if actual_log_path.exists():
        try:
            return actual_log_path.read_text(encoding='utf-8')
        except Exception as e:
            print(f"Warning: Failed to read {actual_log_path}: {e}")
    return None


def build_step_result(
    test_case: dict[str, Any],
    sequence_id: int,
    step_num: int,
    execution_entry: Optional[dict[str, Any]],
    logs_dir: Optional[Path]
) -> dict[str, Any]:
    """
    Build a step result (Pass/Fail/NotExecuted) from test case and execution data.
    
    Args:
        test_case: Parsed test case metadata
        sequence_id: Sequence ID
        step_num: Step number
        execution_entry: Execution log entry for this step (or None if not executed)
        logs_dir: Directory containing actual log files (optional)
        
    Returns:
        Step result dictionary with variant (Pass/Fail/NotExecuted)
    """
    # Find the step definition in the test case
    step_def = None
    for seq in test_case['test_sequences']:
        if seq['id'] == sequence_id:
            for step in seq['steps']:
                if step['step'] == step_num:
                    step_def = step
                    break
            break
    
    if not step_def:
        # Step not found in test case definition
        return {
            "NotExecuted": {
                "step": step_num,
                "description": f"Step {step_num} (not defined in test case)"
            }
        }
    
    # If no execution entry, mark as NotExecuted
    if not execution_entry:
        return {
            "NotExecuted": {
                "step": step_num,
                "description": step_def['description']
            }
        }
    
    # Check if both verifications passed
    result_pass = execution_entry.get('result_verification_pass', False)
    output_pass = execution_entry.get('output_verification_pass', False)
    
    if result_pass and output_pass:
        # Pass
        return {
            "Pass": {
                "step": step_num,
                "description": step_def['description']
            }
        }
    else:
        # Fail - populate expected and actual values
        expected = step_def.get('expected', {})
        
        # Get actual output - prefer .actual.log file, fall back to execution log
        actual_output = None
        if logs_dir:
            actual_output = read_actual_log_file(logs_dir, test_case['id'], sequence_id, step_num)
        if actual_output is None:
            actual_output = execution_entry.get('output', '')
        
        # Determine reason for failure
        reasons = []
        if not result_pass:
            reasons.append("result verification failed")
        if not output_pass:
            reasons.append("output verification failed")
        reason = " and ".join(reasons)
        
        return {
            "Fail": {
                "step": step_num,
                "description": step_def['description'],
                "expected": {
                    "success": expected.get('success'),
                    "result": str(expected.get('result', '')),
                    "output": expected.get('output', '')
                },
                "actual_result": str(execution_entry.get('exit_code', '')),
                "actual_output": actual_output,
                "reason": reason
            }
        }


def build_verification_result_from_files(
    test_case: dict[str, Any],
    execution_log: list[dict[str, Any]],
    logs_dir: Optional[Path]
) -> dict[str, Any]:
    """
    Build a complete TestCaseVerificationResult from test case and execution log.
    
    Args:
        test_case: Parsed test case metadata
        execution_log: List of execution log entries
        logs_dir: Directory containing actual log files (optional)
        
    Returns:
        TestCaseVerificationResult dictionary
    """
    # Create execution log lookup by (sequence, step)
    exec_lookup = {}
    for entry in execution_log:
        seq_id = entry.get('test_sequence')
        step_num = entry.get('step')
        if seq_id is not None and step_num is not None:
            exec_lookup[(seq_id, step_num)] = entry
    
    sequences = []
    total_steps = 0
    passed_steps = 0
    failed_steps = 0
    not_executed_steps = 0
    
    # Process each sequence
    for seq_def in test_case['test_sequences']:
        seq_id = seq_def['id']
        step_results = []
        seq_all_passed = True
        
        # Process each step in the sequence
        for step_def in seq_def['steps']:
            step_num = step_def['step']
            exec_entry = exec_lookup.get((seq_id, step_num))
            
            step_result = build_step_result(test_case, seq_id, step_num, exec_entry, logs_dir)
            step_results.append(step_result)
            
            # Update counters
            total_steps += 1
            if "Pass" in step_result:
                passed_steps += 1
            elif "Fail" in step_result:
                failed_steps += 1
                seq_all_passed = False
            elif "NotExecuted" in step_result:
                not_executed_steps += 1
                seq_all_passed = False
        
        sequence = {
            "sequence_id": seq_id,
            "name": seq_def['name'],
            "step_results": step_results,
            "all_steps_passed": seq_all_passed
        }
        sequences.append(sequence)
    
    result = {
        "test_case_id": test_case['id'],
        "description": test_case['description'],
        "sequences": sequences,
        "total_steps": total_steps,
        "passed_steps": passed_steps,
        "failed_steps": failed_steps,
        "not_executed_steps": not_executed_steps,
        "overall_pass": failed_steps == 0 and not_executed_steps == 0
    }
    
    # Add optional metadata fields
    if test_case.get('requirement') is not None:
        result['requirement'] = test_case['requirement']
    if test_case.get('item') is not None:
        result['item'] = test_case['item']
    if test_case.get('tc') is not None:
        result['tc'] = test_case['tc']
    
    return result


def scan_test_cases(test_case_dir: Path, recursive: bool = False) -> list[Path]:
    """
    Scan directory for test case YAML files.
    
    Args:
        test_case_dir: Directory to scan
        recursive: Whether to scan subdirectories recursively
        
    Returns:
        List of test case YAML file paths
    """
    patterns = ['*.yaml', '*.yml']
    files = []
    
    for pattern in patterns:
        if recursive:
            files.extend(test_case_dir.rglob(pattern))
        else:
            files.extend(test_case_dir.glob(pattern))
    
    return sorted(files)


def process_test_case_directory(
    test_case_dir: Path,
    logs_dir: Path,
    recursive: bool = False,
    verbose: bool = False
) -> list[dict[str, Any]]:
    """
    Process test case directory and build verification results.
    
    Args:
        test_case_dir: Directory containing test case YAML files
        logs_dir: Directory containing execution log files
        recursive: Whether to scan subdirectories recursively
        verbose: Whether to print verbose output
        
    Returns:
        List of TestCaseVerificationResult dictionaries
    """
    test_results = []
    
    # Scan for test case files
    yaml_files = scan_test_cases(test_case_dir, recursive)
    
    if verbose:
        print(f"Found {len(yaml_files)} YAML files to process")
    
    for yaml_path in yaml_files:
        # Parse test case
        test_case = parse_test_case_yaml(yaml_path)
        if not test_case:
            continue
        
        test_case_id = test_case['id']
        if verbose:
            print(f"Processing test case: {test_case_id}")
        
        # Find corresponding execution log
        log_path = logs_dir / f"{test_case_id}_execution_log.json"
        if not log_path.exists():
            if verbose:
                print(f"  Warning: No execution log found at {log_path}")
            # Still process it, but all steps will be NotExecuted
            execution_log = []
        else:
            execution_log = parse_execution_log(log_path)
            if verbose:
                print(f"  Found execution log with {len(execution_log)} entries")
        
        # Build verification result
        result = build_verification_result_from_files(test_case, execution_log, logs_dir)
        test_results.append(result)
    
    return test_results


def convert_test_case_to_tpdg(test_case: dict[str, Any]) -> dict[str, Any]:
    """
    Convert a TestCaseVerificationResult to TPDG test result format.
    
    Args:
        test_case: TestCaseVerificationResult dictionary
        
    Returns:
        TPDG test result dictionary
    """
    tpdg_result = {
        "test_case_id": test_case["test_case_id"],
        "description": test_case["description"],
        "sequences": test_case["sequences"],
        "total_steps": test_case["total_steps"],
        "passed_steps": test_case["passed_steps"],
        "failed_steps": test_case["failed_steps"],
        "not_executed_steps": test_case["not_executed_steps"],
        "overall_pass": test_case["overall_pass"]
    }
    
    # Add optional metadata fields
    if "requirement" in test_case and test_case["requirement"] is not None:
        tpdg_result["requirement"] = test_case["requirement"]
    if "item" in test_case and test_case["item"] is not None:
        tpdg_result["item"] = test_case["item"]
    if "tc" in test_case and test_case["tc"] is not None:
        tpdg_result["tc"] = test_case["tc"]
    
    return tpdg_result


def create_tpdg_container(
    test_results: list[dict[str, Any]],
    title: Optional[str] = None,
    project: Optional[str] = None
) -> dict[str, Any]:
    """
    Create a TPDG container structure from test results.
    
    Args:
        test_results: List of TestCaseVerificationResult dictionaries
        title: Optional title for the container
        project: Optional project name
        
    Returns:
        TPDG container dictionary
    """
    # Convert each test case to TPDG format
    tpdg_results = [convert_test_case_to_tpdg(tc) for tc in test_results]
    
    # Calculate metadata
    total_test_cases = len(test_results)
    passed_test_cases = sum(1 for tc in test_results if tc["overall_pass"])
    failed_test_cases = total_test_cases - passed_test_cases
    
    container = {
        "type": "test_results_container",
        "schema": "tcms/testcase_results_container.schema.v1.json",
        "title": title or f"Test Results - {datetime.now().strftime('%Y-%m-%d')}",
        "project": project or "Test Suite Execution",
        "test_date": datetime.now().isoformat(),
        "test_results": tpdg_results,
        "metadata": {
            "total_test_cases": total_test_cases,
            "passed_test_cases": passed_test_cases,
            "failed_test_cases": failed_test_cases
        }
    }
    
    return container


def parse_verifier_json(data: dict[str, Any]) -> list[dict[str, Any]]:
    """
    Parse verifier JSON and extract test case results.
    
    Args:
        data: Parsed JSON data
        
    Returns:
        List of TestCaseVerificationResult dictionaries
    """
    # Determine the format
    if "test_results" in data:
        # ContainerReport format
        return data["test_results"]
    elif "test_cases" in data:
        # BatchVerificationReport format
        return data["test_cases"]
    elif "test_case_id" in data:
        # Single TestCaseVerificationResult
        return [data]
    else:
        raise ValueError("Unknown JSON structure. Expected ContainerReport, BatchVerificationReport, or TestCaseVerificationResult")


def process_verifier_json(
    input_path: Path,
    verbose: bool = False
) -> list[dict[str, Any]]:
    """
    Process verifier JSON input.
    
    Args:
        input_path: Path to input JSON file
        verbose: Whether to print verbose output
        
    Returns:
        List of TestCaseVerificationResult dictionaries
    """
    if verbose:
        print(f"Reading verifier JSON: {input_path}")
    
    try:
        with open(input_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
    except json.JSONDecodeError as e:
        raise ValueError(f"Failed to parse JSON: {e}")
    except Exception as e:
        raise ValueError(f"Failed to read file: {e}")
    
    test_results = parse_verifier_json(data)
    
    if verbose:
        print(f"Extracted {len(test_results)} test case(s)")
    
    return test_results


def write_tpdg_yaml(container: dict[str, Any], output_path: Path, verbose: bool = False) -> None:
    """
    Write TPDG container to YAML file.
    
    Args:
        container: TPDG container dictionary
        output_path: Path to output YAML file
        verbose: Whether to print verbose output
    """
    # Ensure parent directory exists
    output_path.parent.mkdir(parents=True, exist_ok=True)
    
    with open(output_path, 'w', encoding='utf-8') as f:
        yaml.dump(container, f, default_flow_style=False, sort_keys=False, allow_unicode=True)
    
    if verbose:
        print(f"✓ Wrote TPDG container to: {output_path}")


def main() -> int:
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Convert verification results to TPDG container format",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Convert from verifier JSON
  %(prog)s --input verification.json --output container.yaml

  # Build from test cases and execution logs
  %(prog)s --test-case-dir testcases/ --logs-dir logs/ --output container.yaml

  # Recursive scan with custom metadata
  %(prog)s --test-case-dir testcases/ --logs-dir logs/ --recursive \\
           --output container.yaml --title "Q1 2024 Results" --project "My Project"

Modes:
  1. Verifier JSON mode (--input):
     Reads existing verifier JSON output and converts to TPDG format
  
  2. Test case directory mode (--test-case-dir + --logs-dir):
     Scans test case YAML files, matches with execution logs, builds
     verification results, and converts to TPDG format
        """
    )
    
    # Input mode selection
    input_group = parser.add_mutually_exclusive_group(required=True)
    input_group.add_argument(
        '--input',
        type=Path,
        help='Input verifier JSON file (ContainerReport, BatchVerificationReport, or TestCaseVerificationResult)'
    )
    input_group.add_argument(
        '--test-case-dir',
        type=Path,
        help='Directory containing test case YAML files'
    )
    
    # Test case directory mode options
    parser.add_argument(
        '--logs-dir',
        type=Path,
        help='Directory containing execution log JSON files (required with --test-case-dir)'
    )
    parser.add_argument(
        '--recursive',
        action='store_true',
        help='Recursively scan subdirectories in --test-case-dir and --logs-dir'
    )
    
    # Output options
    parser.add_argument(
        '--output',
        type=Path,
        required=True,
        help='Output TPDG container YAML file'
    )
    
    # Container metadata
    parser.add_argument(
        '--title',
        type=str,
        help='Title for the TPDG container (default: auto-generated)'
    )
    parser.add_argument(
        '--project',
        type=str,
        help='Project name for the TPDG container (default: "Test Suite Execution")'
    )
    
    # General options
    parser.add_argument(
        '-v', '--verbose',
        action='store_true',
        help='Enable verbose output'
    )
    
    args = parser.parse_args()
    
    # Validate arguments
    if args.test_case_dir and not args.logs_dir:
        parser.error("--logs-dir is required when using --test-case-dir")
    
    try:
        # Process based on mode
        if args.input:
            # Verifier JSON mode
            if not args.input.exists():
                print(f"✗ Error: Input file not found: {args.input}")
                return 1
            
            test_results = process_verifier_json(args.input, args.verbose)
        else:
            # Test case directory mode
            if not args.test_case_dir.exists():
                print(f"✗ Error: Test case directory not found: {args.test_case_dir}")
                return 1
            if not args.logs_dir.exists():
                print(f"✗ Error: Logs directory not found: {args.logs_dir}")
                return 1
            
            test_results = process_test_case_directory(
                args.test_case_dir,
                args.logs_dir,
                args.recursive,
                args.verbose
            )
        
        if not test_results:
            print("✗ Warning: No test results found")
            # Still create an empty container
        
        # Create TPDG container
        container = create_tpdg_container(
            test_results,
            title=args.title,
            project=args.project
        )
        
        # Write output
        write_tpdg_yaml(container, args.output, args.verbose)
        
        # Summary
        print(f"\n✓ Successfully generated TPDG container with {len(test_results)} test case(s)")
        print(f"  Output: {args.output}")
        
        return 0
        
    except Exception as e:
        print(f"\n✗ Error: {e}")
        if args.verbose:
            import traceback
            traceback.print_exc()
        return 1


if __name__ == '__main__':
    sys.exit(main())
