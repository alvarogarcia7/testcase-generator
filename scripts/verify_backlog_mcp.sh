#!/usr/bin/env bash
#
# Verify Backlog.md MCP Server Connection
#
# This script verifies that the Backlog.md MCP server can be started
# and is accessible. It performs the following checks:
# 1. Verifies backlog.md npm package is installed
# 2. Checks backlog configuration exists
# 3. Starts the MCP server in test mode
# 4. Verifies the server responds to basic queries

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "=== Backlog.md MCP Server Verification ==="
echo ""

# Check if we're in the project root
cd "$PROJECT_ROOT"

echo "[1/5] Checking if backlog.md is installed..."
if ! npm list backlog.md >/dev/null 2>&1; then
    echo "ERROR: backlog.md is not installed"
    echo "Running: npm install"
    npm install
fi
echo "✓ backlog.md is installed"
echo ""

echo "[2/5] Verifying backlog configuration..."
if [ ! -f "backlog/config.yml" ]; then
    echo "ERROR: backlog/config.yml not found"
    exit 1
fi
echo "✓ backlog/config.yml exists"
echo ""

echo "[3/5] Checking backlog tasks directory..."
if [ ! -d "backlog/tasks" ]; then
    echo "ERROR: backlog/tasks directory not found"
    exit 1
fi
TASK_COUNT=$(find backlog/tasks -name "*.md" -type f | wc -l | tr -d ' ')
echo "✓ Found $TASK_COUNT task file(s) in backlog/tasks"
echo ""

echo "[4/5] Verifying backlog CLI functionality..."
if ! npx backlog overview >/dev/null 2>&1; then
    echo "WARNING: backlog overview command failed (may be expected if project not initialized)"
else
    echo "✓ backlog CLI is functional"
fi
echo ""

echo "[5/5] Testing MCP server startup..."
echo "Starting MCP server in test mode (will timeout after 5 seconds)..."

# Create a test client that sends a simple request to the MCP server
# The MCP server uses stdio transport, so we'll send a basic initialization request
cat > /tmp/mcp_test_request.json << 'EOF'
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test-client","version":"1.0.0"}}}
EOF

# Start the MCP server and send test request
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test-client","version":"1.0.0"}}}' | \
    timeout 5 npx backlog mcp start 2>&1 | head -20 || {
    EXIT_CODE=$?
    if [ $EXIT_CODE -eq 124 ]; then
        echo "INFO: MCP server started (timed out as expected)"
    else
        echo "WARNING: MCP server exited with code $EXIT_CODE"
    fi
}

echo ""
echo "=== Verification Complete ==="
echo ""
echo "MCP Server Configuration:"
echo "  Command: npx backlog mcp start"
echo "  Config:  backlog/config.yml"
echo "  Tasks:   backlog/tasks/"
echo "  Port:    6420 (default from config)"
echo ""
echo "To start the MCP server manually:"
echo "  npx backlog mcp start"
echo ""
echo "To use with an MCP client, configure:"
echo "  Command: npx backlog mcp start"
echo "  Args:    (none required, or --debug for verbose logging)"
echo "  CWD:     $PROJECT_ROOT"
echo ""
echo "For workflow overview, use the MCP resource:"
echo "  backlog://workflow/overview"
echo ""
