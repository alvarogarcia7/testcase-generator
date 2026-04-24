use crate::models::*;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub struct HtmlReportGenerator;

impl HtmlReportGenerator {
    pub fn generate(report: &CoverageReport, template_path: Option<&Path>) -> Result<String> {
        if let Some(template) = template_path {
            Self::generate_from_template(report, template)
        } else {
            Self::generate_default(report)
        }
    }

    fn generate_from_template(report: &CoverageReport, template_path: &Path) -> Result<String> {
        let template = fs::read_to_string(template_path).context(format!(
            "Failed to read HTML template file: {:?}",
            template_path
        ))?;

        let html = template
            .replace(
                "{{GENERATED_AT}}",
                &report
                    .generated_at
                    .format("%Y-%m-%d %H:%M:%S UTC")
                    .to_string(),
            )
            .replace(
                "{{TOTAL_REQUIREMENTS}}",
                &report.total_requirements.to_string(),
            )
            .replace(
                "{{FULLY_COVERED}}",
                &report.fully_covered_requirements.to_string(),
            )
            .replace(
                "{{PARTIALLY_COVERED}}",
                &report.partially_covered_requirements.to_string(),
            )
            .replace("{{UNCOVERED}}", &report.uncovered_requirements.to_string())
            .replace(
                "{{REQUIREMENTS_ROWS}}",
                &Self::generate_requirements_rows(report),
            );

        Ok(html)
    }

    fn generate_default(report: &CoverageReport) -> Result<String> {
        const DEFAULT_TEMPLATE: &str = include_str!("../templates/default_report.html");

        let html = DEFAULT_TEMPLATE
            .replace(
                "{{GENERATED_AT}}",
                &report
                    .generated_at
                    .format("%Y-%m-%d %H:%M:%S UTC")
                    .to_string(),
            )
            .replace(
                "{{TOTAL_REQUIREMENTS}}",
                &report.total_requirements.to_string(),
            )
            .replace(
                "{{FULLY_COVERED}}",
                &report.fully_covered_requirements.to_string(),
            )
            .replace(
                "{{PARTIALLY_COVERED}}",
                &report.partially_covered_requirements.to_string(),
            )
            .replace("{{UNCOVERED}}", &report.uncovered_requirements.to_string())
            .replace(
                "{{REQUIREMENTS_ROWS}}",
                &Self::generate_requirements_rows(report),
            );

        Ok(html)
    }

