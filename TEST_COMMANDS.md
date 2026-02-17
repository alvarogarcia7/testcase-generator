# Test Commands for Cross-Platform Compatibility Tests

## As Requested in Task

Run all compatibility tests with pattern matching:
```bash
cargo test --all-features -- json_escape_integration_test::test_.*compatibility
```

This command will run all tests with "compatibility" in their name, which includes:
- test_bash_32_compatibility
- test_sed_bsd_compatibility
- test_awk_cross_platform_compatibility
- test_script_execution_cross_platform
- test_newline_handling_cross_platform
- test_posix_shell_compatibility
- test_awk_printf_pattern_compatibility

## Run All New Compatibility Tests Individually

```bash
# Bash 3.2+ compatibility
cargo test --all-features -- json_escape_integration_test::test_bash_32_compatibility

# BSD sed compatibility (no -r flag)
cargo test --all-features -- json_escape_integration_test::test_sed_bsd_compatibility

# Cross-platform awk compatibility
cargo test --all-features -- json_escape_integration_test::test_awk_cross_platform_compatibility

# Script execution on current platform
cargo test --all-features -- json_escape_integration_test::test_script_execution_cross_platform

# Newline handling (LF/CRLF/CR)
cargo test --all-features -- json_escape_integration_test::test_newline_handling_cross_platform

# sed/awk fallback with special characters
cargo test --all-features -- json_escape_integration_test::test_sed_awk_fallback_special_chars

# POSIX shell compatibility
cargo test --all-features -- json_escape_integration_test::test_posix_shell_compatibility

# sed basic patterns only (no GNU extensions)
cargo test --all-features -- json_escape_integration_test::test_sed_uses_basic_patterns_only

# awk printf pattern compatibility
cargo test --all-features -- json_escape_integration_test::test_awk_printf_pattern_compatibility

# sed/awk execution with complex characters
cargo test --all-features -- json_escape_integration_test::test_sed_awk_execution_cross_platform

# Empty output edge case
cargo test --all-features -- json_escape_integration_test::test_empty_output_cross_platform

# printf usage for portability
cargo test --all-features -- json_escape_integration_test::test_printf_for_portability

# Whitespace handling
cargo test --all-features -- json_escape_integration_test::test_no_trailing_whitespace_issues
```

## Run All Integration Tests

```bash
cargo test --all-features --test json_escape_integration_test
```

## Run Tests by Category

```bash
# All compatibility tests
cargo test --all-features --test json_escape_integration_test compatibility

# All cross-platform tests
cargo test --all-features --test json_escape_integration_test cross_platform

# All fallback tests
cargo test --all-features --test json_escape_integration_test fallback
```
