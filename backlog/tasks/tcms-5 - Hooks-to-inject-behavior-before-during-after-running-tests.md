---
id: TCMS-5
title: Hooks to inject behavior before/during/after running tests
status: Done
assignee: []
created_date: '2026-02-26 18:41'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create eight types of hooks:
1. **`script_start`** - Executes once at the very beginning of test script execution
2. **`setup_test`** - Executes once after prerequisites, before any test sequences
3. **`before_sequence`** - Executes before each test sequence starts
4. **`after_sequence`** - Executes after each test sequence completes
5. **`before_step`** - Executes before each test step runs
6. **`after_step`** - Executes after each test step completes
7. **`teardown_test`** - Executes once after all test sequences complete
8. **`script_end`** - Executes once at the very end of test script execution
<!-- SECTION:DESCRIPTION:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 All tests are passing. Run make test
<!-- DOD:END -->
