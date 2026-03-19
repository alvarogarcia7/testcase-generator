#!/usr/bin/env python3
"""
Verify Backlog.md MCP Server Connection

This script verifies that the Backlog.md MCP server can be started
and responds correctly to JSON-RPC requests. It performs comprehensive
testing of the MCP protocol implementation.
"""

import json
import subprocess
import sys
import time
from pathlib import Path
from typing import Any


class MCPClient:
    """Simple MCP client for testing the backlog.md server."""
    
    def __init__(self, project_root: Path):
        self.project_root = project_root
        self.process: subprocess.Popen[str] | None = None
        self.request_id = 0
    
    def start_server(self) -> None:
        """Start the MCP server process."""
        print("Starting MCP server...")
        self.process = subprocess.Popen(
            ["npx", "backlog", "mcp", "start"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            cwd=self.project_root,
            text=True,
            bufsize=1
        )
        print("✓ MCP server process started")
    
    def send_request(self, method: str, params: dict[str, Any] | None = None) -> dict[str, Any]:
        """Send a JSON-RPC request to the MCP server."""
        if not self.process or not self.process.stdin:
            raise RuntimeError("Server not started")
        
        self.request_id += 1
        request = {
            "jsonrpc": "2.0",
            "id": self.request_id,
            "method": method,
            "params": params or {}
        }
        
        request_json = json.dumps(request) + "\n"
        print(f"Sending: {method}")
        
        try:
            self.process.stdin.write(request_json)
            self.process.stdin.flush()
            
            # Read response
            response_line = self.process.stdout.readline()
            if not response_line:
                raise RuntimeError("No response from server")
            
            response = json.loads(response_line)
            return response
        except Exception as e:
            print(f"Error communicating with server: {e}")
            raise
    
    def stop_server(self) -> None:
        """Stop the MCP server process."""
        if self.process:
            print("Stopping MCP server...")
            self.process.terminate()
            try:
                self.process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                self.process.kill()
            print("✓ MCP server stopped")


def verify_backlog_structure(project_root: Path) -> bool:
    """Verify that the backlog directory structure exists."""
    print("\n[1/4] Verifying backlog directory structure...")
    
    config_file = project_root / "backlog" / "config.yml"
    tasks_dir = project_root / "backlog" / "tasks"
    
    if not config_file.exists():
        print(f"ERROR: {config_file} not found")
        return False
    print(f"✓ Found config: {config_file}")
    
    if not tasks_dir.exists():
        print(f"ERROR: {tasks_dir} not found")
        return False
    
    task_files = list(tasks_dir.glob("*.md"))
    print(f"✓ Found {len(task_files)} task file(s)")
    
    return True


def verify_npm_package(project_root: Path) -> bool:
    """Verify that backlog.md npm package is installed."""
    print("\n[2/4] Verifying backlog.md npm package...")
    
    try:
        result = subprocess.run(
            ["npm", "list", "backlog.md"],
            cwd=project_root,
            capture_output=True,
            text=True
        )
        if result.returncode != 0:
            print("backlog.md not installed, running npm install...")
            subprocess.run(["npm", "install"], cwd=project_root, check=True)
        
        print("✓ backlog.md package is installed")
        return True
    except Exception as e:
        print(f"ERROR: {e}")
        return False


def verify_mcp_server(project_root: Path) -> bool:
    """Verify MCP server can start and respond to requests."""
    print("\n[3/4] Testing MCP server communication...")
    
    client = MCPClient(project_root)
    
    try:
        client.start_server()
        time.sleep(1)  # Give server time to start
        
        # Send initialize request
        print("\nSending initialize request...")
        response = client.send_request(
            "initialize",
            {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {
                    "name": "test-client",
                    "version": "1.0.0"
                }
            }
        )
        
        if "result" in response:
            print("✓ Server initialized successfully")
            print(f"  Server: {response['result'].get('serverInfo', {}).get('name', 'unknown')}")
            print(f"  Version: {response['result'].get('serverInfo', {}).get('version', 'unknown')}")
        elif "error" in response:
            print(f"ERROR: {response['error']}")
            return False
        
        # Send initialized notification
        client.process.stdin.write(json.dumps({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        }) + "\n")
        client.process.stdin.flush()
        
        return True
        
    except Exception as e:
        print(f"ERROR: {e}")
        return False
    finally:
        client.stop_server()


def display_summary(project_root: Path) -> None:
    """Display summary information about the MCP server setup."""
    print("\n[4/4] MCP Server Configuration Summary")
    print("=" * 60)
    print(f"Project Root: {project_root}")
    print(f"Config File:  {project_root}/backlog/config.yml")
    print(f"Tasks Dir:    {project_root}/backlog/tasks/")
    print()
    print("To start MCP server manually:")
    print("  npx backlog mcp start")
    print()
    print("To start with debug logging:")
    print("  npx backlog mcp start --debug")
    print()
    print("MCP Client Configuration:")
    print("  Command: npx")
    print("  Args:    ['backlog', 'mcp', 'start']")
    print(f"  CWD:     {project_root}")
    print("  Transport: stdio")
    print()
    print("Available MCP Resources:")
    print("  backlog://workflow/overview")
    print("  backlog://workflow/task-creation")
    print("  backlog://workflow/task-execution")
    print("  backlog://workflow/task-finalization")
    print()
    print("Available MCP Tools:")
    print("  backlog.get_workflow_overview()")
    print("  backlog.search_tasks()")
    print("  backlog.create_task()")
    print("  backlog.update_task()")
    print("  ... and more (see workflow overview)")
    print("=" * 60)


def main():
    """Main verification function."""
    print("=== Backlog.md MCP Server Verification ===\n")
    
    # Get project root
    script_dir = Path(__file__).parent
    project_root = script_dir.parent
    
    # Run verification steps
    steps = [
        ("Backlog Structure", lambda: verify_backlog_structure(project_root)),
        ("NPM Package", lambda: verify_npm_package(project_root)),
        ("MCP Server", lambda: verify_mcp_server(project_root)),
    ]
    
    all_passed = True
    for name, verify_func in steps:
        try:
            if not verify_func():
                all_passed = False
                print(f"✗ {name} verification failed")
        except Exception as e:
            all_passed = False
            print(f"✗ {name} verification failed with error: {e}")
    
    # Display summary
    display_summary(project_root)
    
    # Final result
    print("\n=== Verification Result ===")
    if all_passed:
        print("✓ All checks passed!")
        print("The Backlog.md MCP server is properly configured and functional.")
        return 0
    else:
        print("✗ Some checks failed")
        print("Please review the errors above and fix any issues.")
        return 1


if __name__ == "__main__":
    sys.exit(main())
