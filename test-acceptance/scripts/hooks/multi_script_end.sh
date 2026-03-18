#!/usr/bin/env bash

# Multi-hook script_end (on_error: continue)
echo "script_end executed" >> /tmp/multi_hooks_$$.log

# Note: We don't cleanup /tmp/multi_hooks_$$.log here
# The test needs to verify the log content after script completes
