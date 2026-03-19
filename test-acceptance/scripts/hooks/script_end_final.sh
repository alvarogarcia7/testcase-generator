#!/usr/bin/env bash
# Hook: script_end - Final cleanup and logging

# This hook executes once at the very end of the test script
# It can access all test execution context and perform final cleanup

set -e

# Final logging
echo "Script end hook executed" >&2

# Cleanup any remaining test artifacts
rm -f /tmp/script_end_marker_$$.txt 2>/dev/null || true

exit 0
