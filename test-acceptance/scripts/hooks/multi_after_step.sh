#!/usr/bin/env bash

# Multi-hook after_step (on_error: continue)
STEP_NUM="${TEST_STEP_NUMBER:-unknown}"
EXIT="${STEP_EXIT_CODE:-unknown}"
echo "after_step executed: Step $STEP_NUM (exit: $EXIT)" >> /tmp/multi_hooks_$$.log
