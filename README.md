# MkDocs Docker Setup

A minimal, standalone MkDocs documentation setup with Docker support, designed to be easily integrated into any project.

## Purpose

This repository provides a complete Docker-based MkDocs environment that can be:
- Included as a Git submodule in your project
- Copied directly into a subdirectory
- Used as a standalone documentation repository

The setup includes:
- Docker and Docker Compose configurations
- MkDocs with Material theme
- PDF export capability
- All necessary Python dependencies
- Ready-to-use Makefile commands

## Prerequisites

- **Docker** (20.10 or later)
- **Docker Compose** (v2.0 or later, optional but recommended)

No Python installation required when using Docker.

## Quick Start

### 1. Build the Docker Image

```bash
make docs-docker-build
```

### 2. Serve Documentation Locally

```bash
make docs-docker-serve
```

Access at: http://localhost:8000

### 3. Build Static Site

```bash
make docs-docker-build-site
```

Output: `site/` directory

### 4. Build with PDF Export

```bash
make docs-docker-build-pdf
```

PDF output: `site/pdf/testcase-manager-documentation.pdf`

## Available Makefile Commands

### Docker Commands

| Command | Description |
|---------|-------------|
| `make docs-docker-build` | Build the MkDocs Docker image |
| `make docs-docker-serve` | Start development server with live reload |
| `make docs-docker-build-site` | Build static HTML site |
| `make docs-docker-build-pdf` | Build site with PDF export |
| `make docs-docker-clean` | Remove Docker image |

### Docker Compose Commands

| Command | Description |
|---------|-------------|
| `make docs-compose-up` | Start development server (detached) |
| `make docs-compose-build-site` | Build static HTML site |
| `make docs-compose-build-pdf` | Build site with PDF export |
| `make docs-compose-down` | Stop all services |

## Integration Instructions

### Option 1: Git Submodule

Add this repository as a submodule in your project:

```bash
# Add submodule
git submodule add https://github.com/your-org/mkdocs-docker-setup.git docs-setup

# Initialize and update
git submodule update --init --recursive

# Use the setup
cd docs-setup
make docs-docker-build
make docs-docker-serve
```

### Option 2: Subdirectory Copy

Copy the essential files to your project:

```bash
# Copy files to your project
cp -r mkdocs-docker-setup/Dockerfile.mkdocs your-project/
cp -r mkdocs-docker-setup/docker-compose.mkdocs.yml your-project/
cp -r mkdocs-docker-setup/requirements.txt your-project/
cp -r mkdocs-docker-setup/mkdocs.yml your-project/
cp -r mkdocs-docker-setup/docs your-project/

# Add Makefile targets (append to your existing Makefile)
cat mkdocs-docker-setup/Makefile >> your-project/Makefile
```

### Option 3: Direct Copy

Clone and customize for your specific needs:

```bash
# Clone the repository
git clone https://github.com/your-org/mkdocs-docker-setup.git my-docs
cd my-docs

# Customize mkdocs.yml for your project
vim mkdocs.yml

# Build and serve
make docs-docker-build
make docs-docker-serve
```

## Configuration

### Customize for Your Project

Edit `mkdocs.yml` to configure:

- **Site information**: `site_name`, `site_description`, `site_author`
- **Repository URL**: `repo_url`
- **Navigation**: `nav` section
- **Theme settings**: colors, fonts, features
- **PDF settings**: title, author, output path

Example minimal configuration:

```yaml
site_name: My Project Documentation
site_description: Documentation for my awesome project
repo_url: https://github.com/my-org/my-project

theme:
  name: material
  palette:
    - scheme: default
      primary: indigo
      accent: blue

nav:
  - Home: index.md
  - Getting Started: getting-started.md
  - API Reference: api.md
```

### Add Documentation Content

1. Place Markdown files in the `docs/` directory
2. Update navigation in `mkdocs.yml`
3. Rebuild to see changes

```bash
# Create a new doc page
echo "# Getting Started" > docs/getting-started.md

# Add to mkdocs.yml nav section
# Then rebuild
make docs-docker-build-site
```

## File Structure

```
.
├── Dockerfile.mkdocs           # Docker image definition
├── docker-compose.mkdocs.yml   # Docker Compose configuration
├── requirements.txt             # Python dependencies (MkDocs + plugins)
├── mkdocs.yml                   # MkDocs configuration
├── Makefile                     # Build commands
├── docs/                        # Documentation content (Markdown files)
└── site/                        # Generated output (created after build)
```

