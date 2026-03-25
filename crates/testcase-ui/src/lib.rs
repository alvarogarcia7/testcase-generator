pub mod builder;
pub mod complex_structure_editor;
pub mod creator;
pub mod editor;
pub mod fuzzy;
pub mod oracle;
pub mod prompts;
pub mod recovery_extensions;
pub mod ui;
pub mod validation;

// Re-export git functionality from testcase-git crate
pub use testcase_git::{CommitInfo, GitManager};

// Export main items as requested
pub use builder::TestCaseBuilder;
pub use complex_structure_editor::ComplexStructureEditor;
pub use creator::TestCaseCreator;
pub use editor::{EditorFlow, TestCaseEditor};
pub use fuzzy::{MultiInput, TestCaseFuzzyFinder};
pub use oracle::{AnswerVariant, HardcodedOracle, MenuCliOracle, Oracle, TtyCliOracle};
pub use prompts::{Prompts, TestCaseMetadata};
pub use ui::{print_title, TitleStyle};
