# Test Campaign Management

## Overview

The Test Campaign Management system provides a structured approach to organizing, executing, and tracking test execution campaigns. A campaign is a collection of test runs with comprehensive evidence tracking, metadata management, and reporting capabilities.

## Campaign Lifecycle

A typical campaign follows this lifecycle:

1. **START** - Initialize campaign structure and metadata
2. **RUN** - Execute tests (can be run multiple times)
3. **COLLECT** - Gather evidence and artifacts
4. **STOP** - Finalize campaign and generate reports

## Scripts

### 1. campaign-start.sh

Creates a new test campaign with directory structure and metadata tracking.

**Usage:**
```bash
./scripts/campaign-start.sh --name "Campaign_Name" [OPTIONS]
```

**Options:**
- `--name NAME` - Campaign name (required)
- `--description DESC` - Campaign description (optional)
- `--output-dir DIR` - Custom output directory (default: campaigns/<name>)
- `--testcase-dir DIR` - Test case directory (default: testcases)
- `--verbose` - Enable verbose output
- `--help` - Show help message

**Example:**
```bash
# Start a new campaign
./scripts/campaign-start.sh --name "Sprint_23_Regression" \
    --description "Full regression suite for Sprint 23"

# Start campaign with custom location
./scripts/campaign-start.sh --name "Release_v2.0" \
    --output-dir /tmp/campaigns/release_v2
```

**Directory Structure Created:**
```
campaigns/<name>/
├── testcases/              # Test case YAML files (organized by run)
├── execution_logs/         # JSON execution logs (organized by run)
├── verification_results/   # Verification results (organized by run)
├── evidence/               # Additional evidence files
├── reports/                # Generated reports
└── metadata/               # Campaign and run metadata
    ├── campaign.yaml       # Campaign metadata
    ├── state.txt           # Campaign state (ACTIVE/COMPLETED)
    ├── run_counter.txt     # Run counter
    └── run_*.yaml          # Individual run metadata
```

---

### 2. campaign-run.sh

Executes test cases for an active campaign with optional regex filtering.

**Usage:**
```bash
./scripts/campaign-run.sh --campaign <dir> [OPTIONS]
```

**Options:**
- `--campaign DIR` - Campaign directory (required)
- `--pattern REGEX` - Test case filename regex pattern (default: .*)
- `--testcase-dir DIR` - Override test case directory
- `--parallel N` - Run N tests in parallel (default: 1)
- `--continue-on-error` - Continue even if some tests fail
- `--skip-verification` - Skip verification phase
- `--verbose` - Enable verbose output
- `--help` - Show help message

**Examples:**
```bash
# Run all tests in campaign
./scripts/campaign-run.sh --campaign campaigns/Sprint_23

# Run only tests matching a pattern
./scripts/campaign-run.sh --campaign campaigns/Sprint_23 \
    --pattern "EXAMPLE_.*\.yml"

# Run with specific test directory
./scripts/campaign-run.sh --campaign campaigns/Sprint_23 \
    --testcase-dir testcases/bdd_examples

# Run tests in parallel
./scripts/campaign-run.sh --campaign campaigns/Sprint_23 \
    --parallel 4 --continue-on-error

# Run without verification (faster for debugging)
./scripts/campaign-run.sh --campaign campaigns/Sprint_23 \
    --skip-verification
```

**Run Organization:**

Each run is assigned a unique ID: `run_<number>_<timestamp>`

Example:
- `run_1_20240115_143022` - First run
- `run_2_20240115_150130` - Second run

Run-specific artifacts are organized by run ID:
```
campaigns/<name>/
├── testcases/
│   ├── run_1_20240115_143022/
│   │   ├── test_case_1.yml
│   │   └── test_case_2.yml
│   └── run_2_20240115_150130/
│       └── test_case_3.yml
├── execution_logs/
│   ├── run_1_20240115_143022/
│   │   ├── test_case_1_execution_log.json
│   │   └── test_case_2_execution_log.json
│   └── run_2_20240115_150130/
│       └── test_case_3_execution_log.json
└── verification_results/
    ├── run_1_20240115_143022_verification.json
    ├── run_1_20240115_143022_verification.yaml
    ├── run_2_20240115_150130_verification.json
    └── run_2_20240115_150130_verification.yaml
```

**Pattern Matching:**

The `--pattern` option uses regex to filter test case filenames:

