use anyhow::Result;
use testcase_manager::{FuzzySearchResult, TestCaseFuzzyFinder};

fn main() -> Result<()> {
    println!("TTY Fallback Demo");
    println!("=================\n");
    println!("This example demonstrates the TTY detection and fallback mechanism.");
    println!("When running in a non-TTY environment (e.g., VS Code debug console),");
    println!("the fuzzy finder will automatically switch to numbered selection.\n");

    let options = vec![
        "Option A: First choice".to_string(),
        "Option B: Second choice".to_string(),
        "Option C: Third choice".to_string(),
        "Option D: Fourth choice".to_string(),
        "Option E: Fifth choice".to_string(),
    ];

    println!("=== Single Selection Demo ===\n");
    match TestCaseFuzzyFinder::search_strings(&options, "Select an option:")? {
        FuzzySearchResult::Selected(selected) => {
            println!("\n✓ You selected: {}", selected);
        }
        FuzzySearchResult::Cancelled => {
            println!("\n✗ Selection cancelled");
        }
        FuzzySearchResult::Error(e) => {
            println!("\n✗ Error during selection: {}", e);
        }
    }

    println!("\n=== Multi-Selection Demo ===\n");
    let multi_options = vec![
        "Feature 1: Authentication".to_string(),
        "Feature 2: Authorization".to_string(),
        "Feature 3: Logging".to_string(),
        "Feature 4: Monitoring".to_string(),
        "Feature 5: Testing".to_string(),
    ];

    let selected_items =
        TestCaseFuzzyFinder::multi_select(&multi_options, "Select multiple features:")?;

    if selected_items.is_empty() {
        println!("\n✗ No items selected");
    } else {
        println!("\n✓ You selected {} item(s):", selected_items.len());
        for item in selected_items {
            println!("  - {}", item);
        }
    }

    println!("\n✓ Demo completed!");
    Ok(())
}
