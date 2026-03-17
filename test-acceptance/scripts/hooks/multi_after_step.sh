#!/usr/bin/env bash

# Multi-hook after_step (on_error: continue)
STEP_NUMBER="${TEST_STEP_NUMBER:-unknown}"
EXIT_CODE="${STEP_EXIT_CODE:-unknown}"
echo "after_step executed: Step $STEP_NUMBER (exit: $EXIT_CODE)" >> /tmp/multi_hooks_$$.log
