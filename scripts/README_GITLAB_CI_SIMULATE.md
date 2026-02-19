# GitLab CI Pages Job Simulation Script

## Overview

The GitLab CI Pages job simulation script (`simulate-gitlab-ci-pages.sh`) allows you to locally simulate the GitLab CI Pages deployment pipeline before pushing changes to GitLab. This helps catch issues early and reduces the need for trial-and-error commits.

## Script Location

```
scripts/simulate-gitlab-ci-pages.sh
```

## Purpose

This script simulates the exact steps that GitLab CI executes in the Pages job:

1. Pull the python:3.11 Docker image
2. Install dependencies from requirements.txt
3. Build documentation with ENABLE_PDF_EXPORT=1
4. Generate artifacts in the public/ directory
5. Verify artifact contents and structure

## Usage

### Basic Execution

```bash
make gitlab-ci-simulate
```

or

```bash
./scripts/simulate-gitlab-ci-pages.sh
```

### With Options

```bash
# Clean previous artifacts before running
./scripts/simulate-gitlab-ci-pages.sh --clean

# Keep temporary files for debugging
./scripts/simulate-gitlab-ci-pages.sh --keep-temp

# Enable verbose output
./scripts/simulate-gitlab-ci-pages.sh --verbose

# Combine options
./scripts/simulate-gitlab-ci-pages.sh --clean --verbose

# Show help
./scripts/simulate-gitlab-ci-pages.sh --help
```

## Command Options

| Option | Description |
|--------|-------------|
| `--clean` | Remove existing public/ directory before simulation |
| `--keep-temp` | Keep temporary working directory for debugging |
| `--verbose` | Show detailed output including pip and mkdocs logs |
| `--help` | Display help message with usage information |

## Prerequisites

### Required Software

- **Docker**: Must be installed and running
- **bash 3.2+**: Required for script execution

### Required Files

- `.gitlab-ci.yml` - GitLab CI configuration file
- `requirements.txt` - Python dependencies for MkDocs
- `mkdocs.yml` - MkDocs configuration file
- `docs/` directory - Documentation source files

## Simulation Steps

### Step 1: Prerequisites Check

Verifies:
- Docker is installed and accessible
- Docker daemon is running
- All required files exist (.gitlab-ci.yml, requirements.txt, mkdocs.yml)
- docs/ directory exists

### Step 2: Pull Docker Image

- Pulls the python:3.11 Docker image
- Same image used in GitLab CI
- Cached locally for faster subsequent runs

### Step 3: Prepare Working Directory

- Creates temporary working directory
- Copies all required files
- Isolates simulation from project directory

### Step 4: Install Dependencies

- Runs `pip install -r requirements.txt` in container
- Reports installation time
- Same as GitLab CI "script" step

### Step 5: Build Documentation

- Runs `ENABLE_PDF_EXPORT=1 mkdocs build --site-dir public`
- Generates HTML and PDF documentation
- Reports build time

### Step 6: Verify Artifacts

- Checks public/ directory was created
- Verifies index.html exists
- Checks for PDF documentation
- Reports file counts and total size

### Step 7: Copy Artifacts

- Copies generated public/ directory to project root
- Overwrites existing public/ directory if present
- Makes artifacts available for local viewing

### Step 8: Artifact Configuration Info

- Displays information about GitLab CI artifact configuration
- Shows expiration settings (30 days)
- Explains artifact availability in GitLab

### Step 9: Cache Information

- Displays information about pip cache configuration
- Explains how cache improves build performance
- Shows cache path (~/.cache/pip)

## Expected Output

### Success Output