## PDF Generation

PDF export is enabled via the `ENABLE_PDF_EXPORT` environment variable:

```bash
# Enable PDF in Docker Compose
make docs-compose-build-pdf

# Enable PDF in standard Docker
make docs-docker-build-pdf

# Manually enable PDF
docker run --rm -e ENABLE_PDF_EXPORT=1 -v "$(pwd)/site:/docs/site" testcase-manager-docs:latest
```

PDF will be generated at: `site/pdf/<your-pdf-name>.pdf`

Configure PDF settings in `mkdocs.yml`:

```yaml
plugins:
  - with-pdf:
      enabled_if_env: ENABLE_PDF_EXPORT
      output_path: pdf/my-documentation.pdf
      cover_title: My Documentation
      cover_subtitle: Comprehensive Guide
```

## Parent Repository Integration

### Makefile Integration

Add these targets to your parent project's Makefile:

```makefile
# Documentation targets
docs-build:
	cd docs-setup && make docs-docker-build

docs-serve:
	cd docs-setup && make docs-docker-serve

docs-build-site:
	cd docs-setup && make docs-docker-build-site

docs-build-pdf:
	cd docs-setup && make docs-docker-build-pdf

docs-clean:
	cd docs-setup && make docs-docker-clean
	rm -rf docs-setup/site

.PHONY: docs-build docs-serve docs-build-site docs-build-pdf docs-clean
```

### CI/CD Integration

Example GitHub Actions workflow:

```yaml
name: Build Documentation

on:
  push:
    branches: [ main ]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Build Docker image
        run: make docs-docker-build
      
      - name: Build documentation with PDF
        run: make docs-docker-build-pdf
      
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./site
```

Example GitLab CI:

```yaml
docs:
  image: docker:latest
  services:
    - docker:dind
  script:
    - make docs-docker-build
    - make docs-docker-build-pdf
  artifacts:
    paths:
      - site/
  only:
    - main
```

## Advantages of Docker Setup

- ✅ **No local Python installation required**
- ✅ **Consistent environment** across all platforms (Linux, macOS, Windows)
- ✅ **Isolated dependencies** - won't conflict with other Python projects
- ✅ **Perfect for CI/CD** - reproducible builds
- ✅ **Easy version management** - pin versions in Dockerfile
- ✅ **Portable** - works anywhere Docker runs

## Troubleshooting

### Port Already in Use

If port 8000 is already in use:

```bash
# Stop conflicting service or use a different port
docker run --rm -p 8080:8000 -v "$(pwd)/docs:/docs/docs" -v "$(pwd)/mkdocs.yml:/docs/mkdocs.yml" testcase-manager-docs:latest mkdocs serve -a 0.0.0.0:8000
```

Access at: http://localhost:8080

### Permission Issues

If you encounter permission issues with generated files:

```bash
# Fix ownership (Linux/macOS)
sudo chown -R $USER:$USER site/

# Or rebuild with current user
docker run --rm --user $(id -u):$(id -g) -v "$(pwd)/site:/docs/site" testcase-manager-docs:latest
```

### Changes Not Reflecting

1. Rebuild the Docker image if you modified `requirements.txt` or `Dockerfile.mkdocs`:
   ```bash
   make docs-docker-clean
   make docs-docker-build
   ```

2. For content changes, just refresh the browser (live reload enabled in serve mode)

3. Clear the site directory if experiencing caching issues:
   ```bash
   rm -rf site/
   ```

## Dependencies

All dependencies are specified in `requirements.txt`:

- `mkdocs>=1.5.0` - Main documentation generator
- `mkdocs-material>=9.5.0` - Material theme
- `mkdocs-with-pdf>=0.9.3` - PDF export plugin
- `markdown>=3.5` - Markdown processor
- `pymdown-extensions>=10.7` - Additional Markdown extensions

## Support

For issues, questions, or contributions:
- Open an issue on the repository
- Check existing documentation in `docs/`
- Review MkDocs documentation: https://www.mkdocs.org/
- Material theme docs: https://squidfunk.github.io/mkdocs-material/

## License

See LICENSE file for details.
