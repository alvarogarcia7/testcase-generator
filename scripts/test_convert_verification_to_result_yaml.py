#!/usr/bin/env python3.14
"""
Unit tests for convert_verification_to_result_yaml.py

Tests all major functions and workflows including:
- Loading execution logs and test cases
- Enriching step results with expected/actual data
- Multiple mode with ContainerReport input creates N separate YAML files
- Single mode with ContainerReport input creates one YAML file as array
- Multiple mode with stdin ContainerReport produces individual files
- Single mode with stdin produces single array file
- Validation that single-mode output is parseable as YAML array and each element has type: test_result
- Error when --single is used with directory path instead of file path
- Validation of mandatory execution-logs and testcases arguments
"""

import unittest
import json
import tempfile
import shutil
import sys
from pathlib import Path
from io import StringIO

try:
    import yaml

    YAML_AVAILABLE = True
except ImportError:
    YAML_AVAILABLE = False

# Import the module under test
sys.path.insert(0, str(Path(__file__).parent))
from convert_verification_to_result_yaml import (
    parse_step_result,
    convert_test_case_to_result,
    write_result_yaml,
    write_multiple_result_files,
    write_single_result_file,
    process_verification_json,
    load_execution_log,
    load_test_case,
    get_expected_from_testcase,
    enrich_step_result,
    main,
)


class TestLoadExecutionLog(unittest.TestCase):
    """Tests for load_execution_log function."""

    def setUp(self):
        """Create temporary directory for test files."""
        self.temp_dir = tempfile.mkdtemp()
        self.temp_path = Path(self.temp_dir)

    def tearDown(self):
        """Clean up temporary directory."""
        shutil.rmtree(self.temp_dir)

    def test_load_valid_execution_log(self):
        """Test loading a valid execution log."""
        log_data = [
            {
                "test_sequence": 1,
                "step": 1,
                "command": "echo hello",
                "exit_code": 0,
                "output": "hello",
                "result_verification_pass": True,
                "output_verification_pass": True
            },
            {
                "test_sequence": 1,
                "step": 2,
                "command": "true",
                "exit_code": 0,
                "output": "",
                "result_verification_pass": True,
                "output_verification_pass": True
            }
        ]
        
        log_path = self.temp_path / "test_log.json"
        with open(log_path, 'w', encoding='utf-8') as f:
            json.dump(log_data, f)
        
        execution_map = load_execution_log(log_path)
        
        self.assertEqual(len(execution_map), 2)
        self.assertIn((1, 1), execution_map)
        self.assertIn((1, 2), execution_map)
        self.assertEqual(execution_map[(1, 1)]["output"], "hello")
        self.assertEqual(execution_map[(1, 2)]["exit_code"], 0)

    def test_load_nonexistent_log(self):
        """Test loading a non-existent execution log."""
        log_path = self.temp_path / "nonexistent.json"
        execution_map = load_execution_log(log_path)
        
        self.assertEqual(len(execution_map), 0)


class TestLoadTestCase(unittest.TestCase):
    """Tests for load_test_case function."""

    def setUp(self):
        """Create temporary directory for test files."""
        self.temp_dir = tempfile.mkdtemp()
        self.temp_path = Path(self.temp_dir)

    def tearDown(self):
        """Clean up temporary directory."""
        shutil.rmtree(self.temp_dir)

    def test_load_valid_test_case(self):
        """Test loading a valid test case."""
        testcase_data = {
            "type": "test_case",
            "schema": "tcms/test-case.schema.v1.json",
            "id": "TC-001",
            "description": "Test case",
            "test_sequences": [
                {
                    "id": 1,
                    "name": "Sequence 1",
                    "steps": [
                        {
                            "step": 1,
                            "description": "Step 1",
                            "command": "echo hello",
                            "expected": {
                                "success": True,
                                "result": "0",
                                "output": "hello"
                            }
                        }
                    ]
                }
            ]
        }
        
        tc_path = self.temp_path / "TC-001.yaml"
        with open(tc_path, 'w', encoding='utf-8') as f:
            yaml.dump(testcase_data, f)
        
        testcase = load_test_case(tc_path)
        
        self.assertIsNotNone(testcase)
        self.assertEqual(testcase["id"], "TC-001")
        self.assertEqual(len(testcase["test_sequences"]), 1)


class TestGetExpectedFromTestcase(unittest.TestCase):
    """Tests for get_expected_from_testcase function."""

    def test_get_expected_values(self):
        """Test extracting expected values from test case."""
        testcase = {
            "test_sequences": [
                {
                    "id": 1,
                    "steps": [
                        {
                            "step": 1,
                            "expected": {
                                "success": True,
                                "result": "0",
                                "output": "hello"
                            }
                        },
                        {
                            "step": 2,
                            "expected": {
                                "result": 0,
                                "output": ""
                            }
                        }
                    ]
                }
            ]
        }
        
        expected = get_expected_from_testcase(testcase, 1, 1)
        self.assertIsNotNone(expected)
        self.assertEqual(expected["result"], "0")
        self.assertEqual(expected["output"], "hello")
        self.assertTrue(expected["success"])
        
        # Test with integer result being converted to string
        expected2 = get_expected_from_testcase(testcase, 1, 2)
        self.assertIsNotNone(expected2)
        self.assertEqual(expected2["result"], "0")

    def test_get_expected_nonexistent_step(self):
        """Test getting expected for non-existent step."""
        testcase = {
            "test_sequences": [
                {
                    "id": 1,
                    "steps": []
                }
            ]
        }
        
        expected = get_expected_from_testcase(testcase, 1, 99)
        self.assertIsNone(expected)


