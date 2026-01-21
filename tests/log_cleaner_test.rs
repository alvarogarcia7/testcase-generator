use testcase_manager::log_cleaner::LogCleaner;
use testcase_manager::models::TestExecutionLog;

// ============================================================================
// Timestamp Removal Tests - RFC3339
// ============================================================================

#[test]
fn test_remove_timestamps_rfc3339_basic() {
    let cleaner = LogCleaner::new();
    let text = "Event occurred at 2024-01-15T10:30:45Z";
    let result = cleaner.remove_timestamps(text);
    assert_eq!(result, "Event occurred at [TIMESTAMP]");
}

#[test]
fn test_remove_timestamps_rfc3339_with_milliseconds() {
    let cleaner = LogCleaner::new();
    let text = "Timestamp: 2024-01-15T10:30:45.123Z";
    let result = cleaner.remove_timestamps(text);
    assert_eq!(result, "Timestamp: [TIMESTAMP]");
}

#[test]
fn test_remove_timestamps_rfc3339_with_timezone() {
    let cleaner = LogCleaner::new();
    let text = "Log at 2024-01-15T10:30:45+05:30 and 2024-01-15T10:30:45-08:00";
    let result = cleaner.remove_timestamps(text);
    assert_eq!(result, "Log at [TIMESTAMP] and [TIMESTAMP]");
}

#[test]
fn test_remove_timestamps_rfc3339_with_microseconds() {
    let cleaner = LogCleaner::new();
    let text = "Precision: 2024-12-31T23:59:59.999999Z";
    let result = cleaner.remove_timestamps(text);
    assert_eq!(result, "Precision: [TIMESTAMP]");
}

#[test]
fn test_remove_timestamps_rfc3339_multiple() {
    let cleaner = LogCleaner::new();
    let text = "Start: 2024-01-01T00:00:00Z, End: 2024-12-31T23:59:59.999+00:00";
    let result = cleaner.remove_timestamps(text);
    assert_eq!(result, "Start: [TIMESTAMP], End: [TIMESTAMP]");
}

#[test]
fn test_remove_timestamps_rfc3339_in_sentence() {
    let cleaner = LogCleaner::new();
    let text = "The server started at 2024-01-15T10:30:45Z and stopped at 2024-01-15T12:30:45.123+00:00 unexpectedly.";
    let result = cleaner.remove_timestamps(text);
    assert_eq!(
        result,
        "The server started at [TIMESTAMP] and stopped at [TIMESTAMP] unexpectedly."
    );
}

// ============================================================================
// Timestamp Removal Tests - Unix Timestamps
// ============================================================================

#[test]
fn test_remove_timestamps_unix_10_digits() {
    let cleaner = LogCleaner::new();
    let text = "Unix timestamp: 1705315845";
    let result = cleaner.remove_timestamps(text);
    assert_eq!(result, "Unix timestamp: [TIMESTAMP]");
}

#[test]
fn test_remove_timestamps_unix_13_digits_milliseconds() {
    let cleaner = LogCleaner::new();
    let text = "Unix timestamp with milliseconds: 1705315845123";
    let result = cleaner.remove_timestamps(text);
    assert_eq!(result, "Unix timestamp with milliseconds: [TIMESTAMP]");
}

#[test]
fn test_remove_timestamps_unix_multiple() {
    let cleaner = LogCleaner::new();
    let text = "Start: 1705315845, End: 1705315945123";
    let result = cleaner.remove_timestamps(text);
    assert_eq!(result, "Start: [TIMESTAMP], End: [TIMESTAMP]");
}

#[test]
fn test_remove_timestamps_unix_word_boundary() {
    let cleaner = LogCleaner::new();
    // Should NOT match numbers that are part of longer numeric sequences
    let text = "ID: 12345678901234567890 but timestamp: 1705315845";
    let result = cleaner.remove_timestamps(text);
    // The long ID should remain, only the 10-digit timestamp should be replaced
    assert!(result.contains("12345678901234567890"));
    assert!(result.contains("[TIMESTAMP]"));
}

#[test]
fn test_remove_timestamps_unix_in_log_output() {
    let cleaner = LogCleaner::new();
    let text = "[1705315845] Server started\n[1705315846123] Connection established";
    let result = cleaner.remove_timestamps(text);
    assert_eq!(
        result,
        "[[TIMESTAMP]] Server started\n[[TIMESTAMP]] Connection established"
    );
}

