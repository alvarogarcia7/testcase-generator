#!/usr/bin/env bash
set -e

# Before step hook - logs step context
STEP_NUMBER="${TEST_STEP_NUMBER:-unknown}"
STEP_DESC="${TEST_STEP_DESCRIPTION:-unknown}"
SEQUENCE_ID="${TEST_SEQUENCE_ID:-unknown}"

echo "before_step: TEST_SEQUENCE_ID=$SEQUENCE_ID, TEST_STEP_NUMBER=$STEP_NUMBER, TEST_STEP_DESCRIPTION=$STEP_DESC" >> /tmp/step_context_$$.log
