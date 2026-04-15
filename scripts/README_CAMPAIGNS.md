# Test Campaign Management Scripts

## Quick Links

- **Quick Start**: [CAMPAIGN_QUICK_START.md](CAMPAIGN_QUICK_START.md) - Get started in 5 minutes
- **Full Documentation**: [CAMPAIGN_MANAGEMENT_README.md](CAMPAIGN_MANAGEMENT_README.md) - Complete reference
- **Implementation Summary**: [../CAMPAIGN_IMPLEMENTATION_SUMMARY.md](../CAMPAIGN_IMPLEMENTATION_SUMMARY.md) - Technical details

## Scripts

| Script | Purpose | Documentation |
|--------|---------|---------------|
| `campaign-start.sh` | Initialize a new test campaign | See [CAMPAIGN_MANAGEMENT_README.md](CAMPAIGN_MANAGEMENT_README.md#1-campaign-startsh) |
| `campaign-run.sh` | Execute tests within a campaign | See [CAMPAIGN_MANAGEMENT_README.md](CAMPAIGN_MANAGEMENT_README.md#2-campaign-runsh) |
| `campaign-collect-evidence.sh` | Collect campaign evidence | See [CAMPAIGN_MANAGEMENT_README.md](CAMPAIGN_MANAGEMENT_README.md#3-campaign-collect-evidencesh) |
| `campaign-stop.sh` | Finalize campaign and generate reports | See [CAMPAIGN_MANAGEMENT_README.md](CAMPAIGN_MANAGEMENT_README.md#4-campaign-stopsh) |

## Quick Example

```bash
# 1. Start campaign
./scripts/campaign-start.sh --name "My_First_Campaign"

# 2. Run all tests
./scripts/campaign-run.sh --campaign campaigns/My_First_Campaign

# 3. Collect evidence
./scripts/campaign-collect-evidence.sh --campaign campaigns/My_First_Campaign --checksums

# 4. Stop campaign
./scripts/campaign-stop.sh --campaign campaigns/My_First_Campaign
```

## Features

- ✅ **Organized test execution** with campaign structure
- ✅ **Regex pattern matching** for test selection
- ✅ **Multiple test runs** with unique run IDs
- ✅ **Parallel test execution** support
- ✅ **Evidence collection** with SHA256 checksums
- ✅ **Comprehensive reporting** with Markdown summaries
- ✅ **CI/CD integration** ready
- ✅ **State management** (ACTIVE/COMPLETED)

## Campaign Structure

```
campaigns/<name>/
├── testcases/              # Test case YAML files (by run)
├── execution_logs/         # JSON execution logs (by run)
├── verification_results/   # Verification results
├── evidence/               # Additional evidence
├── reports/                # Generated reports
├── metadata/               # Campaign and run metadata
└── README.md              # Campaign README
```

## Help

Each script has built-in help:

```bash
./scripts/campaign-start.sh --help
./scripts/campaign-run.sh --help
./scripts/campaign-collect-evidence.sh --help
./scripts/campaign-stop.sh --help
```

## Documentation

- [CAMPAIGN_QUICK_START.md](CAMPAIGN_QUICK_START.md) - Quick start guide
- [CAMPAIGN_MANAGEMENT_README.md](CAMPAIGN_MANAGEMENT_README.md) - Complete documentation
- [../CAMPAIGN_IMPLEMENTATION_SUMMARY.md](../CAMPAIGN_IMPLEMENTATION_SUMMARY.md) - Implementation details
