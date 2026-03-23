#!/usr/bin/env python3
"""
Convert verifier JSON output to TPDG-compatible container YAML.

This script:
1. Reads verifier JSON output (BatchVerificationReport or TestCaseVerificationResult)
2. Transforms step results from internally-tagged (status field) to externally-tagged format
3. Wraps results in TPDG container structure (title, project, test_date, test_results, metadata)
4. Optionally validates each test result against the schema

Input JSON format (internally-tagged):
  {"status": "pass", "step": N, "description": "..."}
  {"status": "fail", "step": N, "description": "...", "expected": {...}, ...}

Output YAML format (externally-tagged):
  Pass: {step: N, description: "..."}
  Fail: {step: N, description: "...", expected: {...}, ...}
"""

import sys
import json
import argparse
import tempfile
import subprocess
from pathlib import Path
from datetime import datetime, timezone
from typing import Dict, Any, List, Optional

try:
    import yaml
    YAML_AVAILABLE = True
except ImportError:
    YAML_AVAILABLE = False
    print("Error: PyYAML is required. Install with: pip3 install pyyaml")
    sys.exit(1)


def find_validate_yaml_binary() -> Optional[str]:
    """
    Find the validate-yaml binary, checking in order:
    1. target/release/validate-yaml
    2. target/debug/validate-yaml
    3. validate-yaml in PATH
    
    Returns:
        Path to validate-yaml binary, or None if not found
    """
    # Check release build
    release_path = Path("target/release/validate-yaml")
    if release_path.exists() and release_path.is_file():
        return str(release_path)
    
    # Check debug build
    debug_path = Path("target/debug/validate-yaml")
    if debug_path.exists() and debug_path.is_file():
        return str(debug_path)
    
    # Check PATH
    try:
        result = subprocess.run(
            ["which", "validate-yaml"],
            capture_output=True,
            text=True,
            check=False
        )
        if result.returncode == 0 and result.stdout.strip():
            return result.stdout.strip()
    except Exception:
        pass
    
    return None


def transform_step_result(step_result: Dict[str, Any]) -> Dict[str, Any]:
    """
    Transform a step result from internally-tagged to externally-tagged format.
    
    Input (internally-tagged):
      {"status": "pass", "step": 1, "description": "..."}
      {"status": "fail", "step": 1, "description": "...", "expected": {...}, ...}
      {"status": "not_executed", "step": 1, "description": "..."}
    
    Output (externally-tagged):
      {"Pass": {"step": 1, "description": "..."}}
      {"Fail": {"step": 1, "description": "...", "expected": {...}, ...}}
      {"NotExecuted": {"step": 1, "description": "..."}}
    
    Args:
        step_result: Step result dictionary with 'status' field
        
    Returns:
        Externally-tagged step result dictionary
    """
    status = step_result.get("status", "").lower()
    
    # Create a copy without the status field
    step_data = {k: v for k, v in step_result.items() if k != "status"}
    
    # Map status to externally-tagged variant
    if status == "pass":
        return {"Pass": step_data}
    elif status == "fail":
        return {"Fail": step_data}
    elif status == "not_executed":
        return {"NotExecuted": step_data}
    else:
        # Unknown status, return as-is
        return step_result


def convert_test_case_to_tpdg(test_case: Dict[str, Any]) -> Dict[str, Any]:
    """
    Convert a TestCaseVerificationResult to TPDG format.
    
    Args:
        test_case: Dictionary containing test case verification result
        
    Returns:
        Dictionary with test case in TPDG format
    """
    result = {
        "test_case_id": test_case["test_case_id"],
        "description": test_case["description"],
        "sequences": [],
        "total_steps": test_case["total_steps"],
        "passed_steps": test_case["passed_steps"],
        "failed_steps": test_case["failed_steps"],
        "not_executed_steps": test_case["not_executed_steps"],
        "overall_pass": test_case["overall_pass"]
    }
    
    # Add optional test-case level metadata fields if present
    for field in ["requirement", "item", "tc"]:
        if field in test_case and test_case[field] is not None:
            result[field] = test_case[field]
    
    # Process sequences
    for sequence in test_case.get("sequences", []):
        seq_result = {
            "sequence_id": sequence["sequence_id"],
            "name": sequence["name"],
            "step_results": [],
            "all_steps_passed": sequence["all_steps_passed"]
        }
        
        # Add optional sequence metadata if present
        for field in ["requirement", "item", "tc"]:
            if field in sequence and sequence[field] is not None:
                seq_result[field] = sequence[field]
        
        # Transform step results from internally-tagged to externally-tagged
        for step_result in sequence.get("step_results", []):
            transformed_step = transform_step_result(step_result)
            seq_result["step_results"].append(transformed_step)
        
        result["sequences"].append(seq_result)
    
    return result


