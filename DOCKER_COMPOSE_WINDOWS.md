# Docker Compose for Windows

This guide explains how to run the Test Case Manager using Docker Compose on Windows.

## Prerequisites

### Required Software

1. **Docker Desktop for Windows**
   - Download from: https://www.docker.com/products/docker-desktop
   - Ensure Docker Desktop is running before executing commands
   - Docker Desktop includes Docker Compose

2. **Git for Windows** (Optional, for version control)
   - Download from: https://git-scm.com/download/win

### System Requirements

- Windows 10/11 (64-bit)
- WSL 2 backend enabled in Docker Desktop (recommended)
- At least 4GB RAM available for Docker
- 10GB free disk space

## Quick Start

### 1. Navigate to the Project Directory

Open PowerShell or Command Prompt and navigate to the project root:

```powershell
cd C:\path\to\testcase-manager
```

### 2. Configure Environment Variables

Copy the example environment file:

```powershell
# PowerShell
Copy-Item .env.docker .env

# Command Prompt
copy .env.docker .env
```

Edit `.env` with your preferred text editor and customize the values:

```powershell
notepad .env
```

Required changes:
- `GIT_AUTHOR_NAME` - Your name
- `GIT_AUTHOR_EMAIL` - Your email address

### 3. Build and Start the Container

Build the Docker image and start the container:

```powershell
docker-compose up -d --build
```

This command:
- Builds the Docker image from the Dockerfile
- Creates and starts the container in detached mode
- Mounts all necessary volumes from your Windows filesystem

### 4. Access the Container

Enter the running container:

```powershell
docker-compose exec testcase-manager bash
```

Once inside, you can use all the Test Case Manager tools:

```bash
# Run the main tool
tcm --help

# List available binaries
ls -la /usr/local/bin/

# View the README
cat /root/README.md
```

### 5. Stop the Container

When you're done:

```powershell
# Stop the container
docker-compose stop

# Stop and remove the container
docker-compose down
```

## Volume Mounts

The `docker-compose.yml` file mounts the following directories from your Windows filesystem into the container:

| Windows Path | Container Path | Access | Purpose |
|--------------|----------------|---------|---------|
| `.\testcases` | `/app/testcases` | Read/Write | Your test case YAML files |
| `.\data` | `/app/data` | Read/Write | Test data and configurations |
| `.\schemas` | `/app/schemas` | Read-Only | JSON schemas for validation |
| `.\scripts` | `/app/scripts` | Read-Only | Shell scripts and utilities |
| `.\examples` | `/app/examples` | Read-Only | Example test cases |
| `.\output` | `/app/output` | Read/Write | Generated outputs and reports |

**Note**: All file changes made inside the container are immediately reflected on your Windows filesystem and vice versa.

## Common Operations

### Create Output Directory

Before running commands that generate output, create the output directory:

```powershell
# PowerShell
New-Item -ItemType Directory -Force -Path output

# Command Prompt
mkdir output
```

### Run Commands Without Entering the Container

Execute commands directly:

```powershell
# Generate a test script
docker-compose exec testcase-manager test-executor generate /app/testcases/example.yml --output /app/output/test.sh

# Validate a YAML file
docker-compose exec testcase-manager validate-yaml /app/testcases/example.yml --schema /app/schemas/testcase.schema.json

# Run the test case manager interactively
docker-compose exec testcase-manager tcm create --output /app/testcases/new_test.yml
```

### Watch Mode for YAML Validation

To continuously monitor YAML files for changes:

```powershell
docker-compose exec testcase-manager validate-yaml /app/testcases/*.yml --schema /app/schemas/testcase.schema.json --watch
```

Press `Ctrl+C` to stop watching.

### View Container Logs

```powershell
docker-compose logs -f testcase-manager
```

### Restart the Container

```powershell
docker-compose restart testcase-manager
```

### Rebuild After Code Changes

If you modify the source code:

```powershell
docker-compose up -d --build --force-recreate
```

## Working with Git Inside the Container

The container is configured to use your git credentials from the `.env` file. To initialize a git repository for your test cases:

```bash
# Inside the container
cd /app/testcases
git init
git add .
git commit -m "Initial commit"
```

All git operations will use the credentials specified in your `.env` file.

## Advanced Configuration

### Custom Resource Limits

Uncomment and adjust the resource limits in `docker-compose.yml`:

```yaml
deploy:
  resources:
    limits:
      cpus: '2'
      memory: 2G
    reservations:
      cpus: '1'
      memory: 512M
```

### Multiple Containers

You can run multiple instances by duplicating the service and changing the container name:

```yaml
services:
  testcase-manager-dev:
    # ... same configuration ...
    container_name: testcase-manager-dev
    
  testcase-manager-prod:
    # ... same configuration ...
    container_name: testcase-manager-prod
    volumes:
      - ./testcases-prod:/app/testcases
```

