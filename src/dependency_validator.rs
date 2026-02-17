use crate::models::{InitialConditionItem, InitialConditions, TestCase};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Error type for dependency validation failures
#[derive(Debug, Clone)]
pub struct DependencyError {
    /// Path to the file containing the error
    pub file_path: PathBuf,
    /// Location within the file (e.g., "initial_conditions.device1[0]")
    pub location: String,
    /// The unresolved reference string
    pub reference: String,
    /// Type of dependency error
    pub error_type: DependencyErrorType,
}

/// Types of dependency validation errors
#[derive(Debug, Clone, PartialEq)]
pub enum DependencyErrorType {
    /// A reference (from ref field) could not be resolved
    UnresolvedRef,
    /// A test case ID (from include) could not be resolved
    UnresolvedTestCaseId,
    /// A test sequence ID could not be resolved
    UnresolvedTestSequenceId,
}

impl std::fmt::Display for DependencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.error_type {
            DependencyErrorType::UnresolvedRef => {
                write!(
                    f,
                    "{}:{} - Unresolved reference: '{}'",
                    self.file_path.display(),
                    self.location,
                    self.reference
                )
            }
            DependencyErrorType::UnresolvedTestCaseId => {
                write!(
                    f,
                    "{}:{} - Unresolved test case ID: '{}'",
                    self.file_path.display(),
                    self.location,
                    self.reference
                )
            }
            DependencyErrorType::UnresolvedTestSequenceId => {
                write!(
                    f,
                    "{}:{} - Unresolved test sequence ID: '{}'",
                    self.file_path.display(),
                    self.location,
                    self.reference
                )
            }
        }
    }
}

/// Validator for cross-file dependencies in test cases
pub struct DependencyValidator {
    /// Set of all reference strings defined across test cases
    refs: HashSet<String>,
    /// Set of all test case IDs
    test_case_ids: HashSet<String>,
    /// Map from test case ID to set of test sequence IDs within that test case
    test_sequences: HashMap<String, HashSet<i64>>,
}

impl DependencyValidator {
    /// Create a new dependency validator
    pub fn new() -> Self {
        Self {
            refs: HashSet::new(),
            test_case_ids: HashSet::new(),
            test_sequences: HashMap::new(),
        }
    }

    /// Collect all definitions (test case IDs, test sequence IDs, and references) from a test case
    pub fn collect_definitions(&mut self, _file_path: &Path, test_case: &TestCase) {
        self.test_case_ids.insert(test_case.id.clone());

        let mut seq_ids = HashSet::new();
        for test_sequence in &test_case.test_sequences {
            seq_ids.insert(test_sequence.id);

            if let Some(ref reference) = test_sequence.reference {
                self.refs.insert(reference.clone());
            }

            for step in &test_sequence.steps {
                if let Some(ref reference) = step.reference {
                    self.refs.insert(reference.clone());
                }
            }
        }
        self.test_sequences.insert(test_case.id.clone(), seq_ids);
    }

    /// Validate all references in a test case against collected definitions
    pub fn validate_references(
        &self,
        file_path: &Path,
        test_case: &TestCase,
    ) -> Vec<DependencyError> {
        let mut errors = Vec::new();

        errors.extend(self.validate_initial_conditions_refs(
            file_path,
            test_case,
            &test_case.general_initial_conditions,
            "general_initial_conditions",
        ));

        errors.extend(self.validate_initial_conditions_refs(
            file_path,
            test_case,
            &test_case.initial_conditions,
            "initial_conditions",
        ));

        for (seq_idx, test_sequence) in test_case.test_sequences.iter().enumerate() {
            errors.extend(self.validate_initial_conditions_refs(
                file_path,
                test_case,
                &test_sequence.initial_conditions,
                &format!("test_sequences[{}].initial_conditions", seq_idx),
            ));
        }

        errors
    }

