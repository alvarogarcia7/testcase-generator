# GitLab CI/CD Quick Reference

This project includes a comprehensive GitLab CI/CD pipeline with automated testing, coverage analysis, performance benchmarking, and Docker image building.

## 🚀 Quick Start

1. **Setup GitLab Variables** (Settings > CI/CD > Variables):
   - `GITLAB_TOKEN` - Personal access token with `api` scope (for MR comments)
   - `CI_REGISTRY_USER` - Docker registry username
   - `CI_REGISTRY_PASSWORD` - Docker registry password

2. **Push your code** to trigger the pipeline automatically

3. **View results** in merge request comments or pipeline artifacts

## 📋 Pipeline Stages

| Stage | Jobs | Description |
|-------|------|-------------|
| **Build** | `build` | Compile Rust project with release optimizations |
| **Test** | `test:unit`, `test:e2e`, `test:doc` | Run all test suites |
| **Lint** | `lint:clippy`, `lint:rustfmt` | Code quality checks |
| **Coverage** | `coverage` | Generate and compare code coverage |
| **Docker** | `docker:build`, `docker:verify`, `docker:push` | Build and verify Docker image |
| **Performance** | `performance:baseline`, `performance:compare` | Benchmark and detect regressions |
| **Report** | `report:summary` | Generate comprehensive pipeline summary |

## 🎯 Features

### Automatic MR Comments

The pipeline posts detailed comments on merge requests:

- ✅ **Test Results** - Pass/fail counts, failed test names
- 📊 **Coverage Changes** - Line and function coverage comparison
- 🔍 **Lint Warnings** - Clippy warnings and errors
- ⚡ **Performance** - Regressions and improvements
- 🐳 **Docker Analysis** - Image size and security findings
- 📝 **Summary Report** - Comprehensive overview of all stages

### Coverage Tracking

- Compares with master branch baseline
- Warns if coverage drops more than 1%
- Generates HTML reports in artifacts

### Performance Monitoring

- Benchmarks key operations
- Detects regressions >10% slower
- Tracks improvements >10% faster

### Docker Integration

- Multi-stage builds for optimization
- Automatic image verification
- Basic security scanning
- Registry push on master branch

## 📦 Artifacts

View pipeline artifacts for detailed reports:

- `test_results.json` - Parsed test results
- `coverage/html/` - Interactive coverage report
- `perf_comparison.json` - Performance comparison data
- `docker_scan_results.json` - Docker security analysis
- `pipeline_summary.md` - Comprehensive summary

## 🛠️ Running Locally

```bash
# Build
make build

# Test
make test

# Lint
make lint

# Benchmark
python3 scripts/ci_benchmark.py --output perf.json

# Coverage
export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="coverage-%p-%m.profraw"
cargo test --all --all-features --tests
```

## 📚 Documentation

For detailed documentation, see [docs/GITLAB_CI_SETUP.md](docs/GITLAB_CI_SETUP.md)

## 🔧 Scripts

CI/CD automation scripts are in the `scripts/` directory:

- `ci_parse_test_results.py` - Parse test output
- `ci_post_mr_comment.py` - Post test results
- `ci_compare_coverage.py` - Compare coverage
- `ci_post_coverage_comment.py` - Post coverage report
- `ci_post_clippy_comment.py` - Post lint results
- `ci_benchmark.py` - Run benchmarks
- `ci_compare_performance.py` - Compare performance
- `ci_post_performance_comment.py` - Post performance report
- `ci_docker_security_scan.py` - Scan Docker image
- `ci_generate_summary_report.py` - Generate summary
- `ci_post_summary_comment.py` - Post summary

## 🐛 Troubleshooting

### MR Comments Not Appearing

1. Check `GITLAB_TOKEN` is set with `api` scope
2. Verify token has project permissions
3. Ensure running on a merge request

### Coverage Job Failing

1. Verify `grcov` can be downloaded
2. Check `llvm-tools-preview` is available
3. Test compilation with coverage flags

### Docker Build Failing

1. Check Dockerfile syntax
2. Verify Docker-in-Docker service is available
3. Review registry credentials

## 🤝 Contributing

Before submitting a merge request:

1. Ensure all tests pass
2. Review coverage changes
3. Check for Clippy warnings
4. Verify no performance regressions
5. Review the automated MR comments

## 📞 Support

For issues or questions:

1. Check GitLab CI/CD logs
2. Review [docs/GITLAB_CI_SETUP.md](docs/GITLAB_CI_SETUP.md)
3. Create an issue in the repository
