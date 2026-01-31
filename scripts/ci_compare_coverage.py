#!/usr/bin/env python3
"""
Compare code coverage between current and baseline versions.
"""

import os
import sys
import json
import re
import requests


def parse_lcov(lcov_file: str) -> dict:
    """Parse LCOV coverage file and extract metrics."""
    
    with open(lcov_file, 'r') as f:
        content = f.read()
    
    total_lines = 0
    covered_lines = 0
    total_functions = 0
    covered_functions = 0
    
    for line in content.split('\n'):
        if line.startswith('LF:'):
            total_lines += int(line.split(':')[1])
        elif line.startswith('LH:'):
            covered_lines += int(line.split(':')[1])
        elif line.startswith('FNF:'):
            total_functions += int(line.split(':')[1])
        elif line.startswith('FNH:'):
            covered_functions += int(line.split(':')[1])
    
    line_coverage = (covered_lines / total_lines * 100) if total_lines > 0 else 0
    function_coverage = (covered_functions / total_functions * 100) if total_functions > 0 else 0
    
    return {
        'line_coverage': round(line_coverage, 2),
        'function_coverage': round(function_coverage, 2),
        'covered_lines': covered_lines,
        'total_lines': total_lines,
        'covered_functions': covered_functions,
        'total_functions': total_functions
    }


def get_baseline_coverage() -> dict:
    """Fetch baseline coverage from master branch artifacts."""
    
    gitlab_token = os.getenv('GITLAB_TOKEN') or os.getenv('CI_JOB_TOKEN')
    project_id = os.getenv('CI_PROJECT_ID')
    gitlab_url = os.getenv('CI_SERVER_URL', 'https://gitlab.com')
    
    if not all([gitlab_token, project_id]):
        return None
    
    api_url = f"{gitlab_url}/api/v4/projects/{project_id}/jobs"
    headers = {'PRIVATE-TOKEN': gitlab_token}
    
    try:
        params = {
            'scope[]': 'success',
            'per_page': 10
        }
        response = requests.get(api_url, headers=headers, params=params, timeout=30)
        response.raise_for_status()
        
        jobs = response.json()
        for job in jobs:
            if job.get('name') == 'coverage' and job.get('ref') == 'master':
                artifact_url = f"{gitlab_url}/api/v4/projects/{project_id}/jobs/{job['id']}/artifacts/lcov.info"
                artifact_response = requests.get(artifact_url, headers=headers, timeout=30)
                
                if artifact_response.status_code == 200:
                    with open('/tmp/baseline_lcov.info', 'wb') as f:
                        f.write(artifact_response.content)
                    return parse_lcov('/tmp/baseline_lcov.info')
        
        return None
    except Exception as e:
        print(f"Error fetching baseline coverage: {e}", file=sys.stderr)
        return None


def main():
    if len(sys.argv) < 2:
        print("Usage: compare_coverage.py <lcov.info>", file=sys.stderr)
        sys.exit(1)
    
    current_lcov = sys.argv[1]
    current_coverage = parse_lcov(current_lcov)
    baseline_coverage = get_baseline_coverage()
    
    result = {
        'current': current_coverage,
        'baseline': baseline_coverage,
        'diff': None
    }
    
    if baseline_coverage:
        result['diff'] = {
            'line_coverage': round(current_coverage['line_coverage'] - baseline_coverage['line_coverage'], 2),
            'function_coverage': round(current_coverage['function_coverage'] - baseline_coverage['function_coverage'], 2)
        }
    
    print(json.dumps(result, indent=2))


if __name__ == '__main__':
    main()