// ============================================================================
// Timestamp Removal Tests - Relative Time
// ============================================================================

#[test]
fn test_remove_timestamps_relative_seconds() {
    let cleaner = LogCleaner::new();
    let text = "Updated 30 seconds ago";
    let result = cleaner.remove_timestamps(text);
    assert_eq!(result, "Updated [RELATIVE_TIME]");
}

#[test]
fn test_remove_timestamps_relative_minutes() {
    let cleaner = LogCleaner::new();
    let text = "Modified 5 minutes ago and 15 mins ago";
    let result = cleaner.remove_timestamps(text);
    assert_eq!(result, "Modified [RELATIVE_TIME] and [RELATIVE_TIME]");
}

#[test]
fn test_remove_timestamps_relative_hours() {
    let cleaner = LogCleaner::new();
    let text = "Last seen 2 hours ago or 3 hrs ago";
    let result = cleaner.remove_timestamps(text);
    assert_eq!(result, "Last seen [RELATIVE_TIME] or [RELATIVE_TIME]");
}

#[test]
fn test_remove_timestamps_relative_days_weeks() {
    let cleaner = LogCleaner::new();
    let text = "Created 7 days ago, updated 2 weeks ago";
    let result = cleaner.remove_timestamps(text);
    assert_eq!(result, "Created [RELATIVE_TIME], updated [RELATIVE_TIME]");
}

#[test]
fn test_remove_timestamps_relative_months_years() {
    let cleaner = LogCleaner::new();
    let text = "Published 3 months ago, archived 1 year ago";
    let result = cleaner.remove_timestamps(text);
    assert_eq!(
        result,
        "Published [RELATIVE_TIME], archived [RELATIVE_TIME]"
    );
}

#[test]
fn test_remove_timestamps_relative_singular() {
    let cleaner = LogCleaner::new();
    let text = "1 second ago, 1 minute ago, 1 hour ago, 1 day ago";
    let result = cleaner.remove_timestamps(text);
    assert_eq!(
        result,
        "[RELATIVE_TIME], [RELATIVE_TIME], [RELATIVE_TIME], [RELATIVE_TIME]"
    );
}

#[test]
fn test_remove_timestamps_relative_abbreviations() {
    let cleaner = LogCleaner::new();
    let text = "5 sec ago, 10 min ago, 2 hr ago";
    let result = cleaner.remove_timestamps(text);
    assert_eq!(result, "[RELATIVE_TIME], [RELATIVE_TIME], [RELATIVE_TIME]");
}

#[test]
fn test_remove_timestamps_relative_with_whitespace_variations() {
    let cleaner = LogCleaner::new();
    let text = "5minutes ago and 10 seconds ago";
    let result = cleaner.remove_timestamps(text);
    assert_eq!(result, "[RELATIVE_TIME] and [RELATIVE_TIME]");
}

// ============================================================================
// Timestamp Removal Tests - Mixed Formats
// ============================================================================

#[test]
fn test_remove_timestamps_mixed_formats() {
    let cleaner = LogCleaner::new();
    let text = "RFC3339: 2024-01-15T10:30:45Z, Unix: 1705315845, Relative: 5 minutes ago";
    let result = cleaner.remove_timestamps(text);
    assert_eq!(
        result,
        "RFC3339: [TIMESTAMP], Unix: [TIMESTAMP], Relative: [RELATIVE_TIME]"
    );
}

#[test]
fn test_remove_timestamps_complex_log_line() {
    let cleaner = LogCleaner::new();
    let text = "[2024-01-15T10:30:45.123Z] [1705315845] Event occurred 30 seconds ago";
    let result = cleaner.remove_timestamps(text);
    assert_eq!(
        result,
        "[[TIMESTAMP]] [[TIMESTAMP]] Event occurred [RELATIVE_TIME]"
    );
}

// ============================================================================
// ANSI Code Removal Tests - Color Codes
// ============================================================================

#[test]
fn test_remove_ansi_codes_red_text() {
    let cleaner = LogCleaner::new();
    let text = "\x1b[31mError\x1b[0m";
    let result = cleaner.remove_ansi_codes(text);
    assert_eq!(result, "Error");
}

#[test]
fn test_remove_ansi_codes_green_text() {
    let cleaner = LogCleaner::new();
    let text = "\x1b[32mSuccess\x1b[0m";
    let result = cleaner.remove_ansi_codes(text);
    assert_eq!(result, "Success");
}

