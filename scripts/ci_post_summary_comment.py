#!/usr/bin/env python3
"""
Post comprehensive pipeline summary as a comment on GitLab Merge Request.
"""

import os
import sys
import requests


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
        print(f"Summary comment posted successfully to MR !{mr_iid}")
    except requests.exceptions.RequestException as e:
        print(f"Failed to post comment: {e}", file=sys.stderr)


def main():
    if len(sys.argv) < 2:
        print("Usage: post_summary_comment.py <pipeline_summary.md>", file=sys.stderr)
        sys.exit(1)
    
    summary_file = sys.argv[1]
    
    try:
        with open(summary_file, 'r') as f:
            comment = f.read()
    except FileNotFoundError as e:
        print(f"Error reading summary file: {e}", file=sys.stderr)
        sys.exit(1)
    
    post_gitlab_comment(comment)


if __name__ == '__main__':
    main()
