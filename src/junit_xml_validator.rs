use anyhow::{Context, Result};

/// Validates JUnit XML against the Maven Surefire XSD schema requirements.
///
/// This function validates the XML structure and constraints defined in:
/// https://maven.apache.org/surefire/maven-surefire-plugin/xsd/surefire-test-report.xsd
///
/// Key validations performed:
/// - Root element must be <testsuite>
/// - Required attributes: name, tests, failures, skipped, time
/// - All numeric attributes must be valid numbers
/// - Number of <testcase> elements must match the tests attribute
/// - Each <testcase> must have name and time attributes
/// - Count of <failure> elements must match failures attribute
/// - Count of <skipped> elements must match skipped attribute
pub fn validate_junit_xml(xml: &str) -> Result<()> {
    use roxmltree::Document;

    let doc = Document::parse(xml).context("Failed to parse XML")?;

    let root = doc.root_element();

    if root.tag_name().name() != "testsuite" {
        anyhow::bail!(
            "Root element must be <testsuite>, found <{}>",
            root.tag_name().name()
        );
    }

    // Validate required attributes
    if !root.has_attribute("name") {
        anyhow::bail!("testsuite element is missing required 'name' attribute");
    }
    if !root.has_attribute("tests") {
        anyhow::bail!("testsuite element is missing required 'tests' attribute");
    }
    if !root.has_attribute("failures") {
        anyhow::bail!("testsuite element is missing required 'failures' attribute");
    }
    if !root.has_attribute("skipped") {
        anyhow::bail!("testsuite element is missing required 'skipped' attribute");
    }
    if !root.has_attribute("time") {
        anyhow::bail!("testsuite element is missing required 'time' attribute");
    }

    // Validate attribute types and values
    let tests_count = root
        .attribute("tests")
        .and_then(|v| v.parse::<usize>().ok())
        .context("'tests' attribute must be a valid non-negative integer")?;

    let failures_count = root
        .attribute("failures")
        .and_then(|v| v.parse::<usize>().ok())
        .context("'failures' attribute must be a valid non-negative integer")?;

    let skipped_count = root
        .attribute("skipped")
        .and_then(|v| v.parse::<usize>().ok())
        .context("'skipped' attribute must be a valid non-negative integer")?;

    let time_value = root
        .attribute("time")
        .and_then(|v| v.parse::<f64>().ok())
        .context("'time' attribute must be a valid number")?;

    if time_value < 0.0 {
        anyhow::bail!(
            "'time' attribute must be non-negative, found: {}",
            time_value
        );
    }

    // Count testcase elements
    let testcases: Vec<_> = root
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "testcase")
        .collect();

    if testcases.len() != tests_count {
        anyhow::bail!(
            "Number of <testcase> elements ({}) does not match 'tests' attribute ({})",
            testcases.len(),
            tests_count
        );
    }

    let mut actual_failures = 0;
    let mut actual_skipped = 0;

    // Validate each testcase element
    for (idx, testcase) in testcases.iter().enumerate() {
        if !testcase.has_attribute("name") {
            anyhow::bail!(
                "testcase element #{} is missing required 'name' attribute",
                idx + 1
            );
        }
        if !testcase.has_attribute("time") {
            anyhow::bail!(
                "testcase element #{} is missing required 'time' attribute",
                idx + 1
            );
        }

        let tc_time = testcase
            .attribute("time")
            .and_then(|v| v.parse::<f64>().ok())
            .context(format!(
                "testcase #{} 'time' attribute must be a valid number",
                idx + 1
            ))?;

        if tc_time < 0.0 {
            anyhow::bail!(
                "testcase #{} 'time' attribute must be non-negative, found: {}",
                idx + 1,
                tc_time
            );
        }

        let has_failure = testcase
            .children()
            .any(|n| n.is_element() && n.tag_name().name() == "failure");
        let has_skipped = testcase
            .children()
            .any(|n| n.is_element() && n.tag_name().name() == "skipped");

        if has_failure {
            actual_failures += 1;
        }
        if has_skipped {
            actual_skipped += 1;
        }

        // Validate failure and error elements
        for child in testcase.children().filter(|n| n.is_element()) {
            match child.tag_name().name() {
                "failure" => {
                    if !child.has_attribute("message") && child.text().is_none() {
                        anyhow::bail!(
                            "testcase #{} <failure> element must have 'message' attribute or text content",
                            idx + 1
                        );
                    }
                }
                "error" => {
                    if !child.has_attribute("message") && child.text().is_none() {
                        anyhow::bail!(
                            "testcase #{} <error> element must have 'message' attribute or text content",
                            idx + 1
                        );
                    }
                }
                _ => {}
            }
        }
    }

    // Validate counts match
    if actual_failures != failures_count {
        anyhow::bail!(
            "Number of <failure> elements ({}) does not match 'failures' attribute ({})",
            actual_failures,
            failures_count
        );
    }

    if actual_skipped != skipped_count {
        anyhow::bail!(
            "Number of <skipped> elements ({}) does not match 'skipped' attribute ({})",
            actual_skipped,
            skipped_count
        );
    }

    Ok(())
}
