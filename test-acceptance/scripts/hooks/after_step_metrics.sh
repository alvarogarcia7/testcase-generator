#!/usr/bin/env bash

# After step hook - collects metrics (on_error: continue)
STEP_NUM="${TEST_STEP_NUMBER:-unknown}"
EXIT="${STEP_EXIT_CODE:-unknown}"

echo "after_step metrics: Step $STEP_NUM - Exit code: $EXIT" >> /tmp/after_step_metrics_$$.log
