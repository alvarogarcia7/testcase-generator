#!/usr/bin/env bash

# After step hook - logs variables including captured ones (on_error: continue)
STEP_NUMBER="${TEST_STEP_NUMBER:-unknown}"
echo "Step $STEP_NUMBER variables:" >> /tmp/hook_vars_$$.log
echo "SEQUENCE_VAR=${SEQUENCE_VAR:-not_set}" >> /tmp/hook_vars_$$.log
echo "CAPTURED_VAR=${CAPTURED_VAR:-not_captured_yet}" >> /tmp/hook_vars_$$.log
