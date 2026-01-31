#!/usr/bin/env bash
# Setup executable permissions for CI scripts

set -e

echo "Setting executable permissions for CI scripts..."

chmod +x scripts/ci_parse_test_results.py
chmod +x scripts/ci_post_mr_comment.py
chmod +x scripts/ci_post_clippy_comment.py
chmod +x scripts/ci_compare_coverage.py
chmod +x scripts/ci_post_coverage_comment.py
chmod +x scripts/ci_benchmark.py
chmod +x scripts/ci_compare_performance.py
chmod +x scripts/ci_post_performance_comment.py
chmod +x scripts/ci_docker_security_scan.py
chmod +x scripts/ci_generate_summary_report.py
chmod +x scripts/ci_post_summary_comment.py

echo "✅ All CI scripts are now executable"