class TestEnrichStepResult(unittest.TestCase):
    """Tests for enrich_step_result function."""

    def test_enrich_fail_step_with_execution_data(self):
        """Test enriching a Fail step with execution data."""
        step_result = {
            "Fail": {
                "step": 1,
                "description": "Failed step",
                "expected": {},
                "actual_result": "",
                "actual_output": "",
                "reason": "Test failure"
            }
        }
        
        execution_map = {
            (1, 1): {
                "exit_code": 1,
                "output": "error message"
            }
        }
        
        testcase = {
            "test_sequences": [
                {
                    "id": 1,
                    "steps": [
                        {
                            "step": 1,
                            "expected": {
                                "success": True,
                                "result": "0",
                                "output": "success"
                            }
                        }
                    ]
                }
            ]
        }
        
        enriched = enrich_step_result(step_result, execution_map, testcase, 1)
        
        self.assertIn("Fail", enriched)
        fail_data = enriched["Fail"]
        self.assertEqual(fail_data["actual_result"], "1")
        self.assertEqual(fail_data["actual_output"], "error message")
        self.assertEqual(fail_data["expected"]["result"], "0")
        self.assertEqual(fail_data["expected"]["output"], "success")

    def test_enrich_pass_step_unchanged(self):
        """Test that Pass steps are not modified."""
        step_result = {
            "Pass": {
                "step": 1,
                "description": "Passed step"
            }
        }
        
        execution_map = {}
        testcase = None
        
        enriched = enrich_step_result(step_result, execution_map, testcase, 1)
        
        self.assertEqual(enriched, step_result)


class TestParseStepResult(unittest.TestCase):
    """Tests for parse_step_result function."""

    def test_parse_pass_step(self):
        """Test parsing a Pass step result."""
        step_result = {"Pass": {"step": 1, "description": "Step passed"}}

        result = parse_step_result(step_result)

        self.assertIn("Pass", result)
        self.assertEqual(result["Pass"]["step"], 1)
        self.assertEqual(result["Pass"]["description"], "Step passed")

    def test_parse_fail_step(self):
        """Test parsing a Fail step result."""
        step_result = {
            "Fail": {
                "step": 2,
                "description": "Step failed",
                "expected": {"result": "0", "output": ""},
                "actual_result": "1",
                "actual_output": "",
                "reason": "Result mismatch",
            }
        }

        result = parse_step_result(step_result)

        self.assertIn("Fail", result)
        self.assertEqual(result["Fail"]["step"], 2)
        self.assertEqual(result["Fail"]["description"], "Step failed")
        self.assertEqual(result["Fail"]["expected"]["result"], "0")
        self.assertEqual(result["Fail"]["actual_result"], "1")

    def test_parse_not_executed_step(self):
        """Test parsing a NotExecuted step result."""
        step_result = {"NotExecuted": {"step": 3, "description": "Step not executed"}}

        result = parse_step_result(step_result)

        self.assertIn("NotExecuted", result)
        self.assertEqual(result["NotExecuted"]["step"], 3)
        self.assertEqual(result["NotExecuted"]["description"], "Step not executed")

    def test_parse_unknown_variant(self):
        """Test parsing unknown variant returns as-is."""
        step_result = {"Unknown": {"step": 4, "description": "Unknown variant"}}

        result = parse_step_result(step_result)

        self.assertEqual(result, step_result)


class TestConvertTestCaseToResult(unittest.TestCase):
    """Tests for convert_test_case_to_result function."""

    def test_convert_basic_test_case(self):
        """Test converting a basic test case to result format."""
        test_case = {
            "test_case_id": "TC-001",
            "description": "Test case 001",
            "sequences": [],
            "total_steps": 5,
            "passed_steps": 3,
            "failed_steps": 1,
            "not_executed_steps": 1,
            "overall_pass": False,
        }

        result = convert_test_case_to_result(test_case, {}, None)

        self.assertEqual(result["type"], "test_result")
        self.assertEqual(result["schema"], "tcms/test-result.schema.v1.json")
        self.assertEqual(result["test_case_id"], "TC-001")
        self.assertEqual(result["description"], "Test case 001")
        self.assertEqual(result["total_steps"], 5)
        self.assertEqual(result["passed_steps"], 3)
        self.assertEqual(result["failed_steps"], 1)
        self.assertEqual(result["not_executed_steps"], 1)
        self.assertFalse(result["overall_pass"])

    def test_convert_with_optional_metadata(self):
        """Test converting test case with optional metadata."""
        test_case = {
            "test_case_id": "TC-002",
            "description": "Test case 002",
            "requirement": "REQ-001",
            "item": 1,
            "tc": 2,
            "sequences": [],
            "total_steps": 0,
            "passed_steps": 0,
            "failed_steps": 0,
            "not_executed_steps": 0,
            "overall_pass": True,
        }

        result = convert_test_case_to_result(test_case, {}, None)

        self.assertEqual(result["requirement"], "REQ-001")
        self.assertEqual(result["item"], 1)
        self.assertEqual(result["tc"], 2)

    def test_convert_without_optional_metadata(self):
        """Test that None values are not included."""
        test_case = {
            "test_case_id": "TC-003",
            "description": "Test case 003",
            "requirement": None,
            "item": None,
            "tc": None,
            "sequences": [],
            "total_steps": 0,
            "passed_steps": 0,
            "failed_steps": 0,
            "not_executed_steps": 0,
            "overall_pass": True,
        }

        result = convert_test_case_to_result(test_case, {}, None)

        self.assertNotIn("requirement", result)
        self.assertNotIn("item", result)
        self.assertNotIn("tc", result)

    def test_convert_with_sequences(self):
        """Test converting test case with sequences and step results."""
        test_case = {
            "test_case_id": "TC-004",
            "description": "Test with sequences",
            "sequences": [
                {
                    "sequence_id": 1,
                    "name": "Sequence 1",
                    "step_results": [
                        {"Pass": {"step": 1, "description": "Step passed"}},
                        {
                            "Fail": {
                                "step": 2,
                                "description": "Step failed",
                                "expected": {"result": "0", "output": ""},
                                "actual_result": "1",
                                "actual_output": "",
                                "reason": "Failed"
                            }
                        },
                    ],
                    "all_steps_passed": False,
                }
            ],
            "total_steps": 2,
            "passed_steps": 1,
            "failed_steps": 1,
            "not_executed_steps": 0,
            "overall_pass": False,
        }

        result = convert_test_case_to_result(test_case, {}, None)

        self.assertEqual(len(result["sequences"]), 1)
        seq = result["sequences"][0]
        self.assertEqual(seq["sequence_id"], 1)
        self.assertEqual(seq["name"], "Sequence 1")
        self.assertFalse(seq["all_steps_passed"])
        self.assertEqual(len(seq["step_results"]), 2)
        self.assertIn("Pass", seq["step_results"][0])
        self.assertIn("Fail", seq["step_results"][1])

    def test_convert_with_sequence_metadata(self):
        """Test converting sequences with optional metadata."""
        test_case = {
            "test_case_id": "TC-005",
            "description": "Test with sequence metadata",
            "sequences": [
                {
                    "sequence_id": 1,
                    "name": "Sequence 1",
                    "requirement": "SEQ-REQ-001",
                    "item": 1,
                    "tc": 1,
                    "step_results": [],
                    "all_steps_passed": True,
                }
            ],
            "total_steps": 0,
            "passed_steps": 0,
            "failed_steps": 0,
            "not_executed_steps": 0,
            "overall_pass": True,
        }

        result = convert_test_case_to_result(test_case, {}, None)

        seq = result["sequences"][0]
        self.assertEqual(seq["requirement"], "SEQ-REQ-001")
        self.assertEqual(seq["item"], 1)
        self.assertEqual(seq["tc"], 1)