#[test]
fn test_remove_ansi_codes_multiple_colors() {
    let cleaner = LogCleaner::new();
    let text = "\x1b[31mError\x1b[0m and \x1b[32mSuccess\x1b[0m and \x1b[33mWarning\x1b[0m";
    let result = cleaner.remove_ansi_codes(text);
    assert_eq!(result, "Error and Success and Warning");
}

#[test]
fn test_remove_ansi_codes_bold_and_color() {
    let cleaner = LogCleaner::new();
    let text = "\x1b[1;31mBold Red\x1b[0m and \x1b[1;32mBold Green\x1b[0m";
    let result = cleaner.remove_ansi_codes(text);
    assert_eq!(result, "Bold Red and Bold Green");
}

#[test]
fn test_remove_ansi_codes_background_colors() {
    let cleaner = LogCleaner::new();
    let text = "\x1b[41mRed Background\x1b[0m and \x1b[42mGreen Background\x1b[0m";
    let result = cleaner.remove_ansi_codes(text);
    assert_eq!(result, "Red Background and Green Background");
}

#[test]
fn test_remove_ansi_codes_256_colors() {
    let cleaner = LogCleaner::new();
    let text = "\x1b[38;5;208mOrange Text\x1b[0m";
    let result = cleaner.remove_ansi_codes(text);
    assert_eq!(result, "Orange Text");
}

#[test]
fn test_remove_ansi_codes_rgb_colors() {
    let cleaner = LogCleaner::new();
    let text = "\x1b[38;2;255;128;0mRGB Orange\x1b[0m";
    let result = cleaner.remove_ansi_codes(text);
    assert_eq!(result, "RGB Orange");
}

// ============================================================================
// ANSI Code Removal Tests - Cursor Positioning
// ============================================================================

#[test]
fn test_remove_ansi_codes_cursor_up() {
    let cleaner = LogCleaner::new();
    let text = "Line 1\x1b[1ALine 2";
    let result = cleaner.remove_ansi_codes(text);
    assert_eq!(result, "Line 1Line 2");
}

#[test]
fn test_remove_ansi_codes_cursor_down() {
    let cleaner = LogCleaner::new();
    let text = "Line 1\x1b[2BLine 2";
    let result = cleaner.remove_ansi_codes(text);
    assert_eq!(result, "Line 1Line 2");
}

#[test]
fn test_remove_ansi_codes_cursor_forward() {
    let cleaner = LogCleaner::new();
    let text = "Text\x1b[5CMore text";
    let result = cleaner.remove_ansi_codes(text);
    assert_eq!(result, "TextMore text");
}

#[test]
fn test_remove_ansi_codes_cursor_backward() {
    let cleaner = LogCleaner::new();
    let text = "Text\x1b[3DMore text";
    let result = cleaner.remove_ansi_codes(text);
    assert_eq!(result, "TextMore text");
}

#[test]
fn test_remove_ansi_codes_cursor_position() {
    let cleaner = LogCleaner::new();
    let text = "\x1b[10;20HPositioned text";
    let result = cleaner.remove_ansi_codes(text);
    assert_eq!(result, "Positioned text");
}

#[test]
fn test_remove_ansi_codes_clear_screen() {
    let cleaner = LogCleaner::new();
    let text = "\x1b[2JCleared screen";
    let result = cleaner.remove_ansi_codes(text);
    assert_eq!(result, "Cleared screen");
}

#[test]
fn test_remove_ansi_codes_erase_line() {
    let cleaner = LogCleaner::new();
    let text = "Some text\x1b[KErased to end";
    let result = cleaner.remove_ansi_codes(text);
    assert_eq!(result, "Some textErased to end");
}

// ============================================================================
// ANSI Code Removal Tests - Mixed Styles
// ============================================================================

#[test]
fn test_remove_ansi_codes_mixed_styles() {
    let cleaner = LogCleaner::new();
    let text = "\x1b[1mBold\x1b[0m \x1b[3mItalic\x1b[0m \x1b[4mUnderline\x1b[0m";
    let result = cleaner.remove_ansi_codes(text);
    assert_eq!(result, "Bold Italic Underline");
}

#[test]
fn test_remove_ansi_codes_nested_styles() {
    let cleaner = LogCleaner::new();
    let text = "\x1b[1m\x1b[31mBold Red\x1b[0m\x1b[0m";
    let result = cleaner.remove_ansi_codes(text);
    assert_eq!(result, "Bold Red");
}

