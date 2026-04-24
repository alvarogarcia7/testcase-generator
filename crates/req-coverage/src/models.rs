use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RequirementDefinition {
    pub id: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RequirementDefinitions {
    pub requirements: Vec<RequirementDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CoverageType {
    Full,
    Partial,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RequirementCoverage {
    #[serde(rename = "type")]
    pub coverage_type: CoverageType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub covers: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum CoverageStatus {
    CoveredPass,
    CoveredFail,
    PartialCoveredPass,
    PartialCoveredFail,
    Uncovered,
}

impl CoverageStatus {
    pub fn color(&self) -> &'static str {
        match self {
            CoverageStatus::CoveredPass => "green",
            CoverageStatus::CoveredFail => "red",
            CoverageStatus::PartialCoveredPass => "yellow",
            CoverageStatus::PartialCoveredFail => "orange",
            CoverageStatus::Uncovered => "gray",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            CoverageStatus::CoveredPass => "Covered (All Passed)",
            CoverageStatus::CoveredFail => "Covered (Some Failed)",
            CoverageStatus::PartialCoveredPass => "Partially Covered (All Passed)",
            CoverageStatus::PartialCoveredFail => "Partially Covered (Some Failed)",
            CoverageStatus::Uncovered => "No Coverage",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TestCaseResult {
    pub test_case_id: String,
    pub status: TestStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub covers: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum TestStatus {
    Pass,
    Fail,
    NotExecuted,
}

impl TestStatus {
    #[allow(dead_code)]
    pub fn color(&self) -> &'static str {
        match self {
            TestStatus::Pass => "green",
            TestStatus::Fail => "red",
            TestStatus::NotExecuted => "gray",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RequirementCoverageItem {
    pub requirement_id: String,
    pub coverage_type: CoverageType,
    pub test_cases: Vec<TestCaseResult>,
    pub status: CoverageStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub covered_portions: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_errors: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CoverageReport {
    pub generated_at: DateTime<Utc>,
    pub total_requirements: usize,
    pub fully_covered_requirements: usize,
    pub partially_covered_requirements: usize,
    pub uncovered_requirements: usize,
    pub requirements: Vec<RequirementCoverageItem>,
}

impl CoverageReport {
    pub fn new() -> Self {
        Self {
            generated_at: Utc::now(),
            total_requirements: 0,
            fully_covered_requirements: 0,
            partially_covered_requirements: 0,
            uncovered_requirements: 0,
            requirements: Vec::new(),
        }
    }

    pub fn add_requirement(&mut self, item: RequirementCoverageItem) {
        self.total_requirements += 1;

        match &item.coverage_type {
            CoverageType::Full => {
                if !item.test_cases.is_empty() {
                    self.fully_covered_requirements += 1;
                } else {
                    self.uncovered_requirements += 1;
                }
            }
            CoverageType::Partial => {
                if !item.test_cases.is_empty() {
                    self.partially_covered_requirements += 1;
                } else {
                    self.uncovered_requirements += 1;
                }
            }
        }

        self.requirements.push(item);
    }

    pub fn compute_statistics(&mut self) {
        self.total_requirements = self.requirements.len();
        self.fully_covered_requirements = 0;
        self.partially_covered_requirements = 0;
        self.uncovered_requirements = 0;

        for req in &self.requirements {
            match &req.coverage_type {
                CoverageType::Full => {
                    if !req.test_cases.is_empty() {
                        self.fully_covered_requirements += 1;
                    } else {
                        self.uncovered_requirements += 1;
                    }
                }
                CoverageType::Partial => {
                    if !req.test_cases.is_empty() {
                        self.partially_covered_requirements += 1;
                    } else {
                        self.uncovered_requirements += 1;
                    }
                }
            }
        }
    }
}

impl Default for CoverageReport {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct VerificationContainer {
    #[allow(dead_code)]
    pub title: Option<String>,
    #[allow(dead_code)]
    pub project: Option<String>,
    #[allow(dead_code)]
    pub test_date: Option<String>,
    pub test_results: Vec<TestCaseVerificationResult>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TestCaseVerificationResult {
    pub test_case_id: String,
    #[allow(dead_code)]
    pub description: Option<String>,
    pub overall_pass: bool,
}

pub type RequirementMap = BTreeMap<String, RequirementCoverageItem>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coverage_type_serialization() {
        let full = CoverageType::Full;
        let partial = CoverageType::Partial;

        let full_json = serde_json::to_string(&full).unwrap();
        let partial_json = serde_json::to_string(&partial).unwrap();

        assert_eq!(full_json, "\"full\"");
        assert_eq!(partial_json, "\"partial\"");
    }

    #[test]
    fn test_coverage_status_colors() {
        assert_eq!(CoverageStatus::CoveredPass.color(), "green");
        assert_eq!(CoverageStatus::CoveredFail.color(), "red");
        assert_eq!(CoverageStatus::PartialCoveredPass.color(), "yellow");
        assert_eq!(CoverageStatus::PartialCoveredFail.color(), "orange");
        assert_eq!(CoverageStatus::Uncovered.color(), "gray");
    }

    #[test]
    fn test_coverage_status_display_names() {
        assert_eq!(
            CoverageStatus::CoveredPass.display_name(),
            "Covered (All Passed)"
        );
        assert_eq!(
            CoverageStatus::CoveredFail.display_name(),
            "Covered (Some Failed)"
        );
        assert_eq!(
            CoverageStatus::PartialCoveredPass.display_name(),
            "Partially Covered (All Passed)"
        );
        assert_eq!(
            CoverageStatus::PartialCoveredFail.display_name(),
            "Partially Covered (Some Failed)"
        );
        assert_eq!(CoverageStatus::Uncovered.display_name(), "No Coverage");
    }

    #[test]
    fn test_test_status_colors() {
        assert_eq!(TestStatus::Pass.color(), "green");
        assert_eq!(TestStatus::Fail.color(), "red");
        assert_eq!(TestStatus::NotExecuted.color(), "gray");
    }

    #[test]
    fn test_coverage_report_new() {
        let report = CoverageReport::new();
        assert_eq!(report.total_requirements, 0);
        assert_eq!(report.fully_covered_requirements, 0);
        assert_eq!(report.partially_covered_requirements, 0);
        assert_eq!(report.uncovered_requirements, 0);
        assert!(report.requirements.is_empty());
    }

    #[test]
    fn test_coverage_report_add_requirement_full() {
        let mut report = CoverageReport::new();

        let item = RequirementCoverageItem {
            requirement_id: "REQ-001".to_string(),
            coverage_type: CoverageType::Full,
            test_cases: vec![TestCaseResult {
                test_case_id: "TC-001".to_string(),
                status: TestStatus::Pass,
                covers: None,
                description: Some("Test case".to_string()),
            }],
            status: CoverageStatus::CoveredPass,
            requirement_text: None,
            covered_portions: None,
            coverage_errors: None,
        };

        report.add_requirement(item);

        assert_eq!(report.total_requirements, 1);
        assert_eq!(report.fully_covered_requirements, 1);
        assert_eq!(report.partially_covered_requirements, 0);
        assert_eq!(report.uncovered_requirements, 0);
    }

    #[test]
    fn test_coverage_report_add_requirement_partial() {
        let mut report = CoverageReport::new();

        let item = RequirementCoverageItem {
            requirement_id: "REQ-001".to_string(),
            coverage_type: CoverageType::Partial,
            test_cases: vec![TestCaseResult {
                test_case_id: "TC-001".to_string(),
                status: TestStatus::Pass,
                covers: Some("partial coverage".to_string()),
                description: Some("Test case".to_string()),
            }],
            status: CoverageStatus::PartialCoveredPass,
            requirement_text: Some("full requirement text".to_string()),
            covered_portions: Some(vec!["partial coverage".to_string()]),
            coverage_errors: None,
        };

        report.add_requirement(item);

        assert_eq!(report.total_requirements, 1);
        assert_eq!(report.fully_covered_requirements, 0);
        assert_eq!(report.partially_covered_requirements, 1);
        assert_eq!(report.uncovered_requirements, 0);
    }

    #[test]
    fn test_coverage_report_add_requirement_uncovered() {
        let mut report = CoverageReport::new();

        let item = RequirementCoverageItem {
            requirement_id: "REQ-001".to_string(),
            coverage_type: CoverageType::Full,
            test_cases: vec![],
            status: CoverageStatus::Uncovered,
            requirement_text: None,
            covered_portions: None,
            coverage_errors: None,
        };

        report.add_requirement(item);

        assert_eq!(report.total_requirements, 1);
        assert_eq!(report.fully_covered_requirements, 0);
        assert_eq!(report.partially_covered_requirements, 0);
        assert_eq!(report.uncovered_requirements, 1);
    }

    #[test]
    fn test_coverage_report_compute_statistics() {
        let mut report = CoverageReport::new();

        report.requirements.push(RequirementCoverageItem {
            requirement_id: "REQ-001".to_string(),
            coverage_type: CoverageType::Full,
            test_cases: vec![TestCaseResult {
                test_case_id: "TC-001".to_string(),
                status: TestStatus::Pass,
                covers: None,
                description: None,
            }],
            status: CoverageStatus::CoveredPass,
            requirement_text: None,
            covered_portions: None,
            coverage_errors: None,
        });

        report.requirements.push(RequirementCoverageItem {
            requirement_id: "REQ-002".to_string(),
            coverage_type: CoverageType::Partial,
            test_cases: vec![TestCaseResult {
                test_case_id: "TC-002".to_string(),
                status: TestStatus::Pass,
                covers: Some("partial".to_string()),
                description: None,
            }],
            status: CoverageStatus::PartialCoveredPass,
            requirement_text: None,
            covered_portions: None,
            coverage_errors: None,
        });

        report.requirements.push(RequirementCoverageItem {
            requirement_id: "REQ-003".to_string(),
            coverage_type: CoverageType::Full,
            test_cases: vec![],
            status: CoverageStatus::Uncovered,
            requirement_text: None,
            covered_portions: None,
            coverage_errors: None,
        });

        report.compute_statistics();

        assert_eq!(report.total_requirements, 3);
        assert_eq!(report.fully_covered_requirements, 1);
        assert_eq!(report.partially_covered_requirements, 1);
        assert_eq!(report.uncovered_requirements, 1);
    }

    #[test]
    fn test_requirement_definition_serialization() {
        let def = RequirementDefinition {
            id: "REQ-001".to_string(),
            text: "Requirement text".to_string(),
            description: Some("Description".to_string()),
        };

        let json = serde_json::to_string(&def).unwrap();
        let deserialized: RequirementDefinition = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, "REQ-001");
        assert_eq!(deserialized.text, "Requirement text");
        assert_eq!(deserialized.description, Some("Description".to_string()));
    }

    #[test]
    fn test_requirement_definitions_serialization() {
        let defs = RequirementDefinitions {
            requirements: vec![
                RequirementDefinition {
                    id: "REQ-001".to_string(),
                    text: "First requirement".to_string(),
                    description: None,
                },
                RequirementDefinition {
                    id: "REQ-002".to_string(),
                    text: "Second requirement".to_string(),
                    description: Some("Description".to_string()),
                },
            ],
        };

        let json = serde_json::to_string(&defs).unwrap();
        let deserialized: RequirementDefinitions = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.requirements.len(), 2);
        assert_eq!(deserialized.requirements[0].id, "REQ-001");
        assert_eq!(deserialized.requirements[1].id, "REQ-002");
    }
}
