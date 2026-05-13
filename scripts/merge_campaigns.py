#!/usr/bin/env python3.14
"""
Merge verification results from multiple test campaigns.

This script:
1. Reads verification YAML files from multiple campaign folders (each has 20_verification/ subdirectory)
2. Extracts test case results from each campaign
3. Merges results by test_case_id using the specified merge strategy
4. Outputs a single test-results-container YAML with merged results

Merge strategies:
- or: failure OR success = success (any campaign passes => passes)
- and: failure AND success = failure (all campaigns must pass)
- oldest: use result from campaign with earliest execution timestamp
- newest: use result from campaign with latest execution timestamp

For oldest/newest: timestamps are read from execution logs in 10_test_results/execution_logs/*.json
"""

import sys
import json
import argparse
from pathlib import Path
from typing import Any, Optional
from datetime import datetime, timezone
from collections import defaultdict

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


def get_schema_base_path() -> Path:
    """Get the base path for schemas directory."""
    script_dir = Path(__file__).parent.parent
    return script_dir / "schemas"


def load_schema(schema_path: Path) -> Optional[dict[str, Any]]:
    """Load a JSON schema from file with caching."""
    if not JSONSCHEMA_AVAILABLE:
        return None

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


def validate_output_container(data: dict[str, Any], verbose: bool = False) -> bool:
    """Validate output container YAML against test-results-container schema."""
    if not JSONSCHEMA_AVAILABLE:
        return True

    schema_base = get_schema_base_path()
    schema_path = schema_base / "tcms" / "test-results-container.schema.v1.json"

    if not schema_path.exists():
        if verbose:
            print(f"Warning: Schema not found: {schema_path}")
        return True

    schema = load_schema(schema_path)
    if not schema:
        return True

    try:
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


def load_verification_from_campaign(campaign_dir: Path, verbose: bool = False) -> dict[str, dict[str, Any]]:
    """
    Load all verification YAML files from a campaign's 20_verification/ directory.

    Args:
        campaign_dir: Path to campaign root directory
        verbose: Whether to print debug information

    Returns:
        Dictionary mapping test_case_id -> verification result
    """
    verification_dir = campaign_dir / "20_verification"
    results = {}

    if not verification_dir.exists():
        if verbose:
            print(f"Warning: No 20_verification directory in {campaign_dir}")
        return results

    # Find all *_verification.yaml files
    for yaml_file in verification_dir.glob("*_verification.yaml"):
        try:
            with open(yaml_file, 'r', encoding='utf-8') as f:
                data = yaml.safe_load(f)

            if data and isinstance(data, dict):
                test_case_id = data.get("test_case_id")
                if test_case_id:
                    results[test_case_id] = data
                    if verbose:
                        print(f"  Loaded {test_case_id} from {yaml_file.name}")
        except Exception as e:
            print(f"Warning: Failed to load {yaml_file}: {e}")

    return results


def get_execution_timestamp(campaign_dir: Path, verbose: bool = False) -> Optional[datetime]:
    """
    Get the execution timestamp from a campaign's execution logs.

    Args:
        campaign_dir: Path to campaign root directory
        verbose: Whether to print debug information

    Returns:
        datetime of execution (earliest timestamp from logs) or None if not available
    """
    logs_dir = campaign_dir / "10_test_results" / "execution_logs"

    if not logs_dir.exists():
        if verbose:
            print(f"Warning: No execution logs in {campaign_dir}")
        return None

    earliest_timestamp = None

    # Check all JSON execution log files
    for log_file in logs_dir.glob("*.json"):
        try:
            with open(log_file, 'r', encoding='utf-8') as f:
                log_data = json.load(f)

            if isinstance(log_data, list):
                for entry in log_data:
                    timestamp_str = entry.get("timestamp")
                    if timestamp_str:
                        try:
                            # Parse ISO 8601 timestamp
                            # Handle both with and without timezone
                            if "+" in timestamp_str or timestamp_str.endswith("Z"):
                                ts = datetime.fromisoformat(timestamp_str.replace("Z", "+00:00"))
                            else:
                                ts = datetime.fromisoformat(timestamp_str)

                            if earliest_timestamp is None or ts < earliest_timestamp:
                                earliest_timestamp = ts
                        except Exception:
                            pass
        except Exception as e:
            if verbose:
                print(f"Warning: Failed to read execution log {log_file}: {e}")

    if earliest_timestamp is None and logs_dir.exists():
        # Fallback to file modification time
        log_files = list(logs_dir.glob("*.json"))
        if log_files:
            earliest_timestamp = datetime.fromtimestamp(min(f.stat().st_mtime for f in log_files))

    return earliest_timestamp