class TestWriteResultYaml(unittest.TestCase):
    """Tests for write_result_yaml function."""

    def setUp(self):
        """Create temporary directory for test files."""
        self.temp_dir = tempfile.mkdtemp()
        self.temp_path = Path(self.temp_dir)

    def tearDown(self):
        """Clean up temporary directory."""
        shutil.rmtree(self.temp_dir)

    def test_write_result_yaml_creates_file(self):
        """Test that write_result_yaml creates a YAML file."""
        result = {
            "type": "test_result",
            "schema": "tcms/test-result.schema.v1.json",
            "test_case_id": "TC-001",
            "description": "Test",
            "sequences": [],
            "total_steps": 1,
            "passed_steps": 1,
            "failed_steps": 0,
            "not_executed_steps": 0,
            "overall_pass": True,
        }

        output_path = self.temp_path / "test_result.yaml"
        write_result_yaml(result, output_path)

        self.assertTrue(output_path.exists())

        with open(output_path, "r", encoding="utf-8") as f:
            loaded = yaml.safe_load(f)

        self.assertEqual(loaded["type"], "test_result")
        self.assertEqual(loaded["schema"], "tcms/test-result.schema.v1.json")
        self.assertEqual(loaded["test_case_id"], "TC-001")

    def test_write_result_yaml_creates_parent_directory(self):
        """Test that write_result_yaml creates parent directories if needed."""
        result = {
            "type": "test_result",
            "schema": "tcms/test-result.schema.v1.json",
            "test_case_id": "TC-002",
            "description": "Test",
            "sequences": [],
            "total_steps": 0,
            "passed_steps": 0,
            "failed_steps": 0,
            "not_executed_steps": 0,
            "overall_pass": True,
        }

        output_path = self.temp_path / "subdir" / "nested" / "result.yaml"
        write_result_yaml(result, output_path)

        self.assertTrue(output_path.exists())
        self.assertTrue(output_path.parent.exists())


class TestWriteMultipleResultFiles(unittest.TestCase):
    """Tests for write_multiple_result_files function (multiple mode with ContainerReport)."""

    def setUp(self):
        """Create temporary directory for test files."""
        self.temp_dir = tempfile.mkdtemp()
        self.temp_path = Path(self.temp_dir)

    def tearDown(self):
        """Clean up temporary directory."""
        shutil.rmtree(self.temp_dir)

    def test_write_multiple_files_from_container_report(self):
        """Test that multiple mode creates N separate YAML files from ContainerReport."""
        test_cases = [
            {
                "test_case_id": "TC-001",
                "description": "First test case",
                "sequences": [],
                "total_steps": 2,
                "passed_steps": 2,
                "failed_steps": 0,
                "not_executed_steps": 0,
                "overall_pass": True,
            },
            {
                "test_case_id": "TC-002",
                "description": "Second test case",
                "sequences": [],
                "total_steps": 3,
                "passed_steps": 2,
                "failed_steps": 1,
                "not_executed_steps": 0,
                "overall_pass": False,
            },
            {
                "test_case_id": "TC-003",
                "description": "Third test case",
                "sequences": [],
                "total_steps": 1,
                "passed_steps": 0,
                "failed_steps": 0,
                "not_executed_steps": 1,
                "overall_pass": False,
            },
        ]

        output_dir = self.temp_path / "results"
        count = write_multiple_result_files(test_cases, output_dir, None, None)

        self.assertEqual(count, 3)

        # Verify all files exist
        file1 = output_dir / "TC-001_result.yaml"
        file2 = output_dir / "TC-002_result.yaml"
        file3 = output_dir / "TC-003_result.yaml"

        self.assertTrue(file1.exists())
        self.assertTrue(file2.exists())
        self.assertTrue(file3.exists())

        # Verify content of each file
        with open(file1, "r", encoding="utf-8") as f:
            result1 = yaml.safe_load(f)
        self.assertEqual(result1["type"], "test_result")
        self.assertEqual(result1["schema"], "tcms/test-result.schema.v1.json")
        self.assertEqual(result1["test_case_id"], "TC-001")
        self.assertTrue(result1["overall_pass"])

        with open(file2, "r", encoding="utf-8") as f:
            result2 = yaml.safe_load(f)
        self.assertEqual(result2["type"], "test_result")
        self.assertEqual(result2["test_case_id"], "TC-002")
        self.assertFalse(result2["overall_pass"])

        with open(file3, "r", encoding="utf-8") as f:
            result3 = yaml.safe_load(f)
        self.assertEqual(result3["type"], "test_result")
        self.assertEqual(result3["test_case_id"], "TC-003")
        self.assertFalse(result3["overall_pass"])

    def test_write_multiple_files_empty_list(self):
        """Test that multiple mode handles empty test case list."""
        test_cases = []
        output_dir = self.temp_path / "results"

        count = write_multiple_result_files(test_cases, output_dir, None, None)

        self.assertEqual(count, 0)


