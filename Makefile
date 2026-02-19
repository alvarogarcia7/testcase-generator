# Docker documentation targets
DOCS_CONTAINER = testcase-manager-docs
DOCS_IMAGE = $(DOCS_CONTAINER):latest
CURRENT_DIR = $(shell pwd)

test:
	${MAKE} docs-docker-clean
	${MAKE} docs-docker-build
	${MAKE} docs-docker-build-site
	${MAKE} docs-docker-build-pdf
#   Docker-compose based
	${MAKE} docs-docker-clean
	${MAKE} docs-compose-build-site
	${MAKE} docs-compose-build-pdf
# Do not test these: they keep the server in watch mode
#	${MAKE} docs-compose-up
#	${MAKE} docs-compose-down
.PHONY: test


docs-docker-build:
	docker build -f Dockerfile.mkdocs -t $(DOCS_IMAGE) .
.PHONY: docs-docker-build

docs-docker-serve:
	docker run --rm -p 8000:8000 -v "$(CURRENT_DIR)/docs:/docs/docs" -v "$(CURRENT_DIR)/mkdocs.yml:/docs/mkdocs.yml" $(DOCS_IMAGE) mkdocs serve -a 0.0.0.0:8000
.PHONY: docs-docker-serve

docs-docker-build-site:
	docker run --rm -v "$(CURRENT_DIR)/site:/docs/site" $(DOCS_IMAGE)
.PHONY: docs-docker-build-site

docs-docker-build-pdf:
	docker run --rm -e ENABLE_PDF_EXPORT=1 -v "$(CURRENT_DIR)/site:/docs/site" $(DOCS_IMAGE)
.PHONY: docs-docker-build-pdf

docs-docker-clean:
	docker rmi $(DOCS_IMAGE) 2>/dev/null || true
.PHONY: docs-docker-clean

# Docker Compose documentation targets
docs-compose-up:
	docker-compose -f docker-compose.mkdocs.yml up mkdocs
.PHONY: docs-compose-up

docs-compose-build-site:
	docker-compose -f docker-compose.mkdocs.yml run --rm mkdocs-build
.PHONY: docs-compose-build-site

docs-compose-build-pdf:
	docker-compose -f docker-compose.mkdocs.yml run --rm mkdocs-build-pdf
.PHONY: docs-compose-build-pdf

docs-compose-down:
	docker-compose -f docker-compose.mkdocs.yml down
.PHONY: docs-compose-down
