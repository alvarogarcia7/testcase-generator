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
        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Requirement Coverage Report</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            background: #f5f5f5;
            padding: 20px;
            line-height: 1.6;
        }}
        
        .container {{
            max-width: 1400px;
            margin: 0 auto;
            background: white;
            padding: 30px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }}
        
        h1 {{
            color: #333;
            margin-bottom: 10px;
            font-size: 2rem;
        }}
        
        .timestamp {{
            color: #666;
            font-size: 0.9rem;
            margin-bottom: 30px;
        }}
        
        .dashboard {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-bottom: 40px;
        }}
        
        .stat-card {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 20px;
            border-radius: 8px;
            text-align: center;
        }}
        
        .stat-card.total {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        }}
        
        .stat-card.full {{
            background: linear-gradient(135deg, #11998e 0%, #38ef7d 100%);
        }}
        
        .stat-card.partial {{
            background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
        }}
        
        .stat-card.uncovered {{
            background: linear-gradient(135deg, #4b4b4b 0%, #757575 100%);
        }}
        
        .stat-number {{
            font-size: 2.5rem;
            font-weight: bold;
            margin-bottom: 5px;
        }}
        
        .stat-label {{
            font-size: 0.9rem;
            opacity: 0.9;
        }}
        
        .requirements-table {{
            width: 100%;
            border-collapse: collapse;
            margin-top: 20px;
        }}
        
        .requirements-table th {{
            background: #f8f9fa;
            padding: 12px;
            text-align: left;
            border-bottom: 2px solid #dee2e6;
            font-weight: 600;
            color: #495057;
        }}
        
        .requirements-table td {{
            padding: 12px;
            border-bottom: 1px solid #dee2e6;
        }}
        
        .requirements-table tr:hover {{
            background: #f8f9fa;
        }}
        
        .status-badge {{
            display: inline-block;
            padding: 4px 12px;
            border-radius: 12px;
            font-size: 0.85rem;
            font-weight: 500;
        }}
        
        .status-badge.green {{
            background: #d4edda;
            color: #155724;
        }}
        
        .status-badge.red {{
            background: #f8d7da;
            color: #721c24;
        }}
        
        .status-badge.yellow {{
            background: #fff3cd;
            color: #856404;
        }}
        
        .status-badge.orange {{
            background: #ffe5d0;
            color: #8a4700;
        }}
        
        .status-badge.gray {{
            background: #e2e3e5;
            color: #383d41;
        }}
        
        .coverage-type {{
            display: inline-block;
            padding: 2px 8px;
            border-radius: 4px;
            font-size: 0.8rem;
            font-weight: 500;
            background: #e9ecef;
            color: #495057;
        }}
        
        .test-cases {{
            margin-top: 10px;
            padding-left: 20px;
        }}
        
        .test-case-item {{
            padding: 8px;
            margin: 4px 0;
            background: #f8f9fa;
            border-left: 3px solid #dee2e6;
            border-radius: 4px;
            font-size: 0.9rem;
        }}
        
        .test-case-item.pass {{
            border-left-color: #28a745;
        }}
        
        .test-case-item.fail {{
            border-left-color: #dc3545;
        }}
        
        .test-case-item.not-executed {{
            border-left-color: #6c757d;
        }}
        
        .test-case-id {{
            font-weight: 600;
            color: #495057;
        }}
        
        .test-case-description {{
            color: #6c757d;
            margin-left: 8px;
        }}
        
        .test-case-covers {{
            display: block;
            color: #007bff;
            font-style: italic;
            margin-top: 4px;
        }}
        
        .expandable {{
            cursor: pointer;
        }}
        
        .expandable::before {{
            content: '▶ ';
            display: inline-block;
            transition: transform 0.2s;
        }}
        
        .expandable.expanded::before {{
            transform: rotate(90deg);
        }}
        
        .details {{
            display: none;
        }}
        
        .details.show {{
            display: block;
        }}
        
        h2 {{
            color: #333;
            margin-top: 40px;
            margin-bottom: 20px;
            padding-bottom: 10px;
            border-bottom: 2px solid #dee2e6;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>📊 Requirement Coverage Report</h1>
        <div class="timestamp">Generated: {}</div>
        
        <div class="dashboard">
            <div class="stat-card total">
                <div class="stat-number">{}</div>
                <div class="stat-label">Total Requirements</div>
            </div>
            <div class="stat-card full">
                <div class="stat-number">{}</div>
                <div class="stat-label">Fully Covered</div>
            </div>
            <div class="stat-card partial">
                <div class="stat-number">{}</div>
                <div class="stat-label">Partially Covered</div>
            </div>
            <div class="stat-card uncovered">
                <div class="stat-number">{}</div>
                <div class="stat-label">Uncovered</div>
            </div>
        </div>
        
        <h2>Requirements Details</h2>
        
        <table class="requirements-table">
            <thead>
                <tr>
                    <th>Requirement ID</th>
                    <th>Coverage Type</th>
                    <th>Status</th>
                    <th>Test Cases</th>
                    <th>Pass/Fail</th>
                </tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>
    </div>
    
    <script>
        document.querySelectorAll('.expandable').forEach(el => {{
            el.addEventListener('click', function() {{
                this.classList.toggle('expanded');
                const detailsId = this.getAttribute('data-details');
                const details = document.getElementById(detailsId);
                details.classList.toggle('show');
            }});
        }});
    </script>
</body>
</html>"#,
            report.generated_at.format("%Y-%m-%d %H:%M:%S UTC"),
            report.total_requirements,
            report.fully_covered_requirements,
            report.partially_covered_requirements,
            report.uncovered_requirements,
            Self::generate_requirements_rows(report)
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
                    .filter(|tc| tc.status == TestStatus::Fail)
                    .count();

                let details_id = format!("details-{}", idx);

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
                        <td colspan=\"5\" class=\"details\" id=\"{}\">{}</td>\
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
                    test_cases_html
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
