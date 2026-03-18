#!/usr/bin/env bash
set -e

# Script start hook for script_end test - initializes tracking
START_TIME=$(date +%s)
echo "$START_TIME" > /tmp/script_end_start_time_$$.txt
touch /tmp/script_end_tracking_$$.log
