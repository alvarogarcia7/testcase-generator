#!/bin/bash

# Test script for test-orchestrator examples
# Tests all subcommands with example data

set -e

echo "=========================================="
echo "Testing test-orchestrator example data"
echo "=========================================="
echo ""

# Setup test directories
TEST_DIR_RUN="/tmp/orchestrator-run-test-$$"
TEST_DIR_RUN_ALL="/tmp/orchestrator-run-all-test-$$"

echo "Setting up test directories..."
mkdir -p "$TEST_DIR_RUN"
mkdir -p "$TEST_DIR_RUN_ALL"

# Copy test files
echo "Copying test files..."
cp examples/EXAMPLE_RUN_001.yml "$TEST_DIR_RUN/"
cp examples/EXAMPLE_RUN_002.yml "$TEST_DIR_RUN/"
cp examples/EXAMPLE_RUN_ALL_A.yml "$TEST_DIR_RUN_ALL/"
cp examples/EXAMPLE_RUN_ALL_B.yml "$TEST_DIR_RUN_ALL/"

echo "Test directories created:"
echo "  - $TEST_DIR_RUN"
echo "  - $TEST_DIR_RUN_ALL"
echo ""

# Test 1: run subcommand with single test case
echo "=========================================="
echo "Test 1: 'run' subcommand - single test"
echo "=========================================="
cargo run --bin test-orchestrator -- run EXAMPLE_RUN_001 -p "$TEST_DIR_RUN"
echo "✓ Test 1 passed"
echo ""

# Test 2: run subcommand with multiple test cases
echo "=========================================="
echo "Test 2: 'run' subcommand - multiple tests"
echo "=========================================="
cargo run --bin test-orchestrator -- run EXAMPLE_RUN_001 EXAMPLE_RUN_002 -p "$TEST_DIR_RUN"
echo "✓ Test 2 passed"
echo ""

# Test 3: run-all subcommand
echo "=========================================="
echo "Test 3: 'run-all' subcommand"
echo "=========================================="
cargo run --bin test-orchestrator -- run-all -p "$TEST_DIR_RUN_ALL"
echo "✓ Test 3 passed"
echo ""

# Test 4: verify subcommand
echo "=========================================="
echo "Test 4: 'verify' subcommand"
echo "=========================================="
cargo run --bin test-orchestrator -- verify \
  --test-case examples/EXAMPLE_VERIFY_001.yml \
  --execution-log examples/EXAMPLE_VERIFY_001_execution_log.json
echo "✓ Test 4 passed"
echo ""

# Test 5: info subcommand
echo "=========================================="
echo "Test 5: 'info' subcommand"
echo "=========================================="
cargo run --bin test-orchestrator -- info -p "$TEST_DIR_RUN" >/dev/null
echo "✓ Test 5 passed"
echo ""

# Cleanup
echo "=========================================="
echo "Cleaning up test directories..."
echo "=========================================="
rm -rf "$TEST_DIR_RUN"
rm -rf "$TEST_DIR_RUN_ALL"
echo "Cleanup complete"
echo ""

# Summary
echo "=========================================="
echo "ALL ORCHESTRATOR EXAMPLE TESTS PASSED!"
echo "=========================================="
echo ""
echo "Summary:"
echo "  ✓ 5/5 tests passed"
echo "  ✓ All subcommands validated:"
echo "    - run (single and multiple)"
echo "    - run-all"
echo "    - verify"
echo "    - info"
