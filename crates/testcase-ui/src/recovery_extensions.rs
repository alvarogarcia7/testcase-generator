use anyhow::Result;
use testcase_storage::RecoveryManager;

/// Extension trait for RecoveryManager that adds prompt-specific functionality
pub trait RecoveryManagerPromptsExt {
    /// Prompt the user for recovery with interactive prompts
    fn prompt_for_recovery(&self) -> Result<bool>;
}

impl RecoveryManagerPromptsExt for RecoveryManager {
    fn prompt_for_recovery(&self) -> Result<bool> {
        use crate::Prompts;

        if !self.recovery_file_exists() {
            return Ok(false);
        }

        if let Some(state) = self.load_state()? {
            self.display_recovery_info(&state);

            let resume = Prompts::confirm("Resume from saved state?")?;

            if !resume {
                let delete = Prompts::confirm("Delete recovery file?")?;
                if delete {
                    self.delete_recovery_file()?;
                    println!("✓ Recovery file deleted");
                }
            }

            return Ok(resume);
        }

        Ok(false)
    }
}
