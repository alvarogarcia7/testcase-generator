#!/usr/bin/env bash

# Hook that always fails but allows continuation (on_error: continue)
echo "Hook failing but test continues" >&2
exit 1
