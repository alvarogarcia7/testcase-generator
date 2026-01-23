#!/usr/bin/env python3
"""
Post Clippy lint results as a comment on GitLab Merge Request.
"""

import os
import sys
import re
import requests


def parse_clippy_output(output_file: str) -> dict:
    """Parse Clippy output and extract warnings/errors."""
    
    with open(output_file, 'r') as f:
        content = f.read()
    
    warnings = []
    errors = []
    
    warning_pattern = r'warning: ([^\n]+)\n\s+-->\s+([^\n]+)'
    error_pattern = r'error: ([^\n]+)\n\s+-->\s+([^\n]+)'
    
    for match in re.finditer(warning_pattern, content):
        warnings.append({
            'message': match.group(1),
            'location': match.group(2)
        })
    
    for match in re.finditer(error_pattern, content):
        errors.append({
            'message': match.group(1),
            'location': match.group(2)
        })
    
    return {
        'warnings': warnings,
        'errors': errors,
        'warning_count': len(warnings),
        'error_count': len(errors)
    }


def create_clippy_comment(results: dict) -> str:
    """Create a formatted markdown comment for Clippy results."""
    
    status_emoji = "✅" if results['error_count'] == 0 else "❌"
    
    comment = f"""## {status_emoji} Clippy Lint Results

**Summary:**
- ⚠️ Warnings: {results['warning_count']}
- ❌ Errors: {results['error_count']}

"""
    
    if results['errors']:
        comment += "### Errors:\n"
        for error in results['errors'][:10]:
            comment += f"- **{error['location']}**: {error['message']}\n"
        if len(results['errors']) > 10:
            comment += f"\n*... and {len(results['errors']) - 10} more errors*\n"
        comment += "\n"
    
    if results['warnings']:
        comment += "### Warnings:\n"
        for warning in results['warnings'][:10]:
            comment += f"- **{warning['location']}**: {warning['message']}\n"
        if len(results['warnings']) > 10:
            comment += f"\n*... and {len(results['warnings']) - 10} more warnings*\n"
        comment += "\n"
    
    comment += f"*View the full Clippy output in the [pipeline artifacts]({os.getenv('CI_JOB_URL')}/artifacts/browse)*\n"
    
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
        print(f"Clippy comment posted successfully to MR !{mr_iid}")
    except requests.exceptions.RequestException as e:
        print(f"Failed to post comment: {e}", file=sys.stderr)


def main():
    if len(sys.argv) < 2:
        print("Usage: post_clippy_comment.py <clippy_output.txt>", file=sys.stderr)
        sys.exit(1)
    
    output_file = sys.argv[1]
    
    try:
        results = parse_clippy_output(output_file)
    except FileNotFoundError as e:
        print(f"Error reading output file: {e}", file=sys.stderr)
        sys.exit(1)
    
    if results['warning_count'] > 0 or results['error_count'] > 0:
        comment = create_clippy_comment(results)
        post_gitlab_comment(comment)
    else:
        print("No Clippy warnings or errors found. Skipping comment.")


if __name__ == '__main__':
    main()
