# GitLab CI/CD Pipeline Setup

This document describes the GitLab CI/CD pipeline configuration for automated testing, coverage analysis, performance benchmarking, and Docker image building.

## Overview

The pipeline consists of the following stages:

1. **Build** - Compile the Rust project with all features
2. **Test** - Run unit tests, E2E tests, and doc tests
3. **Lint** - Run Clippy and rustfmt checks
4. **Coverage** - Generate code coverage reports
5. **Docker** - Build, verify, and push Docker images
6. **Performance** - Run benchmarks and detect regressions
7. **Report** - Generate comprehensive summary report

## Pipeline Features

### 🧪 Automated Testing

- **Unit Tests**: Runs all unit tests with full feature flags
- **E2E Tests**: Executes end-to-end integration tests
- **Doc Tests**: Validates documentation examples
- **JUnit Reports**: Generates XML reports for GitLab integration

### 📊 Code Coverage

- Uses `grcov` for coverage analysis
- Generates LCOV and HTML reports
- Compares coverage with master branch baseline
- Posts coverage changes to merge requests
- Tracks line and function coverage

### 🔍 Code Quality

- **Clippy**: Static analysis with all lints enabled
- **rustfmt**: Code formatting verification
- Automatic MR comments for lint warnings

### 🐳 Docker Integration

- Multi-stage Docker builds
- Image verification and testing
- Basic security scanning
- Automatic image tagging and registry push
- Size and configuration analysis

### ⚡ Performance Monitoring

- Automated benchmarking of key operations
- Baseline comparison with master branch
- Regression detection (>10% threshold)
- Performance trend tracking

### 💬 Automatic MR Comments

The pipeline automatically posts detailed comments on merge requests with:

- Test results (passed/failed counts, failures list)
- Code coverage changes (with trend indicators)
- Clippy warnings and errors
- Performance regressions/improvements
- Docker image analysis
- Comprehensive pipeline summary

## Configuration

### Required GitLab Variables

Set these in your GitLab project settings (`Settings > CI/CD > Variables`):

| Variable | Description | Required |
|----------|-------------|----------|
| `GITLAB_TOKEN` | Personal access token with `api` scope | Yes (for MR comments) |
| `CI_REGISTRY_USER` | Docker registry username | Yes (for docker:push) |
| `CI_REGISTRY_PASSWORD` | Docker registry password | Yes (for docker:push) |

### Built-in GitLab Variables Used

The pipeline uses these automatically provided variables:

- `CI_PROJECT_ID` - Project identifier
- `CI_MERGE_REQUEST_IID` - Merge request ID
- `CI_COMMIT_REF_SLUG` - Branch/tag name
- `CI_COMMIT_SHORT_SHA` - Short commit hash
- `CI_SERVER_URL` - GitLab instance URL
- `CI_JOB_TOKEN` - Job-specific token

## Pipeline Scripts

The CI/CD automation scripts are located in the `scripts/` directory:

### Test Analysis
- `ci_parse_test_results.py` - Parse cargo test output and generate JSON/XML reports
- `ci_post_mr_comment.py` - Post test results to merge requests

### Coverage Analysis
- `ci_compare_coverage.py` - Compare coverage with baseline
- `ci_post_coverage_comment.py` - Post coverage report to MR

### Linting
- `ci_post_clippy_comment.py` - Post Clippy warnings to MR

### Performance
- `ci_benchmark.py` - Run performance benchmarks
- `ci_compare_performance.py` - Compare with baseline performance
- `ci_post_performance_comment.py` - Post performance report to MR

### Docker
- `ci_docker_security_scan.py` - Basic Docker image security analysis

### Reporting
- `ci_generate_summary_report.py` - Generate comprehensive pipeline summary
- `ci_post_summary_comment.py` - Post summary to merge request

## Artifacts

The pipeline generates and stores the following artifacts:

### Build Artifacts (1 week retention)
- Compiled binaries in `target/release/`
- All binaries copied to `artifacts/binaries/`

