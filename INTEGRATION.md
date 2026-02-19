# Integration Guide

How to integrate this MkDocs setup into your project.

## Integration Methods

| Method | Best For |
|--------|----------|
| **Git Submodule** | Keeping setup separate and updatable |
| **Directory Copy** | Full control and customization |

## Method 1: Git Submodule

```bash
# Add submodule
git submodule add <repo-url> docs-setup
cd docs-setup
git checkout main

# Initialize
git submodule update --init --recursive
```

Add to parent Makefile:

```makefile
docs-build:
	cd docs-setup && make docs-docker-build

docs-serve:
	cd docs-setup && make docs-docker-serve
```

## Method 2: Directory Copy

```bash
# Copy files
mkdir docs-setup
cp -r <source>/Dockerfile.mkdocs docs-setup/
cp -r <source>/requirements.txt docs-setup/
cp -r <source>/mkdocs.yml docs-setup/
cp -r <source>/Makefile docs-setup/
cp -r <source>/docs docs-setup/
```

## Configuration

Edit `mkdocs.yml` for your project:

```yaml
site_name: Your Project
site_description: Your description
nav:
  - Home: index.md
  - Guide: guide.md
```

## Usage

```bash
# Build
make docs-build

# Serve locally
make docs-serve

# Build static site
make docs-build-site
```
