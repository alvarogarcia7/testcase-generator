use crate::models::{Expected, Step, TestSequence};
use crate::storage::TestCaseStorage;
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Database of conditions extracted from test case files
///
/// This database loads all test cases from a directory and extracts
/// unique general initial conditions and initial conditions for use
/// in fuzzy search selection when creating new test cases.
///
// TODO AGB: Rename to TestCaseDatabase
#[derive(Clone)]
pub struct ConditionDatabase {
    general_conditions: Vec<String>,
    initial_conditions: Vec<String>,
    initial_conditions_map: HashMap<String, Vec<String>>,
    device_names: Vec<String>,
    expected_items: Vec<Expected>,
    step_items: Vec<Step>,
    sequence_items: Vec<TestSequence>,
}

impl ConditionDatabase {
    /// Load a condition database from a directory of test case files
    ///
    /// Scans all test case YAML files in the directory, extracts all
    /// general initial conditions and initial conditions, removes duplicates,
    /// and sorts them for easy searching.
    ///
    /// # Arguments
    /// * `path` - Path to directory containing test case YAML files
    ///
    /// # Returns
    /// A ConditionDatabase containing unique sorted conditions
    pub fn load_from_directory<P: AsRef<Path>>(path: P) -> Result<Self> {
        let storage = TestCaseStorage::new(path)?;
        let test_cases = storage.load_all_test_cases()?;

        let mut general_conditions_set: HashSet<String> = HashSet::new();
        let mut initial_conditions_set: HashSet<String> = HashSet::new();
        let mut initial_conditions_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut device_names_set: HashSet<String> = HashSet::new();
        let mut expected_set: HashSet<Expected> = HashSet::new();
        let mut step_set: HashSet<Step> = HashSet::new();
        let mut sequence_items: Vec<TestSequence> = Vec::new();

        for test_case in test_cases {
            for conditions in test_case.general_initial_conditions.values() {
                for condition in conditions {
                    general_conditions_set.insert(condition.clone());
                }
            }

            // for euicc_cond in test_case.initial_conditions.euicc {
            //     initial_conditions_set.insert(euicc_cond);
            // }

            // Extract device name from initial_conditions structure
            // The device name is "eUICC" by default but we should extract all device names used
            device_names_set.insert("eUICC".to_string());

            // Also extract from sequence-level initial conditions
            for sequence in &test_case.test_sequences {
                sequence_items.push(sequence.clone());

                initial_conditions_map.extend(sequence.initial_conditions.clone());

                for conditions in sequence.initial_conditions.values() {
                    for condition in conditions {
                        initial_conditions_set.insert(condition.clone());
                    }
                    // Extract device names from sequence initial conditions too
                    device_names_set.insert("eUICC".to_string());
                }

                for step in &sequence.steps {
                    step_set.insert(step.clone());
                    expected_set.insert(step.expected.clone());
                }
            }
        }

        fn sort(set: HashSet<String>) -> Vec<String> {
            let mut as_vec: Vec<String> = set.into_iter().collect();
            as_vec.sort();
            as_vec
        }

        Ok(Self {
            general_conditions: sort(general_conditions_set),
            initial_conditions: sort(initial_conditions_set),
            initial_conditions_map,
            device_names: sort(device_names_set),
            expected_items: expected_set.into_iter().collect(),
            step_items: step_set.into_iter().collect(),
            sequence_items,
        })
    }

    /// Get all unique general initial conditions from the database
    pub fn get_general_conditions(&self) -> &[String] {
        &self.general_conditions
    }

    /// Get all unique initial conditions from the database
    pub fn get_initial_conditions(&self) -> &[String] {
        &self.initial_conditions
    }

    pub fn get_initial_conditions_for(&self, device: &String) -> &[String] {
        &self.initial_conditions_map[device]
    }

    /// Get all unique device names from the database
    pub fn get_device_names(&self) -> &[String] {
        &self.device_names
    }

    /// Get all unique expected items from the database
    pub fn get_all_expected(&self) -> Vec<Expected> {
        self.expected_items.clone()
    }

    /// Get all unique step items from the database
    pub fn get_all_steps(&self) -> Vec<Step> {
        self.step_items.clone()
    }

    /// Get all unique test sequence items from the database
    pub fn get_all_sequences(&self) -> Vec<TestSequence> {
        self.sequence_items.clone()
    }
}