def create_tpdg_container(
    test_cases: List[Dict[str, Any]],
    title: Optional[str] = None,
    project: Optional[str] = None,
    test_date: Optional[str] = None
) -> Dict[str, Any]:
    """
    Create TPDG container structure wrapping test results.
    
    Args:
        test_cases: List of test case verification results
        title: Optional title override
        project: Optional project override
        test_date: Optional test date override (ISO 8601 format)
        
    Returns:
        Dictionary with TPDG container structure
    """
    # Generate defaults if not provided
    if title is None:
        title = "Test Verification Results"
    
    if project is None:
        project = "Test Execution"
    
    if test_date is None:
        # Use current UTC time in ISO 8601 format
        test_date = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    
    # Convert test cases to TPDG format
    test_results = [convert_test_case_to_tpdg(tc) for tc in test_cases]
    
    # Calculate metadata
    total_test_cases = len(test_cases)
    passed_test_cases = sum(1 for tc in test_cases if tc.get("overall_pass", False))
    failed_test_cases = total_test_cases - passed_test_cases
    
    # Build container
    container = {
        "title": title,
        "project": project,
        "test_date": test_date,
        "test_results": test_results,
        "metadata": {
            "total_test_cases": total_test_cases,
            "passed_test_cases": passed_test_cases,
            "failed_test_cases": failed_test_cases
        }
    }
    
    return container


def validate_test_result(
    test_result: Dict[str, Any],
    validate_yaml_bin: str,
    schema_path: str,
    verbose: bool = False
) -> bool:
    """
    Validate a single test result against the schema.
    
    Args:
        test_result: Test result dictionary
        validate_yaml_bin: Path to validate-yaml binary
        schema_path: Path to schema file
        verbose: Whether to print verbose output
        
    Returns:
        True if validation passes, False otherwise
    """
    # Create temporary YAML file
    with tempfile.NamedTemporaryFile(mode='w', suffix='.yaml', delete=False) as tmp:
        yaml.dump(test_result, tmp, default_flow_style=False, sort_keys=False, allow_unicode=True)
        tmp_path = tmp.name
    
    try:
        # Run validation
        result = subprocess.run(
            [validate_yaml_bin, "--schema", schema_path, tmp_path],
            capture_output=True,
            text=True,
            check=False
        )
        
        if verbose:
            if result.stdout:
                print(result.stdout, end='')
            if result.stderr:
                print(result.stderr, end='', file=sys.stderr)
        
        return result.returncode == 0
    finally:
        # Clean up temporary file
        Path(tmp_path).unlink(missing_ok=True)


def parse_verification_json(data: Dict[str, Any]) -> List[Dict[str, Any]]:
    """
    Parse verification JSON and extract test cases.
    
    Args:
        data: Parsed JSON data
        
    Returns:
        List of test case dictionaries
    """
    test_cases = []
    
    if "test_cases" in data:
        # This is a BatchVerificationReport
        test_cases = data["test_cases"]
    elif "test_case_id" in data:
        # This is a single TestCaseVerificationResult
        test_cases = [data]
    else:
        raise ValueError("Unknown JSON structure. Expected BatchVerificationReport or TestCaseVerificationResult")
    
    return test_cases


def main() -> int:
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Convert verifier JSON output to TPDG-compatible container YAML",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Convert verification JSON to TPDG YAML
  %(prog)s --input verification.json --output results.yaml

  # Convert with custom title and project
  %(prog)s --input verification.json --output results.yaml \\
    --title "Q1 2024 Test Results" --project "SGP.22 Compliance"

  # Convert from stdin
  cat verification.json | %(prog)s --output results.yaml

  # Validate test results during conversion
  %(prog)s --input verification.json --output results.yaml --validate

Input JSON format:
  The script accepts either:
  1. BatchVerificationReport: {"test_cases": [...], ...}
  2. TestCaseVerificationResult: {"test_case_id": "...", "description": "...", ...}
  
  Step results use internally-tagged format with 'status' field:
    {"status": "pass", "step": N, ...}
    {"status": "fail", "step": N, "expected": {...}, ...}