```bash
# Run all test cases (default)
--pattern ".*"

# Run only EXAMPLE test cases
--pattern "EXAMPLE.*"

# Run specific test case by ID
--pattern "SELF_VALIDATED_EXAMPLE_001\.yml"

# Run test cases starting with 'gsma'
--pattern "gsma.*"

# Run all test cases ending with specific pattern
--pattern ".*_TC\.ya?ml"
```

---

### 3. campaign-collect-evidence.sh

Collects all campaign artifacts into a compressed archive suitable for audit or compliance.

**Usage:**
```bash
./scripts/campaign-collect-evidence.sh --campaign <dir> [OPTIONS]
```

**Options:**
- `--campaign DIR` - Campaign directory (required)
- `--output FILE` - Output archive file (default: <campaign>_evidence.tar.gz)
- `--format FORMAT` - Archive format: tar.gz, tar.bz2, zip (default: tar.gz)
- `--include-binaries` - Include binary artifacts
- `--checksums` - Generate SHA256 checksums for all files
- `--verbose` - Enable verbose output
- `--help` - Show help message

**Examples:**
```bash
# Collect all evidence with default settings
./scripts/campaign-collect-evidence.sh --campaign campaigns/Sprint_23

# Create ZIP archive with checksums
./scripts/campaign-collect-evidence.sh --campaign campaigns/Sprint_23 \
    --format zip --checksums

# Custom output location
./scripts/campaign-collect-evidence.sh --campaign campaigns/Sprint_23 \
    --output /archive/evidence_2024_Q1.tar.gz \
    --checksums
```

**Generated Files:**
- Archive file (tar.gz, tar.bz2, or zip)
- SHA256 checksum file (if `--checksums` is used)
- Evidence collection report (text file)

**Archive Contents:**
- All test case files executed in the campaign
- All execution logs (JSON format)
- All verification results (JSON/YAML format)
- Campaign metadata and run information
- Generated reports
- Additional evidence files
- MANIFEST.txt - Directory structure and file counts
- SHA256SUMS.txt - Checksums for all files (if enabled)

---

### 4. campaign-stop.sh

Finalizes a campaign, calculates statistics, and generates summary reports.

**Usage:**
```bash
./scripts/campaign-stop.sh --campaign <dir> [OPTIONS]
```

**Options:**
- `--campaign DIR` - Campaign directory (required)
- `--summary TEXT` - Final summary text (optional)
- `--generate-reports` - Generate final reports
- `--collect-evidence` - Automatically collect evidence
- `--verbose` - Enable verbose output
- `--help` - Show help message

**Examples:**
```bash
# Stop campaign with default settings
./scripts/campaign-stop.sh --campaign campaigns/Sprint_23

# Stop with custom summary and evidence collection
./scripts/campaign-stop.sh --campaign campaigns/Sprint_23 \
    --summary "All regression tests passed successfully" \
    --collect-evidence

# Stop and generate reports
./scripts/campaign-stop.sh --campaign campaigns/Sprint_23 \
    --generate-reports
```

**Actions Performed:**
1. Validates campaign is active
2. Calculates aggregate statistics from all runs
3. Updates campaign metadata with final statistics
4. Changes campaign state to COMPLETED
5. Generates final summary report (Markdown)
6. Optionally collects evidence
7. Optionally generates additional reports

**Generated Report:**
- `reports/CAMPAIGN_SUMMARY.md` - Comprehensive campaign summary with:
  - Campaign overview and timeline
  - Aggregate statistics (pass rate, total tests, etc.)
  - Individual run summaries
  - Evidence and artifact locations

---

## Complete Workflow Example

### Scenario: Sprint 23 Regression Testing

```bash
# 1. Start the campaign
./scripts/campaign-start.sh \
    --name "Sprint_23_Regression" \
    --description "Full regression suite for Sprint 23 release"

# Campaign created at: campaigns/Sprint_23_Regression

# 2. First run - Execute all EXAMPLE tests
./scripts/campaign-run.sh \
    --campaign campaigns/Sprint_23_Regression \
    --pattern "EXAMPLE.*\.yml" \
    --verbose

# 3. Second run - Execute BDD examples
./scripts/campaign-run.sh \
    --campaign campaigns/Sprint_23_Regression \
    --testcase-dir testcases/bdd_examples \
    --continue-on-error

# 4. Third run - Execute all remaining tests
./scripts/campaign-run.sh \
    --campaign campaigns/Sprint_23_Regression \
    --pattern ".*" \
    --parallel 4

# 5. Collect evidence (can be done at any time)
./scripts/campaign-collect-evidence.sh \
    --campaign campaigns/Sprint_23_Regression \
    --checksums \
    --verbose

# 6. Stop the campaign and generate final reports
./scripts/campaign-stop.sh \
    --campaign campaigns/Sprint_23_Regression \
    --summary "Sprint 23 regression complete. 95% pass rate achieved." \
    --generate-reports \
    --collect-evidence
```

