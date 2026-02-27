use crate::models::{
    IncludeRef, InitialConditionItem, InitialConditions, TestCase, TestSequenceRefTarget,
};
use std::collections::HashMap;

/// Error type for dependency resolution failures
#[derive(Debug, Clone)]
pub enum ResolutionError {
    /// A test case ID could not be found in the index
    TestCaseNotFound { id: String },
    /// A reference could not be found in the ref index
    RefNotFound { reference: String },
    /// A test sequence could not be found
    TestSequenceNotFound { test_case_id: String, seq_id: i64 },
    /// Invalid step range expression
    InvalidStepRange { expression: String, reason: String },
}

impl std::fmt::Display for ResolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolutionError::TestCaseNotFound { id } => {
                write!(f, "Test case not found: '{}'", id)
            }
            ResolutionError::RefNotFound { reference } => {
                write!(f, "Reference not found: '{}'", reference)
            }
            ResolutionError::TestSequenceNotFound {
                test_case_id,
                seq_id,
            } => {
                write!(
                    f,
                    "Test sequence {} not found in test case '{}'",
                    seq_id, test_case_id
                )
            }
            ResolutionError::InvalidStepRange { expression, reason } => {
                write!(f, "Invalid step range '{}': {}", expression, reason)
            }
        }
    }
}

impl std::error::Error for ResolutionError {}

/// Content that can be referenced by a ref string
#[derive(Debug, Clone)]
pub enum RefContent {
    /// Content from a test sequence
    TestSequence { description: String },
    /// Content from a step
    Step { description: String },
}

/// Dependency resolver that expands includes and resolves references
pub struct DependencyResolver {
    /// Index of all test cases by ID
    test_case_index: HashMap<String, TestCase>,
    /// Index mapping ref strings to their content
    ref_index: HashMap<String, RefContent>,
}

impl DependencyResolver {
    /// Create a new dependency resolver from a test case index
    pub fn new(test_case_index: HashMap<String, TestCase>) -> Self {
        let ref_index = Self::build_ref_index(&test_case_index);
        Self {
            test_case_index,
            ref_index,
        }
    }

    /// Build an index of all ref strings to their content
    fn build_ref_index(test_case_index: &HashMap<String, TestCase>) -> HashMap<String, RefContent> {
        let mut ref_index = HashMap::new();

        for test_case in test_case_index.values() {
            for test_sequence in &test_case.test_sequences {
                // Index test sequence refs
                if let Some(ref reference) = test_sequence.reference {
                    ref_index.insert(
                        reference.clone(),
                        RefContent::TestSequence {
                            description: test_sequence.description.clone(),
                        },
                    );
                }

                // Index step refs
                for step in &test_sequence.steps {
                    if let Some(ref reference) = step.reference {
                        ref_index.insert(
                            reference.clone(),
                            RefContent::Step {
                                description: step.description.clone(),
                            },
                        );
                    }
                }
            }
        }

        ref_index
    }

    /// Resolve all dependencies in a test case
    ///
    /// This operation is idempotent - resolving an already-resolved test case
    /// produces identical output.
    pub fn resolve(&self, test_case: &TestCase) -> Result<TestCase, ResolutionError> {
        let mut resolved = test_case.clone();

        // Resolve general_initial_conditions
        resolved.general_initial_conditions =
            self.resolve_initial_conditions(&resolved.general_initial_conditions, test_case)?;

        // Resolve initial_conditions
        resolved.initial_conditions =
            self.resolve_initial_conditions(&resolved.initial_conditions, test_case)?;

        // Resolve test_sequences initial_conditions
        for test_sequence in &mut resolved.test_sequences {
            test_sequence.initial_conditions =
                self.resolve_initial_conditions(&test_sequence.initial_conditions, test_case)?;
        }

        Ok(resolved)
    }

