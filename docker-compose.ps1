#!/usr/bin/env pwsh
# Docker Compose Helper Script for Windows PowerShell
# This script provides convenient commands for managing the Test Case Manager container

param(
    [Parameter(Position=0)]
    [string]$Command,
    
    [Parameter(Position=1, ValueFromRemainingArguments=$true)]
    [string[]]$Arguments
)

function Show-Usage {
    Write-Host "Usage: .\docker-compose.ps1 [command] [arguments]" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Available commands:" -ForegroundColor Yellow
    Write-Host "  setup      - Initial setup: copy .env file and create output directory"
    Write-Host "  build      - Build the Docker image"
    Write-Host "  start      - Start the container"
    Write-Host "  stop       - Stop the container"
    Write-Host "  restart    - Restart the container"
    Write-Host "  shell      - Open a bash shell in the container"
    Write-Host "  logs       - View container logs"
    Write-Host "  down       - Stop and remove the container"
    Write-Host "  rebuild    - Rebuild and restart the container"
    Write-Host "  clean      - Remove container and image"
    Write-Host "  status     - Show container status"
    Write-Host "  tcm        - Run tcm command (pass additional args)"
    Write-Host "  validate   - Validate YAML files (pass file path)"
    Write-Host "  execute    - Execute a test case (pass file path)"
    Write-Host "  help       - Show this help message"
    Write-Host ""
}

function Invoke-Setup {
    Write-Host "Setting up environment..." -ForegroundColor Cyan
    
    if (-not (Test-Path .env)) {
        Copy-Item .env.docker .env
        Write-Host "Created .env file. Please edit it with your configuration." -ForegroundColor Green
    } else {
        Write-Host ".env file already exists." -ForegroundColor Yellow
    }
    
    if (-not (Test-Path output)) {
        New-Item -ItemType Directory -Name output | Out-Null
        Write-Host "Created output directory." -ForegroundColor Green
    } else {
        Write-Host "output directory already exists." -ForegroundColor Yellow
    }
    
    Write-Host "Setup complete!" -ForegroundColor Green
}

function Invoke-Build {
    Write-Host "Building Docker image..." -ForegroundColor Cyan
    docker-compose build
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Build completed successfully!" -ForegroundColor Green
    } else {
        Write-Host "Build failed!" -ForegroundColor Red
    }
}

function Invoke-Start {
    Write-Host "Starting container..." -ForegroundColor Cyan
    docker-compose up -d
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Container started successfully!" -ForegroundColor Green
    } else {
        Write-Host "Failed to start container!" -ForegroundColor Red
    }
}

function Invoke-Stop {
    Write-Host "Stopping container..." -ForegroundColor Cyan
    docker-compose stop
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Container stopped successfully!" -ForegroundColor Green
    } else {
        Write-Host "Failed to stop container!" -ForegroundColor Red
    }
}

function Invoke-Restart {
    Write-Host "Restarting container..." -ForegroundColor Cyan
    docker-compose restart
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Container restarted successfully!" -ForegroundColor Green
    } else {
        Write-Host "Failed to restart container!" -ForegroundColor Red
    }
}

function Invoke-Shell {
    Write-Host "Opening bash shell..." -ForegroundColor Cyan
    docker-compose exec testcase-manager bash
}

function Invoke-Logs {
    Write-Host "Showing logs (Ctrl+C to exit)..." -ForegroundColor Cyan
    docker-compose logs -f testcase-manager
}

function Invoke-Down {
    Write-Host "Stopping and removing container..." -ForegroundColor Cyan
    docker-compose down
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Container removed successfully!" -ForegroundColor Green
    } else {
        Write-Host "Failed to remove container!" -ForegroundColor Red
    }
}

function Invoke-Rebuild {
    Write-Host "Rebuilding and restarting container..." -ForegroundColor Cyan
    docker-compose down
    docker-compose up -d --build --force-recreate
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Container rebuilt and started successfully!" -ForegroundColor Green
    } else {
        Write-Host "Failed to rebuild container!" -ForegroundColor Red
    }
}

function Invoke-Clean {
    Write-Host "Removing container and image..." -ForegroundColor Cyan
    docker-compose down
    docker rmi testcase-manager:latest
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Cleanup completed successfully!" -ForegroundColor Green
    } else {
        Write-Host "Cleanup completed with warnings" -ForegroundColor Yellow
    }
}

