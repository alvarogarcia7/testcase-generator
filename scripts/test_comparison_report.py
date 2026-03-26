#!/usr/bin/env python3.14
"""
Test Comparison Report Generator

This tool generates a JSON report comparing test execution before and after crate splitting.
It analyzes cargo test output to extract:
1. Which tests were executed before and after the change
2. After the splitting, in which crate is each test
3. Total time of execution of tests before and after

Usage:
    # Generate report from saved test outputs
    python scripts/test_comparison_report.py --before before.txt --after after.txt --output report.json

    # Or run tests and generate report
    python scripts/test_comparison_report.py --run-tests --output report.json

    # With custom git refs
    python scripts/test_comparison_report.py --run-tests --before-ref main --after-ref split-binaries-into-crates --output report.json
"""

import argparse
import json
import re
import subprocess
import sys
import time
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Set, Tuple


class TestInfo:
    """Information about a single test."""
    
    def __init__(self, name: str, crate: Optional[str] = None, status: str = "ok", duration: float = 0.0):
        self.name = name
        self.crate = crate
        self.status = status
        self.duration = duration
    
    def to_dict(self) -> dict:
        return {
            "name": self.name,
            "crate": self.crate,
            "status": self.status,
            "duration_seconds": self.duration
        }


class TestExecutionReport:
    """Test execution analysis and reporting."""
    
    def __init__(self):
        self.tests: List[TestInfo] = []
        self.total_duration: float = 0.0
        self.passed_count: int = 0
        self.failed_count: int = 0
        self.ignored_count: int = 0
        self.filtered_count: int = 0
    
    def add_test(self, test: TestInfo):
        self.tests.append(test)
        if test.status == "ok":
            self.passed_count += 1
        elif test.status == "FAILED":
            self.failed_count += 1
    
    def to_dict(self) -> dict:
        return {
            "total_tests": len(self.tests),
            "passed": self.passed_count,
            "failed": self.failed_count,
            "ignored": self.ignored_count,
            "filtered": self.filtered_count,
            "total_duration_seconds": self.total_duration,
            "tests": [test.to_dict() for test in sorted(self.tests, key=lambda t: t.name)]
        }


class TestOutputParser:
    """Parser for cargo test output."""
    
    # Regex patterns for parsing cargo test output
    TEST_LINE_PATTERN = re.compile(r'test\s+([^\s]+)\s+\.\.\.\s+(ok|FAILED|ignored)')
    TEST_WITH_CRATE_PATTERN = re.compile(r'test\s+(\S+)::([^\s]+)\s+\.\.\.\s+(ok|FAILED|ignored)')
    DURATION_PATTERN = re.compile(r'(\d+)m(\d+\.\d+)s')
    SUMMARY_PATTERN = re.compile(r'test result:\s+(\w+)\.\s+(\d+)\s+passed;\s+(\d+)\s+failed;\s+(\d+)\s+ignored')
    CRATE_RUNNING_PATTERN = re.compile(r'Running.*\(target/.*/deps/(.+?)-[a-f0-9]+\)')
    
    @staticmethod
    def parse_duration(duration_str: str) -> float:
        """Parse duration string like '0m 0.43s' or '1m 23.45s' to seconds."""
        match = TestOutputParser.DURATION_PATTERN.search(duration_str)
        if match:
            minutes = int(match.group(1))
            seconds = float(match.group(2))
            return minutes * 60 + seconds
        
        # Try to parse just seconds
        try:
            return float(duration_str.replace('s', '').strip())
        except ValueError:
            return 0.0
    
    @staticmethod
    def extract_crate_name(line: str) -> Optional[str]:
        """Extract crate name from 'Running ...' line."""
        match = TestOutputParser.CRATE_RUNNING_PATTERN.search(line)
        if match:
            crate_name = match.group(1)
            # Clean up crate name (remove build hash)
            crate_name = crate_name.replace('_', '-')
            return crate_name
        return None
    
    @staticmethod
    def parse_test_output(output: str) -> TestExecutionReport:
        """Parse cargo test output and extract test information."""
        report = TestExecutionReport()
        current_crate = None
        lines = output.split('\n')
        
        for line in lines:
            # Check if this is a "Running" line indicating which crate's tests are running
            crate_name = TestOutputParser.extract_crate_name(line)
            if crate_name:
                current_crate = crate_name
                continue
            
            # Parse test result line
            match = TestOutputParser.TEST_LINE_PATTERN.search(line)
            if match:
                test_full_name = match.group(1)
                status = match.group(2)
                
                # Extract crate name from test path if present
                test_name = test_full_name
                crate = current_crate
                
                if '::' in test_full_name:
                    parts = test_full_name.split('::')
                    # First part might be the crate name
                    potential_crate = parts[0]
                    test_name = '::'.join(parts[1:]) if len(parts) > 1 else test_full_name
                    # Use the potential crate if we don't have a current crate
                    if not crate:
                        crate = potential_crate
                
                test = TestInfo(name=test_name, crate=crate, status=status)
                report.add_test(test)
            
            # Parse summary line for counts
            summary_match = TestOutputParser.SUMMARY_PATTERN.search(line)
            if summary_match:
                report.passed_count = int(summary_match.group(2))
                report.failed_count = int(summary_match.group(3))
                report.ignored_count = int(summary_match.group(4))
            
            # Extract total duration if present
            if 'finished in' in line.lower() or 'test result:' in line.lower():
                duration = TestOutputParser.parse_duration(line)
                if duration > 0:
                    report.total_duration = duration
        
        return report


