#!/usr/bin/env bash

# Teardown test hook - performs final test cleanup
# This hook executes once after all test sequences, before script_end
# Note: Does not use set -e to ensure cleanup continues even on errors

TEARDOWN_TIME=$(date +%s)
echo "teardown_test: Test teardown started at $(date)" >> /tmp/teardown_test_$$.log

# Cleanup test workspace if it exists
if [ -d /tmp/test_workspace_$$ ]; then
    rm -rf /tmp/test_workspace_$$
    echo "teardown_test: Removed /tmp/test_workspace_$$" >> /tmp/teardown_test_$$.log
fi

echo "teardown_test: Teardown complete" >> /tmp/teardown_test_$$.log
