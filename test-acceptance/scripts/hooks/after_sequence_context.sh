#!/usr/bin/env bash
# Hook: after_sequence - Log sequence completion context

# This hook executes after each test sequence
# It demonstrates capturing sequence completion

set -e

# Log sequence completion
echo "sequence_${TEST_SEQUENCE_ID}_completed" >> "/tmp/sequence_context_$$.log"

exit 0
