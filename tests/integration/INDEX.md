# Integration Tests - Documentation Index

Quick navigation guide for all integration test documentation.

## 📚 Documentation Files

### For Users

| File | Purpose | When to Read |
|------|---------|--------------|
| [README.md](README.md) | Main user documentation | **Start here** - First time running tests |
| [QUICK_REFERENCE.md](QUICK_REFERENCE.md) | Command cheat sheet | Quick lookup of commands and options |
| [VALIDATE_FILES_QUICK_REF.md](VALIDATE_FILES_QUICK_REF.md) | validate-files.sh quick reference | Quick lookup for validation tests |

### For Developers

| File | Purpose | When to Read |
|------|---------|--------------|
| [TESTING_GUIDE.md](TESTING_GUIDE.md) | Comprehensive testing guide | Deep dive into test architecture |
| [test_scenarios.md](test_scenarios.md) | Test coverage matrix | Understanding what's tested |
| [IMPLEMENTATION_SUMMARY.md](IMPLEMENTATION_SUMMARY.md) | Implementation details | Learning how tests work |
| [VALIDATE_FILES_TEST_COVERAGE.md](VALIDATE_FILES_TEST_COVERAGE.md) | validate-files.sh test coverage | Detailed validation test documentation |

### Reference

| File | Purpose | When to Read |
|------|---------|--------------|
| [INDEX.md](INDEX.md) | This file | Navigation and overview |
| [TIMEOUT_FIX.md](TIMEOUT_FIX.md) | Timeout fix details | Understanding timeout resolution |

## 🧪 Test Files

### Expect Scripts

| File | Description | Duration | Use Case |
|------|-------------|----------|----------|
| `e2e_complete_workflow.exp` | Full workflow test | ~30s | Complete regression testing |
| `e2e_basic_workflow.exp` | Quick smoke test | ~10s | Pre-commit validation |
| `validate_files_integration.exp` | validate-files.sh tests | ~15-30s | Validation framework testing |

### Shell Runners

| File | Description | Usage |
|------|-------------|-------|
| `run_e2e_test.sh` | Single test runner | `./run_e2e_test.sh [--build]` |
| `run_validate_files_test.sh` | validate-files.sh test runner | `./run_validate_files_test.sh` |
| `test_json_escape_e2e.sh` | json-escape feature test | `./test_json_escape_e2e.sh [--no-remove]` |
| `run_all_tests.sh` | All tests runner | `./run_all_tests.sh [--build]` |
| `ci_test.sh` | CI-friendly runner | `./ci_test.sh` |
| `check_environment.sh` | Environment checker | `./check_environment.sh` |

## 🎯 Quick Start Paths

### I want to run tests quickly
1. Read [QUICK_REFERENCE.md](QUICK_REFERENCE.md)
2. Run `make test-e2e`

### I'm setting up for the first time
1. Read [README.md](README.md) → Prerequisites section
2. Run `./check_environment.sh`
3. Run `make test-e2e-all`

### I'm debugging a test failure
1. Read [TESTING_GUIDE.md](TESTING_GUIDE.md) → Debugging Tests section
2. Review error output
3. Check [QUICK_REFERENCE.md](QUICK_REFERENCE.md) → Common Issues

### I'm adding a new test
1. Read [TESTING_GUIDE.md](TESTING_GUIDE.md) → Extending Tests section
2. Copy test template
3. Update [test_scenarios.md](test_scenarios.md)
4. Update [IMPLEMENTATION_SUMMARY.md](IMPLEMENTATION_SUMMARY.md)

### I'm setting up CI/CD
1. Read [TESTING_GUIDE.md](TESTING_GUIDE.md) → CI/CD Integration section
2. Use `ci_test.sh` runner
3. Reference `.github/workflows/integration-tests.yml`

### I need to understand coverage
1. Read [test_scenarios.md](test_scenarios.md) → Coverage Matrix
2. Review test data in [TESTING_GUIDE.md](TESTING_GUIDE.md)

## 📖 Document Purposes

### README.md
**For**: End users and developers  
**Contains**:
- Overview of integration tests
- Prerequisites and installation
- Running instructions
- Test descriptions
- Expected output examples
- Troubleshooting guide

### QUICK_REFERENCE.md
**For**: Developers needing quick answers  
**Contains**:
- One-line commands
- Test data reference
- Common issues and solutions
- Performance metrics
- Exit codes
- Pro tips

