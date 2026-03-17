#!/usr/bin/env bash

# After step hook - collects metrics (on_error: continue)
STEP_NUMBER="${TEST_STEP_NUMBER:-unknown}"
EXIT_CODE="${STEP_EXIT_CODE:-unknown}"

echo "after_step metrics: Step $STEP_NUMBER - Exit code: $EXIT_CODE" >> /tmp/after_step_metrics_$$.log
