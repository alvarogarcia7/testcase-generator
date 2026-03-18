#!/usr/bin/env bash

# After step hook - logs step context including exit code (on_error: continue)
STEP_NUMBER="${TEST_STEP_NUMBER:-unknown}"
STEP_DESC="${TEST_STEP_DESCRIPTION:-unknown}"
EXIT_CODE="${STEP_EXIT_CODE:-unknown}"
SEQUENCE_ID="${TEST_SEQUENCE_ID:-unknown}"

echo "after_step: TEST_SEQUENCE_ID=$SEQUENCE_ID, TEST_STEP_NUMBER=$STEP_NUMBER, STEP_EXIT_CODE=$EXIT_CODE" >> /tmp/step_context_$$.log
