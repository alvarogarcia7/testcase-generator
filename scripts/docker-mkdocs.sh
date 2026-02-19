#!/usr/bin/env bash
set -e

# Get script directory and source logger
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

# Script to help manage Docker-based MkDocs documentation builds
# This provides a convenient wrapper around Docker and Docker Compose commands

DOCS_IMAGE="testcase-manager-docs:latest"
COMPOSE_FILE="docker-compose.mkdocs.yml"

show_usage() {
    cat << EOF
Docker MkDocs Documentation Manager

Usage: $0 <command> [options]

Commands:
  build           Build the MkDocs Docker image
  serve           Start documentation server with live reload
  build-site      Build static HTML documentation
  build-pdf       Build documentation with PDF export
  clean           Remove Docker image and generated files
  status          Show Docker image and container status
  help            Show this help message

Docker Compose Commands:
  compose-up      Start development server using Docker Compose
  compose-build   Build static site using Docker Compose
  compose-pdf     Build with PDF using Docker Compose
  compose-down    Stop Docker Compose services

Options:
  -p, --port      Port for development server (default: 8000)
  -v, --verbose   Enable verbose output
  -h, --help      Show this help message

Examples:
  $0 build                    # Build the Docker image
  $0 serve                    # Start development server
  $0 serve --port 8080        # Start server on port 8080
  $0 build-site              # Build static documentation
  $0 build-pdf               # Build with PDF export
  $0 compose-up              # Use Docker Compose

EOF
}

check_docker() {
    if ! command -v docker &> /dev/null; then
        log_error "Docker is not installed or not in PATH"
        log_info "Please install Docker from https://www.docker.com/get-started"
        exit 1
    fi

    if ! docker info &> /dev/null; then
        log_error "Docker is not running"
        log_info "Please start Docker and try again"
        exit 1
    fi

    log_verbose "Docker is installed and running"
}

check_docker_compose() {
    if ! command -v docker-compose &> /dev/null; then
        log_warning "Docker Compose is not installed"
        log_info "Install Docker Compose from https://docs.docker.com/compose/install/"
        return 1
    fi
    log_verbose "Docker Compose is available"
    return 0
}

build_image() {
    section "Building MkDocs Docker Image"
    
    log_info "Building Docker image: $DOCS_IMAGE"
    if docker build -f Dockerfile.mkdocs -t "$DOCS_IMAGE" .; then
        pass "Docker image built successfully"
        log_info "Image: $DOCS_IMAGE"
    else
        fail "Failed to build Docker image"
        exit 1
    fi
}

serve_docs() {
    local port="${1:-8000}"
    
    section "Starting MkDocs Development Server"
    
    log_info "Starting server on port $port"
    log_info "Documentation will be available at: http://localhost:$port"
    log_info "Press Ctrl+C to stop the server"
    
    docker run --rm \
        -p "$port:8000" \
        -v "$(pwd)/docs:/docs/docs" \
        -v "$(pwd)/mkdocs.yml:/docs/mkdocs.yml" \
        "$DOCS_IMAGE" \
        mkdocs serve -a 0.0.0.0:8000
}

build_site() {
    section "Building Static Documentation Site"
    
    log_info "Building documentation site..."
    if docker run --rm -v "$(pwd)/site:/docs/site" "$DOCS_IMAGE"; then
        pass "Documentation site built successfully"
        log_info "Output directory: site/"
    else
        fail "Failed to build documentation site"
        exit 1
    fi
}

build_pdf() {
    section "Building Documentation with PDF Export"
    
    log_info "Building documentation with PDF..."
    if docker run --rm \
        -e ENABLE_PDF_EXPORT=1 \
        -v "$(pwd)/site:/docs/site" \
        "$DOCS_IMAGE"; then
        pass "Documentation with PDF built successfully"
        log_info "Output directory: site/"
        log_info "PDF location: site/pdf/testcase-manager-documentation.pdf"
    else
        fail "Failed to build documentation with PDF"
        exit 1
    fi
}

