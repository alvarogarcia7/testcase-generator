#!/usr/bin/env python3.14
"""
Unit tests for merge_campaigns.py script.

Tests all merge strategies and edge cases:
- or strategy: any pass = overall pass
- and strategy: all must pass
- oldest strategy: uses campaign with earliest timestamp
- newest strategy: uses campaign with latest timestamp
"""

import unittest
import tempfile
import json
from pathlib import Path
from datetime import datetime, timedelta, timezone

try:
    import yaml
except ImportError:
    print("Error: PyYAML is required for tests")
    exit(1)

# Import the merge_campaigns module
import sys
sys.path.insert(0, str(Path(__file__).parent))
from merge_campaigns import (
    merge_results_or,
    merge_results_and,
    merge_results_oldest,
    merge_results_newest,
    merge_campaigns,
    load_verification_from_campaign,
    get_execution_timestamp,
)


class TestMergeStrategies(unittest.TestCase):
    """Test individual merge strategies."""

    def setUp(self):
        """Set up test fixtures."""
        # Result that passes
        self.pass_result = {
            "test_case_id": "TC_PASS",
            "description": "Passing test",
            "overall_pass": True,
            "total_steps": 3,
            "passed_steps": 3,
            "failed_steps": 0,
            "not_executed_steps": 0,
        }

        # Result that fails
        self.fail_result = {
            "test_case_id": "TC_FAIL",
            "description": "Failing test",
            "overall_pass": False,
            "total_steps": 3,
            "passed_steps": 1,
            "failed_steps": 2,
            "not_executed_steps": 0,
        }

    def test_or_strategy_all_pass(self):
        """Test OR strategy when all campaigns pass."""
        results = [self.pass_result, self.pass_result]
        merged = merge_results_or(results)
        self.assertTrue(merged["overall_pass"])

    def test_or_strategy_all_fail(self):
        """Test OR strategy when all campaigns fail."""
        results = [self.fail_result, self.fail_result]
        merged = merge_results_or(results)
        self.assertFalse(merged["overall_pass"])

    def test_or_strategy_mixed(self):
        """Test OR strategy with mixed pass/fail (should pass)."""
        results = [self.pass_result, self.fail_result]
        merged = merge_results_or(results)
        self.assertTrue(merged["overall_pass"])

    def test_or_strategy_aggregates_steps(self):
        """Test OR strategy aggregates step counts."""
        results = [self.pass_result, self.fail_result]
        merged = merge_results_or(results)
        self.assertEqual(merged["total_steps"], 6)
        self.assertEqual(merged["passed_steps"], 4)
        self.assertEqual(merged["failed_steps"], 2)

    def test_and_strategy_all_pass(self):
        """Test AND strategy when all campaigns pass."""
        results = [self.pass_result, self.pass_result]
        merged = merge_results_and(results)
        self.assertTrue(merged["overall_pass"])

    def test_and_strategy_all_fail(self):
        """Test AND strategy when all campaigns fail."""
        results = [self.fail_result, self.fail_result]
        merged = merge_results_and(results)
        self.assertFalse(merged["overall_pass"])

    def test_and_strategy_mixed(self):
        """Test AND strategy with mixed pass/fail (should fail)."""
        results = [self.pass_result, self.fail_result]
        merged = merge_results_and(results)
        self.assertFalse(merged["overall_pass"])

    def test_oldest_strategy(self):
        """Test OLDEST strategy uses earliest timestamp."""
        now = datetime.now(timezone.utc)
        old_time = now - timedelta(hours=1)

        results_with_time = [
            (self.pass_result, now),
            (self.fail_result, old_time),
        ]

        merged = merge_results_oldest(results_with_time)
        # Should use the oldest (fail_result)
        self.assertFalse(merged["overall_pass"])

    def test_newest_strategy(self):
        """Test NEWEST strategy uses latest timestamp."""
        now = datetime.now(timezone.utc)
        old_time = now - timedelta(hours=1)

        results_with_time = [
            (self.fail_result, old_time),
            (self.pass_result, now),
        ]

        merged = merge_results_newest(results_with_time)
        # Should use the newest (pass_result)
        self.assertTrue(merged["overall_pass"])


