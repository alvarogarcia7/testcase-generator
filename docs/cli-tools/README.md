# CLI Tools Reference

Test Case Manager provides a comprehensive suite of command-line tools for managing and executing test cases.

## Core Tools

### Test Case Management

- **editor / tcm / testcase-manager** - Interactive test case creation and management
  - Primary tool for creating and editing test cases
  - See [main documentation](../index.md#1-editor-test-case-manager--tcm) for complete command reference

### Test Execution & Verification

- **[test-verify](test-verify-usage.md)** - Verify test execution logs against test case definitions
  - [Usage Guide](test-verify-usage.md) - Detailed usage and examples
  - [Workflow Guide](test-verify-workflow.md) - Integration workflows
  - [Quick Reference](test-verify-quick-reference.md) - Command cheat sheet

### Validation Tools

- **[validate-yaml](validate-yaml.md)** - YAML validation with watch mode support
  - Single or multi-file validation
  - Watch mode for continuous monitoring
  - Schema enforcement

## Utility Tools

### JSON Processing

- **[json-escape](json-escaping-config.md)** - JSON string escaping utility
  - Read from stdin and perform JSON escaping
  - Test mode for validation
  - See also: [JSON Escaping Configuration](json-escaping-config.md)

### Script Cleanup

- **script-cleanup** - Terminal script capture cleanup
  - Remove ANSI codes and control characters
  - Process backspace characters
  - Clean terminal recordings

See [main documentation](../index.md#executables) for complete tool descriptions and usage examples.

## Quick Reference

| Tool | Purpose | Quick Example |
|------|---------|---------------|
| `editor` | Create/edit test cases | `editor complete --output testcases/my_test.yml` |
| `test-verify` | Verify test execution | `test-verify single --log exec.log --test-case-id TC001` |
| `validate-yaml` | Validate YAML files | `validate-yaml testcase.yml --schema schema.json --watch` |
| `test-executor` | Execute tests | `test-executor execute testcase.yml` |
| `test-orchestrator` | Orchestrate tests | `test-orchestrator run TC001 TC002 --workers 8` |
| `json-escape` | Escape JSON strings | `echo "text" \| json-escape` |
| `script-cleanup` | Clean terminal output | `script-cleanup -i raw.log -o clean.txt` |

## Integration Examples

- [GitLab CI Integration](../development/gitlab-ci-setup.md)
- [GitHub Actions Integration](../development/gitlab-ci-examples.md)