#[test]
fn test_remove_ansi_codes_complex_terminal_output() {
    let cleaner = LogCleaner::new();
    let text = "\x1b[2J\x1b[H\x1b[32mWelcome\x1b[0m\x1b[10;20H\x1b[1;31mError!\x1b[0m";
    let result = cleaner.remove_ansi_codes(text);
    assert_eq!(result, "WelcomeError!");
}

// ============================================================================
// Path Normalization Tests - Absolute to Relative (Unix)
// ============================================================================

#[test]
fn test_normalize_paths_unix_absolute() {
    let cleaner = LogCleaner::new();
    let text = "File at /home/user/project/src/main.rs";
    let result = cleaner.normalize_paths(text);
    // Should extract just the filename
    assert!(result.contains("main.rs"));
    assert!(!result.contains("/home/user/project"));
}

#[test]
fn test_normalize_paths_unix_multiple() {
    let cleaner = LogCleaner::new();
    let text = "Files: /tmp/file1.txt and /var/log/app.log";
    let result = cleaner.normalize_paths(text);
    assert!(result.contains("file1.txt"));
    assert!(result.contains("app.log"));
}

#[test]
fn test_normalize_paths_unix_in_quotes() {
    let cleaner = LogCleaner::new();
    let text = r#"Error in "/home/user/project/src/lib.rs""#;
    let result = cleaner.normalize_paths(text);
    assert!(result.contains("lib.rs"));
}

#[test]
fn test_normalize_paths_unix_in_single_quotes() {
    let cleaner = LogCleaner::new();
    let text = "Error in '/usr/local/bin/app'";
    let result = cleaner.normalize_paths(text);
    assert!(result.contains("app"));
}

#[test]
fn test_normalize_paths_unix_in_parentheses() {
    let cleaner = LogCleaner::new();
    let text = "Error (/home/user/error.log)";
    let result = cleaner.normalize_paths(text);
    assert!(result.contains("error.log"));
}

#[test]
fn test_normalize_paths_unix_with_extension() {
    let cleaner = LogCleaner::new();
    let text = "/etc/config.yaml /var/data.json /tmp/test.rs";
    let result = cleaner.normalize_paths(text);
    assert!(result.contains("config.yaml"));
    assert!(result.contains("data.json"));
    assert!(result.contains("test.rs"));
}

#[test]
fn test_normalize_paths_unix_deep_hierarchy() {
    let cleaner = LogCleaner::new();
    let text = "/home/user/projects/rust/myapp/src/modules/auth/handler.rs";
    let result = cleaner.normalize_paths(text);
    assert!(result.contains("handler.rs"));
}

// ============================================================================
// Path Normalization Tests - Windows Paths
// ============================================================================

#[test]
fn test_normalize_paths_windows_absolute() {
    let cleaner = LogCleaner::new();
    let text = r"File at C:\Users\user\project\src\main.rs";
    let result = cleaner.normalize_paths(text);
    assert!(result.contains("main.rs"));
}

#[test]
fn test_normalize_paths_windows_forward_slash() {
    let cleaner = LogCleaner::new();
    let text = "File at C:/Users/user/project/src/main.rs";
    let result = cleaner.normalize_paths(text);
    assert!(result.contains("main.rs"));
}

#[test]
fn test_normalize_paths_windows_different_drives() {
    let cleaner = LogCleaner::new();
    let text = r"Files: C:\project\file1.txt and D:\backup\file2.txt";
    let result = cleaner.normalize_paths(text);
    assert!(result.contains("file1.txt"));
    assert!(result.contains("file2.txt"));
}

#[test]
fn test_normalize_paths_windows_program_files() {
    let cleaner = LogCleaner::new();
    let text = r"Path: C:\Program Files\MyApp\bin\app.exe";
    let result = cleaner.normalize_paths(text);
    assert!(result.contains("app.exe"));
}

// ============================================================================
// Path Normalization Tests - Mixed Paths
// ============================================================================

#[test]
fn test_normalize_paths_mixed_unix_windows() {
    let cleaner = LogCleaner::new();
    let text = r"Unix: /home/user/file.txt Windows: C:\Users\user\file.txt";
    let result = cleaner.normalize_paths(text);
    // Both should be normalized to just the filename
    let count = result.matches("file.txt").count();
    assert_eq!(count, 2);
}

