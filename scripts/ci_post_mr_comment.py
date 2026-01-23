#!/usr/bin/env python3
"""
Post test results as a comment on GitLab Merge Request.
"""

import os
import sys
import json
import requests
from typing import Dict, Any


def create_test_comment(results: Dict[str, Any]) -> str:
    """Create a formatted markdown comment for test results."""
    
    status_emoji = "✅" if results['failed'] == 0 else "❌"
    
    comment = f"""## {status_emoji} Test Results

**Summary:**
- ✅ Passed: {results['passed']}
- ❌ Failed: {results['failed']}
- ⏭️ Ignored: {results['ignored']}
- 📊 Total: {results['total']}
- ⏱️ Duration: {results['duration']}

"""
    
    if results['failures']:
        comment += "### Failed Tests:\n"
        for failure in results['failures']:
            comment += f"- `{failure}`\n"
        comment += "\n"
    
    comment += f"*View the full test output in the [pipeline artifacts]({os.getenv('CI_JOB_URL')}/artifacts/browse)*\n"
    
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
        print(f"Comment posted successfully to MR !{mr_iid}")
    except requests.exceptions.RequestException as e:
        print(f"Failed to post comment: {e}", file=sys.stderr)
        sys.exit(1)


def main():
    if len(sys.argv) < 2:
        print("Usage: post_mr_comment.py <test_results.json>", file=sys.stderr)
        sys.exit(1)
    
    results_file = sys.argv[1]
    
    try:
        with open(results_file, 'r') as f:
            results = json.load(f)
    except (FileNotFoundError, json.JSONDecodeError) as e:
        print(f"Error reading results file: {e}", file=sys.stderr)
        sys.exit(1)
    
    comment = create_test_comment(results)
    post_gitlab_comment(comment)


if __name__ == '__main__':
    main()
