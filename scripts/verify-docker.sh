#!/bin/bash
set -e

IMAGE_NAME="testcase-manager:latest"

echo "=================================="
echo "Docker Image Verification Script"
echo "=================================="
echo ""

# Check if image exists
if ! docker image inspect "$IMAGE_NAME" >/dev/null 2>&1; then
    echo "ERROR: Image '$IMAGE_NAME' not found."
    echo "Please build the image first using: ./scripts/build-docker.sh"
    exit 1
fi

echo "✓ Image '$IMAGE_NAME' found"
echo ""

# Verify binaries are present
echo "Checking binaries..."
BINARIES=(
    "tcm"
    "validate-yaml"
    "validate-json"
    "trm"
    "test-verify"
    "test-executor"
    "editor"
    "test-orchestrator"
)

for binary in "${BINARIES[@]}"; do
    if docker run --rm "$IMAGE_NAME" test -f "/usr/local/bin/$binary"; then
        echo "  ✓ $binary"
    else
        echo "  ✗ $binary - MISSING"
        exit 1
    fi
done

echo ""

# Verify data directory
echo "Checking data directory..."
if docker run --rm "$IMAGE_NAME" test -d "/app/data"; then
    echo "  ✓ /app/data exists"
else
    echo "  ✗ /app/data - MISSING"
    exit 1
fi

echo ""

# Verify README
echo "Checking README..."
if docker run --rm "$IMAGE_NAME" test -f "/root/README.md"; then
    echo "  ✓ /root/README.md exists"
else
    echo "  ✗ /root/README.md - MISSING"
    exit 1
fi

echo ""

# Test binary execution
echo "Testing binary execution..."
if docker run --rm "$IMAGE_NAME" tcm --version >/dev/null 2>&1; then
    echo "  ✓ tcm executes successfully"
else
    echo "  ✗ tcm failed to execute"
    exit 1
fi

echo ""

# Check for unwanted files in /usr/local/bin
echo "Checking for extra files in /usr/local/bin..."
EXTRA_FILES=$(docker run --rm "$IMAGE_NAME" sh -c 'ls -A /usr/local/bin | grep -v "^tcm$\|^validate-yaml$\|^validate-json$\|^trm$\|^test-verify$\|^test-executor$\|^editor$\|^test-orchestrator$" || true')

if [ -z "$EXTRA_FILES" ]; then
    echo "  ✓ No extra files found"
else
    echo "  ⚠ Extra files found in /usr/local/bin:"
    echo "$EXTRA_FILES" | sed 's/^/    /'
fi

echo ""
echo "=================================="
echo "✓ All checks passed!"
echo "=================================="
echo ""
echo "To run the container:"
echo "  docker run -it --rm $IMAGE_NAME"
echo ""
echo "To view the README:"
echo "  docker run --rm $IMAGE_NAME cat /root/README.md"
