#!/usr/bin/env bash

# After sequence hook - cleans up temporary files (on_error: continue)
SEQUENCE_ID="${TEST_SEQUENCE_ID:-unknown}"

# Clean up sequence-specific temp files
rm -f "/tmp/sequence_${SEQUENCE_ID}_$$.tmp"
rm -f "/tmp/after_sequence_marker_${SEQUENCE_ID}_$$.txt"

echo "Cleaned up sequence $SEQUENCE_ID" >> /tmp/after_sequence_cleanup_$$.log
