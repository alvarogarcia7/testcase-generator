#!/usr/bin/env python3
"""
Post code coverage comparison as a comment on GitLab Merge Request.
"""

import os
import sys
import json
import requests


def create_coverage_comment(results: dict) -> str:
    """Create a formatted markdown comment for coverage results."""
    
    current = results['current']
    baseline = results.get('baseline')
    diff = results.get('diff')
    
    comment = "## 📊 Code Coverage Report\n\n"
    
    comment += "### Current Coverage:\n"
    comment += f"- **Lines:** {current['line_coverage']}% ({current['covered_lines']}/{current['total_lines']})\n"
    comment += f"- **Functions:** {current['function_coverage']}% ({current['covered_functions']}/{current['total_functions']})\n\n"
    
    if baseline and diff:
        line_emoji = "🔺" if diff['line_coverage'] > 0 else ("🔻" if diff['line_coverage'] < 0 else "➡️")
        func_emoji = "🔺" if diff['function_coverage'] > 0 else ("🔻" if diff['function_coverage'] < 0 else "➡️")
        
        comment += "### Comparison with Master:\n"
        comment += f"- **Lines:** {line_emoji} {diff['line_coverage']:+.2f}% (was {baseline['line_coverage']}%)\n"
        comment += f"- **Functions:** {func_emoji} {diff['function_coverage']:+.2f}% (was {baseline['function_coverage']}%)\n\n"
        
        if diff['line_coverage'] < -1.0:
            comment += "⚠️ **Warning:** Coverage decreased by more than 1%\n\n"
    
    comment += f"*View detailed coverage report in the [pipeline artifacts]({os.getenv('CI_JOB_URL')}/artifacts/browse/coverage/html)*\n"
    
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
        print(f"Coverage comment posted successfully to MR !{mr_iid}")
    except requests.exceptions.RequestException as e:
        print(f"Failed to post comment: {e}", file=sys.stderr)


def main():
    if len(sys.argv) < 2:
        print("Usage: post_coverage_comment.py <coverage_report.json>", file=sys.stderr)
        sys.exit(1)
    
    results_file = sys.argv[1]
    
    try:
        with open(results_file, 'r') as f:
            results = json.load(f)
    except (FileNotFoundError, json.JSONDecodeError) as e:
        print(f"Error reading results file: {e}", file=sys.stderr)
        sys.exit(1)
    
    comment = create_coverage_comment(results)
    post_gitlab_comment(comment)


if __name__ == '__main__':
    main()
