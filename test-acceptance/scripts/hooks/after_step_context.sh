#!/usr/bin/env bash
# Hook: after_step - Log step completion context

# This hook executes after each test step
# It has access to STEP_EXIT_CODE and COMMAND_OUTPUT

set -e

# Log step completion
echo "step_${TEST_STEP_NUMBER}_exit_code=${STEP_EXIT_CODE}" >> "/tmp/step_context_$$.log"

exit 0