#[test]
fn test_normalize_paths_url_should_not_match() {
    let cleaner = LogCleaner::new();
    let text = "URL: https://example.com/path/to/resource";
    let result = cleaner.normalize_paths(text);
    // URLs should remain unchanged (the regex should not match HTTP URLs)
    assert_eq!(result, text);
}

#[test]
fn test_normalize_paths_relative_unchanged() {
    let cleaner = LogCleaner::new();
    let text = "Relative: src/main.rs and ./test.rs";
    let result = cleaner.normalize_paths(text);
    // Relative paths without leading / or drive letter should remain unchanged
    assert_eq!(result, text);
}

// ============================================================================
// Whitespace Normalization Tests
// ============================================================================

#[test]
fn test_normalize_whitespace_multiple_spaces() {
    let cleaner = LogCleaner::new();
    let text = "Text  with    multiple     spaces";
    let result = cleaner.normalize_whitespace(text);
    assert_eq!(result, "Text with multiple spaces");
}

#[test]
fn test_normalize_whitespace_multiple_newlines() {
    let cleaner = LogCleaner::new();
    let text = "Line1\n\n\n\n\nLine2";
    let result = cleaner.normalize_whitespace(text);
    assert_eq!(result, "Line1\n\nLine2");
}

#[test]
fn test_normalize_whitespace_leading_trailing() {
    let cleaner = LogCleaner::new();
    let text = "   Text with spaces   ";
    let result = cleaner.normalize_whitespace(text);
    assert_eq!(result, "Text with spaces");
}

#[test]
fn test_normalize_whitespace_mixed() {
    let cleaner = LogCleaner::new();
    let text = "  Line1    with   spaces\n\n\n\nLine2  \n\n\n  Line3  ";
    let result = cleaner.normalize_whitespace(text);
    assert_eq!(result, "Line1 with spaces\n\nLine2\n\nLine3");
}

#[test]
fn test_normalize_whitespace_tabs_preserved() {
    let cleaner = LogCleaner::new();
    let text = "Text\twith\ttabs";
    let result = cleaner.normalize_whitespace(text);
    // Tabs are preserved, only multiple spaces are normalized
    assert_eq!(result, "Text\twith\ttabs");
}

#[test]
fn test_normalize_whitespace_single_newline_preserved() {
    let cleaner = LogCleaner::new();
    let text = "Line1\nLine2\nLine3";
    let result = cleaner.normalize_whitespace(text);
    assert_eq!(result, "Line1\nLine2\nLine3");
}

#[test]
fn test_normalize_whitespace_double_newline_preserved() {
    let cleaner = LogCleaner::new();
    let text = "Paragraph1\n\nParagraph2";
    let result = cleaner.normalize_whitespace(text);
    assert_eq!(result, "Paragraph1\n\nParagraph2");
}

// ============================================================================
// Full Cleanup Pipeline Tests
// ============================================================================

#[test]
fn test_full_cleanup_simple_log() {
    let cleaner = LogCleaner::new();
    let log = TestExecutionLog {
        test_case_id: "TC001".to_string(),
        sequence_id: 1,
        timestamp: "2024-01-15T10:30:45Z".to_string(),
        actual_output: "Simple output".to_string(),
        actual_success: true,
        duration_ms: 1000,
        error_message: None,
    };

    let cleaned = cleaner.clean_execution_log(&log);
    assert_eq!(cleaned.timestamp, "[TIMESTAMP]");
    assert_eq!(cleaned.actual_output, "Simple output");
    assert_eq!(cleaned.error_message, None);
}

#[test]
fn test_full_cleanup_with_ansi_and_timestamps() {
    let cleaner = LogCleaner::new();
    let log = TestExecutionLog {
        test_case_id: "TC002".to_string(),
        sequence_id: 1,
        timestamp: "2024-01-15T10:30:45Z".to_string(),
        actual_output: "\x1b[31mError at 2024-01-15T10:30:45Z\x1b[0m".to_string(),
        actual_success: false,
        duration_ms: 1000,
        error_message: Some("\x1b[1;31mFailed at 2024-01-15T10:30:45Z\x1b[0m".to_string()),
    };

    let cleaned = cleaner.clean_execution_log(&log);
    assert_eq!(cleaned.timestamp, "[TIMESTAMP]");
    assert_eq!(cleaned.actual_output, "Error at [TIMESTAMP]");
    assert_eq!(
        cleaned.error_message,
        Some("Failed at [TIMESTAMP]".to_string())
    );
}

