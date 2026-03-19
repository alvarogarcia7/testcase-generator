#!/usr/bin/env bash
# Hook: before_sequence - Capture sequence context

# This hook executes before each test sequence
# It demonstrates capturing sequence context

set -e

# Log sequence context
echo "sequence_id=${TEST_SEQUENCE_ID}" >> "/tmp/sequence_context_$$.log"
echo "sequence_name=${TEST_SEQUENCE_NAME}" >> "/tmp/sequence_context_$$.log"

exit 0
