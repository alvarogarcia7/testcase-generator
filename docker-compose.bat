@echo off
REM Docker Compose Helper Script for Windows
REM This script provides convenient commands for managing the Test Case Manager container

setlocal enabledelayedexpansion

if "%1"=="" (
    echo Usage: docker-compose.bat [command]
    echo.
    echo Available commands:
    echo   setup      - Initial setup: copy .env file and create output directory
    echo   build      - Build the Docker image
    echo   start      - Start the container
    echo   stop       - Stop the container
    echo   restart    - Restart the container
    echo   shell      - Open a bash shell in the container
    echo   logs       - View container logs
    echo   down       - Stop and remove the container
    echo   rebuild    - Rebuild and restart the container
    echo   clean      - Remove container and image
    echo   status     - Show container status
    echo   tcm        - Run tcm command (pass additional args)
    echo   validate   - Validate YAML files (pass file path)
    echo   execute    - Execute a test case (pass file path)
    echo   help       - Show this help message
    echo.
    exit /b 0
)

set COMMAND=%1
shift

if "%COMMAND%"=="setup" (
    echo Setting up environment...
    if not exist .env (
        copy .env.docker .env
        echo Created .env file. Please edit it with your configuration.
    ) else (
        echo .env file already exists.
    )
    if not exist output (
        mkdir output
        echo Created output directory.
    ) else (
        echo output directory already exists.
    )
    echo Setup complete!
    exit /b 0
)

if "%COMMAND%"=="build" (
    echo Building Docker image...
    docker-compose build
    exit /b %ERRORLEVEL%
)

if "%COMMAND%"=="start" (
    echo Starting container...
    docker-compose up -d
    exit /b %ERRORLEVEL%
)

if "%COMMAND%"=="stop" (
    echo Stopping container...
    docker-compose stop
    exit /b %ERRORLEVEL%
)

if "%COMMAND%"=="restart" (
    echo Restarting container...
    docker-compose restart
    exit /b %ERRORLEVEL%
)

if "%COMMAND%"=="shell" (
    echo Opening bash shell...
    docker-compose exec testcase-manager bash
    exit /b %ERRORLEVEL%
)

if "%COMMAND%"=="logs" (
    echo Showing logs...
    docker-compose logs -f testcase-manager
    exit /b %ERRORLEVEL%
)

if "%COMMAND%"=="down" (
    echo Stopping and removing container...
    docker-compose down
    exit /b %ERRORLEVEL%
)

if "%COMMAND%"=="rebuild" (
    echo Rebuilding and restarting container...
    docker-compose down
    docker-compose up -d --build --force-recreate
    exit /b %ERRORLEVEL%
)

if "%COMMAND%"=="clean" (
    echo Removing container and image...
    docker-compose down
    docker rmi testcase-manager:latest
    exit /b %ERRORLEVEL%
)

if "%COMMAND%"=="status" (
    echo Container status:
    docker-compose ps
    exit /b %ERRORLEVEL%
)

if "%COMMAND%"=="tcm" (
    set ARGS=
    :parse_args_tcm
    if "%1"=="" goto run_tcm
    set ARGS=!ARGS! %1
    shift
    goto parse_args_tcm
    :run_tcm
    docker-compose exec testcase-manager tcm !ARGS!
    exit /b %ERRORLEVEL%
)

if "%COMMAND%"=="validate" (
    if "%1"=="" (
        echo Error: Please specify a YAML file path
        echo Usage: docker-compose.bat validate [file-path]
        exit /b 1
    )
    docker-compose exec testcase-manager validate-yaml %1 --schema /app/schemas/testcase.schema.json
    exit /b %ERRORLEVEL%
)

if "%COMMAND%"=="execute" (
    if "%1"=="" (
        echo Error: Please specify a test case file path
        echo Usage: docker-compose.bat execute [file-path]
        exit /b 1
    )
    docker-compose exec testcase-manager test-executor execute %1
    exit /b %ERRORLEVEL%
)

if "%COMMAND%"=="help" (
    echo Docker Compose Helper for Test Case Manager
    echo.
    echo Usage: docker-compose.bat [command] [args]
    echo.
    echo Setup and Management:
    echo   setup      - Initial setup: copy .env file and create output directory
    echo   build      - Build the Docker image
    echo   start      - Start the container in detached mode
    echo   stop       - Stop the running container
    echo   restart    - Restart the container
    echo   shell      - Open an interactive bash shell in the container
    echo   logs       - View and follow container logs (Ctrl+C to exit)
    echo   down       - Stop and remove the container
    echo   rebuild    - Rebuild the image and restart the container
    echo   clean      - Remove container and image (full cleanup)
    echo   status     - Show current container status
    echo.
    echo Tool Commands:
    echo   tcm [args]        - Run tcm command with optional arguments
    echo                       Example: docker-compose.bat tcm --help
    echo   validate [file]   - Validate a YAML file against schema
    echo                       Example: docker-compose.bat validate /app/testcases/test.yml
    echo   execute [file]    - Execute a test case
    echo                       Example: docker-compose.bat execute /app/testcases/test.yml
    echo   help              - Show this help message
    echo.
    echo Examples:
    echo   docker-compose.bat setup
    echo   docker-compose.bat build
    echo   docker-compose.bat start
    echo   docker-compose.bat shell
    echo   docker-compose.bat tcm create --output /app/testcases/new_test.yml
    echo   docker-compose.bat validate /app/testcases/example.yml
    echo   docker-compose.bat logs
    echo   docker-compose.bat down
    echo.
    exit /b 0
)

echo Unknown command: %COMMAND%
echo Run 'docker-compose.bat help' for usage information
exit /b 1
