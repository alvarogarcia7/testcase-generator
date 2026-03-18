#!/usr/bin/env bash
set -e

# Multi-hook before_step
STEP_NUM="${TEST_STEP_NUMBER:-unknown}"
echo "before_step executed: Step $STEP_NUM" >> /tmp/multi_hooks_$$.log