def merge_results_or(results_per_testcase: list[dict[str, Any]]) -> dict[str, Any]:
    """Merge strategy: or - any pass = overall pass."""
    if not results_per_testcase:
        return {}

    base = results_per_testcase[0].copy()
    overall_pass = any(r.get("overall_pass", False) for r in results_per_testcase)
    base["overall_pass"] = overall_pass

    # Aggregate step counts from all results
    total_steps = sum(r.get("total_steps", 0) for r in results_per_testcase)
    passed_steps = sum(r.get("passed_steps", 0) for r in results_per_testcase)
    failed_steps = sum(r.get("failed_steps", 0) for r in results_per_testcase)
    not_executed_steps = sum(r.get("not_executed_steps", 0) for r in results_per_testcase)

    base["total_steps"] = total_steps
    base["passed_steps"] = passed_steps
    base["failed_steps"] = failed_steps
    base["not_executed_steps"] = not_executed_steps

    return base


def merge_results_and(results_per_testcase: list[dict[str, Any]]) -> dict[str, Any]:
    """Merge strategy: and - all must pass = overall pass."""
    if not results_per_testcase:
        return {}

    base = results_per_testcase[0].copy()
    overall_pass = all(r.get("overall_pass", False) for r in results_per_testcase)
    base["overall_pass"] = overall_pass

    # Aggregate step counts from all results
    total_steps = sum(r.get("total_steps", 0) for r in results_per_testcase)
    passed_steps = sum(r.get("passed_steps", 0) for r in results_per_testcase)
    failed_steps = sum(r.get("failed_steps", 0) for r in results_per_testcase)
    not_executed_steps = sum(r.get("not_executed_steps", 0) for r in results_per_testcase)

    base["total_steps"] = total_steps
    base["passed_steps"] = passed_steps
    base["failed_steps"] = failed_steps
    base["not_executed_steps"] = not_executed_steps

    return base


def merge_results_oldest(
    results_per_testcase: list[tuple[dict[str, Any], datetime]]
) -> dict[str, Any]:
    """Merge strategy: oldest - use result from campaign with earliest timestamp."""
    if not results_per_testcase:
        return {}

    # Sort by timestamp, oldest first
    sorted_results = sorted(results_per_testcase, key=lambda x: x[1])
    return sorted_results[0][0].copy()


def merge_results_newest(
    results_per_testcase: list[tuple[dict[str, Any], datetime]]
) -> dict[str, Any]:
    """Merge strategy: newest - use result from campaign with latest timestamp."""
    if not results_per_testcase:
        return {}

    # Sort by timestamp, newest last
    sorted_results = sorted(results_per_testcase, key=lambda x: x[1])
    return sorted_results[-1][0].copy()


