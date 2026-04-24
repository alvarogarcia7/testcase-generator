use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::models::*;
use testcase_models::TestCase;
use testcase_storage::TestCaseStorage;

pub struct CoverageAnalyzer {
    test_case_storage: TestCaseStorage,
}

impl CoverageAnalyzer {
    pub fn new(test_cases_folder: &Path) -> Result<Self> {
        let test_case_storage = TestCaseStorage::new(test_cases_folder)
            .context("Failed to initialize test case storage")?;

        Ok(Self { test_case_storage })
    }

    pub fn analyze(&self, test_results_folder: &Path) -> Result<CoverageReport> {
        log::info!("Starting coverage analysis");
        log::debug!(
            "Test cases folder: {:?}",
            self.test_case_storage.base_path()
        );
        log::debug!("Test results folder: {:?}", test_results_folder);

        let test_cases = self.load_all_test_cases()?;
        log::info!("Loaded {} test cases", test_cases.len());

        let verification_results = self.load_verification_results(test_results_folder)?;
        log::info!("Loaded {} verification results", verification_results.len());

        let mut requirement_map: RequirementMap = BTreeMap::new();

        for test_case in test_cases {
            self.process_test_case(&test_case, &verification_results, &mut requirement_map)?;
        }

        let mut report = CoverageReport::new();
        for (_req_id, item) in requirement_map {
            report.add_requirement(item);
        }

        report.compute_statistics();
        report.generated_at = chrono::Utc::now();

        log::info!("Coverage analysis complete");
        log::info!("  Total requirements: {}", report.total_requirements);
        log::info!("  Fully covered: {}", report.fully_covered_requirements);
        log::info!(
            "  Partially covered: {}",
            report.partially_covered_requirements
        );
        log::info!("  Uncovered: {}", report.uncovered_requirements);

        Ok(report)
    }

    fn load_all_test_cases(&self) -> Result<Vec<TestCase>> {
        let mut test_cases = Vec::new();
        let base_path = self.test_case_storage.base_path();

        log::debug!("Scanning for test case files in: {:?}", base_path);

        for entry in WalkDir::new(base_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy();
                    if ext_str == "yaml" || ext_str == "yml" {
                        log::debug!("Attempting to load test case from: {:?}", path);
                        match self.load_test_case_file(path) {
                            Ok(test_case) => {
                                log::debug!(
                                    "Loaded test case: {} (requirement: {})",
                                    test_case.id,
                                    test_case.requirement
                                );
                                test_cases.push(test_case);
                            }
                            Err(e) => {
                                log::warn!("Failed to load test case from {:?}: {}", path, e);
                            }
                        }
                    }
                }
            }
        }

        Ok(test_cases)
    }

    fn load_test_case_file(&self, path: &Path) -> Result<TestCase> {
        let content =
            fs::read_to_string(path).context(format!("Failed to read file: {:?}", path))?;

        let test_case: TestCase = serde_yaml::from_str(&content)
            .context(format!("Failed to parse YAML from: {:?}", path))?;

        Ok(test_case)
    }

    fn load_verification_results(
        &self,
        test_results_folder: &Path,
    ) -> Result<BTreeMap<String, TestCaseVerificationResult>> {
        let mut results = BTreeMap::new();

        log::debug!(
            "Scanning for verification result files in: {:?}",
            test_results_folder
        );

        for entry in WalkDir::new(test_results_folder)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    let file_name_str = file_name.to_string_lossy();
                    if file_name_str.ends_with("_container.yaml")
                        || file_name_str.ends_with("_container.yml")
                    {
                        log::debug!("Loading verification result from: {:?}", path);
                        match self.load_verification_file(path) {
                            Ok(container) => {
                                for test_result in container.test_results {
                                    log::debug!(
                                        "Found verification result for test case: {} (pass: {})",
                                        test_result.test_case_id,
                                        test_result.overall_pass
                                    );
                                    results.insert(test_result.test_case_id.clone(), test_result);
                                }
                            }
                            Err(e) => {
                                log::warn!(
                                    "Failed to load verification result from {:?}: {}",
                                    path,
                                    e
                                );
                            }
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    fn load_verification_file(&self, path: &Path) -> Result<VerificationContainer> {
        let content =
            fs::read_to_string(path).context(format!("Failed to read file: {:?}", path))?;

        let container: VerificationContainer = serde_yaml::from_str(&content)
            .context(format!("Failed to parse YAML from: {:?}", path))?;

        Ok(container)
    }

    fn process_test_case(
        &self,
        test_case: &TestCase,
        verification_results: &BTreeMap<String, TestCaseVerificationResult>,
        requirement_map: &mut RequirementMap,
    ) -> Result<()> {
        let (coverage_type, covers) = if let Some(ref cov) = test_case.requirement_coverage {
            let cov_type = match cov.coverage_type {
                testcase_models::RequirementCoverageType::Full => CoverageType::Full,
                testcase_models::RequirementCoverageType::Partial => CoverageType::Partial,
            };
            (cov_type, cov.covers.clone())
        } else {
            (CoverageType::Full, None)
        };

        let test_status = verification_results
            .get(&test_case.id)
            .map(|result| {
                if result.overall_pass {
                    TestStatus::Pass
                } else {
                    TestStatus::Fail
                }
            })
            .unwrap_or(TestStatus::NotExecuted);

        let mut requirements_to_process = vec![test_case.requirement.clone()];

        if let Some(ref cov) = test_case.requirement_coverage {
            if let Some(ref additional) = cov.additional_requirements {
                requirements_to_process.extend(additional.clone());
            }
        }

        for requirement_id in requirements_to_process {
            let test_case_result = TestCaseResult {
                test_case_id: test_case.id.clone(),
                status: test_status.clone(),
                covers: covers.clone(),
                description: Some(test_case.description.clone()),
            };

            let item = requirement_map
                .entry(requirement_id.clone())
                .or_insert_with(|| RequirementCoverageItem {
                    requirement_id: requirement_id.clone(),
                    coverage_type: coverage_type.clone(),
                    test_cases: Vec::new(),
                    status: CoverageStatus::Uncovered,
                });

            item.test_cases.push(test_case_result);

            let has_failures = item
                .test_cases
                .iter()
                .any(|tc| tc.status == TestStatus::Fail);
            let has_coverage = !item.test_cases.is_empty();

            item.status = match (&item.coverage_type, has_coverage, has_failures) {
                (CoverageType::Full, true, false) => CoverageStatus::CoveredPass,
                (CoverageType::Full, true, true) => CoverageStatus::CoveredFail,
                (CoverageType::Partial, true, false) => CoverageStatus::PartialCoveredPass,
                (CoverageType::Partial, true, true) => CoverageStatus::PartialCoveredFail,
                _ => CoverageStatus::Uncovered,
            };
        }

        Ok(())
    }
}
