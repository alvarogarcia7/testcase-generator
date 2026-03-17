#!/usr/bin/env bash
set -e

# Before sequence hook - logs sequence start with context
SEQUENCE_ID="${TEST_SEQUENCE_ID:-unknown}"
SEQUENCE_NAME="${TEST_SEQUENCE_NAME:-unknown}"

echo "before_sequence: Sequence $SEQUENCE_ID: $SEQUENCE_NAME" >> /tmp/before_sequence_$$.log
