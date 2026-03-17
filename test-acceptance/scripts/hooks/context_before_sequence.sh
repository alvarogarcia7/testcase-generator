#!/usr/bin/env bash
set -e

# Before sequence hook - logs sequence context
SEQUENCE_ID="${TEST_SEQUENCE_ID:-unknown}"
SEQUENCE_NAME="${TEST_SEQUENCE_NAME:-unknown}"

echo "before_sequence: TEST_SEQUENCE_ID=$SEQUENCE_ID, TEST_SEQUENCE_NAME=$SEQUENCE_NAME" >> /tmp/sequence_context_$$.log
