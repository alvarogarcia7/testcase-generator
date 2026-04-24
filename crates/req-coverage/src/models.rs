use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
