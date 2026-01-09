use crate::storage::TestCaseStorage;
use anyhow::Result;
use std::collections::HashSet;
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
    device_names: Vec<String>,
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
        let mut device_names_set: HashSet<String> = HashSet::new();

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
                for conditions in sequence.initial_conditions.values() {
                    for condition in conditions {
                        initial_conditions_set.insert(condition.clone());
                    }
                    // Extract device names from sequence initial conditions too
                    device_names_set.insert("eUICC".to_string());
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
            device_names: sort(device_names_set),
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

    /// Get all unique device names from the database
    pub fn get_device_names(&self) -> &[String] {
        &self.device_names
    }
}