---

## Campaign Metadata

### campaign.yaml

Campaign metadata is stored in `metadata/campaign.yaml`:

```yaml
# Test Campaign Metadata
campaign:
  name: "Sprint_23_Regression"
  description: "Full regression suite for Sprint 23 release"
  start_time: "2024-01-15T14:30:00Z"
  stop_time: "2024-01-15T16:45:00Z"
  status: "completed"
  
configuration:
  testcase_dir: "/path/to/testcases"
  output_dir: "/path/to/campaigns/Sprint_23_Regression"
  
statistics:
  total_runs: 3
  total_tests_executed: 45
  total_tests_success: 43
  total_tests_failed: 2
  total_tests_error: 0
  pass_rate: 95.56
  
environment:
  hostname: "test-server-01"
  user: "testuser"
  platform: "Linux"
  architecture: "x86_64"

summary:
  text: "Sprint 23 regression complete. 95% pass rate achieved."
```

### run_*.yaml

Individual run metadata is stored in `metadata/run_<id>.yaml`:

```yaml
# Test Run Metadata
run:
  id: "run_1_20240115_143022"
  number: 1
  timestamp: "20240115_143022"
  start_time: "2024-01-15T14:30:22Z"
  end_time: "2024-01-15T14:45:18Z"
  
configuration:
  testcase_pattern: "EXAMPLE.*\\.yml"
  parallel_jobs: 1
  continue_on_error: false
  skip_verification: false
  
results:
  total_tests: 15
  execution_success: 14
  execution_failed: 1
  execution_error: 0
  
paths:
  testcases: "campaigns/Sprint_23_Regression/testcases/run_1_20240115_143022"
  execution_logs: "campaigns/Sprint_23_Regression/execution_logs/run_1_20240115_143022"
  verification_json: "campaigns/Sprint_23_Regression/verification_results/run_1_20240115_143022_verification.json"
  verification_yaml: "campaigns/Sprint_23_Regression/verification_results/run_1_20240115_143022_verification.yaml"
```

---

## Campaign State Management

### States

- **ACTIVE** - Campaign is active and can accept new test runs
- **COMPLETED** - Campaign has been stopped and is read-only

### State Transitions

```
START → ACTIVE
ACTIVE → RUN (multiple times) → ACTIVE
ACTIVE → STOP → COMPLETED
```

Once a campaign is COMPLETED:
- No more test runs can be added
- Campaign metadata is finalized
- Statistics are calculated and frozen
- Summary reports are generated

---

## Evidence Package Structure

When evidence is collected, a compressed archive is created with the following structure:

```
<campaign_name>_evidence/
├── MANIFEST.txt                    # Package manifest
├── SHA256SUMS.txt                  # Checksums (if enabled)
├── README.md                       # Campaign README
├── testcases/                      # Test case files
│   ├── run_1_*/
│   ├── run_2_*/
│   └── ...
├── execution_logs/                 # Execution logs
│   ├── run_1_*/
│   ├── run_2_*/
│   └── ...
├── verification_results/           # Verification results
│   ├── run_1_*_verification.json
│   ├── run_1_*_verification.yaml
│   └── ...
├── metadata/                       # Campaign metadata
│   ├── campaign.yaml
│   ├── run_1_*.yaml
│   ├── run_2_*.yaml
│   └── ...
├── reports/                        # Generated reports
│   └── CAMPAIGN_SUMMARY.md
└── evidence/                       # Additional evidence
```

---

## Integration with CI/CD

### GitLab CI Example

```yaml
test_campaign:
  stage: test
  script:
    # Start campaign
    - ./scripts/campaign-start.sh --name "CI_Build_${CI_PIPELINE_ID}"
    
    # Run tests
    - ./scripts/campaign-run.sh 
        --campaign "campaigns/CI_Build_${CI_PIPELINE_ID}"
        --pattern ".*"
        --continue-on-error
    
    # Collect evidence
    - ./scripts/campaign-collect-evidence.sh
        --campaign "campaigns/CI_Build_${CI_PIPELINE_ID}"
        --checksums
    
    # Stop campaign
    - ./scripts/campaign-stop.sh
        --campaign "campaigns/CI_Build_${CI_PIPELINE_ID}"
        --summary "CI Pipeline ${CI_PIPELINE_ID} completed"
  
  artifacts:
    paths:
      - campaigns/CI_Build_${CI_PIPELINE_ID}/
      - CI_Build_${CI_PIPELINE_ID}_evidence.tar.gz
    expire_in: 30 days
```

