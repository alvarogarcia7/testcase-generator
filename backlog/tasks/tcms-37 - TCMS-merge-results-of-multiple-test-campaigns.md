---
id: TCMS-37
title: TCMS merge results of multiple test campaigns
status: To Do
assignee: []
created_date: '2026-05-13 16:02'
updated_date: '2026-05-14 12:14'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Currently, TCMS only validates only one test campaign.

Modify TCMS so that it is possible to execute multiple campaigns, verifying multiple campaigns, etc. Each campaign is one folder.

Create a python script to merge the results of those campaigns:
- --merge-strategy=or -> failure OR success = success
- --merge-strategy=and -> failure AND success = failure
- --merge-strategy=oldest -> the result is the result of the oldest execution (by timestamp)
- --merge-strategy=newest -> the result is the result of the newest execution (by timestamp)
<!-- SECTION:DESCRIPTION:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 All tests are passing. Run make test
<!-- DOD:END -->