class TestWriteSingleResultFile(unittest.TestCase):
    """Tests for write_single_result_file function (single mode with ContainerReport)."""

    def setUp(self):
        """Create temporary directory for test files."""
        self.temp_dir = tempfile.mkdtemp()
        self.temp_path = Path(self.temp_dir)

    def tearDown(self):
        """Clean up temporary directory."""
        shutil.rmtree(self.temp_dir)

    def test_write_single_file_from_container_report(self):
        """Test that single mode creates one YAML file as array with N elements from ContainerReport."""
        test_cases = [
            {
                "test_case_id": "TC-001",
                "description": "First test case",
                "sequences": [],
                "total_steps": 2,
                "passed_steps": 2,
                "failed_steps": 0,
                "not_executed_steps": 0,
                "overall_pass": True,
            },
            {
                "test_case_id": "TC-002",
                "description": "Second test case",
                "sequences": [],
                "total_steps": 3,
                "passed_steps": 2,
                "failed_steps": 1,
                "not_executed_steps": 0,
                "overall_pass": False,
            },
            {
                "test_case_id": "TC-003",
                "description": "Third test case",
                "sequences": [],
                "total_steps": 1,
                "passed_steps": 0,
                "failed_steps": 0,
                "not_executed_steps": 1,
                "overall_pass": False,
            },
        ]

        output_file = self.temp_path / "results.yaml"
        count = write_single_result_file(test_cases, output_file, None, None)

        self.assertEqual(count, 3)
        self.assertTrue(output_file.exists())

        # Verify content is a YAML array
        with open(output_file, "r", encoding="utf-8") as f:
            results = yaml.safe_load(f)

        self.assertIsInstance(results, list)
        self.assertEqual(len(results), 3)

        # Verify each element has type: test_result
        for i, result in enumerate(results):
            self.assertEqual(result["type"], "test_result")
            self.assertEqual(result["schema"], "tcms/test-result.schema.v1.json")
            self.assertEqual(result["test_case_id"], f"TC-{i + 1:03d}")

        # Verify specific content
        self.assertTrue(results[0]["overall_pass"])
        self.assertFalse(results[1]["overall_pass"])
        self.assertFalse(results[2]["overall_pass"])

    def test_single_mode_output_parseable_as_yaml_array(self):
        """Test that single-mode output is parseable as YAML array and each element has type: test_result field."""
        test_cases = [
            {
                "test_case_id": "TC-PARSE-001",
                "description": "Parseable test 1",
                "sequences": [
                    {
                        "sequence_id": 1,
                        "name": "Seq 1",
                        "step_results": [
                            {"Pass": {"step": 1, "description": "Passed"}}
                        ],
                        "all_steps_passed": True,
                    }
                ],
                "total_steps": 1,
                "passed_steps": 1,
                "failed_steps": 0,
                "not_executed_steps": 0,
                "overall_pass": True,
            },
            {
                "test_case_id": "TC-PARSE-002",
                "description": "Parseable test 2",
                "sequences": [],
                "total_steps": 0,
                "passed_steps": 0,
                "failed_steps": 0,
                "not_executed_steps": 0,
                "overall_pass": True,
            },
        ]

        output_file = self.temp_path / "parseable.yaml"
        write_single_result_file(test_cases, output_file, None, None)

        # Parse the YAML file
        with open(output_file, "r", encoding="utf-8") as f:
            parsed = yaml.safe_load(f)

        # Verify it's a list
        self.assertIsInstance(parsed, list, "Output should be parseable as YAML array")

        # Verify each element has type: test_result
        for idx, element in enumerate(parsed):
            self.assertIn("type", element, f"Element {idx} missing 'type' field")
            self.assertEqual(
                element["type"], "test_result", f"Element {idx} should have type: test_result"
            )
            self.assertEqual(
                element["schema"], "tcms/test-result.schema.v1.json", f"Element {idx} should have correct schema"
            )
            self.assertIn("test_case_id", element)
            self.assertIn("description", element)
            self.assertIn("sequences", element)
            self.assertIn("total_steps", element)
            self.assertIn("overall_pass", element)

    def test_write_single_file_empty_list(self):
        """Test that single mode handles empty test case list."""
        test_cases = []
        output_file = self.temp_path / "empty_results.yaml"

        count = write_single_result_file(test_cases, output_file, None, None)

        self.assertEqual(count, 0)
        self.assertTrue(output_file.exists())

        with open(output_file, "r", encoding="utf-8") as f:
            results = yaml.safe_load(f)

        self.assertIsInstance(results, list)
        self.assertEqual(len(results), 0)


