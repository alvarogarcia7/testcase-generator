use crate::storage::TestCaseStorage;
use anyhow::Result;
use std::collections::HashSet;
use std::path::Path;

/// Database of conditions extracted from test case files
///
/// This database loads all test cases from a directory and extracts
/// unique general initial conditions and initial conditions for use
/// in fuzzy search selection when creating new test cases.
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

        let mut general_conditions_set = HashSet::new();
        let mut initial_conditions_set = HashSet::new();
        let mut device_names_set = HashSet::new();

        for test_case in test_cases {
            for general_cond in test_case.general_initial_conditions {
                for euicc_cond in general_cond.euicc {
                    general_conditions_set.insert(euicc_cond);
                }
            }

            for euicc_cond in test_case.initial_conditions.euicc {
                initial_conditions_set.insert(euicc_cond);
            }

            // Extract device name from initial_conditions structure
            // The device name is "eUICC" by default but we should extract all device names used
            device_names_set.insert("eUICC".to_string());

            // Also extract from sequence-level initial conditions
            for sequence in &test_case.test_sequences {
                for seq_init_cond in &sequence.initial_conditions {
                    for euicc_cond in &seq_init_cond.euicc {
                        initial_conditions_set.insert(euicc_cond.clone());
                    }
                    // Extract device names from sequence initial conditions too
                    device_names_set.insert("eUICC".to_string());
                }
            }
        }

        let mut general_conditions: Vec<String> = general_conditions_set.into_iter().collect();
        let mut initial_conditions: Vec<String> = initial_conditions_set.into_iter().collect();
        let mut device_names: Vec<String> = device_names_set.into_iter().collect();

        general_conditions.sort();
        initial_conditions.sort();
        device_names.sort();

        Ok(Self {
            general_conditions,
            initial_conditions,
            device_names,
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
