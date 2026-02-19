# MkDocs Docker Setup Integration Guide

This guide explains how to include this MkDocs Docker setup in another repository using different integration methods.

## Table of Contents

- [Integration Methods Overview](#integration-methods-overview)
- [Method 1: Git Submodule](#method-1-git-submodule)
- [Method 2: Subdirectory Copy](#method-2-subdirectory-copy)
- [Method 3: Cherry-Picking Files](#method-3-cherry-picking-files)
- [Integrating Makefile Targets](#integrating-makefile-targets)
- [Adjusting Docker Compose Paths](#adjusting-docker-compose-paths)
- [Using .dockerignore.mkdocs](#using-dockerignoremkdocs)
- [Configuration Examples](#configuration-examples)
- [Troubleshooting](#troubleshooting)

---

## Integration Methods Overview

Choose the integration method that best fits your project structure:

| Method | Best For | Pros | Cons |
|--------|----------|------|------|
| **Git Submodule** | Keeping setup separate and updatable | Easy updates, clean separation | Requires submodule management |
| **Subdirectory Copy** | Full control and customization | Simple, no external dependencies | Manual updates required |
| **Cherry-Pick Files** | Minimal integration | Only copy what you need | More manual configuration |

---

## Method 1: Git Submodule

Add this MkDocs setup as a Git submodule to your project. This keeps the setup separate and easily updatable.

### Step 1: Add the Submodule

```bash
# From your project root
git submodule add https://github.com/your-org/mkdocs-docker-setup.git docs-docker
cd docs-docker
git checkout main
```

### Step 2: Initialize and Update

```bash
# First time setup
git submodule update --init --recursive

# Later updates
git submodule update --remote docs-docker
```

### Step 3: Create Parent Makefile Targets

Add to your project's `Makefile`:

```makefile
# Documentation targets using submodule
DOCS_DIR = docs-docker

docs-build:
	cd $(DOCS_DIR) && make docs-docker-build
.PHONY: docs-build

docs-serve:
	cd $(DOCS_DIR) && make docs-docker-serve
.PHONY: docs-serve

docs-build-site:
	cd $(DOCS_DIR) && make docs-docker-build-site
.PHONY: docs-build-site

docs-build-pdf:
	cd $(DOCS_DIR) && make docs-docker-build-pdf
.PHONY: docs-build-pdf

docs-clean:
	cd $(DOCS_DIR) && make docs-docker-clean
	rm -rf $(DOCS_DIR)/site
.PHONY: docs-clean
```

### Step 4: Customize Configuration

Copy and customize the `mkdocs.yml` in the submodule:

```bash
cd docs-docker
# Edit mkdocs.yml for your project
vim mkdocs.yml
# Commit changes in the submodule
git add mkdocs.yml
git commit -m "Customize MkDocs configuration"
```

### Step 5: Usage

```bash
# Build the Docker image
make docs-build

# Serve locally at http://localhost:8000
make docs-serve

# Build static site
make docs-build-site
```

### Advantages

- ✅ Easy to pull updates from the upstream repository
- ✅ Clean separation of documentation infrastructure
- ✅ Can track specific versions via commit hashes
- ✅ Shared improvements across multiple projects

### Considerations

- Requires team members to run `git submodule update --init`
- Need to commit submodule reference changes in parent repo

---

## Method 2: Subdirectory Copy

Copy the entire setup into a subdirectory of your project for full control.

### Step 1: Copy Files

```bash
# From your project root
git clone https://github.com/your-org/mkdocs-docker-setup.git temp-docs
cp -r temp-docs/. ./docs-docker/
rm -rf temp-docs
rm -rf docs-docker/.git
```

Or copy specific directories:

```bash
mkdir -p docs-docker
cp -r mkdocs-docker-setup/Dockerfile.mkdocs docs-docker/
cp -r mkdocs-docker-setup/docker-compose.mkdocs.yml docs-docker/
cp -r mkdocs-docker-setup/requirements.txt docs-docker/
cp -r mkdocs-docker-setup/mkdocs.yml docs-docker/
cp -r mkdocs-docker-setup/.dockerignore.mkdocs docs-docker/
cp -r mkdocs-docker-setup/Makefile docs-docker/
cp -r mkdocs-docker-setup/docs docs-docker/
```

### Step 2: Adjust Paths (if needed)

If you placed the setup in a subdirectory, paths in Docker Compose and Makefile may need adjustment. See [Adjusting Docker Compose Paths](#adjusting-docker-compose-paths).

### Step 3: Integrate Makefile Targets

Add to your project's `Makefile`:

```makefile
# Documentation targets in subdirectory
DOCS_DIR = docs-docker

docs-build:
	cd $(DOCS_DIR) && make docs-docker-build
.PHONY: docs-build

docs-serve:
	cd $(DOCS_DIR) && make docs-docker-serve
.PHONY: docs-serve

docs-build-site:
	cd $(DOCS_DIR) && make docs-docker-build-site
.PHONY: docs-build-site

docs-build-pdf:
	cd $(DOCS_DIR) && make docs-docker-build-pdf
.PHONY: docs-build-pdf

docs-compose-up:
	cd $(DOCS_DIR) && make docs-compose-up
.PHONY: docs-compose-up

docs-compose-down:
	cd $(DOCS_DIR) && make docs-compose-down
.PHONY: docs-compose-down

docs-clean:
	cd $(DOCS_DIR) && make docs-docker-clean
	rm -rf $(DOCS_DIR)/site
.PHONY: docs-clean
```

### Step 4: Customize for Your Project

Edit `docs-docker/mkdocs.yml`:

```yaml
site_name: Your Project Name
site_description: Your project description
site_author: Your Team
repo_url: https://github.com/your-org/your-project

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

### Step 5: Add Documentation Content

```bash
# Create your documentation files
echo "# Welcome" > docs-docker/docs/index.md
echo "# Getting Started" > docs-docker/docs/getting-started.md

# Build and serve
make docs-build
make docs-serve
```

### Advantages

- ✅ Full control over all files
- ✅ Easy to customize without affecting upstream
- ✅ No submodule management required
- ✅ All files tracked in your repository

### Considerations

- Manual effort required to pull updates from upstream
- Larger repository size

---

## Method 3: Cherry-Picking Files

Copy only the essential files needed for your documentation setup.

### Minimum Required Files

At minimum, you need these files:

```
your-project/
├── Dockerfile.mkdocs           # Required
├── requirements.txt             # Required
├── mkdocs.yml                   # Required
├── docs/                        # Required: your documentation
│   └── index.md
└── .dockerignore.mkdocs         # Recommended
```

### Step 1: Copy Essential Files

```bash
# Copy required files
cp mkdocs-docker-setup/Dockerfile.mkdocs ./
cp mkdocs-docker-setup/requirements.txt ./
cp mkdocs-docker-setup/mkdocs.yml ./
cp mkdocs-docker-setup/.dockerignore.mkdocs ./

# Create docs directory if it doesn't exist
mkdir -p docs
```

### Step 2: Optional Files

```bash
# Copy Docker Compose (recommended)
cp mkdocs-docker-setup/docker-compose.mkdocs.yml ./

# Copy Makefile targets (recommended)
# Either copy the entire Makefile or extract relevant targets
cat mkdocs-docker-setup/Makefile >> Makefile
```

### Step 3: Create Build Commands

If not using the Makefile, you can build and run directly:

```bash
# Build the image
docker build -f Dockerfile.mkdocs -t my-project-docs:latest .

# Serve documentation
docker run --rm -p 8000:8000 \
  -v "$(pwd)/docs:/docs/docs" \
  -v "$(pwd)/mkdocs.yml:/docs/mkdocs.yml" \
  my-project-docs:latest mkdocs serve -a 0.0.0.0:8000

# Build static site
docker run --rm \
  -v "$(pwd)/site:/docs/site" \
  my-project-docs:latest mkdocs build

# Build with PDF
docker run --rm \
  -e ENABLE_PDF_EXPORT=1 \
  -v "$(pwd)/site:/docs/site" \
  my-project-docs:latest mkdocs build
```

### Step 4: Add Makefile Targets (Optional)

Create a minimal documentation section in your `Makefile`:

```makefile
# MkDocs documentation targets
DOCS_IMAGE = my-project-docs:latest
CURRENT_DIR = $(shell pwd)

docs-build:
	docker build -f Dockerfile.mkdocs -t $(DOCS_IMAGE) .
.PHONY: docs-build

docs-serve:
	docker run --rm -p 8000:8000 \
		-v "$(CURRENT_DIR)/docs:/docs/docs" \
		-v "$(CURRENT_DIR)/mkdocs.yml:/docs/mkdocs.yml" \
		$(DOCS_IMAGE) mkdocs serve -a 0.0.0.0:8000
.PHONY: docs-serve

docs-build-site:
	docker run --rm \
		-v "$(CURRENT_DIR)/site:/docs/site" \
		$(DOCS_IMAGE) mkdocs build
.PHONY: docs-build-site

docs-build-pdf:
	docker run --rm -e ENABLE_PDF_EXPORT=1 \
		-v "$(CURRENT_DIR)/site:/docs/site" \
		$(DOCS_IMAGE) mkdocs build
.PHONY: docs-build-pdf

docs-clean:
	docker rmi $(DOCS_IMAGE) 2>/dev/null || true
	rm -rf site/
.PHONY: docs-clean
```

### Advantages

- ✅ Minimal footprint in your repository
- ✅ Only include what you need
- ✅ Full customization freedom

### Considerations

- More manual setup required
- Need to manually track updates from upstream

---

## Integrating Makefile Targets

### Direct Integration

If you want documentation targets directly in your main Makefile:

```makefile
# Include at the top of your Makefile
DOCS_CONTAINER = my-project-docs
DOCS_IMAGE = $(DOCS_CONTAINER):latest
CURRENT_DIR = $(shell pwd)

# If docs are in a subdirectory, adjust paths
DOCS_SUBDIR = docs-docker
DOCS_PATH = $(CURRENT_DIR)/$(DOCS_SUBDIR)

# Docker documentation targets
docs-docker-build:
	docker build -f $(DOCS_SUBDIR)/Dockerfile.mkdocs -t $(DOCS_IMAGE) $(DOCS_SUBDIR)
.PHONY: docs-docker-build

docs-docker-serve:
	docker run --rm -p 8000:8000 \
		-v "$(DOCS_PATH)/docs:/docs/docs" \
		-v "$(DOCS_PATH)/mkdocs.yml:/docs/mkdocs.yml" \
		$(DOCS_IMAGE) mkdocs serve -a 0.0.0.0:8000
.PHONY: docs-docker-serve

docs-docker-build-site:
	docker run --rm \
		-v "$(DOCS_PATH)/site:/docs/site" \
		$(DOCS_IMAGE) mkdocs build
.PHONY: docs-docker-build-site

docs-docker-build-pdf:
	docker run --rm -e ENABLE_PDF_EXPORT=1 \
		-v "$(DOCS_PATH)/site:/docs/site" \
		$(DOCS_IMAGE) mkdocs build
.PHONY: docs-docker-build-pdf

docs-docker-clean:
	docker rmi $(DOCS_IMAGE) 2>/dev/null || true
	rm -rf $(DOCS_PATH)/site
.PHONY: docs-docker-clean

# Docker Compose documentation targets
docs-compose-up:
	docker-compose -f $(DOCS_SUBDIR)/docker-compose.mkdocs.yml up mkdocs
.PHONY: docs-compose-up

docs-compose-build-site:
	docker-compose -f $(DOCS_SUBDIR)/docker-compose.mkdocs.yml run --rm mkdocs-build
.PHONY: docs-compose-build-site

docs-compose-build-pdf:
	docker-compose -f $(DOCS_SUBDIR)/docker-compose.mkdocs.yml run --rm mkdocs-build-pdf
.PHONY: docs-compose-build-pdf

docs-compose-down:
	docker-compose -f $(DOCS_SUBDIR)/docker-compose.mkdocs.yml down
.PHONY: docs-compose-down
```

### Wrapper Targets

Create simple wrapper targets that delegate to the subdirectory:

```makefile
# Documentation wrapper targets
docs-build:
	$(MAKE) -C docs-docker docs-docker-build
.PHONY: docs-build

docs-serve:
	$(MAKE) -C docs-docker docs-docker-serve
.PHONY: docs-serve

docs-site:
	$(MAKE) -C docs-docker docs-docker-build-site
.PHONY: docs-site

docs-pdf:
	$(MAKE) -C docs-docker docs-docker-build-pdf
.PHONY: docs-pdf

docs-clean:
	$(MAKE) -C docs-docker docs-docker-clean
.PHONY: docs-clean
```

### Conditional Targets

Include documentation targets only if the directory exists:

```makefile
# Conditional documentation targets
ifneq (,$(wildcard docs-docker/Makefile))
docs-build:
	cd docs-docker && $(MAKE) docs-docker-build
.PHONY: docs-build

docs-serve:
	cd docs-docker && $(MAKE) docs-docker-serve
.PHONY: docs-serve
else
docs-build docs-serve:
	@echo "Documentation setup not found. Run: git submodule update --init"
.PHONY: docs-build docs-serve
endif
```

---

## Adjusting Docker Compose Paths

The `docker-compose.mkdocs.yml` file uses relative paths. When integrating into a parent repository, you may need to adjust these paths.

### Original Paths (Standalone Setup)

```yaml
services:
  mkdocs:
    build:
      context: .
      dockerfile: Dockerfile.mkdocs
    volumes:
      - ./docs:/docs/docs
      - ./mkdocs.yml:/docs/mkdocs.yml
      - ./site:/docs/site
      - ./README.md:/docs/README.md
```

### Scenario 1: Setup in Subdirectory (e.g., `docs-docker/`)

If you placed the setup in `docs-docker/`, paths are already correct **when running docker-compose from within that directory**:

```bash
# Run from docs-docker directory
cd docs-docker
docker-compose -f docker-compose.mkdocs.yml up
```

Or run from parent directory by adjusting the command:

```bash
# Run from project root
docker-compose -f docs-docker/docker-compose.mkdocs.yml up
```

### Scenario 2: Docker Compose at Root, Docs in Subdirectory

If you want `docker-compose.mkdocs.yml` at your project root but docs in a subdirectory:

```yaml
services:
  mkdocs:
    build:
      context: ./docs-docker
      dockerfile: Dockerfile.mkdocs
    volumes:
      - ./docs-docker/docs:/docs/docs
      - ./docs-docker/mkdocs.yml:/docs/mkdocs.yml
      - ./docs-docker/site:/docs/site
      - ./README.md:/docs/README.md  # Use parent README if desired
```

### Scenario 3: Sharing Project README

To include your project's README in the documentation:

```yaml
services:
  mkdocs:
    volumes:
      - ./docs:/docs/docs
      - ./mkdocs.yml:/docs/mkdocs.yml
      - ./site:/docs/site
      - ./README.md:/docs/README.md              # Main README
      - ./docs/additional.md:/docs/docs/additional.md  # Additional files
```

Then reference it in `mkdocs.yml`:

```yaml
nav:
  - Home: index.md
  - Overview: ../README.md  # Include parent README
  - Getting Started: getting-started.md
```

### Scenario 4: Documentation Source in Different Location

If your documentation source is elsewhere (e.g., `documentation/`):

```yaml
services:
  mkdocs:
    build:
      context: .
      dockerfile: Dockerfile.mkdocs
    volumes:
      - ./documentation:/docs/docs          # Changed from ./docs
      - ./mkdocs.yml:/docs/mkdocs.yml
      - ./site:/docs/site
```

### Using Environment Variables

For flexible paths, use environment variables:

```yaml
services:
  mkdocs:
    volumes:
      - ${DOCS_SOURCE_DIR:-./docs}:/docs/docs
      - ${MKDOCS_CONFIG:-./mkdocs.yml}:/docs/mkdocs.yml
      - ${DOCS_OUTPUT_DIR:-./site}:/docs/site
```

Usage:

```bash
# Use defaults
docker-compose -f docker-compose.mkdocs.yml up

# Override paths
DOCS_SOURCE_DIR=./documentation docker-compose -f docker-compose.mkdocs.yml up
```

---

## Using .dockerignore.mkdocs

The `.dockerignore.mkdocs` file specifies what should NOT be copied into the Docker image during build.

### Using with `-f` Flag

Docker's `--dockerignore` flag is not directly supported. Instead, use the `-f` flag with `docker build`:

#### Method 1: Rename to `.dockerignore` (Recommended)

```bash
# Copy or rename the file
cp .dockerignore.mkdocs .dockerignore

# Build normally
docker build -f Dockerfile.mkdocs -t my-docs:latest .
```

#### Method 2: Temporary Symlink

```bash
# Create temporary symlink
ln -sf .dockerignore.mkdocs .dockerignore

# Build
docker build -f Dockerfile.mkdocs -t my-docs:latest .

# Clean up (optional)
rm .dockerignore
```

#### Method 3: Build Script

Create a `build-docs.sh` script:

```bash
#!/bin/bash
set -e

# Backup existing .dockerignore if present
if [ -f .dockerignore ]; then
    mv .dockerignore .dockerignore.backup
fi

# Use MkDocs-specific dockerignore
cp .dockerignore.mkdocs .dockerignore

# Build the image
docker build -f Dockerfile.mkdocs -t my-docs:latest .

# Restore original .dockerignore
if [ -f .dockerignore.backup ]; then
    mv .dockerignore.backup .dockerignore
else
    rm .dockerignore
fi
```

Make it executable:

```bash
chmod +x build-docs.sh
./build-docs.sh
```

### Contents of .dockerignore.mkdocs

The file excludes unnecessary files from the Docker build context:

```dockerignore
# Version control
.git/
.gitignore
.gitattributes

# IDE and editor files
.vscode/
.idea/
*.swp
*.swo
*~

# Documentation build artifacts
site/
mkdocs-venv/

# OS files
.DS_Store
Thumbs.db

# CI/CD
.github/
.gitlab-ci.yml

# Other Docker files
Dockerfile
docker-compose.yml
.dockerignore
```

### When to Customize .dockerignore.mkdocs

Add to `.dockerignore.mkdocs` if your project has:

- **Build outputs**: `dist/`, `build/`, `target/`
- **Dependencies**: `node_modules/`, `vendor/`, `venv/`
- **Test files**: `tests/`, `__pycache__/`, `*.pyc`
- **Large media**: `*.mp4`, `*.zip`, `large-files/`
- **Sensitive data**: `secrets/`, `.env`, `*.key`

Example customized `.dockerignore.mkdocs`:

```dockerignore
# Original excludes
.git/
.gitignore
.vscode/
.idea/
site/
.DS_Store

# Project-specific excludes
node_modules/
venv/
__pycache__/
*.pyc
dist/
build/
.env
.env.local
tests/
coverage/
*.log
```

### Makefile Integration with .dockerignore.mkdocs

Update your Makefile to handle the `.dockerignore` file:

```makefile
docs-docker-build:
	@# Backup existing .dockerignore
	@if [ -f .dockerignore ]; then mv .dockerignore .dockerignore.backup; fi
	@# Use MkDocs-specific dockerignore
	@cp .dockerignore.mkdocs .dockerignore
	@# Build the image
	docker build -f Dockerfile.mkdocs -t $(DOCS_IMAGE) .
	@# Restore original .dockerignore
	@if [ -f .dockerignore.backup ]; then mv .dockerignore.backup .dockerignore; else rm .dockerignore; fi
.PHONY: docs-docker-build
```

Or create a separate target:

```makefile
.PHONY: docs-docker-build
docs-docker-build: docs-setup-dockerignore
	docker build -f Dockerfile.mkdocs -t $(DOCS_IMAGE) .
	$(MAKE) docs-restore-dockerignore

.PHONY: docs-setup-dockerignore
docs-setup-dockerignore:
	@if [ -f .dockerignore ]; then mv .dockerignore .dockerignore.backup; fi
	@cp .dockerignore.mkdocs .dockerignore

.PHONY: docs-restore-dockerignore
docs-restore-dockerignore:
	@if [ -f .dockerignore.backup ]; then \
		mv .dockerignore.backup .dockerignore; \
	else \
		rm -f .dockerignore; \
	fi
```

---

## Configuration Examples

### Example 1: Complete Integration in Subdirectory

Project structure:

```
my-project/
├── Makefile                    # Main project Makefile
├── README.md
├── src/                        # Your source code
├── docs-setup/                 # MkDocs Docker setup (submodule or copy)
│   ├── Dockerfile.mkdocs
│   ├── docker-compose.mkdocs.yml
│   ├── requirements.txt
│   ├── mkdocs.yml
│   ├── .dockerignore.mkdocs
│   ├── Makefile
│   └── docs/
│       └── index.md
└── .gitignore
```

Main `Makefile`:

```makefile
# Documentation targets
DOCS_DIR = docs-setup

.PHONY: docs-build
docs-build:
	cd $(DOCS_DIR) && $(MAKE) docs-docker-build

.PHONY: docs-serve
docs-serve:
	cd $(DOCS_DIR) && $(MAKE) docs-docker-serve

.PHONY: docs-site
docs-site:
	cd $(DOCS_DIR) && $(MAKE) docs-docker-build-site

.PHONY: docs-pdf
docs-pdf:
	cd $(DOCS_DIR) && $(MAKE) docs-docker-build-pdf

.PHONY: docs-clean
docs-clean:
	cd $(DOCS_DIR) && $(MAKE) docs-docker-clean
```

### Example 2: Flat Integration at Root

Project structure:

```
my-project/
├── Makefile
├── README.md
├── Dockerfile.mkdocs           # At root
├── docker-compose.mkdocs.yml   # At root
├── requirements.txt            # At root
├── mkdocs.yml                  # At root
├── .dockerignore.mkdocs        # At root
├── src/
└── docs/
    └── index.md
```

`Makefile` (integrated):

```makefile
# MkDocs variables
DOCS_IMAGE = my-project-docs:latest
CURRENT_DIR = $(shell pwd)

# Build targets
build:
	# Your build commands
	go build -o bin/app ./src

# Documentation targets
.PHONY: docs-build
docs-build:
	@if [ -f .dockerignore ]; then mv .dockerignore .dockerignore.backup; fi
	@cp .dockerignore.mkdocs .dockerignore
	docker build -f Dockerfile.mkdocs -t $(DOCS_IMAGE) .
	@if [ -f .dockerignore.backup ]; then mv .dockerignore.backup .dockerignore; else rm .dockerignore; fi

.PHONY: docs-serve
docs-serve:
	docker run --rm -p 8000:8000 \
		-v "$(CURRENT_DIR)/docs:/docs/docs" \
		-v "$(CURRENT_DIR)/mkdocs.yml:/docs/mkdocs.yml" \
		$(DOCS_IMAGE) mkdocs serve -a 0.0.0.0:8000

.PHONY: docs-site
docs-site:
	docker run --rm -v "$(CURRENT_DIR)/site:/docs/site" $(DOCS_IMAGE) mkdocs build

.PHONY: docs-pdf
docs-pdf:
	docker run --rm -e ENABLE_PDF_EXPORT=1 -v "$(CURRENT_DIR)/site:/docs/site" $(DOCS_IMAGE) mkdocs build

.PHONY: docs-clean
docs-clean:
	docker rmi $(DOCS_IMAGE) 2>/dev/null || true
	rm -rf site/
```

### Example 3: Using Git Submodule

Project structure:

```
my-project/
├── Makefile
├── README.md
├── src/
├── docs/                       # Your documentation content
│   └── index.md
├── .gitmodules
└── mkdocs-setup/               # Git submodule
    ├── Dockerfile.mkdocs
    ├── docker-compose.mkdocs.yml
    ├── requirements.txt
    ├── mkdocs.yml              # Customize this
    ├── .dockerignore.mkdocs
    └── Makefile
```

`.gitmodules`:

```ini
[submodule "mkdocs-setup"]
	path = mkdocs-setup
	url = https://github.com/your-org/mkdocs-docker-setup.git
```

`Makefile`:

```makefile
.PHONY: docs-init
docs-init:
	git submodule update --init --recursive
	@echo "Customize mkdocs-setup/mkdocs.yml for your project"

.PHONY: docs-update
docs-update:
	git submodule update --remote mkdocs-setup

.PHONY: docs-build
docs-build:
	cd mkdocs-setup && $(MAKE) docs-docker-build

.PHONY: docs-serve
docs-serve:
	cd mkdocs-setup && $(MAKE) docs-docker-serve
```

Customize `mkdocs-setup/mkdocs.yml` to point to your docs:

```yaml
# Edit mkdocs-setup/mkdocs.yml
site_name: My Project

# Adjust paths to use parent directory docs
# Note: Docker volumes will need adjustment in docker-compose
```

Adjust `mkdocs-setup/docker-compose.mkdocs.yml`:

```yaml
services:
  mkdocs:
    volumes:
      - ../docs:/docs/docs          # Parent directory docs
      - ./mkdocs.yml:/docs/mkdocs.yml
      - ../site:/docs/site          # Output to parent
      - ../README.md:/docs/README.md
```

---

## Troubleshooting

### Issue: Docker Build Context Too Large

**Symptom**: `docker build` is slow or uploads many unnecessary files.

**Solution**: Ensure `.dockerignore.mkdocs` is properly used:

```bash
# Verify which files are included in context
docker build -f Dockerfile.mkdocs --no-cache -t test . 2>&1 | head -n 20

# Use .dockerignore.mkdocs
cp .dockerignore.mkdocs .dockerignore
docker build -f Dockerfile.mkdocs -t my-docs .
```

Add more exclusions to `.dockerignore.mkdocs`:

```dockerignore
# Large directories
node_modules/
vendor/
.git/
build/
dist/
target/

# Media files
*.mp4
*.avi
*.mov
*.zip
*.tar.gz
```

### Issue: Paths Not Found in Container

**Symptom**: MkDocs can't find documentation files.

**Solution**: Check volume mounts in `docker-compose.mkdocs.yml` or Docker run commands:

```bash
# Debug: List files in container
docker run --rm -v "$(pwd)/docs:/docs/docs" my-docs ls -la /docs/docs

# Verify mount paths match your structure
docker run --rm -v "$(pwd)/documentation:/docs/docs" my-docs mkdocs build
```

### Issue: Permission Denied on Generated Files

**Symptom**: Cannot edit or delete files in `site/` directory.

**Solution**: Fix ownership or run container with your user ID:

```bash
# Fix ownership
sudo chown -R $USER:$USER site/

# Or run with current user
docker run --rm --user $(id -u):$(id -g) \
  -v "$(pwd)/site:/docs/site" \
  my-docs mkdocs build
```

Update Makefile to use current user:

```makefile
docs-build-site:
	docker run --rm --user $(shell id -u):$(shell id -g) \
		-v "$(CURRENT_DIR)/site:/docs/site" \
		$(DOCS_IMAGE) mkdocs build
.PHONY: docs-build-site
```

### Issue: Submodule Not Found

**Symptom**: Git submodule directory is empty.

**Solution**: Initialize submodules:

```bash
# Initialize all submodules
git submodule update --init --recursive

# Initialize specific submodule
git submodule update --init mkdocs-setup
```

Add to your project's README:

```markdown
## Setup

Clone with submodules:
\`\`\`bash
git clone --recurse-submodules https://github.com/your-org/your-project.git
\`\`\`

Or initialize after cloning:
\`\`\`bash
git clone https://github.com/your-org/your-project.git
cd your-project
git submodule update --init --recursive
\`\`\`
```

### Issue: Port 8000 Already in Use

**Symptom**: Cannot start documentation server.

**Solution**: Use a different port:

```bash
# Change port mapping
docker run --rm -p 8080:8000 \
  -v "$(pwd)/docs:/docs/docs" \
  -v "$(pwd)/mkdocs.yml:/docs/mkdocs.yml" \
  my-docs mkdocs serve -a 0.0.0.0:8000

# Access at http://localhost:8080
```

Or update `docker-compose.mkdocs.yml`:

```yaml
services:
  mkdocs:
    ports:
      - "8080:8000"  # Changed from 8000:8000
```

### Issue: PDF Generation Fails

**Symptom**: PDF is not generated or build fails.

**Solution**: Ensure `ENABLE_PDF_EXPORT=1` is set:

```bash
# Check if environment variable is set
docker run --rm \
  -e ENABLE_PDF_EXPORT=1 \
  -v "$(pwd)/site:/docs/site" \
  my-docs mkdocs build

# Verify PDF is generated
ls -la site/pdf/
```

Check `mkdocs.yml` has PDF plugin configured:

```yaml
plugins:
  - with-pdf:
      enabled_if_env: ENABLE_PDF_EXPORT
      output_path: pdf/documentation.pdf
```

### Issue: Changes Not Reflected in Served Docs

**Symptom**: Documentation doesn't update after editing files.

**Solution**: 

1. Ensure files are mounted correctly:
   ```bash
   # Volume must include the edited files
   docker run --rm -p 8000:8000 \
     -v "$(pwd)/docs:/docs/docs" \
     -v "$(pwd)/mkdocs.yml:/docs/mkdocs.yml" \
     my-docs mkdocs serve -a 0.0.0.0:8000
   ```

2. If you changed `requirements.txt` or `Dockerfile.mkdocs`, rebuild:
   ```bash
   make docs-docker-clean
   make docs-docker-build
   ```

3. For MkDocs config changes, restart the container:
   ```bash
   docker-compose -f docker-compose.mkdocs.yml down
   docker-compose -f docker-compose.mkdocs.yml up
   ```

---

## Summary

This integration guide covered:

- ✅ Three integration methods: Git submodule, subdirectory copy, cherry-picking
- ✅ Makefile integration patterns for parent repositories
- ✅ Docker Compose path adjustments for different project structures
- ✅ Using `.dockerignore.mkdocs` with the `-f` flag workaround
- ✅ Real-world configuration examples
- ✅ Common troubleshooting scenarios

Choose the method that best fits your project structure and team workflow. For most teams, using a **Git submodule** or **subdirectory copy** with integrated Makefile targets provides the best balance of convenience and maintainability.