clean_docker() {
    section "Cleaning Docker Resources"
    
    log_info "Removing Docker image: $DOCS_IMAGE"
    if docker rmi "$DOCS_IMAGE" 2>/dev/null; then
        pass "Docker image removed"
    else
        log_warning "Docker image not found or already removed"
    fi
    
    if [ -d "site" ]; then
        log_info "Removing generated site directory"
        rm -rf site/
        pass "Site directory removed"
    fi
}

show_status() {
    section "Docker Status"
    
    log_info "Checking Docker image status..."
    if docker images "$DOCS_IMAGE" --format "table {{.Repository}}\t{{.Tag}}\t{{.Size}}\t{{.CreatedAt}}" | grep -v REPOSITORY; then
        pass "Docker image exists"
    else
        log_warning "Docker image not found"
        log_info "Run '$0 build' to create the image"
    fi
    
    echo ""
    log_info "Checking for running containers..."
    if docker ps --filter "ancestor=$DOCS_IMAGE" --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}" | grep -v NAMES; then
        pass "Container is running"
    else
        log_info "No containers running"
    fi
}

compose_up() {
    section "Starting Docker Compose Services"
    
    if ! check_docker_compose; then
        log_error "Docker Compose is required for this command"
        exit 1
    fi
    
    log_info "Starting development server..."
    docker-compose -f "$COMPOSE_FILE" up mkdocs
}

compose_build() {
    section "Building Site with Docker Compose"
    
    if ! check_docker_compose; then
        log_error "Docker Compose is required for this command"
        exit 1
    fi
    
    log_info "Building documentation site..."
    if docker-compose -f "$COMPOSE_FILE" run --rm mkdocs-build; then
        pass "Documentation site built successfully"
    else
        fail "Failed to build documentation site"
        exit 1
    fi
}

compose_pdf() {
    section "Building PDF with Docker Compose"
    
    if ! check_docker_compose; then
        log_error "Docker Compose is required for this command"
        exit 1
    fi
    
    log_info "Building documentation with PDF..."
    if docker-compose -f "$COMPOSE_FILE" run --rm mkdocs-build-pdf; then
        pass "Documentation with PDF built successfully"
    else
        fail "Failed to build documentation with PDF"
        exit 1
    fi
}

compose_down() {
    section "Stopping Docker Compose Services"
    
    if ! check_docker_compose; then
        log_error "Docker Compose is required for this command"
        exit 1
    fi
    
    log_info "Stopping services..."
    if docker-compose -f "$COMPOSE_FILE" down; then
        pass "Services stopped successfully"
    else
        fail "Failed to stop services"
        exit 1
    fi
}

# Parse arguments
PORT=8000
VERBOSE=0

while [[ $# -gt 0 ]]; do
    case $1 in
        -v|--verbose)
            VERBOSE=1
            export VERBOSE
            shift
            ;;
        -p|--port)
            PORT="$2"
            shift 2
            ;;
        -h|--help|help)
            show_usage
            exit 0
            ;;
        build)
            check_docker
            build_image
            exit 0
            ;;
        serve)
            check_docker
            shift
            # Check for port option after serve command
            if [[ $1 == "-p" ]] || [[ $1 == "--port" ]]; then
                PORT="$2"
                shift 2
            fi
            serve_docs "$PORT"
            exit 0
            ;;
        build-site)
            check_docker
            build_site
            exit 0
            ;;
        build-pdf)
            check_docker
            build_pdf
            exit 0
            ;;
        clean)
            check_docker
            clean_docker
            exit 0
            ;;
        status)
            check_docker
            show_status
            exit 0
            ;;
        compose-up)
            check_docker
            compose_up
            exit 0
            ;;
        compose-build)
            check_docker
            compose_build
            exit 0
            ;;
        compose-pdf)
            check_docker
            compose_pdf
            exit 0
            ;;
        compose-down)
            check_docker
            compose_down
            exit 0
            ;;
        *)
            log_error "Unknown command: $1"
            echo ""
            show_usage
            exit 1
            ;;
    esac
done

# If no command provided, show usage
show_usage
exit 0
