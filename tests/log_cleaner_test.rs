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
    let text = "File at /home/user/project/src/main_editor";
    let result = cleaner.normalize_paths(text);
    // Should extract just the filename
    assert!(result.contains("main_editor"));
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
    let text = r"File at C:\Users\user\project\src\main_editor.rs";
    let result = cleaner.normalize_paths(text);
    assert!(result.contains("main_editor"));
}

#[test]
fn test_normalize_paths_windows_forward_slash() {
    let cleaner = LogCleaner::new();
    let text = "File at C:/Users/user/project/src/main_editor";
    let result = cleaner.normalize_paths(text);
    assert!(result.contains("main_editor"));
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
    let text = "Relative: src/main_editor and ./test.rs";
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
 /home/user/project/src/main_editor | 10 [31m----------[0m
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
    assert!(result.contains("main_editor"));
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

// ============================================================================
// Backspace Processing Tests - Basic Functionality
// ============================================================================

#[test]
fn test_process_backspaces_single_backspace() {
    let cleaner = LogCleaner::new();
    let text = "hello\x08";
    let result = cleaner.process_backspaces(text);
    assert_eq!(result, "hell");
}

#[test]
fn test_process_backspaces_middle_of_text() {
    let cleaner = LogCleaner::new();
    let text = "helo\x08lo";
    let result = cleaner.process_backspaces(text);
    assert_eq!(result, "hello");
}

#[test]
fn test_process_backspaces_delete_character() {
    let cleaner = LogCleaner::new();
    // \x7f is DEL character
    let text = "hello\x7f";
    let result = cleaner.process_backspaces(text);
    assert_eq!(result, "hell");
}

#[test]
fn test_process_backspaces_mixed_backspace_and_delete() {
    let cleaner = LogCleaner::new();
    let text = "hello\x08\x7f world";
    let result = cleaner.process_backspaces(text);
    assert_eq!(result, "hel world");
}

// ============================================================================
// Backspace Processing Tests - Consecutive Backspaces
// ============================================================================

#[test]
fn test_process_backspaces_consecutive_two() {
    let cleaner = LogCleaner::new();
    let text = "hello\x08\x08";
    let result = cleaner.process_backspaces(text);
    assert_eq!(result, "hel");
}

#[test]
fn test_process_backspaces_consecutive_three() {
    let cleaner = LogCleaner::new();
    let text = "hello\x08\x08\x08";
    let result = cleaner.process_backspaces(text);
    assert_eq!(result, "he");
}

#[test]
fn test_process_backspaces_consecutive_many() {
    let cleaner = LogCleaner::new();
    let text = "hello world\x08\x08\x08\x08\x08";
    let result = cleaner.process_backspaces(text);
    assert_eq!(result, "hello ");
}

#[test]
fn test_process_backspaces_consecutive_with_retyping() {
    let cleaner = LogCleaner::new();
    // Simulates typing "test", backspacing 4 times, then typing "best"
    let text = "test\x08\x08\x08\x08best";
    let result = cleaner.process_backspaces(text);
    assert_eq!(result, "best");
}

#[test]
fn test_process_backspaces_multiple_consecutive_groups() {
    let cleaner = LogCleaner::new();
    let text = "hello\x08\x08 world\x08\x08\x08";
    let result = cleaner.process_backspaces(text);
    assert_eq!(result, "hel wo");
}

// ============================================================================
// Backspace Processing Tests - Backspace at Beginning
// ============================================================================

#[test]
fn test_process_backspaces_at_beginning_single() {
    let cleaner = LogCleaner::new();
    let text = "\x08hello";
    let result = cleaner.process_backspaces(text);
    // Backspace at beginning should be no-op (nothing to delete)
    assert_eq!(result, "hello");
}

#[test]
fn test_process_backspaces_at_beginning_multiple() {
    let cleaner = LogCleaner::new();
    let text = "\x08\x08\x08hello";
    let result = cleaner.process_backspaces(text);
    assert_eq!(result, "hello");
}

#[test]
fn test_process_backspaces_only_backspaces() {
    let cleaner = LogCleaner::new();
    let text = "\x08\x08\x08";
    let result = cleaner.process_backspaces(text);
    assert_eq!(result, "");
}

#[test]
fn test_process_backspaces_more_backspaces_than_characters() {
    let cleaner = LogCleaner::new();
    let text = "hi\x08\x08\x08\x08\x08world";
    let result = cleaner.process_backspaces(text);
    // Should delete "hi" and extra backspaces are no-ops
    assert_eq!(result, "world");
}

#[test]
fn test_process_backspaces_empty_string() {
    let cleaner = LogCleaner::new();
    let text = "";
    let result = cleaner.process_backspaces(text);
    assert_eq!(result, "");
}

#[test]
fn test_process_backspaces_exact_match() {
    let cleaner = LogCleaner::new();
    let text = "hello\x08\x08\x08\x08\x08";
    let result = cleaner.process_backspaces(text);
    // All characters deleted
    assert_eq!(result, "");
}

// ============================================================================
// Backspace Processing Tests - Realistic Terminal Scenarios
// ============================================================================

#[test]
fn test_process_backspaces_password_typing() {
    let cleaner = LogCleaner::new();
    // Simulates typing password with typos and corrections
    let text = "pasword\x08\x08sword";
    let result = cleaner.process_backspaces(text);
    assert_eq!(result, "password");
}

#[test]
fn test_process_backspaces_command_correction() {
    let cleaner = LogCleaner::new();
    // Simulates typing "git statsu" then correcting to "git status"
    let text = "git statsu\x08\x08us";
    let result = cleaner.process_backspaces(text);
    assert_eq!(result, "git status");
}

#[test]
fn test_process_backspaces_word_deletion() {
    let cleaner = LogCleaner::new();
    let text = "hello world wrong\x08\x08\x08\x08\x08right";
    let result = cleaner.process_backspaces(text);
    assert_eq!(result, "hello world right");
}

#[test]
fn test_process_backspaces_multiline_with_backspace() {
    let cleaner = LogCleaner::new();
    let text = "line1\nline2\x08\x082\nline3";
    let result = cleaner.process_backspaces(text);
    assert_eq!(result, "line1\nlin2\nline3");
}

// ============================================================================
// Control Character Removal Tests - Basic Control Characters
// ============================================================================

#[test]
fn test_remove_control_characters_null() {
    let cleaner = LogCleaner::new();
    let text = "hello\x00world";
    let result = cleaner.remove_control_characters(text);
    assert_eq!(result, "helloworld");
}

#[test]
fn test_remove_control_characters_bell() {
    let cleaner = LogCleaner::new();
    // \x07 is BEL (bell)
    let text = "hello\x07world";
    let result = cleaner.remove_control_characters(text);
    assert_eq!(result, "helloworld");
}

#[test]
fn test_remove_control_characters_backspace() {
    let cleaner = LogCleaner::new();
    // \x08 is BS (backspace) - should be removed by remove_control_characters
    let text = "hello\x08world";
    let result = cleaner.remove_control_characters(text);
    assert_eq!(result, "helloworld");
}

#[test]
fn test_remove_control_characters_escape() {
    let cleaner = LogCleaner::new();
    // \x1b is ESC (escape)
    let text = "hello\x1bworld";
    let result = cleaner.remove_control_characters(text);
    assert_eq!(result, "helloworld");
}

#[test]
fn test_remove_control_characters_delete() {
    let cleaner = LogCleaner::new();
    // \x7f is DEL
    let text = "hello\x7fworld";
    let result = cleaner.remove_control_characters(text);
    assert_eq!(result, "helloworld");
}

// ============================================================================
// Control Character Removal Tests - Preserved Characters
// ============================================================================

#[test]
fn test_remove_control_characters_preserves_newline() {
    let cleaner = LogCleaner::new();
    let text = "hello\nworld";
    let result = cleaner.remove_control_characters(text);
    assert_eq!(result, "hello\nworld");
}

#[test]
fn test_remove_control_characters_preserves_carriage_return() {
    let cleaner = LogCleaner::new();
    let text = "hello\rworld";
    let result = cleaner.remove_control_characters(text);
    assert_eq!(result, "hello\rworld");
}

#[test]
fn test_remove_control_characters_preserves_tab() {
    let cleaner = LogCleaner::new();
    let text = "hello\tworld";
    let result = cleaner.remove_control_characters(text);
    assert_eq!(result, "hello\tworld");
}

#[test]
fn test_remove_control_characters_preserves_mixed_whitespace() {
    let cleaner = LogCleaner::new();
    let text = "line1\nline2\tcolumn\rstart";
    let result = cleaner.remove_control_characters(text);
    assert_eq!(result, "line1\nline2\tcolumn\rstart");
}

// ============================================================================
// Control Character Removal Tests - Multiple Control Characters
// ============================================================================

#[test]
fn test_remove_control_characters_caret_h() {
    let cleaner = LogCleaner::new();
    // ^H is often used to represent backspace (0x08)
    let text = "hello\x08world\x08test";
    let result = cleaner.remove_control_characters(text);
    assert_eq!(result, "helloworldtest");
}

#[test]
fn test_remove_control_characters_caret_g() {
    let cleaner = LogCleaner::new();
    // ^G is bell (0x07)
    let text = "hello\x07world\x07test";
    let result = cleaner.remove_control_characters(text);
    assert_eq!(result, "helloworldtest");
}

#[test]
fn test_remove_control_characters_mixed_control_chars() {
    let cleaner = LogCleaner::new();
    let text = "hello\x00\x01\x02\x07\x08\x1b\x7fworld";
    let result = cleaner.remove_control_characters(text);
    assert_eq!(result, "helloworld");
}

#[test]
fn test_remove_control_characters_all_c0_controls() {
    let cleaner = LogCleaner::new();
    // Test various C0 control characters (excluding \n, \r, \t)
    let text = "test\x00\x01\x02\x03\x04\x05\x06\x07end";
    let result = cleaner.remove_control_characters(text);
    assert_eq!(result, "testend");
}

#[test]
fn test_remove_control_characters_form_feed() {
    let cleaner = LogCleaner::new();
    // \x0c is form feed
    let text = "page1\x0cpage2";
    let result = cleaner.remove_control_characters(text);
    assert_eq!(result, "page1page2");
}

#[test]
fn test_remove_control_characters_vertical_tab() {
    let cleaner = LogCleaner::new();
    // \x0b is vertical tab
    let text = "line1\x0bline2";
    let result = cleaner.remove_control_characters(text);
    assert_eq!(result, "line1line2");
}

// ============================================================================
// Control Character Removal Tests - Edge Cases
// ============================================================================

#[test]
fn test_remove_control_characters_empty_string() {
    let cleaner = LogCleaner::new();
    let text = "";
    let result = cleaner.remove_control_characters(text);
    assert_eq!(result, "");
}

#[test]
fn test_remove_control_characters_only_control_chars() {
    let cleaner = LogCleaner::new();
    let text = "\x00\x01\x02\x07\x08\x1b\x7f";
    let result = cleaner.remove_control_characters(text);
    assert_eq!(result, "");
}

#[test]
fn test_remove_control_characters_only_preserved_chars() {
    let cleaner = LogCleaner::new();
    let text = "\n\r\t";
    let result = cleaner.remove_control_characters(text);
    assert_eq!(result, "\n\r\t");
}

#[test]
fn test_remove_control_characters_unicode_preserved() {
    let cleaner = LogCleaner::new();
    let text = "hello\x07世界\x08test";
    let result = cleaner.remove_control_characters(text);
    assert_eq!(result, "hello世界test");
}

#[test]
fn test_remove_control_characters_printable_ascii() {
    let cleaner = LogCleaner::new();
    let text = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
    let result = cleaner.remove_control_characters(text);
    assert_eq!(result, text);
}

// ============================================================================
// Script Capture Cleaning Tests - Integration
// ============================================================================

#[test]
fn test_clean_script_capture_simple() {
    let cleaner = LogCleaner::new();
    let text = "hello world";
    let result = cleaner.clean_script_capture(text);
    assert_eq!(result, "hello world");
}

#[test]
fn test_clean_script_capture_with_ansi_codes() {
    let cleaner = LogCleaner::new();
    let text = "\x1b[31mError\x1b[0m message";
    let result = cleaner.clean_script_capture(text);
    assert_eq!(result, "Error message");
}

#[test]
fn test_clean_script_capture_with_backspaces() {
    let cleaner = LogCleaner::new();
    let text = "hello\x08\x08lo";
    let result = cleaner.clean_script_capture(text);
    assert_eq!(result, "hello");
}

#[test]
fn test_clean_script_capture_with_control_characters() {
    let cleaner = LogCleaner::new();
    let text = "hello\x07world\x00test";
    let result = cleaner.clean_script_capture(text);
    assert_eq!(result, "helloworldtest");
}

#[test]
fn test_clean_script_capture_combined_ansi_backspace_control() {
    let cleaner = LogCleaner::new();
    let text = "\x1b[32mSuccess\x08\x08ess\x1b[0m\x07 message\x00";
    let result = cleaner.clean_script_capture(text);
    assert_eq!(result, "Success message");
}

// ============================================================================
// Script Capture Cleaning Tests - Realistic Script Output
// ============================================================================

#[test]
fn test_clean_script_capture_password_prompt() {
    let cleaner = LogCleaner::new();
    // Simulates user typing password with backspace corrections
    let text = "Password: secrt\x08\x08ret\x07";
    let result = cleaner.clean_script_capture(text);
    assert_eq!(result, "Password: secret");
}

#[test]
fn test_clean_script_capture_progress_bar() {
    let cleaner = LogCleaner::new();
    // Simulates a progress bar with ANSI codes and carriage returns
    let text = "\x1b[32m[=====>    ]\x1b[0m 50%\r\x1b[32m[=========>]\x1b[0m 100%";
    let result = cleaner.clean_script_capture(text);
    assert!(result.contains("50%"));
    assert!(result.contains("100%"));
    assert!(!result.contains("\x1b["));
}

#[test]
fn test_clean_script_capture_interactive_command() {
    let cleaner = LogCleaner::new();
    // Simulates interactive command line with typo correction
    let text = "$ git statsu\x08\x08us\x07\nOn branch main";
    let result = cleaner.clean_script_capture(text);
    assert_eq!(result, "$ git status\nOn branch main");
}

#[test]
fn test_clean_script_capture_vim_commands() {
    let cleaner = LogCleaner::new();
    // Simulates vim-like editor output with control characters
    let text = "\x1b[2J\x1b[Hfile.txt\x07\n\x1b[32m-- INSERT --\x1b[0m";
    let result = cleaner.clean_script_capture(text);
    assert!(result.contains("file.txt"));
    assert!(result.contains("-- INSERT --"));
    assert!(!result.contains("\x1b["));
}

#[test]
fn test_clean_script_capture_terminal_colors_bold() {
    let cleaner = LogCleaner::new();
    let text = "\x1b[1;31mERROR:\x1b[0m File not found\x07";
    let result = cleaner.clean_script_capture(text);
    assert_eq!(result, "ERROR: File not found");
}

#[test]
fn test_clean_script_capture_bash_ps1_prompt() {
    let cleaner = LogCleaner::new();
    let text = "\x1b[32muser@host\x1b[0m:\x1b[34m~\x1b[0m$ ls\x08\x08pwd\ntest";
    let result = cleaner.clean_script_capture(text);
    assert!(result.contains("user@host"));
    assert!(result.contains("pwd"));
    assert!(!result.contains("\x1b["));
}

#[test]
fn test_clean_script_capture_multiline_with_corrections() {
    let cleaner = LogCleaner::new();
    let text = "Line 1\nLine 2 with tyop\x08\x08\x08po\nLine 3\x07";
    let result = cleaner.clean_script_capture(text);
    assert_eq!(result, "Line 1\nLine 2 with typo\nLine 3");
}

// ============================================================================
// Script Capture Cleaning Tests - Complex Realistic Scenarios
// ============================================================================

#[test]
fn test_clean_script_capture_npm_install() {
    let cleaner = LogCleaner::new();
    let text = "\x1b[32mnpm\x1b[0m install\n\x1b[33mwarn\x1b[0m deprecated package\x07\n\x1b[32madded\x1b[0m 123 packages";
    let result = cleaner.clean_script_capture(text);
    assert!(result.contains("npm install"));
    assert!(result.contains("warn deprecated package"));
    assert!(result.contains("added 123 packages"));
    assert!(!result.contains("\x1b["));
}

#[test]
fn test_clean_script_capture_compilation_errors() {
    let cleaner = LogCleaner::new();
    let text = "\x1b[1;31merror[E0308]\x1b[0m: mismatched types\n  --> src/main.rs:10:5\x07";
    let result = cleaner.clean_script_capture(text);
    assert!(result.contains("error[E0308]"));
    assert!(result.contains("mismatched types"));
    assert!(!result.contains("\x1b["));
    assert!(!result.contains("\x07"));
}

#[test]
fn test_clean_script_capture_test_output_with_status() {
    let cleaner = LogCleaner::new();
    let text = "test result: \x1b[32mok\x1b[0m. 5 passed\x08\x08\x08\x08\x08\x08\x08\x1b[31mFAILED\x1b[0m. 4 passed; 1 failed";
    let result = cleaner.clean_script_capture(text);
    // The backspaces should remove "5 passed" and it gets replaced
    assert!(result.contains("FAILED. 4 passed; 1 failed"));
    assert!(!result.contains("\x1b["));
}

#[test]
fn test_clean_script_capture_make_output() {
    let cleaner = LogCleaner::new();
    let text =
        "\x1b[32mCC\x1b[0m main.o\n\x1b[32mCC\x1b[0m utils.o\n\x1b[1;32mLD\x1b[0m program\x07";
    let result = cleaner.clean_script_capture(text);
    assert!(result.contains("CC main.o"));
    assert!(result.contains("CC utils.o"));
    assert!(result.contains("LD program"));
    assert!(!result.contains("\x1b["));
}

#[test]
fn test_clean_script_capture_curl_download_progress() {
    let cleaner = LogCleaner::new();
    let text =
        "\x1b[32m###\x1b[0m 30%\r\x1b[32m######\x1b[0m 60%\r\x1b[32m##########\x1b[0m 100%\x07";
    let result = cleaner.clean_script_capture(text);
    assert!(!result.contains("\x1b["));
    assert!(!result.contains("\x07"));
}

#[test]
fn test_clean_script_capture_ssh_output() {
    let cleaner = LogCleaner::new();
    let text =
        "\x07\x1b[?1034hWelcome to Ubuntu\n\x1b[32muser@remote\x1b[0m:~$ ls\x08\x08pwd\n/home/user";
    let result = cleaner.clean_script_capture(text);
    assert!(result.contains("Welcome to Ubuntu"));
    assert!(result.contains("user@remote"));
    assert!(result.contains("pwd"));
    assert!(!result.contains("\x1b["));
}

#[test]
fn test_clean_script_capture_docker_build() {
    let cleaner = LogCleaner::new();
    let text = "Step 1/5 : FROM rust:1.75\n\x1b[32m --->\x1b[0m abc123\x07\nStep 2/5 : WORKDIR /app\n\x1b[32m --->\x1b[0m def456";
    let result = cleaner.clean_script_capture(text);
    assert!(result.contains("Step 1/5"));
    assert!(result.contains("Step 2/5"));
    assert!(result.contains("abc123"));
    assert!(!result.contains("\x1b["));
}

#[test]
fn test_clean_script_capture_git_diff_colors() {
    let cleaner = LogCleaner::new();
    let text = "\x1b[1mdiff --git a/file.txt b/file.txt\x1b[0m\n\x1b[32m+added line\x1b[0m\n\x1b[31m-removed line\x1b[0m\x07";
    let result = cleaner.clean_script_capture(text);
    assert!(result.contains("diff --git"));
    assert!(result.contains("+added line"));
    assert!(result.contains("-removed line"));
    assert!(!result.contains("\x1b["));
}

#[test]
fn test_clean_script_capture_python_repl() {
    let cleaner = LogCleaner::new();
    let text = ">>> print('hello')\nhello\n>>> x = 5\x08\x0810\n>>> x\n10";
    let result = cleaner.clean_script_capture(text);
    assert!(result.contains(">>> print('hello')"));
    assert!(result.contains(">>> x = 10"));
    assert_eq!(result.matches(">>> x").count(), 2);
}

#[test]
fn test_clean_script_capture_empty_string() {
    let cleaner = LogCleaner::new();
    let text = "";
    let result = cleaner.clean_script_capture(text);
    assert_eq!(result, "");
}

#[test]
fn test_clean_script_capture_only_control_and_ansi() {
    let cleaner = LogCleaner::new();
    let text = "\x1b[31m\x1b[0m\x07\x08\x00";
    let result = cleaner.clean_script_capture(text);
    assert_eq!(result, "");
}

// ============================================================================
// Script Capture Cleaning Tests - Strip ANSI Escapes Library Integration
// ============================================================================

#[test]
fn test_clean_script_capture_complex_ansi_sequences() {
    let cleaner = LogCleaner::new();
    // Test that strip-ansi-escapes handles complex sequences
    let text = "\x1b[38;5;208mOrange\x1b[0m \x1b[48;5;21mBackground\x1b[0m";
    let result = cleaner.clean_script_capture(text);
    assert_eq!(result, "Orange Background");
}

#[test]
fn test_clean_script_capture_cursor_movement() {
    let cleaner = LogCleaner::new();
    let text = "Hello\x1b[5DWorld";
    let result = cleaner.clean_script_capture(text);
    assert!(result.contains("Hello"));
    assert!(result.contains("World"));
}

#[test]
fn test_clean_script_capture_clear_sequences() {
    let cleaner = LogCleaner::new();
    let text = "\x1b[2JScreen cleared\x1b[H\x1b[KLine erased";
    let result = cleaner.clean_script_capture(text);
    assert!(result.contains("Screen cleared"));
    assert!(result.contains("Line erased"));
}

#[test]
fn test_clean_script_capture_save_restore_cursor() {
    let cleaner = LogCleaner::new();
    let text = "Position\x1b[s saved\x1b[u restored";
    let result = cleaner.clean_script_capture(text);
    assert!(result.contains("Position"));
    assert!(result.contains("saved"));
    assert!(result.contains("restored"));
}

// ============================================================================
// Script Capture Cleaning Tests - Edge Cases with Unicode
// ============================================================================

#[test]
fn test_clean_script_capture_unicode_with_backspace() {
    let cleaner = LogCleaner::new();
    let text = "Hello 世界\x08\x08界世";
    let result = cleaner.clean_script_capture(text);
    assert_eq!(result, "Hello 界世世");
}

#[test]
fn test_clean_script_capture_emoji_with_ansi() {
    let cleaner = LogCleaner::new();
    let text = "\x1b[32m✓\x1b[0m Success! 🎉\x07";
    let result = cleaner.clean_script_capture(text);
    assert_eq!(result, "✓ Success! 🎉");
}

#[test]
fn test_clean_script_capture_mixed_languages() {
    let cleaner = LogCleaner::new();
    let text = "Hello\x07 мир\x08\x08ир world\x08\x08rld 世界\x1b[0m";
    let result = cleaner.clean_script_capture(text);
    assert!(result.contains("Hello"));
    assert!(result.contains("мир"));
    assert!(result.contains("world"));
    assert!(result.contains("世界"));
}

// ============================================================================
// Script Capture Cleaning Tests - Performance and Large Inputs
// ============================================================================

#[test]
fn test_clean_script_capture_long_text() {
    let cleaner = LogCleaner::new();
    let text = "A".repeat(1000) + "\x08" + &"B".repeat(1000);
    let result = cleaner.clean_script_capture(&text);
    assert_eq!(result.len(), 1999); // 1000 A's minus 1, plus 1000 B's
}

#[test]
fn test_clean_script_capture_many_ansi_codes() {
    let cleaner = LogCleaner::new();
    let mut text = String::new();
    for i in 0..100 {
        text.push_str(&format!("\x1b[{}mText{}\x1b[0m ", 30 + (i % 8), i));
    }
    let result = cleaner.clean_script_capture(&text);
    assert!(!result.contains("\x1b["));
    for i in 0..100 {
        assert!(result.contains(&format!("Text{}", i)));
    }
}

#[test]
fn test_clean_script_capture_many_backspaces() {
    let cleaner = LogCleaner::new();
    let text = "Start".to_owned() + &"\x08".repeat(3) + "End";
    let result = cleaner.clean_script_capture(&text);
    assert_eq!(result, "StEnd");
}

// ============================================================================
// Integration Tests - Full Pipeline with Script Capture
// ============================================================================

#[test]
fn test_integration_realistic_terminal_session() {
    let cleaner = LogCleaner::new();
    let output = "\x1b[32muser@host\x1b[0m:~$ cd project\x07
\x1b[32muser@host\x1b[0m:~/project$ ls\x08\x08pwd
/home/user/project
\x1b[32muser@host\x1b[0m:~/project$ git status\x08\x08\x08\x08\x08\x08\x08log --oneline
\x1b[33m1234567\x1b[0m Initial commit
\x1b[33m8901234\x1b[0m Add feature\x07";

    let result = cleaner.clean_script_capture(output);

    // Should preserve structure
    assert!(result.contains("user@host"));
    assert!(result.contains("pwd"));
    assert!(result.contains("/home/user/project"));
    assert!(result.contains("git log --oneline"));
    assert!(result.contains("Initial commit"));

    // Should remove ANSI codes and control characters
    assert!(!result.contains("\x1b["));
    assert!(!result.contains("\x07"));
}

#[test]
fn test_integration_build_output_with_errors() {
    let cleaner = LogCleaner::new();
    let output = "\x1b[32mCompiling\x1b[0m myapp v0.1.0
\x1b[1;31merror\x1b[0m: unused variable: `x`\x07
 --> src/main.rs:5:9
  |
5 | let x = 10;\x08\x08\x08\x0810;
  |     ^ help: consider using: `_x`

\x1b[31merror\x1b[0m: aborting due to previous error";

    let result = cleaner.clean_script_capture(output);

    assert!(result.contains("Compiling myapp"));
    assert!(result.contains("error: unused variable: `x`"));
    assert!(result.contains("let x = 10;"));
    assert!(result.contains("error: aborting"));
    assert!(!result.contains("\x1b["));
}

#[test]
fn test_integration_interactive_installer() {
    let cleaner = LogCleaner::new();
    let output = "\x1b[1mWelcome to Installer\x1b[0m\x07

Please enter your name: John Doe\x08\x08\x08e
Installation path: /opt/app\x00
\x1b[32m[✓]\x1b[0m Installing dependencies...
\x1b[32m[✓]\x1b[0m Configuration complete\x07

\x1b[1;32mInstallation successful!\x1b[0m";

    let result = cleaner.clean_script_capture(output);

    assert!(result.contains("Welcome to Installer"));
    assert!(result.contains("John Doe"));
    assert!(result.contains("/opt/app"));
    assert!(result.contains("[✓] Installing dependencies"));
    assert!(result.contains("Installation successful!"));
    assert!(!result.contains("\x1b["));
    assert!(!result.contains("\x00"));
    assert!(!result.contains("\x07"));
}
