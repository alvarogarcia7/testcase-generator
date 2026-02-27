---
id: TCMS-6
title: 'Gitlab pipeline: refactor'
status: To Do
assignee: []
created_date: '2026-02-26 18:50'
updated_date: '2026-02-27 15:19'
labels: []
dependencies:
  - TCMS-5
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Speed up pipeline
1. Compile in the stage rust:build-test-lint
2. Then reuse as much as possible to build the docker image
3. Test it, then push.
 

## Do not trigger pipeline for changes in the `backlog` folder
<!-- SECTION:DESCRIPTION:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 All tests are passing. Run make test
<!-- DOD:END -->
