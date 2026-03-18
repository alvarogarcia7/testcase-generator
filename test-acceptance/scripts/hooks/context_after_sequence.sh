#!/usr/bin/env bash

# After sequence hook - logs sequence context (on_error: continue)
SEQUENCE_ID="${TEST_SEQUENCE_ID:-unknown}"
SEQUENCE_NAME="${TEST_SEQUENCE_NAME:-unknown}"

echo "after_sequence: TEST_SEQUENCE_ID=$SEQUENCE_ID, TEST_SEQUENCE_NAME=$SEQUENCE_NAME" >> /tmp/sequence_context_$$.log
