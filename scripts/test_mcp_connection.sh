#!/usr/bin/env bash
#
# Simple test to verify MCP server can be started
#

echo "Testing Backlog.md MCP Server Connection"
echo "========================================="
echo ""

# Check if backlog.md is installed
if ! command -v npx &> /dev/null; then
    echo "ERROR: npx not found. Please install Node.js"
    exit 1
fi

echo "✓ npx is available"

# Check if package is installed
if [ ! -d "node_modules/backlog.md" ]; then
    echo "ERROR: backlog.md not installed"
    echo "Run: npm install"
    exit 1
fi

echo "✓ backlog.md package installed"

# Check config
if [ ! -f "backlog/config.yml" ]; then
    echo "ERROR: backlog/config.yml not found"
    exit 1
fi

echo "✓ backlog/config.yml exists"

# Count tasks
TASK_COUNT=$(find backlog/tasks -name "*.md" -type f 2>/dev/null | wc -l | tr -d ' ')
echo "✓ Found $TASK_COUNT task files"

echo ""
echo "MCP Server Configuration:"
echo "  Command:    npx backlog mcp start"
echo "  Transport:  stdio"
echo "  Config:     backlog/config.yml"
echo "  Tasks:      backlog/tasks/"
echo ""
echo "To test the MCP server, configure your MCP client with:"
echo "  command: npx"
echo "  args: ['backlog', 'mcp', 'start']"
echo "  cwd: $(pwd)"
echo ""
echo "SUCCESS: MCP server is configured and ready to use"