### TESTING_GUIDE.md
**For**: Deep understanding and debugging  
**Contains**:
- Test architecture details
- Technology stack explanation
- Debugging instructions
- Test data reference
- CI/CD integration examples
- Best practices
- Extending tests
- Troubleshooting details

### test_scenarios.md
**For**: Understanding test coverage  
**Contains**:
- Coverage matrix
- Scenario descriptions
- Test data
- Validation checks
- Error scenarios
- Future enhancements

### IMPLEMENTATION_SUMMARY.md
**For**: Understanding implementation  
**Contains**:
- What was implemented
- File listing
- Test coverage details
- Technical details
- Benefits
- Future enhancements

### INDEX.md (this file)
**For**: Navigation  
**Contains**:
- Document overview
- File descriptions
- Quick start paths
- Navigation guide

## 🔍 Finding Information

### Commands and Usage
→ [QUICK_REFERENCE.md](QUICK_REFERENCE.md)

### Prerequisites and Setup
→ [README.md](README.md) → Prerequisites

### Running Tests
→ [README.md](README.md) → Running the Tests  
→ [QUICK_REFERENCE.md](QUICK_REFERENCE.md) → One-Line Commands

### Test Coverage
→ [test_scenarios.md](test_scenarios.md) → Coverage Matrix  
→ [IMPLEMENTATION_SUMMARY.md](IMPLEMENTATION_SUMMARY.md) → Test Coverage

### Debugging
→ [TESTING_GUIDE.md](TESTING_GUIDE.md) → Debugging Tests  
→ [QUICK_REFERENCE.md](QUICK_REFERENCE.md) → Common Issues

### CI/CD Integration
→ [TESTING_GUIDE.md](TESTING_GUIDE.md) → CI/CD Integration  
→ `.github/workflows/integration-tests.yml`

### Adding Tests
→ [TESTING_GUIDE.md](TESTING_GUIDE.md) → Extending Tests  
→ [test_scenarios.md](test_scenarios.md) → Extending Tests

### Test Data
→ [test_scenarios.md](test_scenarios.md) → Test Data  
→ [TESTING_GUIDE.md](TESTING_GUIDE.md) → Test Data Reference  
→ [QUICK_REFERENCE.md](QUICK_REFERENCE.md) → Test Data

### Architecture
→ [TESTING_GUIDE.md](TESTING_GUIDE.md) → Test Architecture  
→ [IMPLEMENTATION_SUMMARY.md](IMPLEMENTATION_SUMMARY.md) → Technical Details

### Performance
→ [QUICK_REFERENCE.md](QUICK_REFERENCE.md) → Performance  
→ [IMPLEMENTATION_SUMMARY.md](IMPLEMENTATION_SUMMARY.md) → Performance

### Troubleshooting
→ [README.md](README.md) → Troubleshooting  
→ [TESTING_GUIDE.md](TESTING_GUIDE.md) → Troubleshooting  
→ [QUICK_REFERENCE.md](QUICK_REFERENCE.md) → Common Issues

## 🚀 Common Tasks

### Run a quick test
```bash
make test-e2e
```
See: [QUICK_REFERENCE.md](QUICK_REFERENCE.md)

### Run all tests
```bash
make test-e2e-all
```
See: [README.md](README.md)

### Check environment
```bash
./tests/integration/check_environment.sh
```
See: [TESTING_GUIDE.md](TESTING_GUIDE.md)

### Debug a failure
1. Review error output
2. Check [TESTING_GUIDE.md](TESTING_GUIDE.md) → Debugging
3. Run with verbose logging
4. Inspect test artifacts

### Add a new test
1. Copy template from [TESTING_GUIDE.md](TESTING_GUIDE.md)
2. Modify workflow
3. Update `run_all_tests.sh`
4. Update documentation

## 📊 Test Statistics

| Metric | Value |
|--------|-------|
| Test Files | 4 (basic + complete + validate-files + json-escape) |
| Runner Scripts | 6 |
| Documentation Files | 8 |
| Total Lines of Code | ~3200+ |
| Test Duration | ~1-3 minutes |
| Coverage | 45+ scenarios |
| Git Commits Tested | 7 (complete) / 2 (basic) |
| validate-files Tests | 16 test cases |
| json-escape Tests | 10 test cases (40+ assertions) |

## 🔧 Maintenance

### Regular Updates
- Update tests when CLI changes
- Keep documentation in sync
- Monitor test performance
- Review coverage gaps

### Version Control
All test files are in git:
```
tests/integration/
├── *.exp           # Version controlled
├── *.sh            # Version controlled
├── *.md            # Version controlled
└── test_*          # Ignored (generated)
```

