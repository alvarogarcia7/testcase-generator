# Watch Mode Quick Reference

## Installation

**Linux:**
```bash
sudo apt-get install inotify-tools
```

**macOS:**
```bash
brew install fswatch
```

## Basic Commands

### Start Watch Mode (default testcases/ directory)
```bash
./scripts/validate-files.sh --pattern '\.ya?ml$' --validator ./scripts/validate-yaml-wrapper.sh --watch
```

### Watch Custom Directory
```bash
./scripts/validate-files.sh --pattern '\.ya?ml$' --validator ./scripts/validate-yaml-wrapper.sh --watch path/to/dir/
```

### Watch with Verbose Output
```bash
./scripts/validate-files.sh --pattern '\.ya?ml$' --validator ./scripts/validate-yaml-wrapper.sh --watch --verbose
```

### Use Convenience Script
```bash
./scripts/watch-yaml-files.sh
```

## Quick Start

1. Install platform-specific tool (inotify-tools or fswatch)
2. Set schema file (if not using default):
   ```bash
   export SCHEMA_FILE=schemas/schema.json
   ```
3. Run watch mode:
   ```bash
   ./scripts/watch-yaml-files.sh
   ```

## What Happens

1. **Initial Validation**: All matching files are validated on startup
2. **Monitoring**: Directory is watched recursively for changes
3. **Auto-Validation**: Changed files are validated immediately
4. **Live Feedback**: Results displayed with color coding:
   - ✓ PASSED (green)
   - ✗ FAILED (red) with error details
5. **Cache Persistence**: Validation cache maintained across sessions

## Stop Watch Mode

Press `Ctrl+C` to exit

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Tool not found | Install inotify-tools (Linux) or fswatch (macOS) |
| Directory not found | Check directory path exists |
| Pattern not matching | Test regex: `echo "file.yaml" \| grep -E '\.ya?ml$'` |
| Validation errors | Run validator directly: `./scripts/validate-yaml-wrapper.sh file.yaml` |

## Common Patterns

| Pattern | Matches |
|---------|---------|
| `\.ya?ml$` | .yaml and .yml files |
| `\.json$` | .json files |
| `\.rs$` | Rust files |
| `\.(yaml\|yml\|json)$` | Multiple extensions |

## Cache Management

### View Cache
```bash
ls -la .validation-cache/
```

### Clear Cache
```bash
rm -rf .validation-cache/
```

### Cache Location (custom)
```bash
./scripts/validate-files.sh --pattern '...' --validator '...' --cache-dir /tmp/cache --watch
```

## Integration with Make

Add to Makefile:
```makefile
.PHONY: watch
watch:
	./scripts/watch-yaml-files.sh
```

Then run: `make watch`

## See Also

- [Watch Mode Guide](WATCH_MODE_GUIDE.md) - Comprehensive documentation
- [validate-files.sh](validate-files.sh) - Main script
- [validate-yaml-wrapper.sh](validate-yaml-wrapper.sh) - YAML validator wrapper