def merge_campaigns(
    campaign_dirs: list[Path],
    merge_strategy: str,
    verbose: bool = False
) -> dict[str, Any]:
    """
    Merge verification results from multiple campaigns.

    Args:
        campaign_dirs: List of campaign directory paths
        merge_strategy: One of: or, and, oldest, newest
        verbose: Whether to print debug information

    Returns:
        Merged container YAML as dictionary
    """
    if verbose:
        print(f"Loading verification results from {len(campaign_dirs)} campaign(s)...")

    # Load results from each campaign
    campaign_data = {}  # campaign_dir -> (results, timestamp)

    for campaign_dir in campaign_dirs:
        campaign_dir = Path(campaign_dir).resolve()

        if not campaign_dir.exists():
            print(f"Error: Campaign directory not found: {campaign_dir}")
            sys.exit(1)

        if verbose:
            print(f"\nProcessing campaign: {campaign_dir}")

        results = load_verification_from_campaign(campaign_dir, verbose)
        timestamp = get_execution_timestamp(campaign_dir, verbose)

        if not results:
            print(f"Warning: No verification results found in {campaign_dir}")

        campaign_data[campaign_dir] = (results, timestamp)

    # Group results by test_case_id
    results_by_testcase = defaultdict(list)

    for campaign_dir, (results, timestamp) in campaign_data.items():
        for test_case_id, result in results.items():
            if merge_strategy in ["oldest", "newest"]:
                # Store tuple of (result, timestamp) for time-based strategies
                if timestamp:
                    results_by_testcase[test_case_id].append((result, timestamp))
                else:
                    # Fallback: use file modification time
                    result_file = campaign_dir / "20_verification" / f"{test_case_id}_verification.yaml"
                    if result_file.exists():
                        mtime = datetime.fromtimestamp(result_file.stat().st_mtime)
                        results_by_testcase[test_case_id].append((result, mtime))
            else:
                results_by_testcase[test_case_id].append(result)

    if verbose:
        print(f"\nMerging results for {len(results_by_testcase)} test case(s)...")

    # Apply merge strategy
    merged_results = []

    for test_case_id in sorted(results_by_testcase.keys()):
        results_list = results_by_testcase[test_case_id]

        if merge_strategy == "or":
            merged = merge_results_or(results_list)
        elif merge_strategy == "and":
            merged = merge_results_and(results_list)
        elif merge_strategy == "oldest":
            merged = merge_results_oldest(results_list)
        elif merge_strategy == "newest":
            merged = merge_results_newest(results_list)
        else:
            print(f"Error: Unknown merge strategy: {merge_strategy}")
            sys.exit(1)

        merged_results.append(merged)

    # Create container YAML
    total_tests = len(merged_results)
    passed_tests = sum(1 for r in merged_results if r.get("overall_pass", False))
    failed_tests = total_tests - passed_tests

    container = {
        "type": "test_results_container",
        "schema": "tcms/test-results-container.schema.v1.json",
        "title": f"Merged Test Results ({merge_strategy} strategy)",
        "project": "Test Case Manager - Merged Campaign Results",
        "test_date": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
        "test_results": merged_results,
        "metadata": {
            "execution_duration": 0.0,
            "total_test_cases": total_tests,
            "passed_test_cases": passed_tests,
            "failed_test_cases": failed_tests,
        }
    }

    return container


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Merge verification results from multiple test campaigns"
    )
    parser.add_argument(
        "--campaigns",
        nargs="+",
        required=True,
        help="Campaign directories to merge"
    )
    parser.add_argument(
        "--merge-strategy",
        choices=["or", "and", "oldest", "newest"],
        required=True,
        help="Merge strategy: or (any pass), and (all pass), oldest, or newest"
    )
    parser.add_argument(
        "--output",
        default=None,
        help="Output file path (default: stdout)"
    )
    parser.add_argument(
        "-v", "--verbose",
        action="store_true",
        help="Enable verbose output"
    )

    args = parser.parse_args()

    # Merge campaigns
    merged = merge_campaigns(args.campaigns, args.merge_strategy, args.verbose)

    # Validate output
    if args.verbose:
        print("\nValidating merged result...")
    validate_output_container(merged, args.verbose)

    # Write output
    yaml_str = yaml.dump(merged, default_flow_style=False, sort_keys=False)

    if args.output:
        output_path = Path(args.output)
        output_path.parent.mkdir(parents=True, exist_ok=True)
        with open(output_path, 'w', encoding='utf-8') as f:
            f.write(yaml_str)
        if args.verbose:
            print(f"\n✓ Merged results written to: {output_path}")
    else:
        print(yaml_str)


if __name__ == "__main__":
    main()
