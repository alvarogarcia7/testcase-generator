#!/usr/bin/env bash

# Multi-hook script_end (on_error: continue)
echo "script_end executed" >> /tmp/multi_hooks_$$.log

# Cleanup
rm -f /tmp/multi_hooks_$$.log
