#!/usr/bin/env python3
"""
Compare performance metrics between baseline and current versions.
"""

import sys
import json
import os
import requests


def load_performance_data(filepath: str) -> dict:
    """Load performance data from a JSON file."""
    
    try:
        with open(filepath, 'r') as f:
            return json.load(f)
    except FileNotFoundError:
        return None
    except json.JSONDecodeError as e:
        print(f"Error parsing {filepath}: {e}", file=sys.stderr)
        return None


def get_baseline_performance() -> dict:
    """Fetch baseline performance from master branch artifacts."""
    
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
            if job.get('name') == 'performance:baseline' and job.get('ref') == 'master':
                artifact_url = f"{gitlab_url}/api/v4/projects/{project_id}/jobs/{job['id']}/artifacts/baseline_perf.json"
                artifact_response = requests.get(artifact_url, headers=headers, timeout=30)
                
                if artifact_response.status_code == 200:
                    return artifact_response.json()
        
        return None
    except Exception as e:
        print(f"Error fetching baseline performance: {e}", file=sys.stderr)
        return None


def compare_metrics(baseline: dict, current: dict) -> dict:
    """Compare performance metrics and detect regressions."""
    
    comparison = {
        'regressions': [],
        'improvements': [],
        'summary': {}
    }
    
    if not baseline or not current:
        return comparison
    
    baseline_tests = baseline.get('tests', {})
    current_tests = current.get('tests', {})
    
    for test_name, current_result in current_tests.items():
        baseline_result = baseline_tests.get(test_name)
        
        if not baseline_result:
            continue
        
        if 'avg_time' in current_result and 'avg_time' in baseline_result:
            baseline_time = baseline_result['avg_time']
            current_time = current_result['avg_time']
            diff = current_time - baseline_time
            percent_change = (diff / baseline_time * 100) if baseline_time > 0 else 0
            
            metric = {
                'test': test_name,
                'baseline': baseline_time,
                'current': current_time,
                'diff': round(diff, 3),
                'percent_change': round(percent_change, 2)
            }
            
            threshold = 10.0
            
            if percent_change > threshold:
                comparison['regressions'].append(metric)
            elif percent_change < -threshold:
                comparison['improvements'].append(metric)
        
        elif 'elapsed_time' in current_result and 'elapsed_time' in baseline_result:
            baseline_time = baseline_result['elapsed_time']
            current_time = current_result['elapsed_time']
            diff = current_time - baseline_time
            percent_change = (diff / baseline_time * 100) if baseline_time > 0 else 0
            
            metric = {
                'test': test_name,
                'baseline': baseline_time,
                'current': current_time,
                'diff': round(diff, 3),
                'percent_change': round(percent_change, 2)
            }
            
            threshold = 10.0
            
            if percent_change > threshold:
                comparison['regressions'].append(metric)
            elif percent_change < -threshold:
                comparison['improvements'].append(metric)
    
    comparison['summary'] = {
        'total_regressions': len(comparison['regressions']),
        'total_improvements': len(comparison['improvements'])
    }
    
    return comparison


def main():
    if len(sys.argv) < 3:
        baseline_perf = get_baseline_performance()
        current_perf = load_performance_data('current_perf.json')
    else:
        baseline_file = sys.argv[1]
        current_file = sys.argv[2]
        
        baseline_perf = load_performance_data(baseline_file)
        current_perf = load_performance_data(current_file)
    
    comparison = compare_metrics(baseline_perf, current_perf)
    
    print(json.dumps(comparison, indent=2))


if __name__ == '__main__':
    main()