class TestComparisonTool:
    """Main tool for comparing test execution before and after changes."""
    
    def __init__(self, verbose: bool = False):
        self.verbose = verbose
    
    def log(self, message: str):
        """Log a message if verbose mode is enabled."""
        if self.verbose:
            print(f"[INFO] {message}", file=sys.stderr)
    
    def run_cargo_tests(self, git_ref: Optional[str] = None) -> Tuple[str, float]:
        """
        Run cargo tests and return the output and execution time.
        
        Args:
            git_ref: Git reference to checkout before running tests (optional)
        
        Returns:
            Tuple of (test output, execution time in seconds)
        """
        if git_ref:
            self.log(f"Checking out git ref: {git_ref}")
            subprocess.run(['git', 'checkout', git_ref], check=True, capture_output=True)
        
        self.log("Running cargo test --workspace...")
        start_time = time.time()
        
        try:
            result = subprocess.run(
                ['cargo', 'test', '--workspace', '--', '--nocapture'],
                capture_output=True,
                text=True,
                timeout=600  # 10 minute timeout
            )
            output = result.stdout + "\n" + result.stderr
        except subprocess.TimeoutExpired:
            self.log("Warning: cargo test timed out after 10 minutes")
            output = "Test execution timed out"
        
        end_time = time.time()
        duration = end_time - start_time
        
        return output, duration
    
    def generate_comparison_report(
        self,
        before_output: str,
        after_output: str,
        before_duration: float,
        after_duration: float,
        before_ref: str = "before",
        after_ref: str = "after"
    ) -> dict:
        """
        Generate a comprehensive comparison report.
        
        Args:
            before_output: Test output from before the change
            after_output: Test output from after the change
            before_duration: Total execution time before
            after_duration: Total execution time after
            before_ref: Git reference or label for before state
            after_ref: Git reference or label for after state
        
        Returns:
            Dictionary containing the comparison report
        """
        self.log("Parsing before output...")
        before_report = TestOutputParser.parse_test_output(before_output)
        before_report.total_duration = before_duration
        
        self.log("Parsing after output...")
        after_report = TestOutputParser.parse_test_output(after_output)
        after_report.total_duration = after_duration
        
        # Build test name sets for comparison
        before_test_names = {test.name for test in before_report.tests}
        after_test_names = {test.name for test in after_report.tests}
        
        # Identify test changes
        new_tests = after_test_names - before_test_names
        removed_tests = before_test_names - after_test_names
        common_tests = before_test_names & after_test_names
        
        # Group tests by crate (after splitting)
        tests_by_crate: Dict[str, List[TestInfo]] = {}
        for test in after_report.tests:
            crate = test.crate or "unknown"
            if crate not in tests_by_crate:
                tests_by_crate[crate] = []
            tests_by_crate[crate].append(test)
        
        # Calculate statistics
        duration_diff = after_duration - before_duration
        duration_percent_change = (duration_diff / before_duration * 100) if before_duration > 0 else 0
        
        report = {
            "metadata": {
                "generated_at": datetime.now().isoformat(),
                "before_ref": before_ref,
                "after_ref": after_ref,
                "tool_version": "1.0.0"
            },
            "summary": {
                "tests_before": len(before_report.tests),
                "tests_after": len(after_report.tests),
                "new_tests": len(new_tests),
                "removed_tests": len(removed_tests),
                "common_tests": len(common_tests),
                "total_duration_before_seconds": before_duration,
                "total_duration_after_seconds": after_duration,
                "duration_difference_seconds": duration_diff,
                "duration_percent_change": round(duration_percent_change, 2)
            },
            "before": {
                "git_ref": before_ref,
                **before_report.to_dict()
            },
            "after": {
                "git_ref": after_ref,
                **after_report.to_dict(),
                "tests_by_crate": {
                    crate: {
                        "test_count": len(tests),
                        "tests": [test.to_dict() for test in sorted(tests, key=lambda t: t.name)]
                    }
                    for crate, tests in sorted(tests_by_crate.items())
                }
            },
            "changes": {
                "new_tests": sorted(list(new_tests)),
                "removed_tests": sorted(list(removed_tests)),
                "common_tests": sorted(list(common_tests))
            }
        }
        
        return report
    
    def save_report(self, report: dict, output_path: Path):
        """Save the report to a JSON file."""
        self.log(f"Saving report to {output_path}")
        with open(output_path, 'w') as f:
            json.dump(report, f, indent=2)
        self.log(f"Report saved successfully")


