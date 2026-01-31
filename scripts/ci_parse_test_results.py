#!/usr/bin/env python3
"""
Parse Rust test output and generate JSON results for CI/CD reporting.
"""

import sys
import re
import json
from typing import Dict, List, Any


def parse_test_output(output_file: str) -> Dict[str, Any]:
    """Parse cargo test output and extract test results."""
    
    with open(output_file, 'r') as f:
        content = f.read()
    
    results = {
        'passed': 0,
        'failed': 0,
        'ignored': 0,
        'total': 0,
        'test_cases': [],
        'failures': [],
        'duration': '0s'
    }
    
    test_pattern = r'test ([\w:]+(?:::\w+)*) \.\.\. (\w+)'
    for match in re.finditer(test_pattern, content):
        test_name, status = match.groups()
        results['test_cases'].append({
            'name': test_name,
            'status': status
        })
        results['total'] += 1
        
        if status == 'ok':
            results['passed'] += 1
        elif status == 'FAILED':
            results['failed'] += 1
            results['failures'].append(test_name)
        elif status == 'ignored':
            results['ignored'] += 1
    
    summary_pattern = r'test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored; \d+ measured; \d+ filtered out; finished in ([\d.]+s)'
    summary_match = re.search(summary_pattern, content)
    if summary_match:
        results['duration'] = summary_match.group(5)
    
    return results


def generate_junit_xml(results: Dict[str, Any], output_file: str = 'test_results.xml'):
    """Generate JUnit XML format for GitLab CI/CD integration."""
    
    xml_lines = ['<?xml version="1.0" encoding="UTF-8"?>']
    xml_lines.append(f'<testsuites tests="{results["total"]}" failures="{results["failed"]}" time="{results["duration"]}">')
    xml_lines.append(f'  <testsuite name="rust-tests" tests="{results["total"]}" failures="{results["failed"]}" time="{results["duration"]}">')
    
    for test in results['test_cases']:
        xml_lines.append(f'    <testcase name="{test["name"]}" classname="rust" time="0">')
        if test['status'] == 'FAILED':
            xml_lines.append('      <failure message="Test failed"/>')
        elif test['status'] == 'ignored':
            xml_lines.append('      <skipped/>')
        xml_lines.append('    </testcase>')
    
    xml_lines.append('  </testsuite>')
    xml_lines.append('</testsuites>')
    
    with open(output_file, 'w') as f:
        f.write('\n'.join(xml_lines))


def main():
    if len(sys.argv) < 2:
        print("Usage: parse_test_results.py <test_output_file>", file=sys.stderr)
        sys.exit(1)
    
    output_file = sys.argv[1]
    results = parse_test_output(output_file)
    
    generate_junit_xml(results)
    
    print(json.dumps(results, indent=2))


if __name__ == '__main__':
    main()
