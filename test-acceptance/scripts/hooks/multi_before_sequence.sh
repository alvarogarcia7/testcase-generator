#!/usr/bin/env bash
set -e

# Multi-hook before_sequence
SEQUENCE_ID="${TEST_SEQUENCE_ID:-unknown}"
SEQUENCE_NAME="${TEST_SEQUENCE_NAME:-unknown}"
echo "before_sequence executed: Sequence $SEQUENCE_ID: $SEQUENCE_NAME" >> /tmp/multi_hooks_$$.log
