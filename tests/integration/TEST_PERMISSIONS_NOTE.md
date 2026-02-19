# Test Script Permissions Note

## New Script Created

The following test script was created and needs execute permissions:

- `tests/integration/test_docker_mkdocs_config_validation_e2e.sh`

## Setting Execute Permission

Before running the test, set execute permission:

```bash
chmod +x tests/integration/test_docker_mkdocs_config_validation_e2e.sh
```

Or use git to update permissions:

```bash
git add tests/integration/test_docker_mkdocs_config_validation_e2e.sh
git update-index --chmod=+x tests/integration/test_docker_mkdocs_config_validation_e2e.sh
```

## Verification

Verify the script has execute permission:

```bash
ls -l tests/integration/test_docker_mkdocs_config_validation_e2e.sh
```

Expected output should show `-rwxr-xr-x` or similar with `x` flags.

## Running the Test

Once permissions are set:

```bash
# Via Makefile (recommended)
make docs-docker-test-config

# Direct execution
./tests/integration/test_docker_mkdocs_config_validation_e2e.sh
```
