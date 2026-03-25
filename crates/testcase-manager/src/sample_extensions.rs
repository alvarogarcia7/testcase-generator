use testcase_storage::SampleData;

/// Extension trait for SampleData that adds oracle-specific functionality
pub trait SampleDataOracleExt {
    /// Create a HardcodedOracle with pre-populated answers for complete workflow
    fn create_oracle_for_complete(&self) -> crate::oracle::HardcodedOracle;
}

impl SampleDataOracleExt for SampleData {
    fn create_oracle_for_complete(&self) -> crate::oracle::HardcodedOracle {
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
        for _seq in 0..2 {
            // Note: We can't actually modify the counter in an immutable reference
            // but this is test/sample code, so it's okay

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
                for _step in 0..3 {
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
