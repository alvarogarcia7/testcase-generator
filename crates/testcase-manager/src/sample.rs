pub struct SampleData {
    sequence_counter: std::cell::Cell<usize>,
    step_counter: std::cell::Cell<usize>,
}

impl SampleData {
    pub fn new() -> Self {
        Self {
            sequence_counter: std::cell::Cell::new(0),
            step_counter: std::cell::Cell::new(0),
        }
    }

    pub fn metadata_requirement(&self) -> String {
        "SGP.22".to_string()
    }

    pub fn metadata_item(&self) -> i64 {
        4
    }

    pub fn metadata_tc(&self) -> i64 {
        42
    }

    pub fn metadata_id(&self) -> String {
        "SGP.22_4.4.2".to_string()
    }

    pub fn metadata_description(&self) -> String {
        "Sample test case for demonstration".to_string()
    }

    pub fn confirm_add_general_conditions(&self) -> bool {
        true
    }

    pub fn confirm_add_initial_conditions(&self) -> bool {
        true
    }

    pub fn confirm_commit(&self) -> bool {
        true
    }

    pub fn confirm_add_another_sequence(&self) -> bool {
        self.sequence_counter.get() < 2
    }

    pub fn confirm_add_steps_to_sequence(&self) -> bool {
        true
    }

    pub fn confirm_add_another_step(&self) -> bool {
        self.step_counter.get() < 3
    }

    pub fn sequence_name(&self) -> String {
        let counter = self.sequence_counter.get();
        self.sequence_counter.set(counter + 1);
        format!("Sample Sequence {}", counter + 1)
    }

    pub fn sequence_description(&self) -> Option<String> {
        Some(format!(
            "This is a sample sequence description for sequence {}",
            self.sequence_counter.get()
        ))
    }

    pub fn confirm_edit_description_in_editor(&self) -> bool {
        false
    }

    pub fn confirm_add_sequence_initial_conditions(&self) -> bool {
        false
    }

    pub fn confirm_use_fuzzy_search(&self) -> bool {
        false
    }

    pub fn confirm_use_database(&self) -> bool {
        false
    }

    pub fn step_description(&self) -> String {
        let counter = self.step_counter.get();
        self.step_counter.set(counter + 1);
        format!("Sample step {}: Perform test action", counter + 1)
    }

    pub fn confirm_is_manual_step(&self) -> bool {
        false
    }

    pub fn step_command(&self) -> String {
        "AT+COMMAND".to_string()
    }

    pub fn confirm_include_success_field(&self) -> bool {
        true
    }

    pub fn expected_success_value(&self) -> bool {
        true
    }

    pub fn expected_result(&self) -> String {
        "OK".to_string()
    }

    pub fn expected_output(&self) -> String {
        "Command executed successfully".to_string()
    }

    pub fn general_condition(&self, index: usize) -> String {
        match index {
            0 => "eUICC is in state ENABLED".to_string(),
            1 => "Test profile is installed".to_string(),
            _ => "".to_string(),
        }
    }

    pub fn initial_condition_device_name(&self) -> String {
        "eUICC".to_string()
    }

    pub fn initial_condition(&self, index: usize) -> String {
        match index {
            0 => "Profile is in state DISABLED".to_string(),
            1 => "No active profile session".to_string(),
            _ => "".to_string(),
        }
    }

    pub fn database_path(&self) -> String {
        "data".to_string()
    }

    pub fn input_optional_stop(&self, index: usize) -> bool {
        index >= 2
    }

    pub fn confirm_keep_defaults(&self) -> bool {
        true
    }

    pub fn confirm_retry(&self) -> bool {
        true
    }

    pub fn confirm_continue_without(&self) -> bool {
        false
    }

    pub fn confirm_final_commit(&self) -> bool {
        true
    }

    pub fn reset_counters(&self) {
        self.sequence_counter.set(0);
        self.step_counter.set(0);
    }
}

impl Default for SampleData {
    fn default() -> Self {
        Self::new()
    }
}

