---
id: TCMS-9
title: 'Hooks: execution even if scripts fail'
status: To Do
assignee: []
created_date: '2026-03-11 12:59'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The hooks must be executed even if the execution steps fail.

The generated script should perform any cleaning up (see `trap`) and the hooks (if defined / if any)

Review B56E33C6-20D8-447D-B170-17AD3442F5A4:

```grep
docs/HOOKS.md
32:[B56E33C6-20D8-447D-B170-17AD3442F5A4]

testcases/verifier_scenarios/hooks/TEST_HOOK_TEARDOWN_001_AFTER_STEP_FAILURE.yml
5:description: 'B56E33C6-20D8-447D-B170-17AD3442F5A4 Hook error scenario for teardown_test hook - script exits with error code during test cleanup'
```
<!-- SECTION:DESCRIPTION:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 All tests are passing. Run make test
- [ ] #2 Open a PR to github, babysit the success there
<!-- DOD:END -->
