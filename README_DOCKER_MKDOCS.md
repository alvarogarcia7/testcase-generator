# Docker Setup for MkDocs Documentation

Quick reference guide for building and serving MkDocs documentation using Docker.

## Prerequisites

- Docker installed and running
- (Optional) Docker Compose for simplified commands

## Quick Start

### Option 1: Helper Script (Recommended)

**Build and serve:**
```bash
./scripts/docker-mkdocs.sh build
./scripts/docker-mkdocs.sh serve
```

**Build static site:**
```bash
./scripts/docker-mkdocs.sh build-site
```

**Build with PDF:**
```bash
./scripts/docker-mkdocs.sh build-pdf
```

### Option 2: Docker Compose

**Start development server:**
```bash
make docs-compose-up
```

**Build documentation:**
```bash
make docs-compose-build-site
```

**Build with PDF:**
```bash
make docs-compose-build-pdf
```

### Option 3: Standard Docker

**Build image:**
```bash
make docs-docker-build
```

**Serve documentation:**
```bash
make docs-docker-serve
```

**Build site:**
```bash
make docs-docker-build-site
```

**Build with PDF:**
```bash
make docs-docker-build-pdf
```

## All Available Commands

### Docker Compose Commands
- `make docs-compose-up` - Start development server with live reload
- `make docs-compose-build-site` - Build static HTML documentation
- `make docs-compose-build-pdf` - Build documentation with PDF export
- `make docs-compose-down` - Stop all services

### Standard Docker Commands
- `make docs-docker-build` - Build the MkDocs Docker image
- `make docs-docker-serve` - Serve documentation with live reload
- `make docs-docker-build-site` - Build static HTML documentation
- `make docs-docker-build-pdf` - Build documentation with PDF export
- `make docs-docker-clean` - Remove the MkDocs Docker image

## Access

After starting the development server, access the documentation at:
- **Local URL:** http://localhost:8000

## Output

- **HTML Site:** Generated in `site/` directory
- **PDF:** Available at `site/pdf/testcase-manager-documentation.pdf` (when using PDF build)

## Advantages

- ✅ No local Python installation required
- ✅ Consistent environment across all platforms
- ✅ Isolated from other Python projects
- ✅ Works on Linux, macOS, and Windows
- ✅ Perfect for CI/CD pipelines

## Files

- `Dockerfile.mkdocs` - Docker image definition
- `docker-compose.mkdocs.yml` - Docker Compose configuration
- `requirements.txt` - Python dependencies
- `mkdocs.yml` - MkDocs configuration

## Detailed Documentation

For complete documentation including troubleshooting and advanced usage, see:
- [docs/DOCKER_MKDOCS.md](docs/DOCKER_MKDOCS.md)

## Local Setup Alternative

If you prefer local setup without Docker:
```bash
make docs-install
make docs-serve
```

See [AGENTS.md](AGENTS.md) for all documentation commands.
