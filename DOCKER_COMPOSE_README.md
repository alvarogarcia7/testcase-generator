# Docker Compose Setup

This project includes Docker Compose configuration for running the Test Case Manager in a containerized environment with volume mounts for easy file access.

## Quick Links

- **Windows Users**: [QUICK_START_WINDOWS.md](QUICK_START_WINDOWS.md) - 5-minute setup guide
- **Windows Complete Guide**: [DOCKER_COMPOSE_WINDOWS.md](DOCKER_COMPOSE_WINDOWS.md) - Comprehensive documentation
- **Docker Build Instructions**: [DOCKER_BUILD_INSTRUCTIONS.md](DOCKER_BUILD_INSTRUCTIONS.md) - Manual Docker setup

## What's Included

### Configuration Files

- **docker-compose.yml** - Main Docker Compose configuration
- **.env.docker** - Environment variable template
- **docker-compose.ps1** - PowerShell helper script for Windows
- **docker-compose.bat** - Batch helper script for Windows

### Features

- ✅ Pre-configured volume mounts for testcases, data, schemas, scripts, and examples
- ✅ Automatic file synchronization between host and container
- ✅ Git integration with configurable credentials
- ✅ Helper scripts for common operations
- ✅ Interactive terminal support
- ✅ Environment variable management
- ✅ Windows-optimized paths and scripts

## Supported Platforms

- **Windows 10/11** - Primary target with helper scripts
- **Linux** - Fully supported
- **macOS** - Fully supported

## Quick Start

### Windows

```powershell
# PowerShell
.\docker-compose.ps1 setup
.\docker-compose.ps1 build
.\docker-compose.ps1 start
.\docker-compose.ps1 shell
```

```cmd
REM Command Prompt
docker-compose.bat setup
docker-compose.bat build
docker-compose.bat start
docker-compose.bat shell
```

### Linux/macOS

```bash
# Copy environment file
cp .env.docker .env

# Edit with your configuration
nano .env

# Create output directory
mkdir -p output

# Build and start
docker-compose up -d --build

# Access container
docker-compose exec testcase-manager bash
```

## Volume Mounts

The following directories are mounted from your host machine:

| Host Path | Container Path | Access | Description |
|-----------|----------------|--------|-------------|
| `./testcases` | `/app/testcases` | RW | Test case YAML files |
| `./data` | `/app/data` | RW | Test data and configurations |
| `./schemas` | `/app/schemas` | RO | JSON schemas |
| `./scripts` | `/app/scripts` | RO | Shell scripts |
| `./examples` | `/app/examples` | RO | Example test cases |
| `./output` | `/app/output` | RW | Generated outputs |

**RW** = Read/Write, **RO** = Read-Only

## Environment Variables

Configure these in your `.env` file:

```bash
# Git Configuration
GIT_AUTHOR_NAME=Your Name
GIT_AUTHOR_EMAIL=your.email@example.com

# Editor
EDITOR=vim

# Logging
RUST_LOG=info
VERBOSE=0
```

## Available Commands

### Container Management

```bash
docker-compose up -d        # Start container
docker-compose down         # Stop and remove container
docker-compose stop         # Stop container
docker-compose start        # Start existing container
docker-compose restart      # Restart container
docker-compose logs -f      # View logs
docker-compose ps           # Show status
```

### Accessing the Container

```bash
docker-compose exec testcase-manager bash
```

### Running Commands

```bash
# Inside container
tcm --help
test-executor execute /app/testcases/example.yml
validate-yaml /app/testcases/*.yml --schema /app/schemas/testcase.schema.json

# From host
docker-compose exec testcase-manager tcm --help
docker-compose exec testcase-manager test-executor execute /app/testcases/example.yml
```

## Helper Scripts

### PowerShell (docker-compose.ps1)

