#!/usr/bin/env bash
# Hook: before_sequence - Initialization before each sequence

# This hook executes before each test sequence
# It has access to TEST_SEQUENCE_ID and TEST_SEQUENCE_NAME

set -e

# Create marker file for verification
echo "before_sequence ${TEST_SEQUENCE_ID}" > "/tmp/before_sequence_${TEST_SEQUENCE_ID}_$$.txt"

exit 0