    fn validate_initial_conditions_refs(
        &self,
        file_path: &Path,
        test_case: &TestCase,
        initial_conditions: &InitialConditions,
        base_location: &str,
    ) -> Vec<DependencyError> {
        let mut errors = Vec::new();

        if let Some(ref include_list) = initial_conditions.include {
            for (idx, include_ref) in include_list.iter().enumerate() {
                if !self.test_case_ids.contains(&include_ref.id) {
                    errors.push(DependencyError {
                        file_path: file_path.to_path_buf(),
                        location: format!("{}.include[{}]", base_location, idx),
                        reference: include_ref.id.clone(),
                        error_type: DependencyErrorType::UnresolvedTestCaseId,
                    });
                }
                // Note: test_sequence field is not validated here because:
                // 1. It's a soft reference that may be resolved at runtime
                // 2. Test sequences might be added dynamically
                // 3. The original implementation did not validate this field
            }
        }

        for (device_name, items) in &initial_conditions.devices {
            for (idx, item) in items.iter().enumerate() {
                match item {
                    InitialConditionItem::RefItem { reference } => {
                        if !self.refs.contains(reference) {
                            errors.push(DependencyError {
                                file_path: file_path.to_path_buf(),
                                location: format!("{}.{}[{}]", base_location, device_name, idx),
                                reference: reference.clone(),
                                error_type: DependencyErrorType::UnresolvedRef,
                            });
                        }
                    }
                    InitialConditionItem::String(_) => {}
                    InitialConditionItem::TestSequenceRef { test_sequence } => {
                        let seq_id = test_sequence.id;
                        let has_sequence =
                            test_case.test_sequences.iter().any(|ts| ts.id == seq_id);
                        if !has_sequence {
                            errors.push(DependencyError {
                                file_path: file_path.to_path_buf(),
                                location: format!(
                                    "{}.{}[{}].test_sequence.id",
                                    base_location, device_name, idx
                                ),
                                reference: seq_id.to_string(),
                                error_type: DependencyErrorType::UnresolvedTestSequenceId,
                            });
                        }
                    }
                }
            }
        }

        errors
    }
}

