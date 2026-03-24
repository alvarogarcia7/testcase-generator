#!/bin/bash

set -euo pipefail

subcommand="$1"

if [[ "$subcommand" = "r" ]]; then
  empty_branch=0
  # Remove
  #
  branch="${2:-""}"
  set +e
  if [[ "$branch" = "" ]]; then
    empty_branch=1
    current_branch="$(git branch -l | grep "*" | awk '{print $2}')"
    branch="${current_branch}"
  fi
  set -e

  set +e
  worktree_path=$(git worktree list --porcelain|grep -E "branch.*${branch}$" -B2 | head -1 | cut -d" " -f2-)
  status_code=$?
  set -e
  
  if [[ $status_code -ne 0 ]]; then
    echo "Worktree not found for branch '${branch}'"
    false
  fi
  
  if [[ $empty_branch -eq 1 ]]; then  
    git checkout main > /dev/null 2>&1 || git checkout master > /dev/null 2>&1 || git checkout $(git branch | grep -vE "\*|\+" | head -1 | awk '{print $1}')  > /dev/null 2>&1
  fi

  git worktree remove "${worktree_path}"
  
fi
