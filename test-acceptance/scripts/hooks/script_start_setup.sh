#!/usr/bin/env bash
set -e

# Script start hook - sets up environment variables and workspace
echo "script_start executed" > /tmp/hook_script_start_$$.txt
export HOOK_WORKSPACE="/tmp/hook_workspace"