### Test Artifacts (1 week retention)
- `test_output.txt` - Raw test output
- `test_results.json` - Parsed test results
- `test_results.xml` - JUnit XML report

### Coverage Artifacts (1 month retention)
- `lcov.info` - LCOV coverage data
- `coverage/html/` - HTML coverage report
- `coverage_report.json` - Coverage comparison data

### Docker Artifacts (1 week retention)
- `docker_inspect.json` - Image metadata
- `docker_scan_results.json` - Security scan results

### Performance Artifacts
- `baseline_perf.json` - Master branch baseline (1 month)
- `current_perf.json` - Current branch metrics (1 week)
- `perf_comparison.json` - Comparison results (1 week)

### Summary Artifacts (1 month retention)
- `pipeline_summary.md` - Comprehensive markdown report

## Usage

### Running Locally

You can run individual pipeline stages locally:

```bash
# Build
make build

# Run tests
make test

# Run linting
make lint

# Run benchmarks
python3 scripts/ci_benchmark.py --output perf.json

# Generate coverage
export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="coverage-%p-%m.profraw"
cargo test --all --all-features --tests
grcov . --binary-path ./target/debug/ -s . -t html -o coverage/html
```

### Viewing Reports

After a pipeline run:

1. **Test Results**: Click on the test job and view the JUnit report
2. **Coverage Report**: Download the `coverage/html/` artifact and open `index.html`
3. **Performance Data**: Check the artifacts for JSON benchmark data
4. **Full Summary**: View the `pipeline_summary.md` artifact or the MR comment

### Customizing Thresholds

You can adjust detection thresholds in the Python scripts:

- **Performance regression threshold**: Edit `ci_compare_performance.py` (default: 10%)
- **Coverage drop warning**: Edit `ci_post_coverage_comment.py` (default: -1%)
- **Benchmark iterations**: Edit `ci_benchmark.py` (default: 10)

## Troubleshooting

### MR Comments Not Posting

1. Verify `GITLAB_TOKEN` is set with `api` scope
2. Check that the token has permissions for the project
3. Ensure the pipeline is running on a merge request

### Coverage Job Failing

1. Ensure `grcov` can be downloaded and extracted
2. Check that `llvm-tools-preview` component is available
3. Verify test compilation succeeds with coverage instrumentation

### Docker Jobs Failing

1. Ensure Docker-in-Docker (dind) service is available
2. Check that the Dockerfile is valid
3. Verify registry credentials are correct

### Performance Baseline Missing

On the first run, the performance comparison will have no baseline. Run the pipeline on master first to establish a baseline.

## CI/CD Best Practices

1. **Always run the full pipeline** before merging to master
2. **Review MR comments** for test failures, coverage drops, or performance regressions
3. **Keep coverage above 70%** for critical modules
4. **Investigate performance regressions** exceeding 10%
5. **Address Clippy warnings** before merging

## Maintenance

### Updating Dependencies

When updating the Rust toolchain or dependencies:

1. Update `.gitlab-ci.yml` image version if needed
2. Test the pipeline on a feature branch
3. Update any script dependencies in `requirements.txt` (if created)

### Adding New Test Files

The benchmark script automatically detects test files in `tests/sample/`. To add new benchmarks, simply add test files to that directory.

### Extending the Pipeline

To add new stages or jobs:

1. Define the job in `.gitlab-ci.yml`
2. Create supporting scripts in `scripts/` if needed
3. Update this documentation
4. Test on a feature branch before merging

## Security Considerations

- Never commit secrets or API tokens to the repository
- Use GitLab's CI/CD variables for sensitive data
- Review Docker image security scan results
- Keep dependencies up to date
- Use the minimum required permissions for tokens

## Support

For issues or questions about the CI/CD pipeline:

1. Check the GitLab CI/CD logs for detailed error messages
2. Review this documentation for configuration details
3. Consult the scripts in `scripts/` for implementation details
4. Create an issue in the project repository
