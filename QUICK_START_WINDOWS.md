# Quick Start Guide for Windows

Get up and running with Test Case Manager on Windows in 5 minutes using Docker Compose.

## Prerequisites

1. **Install Docker Desktop for Windows**
   - Download: https://www.docker.com/products/docker-desktop
   - Install and start Docker Desktop
   - Ensure it's running (check system tray)

2. **Open PowerShell or Command Prompt**
   - Press `Win + X` and select "Windows PowerShell" or "Command Prompt"
   - Or search for "PowerShell" in Start menu

## 5-Minute Setup

### Step 1: Navigate to Project Directory

```powershell
cd C:\path\to\testcase-manager
```

### Step 2: Initial Setup

**Option A: Using PowerShell Script (Recommended)**

```powershell
.\docker-compose.ps1 setup
```

**Option B: Using Batch Script**

```cmd
docker-compose.bat setup
```

**Option C: Manual Setup**

```powershell
# Copy environment file
Copy-Item .env.docker .env

# Create output directory
New-Item -ItemType Directory -Force -Path output

# Edit .env file with your details
notepad .env
```

### Step 3: Build and Start

**PowerShell:**
```powershell
.\docker-compose.ps1 build
.\docker-compose.ps1 start
```

**Command Prompt:**
```cmd
docker-compose.bat build
docker-compose.bat start
```

**Direct Docker Compose:**
```powershell
docker-compose up -d --build
```

### Step 4: Access the Container

**PowerShell:**
```powershell
.\docker-compose.ps1 shell
```

**Command Prompt:**
```cmd
docker-compose.bat shell
```

**Direct Docker Compose:**
```powershell
docker-compose exec testcase-manager bash
```

### Step 5: Start Using the Tools

Once inside the container:

```bash
# View help
tcm --help

# Read the README
cat /root/README.md

# List all available tools
ls -la /usr/local/bin/

# Create a new test case
tcm create-interactive --path /app/testcases

# Validate a YAML file
validate-yaml /app/testcases/example.yml --schema /app/schemas/testcase.schema.json

# Generate a test script
test-executor generate /app/testcases/example.yml --output /app/output/test.sh

# Execute a test case
test-executor execute /app/testcases/example.yml
```

## Common Commands

### Using Helper Scripts

**PowerShell:**

```powershell
# Run tcm with arguments
.\docker-compose.ps1 tcm --help
.\docker-compose.ps1 tcm create --output /app/testcases/new_test.yml

# Validate a YAML file
.\docker-compose.ps1 validate /app/testcases/example.yml

# Execute a test case
.\docker-compose.ps1 execute /app/testcases/example.yml

# View logs
.\docker-compose.ps1 logs

# Stop container
.\docker-compose.ps1 stop

# Restart container
.\docker-compose.ps1 restart

# Remove everything
.\docker-compose.ps1 down
```

**Command Prompt:**

```cmd
REM Run tcm with arguments
docker-compose.bat tcm --help
docker-compose.bat tcm create --output /app/testcases/new_test.yml

REM Validate a YAML file
docker-compose.bat validate /app/testcases/example.yml

REM Execute a test case
docker-compose.bat execute /app/testcases/example.yml

REM View logs
docker-compose.bat logs

REM Stop container
docker-compose.bat stop

REM Restart container
docker-compose.bat restart

REM Remove everything
docker-compose.bat down
```

### Using Docker Compose Directly

```powershell
# Run commands inside container
docker-compose exec testcase-manager tcm --help
docker-compose exec testcase-manager validate-yaml /app/testcases/example.yml --schema /app/schemas/testcase.schema.json
docker-compose exec testcase-manager test-executor execute /app/testcases/example.yml

# View logs
docker-compose logs -f testcase-manager

# Stop container
docker-compose stop

# Start container
docker-compose start

# Restart container
docker-compose restart

# Stop and remove
docker-compose down

# Rebuild
docker-compose up -d --build --force-recreate
```

## File Locations

