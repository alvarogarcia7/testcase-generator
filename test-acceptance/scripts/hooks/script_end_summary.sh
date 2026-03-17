#!/usr/bin/env bash

# Script end hook - generates execution summary (on_error: continue)
if [ -f /tmp/script_end_start_time_$$.txt ]; then
    START_TIME=$(cat /tmp/script_end_start_time_$$.txt)
    END_TIME=$(date +%s)
    DURATION=$((END_TIME - START_TIME))
    
    echo "Test execution summary:" > /tmp/script_end_summary_$$.txt
    echo "Duration: ${DURATION}s" >> /tmp/script_end_summary_$$.txt
    
    if [ -f /tmp/script_end_tracking_$$.log ]; then
        STEP_COUNT=$(wc -l < /tmp/script_end_tracking_$$.log | tr -d ' ')
        echo "Steps tracked: $STEP_COUNT" >> /tmp/script_end_summary_$$.txt
    fi
    
    # Cleanup
    rm -f /tmp/script_end_start_time_$$.txt
    rm -f /tmp/script_end_tracking_$$.log
fi
