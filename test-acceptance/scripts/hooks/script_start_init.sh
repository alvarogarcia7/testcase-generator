#!/usr/bin/env bash
set -e

# Script start hook - initializes global test execution context
# This hook executes once at the very beginning of the test script

SCRIPT_START_TIME=$(date +%s)
echo "script_start: Test execution started at $(date)" >> /tmp/script_start_$$.log
echo "script_start: PID=$$" >> /tmp/script_start_$$.log
