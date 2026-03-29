# TPDG Conversion Logs

This directory contains execution logs from the TPDG conversion script (`run_tpdg_conversion.sh`).

## Log Files

Log files are automatically generated with timestamps when running the conversion script:

- `conversion_YYYYMMDD_HHMMSS.log` - Standard output and conversion details
- `conversion_YYYYMMDD_HHMMSS.err` - Error output (only created if errors occur)
- `latest.log` - Symlink to the most recent log file

## Log Retention

Log files are **ignored by git** (see `.gitignore`) to prevent repository bloat, but the directory structure is preserved via `.gitkeep`.

## Usage

To view the most recent log:
```bash
cat test-acceptance/results/logs/latest.log
```

To view all logs:
```bash
ls -lt test-acceptance/results/logs/
```

To clean old logs:
```bash
# Keep only the last 10 log files
cd test-acceptance/results/logs
ls -t conversion_*.log | tail -n +11 | xargs rm -f
ls -t conversion_*.err | tail -n +11 | xargs rm -f
```

## Log Format

Each log file contains:

1. **Header**: Timestamp, script path, working directory
2. **Configuration**: All paths and settings used
3. **Execution Output**: Verbose output from convert_verification_to_tpdg.py
4. **Statistics**: File size, line count, execution log count
5. **Footer**: Completion timestamp and exit code

## Example Log Entry

```
=========================================
TPDG Conversion Execution Log
=========================================
Started: Thu Mar 26 18:30:00 UTC 2026
Script: scripts/run_tpdg_conversion.sh
Working Directory: /path/to/testcase-generator
=========================================

Configuration:
  Python: /usr/bin/python3
  Conversion Script: scripts/convert_verification_to_tpdg.py
  Test Case Directory: test-acceptance/test_cases
  Logs Directory: test-acceptance/execution_logs
  Output File: test-acceptance/results/acceptance_test_results_container.yaml
  ...

[INFO] Running TPDG conversion...
Found 89 YAML files to process
Processing test case: TC_COMPLEX_ALL_HOOKS_CAPTURE_001
  Warning: No execution log found...
...

Output File Statistics:
  Path: test-acceptance/results/acceptance_test_results_container.yaml
  Size: 86K
  Lines: 2980
  Execution logs found: 0

=========================================
Completed: Thu Mar 26 18:30:05 UTC 2026
Exit Code: 0
=========================================
```

## Related Scripts

- `scripts/run_tpdg_conversion.sh` - Main script that generates these logs
- `scripts/convert_verification_to_tpdg.py` - The conversion script being executed
- `scripts/generate_acceptance_tpdg_container.sh` - Alternative script without detailed logging
