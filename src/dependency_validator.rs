use crate::models::{InitialConditionItem, InitialConditions, TestCase};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct DependencyError {
    pub file_path: PathBuf,
    pub location: String,
    pub reference: String,
    pub error_type: DependencyErrorType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DependencyErrorType {
    UnresolvedRef,
    UnresolvedTestCaseId,
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
        }
    }
}

pub struct DependencyValidator {
    refs: HashSet<String>,
    test_case_ids: HashSet<String>,
}

impl DependencyValidator {
    pub fn new() -> Self {
        Self {
            refs: HashSet::new(),
            test_case_ids: HashSet::new(),
        }
    }

    pub fn collect_definitions(&mut self, _file_path: &Path, test_case: &TestCase) {
        self.test_case_ids.insert(test_case.id.clone());

        for test_sequence in &test_case.test_sequences {
            if let Some(ref reference) = test_sequence.reference {
                self.refs.insert(reference.clone());
            }

            for step in &test_sequence.steps {
                if let Some(ref reference) = step.reference {
                    self.refs.insert(reference.clone());
                }
            }
        }
    }

    pub fn validate_references(
        &self,
        file_path: &Path,
        test_case: &TestCase,
    ) -> Vec<DependencyError> {
        let mut errors = Vec::new();

        errors.extend(self.validate_initial_conditions_refs(
            file_path,
            &test_case.general_initial_conditions,
            "general_initial_conditions",
        ));

        errors.extend(self.validate_initial_conditions_refs(
            file_path,
            &test_case.initial_conditions,
            "initial_conditions",
        ));

        for (seq_idx, test_sequence) in test_case.test_sequences.iter().enumerate() {
            errors.extend(self.validate_initial_conditions_refs(
                file_path,
                &test_sequence.initial_conditions,
                &format!("test_sequences[{}].initial_conditions", seq_idx),
            ));
        }

        errors
    }

    fn validate_initial_conditions_refs(
        &self,
        file_path: &Path,
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
                    InitialConditionItem::TestSequenceRef { .. } => {}
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
}
