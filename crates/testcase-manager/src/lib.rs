pub mod builder;
pub mod cli;
pub mod complex_structure_editor;
pub mod creator;
pub mod database;
pub mod dependency_resolver;
pub mod dependency_validator;
pub mod editor;
pub mod fuzzy;
pub mod git;
pub mod junit_xml_validator;
pub mod oracle;
pub mod orchestrator;
pub mod parser;
pub mod prompts;
pub mod recovery;
pub mod sample;
pub mod storage;
pub mod test_run_storage;
pub mod ui;
pub mod validation;
pub mod verification;

// Re-export from testcase-execution crate
pub use builder::TestCaseBuilder;
pub use cli::Cli;
pub use complex_structure_editor::ComplexStructureEditor;
pub use creator::TestCaseCreator;
pub use database::ConditionDatabase;
pub use dependency_resolver::{DependencyResolver, ResolutionError};
pub use dependency_validator::{
    validate_cross_file_dependencies, DependencyError, DependencyErrorType, DependencyValidator,
};
pub use editor::TestCaseEditor;
pub use fuzzy::TestCaseFuzzyFinder;
pub use git::{CommitInfo, GitManager};
pub use junit_xml_validator::validate_junit_xml;
pub use oracle::{AnswerVariant, HardcodedOracle, MenuCliOracle, Oracle, TtyCliOracle};
pub use parser::{SearchableCollections, TestCaseParser};
pub use prompts::{Prompts, TestCaseMetadata};
pub use recovery::{RecoveryManager, RecoveryState};
pub use sample::SampleData;
pub use storage::{TestCaseFilter, TestCaseFilterer, TestCaseStorage};
pub use test_run_storage::TestRunStorage;
pub use testcase_common::{
    log_yaml_parse_error, resolve_schema_from_payload, CommitMessageTemplates, Config,
    EditorConfig, GitAuthorInfo, JsonEscapingConfig, JsonEscapingMethod, ScriptGenerationConfig,
};
pub use testcase_execution::{
    parse_bdd_statement, BddStepDefinition, BddStepMatcher, BddStepRegistry, ConfirmPrompt,
    DefaultConfirmPrompt, TestExecutor, VarHydrator,
};
pub use testcase_models::ActualResult;
pub use testcase_models::CaptureVar;
pub use testcase_models::CaptureVarsFormat;
pub use testcase_models::EnvVarConfig;
pub use testcase_models::EnvVariable;
pub use testcase_models::Expected;
pub use testcase_models::FieldDiff;
pub use testcase_models::FileValidationStatus;
pub use testcase_models::GeneralVerification;
pub use testcase_models::HookConfig;
pub use testcase_models::HookType;
pub use testcase_models::Hooks;
pub use testcase_models::IncludeRef;
pub use testcase_models::InitialConditionItem;
pub use testcase_models::InitialConditions;
pub use testcase_models::OnError;
pub use testcase_models::Prerequisite;
pub use testcase_models::PrerequisiteType;
pub use testcase_models::Step;
pub use testcase_models::StepExecutionResult;
pub use testcase_models::TestCase;
pub use testcase_models::TestCaseFileInfo;
pub use testcase_models::TestExecutionLog;
pub use testcase_models::TestReportOutput;
pub use testcase_models::TestReportResults;
pub use testcase_models::TestReportSummary;
pub use testcase_models::TestRun;
pub use testcase_models::TestRunMetadata;
pub use testcase_models::TestRunStatus;
pub use testcase_models::TestSequence;
pub use testcase_models::TestSequenceRefTarget;
pub use testcase_models::TestStepExecutionEntry;
pub use testcase_models::TestSuite;
pub use testcase_models::ValidationErrorDetail;
pub use testcase_models::Verification;
pub use testcase_models::VerificationExpression;
pub use testcase_models::VerificationReport;
pub use testcase_models::VerificationStatus;
pub use ui::{print_title, TitleStyle};
pub use validation::SchemaValidator;
pub use verification::StorageTestVerifier;

// Re-export from testcase-verification crate
pub use testcase_verification::{
    BatchVerificationReport, ContainerReport, ContainerReportConfig, ContainerReportMetadata,
    LogCleaner, MatchStrategy, SequenceVerificationResult, StepVerificationResultEnum,
    TemplateCategory, TestVerifier, VerificationTemplate, VerificationTemplateLibrary,
};

// Legacy exports (for backward compatibility)
pub use testcase_verification::DiffDetail;
pub use testcase_verification::ExecutionVerificationResult;
pub use testcase_verification::StepVerificationResult;
pub use testcase_verification::TestCaseVerificationResult;
pub use testcase_verification::TestExecutionLog as VerificationTestExecutionLog;
pub use testcase_verification::VerificationDiff;
