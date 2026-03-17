# Backlog.md MCP Server Setup and Usage

This document describes how to set up and use the Backlog.md MCP (Model Context Protocol) server for task management in this project.

## Overview

The Backlog.md MCP server provides a standardized interface for AI assistants and other tools to interact with the project's task management system. It implements the Model Context Protocol, enabling tools to:

- Search and retrieve task information
- Create and update tasks
- Access workflow documentation
- Query project statistics and milestones
- Manage task dependencies and sequences

## Prerequisites

- Node.js and npm installed
- The backlog.md package is listed in `package.json`

## Installation

The backlog.md package should be installed automatically when running:

```bash
npm install
```

This will install:
- `backlog.md` (version ^1.36.0)
- The MCP server component

## Configuration

The MCP server uses the configuration in `backlog/config.yml`:

```yaml
project_name: "TCMS Project"
default_status: "To Do"
statuses: ["To Do", "In Progress", "In Review", "Done"]
task_prefix: "TCMS"
default_port: 6420
remote_operations: true
# ... other settings
```

Key configuration options:
- **project_name**: Display name for the project
- **task_prefix**: Prefix for task IDs (e.g., TCMS-1, TCMS-2)
- **default_port**: Port for browser interface (not used for MCP stdio transport)
- **remote_operations**: Allow remote access (for browser interface)

## Starting the MCP Server

### Manual Start

To start the MCP server manually:

```bash
npx backlog mcp start
```

With debug logging:

```bash
npx backlog mcp start --debug
```

From a specific directory:

```bash
npx backlog mcp start --cwd /path/to/project
```

### MCP Client Configuration

For AI assistants or other MCP clients, configure the server as follows:

**Command**: `npx`

**Arguments**: `["backlog", "mcp", "start"]`

**Working Directory**: Path to this project root

**Transport**: stdio (standard input/output)

**Example Claude Desktop Configuration** (`~/Library/Application Support/Claude/claude_desktop_config.json` on macOS):

```json
{
  "mcpServers": {
    "backlog-tcms": {
      "command": "npx",
      "args": ["backlog", "mcp", "start"],
      "cwd": "/absolute/path/to/project",
      "env": {}
    }
  }
}
```

## Verification

### Quick Verification

Run the verification script to check the MCP server setup:

```bash
./scripts/verify_backlog_mcp.sh
```

### Comprehensive Verification

Run the Python verification script for detailed testing:

```bash
python3 scripts/verify_backlog_mcp.py
```

This script will:
1. Verify backlog directory structure
2. Check npm package installation
3. Test MCP server communication
4. Display configuration summary

## Available MCP Resources

When connected via an MCP client, the following resources are available:

### Workflow Resources

- **`backlog://workflow/overview`** - Complete workflow overview and guidelines
- **`backlog://workflow/task-creation`** - Guide for creating tasks
- **`backlog://workflow/task-execution`** - Guide for executing tasks
- **`backlog://workflow/task-finalization`** - Guide for finalizing completed tasks

### Usage Example

```
Read the resource: backlog://workflow/overview
```

This provides comprehensive information about:
- When to create tasks vs. ad-hoc work
- Search-first workflow to avoid duplicates
- Task creation guidelines
- Task execution best practices
- Available MCP tools

## Available MCP Tools

The server exposes various tools for task management:

### Core Tools

- `backlog.get_workflow_overview()` - Get workflow documentation
- `backlog.search_tasks(query, filters)` - Search tasks by text or metadata
- `backlog.get_task(taskId)` - Retrieve specific task details
- `backlog.create_task(title, description, ...)` - Create new task
- `backlog.update_task(taskId, updates)` - Update existing task
- `backlog.list_tasks(filters)` - List all tasks with optional filters

### Advanced Tools

- `backlog.get_project_overview()` - Get project statistics
- `backlog.list_sequences()` - Get task execution sequences
- `backlog.search_documents(query)` - Search documentation
- `backlog.search_decisions(query)` - Search decision records

For a complete list, read the `backlog://workflow/overview` resource.

## Workflow Guidelines

### When to Use Backlog

According to the project's AGENTS.md and CLAUDE.md:

**DO create a task for**:
- Features requiring multi-step implementation
- Bug fixes that span multiple components
- Refactoring that takes significant effort
- Work that needs tracking and documentation

**DO NOT create a task for**:
- Simple one-liner fixes
- Documentation updates
- Trivial code cleanup
- Work that's already in progress

### Search-First Workflow

Before creating a new task:

1. Search existing tasks to avoid duplicates
2. Check if related work is already tracked
3. Link dependencies if the task builds on existing work

### Task Creation Best Practices

- Use clear, descriptive titles
- Include detailed descriptions
- Set appropriate status and labels
- Define clear acceptance criteria
- Link dependencies where applicable

## Directory Structure

```
backlog/
├── config.yml          # MCP server configuration
├── tasks/              # Active task files
│   ├── tcms-1 - Create-an-acceptance-test-for-showing-variables.md
│   ├── tcms-2 - Set-up-MKDOCS-for-documentation-of-the-project.md
│   ├── TCMS-12: Acceptance Testing.md
│   └── ... (other tasks)
└── completed/          # Archived completed tasks (if cleanup run)
```

Each task file contains:
- YAML frontmatter with metadata
- Description section
- Definition of done checklist

## Troubleshooting

### Server Won't Start

Check that:
- npm packages are installed: `npm install`
- backlog/config.yml exists
- You're running from the project root

### No Response from Server

- Ensure the server is using stdio transport (not HTTP)
- Check that requests are valid JSON-RPC 2.0 format
- Enable debug logging: `npx backlog mcp start --debug`

### Tasks Not Found

- Verify task files exist in `backlog/tasks/`
- Check task file format (YAML frontmatter required)
- Run `npx backlog overview` to see task count

### MCP Client Can't Connect

- Verify the command and args are correct
- Ensure the working directory points to project root
- Check client logs for connection errors

## Additional Commands

### View Project Overview

```bash
npx backlog overview
```

Shows:
- Total task count by status
- Recent activity
- Upcoming milestones

### Search Tasks (CLI)

```bash
npx backlog search "keyword"
```

### Open Browser Interface

```bash
npx backlog browser
```

Opens web interface at http://localhost:6420

### Task Board View

```bash
npx backlog board
```

Displays Kanban board in terminal

## References

- **AGENTS.md** - Lines 726-753: Backlog.md MCP guidelines
- **CLAUDE.md** - Lines 726-753: Same guidelines for Claude
- **backlog/config.yml** - Server configuration
- **Backlog.md Documentation**: https://github.com/cyanheads/backlog.md

## Support

For issues specific to this project's backlog setup, check:
1. The verification scripts output
2. AGENTS.md workflow instructions
3. Existing task examples in `backlog/tasks/`

For Backlog.md MCP server issues, refer to the official documentation.