class TestProcessVerificationJson(unittest.TestCase):
    """Tests for process_verification_json function."""

    def setUp(self):
        """Create temporary directory for test files."""
        self.temp_dir = tempfile.mkdtemp()
        self.temp_path = Path(self.temp_dir)

    def tearDown(self):
        """Clean up temporary directory."""
        shutil.rmtree(self.temp_dir)

    def test_process_container_report_multiple_mode(self):
        """Test processing ContainerReport in multiple mode."""
        container_report = {
            "test_results": [
                {
                    "test_case_id": "TC-001",
                    "description": "Test 1",
                    "sequences": [],
                    "total_steps": 1,
                    "passed_steps": 1,
                    "failed_steps": 0,
                    "not_executed_steps": 0,
                    "overall_pass": True,
                },
                {
                    "test_case_id": "TC-002",
                    "description": "Test 2",
                    "sequences": [],
                    "total_steps": 1,
                    "passed_steps": 0,
                    "failed_steps": 1,
                    "not_executed_steps": 0,
                    "overall_pass": False,
                },
            ]
        }

        input_file = self.temp_path / "container_report.json"
        with open(input_file, "w", encoding="utf-8") as f:
            json.dump(container_report, f)

        output_dir = self.temp_path / "results"
        count = process_verification_json(
            input_file, output_dir, None, None, output_mode="multiple", verbose=False
        )

        self.assertEqual(count, 2)
        self.assertTrue((output_dir / "TC-001_result.yaml").exists())
        self.assertTrue((output_dir / "TC-002_result.yaml").exists())

    def test_process_container_report_single_mode(self):
        """Test processing ContainerReport in single mode."""
        container_report = {
            "test_results": [
                {
                    "test_case_id": "TC-001",
                    "description": "Test 1",
                    "sequences": [],
                    "total_steps": 1,
                    "passed_steps": 1,
                    "failed_steps": 0,
                    "not_executed_steps": 0,
                    "overall_pass": True,
                },
                {
                    "test_case_id": "TC-002",
                    "description": "Test 2",
                    "sequences": [],
                    "total_steps": 1,
                    "passed_steps": 0,
                    "failed_steps": 1,
                    "not_executed_steps": 0,
                    "overall_pass": False,
                },
            ]
        }

        input_file = self.temp_path / "container_report.json"
        with open(input_file, "w", encoding="utf-8") as f:
            json.dump(container_report, f)

        output_file = self.temp_path / "results.yaml"
        count = process_verification_json(
            input_file, output_file, None, None, output_mode="single", verbose=False
        )

        self.assertEqual(count, 2)
        self.assertTrue(output_file.exists())

        with open(output_file, "r", encoding="utf-8") as f:
            results = yaml.safe_load(f)

        self.assertIsInstance(results, list)
        self.assertEqual(len(results), 2)

    def test_process_batch_verification_report(self):
        """Test processing BatchVerificationReport format."""
        batch_report = {
            "test_cases": [
                {
                    "test_case_id": "TC-BATCH-001",
                    "description": "Batch test",
                    "sequences": [],
                    "total_steps": 1,
                    "passed_steps": 1,
                    "failed_steps": 0,
                    "not_executed_steps": 0,
                    "overall_pass": True,
                }
            ]
        }

        input_file = self.temp_path / "batch_report.json"
        with open(input_file, "w", encoding="utf-8") as f:
            json.dump(batch_report, f)

        output_dir = self.temp_path / "results"
        count = process_verification_json(
            input_file, output_dir, None, None, output_mode="multiple", verbose=False
        )

        self.assertEqual(count, 1)
        self.assertTrue((output_dir / "TC-BATCH-001_result.yaml").exists())

    def test_process_single_test_case(self):
        """Test processing single TestCaseVerificationResult."""
        single_test_case = {
            "test_case_id": "TC-SINGLE-001",
            "description": "Single test",
            "sequences": [],
            "total_steps": 1,
            "passed_steps": 1,
            "failed_steps": 0,
            "not_executed_steps": 0,
            "overall_pass": True,
        }

        input_file = self.temp_path / "single_test_case.json"
        with open(input_file, "w", encoding="utf-8") as f:
            json.dump(single_test_case, f)

        output_dir = self.temp_path / "results"
        count = process_verification_json(
            input_file, output_dir, None, None, output_mode="multiple", verbose=False
        )

        self.assertEqual(count, 1)
        self.assertTrue((output_dir / "TC-SINGLE-001_result.yaml").exists())

    def test_process_invalid_json(self):
        """Test handling of invalid JSON."""
        input_file = self.temp_path / "invalid.json"
        with open(input_file, "w", encoding="utf-8") as f:
            f.write("{ invalid json [")

        output_dir = self.temp_path / "results"
        count = process_verification_json(
            input_file, output_dir, None, None, output_mode="multiple", verbose=False
        )

        self.assertEqual(count, 0)

    def test_process_unknown_json_structure(self):
        """Test handling of unknown JSON structure."""
        unknown_data = {"unknown_field": "value"}

        input_file = self.temp_path / "unknown.json"
        with open(input_file, "w", encoding="utf-8") as f:
            json.dump(unknown_data, f)

        output_dir = self.temp_path / "results"
        count = process_verification_json(
            input_file, output_dir, None, None, output_mode="multiple", verbose=False
        )

        self.assertEqual(count, 0)


