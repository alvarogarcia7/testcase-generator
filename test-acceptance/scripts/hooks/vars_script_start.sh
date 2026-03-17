#!/usr/bin/env bash
set -e

# Script start hook - logs environment variables
echo "Environment variables at script_start:" > /tmp/hook_vars_$$.log
echo "TEST_ENV=${TEST_ENV:-not_set}" >> /tmp/hook_vars_$$.log
echo "CUSTOM_VAR=${CUSTOM_VAR:-not_set}" >> /tmp/hook_vars_$$.log
