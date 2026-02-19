# GitHub Actions Docker Workflow Validation Test

## Overview

This document describes the end-to-end integration test for validating the GitHub Actions Docker workflow configuration (`../.github/workflows/docs.yml`).

## Test Script

**Location:** `tests/integration/test_github_actions_docker_workflow_e2e.sh`

## Purpose

The test validates the GitHub Actions workflow configuration to ensure it:
1. Uses proper YAML syntax and structure
2. Configures Python 3.11 with pip install from requirements.txt
3. Runs mkdocs build commands in a containerized environment
4. Properly uploads the site/ directory as a Pages artifact
5. Sets ENABLE_PDF_EXPORT=1 environment variable for PDF generation
6. Uploads PDF artifacts from site/pdf/ directory
7. Triggers on push to main and workflow_dispatch events
8. Prevents conflicting deployments with concurrency configuration

## What It Tests

### 1. Prerequisites Check (Test 1)
- Docker installation and daemon status
- Presence of required workflow files
- Presence of requirements.txt

### 2. YAML Syntax Validation (Test 2)
- Validates `.github/workflows/docs.yml` has valid YAML syntax
- Uses Python's yaml module for syntax checking

### 3. Workflow Triggers (Test 3)
- Verifies workflow triggers on push to main branch
- Verifies workflow_dispatch trigger is configured
- Ensures proper event-based workflow execution

### 4. Concurrency Configuration (Test 4)
- Validates concurrency group is set to "pages"
- Verifies cancel-in-progress is set to false
- Ensures deployments don't conflict

### 5. Build Job Configuration (Test 5)
- Verifies Python 3.11 is used
- Checks pip install from requirements.txt
- Validates mkdocs build command execution
- Tests containerized build environment

### 6. Upload Pages Artifact (Test 6)
- Verifies actions/upload-pages-artifact is used
- Checks artifact path is set to site/
- Validates artifact upload configuration

### 7. Containerized Build Test (Test 7)
- Simulates GitHub Actions runner environment using Docker
- Pulls python:3.11 official image
- Tests pip install in containerized environment
- Validates mkdocs build in containerized environment
- Verifies site/ directory creation and content

### 8. Build-PDF Job Configuration (Test 8)
- Verifies Python 3.11 is used for PDF build
- Validates ENABLE_PDF_EXPORT=1 environment variable
- Checks system dependencies installation (libcairo2, etc.)
- Ensures proper PDF generation setup

### 9. PDF Artifact Upload (Test 9)
- Verifies actions/upload-artifact is used
- Checks artifact name is "documentation-pdf"
- Validates artifact path is site/pdf/
- Ensures proper PDF artifact configuration

### 10. Build-PDF Containerized Test (Test 10)
- Simulates build-pdf job with ENABLE_PDF_EXPORT=1
- Tests system dependencies installation
- Validates PDF generation in containerized environment
- Verifies site/pdf/ directory creation

### 11. Deploy Job Configuration (Test 11)
- Verifies deploy job depends on build job
- Checks actions/deploy-pages action usage
- Validates github-pages environment configuration
- Ensures proper deployment workflow

### 12. Permissions Configuration (Test 12)
- Validates contents: write permission
- Checks pages: write permission
- Verifies id-token: write permission
- Ensures proper GitHub Pages deployment permissions

### 13. Complete Workflow Simulation (Test 13)
- Simulates entire workflow end-to-end
- Tests all workflow steps in sequence
- Validates artifact generation for both jobs
- Ensures complete workflow functionality

## Usage

### Run the Test

```bash
# Run the test directly
./tests/integration/test_github_actions_docker_workflow_e2e.sh

# Or use the Makefile target
make github-actions-test

# Keep temporary files for debugging
./tests/integration/test_github_actions_docker_workflow_e2e.sh --no-remove
```

### Prerequisites

- **Docker**: Must be installed and running
- **Python 3**: Required for YAML syntax validation
- **Bash 3.2+**: Compatible with macOS and Linux

### Expected Output

The test will output:
- Section headers for each test phase
- ✓ Pass indicators (green) for successful checks
- ✗ Fail indicators (red) for failed checks
- ℹ Info messages for additional context
- Test summary with pass/fail counts

### Exit Codes

- `0`: All tests passed
- `1`: One or more tests failed

## Test Validation Coverage

### Configuration Validation
- ✓ YAML syntax correctness
- ✓ Python version specification (3.11)
- ✓ Dependencies installation method
- ✓ Build commands
- ✓ Environment variables
- ✓ Artifact paths and names
- ✓ Permissions configuration

### Functional Validation
- ✓ Docker image compatibility
- ✓ pip install in containerized environment
- ✓ mkdocs build execution
- ✓ Site directory generation
- ✓ PDF generation with environment variable
- ✓ System dependencies installation
- ✓ Artifact directory structure

### Workflow Validation
- ✓ Trigger configuration
- ✓ Concurrency settings
- ✓ Job dependencies
- ✓ Deployment configuration
- ✓ Environment setup

## Related Files

- **Workflow**: `.github/workflows/docs.yml`
- **Test Script**: `tests/integration/test_github_actions_docker_workflow_e2e.sh`
- **Simulation Script**: `scripts/simulate-github-actions-workflow.sh`
- **Requirements**: `requirements.txt`
- **Logger Library**: `scripts/lib/logger.sh`

## Continuous Integration

This test should be run:
- Before committing changes to the GitHub Actions workflow
- After modifying documentation build dependencies
- When updating Python version or MkDocs configuration
- As part of local validation before pushing to GitHub

## Troubleshooting

### Docker Issues

If Docker-related tests fail:
1. Ensure Docker daemon is running
2. Check Docker has sufficient resources
3. Verify network connectivity for image pulls
4. Test Docker access: `docker run --rm hello-world`

### YAML Syntax Errors

If YAML validation fails:
1. Check `.github/workflows/docs.yml` for syntax errors
2. Validate with: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/docs.yml'))"`
3. Use online YAML validators if needed

### Build Failures

If mkdocs build fails:
1. Check requirements.txt dependencies
2. Verify mkdocs.yml configuration
3. Ensure docs/ directory exists and is populated
4. Test locally: `pip install -r requirements.txt && mkdocs build`

### PDF Generation Issues

If PDF tests fail:
1. PDF generation may require system dependencies
2. This is expected in some environments
3. Warnings are logged but don't fail the test
4. GitHub Actions will install system dependencies as needed

## Related Documentation

- [GitHub Actions Workflow Simulation](README_GITHUB_ACTIONS_SIMULATE.md)
- [GitLab CI Test Documentation](README_GITLAB_CI_TEST.md)
- [MkDocs Test Documentation](README_MKDOCS_TEST.md)
- [Docker Volume Permissions Test](README_DOCKER_VOLUME_PERMISSIONS_TEST.md)

## Notes

- The test uses Docker to simulate the GitHub Actions runner environment
- Tests are designed to be portable across macOS and Linux
- Temporary files are automatically cleaned up unless --no-remove is specified
- The test validates both configuration and functional aspects of the workflow
- PDF generation tests may produce warnings if system dependencies aren't available
