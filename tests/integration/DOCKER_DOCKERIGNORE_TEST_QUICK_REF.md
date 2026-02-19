# Docker .dockerignore Test - Quick Reference

## Run Test
```bash
./tests/integration/test_docker_dockerignore_e2e.sh
```

## What It Tests

### ✓ .dockerignore Content
- Excludes: `target/`, `src/`, `tests/`, `testcases/`
- Excludes: `.git/`, `scripts/`, `backlog/`, `*.profraw`

### ✓ Build Performance
- First build: < 5 minutes
- Incremental build: < 1 minute
- Layer caching: > 5 cached layers

### ✓ Context Optimization
- Build context: MB or kB (not GB)
- Context reduction: Measured vs no .dockerignore
- Excluded files not in image

### ✓ Image Verification
- Excluded directories not present
- Required files present (docs/, mkdocs.yml)
- Image builds successfully

## Test Sections

1. **Prerequisites** - Docker available
2. **Content Verification** - Required excludes present
3. **First Build** - Timing and success
4. **Context Size** - Optimized context
5. **Exclusion Check** - Files not in image
6. **Layer Caching** - Fast incremental builds
7. **Comparison** - With vs without .dockerignore
8. **Actual Command** - `docker build -f Dockerfile.mkdocs -t test-mkdocs .`
9. **Cleanup** - Restore configuration
10. **Summary** - Final statistics

## Command Options

### Verbose Output
```bash
./tests/integration/test_docker_dockerignore_e2e.sh --verbose
```

### Keep Temporary Files
```bash
./tests/integration/test_docker_dockerignore_e2e.sh --no-remove
```

## Expected Results

### First Build
- Time: 2-4 minutes (max 5 minutes)
- Context: < 50 MB
- Success: Image created

### Incremental Build
- Time: 10-30 seconds (max 1 minute)
- Cached: > 5 layers
- Success: Image updated

## Key Metrics

| Metric | With .dockerignore | Without .dockerignore |
|--------|-------------------|----------------------|
| Context Size | < 50 MB | > 100 MB |
| Build Time | 2-4 min | 5-10 min |
| Incremental | < 1 min | > 2 min |

## Troubleshooting

### Build Too Slow
```bash
# Clear build cache
docker builder prune -f

# Check Docker resources
docker info | grep -i memory
```

### Context Too Large
```bash
# Check .dockerignore is active
ls -la .dockerignore

# Verify it's the mkdocs version
diff .dockerignore .dockerignore.mkdocs
```

### Tests Failing
```bash
# Run with verbose output
./tests/integration/test_docker_dockerignore_e2e.sh --verbose

# Check Docker is running
docker info
```

## Quick Validation

```bash
# Check .dockerignore.mkdocs content
grep -E "(target|src|tests|testcases)" .dockerignore.mkdocs

# Build with timing
time docker build -f Dockerfile.mkdocs -t test-mkdocs .

# Check context size in output
# Look for: "Sending build context to Docker daemon  XX.XXkB"
```

## Exit Codes

- `0` = All tests passed
- `1` = One or more tests failed

## Related Tests

- `test_docker_mkdocs_e2e.sh` - General Docker MkDocs tests
- `test_docker_html_build_e2e.sh` - HTML build tests
- `test_docker_pdf_build_e2e.sh` - PDF build tests

## Documentation

Full details: `tests/integration/README_DOCKER_DOCKERIGNORE_TEST.md`