function Invoke-Status {
    Write-Host "Container status:" -ForegroundColor Cyan
    docker-compose ps
}

function Invoke-TCM {
    param([string[]]$Args)
    $argsString = $Args -join ' '
    docker-compose exec testcase-manager tcm $argsString
}

function Invoke-Validate {
    param([string]$FilePath)
    
    if ([string]::IsNullOrEmpty($FilePath)) {
        Write-Host "Error: Please specify a YAML file path" -ForegroundColor Red
        Write-Host "Usage: .\docker-compose.ps1 validate [file-path]" -ForegroundColor Yellow
        exit 1
    }
    
    docker-compose exec testcase-manager validate-yaml $FilePath --schema /app/schemas/testcase.schema.json
}

function Invoke-Execute {
    param([string]$FilePath)
    
    if ([string]::IsNullOrEmpty($FilePath)) {
        Write-Host "Error: Please specify a test case file path" -ForegroundColor Red
        Write-Host "Usage: .\docker-compose.ps1 execute [file-path]" -ForegroundColor Yellow
        exit 1
    }
    
    docker-compose exec testcase-manager test-executor execute $FilePath
}

function Show-Help {
    Write-Host "Docker Compose Helper for Test Case Manager" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Usage: .\docker-compose.ps1 [command] [args]" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Setup and Management:" -ForegroundColor Yellow
    Write-Host "  setup      - Initial setup: copy .env file and create output directory"
    Write-Host "  build      - Build the Docker image"
    Write-Host "  start      - Start the container in detached mode"
    Write-Host "  stop       - Stop the running container"
    Write-Host "  restart    - Restart the container"
    Write-Host "  shell      - Open an interactive bash shell in the container"
    Write-Host "  logs       - View and follow container logs (Ctrl+C to exit)"
    Write-Host "  down       - Stop and remove the container"
    Write-Host "  rebuild    - Rebuild the image and restart the container"
    Write-Host "  clean      - Remove container and image (full cleanup)"
    Write-Host "  status     - Show current container status"
    Write-Host ""
    Write-Host "Tool Commands:" -ForegroundColor Yellow
    Write-Host "  tcm [args]        - Run tcm command with optional arguments"
    Write-Host "                      Example: .\docker-compose.ps1 tcm --help"
    Write-Host "  validate [file]   - Validate a YAML file against schema"
    Write-Host "                      Example: .\docker-compose.ps1 validate /app/testcases/test.yml"
    Write-Host "  execute [file]    - Execute a test case"
    Write-Host "                      Example: .\docker-compose.ps1 execute /app/testcases/test.yml"
    Write-Host "  help              - Show this help message"
    Write-Host ""
    Write-Host "Examples:" -ForegroundColor Yellow
    Write-Host "  .\docker-compose.ps1 setup"
    Write-Host "  .\docker-compose.ps1 build"
    Write-Host "  .\docker-compose.ps1 start"
    Write-Host "  .\docker-compose.ps1 shell"
    Write-Host "  .\docker-compose.ps1 tcm create --output /app/testcases/new_test.yml"
    Write-Host "  .\docker-compose.ps1 validate /app/testcases/example.yml"
    Write-Host "  .\docker-compose.ps1 logs"
    Write-Host "  .\docker-compose.ps1 down"
    Write-Host ""
}

# Main command dispatcher
if ([string]::IsNullOrEmpty($Command)) {
    Show-Usage
    exit 0
}

switch ($Command.ToLower()) {
    "setup"    { Invoke-Setup }
    "build"    { Invoke-Build }
    "start"    { Invoke-Start }
    "stop"     { Invoke-Stop }
    "restart"  { Invoke-Restart }
    "shell"    { Invoke-Shell }
    "logs"     { Invoke-Logs }
    "down"     { Invoke-Down }
    "rebuild"  { Invoke-Rebuild }
    "clean"    { Invoke-Clean }
    "status"   { Invoke-Status }
    "tcm"      { Invoke-TCM -Args $Arguments }
    "validate" { Invoke-Validate -FilePath $Arguments[0] }
    "execute"  { Invoke-Execute -FilePath $Arguments[0] }
    "help"     { Show-Help }
    default {
        Write-Host "Unknown command: $Command" -ForegroundColor Red
        Write-Host "Run '.\docker-compose.ps1 help' for usage information" -ForegroundColor Yellow
        exit 1
    }
}

exit $LASTEXITCODE