impl Default for DependencyValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate dependencies across multiple test case files
///
/// This function performs a two-pass validation:
/// 1. First pass: collect all definitions (test case IDs, test sequence IDs, and references)
/// 2. Second pass: validate that all references can be resolved
///
/// # Arguments
/// * `files` - A slice of tuples containing file paths and their parsed test cases
///
/// # Returns
/// * `Ok(())` if all dependencies are valid
/// * `Err(Vec<DependencyError>)` containing all validation errors found
pub fn validate_cross_file_dependencies(
    files: &[(PathBuf, TestCase)],
) -> Result<(), Vec<DependencyError>> {
    let mut validator = DependencyValidator::new();

    for (file_path, test_case) in files {
        validator.collect_definitions(file_path, test_case);
    }

    let mut all_errors = Vec::new();
    for (file_path, test_case) in files {
        let errors = validator.validate_references(file_path, test_case);
        all_errors.extend(errors);
    }

    if all_errors.is_empty() {
        Ok(())
    } else {
        Err(all_errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{IncludeRef, Step, TestSequence};

    fn create_test_case(id: &str) -> TestCase {
        TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            id.to_string(),
            "Test".to_string(),
        )
    }

    #[test]
    fn test_collect_test_case_ids() {
        let mut validator = DependencyValidator::new();
        let test_case = create_test_case("TC001");

        validator.collect_definitions(Path::new("test.yaml"), &test_case);

        assert!(validator.test_case_ids.contains("TC001"));
    }

    #[test]
    fn test_collect_test_sequence_ref() {
        let mut validator = DependencyValidator::new();
        let mut test_case = create_test_case("TC001");

        let mut test_sequence = TestSequence::new(1, "Seq".to_string(), "Desc".to_string());
        test_sequence.reference = Some("ref-123".to_string());
        test_case.test_sequences.push(test_sequence);

        validator.collect_definitions(Path::new("test.yaml"), &test_case);

        assert!(validator.refs.contains("ref-123"));
    }

    #[test]
    fn test_collect_step_ref() {
        let mut validator = DependencyValidator::new();
        let mut test_case = create_test_case("TC001");

        let mut test_sequence = TestSequence::new(1, "Seq".to_string(), "Desc".to_string());
        let mut step = Step::new(
            1,
            "Step".to_string(),
            "cmd".to_string(),
            "0".to_string(),
            "output".to_string(),
        );
        step.reference = Some("step-ref-456".to_string());
        test_sequence.steps.push(step);
        test_case.test_sequences.push(test_sequence);

        validator.collect_definitions(Path::new("test.yaml"), &test_case);

        assert!(validator.refs.contains("step-ref-456"));
    }

    #[test]
    fn test_unresolved_include_ref() {
        let validator = DependencyValidator::new();
        let mut test_case = create_test_case("TC001");

        let initial_conditions = InitialConditions {
            include: Some(vec![IncludeRef {
                id: "TC002".to_string(),
                test_sequence: None,
            }]),
            ..Default::default()
        };
        test_case.general_initial_conditions = initial_conditions;

        let errors = validator.validate_references(Path::new("test.yaml"), &test_case);

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].reference, "TC002");
        assert_eq!(
            errors[0].error_type,
            DependencyErrorType::UnresolvedTestCaseId
        );
    }

    #[test]
    fn test_unresolved_ref_item() {
        let validator = DependencyValidator::new();
        let mut test_case = create_test_case("TC001");

        let mut initial_conditions = InitialConditions::default();
        initial_conditions.devices.insert(
            "eUICC".to_string(),
            vec![InitialConditionItem::RefItem {
                reference: "missing-ref".to_string(),
            }],
        );
        test_case.initial_conditions = initial_conditions;

        let errors = validator.validate_references(Path::new("test.yaml"), &test_case);

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].reference, "missing-ref");
        assert_eq!(errors[0].error_type, DependencyErrorType::UnresolvedRef);
    }

    #[test]
    fn test_resolved_references() {
        let mut validator = DependencyValidator::new();

        let mut test_case1 = create_test_case("TC001");
        let mut test_sequence = TestSequence::new(1, "Seq".to_string(), "Desc".to_string());
        test_sequence.reference = Some("ref-123".to_string());
        test_case1.test_sequences.push(test_sequence);

        let mut test_case2 = create_test_case("TC002");
        let mut initial_conditions = InitialConditions::default();
        initial_conditions.devices.insert(
            "eUICC".to_string(),
            vec![InitialConditionItem::RefItem {
                reference: "ref-123".to_string(),
            }],
        );
        test_case2.initial_conditions = initial_conditions;

        validator.collect_definitions(Path::new("test1.yaml"), &test_case1);
        let errors = validator.validate_references(Path::new("test2.yaml"), &test_case2);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn test_validate_cross_file_dependencies_success() {
        let mut test_case1 = create_test_case("TC001");
        let mut test_sequence = TestSequence::new(1, "Seq".to_string(), "Desc".to_string());
        test_sequence.reference = Some("ref-abc".to_string());
        test_case1.test_sequences.push(test_sequence);

        let mut test_case2 = create_test_case("TC002");
        let mut initial_conditions = InitialConditions {
            include: Some(vec![IncludeRef {
                id: "TC001".to_string(),
                test_sequence: None,
            }]),
            ..Default::default()
        };
        initial_conditions.devices.insert(
            "eUICC".to_string(),
            vec![InitialConditionItem::RefItem {
                reference: "ref-abc".to_string(),
            }],
        );
        test_case2.general_initial_conditions = initial_conditions;

        let files = vec![
            (PathBuf::from("test1.yaml"), test_case1),
            (PathBuf::from("test2.yaml"), test_case2),
        ];

        let result = validate_cross_file_dependencies(&files);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_cross_file_dependencies_failure() {
        let test_case1 = create_test_case("TC001");

        let mut test_case2 = create_test_case("TC002");
        let mut initial_conditions = InitialConditions {
            include: Some(vec![IncludeRef {
                id: "TC999".to_string(),
                test_sequence: None,
            }]),
            ..Default::default()
        };
        initial_conditions.devices.insert(
            "eUICC".to_string(),
            vec![InitialConditionItem::RefItem {
                reference: "missing-ref".to_string(),
            }],
        );
        test_case2.general_initial_conditions = initial_conditions;

        let files = vec![
            (PathBuf::from("test1.yaml"), test_case1),
            (PathBuf::from("test2.yaml"), test_case2),
        ];

        let result = validate_cross_file_dependencies(&files);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn test_validate_test_sequence_ref_within_same_test_case() {
        use crate::models::TestSequenceRefTarget;

        let mut test_case = create_test_case("TC001");
        let test_sequence1 = TestSequence::new(1, "Seq1".to_string(), "Desc".to_string());
        test_case.test_sequences.push(test_sequence1);

        let mut initial_conditions = InitialConditions::default();
        initial_conditions.devices.insert(
            "device1".to_string(),
            vec![InitialConditionItem::TestSequenceRef {
                test_sequence: TestSequenceRefTarget {
                    id: 1,
                    step: "1".to_string(),
                },
            }],
        );
        test_case.initial_conditions = initial_conditions;

        let validator = DependencyValidator::new();
        let errors = validator.validate_references(Path::new("test.yaml"), &test_case);
        assert_eq!(
            errors.len(),
            0,
            "Valid intra-test-case test_sequence reference should not produce errors"
        );
    }

    #[test]
    fn test_validate_invalid_test_sequence_ref_within_same_test_case() {
        use crate::models::TestSequenceRefTarget;

        let mut test_case = create_test_case("TC001");
        let test_sequence1 = TestSequence::new(1, "Seq1".to_string(), "Desc".to_string());
        test_case.test_sequences.push(test_sequence1);

        let mut initial_conditions = InitialConditions::default();
        initial_conditions.devices.insert(
            "device1".to_string(),
            vec![InitialConditionItem::TestSequenceRef {
                test_sequence: TestSequenceRefTarget {
                    id: 99,
                    step: "1".to_string(),
                },
            }],
        );
        test_case.initial_conditions = initial_conditions;

        let validator = DependencyValidator::new();
        let errors = validator.validate_references(Path::new("test.yaml"), &test_case);
        assert_eq!(
            errors.len(),
            1,
            "Invalid test_sequence.id should produce an error"
        );
        assert_eq!(
            errors[0].error_type,
            DependencyErrorType::UnresolvedTestSequenceId
        );
        assert_eq!(errors[0].reference, "99");
    }

    #[test]
    fn test_collect_multiple_test_sequences() {
        let mut validator = DependencyValidator::new();
        let mut test_case = create_test_case("TC001");

        let seq1 = TestSequence::new(1, "Seq1".to_string(), "Desc1".to_string());
        let seq2 = TestSequence::new(2, "Seq2".to_string(), "Desc2".to_string());
        let seq3 = TestSequence::new(3, "Seq3".to_string(), "Desc3".to_string());
        test_case.test_sequences.push(seq1);
        test_case.test_sequences.push(seq2);
        test_case.test_sequences.push(seq3);

        validator.collect_definitions(Path::new("test.yaml"), &test_case);

        assert!(validator.test_sequences.contains_key("TC001"));
        let seq_ids = validator.test_sequences.get("TC001").unwrap();
        assert_eq!(seq_ids.len(), 3);
        assert!(seq_ids.contains(&1));
        assert!(seq_ids.contains(&2));
        assert!(seq_ids.contains(&3));
    }
}