class TestCampaignLoading(unittest.TestCase):
    """Test loading verification results from campaign directories."""

    def test_load_verification_from_campaign(self):
        """Test loading verification files from a campaign."""
        with tempfile.TemporaryDirectory() as tmpdir:
            campaign_dir = Path(tmpdir)
            verification_dir = campaign_dir / "20_verification"
            verification_dir.mkdir(parents=True)

            # Create a verification YAML
            verification = {
                "type": "test_verification",
                "schema": "tcms/test-verification.schema.v1.json",
                "test_case_id": "TC_TEST_001",
                "description": "Test case",
                "overall_pass": True,
                "total_steps": 1,
                "passed_steps": 1,
                "failed_steps": 0,
                "not_executed_steps": 0,
            }

            verify_file = verification_dir / "TC_TEST_001_verification.yaml"
            with open(verify_file, 'w') as f:
                yaml.dump(verification, f)

            # Load and verify
            results = load_verification_from_campaign(campaign_dir)
            self.assertIn("TC_TEST_001", results)
            self.assertEqual(results["TC_TEST_001"]["test_case_id"], "TC_TEST_001")
            self.assertTrue(results["TC_TEST_001"]["overall_pass"])

    def test_load_verification_multiple_files(self):
        """Test loading multiple verification files."""
        with tempfile.TemporaryDirectory() as tmpdir:
            campaign_dir = Path(tmpdir)
            verification_dir = campaign_dir / "20_verification"
            verification_dir.mkdir(parents=True)

            # Create multiple verification YAMLs
            for i in range(3):
                verification = {
                    "type": "test_verification",
                    "test_case_id": f"TC_TEST_{i:03d}",
                    "overall_pass": i % 2 == 0,
                    "total_steps": 1,
                    "passed_steps": 1,
                    "failed_steps": 0,
                    "not_executed_steps": 0,
                }
                verify_file = verification_dir / f"TC_TEST_{i:03d}_verification.yaml"
                with open(verify_file, 'w') as f:
                    yaml.dump(verification, f)

            results = load_verification_from_campaign(campaign_dir)
            self.assertEqual(len(results), 3)
            self.assertIn("TC_TEST_000", results)
            self.assertIn("TC_TEST_001", results)
            self.assertIn("TC_TEST_002", results)

    def test_load_verification_missing_directory(self):
        """Test loading from campaign with missing 20_verification directory."""
        with tempfile.TemporaryDirectory() as tmpdir:
            campaign_dir = Path(tmpdir)
            results = load_verification_from_campaign(campaign_dir)
            self.assertEqual(results, {})


class TestExecutionTimestamps(unittest.TestCase):
    """Test extracting execution timestamps from campaigns."""

    def test_get_execution_timestamp_from_logs(self):
        """Test getting timestamp from execution logs."""
        with tempfile.TemporaryDirectory() as tmpdir:
            campaign_dir = Path(tmpdir)
            logs_dir = campaign_dir / "10_test_results" / "execution_logs"
            logs_dir.mkdir(parents=True)

            # Create an execution log with timestamps
            execution_log = [
                {
                    "test_sequence": 1,
                    "step": 1,
                    "command": "echo test",
                    "exit_code": 0,
                    "timestamp": "2026-05-13T10:00:00+00:00",
                },
                {
                    "test_sequence": 1,
                    "step": 2,
                    "command": "echo test2",
                    "exit_code": 0,
                    "timestamp": "2026-05-13T10:00:01+00:00",
                },
            ]

            log_file = logs_dir / "TC_TEST_001_execution.json"
            with open(log_file, 'w') as f:
                json.dump(execution_log, f)

            timestamp = get_execution_timestamp(campaign_dir)
            self.assertIsNotNone(timestamp)
            # Should be the earliest timestamp
            self.assertEqual(timestamp.hour, 10)
            self.assertEqual(timestamp.minute, 0)
            self.assertEqual(timestamp.second, 0)

    def test_get_execution_timestamp_multiple_logs(self):
        """Test getting earliest timestamp from multiple log files."""
        with tempfile.TemporaryDirectory() as tmpdir:
            campaign_dir = Path(tmpdir)
            logs_dir = campaign_dir / "10_test_results" / "execution_logs"
            logs_dir.mkdir(parents=True)

            # Create multiple log files with different timestamps (valid seconds: 0-59)
            for i, ts_sec in enumerate([40, 10, 30]):  # 10 is earliest
                execution_log = [
                    {
                        "test_sequence": 1,
                        "step": 1,
                        "timestamp": f"2026-05-13T10:00:{ts_sec:02d}+00:00",
                    }
                ]
                log_file = logs_dir / f"TC_TEST_{i:03d}_execution.json"
                with open(log_file, 'w') as f:
                    json.dump(execution_log, f)

            timestamp = get_execution_timestamp(campaign_dir)
            self.assertIsNotNone(timestamp)
            # Should be the earliest (10 seconds)
            self.assertEqual(timestamp.second, 10)

    def test_get_execution_timestamp_missing_logs(self):
        """Test getting timestamp when execution logs are missing."""
        with tempfile.TemporaryDirectory() as tmpdir:
            campaign_dir = Path(tmpdir)
            timestamp = get_execution_timestamp(campaign_dir)
            self.assertIsNone(timestamp)