    fn generate_requirements_rows(report: &CoverageReport) -> String {
        report
            .requirements
            .iter()
            .enumerate()
            .map(|(idx, req)| {
                let coverage_type_str = match req.coverage_type {
                    CoverageType::Full => "Full",
                    CoverageType::Partial => "Partial",
                };

                let status_color = req.status.color();
                let status_text = req.status.display_name();

                let test_count = req.test_cases.len();
                let passed_count = req
                    .test_cases
                    .iter()
                    .filter(|tc| tc.status == TestStatus::Pass)
                    .count();
                let failed_count = req
                    .test_cases
                    .iter()
                    .filter(|tc| {
                        tc.status == TestStatus::Fail || tc.status == TestStatus::NotExecuted
                    })
                    .count();

                let details_id = format!("details-{}", idx);

                let requirement_text_html = if let Some(ref req_text) = req.requirement_text {
                    let highlighted = Self::highlight_requirement_text(
                        req_text,
                        &req.coverage_type,
                        req.covered_portions_passing.as_deref(),
                        req.covered_portions_failing.as_deref(),
                    );
                    format!(
                        "<div class=\"requirement-text\">\
                            <strong>Requirement Text:</strong><br/>{}\
                        </div>",
                        highlighted
                    )
                } else {
                    String::new()
                };

                let covered_portions_html = if let Some(ref portions) = req.covered_portions {
                    if !portions.is_empty() {
                        let portions_list: Vec<String> = portions
                            .iter()
                            .map(|p| format!("<li>{}</li>", html_escape(p)))
                            .collect();
                        format!(
                            "<div class=\"covered-portions\">\
                                <strong>Covered Portions:</strong>\
                                <ul>{}</ul>\
                            </div>",
                            portions_list.join("")
                        )
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };

                let coverage_errors_html = if let Some(ref errors) = req.coverage_errors {
                    if !errors.is_empty() {
                        let errors_list: Vec<String> = errors
                            .iter()
                            .map(|e| format!("<li class=\"error\">{}</li>", html_escape(e)))
                            .collect();
                        format!(
                            "<div class=\"coverage-errors\">\
                                <strong>Coverage Errors:</strong>\
                                <ul>{}</ul>\
                            </div>",
                            errors_list.join("")
                        )
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };

                let test_cases_html = if req.test_cases.is_empty() {
                    String::from("<div class=\"test-cases\"><em>No test cases</em></div>")
                } else {
                    let items: Vec<String> = req
                        .test_cases
                        .iter()
                        .map(|tc| {
                            let status_class = match tc.status {
                                TestStatus::Pass => "pass",
                                TestStatus::Fail => "fail",
                                TestStatus::NotExecuted => "not-executed",
                            };

                            let covers_html = if let Some(ref covers) = tc.covers {
                                format!(
                                    "<span class=\"test-case-covers\">Covers: {}</span>",
                                    html_escape(covers)
                                )
                            } else {
                                String::new()
                            };

                            let description_html = if let Some(ref desc) = tc.description {
                                format!(
                                    "<span class=\"test-case-description\">{}</span>",
                                    html_escape(desc)
                                )
                            } else {
                                String::new()
                            };

                            format!(
                                "<div class=\"test-case-item {}\">\
                                    <span class=\"test-case-id\">{}</span>{}{}\
                                </div>",
                                status_class,
                                html_escape(&tc.test_case_id),
                                description_html,
                                covers_html
                            )
                        })
                        .collect();

                    format!("<div class=\"test-cases\">{}</div>", items.join(""))
                };

                format!(
                    "<tr>\
                        <td class=\"expandable\" data-details=\"{}\">{}</td>\
                        <td><span class=\"coverage-type\">{}</span></td>\
                        <td><span class=\"status-badge {}\">{}</span></td>\
                        <td>{}</td>\
                        <td>{} / {}</td>\
                    </tr>\
                    <tr>\
                        <td colspan=\"5\" class=\"details\" id=\"{}\">\
                            {}{}{}{}\
                        </td>\
                    </tr>",
                    details_id,
                    html_escape(&req.requirement_id),
                    coverage_type_str,
                    status_color,
                    status_text,
                    test_count,
                    passed_count,
                    failed_count,
                    details_id,
                    requirement_text_html,
                    covered_portions_html,
                    coverage_errors_html,
                    test_cases_html
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn highlight_requirement_text(
        req_text: &str,
        coverage_type: &CoverageType,
        passing_portions: Option<&[String]>,
        failing_portions: Option<&[String]>,
    ) -> String {
        if req_text.is_empty() {
            return String::new();
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        enum CoverageState {
            Uncovered,
            CoveredPassing,
            CoveredFailing,
        }

        // Work with byte positions to match with str::find()
        let mut coverage_map: Vec<CoverageState> = vec![CoverageState::Uncovered; req_text.len()];

        // If coverage_type is Full, mark everything as passing by default
        if *coverage_type == CoverageType::Full {
            for item in coverage_map.iter_mut() {
                *item = CoverageState::CoveredPassing;
            }
        } else {
            // First mark all passing portions (green)
            if let Some(portions) = passing_portions {
                for portion in portions {
                    if portion.is_empty() {
                        continue;
                    }
                    let mut start = 0;
                    while let Some(pos) = req_text[start..].find(portion) {
                        let actual_pos = start + pos;
                        let end_pos = (actual_pos + portion.len()).min(req_text.len());
                        for item in coverage_map
                            .iter_mut()
                            .skip(actual_pos)
                            .take(end_pos - actual_pos)
                        {
                            *item = CoverageState::CoveredPassing;
                        }
                        start = actual_pos + 1;
                    }
                }
            }
        }

        // Then mark failing portions (orange) - these override passing
        if let Some(portions) = failing_portions {
            for portion in portions {
                if portion.is_empty() {
                    continue;
                }
                let mut start = 0;
                while let Some(pos) = req_text[start..].find(portion) {
                    let actual_pos = start + pos;
                    let end_pos = (actual_pos + portion.len()).min(req_text.len());
                    for item in coverage_map
                        .iter_mut()
                        .skip(actual_pos)
                        .take(end_pos - actual_pos)
                    {
                        *item = CoverageState::CoveredFailing;
                    }
                    start = actual_pos + 1;
                }
            }
        }

        // Build highlighted HTML
        let mut result = String::new();
        let mut current_state: Option<CoverageState> = None;
        let bytes = req_text.as_bytes();

        for (byte_idx, &byte) in bytes.iter().enumerate() {
            let state = coverage_map[byte_idx];

            // Close previous span if state changes
            if let Some(prev_state) = current_state {
                if state != prev_state {
                    result.push_str("</span>");
                    current_state = None;
                }
            }

            // Open new span if state changes or at start
            if current_state.is_none() || current_state != Some(state) {
                match state {
                    CoverageState::CoveredPassing => {
                        result.push_str("<span class=\"covered-passing\">");
                    }
                    CoverageState::CoveredFailing => {
                        result.push_str("<span class=\"covered-failing\">");
                    }
                    CoverageState::Uncovered => {
                        result.push_str("<span class=\"uncovered\">");
                    }
                }
                current_state = Some(state);
            }

            // Add the character (escaped)
            let ch = byte as char;
            let escaped_char = match ch {
                '&' => "&amp;",
                '<' => "&lt;",
                '>' => "&gt;",
                '"' => "&quot;",
                '\'' => "&#39;",
                _ => {
                    result.push(ch);
                    continue;
                }
            };
            result.push_str(escaped_char);
        }

        // Close final span
        if current_state.is_some() {
            result.push_str("</span>");
        }

        result
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
