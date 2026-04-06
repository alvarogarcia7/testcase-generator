---
id: TCMS-28
title: Mermaid diagrams for interaction and data flow
status: In Progress
assignee: []
created_date: '2026-04-01 09:10'
updated_date: '2026-04-02 10:29'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
# Create comprehensive crate data flow diagram

Create a new mermaid diagram documenting data flow between all crates in the workspace, including layer architecture, data models flowing between crates (TestCase, TestExecutionLog, VerificationReport), and concrete examples of end-to-end workflows (YAML → Script Generation → Execution → Verification → Reports). Add comprehensive markdown documentation explaining the collaboration patterns, data transformations, and layer responsibilities.

## Tasks

Create docs/data_flow/crate_data_flow.mermaid with a comprehensive flowchart diagram showing: (1) five architectural layers as subgraphs (Base, Foundation, Core, Orchestration, Binary), (2) all crates positioned in their respective layers, (3) data flow arrows between crates annotated with data types like TestCase, TestExecutionLog, VerificationReport, (4) special nodes for external artifacts (YAML files, bash scripts, JSON logs, XML reports, audit logs), (5) styling to differentiate layer types (base=blue, foundation=green, core=yellow, orchestration=orange, binary=red). Use the existing 2026-01-20_diagram_steps.mermaid as a reference for formatting style but create a crate-focused architecture diagram rather than a user workflow diagram.

Add docs/data_flow/workflow_examples.mermaid with three sequence diagrams showing end-to-end data flow for major use cases: (1) Test Creation Flow - editor → testcase-ui → testcase-storage → YAML file → testcase-validation, with TestCase objects flowing through; (2) Test Execution Flow - test-executor reads YAML → testcase-storage loads TestCase → testcase-execution generates bash script → bash executes → TestExecutionLog JSON artifact created → testcase-verification compares against expectations → VerificationReport; (3) Audit & Reporting Flow - audit-verifier signs logs → verifier generates XML reports → audit-traceability tracks all operations. Annotate each arrow with the specific data structure being passed.

Create docs/data_flow/DATA_FLOW.md comprehensive documentation with sections: (1) Overview explaining the five-layer architecture and data flow principles, (2) Core Data Models table listing TestCase, TestSequence, TestStep, Verification, TestExecutionLog, StepVerificationResult, ContainerReport with their source crate and key fields, (3) Layer-by-Layer Data Flow describing responsibilities and data transformations at each layer with examples, (4) Common Workflows with subsections for each use case showing step-by-step data flow with code snippets of key types, (5) Data Transformation Pipeline showing how YAML → TestCase → bash script → JSON log → VerificationReport → XML report with intermediate steps, (6) Cross-Cutting Concerns covering audit logging, configuration, and error handling data flows. Include mermaid diagram references.

Update AGENTS.md to add a "Data Flow and Architecture" section after the "Workspace Structure" section, with: (1) link to docs/data_flow/DATA_FLOW.md and the mermaid diagrams, (2) brief explanation of the five-layer architecture (Base → Foundation → Core → Orchestration → Binary), (3) table showing common data types and which crates produce/consume them, (4) references to crates/README.md for detailed architecture documentation. This provides quick navigation for agents to understand data flow patterns.
<!-- SECTION:DESCRIPTION:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 All tests are passing. Run make test
<!-- DOD:END -->
