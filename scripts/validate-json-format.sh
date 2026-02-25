#!/bin/bash
set -e

echo "=== Validating JSON file formatting ==="

# Validate all JSON files in schemas directory
for f in schemas/**/*.json; do
    if [ -f "$f" ]; then
        echo "Checking: $f"

        # Validate JSON is well-formed
        if ! jq empty "$f" 2>/dev/null; then
            echo "ERROR: Invalid JSON in $f"
            exit 1
        fi

        # Validate formatting (should be properly indented with 2 spaces)
        # Create a temp file with formatted JSON
        temp_file=$(mktemp)
        jq '.' "$f" > "$temp_file"

        # Compare original with formatted version
        if ! diff -q "$f" "$temp_file" > /dev/null 2>&1; then
            echo "ERROR: $f is not properly formatted"
            rm "$temp_file"
            exit 1
        fi

        rm "$temp_file"
        echo "✓ $f is properly formatted"
    fi
done

echo ""
echo "=== All JSON files are properly formatted! ==="