class TestMainValidation(unittest.TestCase):
    """Tests for main function validation logic."""

    def setUp(self):
        """Save original argv."""
        self.original_argv = sys.argv
        self.temp_dir = tempfile.mkdtemp()
        self.temp_path = Path(self.temp_dir)
        
        # Create dummy execution logs and testcases directories
        self.exec_logs_dir = self.temp_path / "logs"
        self.testcases_dir = self.temp_path / "testcases"
        self.exec_logs_dir.mkdir()
        self.testcases_dir.mkdir()

    def tearDown(self):
        """Restore original argv and clean up."""
        sys.argv = self.original_argv
        shutil.rmtree(self.temp_dir)

    def test_single_mode_with_directory_path_error(self):
        """Test that error occurs when --single is used with directory path instead of file path."""
        # Create a dummy input file
        input_file = self.temp_path / "input.json"
        input_file.write_text(json.dumps({"test_results": []}))

        # Use directory path for output in --single mode (should fail)
        output_dir = self.temp_path / "output_dir"
        sys.argv = [
            "convert_verification_to_result_yaml.py",
            str(input_file),
            "-o",
            str(output_dir),
            "--single",
            "--execution-logs",
            str(self.exec_logs_dir),
            "--testcases",
            str(self.testcases_dir),
        ]

        # Run main and expect error exit code
        exit_code = main()

        self.assertEqual(exit_code, 1)

    def test_single_mode_with_file_path_without_extension_error(self):
        """Test that error occurs when --single is used with file path without .yaml/.yml extension."""
        # Create a dummy input file
        input_file = self.temp_path / "input.json"
        input_file.write_text(json.dumps({"test_results": []}))

        # Use file path without proper extension
        output_file = self.temp_path / "output.txt"
        sys.argv = [
            "convert_verification_to_result_yaml.py",
            str(input_file),
            "-o",
            str(output_file),
            "--single",
            "--execution-logs",
            str(self.exec_logs_dir),
            "--testcases",
            str(self.testcases_dir),
        ]

        # Run main and expect error exit code
        exit_code = main()

        self.assertEqual(exit_code, 1)

    def test_single_mode_with_yaml_extension_success(self):
        """Test that --single mode works with .yaml extension."""
        # Create a valid input file
        input_file = self.temp_path / "input.json"
        container_report = {
            "test_results": [
                {
                    "test_case_id": "TC-001",
                    "description": "Test",
                    "sequences": [],
                    "total_steps": 0,
                    "passed_steps": 0,
                    "failed_steps": 0,
                    "not_executed_steps": 0,
                    "overall_pass": True,
                }
            ]
        }
        input_file.write_text(json.dumps(container_report))

        # Use file path with .yaml extension
        output_file = self.temp_path / "output.yaml"
        sys.argv = [
            "convert_verification_to_result_yaml.py",
            str(input_file),
            "-o",
            str(output_file),
            "--single",
            "--execution-logs",
            str(self.exec_logs_dir),
            "--testcases",
            str(self.testcases_dir),
        ]

        # Run main
        exit_code = main()

        self.assertEqual(exit_code, 0)
        self.assertTrue(output_file.exists())

    def test_single_mode_with_yml_extension_success(self):
        """Test that --single mode works with .yml extension."""
        # Create a valid input file
        input_file = self.temp_path / "input.json"
        container_report = {
            "test_results": [
                {
                    "test_case_id": "TC-001",
                    "description": "Test",
                    "sequences": [],
                    "total_steps": 0,
                    "passed_steps": 0,
                    "failed_steps": 0,
                    "not_executed_steps": 0,
                    "overall_pass": True,
                }
            ]
        }
        input_file.write_text(json.dumps(container_report))

        # Use file path with .yml extension
        output_file = self.temp_path / "output.yml"
        sys.argv = [
            "convert_verification_to_result_yaml.py",
            str(input_file),
            "-o",
            str(output_file),
            "--single",
            "--execution-logs",
            str(self.exec_logs_dir),
            "--testcases",
            str(self.testcases_dir),
        ]

        # Run main
        exit_code = main()

        self.assertEqual(exit_code, 0)
        self.assertTrue(output_file.exists())

    def test_nonexistent_input_file_error(self):
        """Test that error occurs when input file doesn't exist."""
        input_file = self.temp_path / "nonexistent.json"
        output_dir = self.temp_path / "output"
        sys.argv = [
            "convert_verification_to_result_yaml.py",
            str(input_file),
            "-o",
            str(output_dir),
            "--execution-logs",
            str(self.exec_logs_dir),
            "--testcases",
            str(self.testcases_dir),
        ]

        exit_code = main()

        self.assertEqual(exit_code, 1)

    def test_input_path_is_directory_error(self):
        """Test that error occurs when input path is a directory."""
        input_dir = self.temp_path / "input_dir"
        input_dir.mkdir()
        output_dir = self.temp_path / "output"
        sys.argv = [
            "convert_verification_to_result_yaml.py",
            str(input_dir),
            "-o",
            str(output_dir),
            "--execution-logs",
            str(self.exec_logs_dir),
            "--testcases",
            str(self.testcases_dir),
        ]

        exit_code = main()

        self.assertEqual(exit_code, 1)

    def test_missing_execution_logs_error(self):
        """Test that error occurs when execution logs directory is missing."""
        input_file = self.temp_path / "input.json"
        input_file.write_text(json.dumps({"test_results": []}))
        
        output_dir = self.temp_path / "output"
        nonexistent_logs = self.temp_path / "nonexistent_logs"
        
        sys.argv = [
            "convert_verification_to_result_yaml.py",
            str(input_file),
            "-o",
            str(output_dir),
            "--execution-logs",
            str(nonexistent_logs),
            "--testcases",
            str(self.testcases_dir),
        ]

        exit_code = main()

        self.assertEqual(exit_code, 1)

    def test_missing_testcases_error(self):
        """Test that error occurs when testcases directory is missing."""
        input_file = self.temp_path / "input.json"
        input_file.write_text(json.dumps({"test_results": []}))
        
        output_dir = self.temp_path / "output"
        nonexistent_testcases = self.temp_path / "nonexistent_testcases"
        
        sys.argv = [
            "convert_verification_to_result_yaml.py",
            str(input_file),
            "-o",
            str(output_dir),
            "--execution-logs",
            str(self.exec_logs_dir),
            "--testcases",
            str(nonexistent_testcases),
        ]

        exit_code = main()

        self.assertEqual(exit_code, 1)


