# Docker Setup for MkDocs Documentation

This guide describes how to build and serve the MkDocs documentation using Docker, which provides a consistent environment across different platforms without requiring local Python setup.

## Prerequisites

- Docker installed and running on your system
- Basic familiarity with Docker commands

## Quick Start

### Using Helper Script (Recommended)

A convenience script is provided for common Docker operations:

```bash
# Build the Docker image
./scripts/docker-mkdocs.sh build

# Serve documentation locally
./scripts/docker-mkdocs.sh serve

# Build static site
./scripts/docker-mkdocs.sh build-site

# Build with PDF
./scripts/docker-mkdocs.sh build-pdf

# Check status
./scripts/docker-mkdocs.sh status
```

### Using Make Commands

Alternatively, use Make commands directly:

**Build the Docker image:**
```bash
make docs-docker-build
```

**Serve documentation locally:**
```bash
make docs-docker-serve
```

The documentation will be available at [http://localhost:8000](http://localhost:8000).

**Build static HTML documentation:**
```bash
make docs-docker-build-site
```

The generated site will be available in the `site/` directory.

**Build with PDF export:**
```bash
make docs-docker-build-pdf
```

The PDF will be available at `site/pdf/testcase-manager-documentation.pdf`.

Changes to documentation files will automatically trigger a rebuild when using the serve command.

Press `Ctrl+C` to stop the development server.

## Docker Commands Reference

### Standard Docker Commands

| Command | Description |
|---------|-------------|
| `make docs-docker-build` | Build the MkDocs Docker image |
| `make docs-docker-serve` | Serve documentation locally with live reload |
| `make docs-docker-build-site` | Build static HTML documentation |
| `make docs-docker-build-pdf` | Build documentation with PDF export |
| `make docs-docker-clean` | Remove the MkDocs Docker image |

### Docker Compose Commands

| Command | Description |
|---------|-------------|
| `make docs-compose-up` | Start documentation server with Docker Compose |
| `make docs-compose-build-site` | Build site with Docker Compose |
| `make docs-compose-build-pdf` | Build site with PDF using Docker Compose |
| `make docs-compose-down` | Stop Docker Compose services |

## Using Docker Compose

Docker Compose provides an even simpler way to manage the documentation services:

### Start Development Server

```bash
make docs-compose-up
```

Or directly:

```bash
docker-compose -f docker-compose.mkdocs.yml up mkdocs
```

### Build Documentation Site

```bash
make docs-compose-build-site
```

Or directly:

```bash
docker-compose -f docker-compose.mkdocs.yml run --rm mkdocs-build
```

### Build with PDF

```bash
make docs-compose-build-pdf
```

Or directly:

```bash
docker-compose -f docker-compose.mkdocs.yml run --rm mkdocs-build-pdf
```

### Stop Services

```bash
make docs-compose-down
```

Or directly:

```bash
docker-compose -f docker-compose.mkdocs.yml down
```

## Direct Docker Commands

If you prefer to use Docker directly without Make or Docker Compose:

### Build Image

```bash
docker build -f Dockerfile.mkdocs -t testcase-manager-docs:latest .
```

### Serve Documentation

```bash
docker run --rm -p 8000:8000 \
  -v $(pwd)/docs:/docs/docs \
  -v $(pwd)/mkdocs.yml:/docs/mkdocs.yml \
  testcase-manager-docs:latest \
  mkdocs serve -a 0.0.0.0:8000
```

### Build Site

```bash
docker run --rm -v $(pwd)/site:/docs/site \
  testcase-manager-docs:latest
```

### Build with PDF

```bash
docker run --rm -e ENABLE_PDF_EXPORT=1 \
  -v $(pwd)/site:/docs/site \
  testcase-manager-docs:latest
```

## Docker Image Details

The MkDocs Docker image:

- Based on `python:3.11-slim` for minimal size
- Includes all dependencies from `requirements.txt`:
  - MkDocs with Material theme
  - PDF export plugin
  - Markdown extensions
- Exposes port 8000 for the development server
- Uses `/docs` as the working directory

## Advantages of Docker Setup

### Consistency
- Same environment across all platforms (Linux, macOS, Windows)
- No conflicts with local Python installations
- Reproducible builds in CI/CD pipelines

### Isolation
- Documentation dependencies don't interfere with other Python projects
- Clean separation from the main Rust project

### Portability
- Easy to share documentation build environment
- Quick setup for new contributors
- Works in containerized CI/CD environments

## Troubleshooting

### Port Already in Use

If port 8000 is already in use, specify a different port:

```bash
docker run --rm -p 8080:8000 \
  -v $(pwd)/docs:/docs/docs \
  -v $(pwd)/mkdocs.yml:/docs/mkdocs.yml \
  testcase-manager-docs:latest \
  mkdocs serve -a 0.0.0.0:8000
```

Then access the documentation at [http://localhost:8080](http://localhost:8080).

### Permission Issues

If you encounter permission issues with the generated `site/` directory:

```bash
# Fix ownership (Linux/macOS)
sudo chown -R $USER:$USER site/
```

### Image Build Failures

If the image fails to build:

1. Ensure Docker is running
2. Check your internet connection (required to download dependencies)
3. Try cleaning Docker cache:
   ```bash
   docker system prune -a
   ```

### Live Reload Not Working

If changes aren't being detected:

1. Ensure files are being modified in the correct `docs/` directory
2. Check that volume mounts are correct
3. Restart the Docker container

## Comparison with Local Setup

| Feature | Docker Setup | Local Setup |
|---------|--------------|-------------|
| Setup Time | Initial image build (~2-3 minutes) | Virtual env setup (~1-2 minutes) |
| Isolation | Full isolation | Python virtualenv only |
| Consistency | Guaranteed across platforms | Depends on Python version |
| Disk Space | ~500MB (image + layers) | ~50MB (virtualenv) |
| Performance | Slightly slower (volumes) | Native performance |
| Updates | Rebuild image | Pip update |

## Integration with CI/CD

The Docker setup is ideal for CI/CD pipelines:

```yaml
# Example GitLab CI/CD job
build-docs:
  stage: build
  image: docker:latest
  services:
    - docker:dind
  script:
    - make docs-docker-build
    - make docs-docker-build-pdf
  artifacts:
    paths:
      - site/
```

## Cleaning Up

Remove the MkDocs Docker image:

```bash
make docs-docker-clean
```

Remove all generated documentation:

```bash
make docs-clean
```

## Related Documentation

- [MkDocs Guide](MKDOCS_GUIDE.md) - General MkDocs usage guide
- [MkDocs Setup Checklist](MKDOCS_SETUP_CHECKLIST.md) - Setup verification
- Main [Docker Setup](DOCKER.md) - Application Docker setup
