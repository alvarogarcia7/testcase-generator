#!/usr/bin/env python3
"""
Run convert_verification_to_tpdg.py on all acceptance test data and generate a results table.

This script:
1. Finds all test case YAML files in test-acceptance/test_cases/
2. For each test case, generates a minimal execution log (if needed)
3. Runs convert_verification_to_tpdg.py
4. Collects results and generates a table
"""

import sys
import json
import subprocess
from pathlib import Path
from typing import Dict, List, Tuple, Any
from collections import defaultdict

try:
    import yaml
    YAML_AVAILABLE = True
except ImportError:
    YAML_AVAILABLE = False
    print("Error: PyYAML is required. Install with: pip3 install pyyaml")
    sys.exit(1)


def find_test_cases(test_dir: Path) -> List[Path]:
    """Find all test case YAML files."""
    patterns = ['*.yaml', '*.yml']
    files = []
    
    for pattern in patterns:
        files.extend(test_dir.rglob(pattern))
    
    return sorted(files)


def parse_test_case(yaml_path: Path) -> Dict[str, Any]:
    """Parse a test case YAML file."""
    try:
        with open(yaml_path, 'r', encoding='utf-8') as f:
            data = yaml.safe_load(f)
        
        # Only process files with type: test_case
        if not isinstance(data, dict) or data.get('type') != 'test_case':
            return None
        
        return data
    except Exception as e:
        return None


def create_mock_execution_log(test_case: Dict[str, Any], log_path: Path) -> bool:
    """Create a minimal mock execution log for a test case."""
    try:
        log_entries = []
        
        for seq in test_case.get('test_sequences', []):
            seq_id = seq.get('id')
            
            for step in seq.get('steps', []):
                step_num = step.get('step')
                expected = step.get('expected', {})
                
                # Create a mock log entry with passing results
                entry = {
                    'test_sequence': seq_id,
                    'step': step_num,
                    'exit_code': expected.get('result', 0) if expected.get('success', True) else 1,
                    'output': 'Mock output',
                    'result_verification_pass': expected.get('success', True),
                    'output_verification_pass': True
                }
                log_entries.append(entry)
        
        # Write log file
        log_path.parent.mkdir(parents=True, exist_ok=True)
        with open(log_path, 'w', encoding='utf-8') as f:
            json.dump(log_entries, f, indent=2)
        
        return True
    except Exception as e:
        print(f"Error creating mock log for {test_case.get('id')}: {e}")
        return False


def run_conversion(test_case_dir: Path, logs_dir: Path, output_path: Path) -> Tuple[bool, str]:
    """Run the conversion script."""
    script_path = Path(__file__).parent / "convert_verification_to_tpdg.py"
    
    if not script_path.exists():
        return False, "Script not found"
    
    try:
        cmd = [
            sys.executable,
            str(script_path),
            '--test-case-dir', str(test_case_dir),
            '--logs-dir', str(logs_dir),
            '--output', str(output_path),
            '--recursive'
        ]
        
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=30
        )
        
        if result.returncode == 0:
            return True, "Success"
        else:
            return False, result.stderr.strip()[:100]
    
    except subprocess.TimeoutExpired:
        return False, "Timeout"
    except Exception as e:
        return False, str(e)[:100]


def get_category_from_path(path: Path) -> str:
    """Extract category from test case path."""
    parts = path.parts
    for i, part in enumerate(parts):
        if part == 'test_cases' and i + 1 < len(parts):
            return parts[i + 1]
    return 'unknown'


def generate_results_table(results: List[Dict[str, Any]]) -> str:
    """Generate a formatted results table."""
    # Group by category
    by_category = defaultdict(list)
    for result in results:
        by_category[result['category']].append(result)
    
    # Build table
    lines = []
    lines.append("# TPDG Conversion Results for Acceptance Tests")
    lines.append("")
    lines.append("## Summary")
    lines.append("")
    
    total_tests = len(results)
    successful = sum(1 for r in results if r['success'])
    failed = total_tests - successful
    test_cases_found = sum(1 for r in results if r['is_test_case'])
    
    lines.append(f"- **Total YAML files**: {total_tests}")
    lines.append(f"- **Test case files** (type: test_case): {test_cases_found}")
    lines.append(f"- **Conversions successful**: {successful}")
    lines.append(f"- **Conversions failed**: {failed}")
    lines.append("")
    
    # By category summary
    lines.append("## Results by Category")
    lines.append("")
    lines.append("| Category | Total | Test Cases | Successful | Failed |")
    lines.append("|----------|-------|------------|------------|--------|")
    
    for category in sorted(by_category.keys()):
        cat_results = by_category[category]
        cat_total = len(cat_results)
        cat_test_cases = sum(1 for r in cat_results if r['is_test_case'])
        cat_success = sum(1 for r in cat_results if r['success'])
        cat_failed = cat_total - cat_success
        
        lines.append(f"| {category} | {cat_total} | {cat_test_cases} | {cat_success} | {cat_failed} |")
    
    lines.append("")
    
    # Detailed results
    lines.append("## Detailed Results")
    lines.append("")
    
    for category in sorted(by_category.keys()):
        lines.append(f"### {category}")
        lines.append("")
        lines.append("| File | Type | Status | Message |")
        lines.append("|------|------|--------|---------|")
        
        for result in sorted(by_category[category], key=lambda x: x['file_name']):
            file_name = result['file_name']
            is_tc = '✓' if result['is_test_case'] else '-'
            status = '✓' if result['success'] else '✗'
            message = result.get('message', '')[:50]
            
            lines.append(f"| {file_name} | {is_tc} | {status} | {message} |")
        
        lines.append("")
    
    return '\n'.join(lines)