### Jenkins Pipeline Example

```groovy
pipeline {
    agent any
    
    stages {
        stage('Test Campaign') {
            steps {
                script {
                    def campaignName = "Jenkins_Build_${env.BUILD_NUMBER}"
                    
                    sh "./scripts/campaign-start.sh --name '${campaignName}'"
                    
                    sh """
                        ./scripts/campaign-run.sh \
                            --campaign 'campaigns/${campaignName}' \
                            --pattern '.*' \
                            --parallel 4 \
                            --continue-on-error
                    """
                    
                    sh """
                        ./scripts/campaign-stop.sh \
                            --campaign 'campaigns/${campaignName}' \
                            --collect-evidence
                    """
                }
            }
        }
    }
    
    post {
        always {
            archiveArtifacts artifacts: 'campaigns/**/*', fingerprint: true
            archiveArtifacts artifacts: '*_evidence.tar.gz*', fingerprint: true
        }
    }
}
```

---

## Best Practices

### Campaign Naming

Use descriptive, date-based names:
- `Sprint_23_Regression`
- `Release_v2.0_Validation`
- `Daily_Smoke_2024-01-15`
- `Hotfix_TICKET-123_Verification`

### Test Organization

- Use `--pattern` to organize tests into logical groups
- Run critical tests first
- Use `--parallel` for independent tests
- Use `--continue-on-error` for comprehensive test execution

### Evidence Collection

- Collect evidence at regular intervals during long campaigns
- Always collect evidence before stopping a campaign
- Use `--checksums` for compliance and audit purposes
- Archive evidence to long-term storage

### Campaign Lifecycle

1. **Planning**: Define campaign scope and test selection criteria
2. **Execution**: Run tests in organized batches with clear patterns
3. **Monitoring**: Review run metadata between executions
4. **Collection**: Gather evidence for audit trail
5. **Finalization**: Stop campaign with comprehensive summary

---

## Troubleshooting

### Campaign Already Exists

**Error:** `Campaign directory already exists`

**Solution:** Use a different name or remove the existing campaign:
```bash
rm -rf campaigns/existing_campaign
```

### Campaign Not Active

**Error:** `Campaign is not active (current state: COMPLETED)`

**Solution:** You cannot run tests on a completed campaign. Start a new campaign.

### No Tests Found

**Error:** `No test cases found matching pattern`

**Solution:** Verify the pattern is correct and test cases exist:
```bash
# List test cases in directory
find testcases -name "*.yml" -o -name "*.yaml"

# Test pattern locally
echo "test_case.yml" | grep -E "PATTERN"
```

### Permission Errors

**Error:** Permission denied errors

**Solution:** Ensure scripts are executable:
```bash
chmod +x scripts/campaign-*.sh
```

---

## Make Targets

The campaign management system is integrated into the project's Makefile for easy testing and demonstration:

### test-campaigns

Runs automated tests of the campaign management system (included in `make test`):

```bash
make test-campaigns
```

This target:
- Creates a test campaign
- Runs a test case through the campaign
- Collects evidence with checksums
- Stops the campaign
- Verifies all artifacts were created correctly
- Cleans up the test campaign

### test-campaigns-full

Runs comprehensive campaign tests with multiple test patterns:

```bash
make test-campaigns-full
```

This target:
- Creates a test campaign
- Runs multiple batches of tests with different patterns
- Tests directory overrides
- Collects evidence
- Stops the campaign with auto-evidence collection
- Cleans up all artifacts

### campaign-demo

Runs an interactive demonstration of the campaign workflow:

```bash
make campaign-demo
```

This target provides a step-by-step walkthrough of:
1. Starting a campaign
2. Running tests
3. Collecting evidence
4. Stopping the campaign

After completion, artifacts are preserved for inspection (not automatically cleaned up).

### Integration with `make test`

Campaign tests are automatically included in the standard test suite:

```bash
make test  # Includes test-campaigns
```

This ensures the campaign management system is validated with every test run.

---

## Related Documentation

- [Test Executor Documentation](../crates/test-executor/README.md)
- [Verifier Documentation](../crates/verifier/README.md)
- [Test Case Schema](../schemas/README.md)
- [AGENTS.md](../AGENTS.md) - Development and build guidelines
- [Makefile](../Makefile) - Build and test targets

---

## Support

For issues, questions, or contributions, please refer to the project README and contribution guidelines.
