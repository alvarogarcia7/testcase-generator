---
id: TCMS-7
title: Validate the generated shell scripts with shellcheck
status: To Do
assignee: []
created_date: '2026-02-27 15:46'
updated_date: '2026-02-27 15:59'
labels: []
dependencies:
  - TCMS-6
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Context: the YAML files are manually written by a human. They make errors in defining variables, etc.

Validate the generated shell scripts with shellcheck

## General
1. Only complain about errors (not warnings). Reduce to noise (reducing false positives) to redirect to fix real issues.

## Validate the shell file in the tests
1. Install the shellcheck tool (https://www.shellcheck.net/) into the dockerfile
2. After generating a valid script (YAML->sh), validate that the generated shell file is valid. As an example, in `tests/integration/test_executor_e2e.sh`

```
# Validate shell script syntax
if bash -n "$PASSING_SCRIPT" 2>/dev/null; then
    pass "Passing script has valid bash syntax"
else
    fail "Passing script has invalid bash syntax"
fi
```

After bash -n, use shellcheck.

## Validate the shell file in the generation
1. In the test-executor.rs application, Generate subcommand.
1. Use shellcheck to verify the shellfile output 
2. If the validation fails, allow the argument (-f) to skip the validation and force producing the shell output.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Execute shellcheck inside the dockerfile container to verify it works
- [ ] #2 Create a make goal to verify a shell script.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 All tests are passing. Run make test
<!-- DOD:END -->