class TestMainStdinMultipleMode(unittest.TestCase):
    """Tests for main function with stdin input in multiple mode."""

    def setUp(self):
        """Create temporary directory for test files."""
        self.temp_dir = tempfile.mkdtemp()
        self.temp_path = Path(self.temp_dir)
        self.original_stdin = sys.stdin
        self.original_argv = sys.argv
        
        # Create dummy execution logs and testcases directories
        self.exec_logs_dir = self.temp_path / "logs"
        self.testcases_dir = self.temp_path / "testcases"
        self.exec_logs_dir.mkdir()
        self.testcases_dir.mkdir()

    def tearDown(self):
        """Clean up temporary directory and restore stdin/argv."""
        shutil.rmtree(self.temp_dir)
        sys.stdin = self.original_stdin
        sys.argv = self.original_argv

    def test_stdin_container_report_multiple_mode(self):
        """Test that multiple mode with stdin ContainerReport produces individual files."""
        container_report = {
            "test_results": [
                {
                    "test_case_id": "TC-STDIN-001",
                    "description": "Stdin test 1",
                    "sequences": [],
                    "total_steps": 1,
                    "passed_steps": 1,
                    "failed_steps": 0,
                    "not_executed_steps": 0,
                    "overall_pass": True,
                },
                {
                    "test_case_id": "TC-STDIN-002",
                    "description": "Stdin test 2",
                    "sequences": [],
                    "total_steps": 1,
                    "passed_steps": 0,
                    "failed_steps": 1,
                    "not_executed_steps": 0,
                    "overall_pass": False,
                },
            ]
        }

        # Mock stdin
        sys.stdin = StringIO(json.dumps(container_report))

        # Mock command line arguments (no input file, --multiple mode)
        output_dir = self.temp_path / "stdin_results"
        sys.argv = [
            "convert_verification_to_result_yaml.py",
            "-o",
            str(output_dir),
            "--multiple",
            "--execution-logs",
            str(self.exec_logs_dir),
            "--testcases",
            str(self.testcases_dir),
        ]

        # Run main
        exit_code = main()

        self.assertEqual(exit_code, 0)
        self.assertTrue((output_dir / "TC-STDIN-001_result.yaml").exists())
        self.assertTrue((output_dir / "TC-STDIN-002_result.yaml").exists())


class TestMainStdinSingleMode(unittest.TestCase):
    """Tests for main function with stdin input in single mode."""

    def setUp(self):
        """Create temporary directory for test files."""
        self.temp_dir = tempfile.mkdtemp()
        self.temp_path = Path(self.temp_dir)
        self.original_stdin = sys.stdin
        self.original_argv = sys.argv
        
        # Create dummy execution logs and testcases directories
        self.exec_logs_dir = self.temp_path / "logs"
        self.testcases_dir = self.temp_path / "testcases"
        self.exec_logs_dir.mkdir()
        self.testcases_dir.mkdir()

    def tearDown(self):
        """Clean up temporary directory and restore stdin/argv."""
        shutil.rmtree(self.temp_dir)
        sys.stdin = self.original_stdin
        sys.argv = self.original_argv

    def test_stdin_container_report_single_mode(self):
        """Test that single mode with stdin produces single array file."""
        container_report = {
            "test_results": [
                {
                    "test_case_id": "TC-STDIN-SINGLE-001",
                    "description": "Stdin single test 1",
                    "sequences": [],
                    "total_steps": 1,
                    "passed_steps": 1,
                    "failed_steps": 0,
                    "not_executed_steps": 0,
                    "overall_pass": True,
                },
                {
                    "test_case_id": "TC-STDIN-SINGLE-002",
                    "description": "Stdin single test 2",
                    "sequences": [],
                    "total_steps": 1,
                    "passed_steps": 0,
                    "failed_steps": 1,
                    "not_executed_steps": 0,
                    "overall_pass": False,
                },
            ]
        }

        # Mock stdin
        sys.stdin = StringIO(json.dumps(container_report))

        # Mock command line arguments (no input file, --single mode)
        output_file = self.temp_path / "stdin_single_results.yaml"
        sys.argv = [
            "convert_verification_to_result_yaml.py",
            "-o",
            str(output_file),
            "--single",
            "--execution-logs",
            str(self.exec_logs_dir),
            "--testcases",
            str(self.testcases_dir),
        ]

        # Run main
        exit_code = main()

        self.assertEqual(exit_code, 0)
        self.assertTrue(output_file.exists())

        # Verify content
        with open(output_file, "r", encoding="utf-8") as f:
            results = yaml.safe_load(f)

        self.assertIsInstance(results, list)
        self.assertEqual(len(results), 2)
        self.assertEqual(results[0]["type"], "test_result")
        self.assertEqual(results[1]["type"], "test_result")