Output YAML format:
  TPDG container with externally-tagged step results:
    Pass: {step: N, ...}
    Fail: {step: N, expected: {...}, ...}
    NotExecuted: {step: N, ...}
        """
    )
    
    parser.add_argument(
        '--input',
        type=str,
        help='Input JSON file (BatchVerificationReport or TestCaseVerificationResult). If omitted, reads from stdin.'
    )
    
    parser.add_argument(
        '--output',
        type=str,
        required=True,
        help='Output YAML file path'
    )
    
    parser.add_argument(
        '--title',
        type=str,
        help='Override title in TPDG container (default: "Test Verification Results")'
    )
    
    parser.add_argument(
        '--project',
        type=str,
        help='Override project in TPDG container (default: "Test Execution")'
    )
    
    parser.add_argument(
        '--test-date',
        type=str,
        help='Override test_date in TPDG container (ISO 8601 format, default: current UTC time)'
    )
    
    parser.add_argument(
        '--validate',
        action='store_true',
        help='Validate each test result against schema (requires validate-yaml binary)'
    )
    
    parser.add_argument(
        '--schema',
        type=str,
        default='schemas/verification-result.schema.json',
        help='Schema file for validation (default: schemas/verification-result.schema.json)'
    )
    
    parser.add_argument(
        '-v', '--verbose',
        action='store_true',
        help='Enable verbose output'
    )
    
    args = parser.parse_args()
    
    # Read input JSON
    try:
        if args.input:
            input_path = Path(args.input)
            if not input_path.exists():
                print(f"✗ Error: Input file not found: {args.input}", file=sys.stderr)
                return 1
            
            if args.verbose:
                print(f"Reading input from: {args.input}")
            
            with open(input_path, 'r') as f:
                data = json.load(f)
        else:
            if args.verbose:
                print("Reading input from stdin...")
            
            stdin_content = sys.stdin.read()
            data = json.loads(stdin_content)
    except json.JSONDecodeError as e:
        print(f"✗ Error: Failed to parse JSON: {e}", file=sys.stderr)
        return 1
    except Exception as e:
        print(f"✗ Error: Failed to read input: {e}", file=sys.stderr)
        return 1
    
    # Parse test cases
    try:
        test_cases = parse_verification_json(data)
        if args.verbose:
            print(f"Parsed {len(test_cases)} test case(s)")
    except ValueError as e:
        print(f"✗ Error: {e}", file=sys.stderr)
        return 1
    
    # Validate test results if requested
    if args.validate:
        validate_yaml_bin = find_validate_yaml_binary()
        if not validate_yaml_bin:
            print("✗ Error: validate-yaml binary not found", file=sys.stderr)
            print("  Build it with: cargo build --bin validate-yaml", file=sys.stderr)
            return 1
        
        schema_path = Path(args.schema)
        if not schema_path.exists():
            print(f"✗ Error: Schema file not found: {args.schema}", file=sys.stderr)
            return 1
        
        if args.verbose:
            print(f"Using validate-yaml: {validate_yaml_bin}")
            print(f"Using schema: {args.schema}")
        
        print(f"Validating {len(test_cases)} test result(s)...")
        validation_failed = False
        
        for i, test_case in enumerate(test_cases, 1):
            test_case_id = test_case.get("test_case_id", f"test_{i}")
            
            if args.verbose:
                print(f"\nValidating test case {i}/{len(test_cases)}: {test_case_id}")
            
            # Validate the original internally-tagged format before transformation
            is_valid = validate_test_result(
                test_case,
                validate_yaml_bin,
                str(schema_path),
                args.verbose
            )
            
            if is_valid:
                print(f"  ✓ {test_case_id}: validation passed")
            else:
                print(f"  ✗ {test_case_id}: validation failed", file=sys.stderr)
                validation_failed = True
        
        if validation_failed:
            print("\n✗ Validation failed for one or more test cases", file=sys.stderr)
            return 1
        
        print(f"\n✓ All {len(test_cases)} test result(s) validated successfully")
    
    # Create TPDG container
    container = create_tpdg_container(
        test_cases,
        title=args.title,
        project=args.project,
        test_date=args.test_date
    )
    
    # Write output YAML
    try:
        output_path = Path(args.output)
        output_path.parent.mkdir(parents=True, exist_ok=True)
        
        with open(output_path, 'w') as f:
            yaml.dump(container, f, default_flow_style=False, sort_keys=False, allow_unicode=True)
        
        print(f"✓ Wrote TPDG container YAML to: {args.output}")
        
        if args.verbose:
            print(f"  Title: {container['title']}")
            print(f"  Project: {container['project']}")
            print(f"  Test Date: {container['test_date']}")
            print(f"  Test Cases: {len(test_cases)}")
            print(f"  Passed: {container['metadata']['passed_test_cases']}")
            print(f"  Failed: {container['metadata']['failed_test_cases']}")
        
        return 0
    except Exception as e:
        print(f"✗ Error: Failed to write output: {e}", file=sys.stderr)
        return 1


if __name__ == '__main__':
    sys.exit(main())
