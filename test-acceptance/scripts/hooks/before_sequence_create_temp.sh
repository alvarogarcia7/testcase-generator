#!/usr/bin/env bash
set -e

# Before sequence hook - creates temporary files
SEQUENCE_ID="${TEST_SEQUENCE_ID:-unknown}"
echo "Creating temp for sequence $SEQUENCE_ID" >> /tmp/before_sequence_temp_$$.log
touch "/tmp/sequence_${SEQUENCE_ID}_$$.tmp"