class TestIntegration(unittest.TestCase):
    """Integration tests for complete workflows."""

    def setUp(self):
        """Create temporary directory for test files."""
        self.temp_dir = tempfile.mkdtemp()
        self.temp_path = Path(self.temp_dir)

    def tearDown(self):
        """Clean up temporary directory."""
        shutil.rmtree(self.temp_dir)

    def test_full_workflow_multiple_mode(self):
        """Test complete workflow from ContainerReport to multiple YAML files."""
        container_report = {
            "test_results": [
                {
                    "test_case_id": "TC-FULL-001",
                    "description": "Full test 1",
                    "requirement": "REQ-001",
                    "item": 1,
                    "tc": 1,
                    "sequences": [
                        {
                            "sequence_id": 1,
                            "name": "Sequence 1",
                            "step_results": [
                                {"Pass": {"step": 1, "description": "Passed step"}},
                                {
                                    "Fail": {
                                        "step": 2,
                                        "description": "Failed step",
                                        "expected": {"result": "0", "output": ""},
                                        "actual_result": "1",
                                        "actual_output": "",
                                        "reason": "Mismatch",
                                    }
                                },
                            ],
                            "all_steps_passed": False,
                        }
                    ],
                    "total_steps": 2,
                    "passed_steps": 1,
                    "failed_steps": 1,
                    "not_executed_steps": 0,
                    "overall_pass": False,
                },
                {
                    "test_case_id": "TC-FULL-002",
                    "description": "Full test 2",
                    "sequences": [],
                    "total_steps": 1,
                    "passed_steps": 1,
                    "failed_steps": 0,
                    "not_executed_steps": 0,
                    "overall_pass": True,
                },
            ]
        }

        input_file = self.temp_path / "container_report.json"
        with open(input_file, "w", encoding="utf-8") as f:
            json.dump(container_report, f)

        output_dir = self.temp_path / "results"
        count = process_verification_json(
            input_file, output_dir, None, None, output_mode="multiple", verbose=True
        )

        self.assertEqual(count, 2)

        # Verify TC-FULL-001
        file1 = output_dir / "TC-FULL-001_result.yaml"
        self.assertTrue(file1.exists())
        with open(file1, "r", encoding="utf-8") as f:
            result1 = yaml.safe_load(f)

        self.assertEqual(result1["type"], "test_result")
        self.assertEqual(result1["test_case_id"], "TC-FULL-001")
        self.assertEqual(result1["requirement"], "REQ-001")
        self.assertEqual(result1["item"], 1)
        self.assertEqual(len(result1["sequences"]), 1)
        self.assertEqual(len(result1["sequences"][0]["step_results"]), 2)
        self.assertIn("Pass", result1["sequences"][0]["step_results"][0])
        self.assertIn("Fail", result1["sequences"][0]["step_results"][1])

        # Verify TC-FULL-002
        file2 = output_dir / "TC-FULL-002_result.yaml"
        self.assertTrue(file2.exists())
        with open(file2, "r", encoding="utf-8") as f:
            result2 = yaml.safe_load(f)

        self.assertEqual(result2["type"], "test_result")
        self.assertEqual(result2["test_case_id"], "TC-FULL-002")
        self.assertTrue(result2["overall_pass"])

    def test_full_workflow_single_mode(self):
        """Test complete workflow from ContainerReport to single YAML array file."""
        container_report = {
            "test_results": [
                {
                    "test_case_id": "TC-SINGLE-FULL-001",
                    "description": "Single full test 1",
                    "sequences": [],
                    "total_steps": 1,
                    "passed_steps": 1,
                    "failed_steps": 0,
                    "not_executed_steps": 0,
                    "overall_pass": True,
                },
                {
                    "test_case_id": "TC-SINGLE-FULL-002",
                    "description": "Single full test 2",
                    "sequences": [],
                    "total_steps": 1,
                    "passed_steps": 0,
                    "failed_steps": 1,
                    "not_executed_steps": 0,
                    "overall_pass": False,
                },
            ]
        }

        input_file = self.temp_path / "container_report.json"
        with open(input_file, "w", encoding="utf-8") as f:
            json.dump(container_report, f)

        output_file = self.temp_path / "all_results.yaml"
        count = process_verification_json(
            input_file, output_file, None, None, output_mode="single", verbose=True
        )

        self.assertEqual(count, 2)
        self.assertTrue(output_file.exists())

        # Verify YAML array
        with open(output_file, "r", encoding="utf-8") as f:
            results = yaml.safe_load(f)

        self.assertIsInstance(results, list)
        self.assertEqual(len(results), 2)

        # Verify each result has type: test_result
        for result in results:
            self.assertEqual(result["type"], "test_result")

        self.assertEqual(results[0]["test_case_id"], "TC-SINGLE-FULL-001")
        self.assertTrue(results[0]["overall_pass"])
        self.assertEqual(results[1]["test_case_id"], "TC-SINGLE-FULL-002")
        self.assertFalse(results[1]["overall_pass"])


def run_tests():
    """Run all tests and return exit code."""
    loader = unittest.TestLoader()
    suite = loader.loadTestsFromModule(sys.modules[__name__])
    runner = unittest.TextTestRunner(verbosity=2)
    result = runner.run(suite)
    return 0 if result.wasSuccessful() else 1


if __name__ == "__main__":
    if not YAML_AVAILABLE:
        print("Error: PyYAML is required. Install with: pip3 install pyyaml")
        sys.exit(1)

    sys.exit(run_tests())