```
=== GitLab CI Pages Job Simulation ===
[INFO] Project root: /path/to/project
[INFO] Docker image: python:3.11
[INFO] Output directory: /path/to/project/public

=== Step 1: Checking Prerequisites ===
✓ Docker is installed
✓ Docker daemon is running
✓ All required files found
✓ docs/ directory found

=== Step 2: Pulling Docker Image ===
[INFO] Pulling python:3.11...
✓ Docker image pulled successfully

=== Step 3: Preparing Working Directory ===
[INFO] Copying project files to temporary directory...
✓ Working directory prepared: /tmp/tmp.XXXXXXXXXX

=== Step 4: Installing Dependencies ===
[INFO] Running: pip install -r requirements.txt
✓ Dependencies installed successfully (15s)

=== Step 5: Building Documentation ===
[INFO] Running: ENABLE_PDF_EXPORT=1 mkdocs build --site-dir public
✓ Documentation built successfully (8s)

=== Step 6: Verifying Artifacts ===
[INFO] Checking public/ directory contents...
✓ public/ directory created
✓ index.html found
✓ PDF documentation directory found
✓ PDF files generated: 1
[INFO] Total files generated: 127
[INFO] Total size: 5.2M

=== Step 7: Copying Artifacts ===
[INFO] Copying artifacts to /path/to/project/public...
✓ Artifacts copied to project directory

=== Step 8: Artifact Configuration ===
[INFO] In GitLab CI, artifacts would be configured as:
[INFO]   - Path: public/
[INFO]   - Expiration: 30 days
[INFO]   - Available in: GitLab Pages, pipeline artifacts

=== Step 9: Cache Information ===
[INFO] In GitLab CI, pip cache would be configured as:
[INFO]   - Path: ~/.cache/pip
[INFO]   - Shared across pipeline runs
[INFO]   - Reduces subsequent build times

=== Simulation Summary ===

✓ GitLab CI Pages job simulated successfully!

[INFO] Pipeline steps completed:
[INFO]   ✓ Docker image pulled
[INFO]   ✓ Dependencies installed (15s)
[INFO]   ✓ Documentation built (8s)
[INFO]   ✓ Artifacts verified and copied

[INFO] Output directory: /path/to/project/public
[INFO] You can view the generated site by running:
[INFO]   python3 -m http.server 8000 --directory /path/to/project/public

[INFO] Or open directly in browser:
[INFO]   open /path/to/project/public/index.html
```

### Failure Output

If any step fails, the output will show:

```
✗ [Step description]
[ERROR] Detailed error message
```

The script will exit with code 1 on failure.

## Output Directory

The simulation generates artifacts in the `public/` directory at the project root:

```
public/
├── index.html              # Main documentation page
├── search/                 # Search functionality
├── assets/                 # CSS, JS, images
├── pdf/                    # PDF documentation
│   └── document.pdf        # Generated PDF
└── ...                     # Additional pages and assets
```

## Viewing Results

After successful simulation, view the documentation:

### Option 1: Python HTTP Server

```bash
python3 -m http.server 8000 --directory public
```

Then open http://localhost:8000 in your browser.

### Option 2: Direct Browser Access

```bash
# macOS
open public/index.html

# Linux
xdg-open public/index.html

# Windows (Git Bash)
start public/index.html
```

## Comparing with GitLab CI

The simulation closely matches GitLab CI behavior:

| Aspect | GitLab CI | Simulation | Match |
|--------|-----------|------------|-------|
| Docker image | python:3.11 | python:3.11 | ✅ |
| Install command | pip install -r requirements.txt | Same | ✅ |
| Build command | ENABLE_PDF_EXPORT=1 mkdocs build --site-dir public | Same | ✅ |
| Output directory | public/ | public/ | ✅ |
| Environment | Container isolated | Container isolated | ✅ |
| Artifacts | Uploaded to GitLab | Copied to project | ≈ |
| Cache | GitLab cache | Not simulated | ❌ |

**Note**: Cache behavior is not fully simulated since each Docker run is isolated. In GitLab CI, pip cache persists across pipeline runs.

## Troubleshooting

### Docker Not Available

**Error**: `Docker is not installed or not in PATH`

**Solution**: Install Docker from https://www.docker.com/get-started and ensure it's in your PATH.

### Docker Daemon Not Running

**Error**: `Docker daemon is not running`

