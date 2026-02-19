# MkDocs Docker Setup

Standalone MkDocs documentation with Docker support.

## Quick Start

### Build and Serve

```bash
# Build Docker image
make docs-docker-build

# Serve locally
make docs-docker-serve
```

Access at: http://localhost:8000

### Build Static Site

```bash
# Build HTML
make docs-docker-build-site

# Build with PDF
make docs-docker-build-pdf
```

## Available Commands

| Command | Description |
|---------|-------------|
| `make docs-docker-build` | Build Docker image |
| `make docs-docker-serve` | Start dev server with live reload |
| `make docs-docker-build-site` | Build static HTML |
| `make docs-docker-build-pdf` | Build with PDF export |
| `make docs-docker-clean` | Remove Docker image |

## Configuration

Edit `mkdocs.yml` to customize:
- Site information
- Navigation structure
- Theme settings
- PDF options

## File Structure

```
.
├── Dockerfile.mkdocs
├── docker-compose.mkdocs.yml
├── requirements.txt
├── mkdocs.yml
├── Makefile
├── docs/           # Markdown content
└── site/           # Generated output
```

## Dependencies

- Docker 20.10+
- Docker Compose v2.0+ (optional)

No Python installation required.
