---
id: TCMS-10
title: >-
  Gitlab pipeline: Verify the image can be run offline + do not depend on latest
  for tagging
status: Done
assignee: []
created_date: '2026-03-11 14:34'
updated_date: '2026-03-12 07:56'
labels: []
dependencies: []
ordinal: 3000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
* Verify the image can be run offline: The image can work offline (no issues with crates.io)
* Do not depend on latest for tagging
* You can build on top of this image offline as well.
* Contents of the docker image:
  * compile tests
  * run tests
<!-- SECTION:DESCRIPTION:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 All tests are passing. Run make test
<!-- DOD:END -->