**Solution**: Start Docker Desktop or Docker service.

### Required Files Missing

**Error**: `Required file not found: [filename]`

**Solution**: Ensure all required files exist:
- .gitlab-ci.yml
- requirements.txt
- mkdocs.yml
- docs/ directory

### pip Install Fails

**Error**: `Failed to install dependencies`

**Solution**: 
- Check requirements.txt for syntax errors
- Verify package versions are available
- Run with `--verbose` flag to see detailed error

### Build Fails

**Error**: `Failed to build documentation`

**Solution**:
- Check mkdocs.yml configuration
- Verify all documentation files exist
- Run with `--verbose` flag to see build errors
- Check for plugin compatibility issues

### Permission Errors

**Error**: Permission denied when copying artifacts

**Solution**: 
- Check write permissions in project directory
- Try running with `--keep-temp` to debug
- Ensure Docker has proper volume mount permissions

## Performance Considerations

- **First Run**: Slower due to Docker image pull and dependency installation (30-60s)
- **Subsequent Runs**: Faster with Docker layer caching (15-30s)
- **Disk Space**: Requires ~500MB for Docker images and artifacts
- **Network**: Required for Docker image pull and pip package downloads

## Development Workflow

### Recommended Workflow

1. Make documentation changes
2. Run simulation locally:
   ```bash
   make gitlab-ci-simulate
   ```
3. Review generated documentation in browser
4. Fix any issues
5. Repeat steps 1-4 until satisfied
6. Commit and push changes
7. GitLab CI will deploy to Pages

### Integration with Testing

Run both validation and simulation:

```bash
# Validate configuration
make gitlab-ci-test

# Simulate pipeline
make gitlab-ci-simulate
```

## Cleanup

### Automatic Cleanup

The script automatically cleans up temporary directories on exit (unless `--keep-temp` is used).

### Manual Cleanup

```bash
# Remove generated artifacts
rm -rf public/

# Remove Docker image
docker rmi python:3.11
```

### Clean Before Running

```bash
./scripts/simulate-gitlab-ci-pages.sh --clean
```

## Debugging

### Keep Temporary Files

```bash
./scripts/simulate-gitlab-ci-pages.sh --keep-temp
```

This preserves the temporary working directory for inspection.

### Enable Verbose Output

```bash
./scripts/simulate-gitlab-ci-pages.sh --verbose
```

Shows detailed pip and mkdocs output.

### Inspect Docker Container

To manually inspect the container environment:

```bash
docker run -it --rm -v "$(pwd):/workspace" python:3.11 bash
cd /workspace
pip install -r requirements.txt
ENABLE_PDF_EXPORT=1 mkdocs build --site-dir public
```

## CI/CD Integration

While primarily for local development, the script can be used in CI/CD:

```yaml
validate:
  stage: test
  script:
    - ./scripts/simulate-gitlab-ci-pages.sh --verbose
  artifacts:
    paths:
      - public/
    expire_in: 1 day
```

## Security Considerations

- Runs in isolated Docker containers
- No modifications to project files (except public/ directory)
- Temporary files are automatically cleaned up
- No persistent state or credentials required

## Compatibility

- ✅ macOS (bash 3.2+)
- ✅ Linux (bash 3.2+)
- ✅ BSD variants
- ✅ Docker Desktop
- ✅ Docker Engine
- ✅ Windows (Git Bash with Docker Desktop)

## Exit Codes

- `0` - Simulation completed successfully
- `1` - Simulation failed at any step

## Related Documentation

- [GitLab CI Test Documentation](README_GITLAB_CI_TEST.md)
- [GitLab CI Test Quick Reference](GITLAB_CI_TEST_QUICK_REF.md)
- [GitLab CI Configuration](../.gitlab-ci.yml)
- [MkDocs Documentation](https://www.mkdocs.org/)
- [AGENTS.md](../AGENTS.md)

## License

This script is part of the testcase-manager project and follows the project's license terms.
