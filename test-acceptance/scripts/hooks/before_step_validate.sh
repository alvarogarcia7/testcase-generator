#!/usr/bin/env bash
set -e

# Before step hook - performs validation
STEP_NUMBER="${TEST_STEP_NUMBER:-unknown}"
STEP_DESC="${TEST_STEP_DESCRIPTION:-unknown}"

echo "before_step validated: Step number: $STEP_NUMBER" >> /tmp/before_step_$$.log
