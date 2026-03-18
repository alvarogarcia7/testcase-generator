---
id: TCMS-12
title: Acceptance Testing
status: To Do
assignee: []
created_date: '2026-02-28'
labels:
  - testing
  - pipeline
  - ci-cd
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
This document describes the 7 stages in the GitLab CI/CD pipeline for the test harness project. The pipeline ensures comprehensive validation of the codebase through multiple phases, from Docker image creation to comprehensive testing and security validation.

### Pipeline Stages Overview

#### Stage 1: Docker Build (`docker:build`)
**Purpose**: Build and prepare the Docker container image with all dependencies.

**Key Activities**:
- Build Docker image with Python and Rust toolchain
- Tag image with commit SHA for traceability
- Verify cargo works in offline mode (no network dependencies)
- Push image to container registry for downstream jobs
- Generate build artifacts including docker inspect output

**Validation**:
- Docker image builds successfully
- All required binaries are present (`editor --help`)
- Offline cargo build succeeds (`cargo --offline build`)

**Output**: Docker image tagged with commit SHA and pushed to registry

---

#### Stage 2: Docker Test (`docker:test`)
**Purpose**: Validate the Docker image functionality in isolation.

**Key Activities**:
- Pull the built Docker image from registry
- Verify system environment (`uname -a`)
- Test editor binary functionality
- Run in network-isolated mode (`--net none`)

**Validation**:
- Image can be pulled successfully
- Editor binary executes correctly
- All required tools are accessible

**Dependencies**: Requires successful completion of `docker:build`

---

#### Stage 3: Docker Push (`docker:push`)
**Purpose**: Promote validated Docker image with stable tags.

**Key Activities**:
- Pull commit-SHA tagged image from registry
- Tag image with `latest` for stable release
- Push latest tag to container registry
- Verify image exists before tagging

**Validation**:
- Image can be pulled by SHA
- Latest tag is created and pushed
- Registry authentication succeeds

**Dependencies**: Requires successful completion of `docker:build` and `docker:test`

---

#### Stage 4: Rust Build, Test, and Lint (`rust:build-test-lint`)
**Purpose**: Comprehensive Rust codebase validation including compilation, testing, and code quality checks.

**Key Activities**:
- **Build Phase**:
  - Compile all binaries with release optimizations
  - Use sccache for compilation caching
  - Extract built binaries to artifacts directory
  
- **Lint Phase**:
  - Run `rustfmt` for code formatting validation
  - Execute `clippy` with strict warnings-as-errors mode
  - Generate clippy output report
  
- **Test Phase**:
  - Run unit tests with all features enabled
  - Execute end-to-end (e2e) tests
  - Run documentation tests
  - Parse test results into structured JSON format
  
- **Coverage Phase**:
  - Generate code coverage using grcov
  - Produce lcov and HTML coverage reports
  - Compare coverage for merge requests
  - Post coverage comments on merge requests

**Validation**:
- All compilation succeeds without warnings
- rustfmt passes (code formatting correct)
- clippy passes with zero warnings
- All unit tests pass
- E2E tests execute successfully
- Coverage meets thresholds

**Output Artifacts**:
- Release binaries (validate-yaml, validate-json, trm, test-verify, test-executor, test-orchestrator)
- Test results (test_output.txt, test_results.json, test_results.xml)
- Coverage reports (lcov.info, coverage/html, cobertura.xml)
- Lint reports (clippy_output.txt)
- E2E test output (e2e_output.txt)

---

#### Stage 5: Docker Security Scan (`docker:verify`)
**Purpose**: Perform security analysis on the built Docker image.

**Key Activities**:
- Analyze docker inspect output
- Run security scanning script
- Identify potential security issues
- Generate security scan results

**Validation**:
- Security scan completes without critical issues
- Docker configuration follows best practices
- No known vulnerabilities detected

**Dependencies**: Requires successful completion of `docker:build`

**Output**: docker_scan_results.json with security findings

---

#### Stage 6: Performance Baseline (Commented - Future Implementation)
**Purpose**: Establish performance benchmarks for the main branch.

**Planned Activities**:
- Run performance benchmarking suite
- Measure execution time and resource usage
- Store baseline metrics for comparison
- Execute on main/master branch only

**Output**: baseline_perf.json with performance metrics

---

#### Stage 7: Performance Comparison & Reporting (Commented - Future Implementation)
**Purpose**: Compare performance against baseline and generate comprehensive pipeline summary.

**Planned Activities**:
- **Performance Compare**:
  - Run current performance benchmarks
  - Compare against baseline from main branch
  - Identify performance regressions
  - Post performance comparison to merge requests
  
- **Report Summary**:
  - Aggregate results from all pipeline stages
  - Generate comprehensive pipeline summary
  - Post summary comment to merge requests
  - Include test, coverage, security, and performance results

**Output**: 
- perf_comparison.json
- pipeline_summary.md

---

### Pipeline Features

**Caching Strategy**:
- Cargo home and build artifacts cached per branch
- sccache for Rust compilation caching (5GB limit)
- Cache policy: pull-push for all jobs

**Artifact Management**:
- Test results preserved for 1 week
- Coverage reports available as artifacts
- JUnit XML reports for test visualization
- Cobertura XML for coverage visualization

**Merge Request Integration**:
- Automated comments for test results
- Automated comments for clippy warnings
- Coverage comparison and reporting
- Performance regression detection

**Error Handling**:
- Strict mode: All warnings treated as errors
- Comprehensive error reporting
- Artifacts preserved on failure (`when: always`)

### Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 All pipeline stages execute successfully
- [ ] #2 Test coverage meets minimum thresholds
- [ ] #3 No clippy warnings or formatting issues
- [ ] #4 Docker image builds and passes security scan
- [ ] #5 All acceptance tests pass
<!-- DOD:END -->

## Acceptance Testing Integration

The acceptance testing framework (described in `test-acceptance/README.md`) validates the complete end-to-end workflow:

1. **Test Case Definition**: YAML test case files in categorized directories
2. **Script Generation**: Convert YAML to executable bash scripts
3. **Test Execution**: Run generated scripts and capture execution logs
4. **Verification**: Convert execution logs to container YAML format
5. **Documentation Generation**: Generate AsciiDoc, Markdown, and HTML reports
6. **Validation**: Schema validation and container compatibility checks
7. **CI/CD Integration**: Automated execution in pipeline

This multi-stage acceptance testing ensures that all features (variables, hooks, prerequisites, conditionals, etc.) work correctly in realistic scenarios before deployment.

<!-- SECTION:DESCRIPTION:END -->
