#!/bin/bash
set -e

echo "=== Validating JSON Schemas ==="

# Validate schema versions
for f in schemas/tcms/*.schema.json; do
    version=$(jq -r '."$schema"' "$f")
    if [ "$version" != "http://json-schema.org/draft-07/schema#" ]; then
        echo "ERROR: $f not Draft-07"
        exit 1
    fi
done
echo "✓ All schemas are Draft-07"

# Validate JSON syntax
for f in schemas/tcms/*.schema.json; do
    jq empty "$f" || { echo "ERROR: Invalid JSON in $f"; exit 1; }
done
echo "✓ All JSON is valid"

# Validate schema structure
for f in schemas/tcms/*.schema.json; do
    type=$(jq -r '.type' "$f")
    props=$(jq -r '.properties' "$f")
    if [ "$type" = "null" ] || [ "$props" = "null" ]; then
        echo "ERROR: $f missing required fields"
        exit 1
    fi
done
echo "✓ All schemas have correct structure"

# Validate samples
echo "Validating samples..."
for f in samples/test-case/*.yml; do
    if [ -f "$f" ]; then
        uv run python validator.py "$f" schemas/tcms/test-case-main.schema.json
    fi
done

for f in samples/test-plan/*.yml; do
    if [ -f "$f" ]; then
        uv run python validator.py "$f" schemas/test-plan/test-plan.schema.json
    fi
done
echo "✓ All samples are valid"

echo ""
echo "=== All validations passed! ==="
