#!/usr/bin/env python3
"""
Run performance benchmarks and collect metrics.
"""

import os
import sys
import json
import time
import subprocess
import argparse
from pathlib import Path


def run_benchmark(binary: str, args: list = None) -> dict:
    """Run a binary and measure its performance."""
    
    if args is None:
        args = []
    
    start_time = time.time()
    start_memory = 0
    
    try:
        result = subprocess.run(
            [binary] + args,
            capture_output=True,
            text=True,
            timeout=60
        )
        
        end_time = time.time()
        elapsed = end_time - start_time
        
        return {
            'success': result.returncode == 0,
            'elapsed_time': round(elapsed, 3),
            'return_code': result.returncode,
            'stdout_len': len(result.stdout),
            'stderr_len': len(result.stderr)
        }
    except subprocess.TimeoutExpired:
        return {
            'success': False,
            'elapsed_time': 60.0,
            'error': 'timeout'
        }
    except Exception as e:
        return {
            'success': False,
            'error': str(e)
        }


def run_validation_benchmark(validator: str, test_file: str) -> dict:
    """Benchmark a validation operation."""
    
    iterations = 10
    times = []
    
    for _ in range(iterations):
        result = run_benchmark(validator, [test_file, 'data/schema.json'])
        if result['success']:
            times.append(result['elapsed_time'])
    
    if times:
        avg_time = sum(times) / len(times)
        min_time = min(times)
        max_time = max(times)
        
        return {
            'avg_time': round(avg_time, 3),
            'min_time': round(min_time, 3),
            'max_time': round(max_time, 3),
            'iterations': len(times)
        }
    else:
        return {
            'error': 'All iterations failed'
        }


def collect_benchmarks() -> dict:
    """Collect all benchmark metrics."""
    
    benchmarks = {
        'timestamp': time.time(),
        'tests': {}
    }
    
    release_dir = Path('target/release')
    if not release_dir.exists():
        print("Release directory not found. Building...", file=sys.stderr)
        subprocess.run(['cargo', 'build', '--release'], check=True)
    
    test_files = [
        'tests/sample/gsma_4.4.2.2_TC.yml',
        'tests/sample/SGP.22_4.4.2.yaml'
    ]
    
    for test_file in test_files:
        if Path(test_file).exists():
            print(f"Benchmarking validation of {test_file}...", file=sys.stderr)
            result = run_validation_benchmark(
                str(release_dir / 'validate-yaml'),
                test_file
            )
            benchmarks['tests'][test_file] = result
    
    binaries_to_test = [
        ('validate-yaml', ['--help']),
        ('validate-json', ['--help']),
        ('test-orchestrator', ['--version'])
    ]
    
    for binary, args in binaries_to_test:
        binary_path = release_dir / binary
        if binary_path.exists():
            print(f"Benchmarking {binary} {' '.join(args)}...", file=sys.stderr)
            result = run_benchmark(str(binary_path), args)
            benchmarks['tests'][f'{binary}_{args[0]}'] = result
    
    return benchmarks


def main():
    parser = argparse.ArgumentParser(description='Run performance benchmarks')
    parser.add_argument('--output', '-o', default='performance.json',
                        help='Output file for benchmark results')
    
    args = parser.parse_args()
    
    benchmarks = collect_benchmarks()
    
    with open(args.output, 'w') as f:
        json.dump(benchmarks, f, indent=2)
    
    print(f"Benchmarks saved to {args.output}", file=sys.stderr)
    print(json.dumps(benchmarks, indent=2))


if __name__ == '__main__':
    main()
