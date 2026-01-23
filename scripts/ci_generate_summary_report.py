#!/usr/bin/env python3
"""
Generate a comprehensive summary report for the GitLab CI/CD pipeline.
"""

import os
import sys
import json
from pathlib import Path


def load_json_file(filepath: str) -> dict:
    """Safely load a JSON file."""
    try:
        if Path(filepath).exists():
            with open(filepath, 'r') as f:
                return json.load(f)
    except Exception:
        pass
    return None


def create_summary_report() -> str:
    """Generate a comprehensive markdown summary of all pipeline stages."""
    
    report = "# 🚀 Pipeline Summary Report\n\n"
    report += f"**Pipeline:** [{os.getenv('CI_PIPELINE_ID', 'N/A')}]({os.getenv('CI_PIPELINE_URL', '#')})\n"
    report += f"**Branch:** `{os.getenv('CI_COMMIT_REF_NAME', 'unknown')}`\n"
    report += f"**Commit:** `{os.getenv('CI_COMMIT_SHORT_SHA', 'N/A')}`\n\n"
    report += "---\n\n"
    
    test_results = load_json_file('test_results.json')
    if test_results:
        status_emoji = "✅" if test_results['failed'] == 0 else "❌"
        report += f"## {status_emoji} Tests\n\n"
        report += f"- **Passed:** {test_results['passed']}\n"
        report += f"- **Failed:** {test_results['failed']}\n"
        report += f"- **Ignored:** {test_results['ignored']}\n"
        report += f"- **Duration:** {test_results['duration']}\n\n"
        
        if test_results['failures']:
            report += "**Failed tests:**\n"
            for failure in test_results['failures'][:5]:
                report += f"- `{failure}`\n"
            if len(test_results['failures']) > 5:
                report += f"- *...and {len(test_results['failures']) - 5} more*\n"
            report += "\n"
    
    coverage_report = load_json_file('coverage_report.json')
    if coverage_report and coverage_report.get('current'):
        current = coverage_report['current']
        report += "## 📊 Coverage\n\n"
        report += f"- **Line Coverage:** {current['line_coverage']}%\n"
        report += f"- **Function Coverage:** {current['function_coverage']}%\n"
        
        if coverage_report.get('diff'):
            diff = coverage_report['diff']
            trend = "📈" if diff['line_coverage'] > 0 else ("📉" if diff['line_coverage'] < 0 else "➡️")
            report += f"- **Change:** {trend} {diff['line_coverage']:+.2f}%\n"
        report += "\n"
    
    perf_comparison = load_json_file('perf_comparison.json')
    if perf_comparison:
        summary = perf_comparison.get('summary', {})
        report += "## ⚡ Performance\n\n"
        report += f"- **Regressions:** {summary.get('total_regressions', 0)}\n"
        report += f"- **Improvements:** {summary.get('total_improvements', 0)}\n"
        
        if summary.get('total_regressions', 0) > 0:
            report += "\n⚠️ Performance regressions detected!\n"
        report += "\n"
    
    docker_scan = load_json_file('docker_scan_results.json')
    if docker_scan:
        summary = docker_scan.get('summary', {})
        report += "## 🐳 Docker Image\n\n"
        report += f"- **Size:** {summary.get('image_size_mb', 0)} MB\n"
        report += f"- **Security Warnings:** {summary.get('total_warnings', 0)}\n"
        report += f"- **Passed Checks:** {summary.get('total_passed', 0)}\n\n"
    
    report += "---\n\n"
    report += f"*Full pipeline details: {os.getenv('CI_PIPELINE_URL', '#')}*\n"
    
    return report


def main():
    report = create_summary_report()
    print(report)


if __name__ == '__main__':
    main()