### Windows Path Formats

Docker Compose on Windows automatically handles path conversions. You can use:

- Relative paths: `./testcases`
- Windows absolute paths: `C:/Users/YourName/testcases`
- UNC paths: `//server/share/testcases`

**Important**: Use forward slashes (`/`) even in Windows paths within Docker Compose.

## Troubleshooting

### Docker Desktop Not Running

**Error**: `error during connect: This error may indicate that the docker daemon is not running`

**Solution**: 
1. Start Docker Desktop from the Start menu
2. Wait for Docker to fully initialize (check system tray icon)
3. Retry your command

### Volume Mount Issues

**Error**: Files created in container are not visible on Windows

**Solution**:
1. Ensure Docker Desktop has file sharing enabled for your drive
2. Open Docker Desktop → Settings → Resources → File Sharing
3. Add the drive/directory containing your project
4. Click "Apply & Restart"

### Permission Denied Errors

**Error**: `Permission denied` when accessing files

**Solution**:
1. Run PowerShell or Command Prompt as Administrator
2. Or, ensure your Windows user has write permissions to the project directory

### Build Failures

**Error**: Build fails with network errors

**Solution**:
1. Check your internet connection
2. Try building without cache: `docker-compose build --no-cache`
3. Ensure Docker has sufficient disk space

### Container Won't Start

**Error**: Container exits immediately

**Solution**:
1. Check logs: `docker-compose logs testcase-manager`
2. Verify `.env` file is properly formatted
3. Ensure all required directories exist
4. Try recreating: `docker-compose down && docker-compose up -d`

### WSL 2 Backend Issues

**Error**: Performance issues or networking problems

**Solution**:
1. Ensure WSL 2 is installed and enabled
2. In Docker Desktop → Settings → General
3. Check "Use the WSL 2 based engine"
4. Restart Docker Desktop

## Performance Tips

### Use WSL 2 Backend

For better performance, use WSL 2 instead of Hyper-V:
1. Install WSL 2: `wsl --install`
2. Enable in Docker Desktop settings
3. Store project files in WSL filesystem for maximum speed

### Optimize Volume Mounts

For better performance with large codebases:
1. Keep frequently accessed files on WSL filesystem
2. Use named volumes for cache directories
3. Consider using `.dockerignore` to exclude unnecessary files

### Resource Allocation

Adjust Docker Desktop resource limits:
1. Docker Desktop → Settings → Resources
2. Increase CPUs and Memory as needed
3. Click "Apply & Restart"

## Integration with Windows Tools

### VS Code Integration

1. Install the Docker extension for VS Code
2. Install the Remote - Containers extension
3. Open the project in VS Code
4. Click "Reopen in Container" when prompted

### Git Bash Integration

Use Git Bash for a Unix-like experience:

```bash
# In Git Bash
docker-compose exec testcase-manager bash
```

### PowerShell Scripts

Create convenient PowerShell aliases:

```powershell
# Add to your PowerShell profile
function TCM { docker-compose exec testcase-manager tcm $args }
function TestExecutor { docker-compose exec testcase-manager test-executor $args }
function ValidateYAML { docker-compose exec testcase-manager validate-yaml $args }

# Usage
TCM --help
TestExecutor generate example.yml --output test.sh
ValidateYAML testcases/*.yml --schema schemas/testcase.schema.json
```

## Next Steps

After successfully running the container:

1. **Read the in-container documentation**:
   ```powershell
   docker-compose exec testcase-manager cat /root/README.md
   ```

2. **Try the example test cases**:
   ```bash
   cd /app/testcases/examples
   ls -la
   ```

3. **Create your first test case**:
   ```bash
   tcm create-interactive --path /app/testcases
   ```

4. **Generate and execute a test**:
   ```bash
   test-executor generate /app/testcases/your_test.yml --output /app/output/test.sh
   test-executor execute /app/testcases/your_test.yml
   ```

5. **Explore all available tools**:
   ```bash
   ls -la /usr/local/bin/
   tcm --help
   test-executor --help
   test-verify --help
   ```

## Additional Resources

- **Project README**: [README.md](README.md)
- **Docker Build Instructions**: [DOCKER_BUILD_INSTRUCTIONS.md](DOCKER_BUILD_INSTRUCTIONS.md)
- **Test Case Manager Documentation**: See `/root/README.md` inside the container
- **Variable Passing Documentation**: [docs/VARIABLE_PASSING.md](docs/VARIABLE_PASSING.md)
- **BDD Initial Conditions**: [docs/BDD_INITIAL_CONDITIONS.md](docs/BDD_INITIAL_CONDITIONS.md)

## Support

For issues or questions:
1. Check the troubleshooting section above
2. Review Docker Desktop logs
3. Consult the project documentation
4. Check container logs: `docker-compose logs -f`
