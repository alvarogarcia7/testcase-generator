# Docker MkDocs Quick Reference

Quick command reference for building and serving MkDocs documentation using Docker.

## Prerequisites

```bash
# Check Docker is installed
docker --version

# Check Docker is running
docker info
```

## Helper Script Commands

```bash
# Build Docker image
./scripts/docker-mkdocs.sh build

# Serve documentation (http://localhost:8000)
./scripts/docker-mkdocs.sh serve

# Serve on custom port
./scripts/docker-mkdocs.sh serve --port 8080

# Build static site
./scripts/docker-mkdocs.sh build-site

# Build with PDF
./scripts/docker-mkdocs.sh build-pdf

# Check status
./scripts/docker-mkdocs.sh status

# Clean up
./scripts/docker-mkdocs.sh clean

# Show help
./scripts/docker-mkdocs.sh help
```

## Make Commands

### Standard Docker

```bash
make docs-docker-build          # Build image
make docs-docker-serve          # Serve at http://localhost:8000
make docs-docker-build-site     # Build static site
make docs-docker-build-pdf      # Build with PDF
make docs-docker-clean          # Remove image
```

### Docker Compose

```bash
make docs-compose-up            # Start server
make docs-compose-build-site    # Build site
make docs-compose-build-pdf     # Build with PDF
make docs-compose-down          # Stop services
```

## Direct Docker Commands

```bash
# Build image
docker build -f Dockerfile.mkdocs -t testcase-manager-docs:latest .

# Serve documentation
docker run --rm -p 8000:8000 \
  -v $(pwd)/docs:/docs/docs \
  -v $(pwd)/mkdocs.yml:/docs/mkdocs.yml \
  testcase-manager-docs:latest \
  mkdocs serve -a 0.0.0.0:8000

# Build site
docker run --rm \
  -v $(pwd)/site:/docs/site \
  testcase-manager-docs:latest

# Build with PDF
docker run --rm \
  -e ENABLE_PDF_EXPORT=1 \
  -v $(pwd)/site:/docs/site \
  testcase-manager-docs:latest

# List images
docker images testcase-manager-docs

# Remove image
docker rmi testcase-manager-docs:latest
```

## Docker Compose Commands

```bash
# Start development server
docker-compose -f docker-compose.mkdocs.yml up mkdocs

# Build site
docker-compose -f docker-compose.mkdocs.yml run --rm mkdocs-build

# Build with PDF
docker-compose -f docker-compose.mkdocs.yml run --rm mkdocs-build-pdf

# Stop services
docker-compose -f docker-compose.mkdocs.yml down
```

## Output Locations

```bash
site/                                           # Generated HTML site
site/pdf/testcase-manager-documentation.pdf    # PDF documentation (if enabled)
```

## Troubleshooting

```bash
# Check if image exists
docker images testcase-manager-docs

# Check if containers are running
docker ps --filter "ancestor=testcase-manager-docs"

# View container logs
docker logs <container-id>

# Remove all stopped containers
docker container prune

# Remove dangling images
docker image prune

# Full cleanup (careful!)
docker system prune -a
```

## Port Conflicts

If port 8000 is in use:

```bash
# Using helper script
./scripts/docker-mkdocs.sh serve --port 8080

# Using Docker directly
docker run --rm -p 8080:8000 \
  -v $(pwd)/docs:/docs/docs \
  -v $(pwd)/mkdocs.yml:/docs/mkdocs.yml \
  testcase-manager-docs:latest \
  mkdocs serve -a 0.0.0.0:8000
```

Then access at http://localhost:8080

## Comparison with Local Setup

| Feature | Docker | Local |
|---------|--------|-------|
| Setup | Build image once | Install virtualenv |
| Speed | Slightly slower | Native speed |
| Isolation | Full | virtualenv only |
| Consistency | 100% | Depends on Python |
| Disk space | ~500MB | ~50MB |

## When to Use Docker

✅ Use Docker when:
- Working on multiple machines
- Contributing to the project
- CI/CD pipelines
- Avoiding Python version conflicts
- Wanting guaranteed reproducibility

❌ Use local setup when:
- Frequent documentation updates
- Network/bandwidth limited
- Disk space constrained
- Familiar with Python virtualenv

## Related Documentation

- **Full Guide:** [DOCKER_MKDOCS.md](DOCKER_MKDOCS.md)
- **Local Setup:** [MKDOCS_GUIDE.md](MKDOCS_GUIDE.md)
- **All Commands:** [AGENTS.md](../AGENTS.md)