    /// Resolve initial conditions by expanding includes and resolving refs
    fn resolve_initial_conditions(
        &self,
        initial_conditions: &InitialConditions,
        current_test_case: &TestCase,
    ) -> Result<InitialConditions, ResolutionError> {
        let mut resolved = InitialConditions {
            include: None, // Remove include field to make idempotent
            devices: initial_conditions.devices.clone(),
        };

        // Expand includes
        if let Some(ref includes) = initial_conditions.include {
            for include_ref in includes {
                self.expand_include(&mut resolved, include_ref)?;
            }
        }

        // Resolve RefItem and TestSequenceRef entries
        for items in resolved.devices.values_mut() {
            let mut new_items = Vec::new();
            for item in items.iter() {
                match item {
                    InitialConditionItem::String(s) => {
                        new_items.push(InitialConditionItem::String(s.clone()));
                    }
                    InitialConditionItem::RefItem { reference } => {
                        let content = self.resolve_ref(reference)?;
                        new_items.push(InitialConditionItem::String(content));
                    }
                    InitialConditionItem::TestSequenceRef { test_sequence } => {
                        let descriptions =
                            self.resolve_test_sequence_ref(test_sequence, current_test_case)?;
                        for desc in descriptions {
                            new_items.push(InitialConditionItem::String(desc));
                        }
                    }
                }
            }
            *items = new_items;
        }

        Ok(resolved)
    }

    /// Expand an include reference by merging devices from the referenced test case
    fn expand_include(
        &self,
        target: &mut InitialConditions,
        include_ref: &IncludeRef,
    ) -> Result<(), ResolutionError> {
        let referenced_test_case = self.test_case_index.get(&include_ref.id).ok_or_else(|| {
            ResolutionError::TestCaseNotFound {
                id: include_ref.id.clone(),
            }
        })?;

        // Determine which initial conditions to include
        let source_conditions = if let Some(ref test_sequence_id) = include_ref.test_sequence {
            // Include from a specific test sequence
            let seq_id: i64 =
                test_sequence_id
                    .parse()
                    .map_err(|_| ResolutionError::InvalidStepRange {
                        expression: test_sequence_id.clone(),
                        reason: "test_sequence must be a valid integer".to_string(),
                    })?;

            let test_sequence = referenced_test_case
                .test_sequences
                .iter()
                .find(|ts| ts.id == seq_id)
                .ok_or_else(|| ResolutionError::TestSequenceNotFound {
                    test_case_id: include_ref.id.clone(),
                    seq_id,
                })?;

            &test_sequence.initial_conditions
        } else {
            // Include from general_initial_conditions
            &referenced_test_case.general_initial_conditions
        };

        // Merge devices from source into target
        for (device, items) in &source_conditions.devices {
            target
                .devices
                .entry(device.clone())
                .or_default()
                .extend(items.clone());
        }

        Ok(())
    }

    /// Resolve a reference to its description
    fn resolve_ref(&self, reference: &str) -> Result<String, ResolutionError> {
        let content =
            self.ref_index
                .get(reference)
                .ok_or_else(|| ResolutionError::RefNotFound {
                    reference: reference.to_string(),
                })?;

        Ok(match content {
            RefContent::TestSequence { description } => description.clone(),
            RefContent::Step { description } => description.clone(),
        })
    }

    /// Resolve a test_sequence reference with step range to descriptions
    fn resolve_test_sequence_ref(
        &self,
        test_sequence_ref: &TestSequenceRefTarget,
        current_test_case: &TestCase,
    ) -> Result<Vec<String>, ResolutionError> {
        // Find the test sequence in the current test case
        let test_sequence = current_test_case
            .test_sequences
            .iter()
            .find(|ts| ts.id == test_sequence_ref.id)
            .ok_or_else(|| ResolutionError::TestSequenceNotFound {
                test_case_id: current_test_case.id.clone(),
                seq_id: test_sequence_ref.id,
            })?;

        // Parse the step range expression
        let step_numbers = parse_step_range(&test_sequence_ref.step)?;

        // Collect descriptions for the specified steps
        let mut descriptions = Vec::new();
        for step_num in step_numbers {
            if let Some(step) = test_sequence.steps.iter().find(|s| s.step == step_num) {
                descriptions.push(step.description.clone());
            }
        }

        Ok(descriptions)
    }
}

