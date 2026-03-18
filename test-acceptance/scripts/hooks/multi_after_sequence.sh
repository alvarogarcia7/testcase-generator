#!/usr/bin/env bash

# Multi-hook after_sequence (on_error: continue)
SEQUENCE_ID="${TEST_SEQUENCE_ID:-unknown}"
SEQUENCE_NAME="${TEST_SEQUENCE_NAME:-unknown}"
echo "after_sequence executed: Sequence $SEQUENCE_ID: $SEQUENCE_NAME" >> /tmp/multi_hooks_$$.log