def main():
    parser = argparse.ArgumentParser(
        description="Generate a JSON report comparing test execution before and after crate splitting"
    )
    
    # Input options
    input_group = parser.add_mutually_exclusive_group(required=True)
    input_group.add_argument(
        '--run-tests',
        action='store_true',
        help='Run cargo tests for both before and after states'
    )
    input_group.add_argument(
        '--before',
        type=Path,
        help='Path to file containing test output from before the change'
    )
    
    # Additional input/output options
    parser.add_argument(
        '--after',
        type=Path,
        help='Path to file containing test output after the change (required with --before)'
    )
    parser.add_argument(
        '--output',
        type=Path,
        default=Path('test_comparison_report.json'),
        help='Output path for the JSON report (default: test_comparison_report.json)'
    )
    
    # Git reference options (for --run-tests)
    parser.add_argument(
        '--before-ref',
        default='main',
        help='Git reference for before state (default: main)'
    )
    parser.add_argument(
        '--after-ref',
        default='split-binaries-into-crates',
        help='Git reference for after state (default: split-binaries-into-crates)'
    )
    
    # Other options
    parser.add_argument(
        '--verbose',
        action='store_true',
        help='Enable verbose output'
    )
    
    args = parser.parse_args()
    
    # Validate arguments
    if not args.run_tests and not args.after:
        parser.error("--after is required when using --before")
    
    tool = TestComparisonTool(verbose=args.verbose)
    
    # Get test outputs
    if args.run_tests:
        print("Running tests before the change...")
        before_output, before_duration = tool.run_cargo_tests(args.before_ref)
        
        print("Running tests after the change...")
        after_output, after_duration = tool.run_cargo_tests(args.after_ref)
        
        before_ref = args.before_ref
        after_ref = args.after_ref
    else:
        print(f"Reading before output from {args.before}")
        with open(args.before) as f:
            before_output = f.read()
        before_duration = 0.0  # Duration not available from file
        
        print(f"Reading after output from {args.after}")
        with open(args.after) as f:
            after_output = f.read()
        after_duration = 0.0  # Duration not available from file
        
        before_ref = str(args.before)
        after_ref = str(args.after)
    
    # Generate and save report
    print("Generating comparison report...")
    report = tool.generate_comparison_report(
        before_output,
        after_output,
        before_duration,
        after_duration,
        before_ref,
        after_ref
    )
    
    tool.save_report(report, args.output)
    
    # Print summary
    print("\n" + "=" * 60)
    print("TEST COMPARISON SUMMARY")
    print("=" * 60)
    print(f"Tests before: {report['summary']['tests_before']}")
    print(f"Tests after:  {report['summary']['tests_after']}")
    print(f"New tests:    {report['summary']['new_tests']}")
    print(f"Removed:      {report['summary']['removed_tests']}")
    print(f"Common:       {report['summary']['common_tests']}")
    print()
    print(f"Duration before: {report['summary']['total_duration_before_seconds']:.2f}s")
    print(f"Duration after:  {report['summary']['total_duration_after_seconds']:.2f}s")
    print(f"Difference:      {report['summary']['duration_difference_seconds']:.2f}s "
          f"({report['summary']['duration_percent_change']:+.1f}%)")
    print()
    
    if 'tests_by_crate' in report['after']:
        print("Tests by crate after splitting:")
        for crate, data in sorted(report['after']['tests_by_crate'].items()):
            print(f"  {crate}: {data['test_count']} tests")
    
    print()
    print(f"Full report saved to: {args.output}")
    print("=" * 60)


if __name__ == '__main__':
    main()
