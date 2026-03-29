# ⚠️ SCCACHE BUILD FAILURE - QUICK FIX

## If Your Build is Failing with Exit Status 254

You're seeing this error:
```
error occurred in cc-rs: command did not execute successfully 
(status code exit status: 254): env ... "sccache" "cc" ...
```

### IMMEDIATE FIX

Run these commands:

```bash
# Disable sccache
unset RUSTC_WRAPPER

# Clean build artifacts
cargo clean

# Rebuild without sccache
cargo build
```

### Why This Happens

sccache can have issues with:
- Native C dependencies (libssh2-sys, openssl-sys, libgit2-sys)
- Non-git directories
- Certain build configurations

### Alternative Methods to Disable sccache

**Method 1: Use the disable script**
```bash
source ./scripts/disable-sccache.sh
cargo clean
cargo build
```

**Method 2: Stop sccache server**
```bash
sccache --stop-server
cargo clean
cargo build
```

**Method 3: Clear sccache cache**
```bash
rm -rf ~/.cache/sccache/testcase-manager
cargo clean
cargo build
```

### Permanent Solution

If sccache keeps causing issues, remove it from your shell profile:

1. Edit `~/.bashrc` or `~/.zshrc`
2. Remove line: `export RUSTC_WRAPPER=sccache`
3. Restart terminal or run: `source ~/.bashrc` (or `~/.zshrc`)

### More Information

- Full setup guide: [docs/SCCACHE_SETUP.md](docs/SCCACHE_SETUP.md)
- Comprehensive docs: [docs/SCCACHE.md](docs/SCCACHE.md)
- Project commands: [AGENTS.md](AGENTS.md#sccache-configuration)

### Still Having Issues?

Make sure sccache is actually disabled:
```bash
echo $RUSTC_WRAPPER
# Should output nothing (blank line)

# If it still shows "sccache", run:
unset RUSTC_WRAPPER
```

Then try building again:
```bash
cargo clean
cargo build
```
