#!/usr/bin/env python3
"""
Post performance comparison results as a comment on GitLab Merge Request.
"""

import os
import sys
import json
import requests


def create_performance_comment(results: dict) -> str:
    """Create a formatted markdown comment for performance results."""
    
    regressions = results.get('regressions', [])
    improvements = results.get('improvements', [])
    summary = results.get('summary', {})
    
    comment = "## ⚡ Performance Report\n\n"
    
    comment += "### Summary:\n"
    comment += f"- 🔴 Regressions: {summary.get('total_regressions', 0)}\n"
    comment += f"- 🟢 Improvements: {summary.get('total_improvements', 0)}\n\n"
    
    if regressions:
        comment += "### ⚠️ Performance Regressions (>10% slower):\n"
        comment += "| Test | Baseline | Current | Change |\n"
        comment += "|------|----------|---------|--------|\n"
        for reg in regressions:
            comment += f"| `{reg['test']}` | {reg['baseline']}s | {reg['current']}s | 🔴 {reg['percent_change']:+.1f}% |\n"
        comment += "\n"
    
    if improvements:
        comment += "### 🎉 Performance Improvements (>10% faster):\n"
        comment += "| Test | Baseline | Current | Change |\n"
        comment += "|------|----------|---------|--------|\n"
        for imp in improvements:
            comment += f"| `{imp['test']}` | {imp['baseline']}s | {imp['current']}s | 🟢 {imp['percent_change']:+.1f}% |\n"
        comment += "\n"
    
    if not regressions and not improvements:
        comment += "✅ No significant performance changes detected.\n\n"
    
    comment += f"*View detailed performance data in the [pipeline artifacts]({os.getenv('CI_JOB_URL')}/artifacts/browse)*\n"
    
    return comment


def post_gitlab_comment(comment: str):
    """Post comment to GitLab MR using the API."""
    
    gitlab_token = os.getenv('GITLAB_TOKEN') or os.getenv('CI_JOB_TOKEN')
    project_id = os.getenv('CI_PROJECT_ID')
    mr_iid = os.getenv('CI_MERGE_REQUEST_IID')
    gitlab_url = os.getenv('CI_SERVER_URL', 'https://gitlab.com')
    
    if not all([gitlab_token, project_id, mr_iid]):
        print("Not a merge request or missing credentials. Skipping comment.", file=sys.stderr)
        return
    
    api_url = f"{gitlab_url}/api/v4/projects/{project_id}/merge_requests/{mr_iid}/notes"
    
    headers = {
        'PRIVATE-TOKEN': gitlab_token,
        'Content-Type': 'application/json'
    }
    
    data = {
        'body': comment
    }
    
    try:
        response = requests.post(api_url, headers=headers, json=data, timeout=30)
        response.raise_for_status()
        print(f"Performance comment posted successfully to MR !{mr_iid}")
    except requests.exceptions.RequestException as e:
        print(f"Failed to post comment: {e}", file=sys.stderr)


def main():
    if len(sys.argv) < 2:
        print("Usage: post_performance_comment.py <perf_comparison.json>", file=sys.stderr)
        sys.exit(1)
    
    results_file = sys.argv[1]
    
    try:
        with open(results_file, 'r') as f:
            results = json.load(f)
    except (FileNotFoundError, json.JSONDecodeError) as e:
        print(f"Error reading results file: {e}", file=sys.stderr)
        sys.exit(1)
    
    comment = create_performance_comment(results)
    post_gitlab_comment(comment)


if __name__ == '__main__':
    main()
