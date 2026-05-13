---
id: TCMS-36
title: Resolve test case dependencies
status: To Do
assignee: []
created_date: '2026-05-13 15:55'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
When a test case has dependencies to another test case, whether references, dependencies, copied steps:

In a phase before generating the scripts, the dependencies must be solved. This creates a temporary test case that has solved the dependencies.

From that temporary test case, the script is generated.
<!-- SECTION:DESCRIPTION:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 All tests are passing. Run make test
<!-- DOD:END -->
