#!/usr/bin/env bash
# Hook: before_step - Capture step context

# This hook executes before each test step
# It demonstrates capturing step context

set -e

# Log step context
echo "step_number=${TEST_STEP_NUMBER}" >> "/tmp/step_context_$$.log"
echo "step_description=${TEST_STEP_DESCRIPTION}" >> "/tmp/step_context_$$.log"

exit 0