## 💡 Tips

1. **Always start with QUICK_REFERENCE.md** for quick lookups
2. **Read README.md first** if you're new to the tests
3. **Consult TESTING_GUIDE.md** for deep dives
4. **Check test_scenarios.md** to understand coverage
5. **Use INDEX.md (this file)** when lost

## 🆘 Getting Help

1. Check appropriate documentation file (see above)
2. Run environment checker
3. Review test output
4. Check if CLI prompts changed
5. Open an issue with details

## 📝 Contributing

When contributing to integration tests:

1. **Write tests first** for new features
2. **Update documentation** with changes
3. **Keep tests fast** (<1 minute total)
4. **Follow patterns** from existing tests
5. **Document test data** in test_scenarios.md

## 🎓 Learning Path

### Beginner
1. Read [README.md](README.md)
2. Run `make test-e2e`
3. Review output

### Intermediate
1. Read [TESTING_GUIDE.md](TESTING_GUIDE.md)
2. Run individual tests
3. Debug a failure

### Advanced
1. Read [IMPLEMENTATION_SUMMARY.md](IMPLEMENTATION_SUMMARY.md)
2. Add a new test
3. Contribute improvements

## 📦 Package Contents

```
tests/integration/
├── Documentation/
│   ├── INDEX.md                           # This file
│   ├── README.md                          # User guide
│   ├── QUICK_REFERENCE.md                # Command reference
│   ├── TESTING_GUIDE.md                  # Comprehensive guide
│   ├── test_scenarios.md                 # Coverage details
│   ├── IMPLEMENTATION_SUMMARY.md         # Implementation info
│   ├── VALIDATE_FILES_TEST_COVERAGE.md   # validate-files.sh coverage
│   └── VALIDATE_FILES_QUICK_REF.md       # validate-files.sh quick ref
│
├── Test Scripts/
│   ├── e2e_complete_workflow.exp         # Full test
│   ├── e2e_basic_workflow.exp            # Quick test
│   └── validate_files_integration.exp    # validate-files.sh tests
│
├── Runners/
│   ├── run_e2e_test.sh                   # Single test
│   ├── run_validate_files_test.sh        # validate-files.sh test
│   ├── run_all_tests.sh                  # All tests
│   ├── ci_test.sh                        # CI runner
│   └── check_environment.sh              # Environment check
│
└── Generated/ (not in git)
    ├── test_e2e_*/                       # Test artifacts
    ├── test_basic_*/                     # Test artifacts
    └── test_validate_files_*/            # validate-files artifacts
```

## Docker MkDocs Tests

### test_docker_mkdocs_e2e.sh
**Purpose**: End-to-end validation of Docker-based MkDocs documentation setup

**What it tests**:
- Dockerfile.mkdocs syntax and best practices
- Docker image build process
- Image creation and verification
- Python dependencies (mkdocs, mkdocs-material, mkdocs-with-pdf)
- System dependencies for PDF generation
- Non-root user configuration and permissions
- Environment variables and defaults
- Image functionality with test documentation build

**Usage**:
```bash
make docs-docker-test
# or
./tests/integration/test_docker_mkdocs_e2e.sh [--no-remove]
```

**Documentation**: See [README_DOCKER_MKDOCS_TEST.md](README_DOCKER_MKDOCS_TEST.md) and [DOCKER_MKDOCS_TEST_QUICK_REF.md](DOCKER_MKDOCS_TEST_QUICK_REF.md)

### test_docker_html_build_e2e.sh
**Purpose**: End-to-end validation of Docker container HTML build process

**What it tests**:
- Docker container HTML builds via `make docs-docker-build-site`
- Site directory creation with complete HTML structure
- Markdown to HTML conversion for all docs
- Asset copying (CSS, JS, images)
- File ownership and permissions from host perspective
- Multiple sequential builds
- Site directory cleanup and recovery
- HTML content quality and structure
- Navigation sections and completeness

**Usage**:
```bash
make docs-docker-test-html
# or
./tests/integration/test_docker_html_build_e2e.sh [--verbose] [--no-remove]
```

**Documentation**: See [README_DOCKER_HTML_BUILD_TEST.md](README_DOCKER_HTML_BUILD_TEST.md) and [DOCKER_HTML_BUILD_TEST_QUICK_REF.md](DOCKER_HTML_BUILD_TEST_QUICK_REF.md)

---

**Last Updated**: 2024 (automatically updated with test changes)  
**Maintained By**: Project contributors  
**License**: Same as project