#[test]
fn test_full_cleanup_with_paths_and_whitespace() {
    let cleaner = LogCleaner::new();
    let log = TestExecutionLog {
        test_case_id: "TC003".to_string(),
        sequence_id: 1,
        timestamp: "2024-01-15T10:30:45Z".to_string(),
        actual_output: "  File  /tmp/test.txt    has   error\n\n\n\n".to_string(),
        actual_success: false,
        duration_ms: 1000,
        error_message: None,
    };

    let cleaned = cleaner.clean_execution_log(&log);
    assert_eq!(cleaned.timestamp, "[TIMESTAMP]");
    assert!(cleaned.actual_output.contains("test.txt"));
    assert!(!cleaned.actual_output.contains("  "));
    assert!(!cleaned.actual_output.contains("\n\n\n"));
}

// ============================================================================
// Realistic Unix Script Output Examples
// ============================================================================

#[test]
fn test_realistic_bash_script_output() {
    let cleaner = LogCleaner::new();
    let output = r#"[2024-01-15T10:30:45.123Z] Starting deployment script...
Checking files in /home/user/deploy/app
Found configuration: /home/user/deploy/app/config.yaml
[32mSUCCESS[0m: Configuration loaded
Updated 5 minutes ago
Timestamp: 1705315845
Copying files from /var/www/app to /var/www/backup
[1;33mWARNING[0m: Backup directory exists at /var/www/backup
Cleaning old logs...


Processing complete at 2024-01-15T10:35:45.456Z"#;

    let result = cleaner.remove_timestamps(output);
    let result = cleaner.remove_ansi_codes(&result);
    let result = cleaner.normalize_paths(&result);
    let result = cleaner.normalize_whitespace(&result);

    // Verify timestamps are removed
    assert!(result.contains("[TIMESTAMP]"));
    assert!(result.contains("[RELATIVE_TIME]"));
    assert!(!result.contains("2024-01-15"));
    assert!(!result.contains("1705315845"));

    // Verify ANSI codes are removed
    assert!(!result.contains("[32m"));
    assert!(!result.contains("[0m"));

    // Verify paths are normalized
    assert!(result.contains("config.yaml"));

    // Verify whitespace is normalized
    assert!(!result.contains("\n\n\n"));
}

#[test]
fn test_realistic_compilation_output() {
    let cleaner = LogCleaner::new();
    let output = r#"[1705315845] Compiling myapp v0.1.0 (/home/user/projects/myapp)
   Compiling proc-macro2 v1.0.70
   Compiling unicode-ident v1.0.12
[32m   Compiling[0m serde v1.0.193
    Finished dev [unoptimized + debuginfo] target(s) in 2m 30s
    
    
Build completed at 2024-01-15T10:30:45.789Z
Artifacts stored in /home/user/projects/myapp/target/debug/"#;

    let result = cleaner.remove_timestamps(output);
    let result = cleaner.remove_ansi_codes(&result);
    let result = cleaner.normalize_paths(&result);
    let result = cleaner.normalize_whitespace(&result);

    assert!(result.contains("[TIMESTAMP]"));
    assert!(!result.contains("2024-01-15"));
    assert!(result.contains("Compiling serde"));
    assert!(!result.contains("[32m"));
}

#[test]
fn test_realistic_test_runner_output() {
    let cleaner = LogCleaner::new();
    let output = r#"
running 5 tests
test tests::test_one ... [32mok[0m
test tests::test_two ... [31mFAILED[0m
test tests::test_three ... ok

failures:

---- tests::test_two stdout ----
thread 'tests::test_two' panicked at 'assertion failed' /home/user/src/tests.rs:42:5
note: run with `RUST_BACKTRACE=1` for a backtrace

test result: [31mFAILED[0m. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s at 2024-01-15T10:30:45Z


"#;

    let result = cleaner.remove_timestamps(output);
    let result = cleaner.remove_ansi_codes(&result);
    let result = cleaner.normalize_paths(&result);
    let result = cleaner.normalize_whitespace(&result);

    assert!(!result.contains("[31m"));
    assert!(!result.contains("[32m"));
    assert!(result.contains("tests.rs"));
    assert!(result.contains("[TIMESTAMP]"));
}

#[test]
fn test_realistic_git_output() {
    let cleaner = LogCleaner::new();
    let output = r#"[33mcommit 1234567890abcdef[0m
Author: Developer <dev@example.com>
Date:   2024-01-15T10:30:45Z

    Updated configuration

 /home/user/project/config.yaml | 5 [32m+++++[0m
 /home/user/project/src/main.rs | 10 [31m----------[0m
 2 files changed, 5 insertions(+), 10 deletions(-)
 
Modified 3 hours ago
"#;

    let result = cleaner.remove_timestamps(output);
    let result = cleaner.remove_ansi_codes(&result);
    let result = cleaner.normalize_paths(&result);
    let result = cleaner.normalize_whitespace(&result);

    assert!(result.contains("[TIMESTAMP]"));
    assert!(result.contains("[RELATIVE_TIME]"));
    assert!(result.contains("config.yaml"));
    assert!(result.contains("main.rs"));
    assert!(!result.contains("[33m"));
}

#[test]
fn test_realistic_server_log_output() {
    let cleaner = LogCleaner::new();
    let output = r#"[2024-01-15T10:30:45.123+00:00] [INFO] Server starting on port 8080
[2024-01-15T10:30:45.456+00:00] [INFO] Loading config from /etc/myapp/config.toml
[2024-01-15T10:30:46.789+00:00] [WARN] Cache directory /var/cache/myapp does not exist, creating...
[2024-01-15T10:30:47.012+00:00] [32m[INFO] Server ready[0m
[2024-01-15T10:30:50.345+00:00] [ERROR] Failed to connect to database at /var/lib/myapp/db.sqlite
[2024-01-15T10:30:50.678+00:00] [31m[ERROR] Shutting down due to critical error[0m

Last restart: 1 hour ago
"#;

    let result = cleaner.remove_timestamps(output);
    let result = cleaner.remove_ansi_codes(&result);
    let result = cleaner.normalize_paths(&result);
    let result = cleaner.normalize_whitespace(&result);

    // All RFC3339 timestamps should be replaced
    assert!(!result.contains("2024-01-15"));
    assert!(result.matches("[TIMESTAMP]").count() >= 6);

    // Relative time should be replaced
    assert!(result.contains("[RELATIVE_TIME]"));

    // ANSI codes should be removed
    assert!(!result.contains("[32m"));
    assert!(!result.contains("[31m"));

    // Paths should be normalized
    assert!(result.contains("config.toml"));
    assert!(result.contains("db.sqlite"));
}

#[test]
fn test_realistic_docker_build_output() {
    let cleaner = LogCleaner::new();
    let output = r#"[1705315845] Sending build context to Docker daemon  123.4MB
Step 1/5 : FROM rust:1.75
[2024-01-15T10:30:46Z] ---> abc123def456
Step 2/5 : WORKDIR /app
[2024-01-15T10:30:47Z] ---> Running in container123
[32m ---> xyz789[0m
Step 3/5 : COPY /home/user/project/Cargo.toml /app/
[2024-01-15T10:30:48Z] ---> 111222333
[1;32mSuccessfully built[0m 444555666
[1;32mSuccessfully tagged[0m myapp:latest

Build completed 2 minutes ago
"#;

    let result = cleaner.remove_timestamps(output);
    let result = cleaner.remove_ansi_codes(&result);
    let result = cleaner.normalize_paths(&result);
    let result = cleaner.normalize_whitespace(&result);

    assert!(result.contains("[TIMESTAMP]"));
    assert!(result.contains("[RELATIVE_TIME]"));
    assert!(result.contains("Cargo.toml"));
    assert!(!result.contains("[32m"));
    assert!(!result.contains("[1;32m"));
}

#[test]
fn test_realistic_systemd_journal_output() {
    let cleaner = LogCleaner::new();
    let output = r#"-- Logs begin at 2024-01-15T10:00:00Z, end at 2024-01-15T10:30:45Z --
Jan 15 10:30:45 myhost myapp[12345]: [INFO] Application started
Jan 15 10:30:46 myhost myapp[12345]: [INFO] Configuration loaded from /etc/myapp/app.conf
Jan 15 10:30:47 myhost myapp[12345]: [32m[INFO] Database connected[0m
Jan 15 10:30:48 myhost myapp[12345]: [WARN] Deprecated API endpoint used
Jan 15 10:30:49 myhost myapp[12345]: [31m[ERROR] Failed to write to /var/log/myapp/error.log[0m

Status checked 30 seconds ago
"#;

    let result = cleaner.remove_timestamps(output);
    let result = cleaner.remove_ansi_codes(&result);
    let result = cleaner.normalize_paths(&result);
    let result = cleaner.normalize_whitespace(&result);

    assert!(result.contains("[TIMESTAMP]"));
    assert!(result.contains("[RELATIVE_TIME]"));
    assert!(result.contains("app.conf"));
    assert!(result.contains("error.log"));
    assert!(!result.contains("[32m"));
}

#[test]
fn test_realistic_npm_install_output() {
    let cleaner = LogCleaner::new();
    let output = r#"[1705315845123] npm install
npm WARN deprecated package@1.0.0: This package is no longer maintained
added 123 packages, and audited 456 packages in 5s

[32m45 packages[0m are looking for funding
  run `npm fund` for details

[33m3 moderate severity vulnerabilities[0m

To address all issues, run:
  npm audit fix

Last updated 10 minutes ago
Build artifacts in C:\Users\user\project\node_modules
"#;

    let result = cleaner.remove_timestamps(output);
    let result = cleaner.remove_ansi_codes(&result);
    let result = cleaner.normalize_paths(&result);
    let result = cleaner.normalize_whitespace(&result);

    assert!(result.contains("[TIMESTAMP]"));
    assert!(result.contains("[RELATIVE_TIME]"));
    assert!(!result.contains("[32m"));
    assert!(!result.contains("[33m"));
    assert!(result.contains("node_modules"));
}

// ============================================================================
// Edge Cases and Special Scenarios
// ============================================================================

#[test]
fn test_empty_input() {
    let cleaner = LogCleaner::new();
    assert_eq!(cleaner.remove_timestamps(""), "");
    assert_eq!(cleaner.remove_ansi_codes(""), "");
    assert_eq!(cleaner.normalize_paths(""), "");
    assert_eq!(cleaner.normalize_whitespace(""), "");
}

#[test]
fn test_no_matches() {
    let cleaner = LogCleaner::new();
    let text = "Simple text with no special content";
    assert_eq!(cleaner.remove_timestamps(text), text);
    assert_eq!(cleaner.remove_ansi_codes(text), text);
    assert_eq!(cleaner.normalize_paths(text), text);
    assert_eq!(cleaner.normalize_whitespace(text), text);
}

#[test]
fn test_only_whitespace() {
    let cleaner = LogCleaner::new();
    let text = "     \n\n\n\n     ";
    let result = cleaner.normalize_whitespace(text);
    assert_eq!(result, "");
}

#[test]
fn test_unicode_content_preserved() {
    let cleaner = LogCleaner::new();
    let text = "Unicode: 你好 мир 🚀";
    assert_eq!(cleaner.remove_timestamps(text), text);
    assert_eq!(cleaner.remove_ansi_codes(text), text);
}

#[test]
fn test_special_characters_preserved() {
    let cleaner = LogCleaner::new();
    let text = "Special: @#$%^&*()_+-=[]{}|;:,.<>?/~`";
    assert_eq!(cleaner.remove_timestamps(text), text);
}

#[test]
fn test_very_long_input() {
    let cleaner = LogCleaner::new();
    let text = "A".repeat(10000);
    let result = cleaner.normalize_whitespace(&text);
    assert_eq!(result.len(), 10000);
}

#[test]
fn test_clean_execution_log_preserves_metadata() {
    let cleaner = LogCleaner::new();
    let log = TestExecutionLog {
        test_case_id: "TC123".to_string(),
        sequence_id: 42,
        timestamp: "2024-01-15T10:30:45Z".to_string(),
        actual_output: "output".to_string(),
        actual_success: true,
        duration_ms: 5000,
        error_message: None,
    };

    let cleaned = cleaner.clean_execution_log(&log);

    // Metadata should be preserved
    assert_eq!(cleaned.test_case_id, "TC123");
    assert_eq!(cleaned.sequence_id, 42);
    assert!(cleaned.actual_success);
    assert_eq!(cleaned.duration_ms, 5000);
}

#[test]
fn test_default_constructor() {
    let cleaner1 = LogCleaner::new();
    let cleaner2 = LogCleaner::default();

    let text = "Test at 2024-01-15T10:30:45Z";
    assert_eq!(
        cleaner1.remove_timestamps(text),
        cleaner2.remove_timestamps(text)
    );
}
