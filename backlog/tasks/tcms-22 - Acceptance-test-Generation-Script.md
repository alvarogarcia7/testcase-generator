---
id: TCMS-19
title: >-
  Audit Log + Cryptographic Validation: SHA-256 Hash Chain for Audit
  Traceability
status: In Progress
assignee: []
created_date: '2026-03-27 09:41'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement a SHA-256 hash chain that computes a digest of each input test case YAML file and propagates it through every pipeline artifact — embedded in generated bash scripts, carried into execution log JSON entries, and verified during the verification stage — so an auditor can cryptographically prove that script generation, execution, and verification all derive from the same well-known input.

## Key Decisions

Use SHA-256 as the hash algorithm (via the sha2 crate) computing the digest over the raw YAML file bytes rather than the parsed/re-serialized model, ensuring the hash matches what the auditor can independently verify with standard tools like shasum -a 256.

Embed the source YAML hash as a source_yaml_sha256 field in the JSON execution log entries written by the generated bash script (computed at script generation time and baked into the script as a constant), rather than computing it at runtime inside the bash script, to guarantee the hash ties to the exact input the code generator consumed.

Add a --verify-source-hash flag to the verifier binary that re-computes the SHA-256 of the test case YAML on disk and compares it against the hash carried in the execution log, failing verification with a clear audit error if they diverge.

## Tasks

Add sha2 to workspace dependencies in root Cargo.toml and to testcase-execution's Cargo.toml. In crates/testcase-execution/src/executor.rs, add a compute_yaml_sha256(content: &[u8]) -> String helper that returns the hex-encoded SHA-256 digest. Modify generate_test_script_with_json_output to accept an Option<String> source hash parameter, and when present: (1) emit a # Source YAML SHA-256: <hash> comment near the test case header, (2) include a "source_yaml_sha256": "<hash>" field in each JSON log entry written to $JSON_LOG. Update generate_test_script to compute the hash from the raw YAML bytes and pass it through. Update test-executor/src/main.rs Commands::Generate to read raw file bytes, compute the hash, and pass it to the executor. Add unit tests verifying the hash appears in generated script output and JSON log entries.

Add sha2 to testcase-verification's Cargo.toml. Extend TestExecutionLog in crates/testcase-verification/src/verification.rs with an Option<String> field source_yaml_sha256. Add a --verify-source-hash flag to the Cli struct in crates/verifier/src/main.rs. When enabled, after loading each test case YAML file, compute its SHA-256 from the raw file bytes and compare against the source_yaml_sha256 field from the parsed execution log entries; if any entry's hash is missing or mismatched, emit a clear error identifying the test case and fail verification. Add the per-test-case source hash into TestCaseVerificationResult and propagate it into ContainerReportMetadata as an optional source_hashes: HashMap<String, String> (test_case_id → sha256) so the final YAML/JSON report includes the hashes for auditor consumption.

Create a new binary crate crates/audit-verifier with a main.rs that accepts CLI arguments for a test case YAML file path and its corresponding execution log JSON path, computes the SHA-256 hash of the raw YAML file bytes using the sha2 crate, parses the execution log to extract all source_yaml_sha256 fields from each JSON entry, compares the computed hash against every extracted hash from the log entries, and prints all mismatches as errors along with warnings for missing hash fields while exiting with status 0 only if all hashes match and no entries lack the hash field. Add the crate to the workspace members list in the root Cargo.toml with dependencies on sha2, serde, serde_json, anyhow, and clap. Include a Makefile target audit-verify that builds and runs this binary against sample test case and execution log files from the test-acceptance directory.

In the audit-verifier crate, add the p256 or ring cryptographic library as a dependency supporting NIST P-521 (secp521r1) elliptic curve operations. Implement functionality to generate or load a P-521 private key, compute an ECDSA signature over the SHA-256 hash of the audit log content (either the entire execution log JSON or a canonical representation of the hash verification results), and append the signature along with the corresponding public key or key identifier to the audit verification output so that external auditors can cryptographically verify the audit trail was signed by the expected authority.

In the audit-verifier crate, create a new binary to verify an audit log, given the keypair, the payload, the signature file,

Create an e2e test for executing a) key generation with a custom string in the description field, b) generate scripts from yaml, c) execute scripts, d) verify test cases, e) verify audit log. Inspect run_tpdg_conversion.sh
<!-- SECTION:DESCRIPTION:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 All tests are passing. Run make test
<!-- DOD:END -->
