#!/usr/bin/env bash
set -e

# Multi-hook before_step
STEP_NUMBER="${TEST_STEP_NUMBER:-unknown}"
echo "before_step executed: Step $STEP_NUMBER" >> /tmp/multi_hooks_$$.log
