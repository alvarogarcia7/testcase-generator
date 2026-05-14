use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::html::HtmlReportGenerator;
use crate::models::CoverageReport;

pub struct ReportGenerator;

impl ReportGenerator {
    pub fn load_coverage_report(input_path: &Path) -> Result<CoverageReport> {
        log::info!("Loading coverage report from: {:?}", input_path);

        let content = fs::read_to_string(input_path)
            .context(format!("Failed to read coverage report: {:?}", input_path))?;

        let report: CoverageReport =
            serde_json::from_str(&content).context("Failed to parse JSON coverage report")?;

        log::debug!(
            "Loaded coverage report with {} requirements",
            report.total_requirements
        );

        Ok(report)
    }

    pub fn generate_html(
        report: &CoverageReport,
        output_dir: &Path,
        template_path: Option<&Path>,
    ) -> Result<()> {
        log::info!("Generating HTML report to: {:?}", output_dir);

        if let Some(template) = template_path {
            log::info!("Using custom HTML template: {:?}", template);
        } else {
            log::info!("Using default HTML template");
        }

        fs::create_dir_all(output_dir).context(format!(
            "Failed to create output directory: {:?}",
            output_dir
        ))?;

        let html_content = HtmlReportGenerator::generate(report, template_path)?;

        let index_path = output_dir.join("index.html");
        fs::write(&index_path, html_content)
            .context(format!("Failed to write HTML file: {:?}", index_path))?;

        log::info!("HTML report written to: {:?}", index_path);

        Ok(())
    }

    pub fn save_coverage_report(report: &CoverageReport, output_path: &Path) -> Result<()> {
        log::info!("Saving coverage report to: {:?}", output_path);

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .context(format!("Failed to create output directory: {:?}", parent))?;
        }

        let json_content = serde_json::to_string_pretty(report)
            .context("Failed to serialize coverage report to JSON")?;

        fs::write(output_path, json_content)
            .context(format!("Failed to write JSON file: {:?}", output_path))?;

        log::info!("Coverage report written to: {:?}", output_path);

        Ok(())
    }
}