class TestIntegrationMergeCampaigns(unittest.TestCase):
    """Integration tests for merging multiple campaigns."""

    def _create_campaign(self, campaign_dir: Path, test_cases: dict[str, bool]):
        """Helper to create a campaign with verification files."""
        verification_dir = campaign_dir / "20_verification"
        verification_dir.mkdir(parents=True)

        logs_dir = campaign_dir / "10_test_results" / "execution_logs"
        logs_dir.mkdir(parents=True)

        # Create verification files for each test case
        for test_id, overall_pass in test_cases.items():
            verification = {
                "type": "test_verification",
                "schema": "tcms/test-verification.schema.v1.json",
                "test_case_id": test_id,
                "description": f"Test {test_id}",
                "overall_pass": overall_pass,
                "total_steps": 3,
                "passed_steps": 3 if overall_pass else 1,
                "failed_steps": 0 if overall_pass else 2,
                "not_executed_steps": 0,
            }

            verify_file = verification_dir / f"{test_id}_verification.yaml"
            with open(verify_file, 'w') as f:
                yaml.dump(verification, f)

        # Create execution log for timestamp extraction
        execution_log = [
            {
                "test_sequence": 1,
                "step": 1,
                "timestamp": "2026-05-13T10:00:00+00:00",
            }
        ]
        log_file = logs_dir / "execution.json"
        with open(log_file, 'w') as f:
            json.dump(execution_log, f)

    def test_merge_two_campaigns_or_strategy(self):
        """Test merging two campaigns with OR strategy."""
        with tempfile.TemporaryDirectory() as tmpdir:
            tmpdir = Path(tmpdir)

            # Campaign 1: TC_001 passes
            campaign1 = tmpdir / "campaign1"
            campaign1.mkdir()
            self._create_campaign(campaign1, {"TC_001": True})

            # Campaign 2: TC_001 fails
            campaign2 = tmpdir / "campaign2"
            campaign2.mkdir()
            self._create_campaign(campaign2, {"TC_001": False})

            # Merge with OR strategy: pass OR fail = pass
            merged = merge_campaigns([campaign1, campaign2], "or")

            self.assertEqual(merged["type"], "test_results_container")
            self.assertEqual(len(merged["test_results"]), 1)
            self.assertTrue(merged["test_results"][0]["overall_pass"])

    def test_merge_two_campaigns_and_strategy(self):
        """Test merging two campaigns with AND strategy."""
        with tempfile.TemporaryDirectory() as tmpdir:
            tmpdir = Path(tmpdir)

            campaign1 = tmpdir / "campaign1"
            campaign1.mkdir()
            self._create_campaign(campaign1, {"TC_001": True})

            campaign2 = tmpdir / "campaign2"
            campaign2.mkdir()
            self._create_campaign(campaign2, {"TC_001": False})

            # Merge with AND strategy: pass AND fail = fail
            merged = merge_campaigns([campaign1, campaign2], "and")

            self.assertEqual(len(merged["test_results"]), 1)
            self.assertFalse(merged["test_results"][0]["overall_pass"])

    def test_merge_non_overlapping_test_cases(self):
        """Test merging campaigns with non-overlapping test cases."""
        with tempfile.TemporaryDirectory() as tmpdir:
            tmpdir = Path(tmpdir)

            campaign1 = tmpdir / "campaign1"
            campaign1.mkdir()
            self._create_campaign(campaign1, {"TC_001": True, "TC_002": True})

            campaign2 = tmpdir / "campaign2"
            campaign2.mkdir()
            self._create_campaign(campaign2, {"TC_003": False, "TC_004": True})

            merged = merge_campaigns([campaign1, campaign2], "or")

            # Should have all 4 test cases
            self.assertEqual(len(merged["test_results"]), 4)
            test_ids = {r["test_case_id"] for r in merged["test_results"]}
            self.assertEqual(test_ids, {"TC_001", "TC_002", "TC_003", "TC_004"})

    def test_merge_container_metadata(self):
        """Test that merged container has correct metadata."""
        with tempfile.TemporaryDirectory() as tmpdir:
            tmpdir = Path(tmpdir)

            campaign1 = tmpdir / "campaign1"
            campaign1.mkdir()
            self._create_campaign(campaign1, {"TC_001": True, "TC_002": False})

            campaign2 = tmpdir / "campaign2"
            campaign2.mkdir()
            self._create_campaign(campaign2, {"TC_003": True})

            merged = merge_campaigns([campaign1, campaign2], "or")

            # Check metadata
            self.assertEqual(merged["metadata"]["total_test_cases"], 3)
            self.assertEqual(merged["metadata"]["passed_test_cases"], 2)  # TC_001, TC_003 (TC_002 fails)
            self.assertEqual(merged["metadata"]["failed_test_cases"], 1)  # TC_002
            self.assertEqual(merged["metadata"]["execution_duration"], 0.0)

    def test_merge_oldest_strategy_uses_earliest_timestamp(self):
        """Test OLDEST strategy in integration."""
        with tempfile.TemporaryDirectory() as tmpdir:
            tmpdir = Path(tmpdir)

            # Campaign 1: newer, passes
            campaign1 = tmpdir / "campaign1"
            campaign1.mkdir()
            verification_dir = campaign1 / "20_verification"
            verification_dir.mkdir(parents=True)
            logs_dir = campaign1 / "10_test_results" / "execution_logs"
            logs_dir.mkdir(parents=True)

            verification = {
                "type": "test_verification",
                "test_case_id": "TC_001",
                "overall_pass": True,
                "total_steps": 1,
                "passed_steps": 1,
                "failed_steps": 0,
                "not_executed_steps": 0,
            }
            with open(verification_dir / "TC_001_verification.yaml", 'w') as f:
                yaml.dump(verification, f)

            execution_log = [{"timestamp": "2026-05-13T10:00:10+00:00"}]
            with open(logs_dir / "execution.json", 'w') as f:
                json.dump(execution_log, f)

            # Campaign 2: older, fails
            campaign2 = tmpdir / "campaign2"
            campaign2.mkdir()
            verification_dir = campaign2 / "20_verification"
            verification_dir.mkdir(parents=True)
            logs_dir = campaign2 / "10_test_results" / "execution_logs"
            logs_dir.mkdir(parents=True)

            verification = {
                "type": "test_verification",
                "test_case_id": "TC_001",
                "overall_pass": False,
                "total_steps": 1,
                "passed_steps": 0,
                "failed_steps": 1,
                "not_executed_steps": 0,
            }
            with open(verification_dir / "TC_001_verification.yaml", 'w') as f:
                yaml.dump(verification, f)

            execution_log = [{"timestamp": "2026-05-13T10:00:00+00:00"}]
            with open(logs_dir / "execution.json", 'w') as f:
                json.dump(execution_log, f)

            merged = merge_campaigns([campaign1, campaign2], "oldest")

            # Should use campaign2 (oldest, fails)
            self.assertFalse(merged["test_results"][0]["overall_pass"])


if __name__ == "__main__":
    unittest.main()
