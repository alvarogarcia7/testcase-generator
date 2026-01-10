/// UI utilities for printing formatted titles and sections
/// Style for printing titles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TitleStyle {
    /// Triple equals style: === Title ===
    TripleEquals,
    /// Box style with Unicode characters: ╔═══╗
    Box,
    /// Simple equals style: =================
    SimpleEquals,
}

/// Print a formatted title with the specified style
///
/// # Arguments
/// * `title` - The title text to display
/// * `style` - The style to use for formatting
///
/// # Examples
/// ```
/// use testcase_manager::print_title;
/// use testcase_manager::ui::TitleStyle;
///
/// print_title("Section Title", TitleStyle::TripleEquals);
/// print_title("Flow Title", TitleStyle::Box);
/// ```
pub fn print_title(title: &str, style: TitleStyle) {
    match style {
        TitleStyle::TripleEquals => {
            println!("\n=== {} ===\n", title);
        }
        TitleStyle::Box => {
            let title_len = title.len();
            let padding = 4;
            let box_width = title_len + padding * 2;
            let padding_str = "═".repeat(box_width);
            let inner_padding = " ".repeat(padding);

            println!("\n╔{}╗", padding_str);
            println!("║{}{}{} ║", inner_padding, title, inner_padding);
            println!("╚{}╝\n", padding_str);
        }
        TitleStyle::SimpleEquals => {
            let separator = "=".repeat(title.len().max(50));
            println!("{}", title);
            println!("{}\n", separator);
        }
    }
}

/// Print a section title using the triple equals style (for log::info)
///
/// This is a convenience function for the most common use case in prompts
pub fn print_section_title(title: &str) {
    log::info!("\n=== {} ===\n", title);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_title_triple_equals() {
        print_title("Test Title", TitleStyle::TripleEquals);
    }

    #[test]
    fn test_print_title_box() {
        print_title("Test Title", TitleStyle::Box);
    }

    #[test]
    fn test_print_title_simple_equals() {
        print_title("Test Title", TitleStyle::SimpleEquals);
    }

    #[test]
    fn test_print_section_title() {
        print_section_title("Test Section");
    }
}
