#!/bin/bash
set -e

echo "Building Test Case Manager Docker image..."
docker build -t testcase-manager:latest .

echo ""
echo "Build complete! To run the container:"
echo "  docker run -it --rm testcase-manager:latest"
echo ""
echo "To verify binaries are present:"
echo "  docker run --rm testcase-manager:latest ls -la /usr/local/bin"
echo ""
echo "To check the README:"
echo "  docker run --rm testcase-manager:latest cat /root/README.md"
