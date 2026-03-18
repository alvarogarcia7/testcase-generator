#!/usr/bin/env bash
# Hook: before_step - Log before each step

# This hook executes before each test step
# It has access to TEST_STEP_NUMBER and TEST_STEP_DESCRIPTION

set -e

# Create marker file for verification
echo "before_step ${TEST_STEP_NUMBER}" > "/tmp/before_step_${TEST_STEP_NUMBER}_$$.txt"

exit 0