impl SampleData {
    /// Create a HardcodedOracle with pre-populated answers for complete workflow
    pub fn create_oracle_for_complete(&self) -> crate::oracle::HardcodedOracle {
        use crate::oracle::{AnswerVariant, HardcodedOracle};
        use std::collections::VecDeque;

        let mut answers = VecDeque::new();

        // Metadata prompts
        answers.push_back(AnswerVariant::String(self.metadata_requirement()));
        answers.push_back(AnswerVariant::Int(self.metadata_item()));
        answers.push_back(AnswerVariant::Int(self.metadata_tc()));
        answers.push_back(AnswerVariant::String(self.metadata_id()));
        answers.push_back(AnswerVariant::String(self.metadata_description()));

        // Commit metadata
        answers.push_back(AnswerVariant::Bool(self.confirm_commit()));

        // Add general initial conditions
        answers.push_back(AnswerVariant::Bool(self.confirm_add_general_conditions()));

        // General conditions prompts (if adding)
        if self.confirm_add_general_conditions() {
            // Assuming we'll add 2 general conditions
            answers.push_back(AnswerVariant::String(self.general_condition(0)));
            answers.push_back(AnswerVariant::String(self.general_condition(1)));
            answers.push_back(AnswerVariant::String("".to_string())); // Stop adding

            // Commit general conditions
            answers.push_back(AnswerVariant::Bool(self.confirm_commit()));
        }

        // Add initial conditions
        answers.push_back(AnswerVariant::Bool(self.confirm_add_initial_conditions()));

        // Initial conditions prompts (if adding)
        if self.confirm_add_initial_conditions() {
            answers.push_back(AnswerVariant::String(self.initial_condition_device_name()));
            answers.push_back(AnswerVariant::String(self.initial_condition(0)));
            answers.push_back(AnswerVariant::String(self.initial_condition(1)));
            answers.push_back(AnswerVariant::String("".to_string())); // Stop adding

            // Commit initial conditions
            answers.push_back(AnswerVariant::Bool(self.confirm_commit()));
        }

        // Test sequences loop (2 sequences)
        for seq in 0..2 {
            self.sequence_counter.set(seq);

            // Add sequence
            answers.push_back(AnswerVariant::String(self.sequence_name()));
            if let Some(desc) = self.sequence_description() {
                answers.push_back(AnswerVariant::String(desc));
            }
            answers.push_back(AnswerVariant::Bool(
                self.confirm_edit_description_in_editor(),
            ));
            answers.push_back(AnswerVariant::Bool(
                self.confirm_add_sequence_initial_conditions(),
            ));

            // Commit sequence
            answers.push_back(AnswerVariant::Bool(self.confirm_commit()));

            // Add steps to sequence
            answers.push_back(AnswerVariant::Bool(self.confirm_add_steps_to_sequence()));

            // Steps loop (3 steps per sequence)
            if self.confirm_add_steps_to_sequence() {
                for step in 0..3 {
                    self.step_counter.set(step);

                    answers.push_back(AnswerVariant::String(self.step_description()));
                    answers.push_back(AnswerVariant::Bool(self.confirm_is_manual_step()));
                    answers.push_back(AnswerVariant::String(self.step_command()));

                    // Expected prompts
                    answers.push_back(AnswerVariant::Bool(self.confirm_include_success_field()));
                    if self.confirm_include_success_field() {
                        answers.push_back(AnswerVariant::Bool(self.expected_success_value()));
                    }
                    answers.push_back(AnswerVariant::String(self.expected_result()));
                    answers.push_back(AnswerVariant::String(self.expected_output()));

                    // Commit step
                    answers.push_back(AnswerVariant::Bool(self.confirm_commit()));

                    // Add another step
                    answers.push_back(AnswerVariant::Bool(self.confirm_add_another_step()));
                }
            }

            // Add another sequence
            answers.push_back(AnswerVariant::Bool(self.confirm_add_another_sequence()));
        }

        // Final commit
        answers.push_back(AnswerVariant::Bool(self.confirm_final_commit()));

        HardcodedOracle::new(answers)
    }
}
