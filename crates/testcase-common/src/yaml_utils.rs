/// Utility functions for YAML error handling
///
/// Log detailed YAML parse error information
pub fn log_yaml_parse_error(error: &serde_yaml::Error, yaml_content: &str, file_name: &str) {
    log::error!("YAML parsing error in file '{}': {}", file_name, error);

    if let Some(location) = error.location() {
        let line = location.line();
        let column = location.column();

        log::error!("Error location: line {}, column {}", line, column);

        // Extract the line with the error and surrounding context
        let lines: Vec<&str> = yaml_content.lines().collect();
        let line_idx = line.saturating_sub(1);

        if line_idx < lines.len() {
            // Show context: 2 lines before, the error line, and 2 lines after
            let start = line_idx.saturating_sub(2);
            let end = (line_idx + 3).min(lines.len());

            log::error!("Context:");
            for (idx, line_content) in lines[start..end].iter().enumerate() {
                let actual_line = start + idx + 1;
                if actual_line == line {
                    log::error!("  >>> {}: {}", actual_line, line_content);
                    // Show column pointer
                    if column > 0 {
                        let pointer = format!(
                            "  >>> {}^",
                            " ".repeat(actual_line.to_string().len() + column)
                        );
                        log::error!("{}", pointer);
                    }
                } else {
                    log::error!("      {}: {}", actual_line, line_content);
                }
            }
        }
    } else {
        // If no location info, log the first few lines for context
        log::error!("YAML content (first 5 lines):");
        for (idx, line) in yaml_content.lines().take(5).enumerate() {
            log::error!("  {}: {}", idx + 1, line);
        }
    }
}