def main():
    """Main entry point."""
    print("=" * 80)
    print("TPDG Conversion Test Suite")
    print("=" * 80)
    print()
    
    # Setup paths
    repo_root = Path(__file__).parent.parent
    test_cases_dir = repo_root / "test-acceptance" / "test_cases"
    logs_dir = repo_root / "test-acceptance" / "mock_execution_logs"
    output_dir = repo_root / "test-acceptance" / "tpdg_conversion_output"
    
    if not test_cases_dir.exists():
        print(f"Error: Test cases directory not found: {test_cases_dir}")
        return 1
    
    # Find all test case files
    print(f"Scanning for test case files in: {test_cases_dir}")
    yaml_files = find_test_cases(test_cases_dir)
    print(f"Found {len(yaml_files)} YAML files")
    print()
    
    # Process each file
    results = []
    test_case_count = 0
    
    for i, yaml_file in enumerate(yaml_files, 1):
        relative_path = yaml_file.relative_to(test_cases_dir)
        file_name = yaml_file.name
        category = get_category_from_path(yaml_file)
        
        print(f"[{i}/{len(yaml_files)}] Processing: {relative_path}")
        
        # Parse test case
        test_case = parse_test_case(yaml_file)
        
        result = {
            'file_name': file_name,
            'relative_path': str(relative_path),
            'category': category,
            'is_test_case': test_case is not None,
            'success': False,
            'message': ''
        }
        
        if test_case is None:
            result['message'] = "Not a test case (type != test_case)"
            print(f"  ⊘ Skipped: Not a test case")
            results.append(result)
            continue
        
        test_case_count += 1
        test_case_id = test_case.get('id', 'UNKNOWN')
        
        # Create mock execution log
        log_path = logs_dir / f"{test_case_id}_execution_log.json"
        if not log_path.exists():
            if not create_mock_execution_log(test_case, log_path):
                result['message'] = "Failed to create mock log"
                print(f"  ✗ Failed to create mock log")
                results.append(result)
                continue
        
        results.append(result)
    
    # Now run the conversion on all test cases at once
    print()
    print("=" * 80)
    print(f"Running TPDG conversion on {test_case_count} test cases...")
    print("=" * 80)
    
    output_path = output_dir / "all_tests_container.yaml"
    output_path.parent.mkdir(parents=True, exist_ok=True)
    
    success, message = run_conversion(test_cases_dir, logs_dir, output_path)
    
    # Update all test case results
    for result in results:
        if result['is_test_case']:
            result['success'] = success
            result['message'] = message
    
    print()
    if success:
        print(f"✓ Conversion successful!")
        print(f"  Output: {output_path}")
        
        # Check if output file exists and has content
        if output_path.exists():
            size = output_path.stat().st_size
            print(f"  Size: {size:,} bytes")
            
            # Parse and show some stats
            try:
                with open(output_path, 'r') as f:
                    container = yaml.safe_load(f)
                
                if isinstance(container, dict):
                    test_results = container.get('test_results', [])
                    print(f"  Test results: {len(test_results)}")
                    
                    if 'metadata' in container:
                        meta = container['metadata']
                        print(f"  Total test cases: {meta.get('total_test_cases', 0)}")
                        print(f"  Passed: {meta.get('passed_test_cases', 0)}")
                        print(f"  Failed: {meta.get('failed_test_cases', 0)}")
            except Exception as e:
                print(f"  (Could not parse output: {e})")
    else:
        print(f"✗ Conversion failed: {message}")
    
    print()
    
    # Generate results table
    print("Generating results table...")
    table = generate_results_table(results)
    
    # Write table to file
    table_path = output_dir / "conversion_results.md"
    with open(table_path, 'w', encoding='utf-8') as f:
        f.write(table)
    
    print(f"✓ Results table written to: {table_path}")
    print()
    
    # Print table to stdout
    print("=" * 80)
    print(table)
    print("=" * 80)
    
    # Return appropriate exit code
    if success and test_case_count > 0:
        return 0
    else:
        return 1


if __name__ == '__main__':
    sys.exit(main())
