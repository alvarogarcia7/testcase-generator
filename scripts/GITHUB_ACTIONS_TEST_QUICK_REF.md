# GitHub Actions Docker Workflow Test - Quick Reference

## Purpose
Validates GitHub Actions workflow configuration for documentation builds with Docker-based runner simulation.

## Quick Start

```bash
# Run test
make github-actions-test

# Or run directly
./tests/integration/test_github_actions_docker_workflow_e2e.sh

# Debug mode (keep temp files)
./tests/integration/test_github_actions_docker_workflow_e2e.sh --no-remove
```

## What It Tests

✓ YAML syntax validation  
✓ Python 3.11 configuration  
✓ pip install from requirements.txt  
✓ mkdocs build in containerized environment  
✓ upload-pages-artifact configuration  
✓ ENABLE_PDF_EXPORT=1 environment variable  
✓ PDF artifact upload from site/pdf/  
✓ Workflow triggers (push to main, workflow_dispatch)  
✓ Concurrency group (prevents conflicting deployments)  
✓ Permissions (contents, pages, id-token: write)  
✓ Deploy job configuration  
✓ Complete workflow simulation  

## Test Sections

1. **Prerequisites** - Docker, workflow files, requirements.txt
2. **YAML Syntax** - Validates workflow syntax
3. **Workflow Triggers** - push to main, workflow_dispatch
4. **Concurrency** - pages group, cancel-in-progress: false
5. **Build Job** - Python 3.11, pip install, mkdocs build
6. **Upload Artifact** - actions/upload-pages-artifact, path: site/
7. **Containerized Build** - Simulates GitHub Actions runner
8. **Build-PDF Job** - ENABLE_PDF_EXPORT=1, system dependencies
9. **PDF Artifact** - actions/upload-artifact, path: site/pdf/
10. **Build-PDF Container** - Tests PDF generation
11. **Deploy Job** - depends on build, actions/deploy-pages
12. **Permissions** - contents, pages, id-token: write
13. **Complete Simulation** - Full workflow end-to-end

## Expected Results

- All tests pass (exit code 0)
- Detailed test output with ✓/✗ indicators
- Test summary showing pass/fail counts

## Prerequisites

- Docker installed and running
- Python 3 (for YAML validation)
- Bash 3.2+
- Internet access (for Docker image pulls)

## Common Issues

### Docker not running
```bash
# macOS: Start Docker Desktop
# Linux: sudo systemctl start docker
```

### YAML syntax errors
```bash
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/docs.yml'))"
```

### Build failures
```bash
# Test locally
pip install -r requirements.txt
mkdocs build
```

## Files Tested

- `.github/workflows/docs.yml` - GitHub Actions workflow
- `requirements.txt` - Python dependencies
- `mkdocs.yml` - MkDocs configuration

## Output Location

- Test logs: stdout/stderr
- Temporary files: Auto-cleaned (unless --no-remove)

## Related Commands

```bash
# Simulate workflow locally
make github-actions-simulate

# View help
./tests/integration/test_github_actions_docker_workflow_e2e.sh --help
```

## Documentation

See `scripts/README_GITHUB_ACTIONS_TEST.md` for detailed documentation.