```powershell
.\docker-compose.ps1 setup      # Initial setup
.\docker-compose.ps1 build      # Build image
.\docker-compose.ps1 start      # Start container
.\docker-compose.ps1 shell      # Open shell
.\docker-compose.ps1 logs       # View logs
.\docker-compose.ps1 stop       # Stop container
.\docker-compose.ps1 down       # Remove container
.\docker-compose.ps1 rebuild    # Rebuild and restart
.\docker-compose.ps1 clean      # Remove everything
.\docker-compose.ps1 status     # Show status
.\docker-compose.ps1 tcm [args] # Run tcm command
.\docker-compose.ps1 validate [file]  # Validate YAML
.\docker-compose.ps1 execute [file]   # Execute test
.\docker-compose.ps1 help       # Show help
```

### Batch (docker-compose.bat)

```cmd
docker-compose.bat setup      REM Initial setup
docker-compose.bat build      REM Build image
docker-compose.bat start      REM Start container
docker-compose.bat shell      REM Open shell
docker-compose.bat logs       REM View logs
docker-compose.bat stop       REM Stop container
docker-compose.bat down       REM Remove container
docker-compose.bat rebuild    REM Rebuild and restart
docker-compose.bat clean      REM Remove everything
docker-compose.bat status     REM Show status
docker-compose.bat tcm [args] REM Run tcm command
docker-compose.bat validate [file]  REM Validate YAML
docker-compose.bat execute [file]   REM Execute test
docker-compose.bat help       REM Show help
```

## Workflow Example

1. **Setup**
   ```bash
   docker-compose up -d --build
   ```

2. **Create test case**
   ```bash
   docker-compose exec testcase-manager tcm create-interactive --path /app/testcases
   ```

3. **Validate**
   ```bash
   docker-compose exec testcase-manager validate-yaml /app/testcases/my_test.yml --schema /app/schemas/testcase.schema.json
   ```

4. **Generate script**
   ```bash
   docker-compose exec testcase-manager test-executor generate /app/testcases/my_test.yml --output /app/output/test.sh
   ```

5. **Execute test**
   ```bash
   docker-compose exec testcase-manager test-executor execute /app/testcases/my_test.yml
   ```

6. **View results**
   - Files appear immediately in your host directories
   - Check `./output/` for generated scripts
   - Check `./testcases/` for test case files

## Troubleshooting

### Docker Not Running

Ensure Docker Desktop is running:
- **Windows**: Check system tray for Docker icon
- **Linux**: `systemctl status docker`
- **macOS**: Check menu bar for Docker icon

### Permission Issues

**Windows**: Run PowerShell/CMD as Administrator

**Linux/macOS**: 
```bash
sudo docker-compose up -d
# Or add user to docker group
sudo usermod -aG docker $USER
```

### Volume Mount Issues

**Windows**: Ensure file sharing is enabled in Docker Desktop settings

**Linux/macOS**: Check directory permissions

### Build Failures

```bash
# Clean everything
docker-compose down
docker system prune -a

# Rebuild without cache
docker-compose build --no-cache
docker-compose up -d
```

## Documentation

- **Windows Quick Start**: [QUICK_START_WINDOWS.md](QUICK_START_WINDOWS.md)
- **Windows Complete Guide**: [DOCKER_COMPOSE_WINDOWS.md](DOCKER_COMPOSE_WINDOWS.md)
- **Docker Build Guide**: [DOCKER_BUILD_INSTRUCTIONS.md](DOCKER_BUILD_INSTRUCTIONS.md)
- **Project README**: [README.md](README.md)

## Additional Resources

- [Variable Passing Documentation](docs/VARIABLE_PASSING.md)
- [BDD Initial Conditions](docs/BDD_INITIAL_CONDITIONS.md)
- [Test Verification Guide](docs/TEST_VERIFY_USAGE.md)

## Support

For issues or questions:
1. Check the troubleshooting section
2. Review the complete documentation
3. Check Docker logs: `docker-compose logs -f`
4. Verify configuration: `docker-compose config`

---

**Ready to start?** Follow the [Quick Start Guide for Windows](QUICK_START_WINDOWS.md) or dive into the [complete documentation](DOCKER_COMPOSE_WINDOWS.md).
