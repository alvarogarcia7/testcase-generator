#!/bin/bash

set -euo pipefail

# Demo script for JUnit XML export functionality

echo "=== JUnit XML Export Demo ==="
echo

# Check if the binary exists
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo not found. Please install Rust."
    exit 1
fi

echo "1. Exporting test runs to stdout:"
echo "   cargo run --bin tcm -- export-junit-xml data/sample_test_runs.json"
echo
cargo run --bin tcm -- export-junit-xml data/sample_test_runs.json
echo

echo "2. Exporting test runs to a file:"
echo "   cargo run --bin tcm -- export-junit-xml data/sample_test_runs.json -o /tmp/junit-results.xml"
echo
cargo run --bin tcm -- export-junit-xml data/sample_test_runs.json -o /tmp/junit-results.xml



if [ ! -f /tmp/junit-results.xml ]; then
    echo "Error: file not generated"
    exit 1
fi

echo
echo "✓ File created successfully: /tmp/junit-results.xml"
echo
echo "3. Contents of generated file:"
cat /tmp/junit-results.xml
echo


echo
echo "=== Demo Complete ==="