All files are automatically synced between Windows and the container:

| Purpose | Windows Path | Container Path |
|---------|--------------|----------------|
| Your test cases | `.\testcases\` | `/app/testcases/` |
| Test data | `.\data\` | `/app/data/` |
| Generated outputs | `.\output\` | `/app/output/` |
| Schemas | `.\schemas\` | `/app/schemas/` |
| Scripts | `.\scripts\` | `/app/scripts/` |
| Examples | `.\examples\` | `/app/examples/` |

**Example:** Creating a file at `/app/testcases/my_test.yml` in the container will appear at `.\testcases\my_test.yml` on Windows.

## Workflow Example

Here's a complete workflow from creation to execution:

```powershell
# 1. Start the container
.\docker-compose.ps1 start

# 2. Enter the container
.\docker-compose.ps1 shell

# Inside container:
# 3. Create a new test case
tcm create-interactive --path /app/testcases
# Follow the prompts...

# 4. Validate the test case
validate-yaml /app/testcases/your_test.yml --schema /app/schemas/testcase.schema.json

# 5. Generate test script
test-executor generate /app/testcases/your_test.yml --output /app/output/test.sh

# 6. Execute the test
test-executor execute /app/testcases/your_test.yml

# 7. View the results
cat your_test_execution_log.json

# 8. Exit container
exit

# 9. View files on Windows
# Open .\testcases\your_test.yml in your favorite editor
# Open .\output\test.sh to see the generated script

# 10. Stop when done
.\docker-compose.ps1 stop
```

## Troubleshooting

### Docker Desktop Not Running

**Problem:** `error during connect: This error may indicate that the docker daemon is not running`

**Solution:** Start Docker Desktop from the Start menu and wait for it to fully initialize.

### Permission Errors

**Problem:** `Permission denied` or `Access is denied`

**Solution:** Run PowerShell or Command Prompt as Administrator.

### Build Fails

**Problem:** Build fails with errors

**Solution:**
```powershell
# Clean and rebuild
docker-compose down
docker system prune -a
docker-compose up -d --build --no-cache
```

### Files Not Showing

**Problem:** Files created in container don't appear on Windows

**Solution:**
1. Open Docker Desktop → Settings → Resources → File Sharing
2. Add the drive where your project is located
3. Click "Apply & Restart"

### Container Exits Immediately

**Problem:** Container starts but exits right away

**Solution:**
```powershell
# Check logs
docker-compose logs testcase-manager

# Verify environment file
Get-Content .env

# Recreate container
docker-compose down
docker-compose up -d
```

## Next Steps

1. **Read the full documentation**: [DOCKER_COMPOSE_WINDOWS.md](DOCKER_COMPOSE_WINDOWS.md)
2. **Explore example test cases**: Check the `.\testcases\examples\` directory
3. **Try the test executor**: Generate and run test scripts
4. **Set up git**: Configure git inside the container for version control
5. **Integrate with your workflow**: Use the helper scripts in your daily work

## Useful Resources

- **Complete Windows Guide**: [DOCKER_COMPOSE_WINDOWS.md](DOCKER_COMPOSE_WINDOWS.md)
- **Project README**: [README.md](README.md)
- **Docker Build Instructions**: [DOCKER_BUILD_INSTRUCTIONS.md](DOCKER_BUILD_INSTRUCTIONS.md)
- **Variable Passing**: [docs/VARIABLE_PASSING.md](docs/VARIABLE_PASSING.md)
- **BDD Initial Conditions**: [docs/BDD_INITIAL_CONDITIONS.md](docs/BDD_INITIAL_CONDITIONS.md)

## Getting Help

If you encounter issues:

1. Check the troubleshooting section above
2. Review Docker Desktop logs
3. Check container logs: `docker-compose logs -f`
4. Read the full documentation: [DOCKER_COMPOSE_WINDOWS.md](DOCKER_COMPOSE_WINDOWS.md)

---

**You're all set!** Start creating and managing your test cases with the Test Case Manager on Windows.