/// Parse a step range expression into a list of step numbers
///
/// Supported formats:
/// - `[1,4]` - Inclusive range (steps 1, 2, 3, 4)
/// - `(1,4)` - Exclusive range (steps 2, 3)
/// - `[1,4)` - Left-inclusive range (steps 1, 2, 3)
/// - `(1,4]` - Right-inclusive range (steps 2, 3, 4)
/// - `2,3,5` - Explicit list (steps 2, 3, 5)
/// - `[4,1]` - Reverse inclusive range (steps 4, 3, 2, 1)
fn parse_step_range(expression: &str) -> Result<Vec<i64>, ResolutionError> {
    let trimmed = expression.trim();

    // Check if it's a range expression (starts with [ or ()
    if (trimmed.starts_with('[') || trimmed.starts_with('('))
        && (trimmed.ends_with(']') || trimmed.ends_with(')'))
    {
        // Parse range expression
        let left_inclusive = trimmed.starts_with('[');
        let right_inclusive = trimmed.ends_with(']');

        let inner = &trimmed[1..trimmed.len() - 1];
        let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();

        if parts.len() != 2 {
            return Err(ResolutionError::InvalidStepRange {
                expression: expression.to_string(),
                reason: "Range expression must have exactly two numbers".to_string(),
            });
        }

        let start: i64 = parts[0]
            .parse()
            .map_err(|_| ResolutionError::InvalidStepRange {
                expression: expression.to_string(),
                reason: format!("Invalid number: '{}'", parts[0]),
            })?;

        let end: i64 = parts[1]
            .parse()
            .map_err(|_| ResolutionError::InvalidStepRange {
                expression: expression.to_string(),
                reason: format!("Invalid number: '{}'", parts[1]),
            })?;

        let mut result = Vec::new();

        if start <= end {
            // Forward range
            let actual_start = if left_inclusive { start } else { start + 1 };
            let actual_end = if right_inclusive { end } else { end - 1 };

            for i in actual_start..=actual_end {
                result.push(i);
            }
        } else {
            // Reverse range
            let actual_start = if left_inclusive { start } else { start - 1 };
            let actual_end = if right_inclusive { end } else { end + 1 };

            for i in (actual_end..=actual_start).rev() {
                result.push(i);
            }
        }

        Ok(result)
    } else {
        // Parse explicit list
        let parts: Vec<&str> = trimmed.split(',').map(|s| s.trim()).collect();
        let mut result = Vec::new();

        for part in parts {
            let num: i64 = part
                .parse()
                .map_err(|_| ResolutionError::InvalidStepRange {
                    expression: expression.to_string(),
                    reason: format!("Invalid number: '{}'", part),
                })?;
            result.push(num);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Step, TestSequence};

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
    fn test_parse_step_range_inclusive() {
        let result = parse_step_range("[1,4]").unwrap();
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_parse_step_range_exclusive() {
        let result = parse_step_range("(1,4)").unwrap();
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_parse_step_range_left_inclusive() {
        let result = parse_step_range("[1,3)").unwrap();
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_parse_step_range_right_inclusive() {
        let result = parse_step_range("(1,3]").unwrap();
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_parse_step_range_reverse_inclusive() {
        let result = parse_step_range("[4,1]").unwrap();
        assert_eq!(result, vec![4, 3, 2, 1]);
    }

    #[test]
    fn test_parse_step_range_reverse_exclusive() {
        let result = parse_step_range("(4,1)").unwrap();
        assert_eq!(result, vec![3, 2]);
    }

    #[test]
    fn test_parse_step_range_explicit_list() {
        let result = parse_step_range("2,3,5").unwrap();
        assert_eq!(result, vec![2, 3, 5]);
    }

    #[test]
    fn test_parse_step_range_explicit_list_with_spaces() {
        let result = parse_step_range("2, 3, 5").unwrap();
        assert_eq!(result, vec![2, 3, 5]);
    }

    #[test]
    fn test_parse_step_range_single_number() {
        let result = parse_step_range("5").unwrap();
        assert_eq!(result, vec![5]);
    }

    #[test]
    fn test_parse_step_range_invalid_range() {
        let result = parse_step_range("[1,2,3]");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_step_range_invalid_number() {
        let result = parse_step_range("[1,abc]");
        assert!(result.is_err());
    }

    #[test]
    fn test_build_ref_index_empty() {
        let index = HashMap::new();
        let ref_index = DependencyResolver::build_ref_index(&index);
        assert_eq!(ref_index.len(), 0);
    }

    #[test]
    fn test_build_ref_index_test_sequence_ref() {
        let mut test_case = create_test_case("TC001");
        let mut test_sequence = TestSequence::new(1, "Seq".to_string(), "Desc".to_string());
        test_sequence.reference = Some("ref-123".to_string());
        test_case.test_sequences.push(test_sequence);

        let mut index = HashMap::new();
        index.insert("TC001".to_string(), test_case);

        let ref_index = DependencyResolver::build_ref_index(&index);
        assert_eq!(ref_index.len(), 1);
        assert!(ref_index.contains_key("ref-123"));

        if let Some(RefContent::TestSequence { description }) = ref_index.get("ref-123") {
            assert_eq!(description, "Desc");
        } else {
            panic!("Expected TestSequence ref content");
        }
    }

    #[test]
    fn test_build_ref_index_step_ref() {
        let mut test_case = create_test_case("TC001");
        let mut test_sequence = TestSequence::new(1, "Seq".to_string(), "Desc".to_string());
        let mut step = Step::new(
            1,
            "Step desc".to_string(),
            "cmd".to_string(),
            "0".to_string(),
            "out".to_string(),
        );
        step.reference = Some("step-ref-456".to_string());
        test_sequence.steps.push(step);
        test_case.test_sequences.push(test_sequence);

        let mut index = HashMap::new();
        index.insert("TC001".to_string(), test_case);

        let ref_index = DependencyResolver::build_ref_index(&index);
        assert_eq!(ref_index.len(), 1);
        assert!(ref_index.contains_key("step-ref-456"));

        if let Some(RefContent::Step { description }) = ref_index.get("step-ref-456") {
            assert_eq!(description, "Step desc");
        } else {
            panic!("Expected Step ref content");
        }
    }

    #[test]
    fn test_expand_include_general_initial_conditions() {
        let mut source_test_case = create_test_case("TC001");
        let mut general_ic = InitialConditions::default();
        general_ic.devices.insert(
            "device1".to_string(),
            vec![InitialConditionItem::String("condition1".to_string())],
        );
        source_test_case.general_initial_conditions = general_ic;

        let mut index = HashMap::new();
        index.insert("TC001".to_string(), source_test_case);

        let resolver = DependencyResolver::new(index);

        let mut target = InitialConditions::default();
        let include_ref = IncludeRef {
            id: "TC001".to_string(),
            test_sequence: None,
        };

        resolver.expand_include(&mut target, &include_ref).unwrap();

        assert_eq!(target.devices.len(), 1);
        assert!(target.devices.contains_key("device1"));
        assert_eq!(
            target.devices.get("device1").unwrap()[0],
            InitialConditionItem::String("condition1".to_string())
        );
    }

    #[test]
    fn test_expand_include_test_sequence_initial_conditions() {
        let mut source_test_case = create_test_case("TC001");
        let mut test_sequence = TestSequence::new(1, "Seq".to_string(), "Desc".to_string());
        let mut seq_ic = InitialConditions::default();
        seq_ic.devices.insert(
            "device2".to_string(),
            vec![InitialConditionItem::String("seq-condition".to_string())],
        );
        test_sequence.initial_conditions = seq_ic;
        source_test_case.test_sequences.push(test_sequence);

        let mut index = HashMap::new();
        index.insert("TC001".to_string(), source_test_case);

        let resolver = DependencyResolver::new(index);

        let mut target = InitialConditions::default();
        let include_ref = IncludeRef {
            id: "TC001".to_string(),
            test_sequence: Some("1".to_string()),
        };

        resolver.expand_include(&mut target, &include_ref).unwrap();

        assert_eq!(target.devices.len(), 1);
        assert!(target.devices.contains_key("device2"));
        assert_eq!(
            target.devices.get("device2").unwrap()[0],
            InitialConditionItem::String("seq-condition".to_string())
        );
    }

    #[test]
    fn test_expand_include_test_case_not_found() {
        let index = HashMap::new();
        let resolver = DependencyResolver::new(index);

        let mut target = InitialConditions::default();
        let include_ref = IncludeRef {
            id: "TC999".to_string(),
            test_sequence: None,
        };

        let result = resolver.expand_include(&mut target, &include_ref);
        assert!(result.is_err());
        match result.unwrap_err() {
            ResolutionError::TestCaseNotFound { id } => assert_eq!(id, "TC999"),
            _ => panic!("Expected TestCaseNotFound error"),
        }
    }

    #[test]
    fn test_resolve_ref_item() {
        let mut test_case = create_test_case("TC001");
        let mut test_sequence = TestSequence::new(1, "Seq".to_string(), "Desc".to_string());
        let mut step = Step::new(
            1,
            "Referenced step".to_string(),
            "cmd".to_string(),
            "0".to_string(),
            "out".to_string(),
        );
        step.reference = Some("ref-abc".to_string());
        test_sequence.steps.push(step);
        test_case.test_sequences.push(test_sequence);

        let mut index = HashMap::new();
        index.insert("TC001".to_string(), test_case.clone());

        let resolver = DependencyResolver::new(index);

        let mut target_test_case = create_test_case("TC002");
        let mut ic = InitialConditions::default();
        ic.devices.insert(
            "device1".to_string(),
            vec![InitialConditionItem::RefItem {
                reference: "ref-abc".to_string(),
            }],
        );
        target_test_case.initial_conditions = ic;

        let resolved = resolver.resolve(&target_test_case).unwrap();

        let device_items = resolved.initial_conditions.devices.get("device1").unwrap();
        assert_eq!(device_items.len(), 1);
        assert_eq!(
            device_items[0],
            InitialConditionItem::String("Referenced step".to_string())
        );
    }

    #[test]
    fn test_resolve_ref_not_found() {
        let index = HashMap::new();
        let resolver = DependencyResolver::new(index);

        let mut target_test_case = create_test_case("TC002");
        let mut ic = InitialConditions::default();
        ic.devices.insert(
            "device1".to_string(),
            vec![InitialConditionItem::RefItem {
                reference: "missing-ref".to_string(),
            }],
        );
        target_test_case.initial_conditions = ic;

        let result = resolver.resolve(&target_test_case);
        assert!(result.is_err());
        match result.unwrap_err() {
            ResolutionError::RefNotFound { reference } => assert_eq!(reference, "missing-ref"),
            _ => panic!("Expected RefNotFound error"),
        }
    }

    #[test]
    fn test_resolve_test_sequence_ref_inclusive_range() {
        let mut test_case = create_test_case("TC001");
        let mut test_sequence = TestSequence::new(1, "Seq".to_string(), "Desc".to_string());

        test_sequence.steps.push(Step::new(
            1,
            "Step 1".to_string(),
            "cmd1".to_string(),
            "0".to_string(),
            "out".to_string(),
        ));
        test_sequence.steps.push(Step::new(
            2,
            "Step 2".to_string(),
            "cmd2".to_string(),
            "0".to_string(),
            "out".to_string(),
        ));
        test_sequence.steps.push(Step::new(
            3,
            "Step 3".to_string(),
            "cmd3".to_string(),
            "0".to_string(),
            "out".to_string(),
        ));

        test_case.test_sequences.push(test_sequence);

        let mut ic = InitialConditions::default();
        ic.devices.insert(
            "device1".to_string(),
            vec![InitialConditionItem::TestSequenceRef {
                test_sequence: TestSequenceRefTarget {
                    id: 1,
                    step: "[1,3]".to_string(),
                },
            }],
        );
        test_case.initial_conditions = ic;

        let mut index = HashMap::new();
        index.insert("TC001".to_string(), test_case.clone());

        let resolver = DependencyResolver::new(index);
        let resolved = resolver.resolve(&test_case).unwrap();

        let device_items = resolved.initial_conditions.devices.get("device1").unwrap();
        assert_eq!(device_items.len(), 3);
        assert_eq!(
            device_items[0],
            InitialConditionItem::String("Step 1".to_string())
        );
        assert_eq!(
            device_items[1],
            InitialConditionItem::String("Step 2".to_string())
        );
        assert_eq!(
            device_items[2],
            InitialConditionItem::String("Step 3".to_string())
        );
    }

    #[test]
    fn test_resolve_test_sequence_ref_exclusive_range() {
        let mut test_case = create_test_case("TC001");
        let mut test_sequence = TestSequence::new(1, "Seq".to_string(), "Desc".to_string());

        test_sequence.steps.push(Step::new(
            1,
            "Step 1".to_string(),
            "cmd1".to_string(),
            "0".to_string(),
            "out".to_string(),
        ));
        test_sequence.steps.push(Step::new(
            2,
            "Step 2".to_string(),
            "cmd2".to_string(),
            "0".to_string(),
            "out".to_string(),
        ));
        test_sequence.steps.push(Step::new(
            3,
            "Step 3".to_string(),
            "cmd3".to_string(),
            "0".to_string(),
            "out".to_string(),
        ));

        test_case.test_sequences.push(test_sequence);

        let mut ic = InitialConditions::default();
        ic.devices.insert(
            "device1".to_string(),
            vec![InitialConditionItem::TestSequenceRef {
                test_sequence: TestSequenceRefTarget {
                    id: 1,
                    step: "(1,3)".to_string(),
                },
            }],
        );
        test_case.initial_conditions = ic;

        let mut index = HashMap::new();
        index.insert("TC001".to_string(), test_case.clone());

        let resolver = DependencyResolver::new(index);
        let resolved = resolver.resolve(&test_case).unwrap();

        let device_items = resolved.initial_conditions.devices.get("device1").unwrap();
        assert_eq!(device_items.len(), 1);
        assert_eq!(
            device_items[0],
            InitialConditionItem::String("Step 2".to_string())
        );
    }

    #[test]
    fn test_resolve_test_sequence_ref_explicit_list() {
        let mut test_case = create_test_case("TC001");
        let mut test_sequence = TestSequence::new(1, "Seq".to_string(), "Desc".to_string());

        test_sequence.steps.push(Step::new(
            1,
            "Step 1".to_string(),
            "cmd1".to_string(),
            "0".to_string(),
            "out".to_string(),
        ));
        test_sequence.steps.push(Step::new(
            2,
            "Step 2".to_string(),
            "cmd2".to_string(),
            "0".to_string(),
            "out".to_string(),
        ));
        test_sequence.steps.push(Step::new(
            5,
            "Step 5".to_string(),
            "cmd5".to_string(),
            "0".to_string(),
            "out".to_string(),
        ));

        test_case.test_sequences.push(test_sequence);

        let mut ic = InitialConditions::default();
        ic.devices.insert(
            "device1".to_string(),
            vec![InitialConditionItem::TestSequenceRef {
                test_sequence: TestSequenceRefTarget {
                    id: 1,
                    step: "2, 5".to_string(),
                },
            }],
        );
        test_case.initial_conditions = ic;

        let mut index = HashMap::new();
        index.insert("TC001".to_string(), test_case.clone());

        let resolver = DependencyResolver::new(index);
        let resolved = resolver.resolve(&test_case).unwrap();

        let device_items = resolved.initial_conditions.devices.get("device1").unwrap();
        assert_eq!(device_items.len(), 2);
        assert_eq!(
            device_items[0],
            InitialConditionItem::String("Step 2".to_string())
        );
        assert_eq!(
            device_items[1],
            InitialConditionItem::String("Step 5".to_string())
        );
    }

    #[test]
    fn test_resolve_removes_include_field() {
        let mut source_test_case = create_test_case("TC001");
        let mut general_ic = InitialConditions::default();
        general_ic.devices.insert(
            "device1".to_string(),
            vec![InitialConditionItem::String("condition1".to_string())],
        );
        source_test_case.general_initial_conditions = general_ic;

        let mut target_test_case = create_test_case("TC002");
        let ic = InitialConditions {
            include: Some(vec![IncludeRef {
                id: "TC001".to_string(),
                test_sequence: None,
            }]),
            ..Default::default()
        };
        target_test_case.initial_conditions = ic;

        let mut index = HashMap::new();
        index.insert("TC001".to_string(), source_test_case);
        index.insert("TC002".to_string(), target_test_case.clone());

        let resolver = DependencyResolver::new(index);
        let resolved = resolver.resolve(&target_test_case).unwrap();

        assert!(resolved.initial_conditions.include.is_none());
        assert_eq!(resolved.initial_conditions.devices.len(), 1);
    }

    #[test]
    fn test_resolve_idempotent() {
        let mut source_test_case = create_test_case("TC001");
        let mut general_ic = InitialConditions::default();
        general_ic.devices.insert(
            "device1".to_string(),
            vec![InitialConditionItem::String("condition1".to_string())],
        );
        source_test_case.general_initial_conditions = general_ic;

        let mut target_test_case = create_test_case("TC002");
        let ic = InitialConditions {
            include: Some(vec![IncludeRef {
                id: "TC001".to_string(),
                test_sequence: None,
            }]),
            ..Default::default()
        };
        target_test_case.initial_conditions = ic;

        let mut index = HashMap::new();
        index.insert("TC001".to_string(), source_test_case);
        index.insert("TC002".to_string(), target_test_case.clone());

        let resolver = DependencyResolver::new(index);

        // First resolution
        let resolved1 = resolver.resolve(&target_test_case).unwrap();

        // Second resolution on already-resolved test case
        let resolved2 = resolver.resolve(&resolved1).unwrap();

        // Should produce identical output
        assert_eq!(
            serde_yaml::to_string(&resolved1).unwrap(),
            serde_yaml::to_string(&resolved2).unwrap()
        );
    }

    #[test]
    fn test_resolve_multiple_includes() {
        let mut tc1 = create_test_case("TC001");
        let mut ic1 = InitialConditions::default();
        ic1.devices.insert(
            "device1".to_string(),
            vec![InitialConditionItem::String("condition1".to_string())],
        );
        tc1.general_initial_conditions = ic1;

        let mut tc2 = create_test_case("TC002");
        let mut ic2 = InitialConditions::default();
        ic2.devices.insert(
            "device2".to_string(),
            vec![InitialConditionItem::String("condition2".to_string())],
        );
        tc2.general_initial_conditions = ic2;

        let mut target = create_test_case("TC003");
        let ic = InitialConditions {
            include: Some(vec![
                IncludeRef {
                    id: "TC001".to_string(),
                    test_sequence: None,
                },
                IncludeRef {
                    id: "TC002".to_string(),
                    test_sequence: None,
                },
            ]),
            ..Default::default()
        };
        target.initial_conditions = ic;

        let mut index = HashMap::new();
        index.insert("TC001".to_string(), tc1);
        index.insert("TC002".to_string(), tc2);
        index.insert("TC003".to_string(), target.clone());

        let resolver = DependencyResolver::new(index);
        let resolved = resolver.resolve(&target).unwrap();

        assert_eq!(resolved.initial_conditions.devices.len(), 2);
        assert!(resolved.initial_conditions.devices.contains_key("device1"));
        assert!(resolved.initial_conditions.devices.contains_key("device2"));
    }

    #[test]
    fn test_resolve_mixed_items() {
        let mut tc1 = create_test_case("TC001");
        let mut test_sequence = TestSequence::new(1, "Seq".to_string(), "Desc".to_string());
        let mut step = Step::new(
            1,
            "Referenced step".to_string(),
            "cmd".to_string(),
            "0".to_string(),
            "out".to_string(),
        );
        step.reference = Some("ref-123".to_string());
        test_sequence.steps.push(step);

        test_sequence.steps.push(Step::new(
            2,
            "Step 2".to_string(),
            "cmd2".to_string(),
            "0".to_string(),
            "out".to_string(),
        ));
        test_sequence.steps.push(Step::new(
            3,
            "Step 3".to_string(),
            "cmd3".to_string(),
            "0".to_string(),
            "out".to_string(),
        ));

        tc1.test_sequences.push(test_sequence);

        let mut target = create_test_case("TC002");
        let mut ic = InitialConditions::default();
        ic.devices.insert(
            "device1".to_string(),
            vec![
                InitialConditionItem::String("plain string".to_string()),
                InitialConditionItem::RefItem {
                    reference: "ref-123".to_string(),
                },
                InitialConditionItem::TestSequenceRef {
                    test_sequence: TestSequenceRefTarget {
                        id: 1,
                        step: "2,3".to_string(),
                    },
                },
            ],
        );
        target.initial_conditions = ic;
        target
            .test_sequences
            .push(TestSequence::new(1, "Seq".to_string(), "Desc".to_string()));
        target
            .test_sequences
            .last_mut()
            .unwrap()
            .steps
            .push(Step::new(
                2,
                "Step 2".to_string(),
                "cmd2".to_string(),
                "0".to_string(),
                "out".to_string(),
            ));
        target
            .test_sequences
            .last_mut()
            .unwrap()
            .steps
            .push(Step::new(
                3,
                "Step 3".to_string(),
                "cmd3".to_string(),
                "0".to_string(),
                "out".to_string(),
            ));

        let mut index = HashMap::new();
        index.insert("TC001".to_string(), tc1);
        index.insert("TC002".to_string(), target.clone());

        let resolver = DependencyResolver::new(index);
        let resolved = resolver.resolve(&target).unwrap();

        let device_items = resolved.initial_conditions.devices.get("device1").unwrap();
        assert_eq!(device_items.len(), 4);
        assert_eq!(
            device_items[0],
            InitialConditionItem::String("plain string".to_string())
        );
        assert_eq!(
            device_items[1],
            InitialConditionItem::String("Referenced step".to_string())
        );
        assert_eq!(
            device_items[2],
            InitialConditionItem::String("Step 2".to_string())
        );
        assert_eq!(
            device_items[3],
            InitialConditionItem::String("Step 3".to_string())
        );
    }

    #[test]
    fn test_resolve_test_sequence_initial_conditions() {
        let mut tc1 = create_test_case("TC001");
        let mut general_ic = InitialConditions::default();
        general_ic.devices.insert(
            "device1".to_string(),
            vec![InitialConditionItem::String(
                "general condition".to_string(),
            )],
        );
        tc1.general_initial_conditions = general_ic;

        let mut target = create_test_case("TC002");
        let mut test_sequence = TestSequence::new(1, "Seq".to_string(), "Desc".to_string());
        let seq_ic = InitialConditions {
            include: Some(vec![IncludeRef {
                id: "TC001".to_string(),
                test_sequence: None,
            }]),
            ..Default::default()
        };
        test_sequence.initial_conditions = seq_ic;
        target.test_sequences.push(test_sequence);

        let mut index = HashMap::new();
        index.insert("TC001".to_string(), tc1);
        index.insert("TC002".to_string(), target.clone());

        let resolver = DependencyResolver::new(index);
        let resolved = resolver.resolve(&target).unwrap();

        let seq_ic = &resolved.test_sequences[0].initial_conditions;
        assert!(seq_ic.include.is_none());
        assert_eq!(seq_ic.devices.len(), 1);
        assert!(seq_ic.devices.contains_key("device1"));
    }

    #[test]
    fn test_resolve_test_sequence_not_found() {
        let mut target = create_test_case("TC001");
        let mut ic = InitialConditions::default();
        ic.devices.insert(
            "device1".to_string(),
            vec![InitialConditionItem::TestSequenceRef {
                test_sequence: TestSequenceRefTarget {
                    id: 99,
                    step: "[1,3]".to_string(),
                },
            }],
        );
        target.initial_conditions = ic;

        let mut index = HashMap::new();
        index.insert("TC001".to_string(), target.clone());

        let resolver = DependencyResolver::new(index);
        let result = resolver.resolve(&target);

        assert!(result.is_err());
        match result.unwrap_err() {
            ResolutionError::TestSequenceNotFound {
                test_case_id,
                seq_id,
            } => {
                assert_eq!(test_case_id, "TC001");
                assert_eq!(seq_id, 99);
            }
            _ => panic!("Expected TestSequenceNotFound error"),
        }
    }

    #[test]
    fn test_resolve_invalid_step_range() {
        let mut target = create_test_case("TC001");
        let mut test_sequence = TestSequence::new(1, "Seq".to_string(), "Desc".to_string());
        test_sequence.steps.push(Step::new(
            1,
            "Step 1".to_string(),
            "cmd1".to_string(),
            "0".to_string(),
            "out".to_string(),
        ));
        target.test_sequences.push(test_sequence);

        let mut ic = InitialConditions::default();
        ic.devices.insert(
            "device1".to_string(),
            vec![InitialConditionItem::TestSequenceRef {
                test_sequence: TestSequenceRefTarget {
                    id: 1,
                    step: "[1,abc]".to_string(),
                },
            }],
        );
        target.initial_conditions = ic;

        let mut index = HashMap::new();
        index.insert("TC001".to_string(), target.clone());

        let resolver = DependencyResolver::new(index);
        let result = resolver.resolve(&target);

        assert!(result.is_err());
        match result.unwrap_err() {
            ResolutionError::InvalidStepRange { expression, .. } => {
                assert_eq!(expression, "[1,abc]");
            }
            _ => panic!("Expected InvalidStepRange error"),
        }
    }
}
