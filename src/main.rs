use std::sync::Arc;

use anyhow::{Context, Error, Result};
use clap::Parser;
use testcase_manager::fuzzy::MultiInput;
use testcase_manager::fuzzy::MultiInput::Input;
use testcase_manager::yaml_utils::log_yaml_parse_error;
use testcase_manager::{
    cli::{Cli, Commands, GitCommands},
    print_title, ConditionDatabase, GitManager, Oracle, Prompts, SampleData, TestCase,
    TestCaseBuilder, TestCaseEditor, TestCaseFuzzyFinder, TestCaseMetadata, TestCaseStorage,
    TestSuite, TitleStyle, TtyCliOracle,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize env_logger with appropriate log level
    let log_level = if cli.verbose { "info" } else { "warn" };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    match cli.command {
        Commands::Create { id } => {
            handle_create(&cli.path, id)?;
        }

        Commands::CreateGeneralInitialConditions { id } => {
            handle_create_general_initial_conditions(&cli.path, id)?;
        }

        Commands::Edit { id, fuzzy } => {
            handle_edit(&cli.path, id, fuzzy)?;
        }

        Commands::CreateInteractive { path } => {
            let work_path = path.as_deref().unwrap_or(&cli.path);
            handle_create_interactive(work_path)?;
        }

        Commands::BuildSequences { path } => {
            let work_path = path.as_deref().unwrap_or(&cli.path);
            handle_build_sequences(work_path)?;
        }

        Commands::List {
            tag,
            status,
            priority,
            verbose: _,
        } => {
            handle_list(&cli.path, tag, status, priority, true)?;
        }

        Commands::View { id, fuzzy } => {
            handle_view(&cli.path, id, fuzzy)?;
        }

        Commands::Delete { id, force } => {
            handle_delete(&cli.path, &id, force)?;
        }

        Commands::Validate { file, all } => {
            handle_validate(&cli.path, file, all)?;
        }

        Commands::Search { query } => {
            handle_search(&cli.path, query)?;
        }

        Commands::Export { output, tags } => {
            handle_export(&cli.path, &output, tags)?;
        }

        Commands::Import {
            file,
            skip_validation,
        } => {
            handle_import(&cli.path, &file, skip_validation)?;
        }

        Commands::Git { command } => {
            handle_git(&cli.path, command)?;
        }

        Commands::Init { path, git } => {
            handle_init(path.as_deref().unwrap_or(&cli.path), git)?;
        }

        Commands::AddSteps { path, sequence_id } => {
            let work_path = path.as_deref().unwrap_or(&cli.path);
            handle_add_steps(work_path, sequence_id)?;
        }

        Commands::BuildSequencesWithSteps { path } => {
            let work_path = path.as_deref().unwrap_or(&cli.path);
            handle_build_sequences_with_steps(work_path)?;
        }

        Commands::Complete {
            output,
            commit_prefix,
            sample,
        } => {
            handle_complete(&output, commit_prefix.as_deref(), sample)?;
        }

        Commands::ParseGeneralConditions { database, path } => {
            let work_path = path.as_deref().unwrap_or(&cli.path);
            handle_parse_general_conditions(&database, work_path)?;
        }

        Commands::ParseInitialConditions { database, path } => {
            let work_path = path.as_deref().unwrap_or(&cli.path);
            handle_parse_initial_conditions(&database, work_path)?;
        }

        Commands::ParseInitialConditionsComplex { database, path } => {
            let work_path = path.as_deref().unwrap_or(&cli.path);
            handle_parse_initial_conditions2(&database, work_path)?;
        }

        Commands::ValidateYaml {
            yaml_file,
            schema_file,
        } => {
            handle_validate_yaml(&yaml_file, &schema_file)?;
        }

        Commands::ExportJunitXml { input, output } => {
            handle_export_junit_xml(&input, &output)?;
        }

        Commands::ValidateJunitXml { xml_file } => {
            handle_validate_junit_xml(&xml_file)?;
        }
    }

    Ok(())
}

fn handle_create_general_initial_conditions(base_path: &str, id: Option<String>) -> Result<()> {
    let mut test_case_builder =
        TestCaseBuilder::new_with_recovery(base_path, Arc::new(TtyCliOracle::new()))?;
    let metadata = TestCaseMetadata {
        requirement: "Req Hardcoded 1".to_string(),
        item: 0x101,
        tc: 0x1,
        id: "Hardcoded Id".to_string(),
        description: "Description".to_string(),
    };
    let x = test_case_builder.append_metadata(metadata)?;
    test_case_builder.add_general_initial_conditions(None);
    test_case_builder.save();
    // let storage = TestCaseStorage::new(base_path)?;

    // let requirement = Prompts::input("Requirement")?;
    // let item = Prompts::input_integer("Item")?;
    // let tc = Prompts::input_integer("TC")?;
    //
    // let id = match id {
    //     Some(id) => id,
    //     None => Prompts::input("Test Case ID")?,
    // };
    //
    // let description = Prompts::input("Description")?;

    // let test_case = TestCase::new(metadata.requirement, item, tc, id, description);

    // let file_path = storage.save_test_case(&test_case)?;
    // log::info!("Test case created: {}", file_path.display());

    Ok(())
}
fn handle_create(base_path: &str, id: Option<String>) -> Result<()> {
    let storage = TestCaseStorage::new(base_path)?;

    let requirement = Prompts::input("Requirement")?;
    let item = Prompts::input_integer("Item")?;
    let tc = Prompts::input_integer("TC")?;

    let id = match id {
        Some(id) => id,
        None => Prompts::input("Test Case ID")?,
    };

    let description = Prompts::input("Description")?;

    let test_case = TestCase::new(requirement, item, tc, id, description);

    let file_path = storage.save_test_case(&test_case)?;
    log::info!("Test case created: {}", file_path.display());

    Ok(())
}

fn handle_edit(base_path: &str, id: Option<String>, fuzzy: bool) -> Result<()> {
    let storage = TestCaseStorage::new(base_path)?;

    let test_case = if let Some(id) = id {
        storage.load_test_case_by_id(&id)?
    } else if fuzzy {
        let test_cases = storage.load_all_test_cases()?;
        TestCaseFuzzyFinder::search(&test_cases)?.context("No test case selected")?
    } else {
        let test_cases = storage.load_all_test_cases()?;
        let ids: Vec<String> = test_cases.iter().map(|tc| tc.id.clone()).collect();
        let index = Prompts::select("Select a test case", ids)?;
        let my_int: usize = index.parse().unwrap();
        test_cases[my_int].clone()
    };

    let edited_test_case = TestCaseEditor::edit_test_case(&test_case)?;

    let file_path = storage.save_test_case(&edited_test_case)?;
    log::info!("Test case updated: {}", file_path.display());

    Ok(())
}

fn handle_list(
    base_path: &str,
    _tag: Option<String>,
    _status: Option<String>,
    _priority: Option<String>,
    verbose: bool,
) -> Result<()> {
    let storage = TestCaseStorage::new(base_path)?;
    let file_infos = storage.load_all_with_validation()?;

    if file_infos.is_empty() {
        log::info!("No test case files found in {} (len = 0).", base_path);
        return Ok(());
    }

    if verbose {
        log::info!("Found {} test case file(s):\n", file_infos.len());

        for file_info in file_infos {
            let file_name = file_info
                .path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");

            match &file_info.status {
                testcase_manager::FileValidationStatus::Valid => {
                    if let Some(tc) = &file_info.test_case {
                        log::info!("✓ {} (Valid)", file_name);
                        log::info!("  ID: {}", tc.id);
                        log::info!("  Requirement: {}", tc.requirement);
                        log::info!("  Item: {}", tc.item);
                        log::info!("  TC: {}", tc.tc);
                        log::info!("  Description: {}", tc.description);
                        log::info!("  Test Sequences: {}", tc.test_sequences.len());
                    } else {
                        log::info!("✓ {} (Valid)", file_name);
                    }
                }
                testcase_manager::FileValidationStatus::ParseError { message } => {
                    log::warn!("✗ {} (Parse Error)", file_name);
                    log::warn!("  Error: {}", message);
                }
                testcase_manager::FileValidationStatus::ValidationError { errors } => {
                    log::warn!(
                        "✗ {} (Schema Validation Failed: {} error(s))",
                        file_name,
                        errors.len()
                    );
                    if let Some(tc) = &file_info.test_case {
                        log::warn!("  ID: {}", tc.id);
                        log::warn!("  Requirement: {}", tc.requirement);
                    }
                    for (idx, error) in errors.iter().enumerate().take(3) {
                        log::warn!(
                            "  Error #{}: Path '{}' - {}",
                            idx + 1,
                            error.path,
                            error.constraint
                        );
                    }
                    if errors.len() > 3 {
                        log::warn!("  ... and {} more error(s)", errors.len() - 3);
                    }
                }
            }
            log::info!("");
        }
    } else {
        log::info!("Found {} test case(s):\n", file_infos.len());

        for file_info in file_infos {
            if let Some(tc) = &file_info.test_case {
                let status_marker = match &file_info.status {
                    testcase_manager::FileValidationStatus::Valid => "✓",
                    testcase_manager::FileValidationStatus::ParseError { .. } => "✗",
                    testcase_manager::FileValidationStatus::ValidationError { .. } => "✗",
                };

                log::info!(
                    "{} {:<30} {:<20} Item: {}, TC: {}",
                    status_marker,
                    tc.id,
                    tc.requirement,
                    tc.item,
                    tc.tc
                );
            } else {
                let file_name = file_info
                    .path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                log::warn!("✗ {:<30} (Failed to load)", file_name);
            }
        }
    }

    Ok(())
}

fn handle_view(base_path: &str, id: Option<String>, fuzzy: bool) -> Result<()> {
    let storage = TestCaseStorage::new(base_path)?;

    let test_case = if let Some(id) = id {
        storage.load_test_case_by_id(&id)?
    } else if fuzzy {
        let test_cases = storage.load_all_test_cases()?;
        TestCaseFuzzyFinder::search(&test_cases)?.context("No test case selected")?
    } else {
        let test_cases = storage.load_all_test_cases()?;
        let ids: Vec<String> = test_cases.iter().map(|tc| tc.id.clone()).collect();
        let index = Prompts::select("Select a test case", ids)?;
        let my_int: usize = index.parse().unwrap();
        test_cases[my_int].clone()
    };

    let yaml = serde_yaml::to_string(&test_case)?;
    log::info!("{}", yaml);

    Ok(())
}

fn handle_delete(base_path: &str, id: &str, force: bool) -> Result<()> {
    let storage = TestCaseStorage::new(base_path)?;

    if !storage.test_case_exists(id) {
        anyhow::bail!("Test case not found: {}", id);
    }

    if !force {
        let confirm = Prompts::confirm(&format!("Delete test case '{}'?", id))?;
        if !confirm {
            log::info!("Cancelled.");
            return Ok(());
        }
    }

    storage.delete_test_case(id)?;
    log::info!("Test case deleted: {}", id);

    Ok(())
}

fn handle_validate(base_path: &str, file: Option<String>, all: bool) -> Result<()> {
    let storage = TestCaseStorage::new(base_path)?;

    if let Some(file_path) = file {
        let test_case = storage.load_test_case(&file_path)?;
        let yaml = serde_yaml::to_string(&test_case)?;
        let validator = testcase_manager::SchemaValidator::new()?;

        let validation_errors = validator.validate_with_details(&yaml)?;

        if validation_errors.is_empty() {
            log::info!("✓ Valid: {}", file_path);
        } else {
            log::error!("✗ Invalid: {}", file_path);
            log::error!("\nValidation Errors:");
            for (idx, error) in validation_errors.iter().enumerate() {
                log::error!("\n  Error #{}: Path '{}'", idx + 1, error.path);
                log::error!("    Constraint: {}", error.constraint);
                log::error!("    Expected: {}", error.expected_constraint);
                log::error!("    Found: {}", error.found_value);
            }
            anyhow::bail!(
                "Validation failed with {} error(s)",
                validation_errors.len()
            );
        }
    } else if all {
        let file_infos = storage.load_all_with_validation()?;
        let mut error_count = 0;
        let mut valid_count = 0;
        let mut parse_error_count = 0;

        for file_info in &file_infos {
            let file_name = file_info
                .path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");

            match &file_info.status {
                testcase_manager::FileValidationStatus::Valid => {
                    log::info!("✓ Valid: {}", file_name);
                    valid_count += 1;
                }
                testcase_manager::FileValidationStatus::ParseError { message } => {
                    log::error!("✗ Parse Error: {}", file_name);
                    log::error!("  {}", message);
                    parse_error_count += 1;
                }
                testcase_manager::FileValidationStatus::ValidationError { errors } => {
                    log::error!("✗ Invalid: {}", file_name);
                    log::error!("  Validation Errors ({}):", errors.len());
                    for (idx, error) in errors.iter().enumerate() {
                        log::error!("\n    Error #{}: Path '{}'", idx + 1, error.path);
                        log::error!("      Constraint: {}", error.constraint);
                        log::error!("      Expected: {}", error.expected_constraint);
                        log::error!("      Found: {}", error.found_value);
                    }
                    log::error!("");
                    error_count += 1;
                }
            }
        }

        print_title("Validation Summary", TitleStyle::TripleEquals);
        log::info!("Total files: {}", file_infos.len());
        log::info!("✓ Valid: {}", valid_count);
        log::warn!("✗ Schema violations: {}", error_count);
        log::warn!("✗ Parse errors: {}", parse_error_count);

        if error_count > 0 || parse_error_count > 0 {
            anyhow::bail!(
                "{} validation error(s) found",
                error_count + parse_error_count
            );
        }
    } else {
        anyhow::bail!("Specify --file or --all");
    }

    Ok(())
}

fn handle_search(base_path: &str, _query: Option<String>) -> Result<()> {
    let storage = TestCaseStorage::new(base_path)?;
    let test_cases = storage.load_all_test_cases()?;

    if let Some(selected) = TestCaseFuzzyFinder::search(&test_cases)? {
        let yaml = serde_yaml::to_string(&selected)?;
        log::info!("{}", yaml);
    }

    Ok(())
}

fn handle_export(base_path: &str, output: &str, _tags: Option<String>) -> Result<()> {
    let storage = TestCaseStorage::new(base_path)?;
    let test_cases = storage.load_all_test_cases()?;

    let test_suite = TestSuite {
        name: "Exported Test Suite".to_string(),
        description: Some("Exported from test case repository".to_string()),
        version: Some("1.0".to_string()),
        test_cases,
    };

    let file_path = storage.save_test_suite(&test_suite, output)?;
    log::info!("Test suite exported: {}", file_path.display());

    Ok(())
}

fn handle_import(base_path: &str, file: &str, skip_validation: bool) -> Result<()> {
    let storage = TestCaseStorage::new(base_path)?;
    let test_suite = storage.load_test_suite(file)?;

    let validator = testcase_manager::SchemaValidator::new()?;

    for test_case in test_suite.test_cases {
        if !skip_validation {
            let yaml = serde_yaml::to_string(&test_case)?;
            validator.validate_chunk(&yaml)?;
        }

        storage.save_test_case(&test_case)?;
        log::info!("Imported: {}", test_case.id);
    }

    Ok(())
}

fn handle_git(base_path: &str, command: GitCommands) -> Result<()> {
    let git = GitManager::open(base_path).or_else(|_| GitManager::init(base_path))?;

    match command {
        GitCommands::Add { ids, all } => {
            if all {
                git.add_all()?;
                log::info!("All files added to staging");
            } else {
                let paths: Vec<_> = ids
                    .iter()
                    .map(|id| format!("{}.yaml", id))
                    .map(std::path::PathBuf::from)
                    .collect();
                git.add(&paths)?;
                log::info!("Added {} file(s) to staging", paths.len());
            }
        }

        GitCommands::Commit { message } => {
            let author_name = std::env::var("GIT_AUTHOR_NAME")
                .unwrap_or_else(|_| "Test Case Manager".to_string());
            let author_email = std::env::var("GIT_AUTHOR_EMAIL")
                .unwrap_or_else(|_| "testcase@example.com".to_string());

            let oid = git.commit(&message, &author_name, &author_email)?;
            log::info!("Committed: {}", oid);
        }

        GitCommands::Status => {
            let statuses = git.status()?;
            if statuses.is_empty() {
                log::info!("No changes");
            } else {
                for (path, status) in statuses {
                    log::info!("{:?} {}", status, path);
                }
            }
        }

        GitCommands::Log { limit } => {
            let commits = git.log(limit)?;
            for commit in commits {
                log::info!(
                    "{} - {} ({})",
                    &commit.id[..7],
                    commit.message.lines().next().unwrap_or(""),
                    commit.author
                );
            }
        }
    }

    Ok(())
}

fn handle_init(path: &str, init_git: bool) -> Result<()> {
    let storage = TestCaseStorage::new(path)?;
    log::info!(
        "Initialized test case repository: {}",
        storage.base_path().display()
    );

    if init_git {
        GitManager::init(path)?;
        log::info!("Initialized git repository");

        let gitignore_path = std::path::Path::new(path).join(".gitignore");
        if !gitignore_path.exists() {
            std::fs::write(&gitignore_path, "*.bak\n*.tmp\n.DS_Store\n")?;
            log::info!("Created .gitignore");
        }
    }

    Ok(())
}

fn handle_create_interactive(path: &str) -> Result<()> {
    let oracle: Arc<dyn Oracle> = Arc::new(TtyCliOracle::new());
    let mut builder = TestCaseBuilder::new_with_recovery(path, oracle)
        .context("Failed to create test case builder")?;

    print_title("Interactive Test Case Creation Workflow", TitleStyle::Box);

    builder.add_metadata().context("Failed to add metadata")?;

    log::info!("✓ Metadata added to structure\n");

    builder
        .commit("Add test case metadata")
        .context("Failed to commit metadata")?;

    if Prompts::confirm_with_default("\nAdd general initial conditions?", true)? {
        builder
            .add_general_initial_conditions(None)
            .context("Failed to add general initial conditions")?;

        log::info!("✓ General initial conditions added\n");

        builder
            .commit("Add general initial conditions")
            .context("Failed to commit general initial conditions")?;
    }

    if Prompts::confirm_with_default("\nAdd initial conditions?", true)? {
        builder
            .add_initial_conditions(None)
            .context("Failed to add initial conditions")?;

        log::info!("✓ Initial conditions added\n");

        builder
            .commit("Add initial conditions")
            .context("Failed to commit initial conditions")?;
    }

    let file_path = builder.save().context("Failed to save test case")?;

    builder.delete_recovery_file()?;
    log::info!("✓ Recovery file deleted");

    print_title("Test Case Created Successfully", TitleStyle::Box);
    log::info!("Saved to: {}", file_path.display());

    Ok(())
}

fn handle_build_sequences(path: &str) -> Result<()> {
    let oracle: Arc<dyn Oracle> = Arc::new(TtyCliOracle::new());
    let mut builder = TestCaseBuilder::new_with_recovery(path, oracle)
        .context("Failed to create test case builder")?;

    print_title("Test Sequence Builder with Git Commits", TitleStyle::Box);

    builder.add_metadata().context("Failed to add metadata")?;

    log::info!("✓ Metadata added to structure\n");

    builder
        .commit("Add test case metadata")
        .context("Failed to commit metadata")?;

    if Prompts::confirm("\nAdd general initial conditions?")? {
        builder
            .add_general_initial_conditions(None)
            .context("Failed to add general initial conditions")?;

        log::info!("✓ General initial conditions added\n");

        builder
            .commit("Add general initial conditions")
            .context("Failed to commit general initial conditions")?;
    }

    prompt_then_do_add_initial_conditions(&mut builder)?;

    builder
        .build_test_sequences_with_commits()
        .context("Failed to build test sequences")?;

    let file_path = builder.save().context("Failed to save test case")?;

    print_title("Test Sequences Built Successfully", TitleStyle::Box);
    log::info!("Saved to: {}", file_path.display());

    builder
        .commit("Complete test case with all sequences")
        .context("Failed to commit final file")?;

    builder.delete_recovery_file()?;
    log::info!("✓ Recovery file deleted");

    Ok(())
}

fn handle_add_steps(path: &str, sequence_id: Option<i64>) -> Result<()> {
    let oracle: Arc<dyn Oracle> = Arc::new(TtyCliOracle::new());
    let mut builder = TestCaseBuilder::new_with_recovery(path, oracle)
        .context("Failed to create test case builder")?;

    print_title("Add Steps to Sequence with Commits", TitleStyle::Box);

    builder.add_metadata().context("Failed to add metadata")?;

    log::info!("✓ Metadata added to structure\n");

    builder
        .commit("Add test case metadata")
        .context("Failed to commit metadata")?;

    if Prompts::confirm("\nAdd general initial conditions?")? {
        builder
            .add_general_initial_conditions(None)
            .context("Failed to add general initial conditions")?;

        log::info!("✓ General initial conditions added\n");

        builder
            .commit("Add general initial conditions")
            .context("Failed to commit general initial conditions")?;
    }

    prompt_then_do_add_initial_conditions(&mut builder)?;

    builder
        .add_test_sequence_interactive()
        .context("Failed to add test sequence")?;

    let sequence_id_val = builder.get_next_sequence_id() - 1;
    let commit_msg = format!("Add test sequence #{}", sequence_id_val);
    builder
        .commit(&commit_msg)
        .context("Failed to commit test sequence")?;

    let seq_id = if let Some(id) = sequence_id {
        id
    } else {
        builder.get_next_sequence_id() - 1
    };

    builder
        .add_steps_to_sequence_by_id_with_commits(seq_id)
        .context("Failed to add steps to sequence")?;

    let file_path = builder.save().context("Failed to save test case")?;

    print_title("Steps Added Successfully", TitleStyle::Box);
    log::info!("Saved to: {}", file_path.display());

    builder
        .commit("Complete test sequence with all steps")
        .context("Failed to commit final file")?;

    builder.delete_recovery_file()?;
    log::info!("✓ Recovery file deleted");

    Ok(())
}

fn prompt_then_do_add_initial_conditions(builder: &mut TestCaseBuilder) -> Result<(), Error> {
    if Prompts::confirm("\nAdd initial conditions?")? {
        builder
            .add_initial_conditions(None)
            .context("Failed to add initial conditions")?;

        log::info!("✓ Initial conditions added\n");

        builder
            .commit("Add initial conditions")
            .context("Failed to commit initial conditions")?;
    }
    Ok(())
}

fn handle_build_sequences_with_steps(path: &str) -> Result<()> {
    let oracle: Arc<dyn Oracle> = Arc::new(TtyCliOracle::new());
    let mut builder = TestCaseBuilder::new_with_recovery(path, oracle)
        .context("Failed to create test case builder")?;

    print_title("Build Test Sequences & Steps with Commits", TitleStyle::Box);

    builder.add_metadata().context("Failed to add metadata")?;

    log::info!("✓ Metadata added to structure\n");

    builder
        .commit("Add test case metadata")
        .context("Failed to commit metadata")?;

    if Prompts::confirm("\nAdd general initial conditions?")? {
        builder
            .add_general_initial_conditions(None)
            .context("Failed to add general initial conditions")?;

        log::info!("✓ General initial conditions added\n");

        builder
            .commit("Add general initial conditions")
            .context("Failed to commit general initial conditions")?;
    }

    prompt_then_do_add_initial_conditions(&mut builder)?;

    builder
        .build_test_sequences_with_step_commits()
        .context("Failed to build test sequences with steps")?;

    let file_path = builder.save().context("Failed to save test case")?;

    print_title("Test Sequences & Steps Built Successfully", TitleStyle::Box);
    log::info!("Saved to: {}", file_path.display());

    builder
        .commit("Complete test case with all sequences and steps")
        .context("Failed to commit final file")?;

    builder.delete_recovery_file()?;
    log::info!("✓ Recovery file deleted");

    Ok(())
}

fn handle_complete(output_path: &str, commit_prefix: Option<&str>, use_sample: bool) -> Result<()> {
    let prefix = commit_prefix.unwrap_or("");
    let commit_msg = |msg: &str| {
        if prefix.is_empty() {
            msg.to_string()
        } else {
            format!("{}: {}", prefix, msg)
        }
    };

    let output_file = std::path::Path::new(output_path);
    let base_dir = output_file
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid output path"))?;

    std::fs::create_dir_all(base_dir).context(format!(
        "Failed to create directory: {}",
        base_dir.display()
    ))?;

    let sample_data = if use_sample {
        Some(SampleData::new())
    } else {
        None
    };

    let oracle: Arc<dyn Oracle> = if let Some(ref sample) = sample_data {
        Arc::new(sample.create_oracle_for_complete())
    } else {
        Arc::new(TtyCliOracle::new())
    };

    let mut builder = TestCaseBuilder::new_with_recovery(base_dir, oracle)
        .context("Failed to create test case builder")?;

    print_title("Complete Interactive Test Case Workflow", TitleStyle::Box);

    if use_sample {
        log::info!("📝 Sample mode enabled: Default values will be pre-populated\n");
    }

    let recovered_metadata = TestCaseMetadata::from_structure(builder.structure());

    loop {
        let metadata = if use_sample && recovered_metadata.is_none() {
            if let Some(sample) = &sample_data {
                let prompts = Prompts::new_with_sample(sample);
                prompts
                    .prompt_metadata_with_sample()
                    .context("Failed to prompt for metadata")?
            } else {
                Prompts::prompt_metadata_with_recovery(recovered_metadata.as_ref())
                    .context("Failed to prompt for metadata")?
            }
        } else {
            Prompts::prompt_metadata_with_recovery(recovered_metadata.as_ref())
                .context("Failed to prompt for metadata")?
        };

        print_title("Validating Metadata", TitleStyle::TripleEquals);
        match metadata.validate(builder.validator()) {
            Ok(_) => {
                log::info!("✓ Metadata is valid\n");

                let yaml_map = metadata.to_yaml();
                for (key, value) in yaml_map {
                    builder.structure_mut().insert(key, value);
                }

                builder.save_recovery_state("metadata")?;
                break;
            }
            Err(e) => {
                log::warn!("✗ Metadata validation failed: {}\n", e);

                builder.save_recovery_state_with_errors("metadata", &e)?;

                if !Prompts::confirm("Retry metadata entry?")? {
                    anyhow::bail!("User cancelled metadata entry");
                }
            }
        }
    }

    builder
        .commit(&commit_msg("Add test case metadata"))
        .context("Failed to commit metadata")?;

    let add_general = if let Some(sample) = &sample_data {
        let prompts = Prompts::new_with_sample(sample);
        prompts.confirm_with_sample(
            "\nAdd general initial conditions?",
            sample.confirm_add_general_conditions(),
        )?
    } else {
        Prompts::confirm("\nAdd general initial conditions?")?
    };

    if add_general {
        loop {
            match builder.add_general_initial_conditions(None) {
                Ok(_) => {
                    log::info!("✓ General initial conditions added\n");
                    builder.save_recovery_state("general_initial_conditions")?;
                    break;
                }
                Err(e) => {
                    log::warn!("✗ General initial conditions validation failed: {}\n", e);
                    builder.save_recovery_state_with_errors("general_initial_conditions", &e)?;

                    let should_retry = if let Some(sample) = &sample_data {
                        let prompts = Prompts::new_with_sample(sample);
                        prompts.confirm_with_sample(
                            "Retry general initial conditions entry?",
                            sample.confirm_retry(),
                        )?
                    } else {
                        Prompts::confirm("Retry general initial conditions entry?")?
                    };

                    if !should_retry {
                        log::warn!("⚠ Skipping general initial conditions\n");
                        break;
                    }
                }
            }
        }

        builder
            .commit(&commit_msg("Add general initial conditions"))
            .context("Failed to commit general initial conditions")?;
    }

    let add_initial = if let Some(sample) = &sample_data {
        let prompts = Prompts::new_with_sample(sample);
        prompts.confirm_with_sample(
            "\nAdd initial conditions?",
            sample.confirm_add_initial_conditions(),
        )?
    } else {
        Prompts::confirm("\nAdd initial conditions?")?
    };

    if add_initial {
        loop {
            match builder.add_initial_conditions(None) {
                Ok(_) => {
                    log::info!("✓ Initial conditions added\n");
                    builder.save_recovery_state("initial_conditions")?;
                    break;
                }
                Err(e) => {
                    log::warn!("✗ Initial conditions validation failed: {}\n", e);
                    builder.save_recovery_state_with_errors("initial_conditions", &e)?;

                    let should_retry = if let Some(sample) = &sample_data {
                        let prompts = Prompts::new_with_sample(sample);
                        prompts.confirm_with_sample(
                            "Retry initial conditions entry?",
                            sample.confirm_retry(),
                        )?
                    } else {
                        Prompts::confirm("Retry initial conditions entry?")?
                    };

                    if !should_retry {
                        log::warn!("⚠ Skipping initial conditions\n");
                        break;
                    }
                }
            }
        }

        builder
            .commit(&commit_msg("Add initial conditions"))
            .context("Failed to commit initial conditions")?;
    }

    print_title("Build Test Sequences with Validation", TitleStyle::Box);

    loop {
        let sequence_added = loop {
            match builder.add_test_sequence_interactive() {
                Ok(_) => {
                    builder.save_recovery_state("test_sequences")?;
                    break true;
                }
                Err(e) => {
                    log::warn!("✗ Test sequence validation failed: {}\n", e);
                    builder.save_recovery_state_with_errors("test_sequences", &e)?;

                    let should_retry = if let Some(sample) = &sample_data {
                        let prompts = Prompts::new_with_sample(sample);
                        prompts.confirm_with_sample(
                            "Retry test sequence entry?",
                            sample.confirm_retry(),
                        )?
                    } else {
                        Prompts::confirm("Retry test sequence entry?")?
                    };

                    if !should_retry {
                        log::warn!("⚠ Skipping this test sequence\n");
                        break false;
                    }
                }
            }
        };

        if !sequence_added {
            let add_another_seq = if let Some(sample) = &sample_data {
                let prompts = Prompts::new_with_sample(sample);
                prompts.confirm_with_sample(
                    "Add another test sequence?",
                    sample.confirm_add_another_sequence(),
                )?
            } else {
                Prompts::confirm("Add another test sequence?")?
            };

            if add_another_seq {
                continue;
            } else {
                break;
            }
        }

        let sequence_index = builder.get_sequence_count() - 1;

        let sequence_id = builder.get_next_sequence_id() - 1;
        builder
            .commit(&commit_msg(&format!("Add test sequence #{}", sequence_id)))
            .context("Failed to commit test sequence")?;

        let add_steps = if let Some(sample) = &sample_data {
            let prompts = Prompts::new_with_sample(sample);
            prompts.confirm_with_sample(
                "\nAdd steps to this sequence now?",
                sample.confirm_add_steps_to_sequence(),
            )?
        } else {
            Prompts::confirm("\nAdd steps to this sequence now?")?
        };

        if add_steps {
            let sequence_id = match builder.get_sequence_id_by_index(sequence_index) {
                Ok(id) => id,
                Err(e) => {
                    log::error!("✗ Failed to get sequence ID: {}", e);

                    if !add_another_test_sequence(&sample_data)? {
                        break;
                    }
                    continue;
                }
            };

            let sequence_name = builder
                .get_sequence_name_by_index(sequence_index)
                .unwrap_or_else(|_| "Unknown".to_string());

            print_title(
                &format!("Add Steps to Sequence #{}: {}", sequence_id, sequence_name),
                TitleStyle::Box,
            );

            let existing_steps = builder.get_all_existing_steps();

            'add_steps: loop {
                let step_number = builder.get_next_step_number(sequence_index)?;

                'step_retry: loop {
                    print_title(
                        &format!("Add Step #{}", step_number),
                        TitleStyle::TripleEquals,
                    );

                    let step_description = if let Some(sample) = &sample_data {
                        let prompts = Prompts::new_with_sample(sample);
                        prompts.input_with_sample("Step description", &sample.step_description())?
                    } else if !existing_steps.is_empty() {
                        log::info!(
                            "\nYou can select from existing step descriptions or enter a new one."
                        );

                        if Prompts::confirm(
                            "Use fuzzy search to select from existing descriptions?",
                        )? {
                            match TestCaseFuzzyFinder::search_strings(
                                &existing_steps,
                                "Select step description: ",
                            )? {
                                Some(desc) => desc,
                                None => {
                                    log::info!("No selection made, entering new description.");
                                    Prompts::input("Step description")?
                                }
                            }
                        } else {
                            Prompts::input("Step description")?
                        }
                    } else {
                        Prompts::input("Step description")?
                    };

                    let manual = if let Some(sample) = &sample_data {
                        let prompts = Prompts::new_with_sample(sample);
                        if prompts.confirm_with_sample(
                            "Is this a manual step?",
                            sample.confirm_is_manual_step(),
                        )? {
                            Some(true)
                        } else {
                            None
                        }
                    } else if Prompts::confirm("Is this a manual step?")? {
                        Some(true)
                    } else {
                        None
                    };

                    let command = if let Some(sample) = &sample_data {
                        let prompts = Prompts::new_with_sample(sample);
                        prompts.input_with_sample("Command", &sample.step_command())?
                    } else {
                        Prompts::input("Command")?
                    };

                    let expected = builder.prompt_for_expected()?;

                    let step = builder.create_step_value(
                        step_number,
                        manual,
                        step_description.clone(),
                        command,
                        expected,
                    )?;

                    print_title("Validating Step", TitleStyle::TripleEquals);
                    match builder.validate_and_append_step(sequence_index, step) {
                        Ok(_) => {
                            log::info!("✓ Step validated and added\n");
                            builder.save().context("Failed to save file")?;
                            builder.save_recovery_state("steps")?;

                            builder
                                .commit(&commit_msg(&format!(
                                    "Add step #{} to sequence #{}: {}",
                                    step_number, sequence_id, step_description
                                )))
                                .context("Failed to commit step")?;

                            if !add_another_step(&sample_data)? {
                                break 'add_steps;
                            }
                            break 'step_retry;
                        }
                        Err(e) => {
                            log::warn!("✗ Step validation failed: {}\n", e);
                            builder.save_recovery_state_with_errors("steps", &e)?;

                            let should_retry_step = if let Some(sample) = &sample_data {
                                let prompts = Prompts::new_with_sample(sample);
                                prompts.confirm_with_sample(
                                    "Retry this step entry?",
                                    sample.confirm_retry(),
                                )?
                            } else {
                                Prompts::confirm("Retry this step entry?")?
                            };

                            if !should_retry_step {
                                log::warn!("⚠ Skipping this step\n");

                                if !add_another_step(&sample_data)? {
                                    break 'add_steps;
                                }
                                break 'step_retry;
                            }
                        }
                    }
                }
            }

            log::info!("\n✓ All steps added to sequence");
        }

        if !add_another_test_sequence(&sample_data)? {
            break;
        }
    }

    print_title("Saving Complete Test Case", TitleStyle::Box);

    let final_yaml_content = builder.to_yaml_string()?;
    std::fs::write(output_path, &final_yaml_content)
        .context(format!("Failed to write output file: {}", output_path))?;

    log::info!("✓ Complete test case saved to: {}\n", output_path);

    builder.commit("Complete test case with all sequences and steps")?;

    builder.delete_recovery_file()?;
    log::info!("✓ Recovery file deleted\n");

    print_title("Test Case Workflow Completed!", TitleStyle::Box);

    Ok(())
}

fn add_another_step(sample_data: &Option<SampleData>) -> Result<bool, Error> {
    let add_another_step = if let Some(sample) = &sample_data {
        let prompts = Prompts::new_with_sample(sample);
        prompts.confirm_with_sample(
            "\nAdd another step to this sequence?",
            sample.confirm_add_another_step(),
        )?
    } else {
        Prompts::confirm("\nAdd another step to this sequence?")?
    };
    Ok(add_another_step)
}

fn add_another_test_sequence(sample_data: &Option<SampleData>) -> Result<bool, Error> {
    let add_another_seq = if let Some(sample) = &sample_data {
        let prompts = Prompts::new_with_sample(sample);
        prompts.confirm_with_sample(
            "\nAdd another test sequence?",
            sample.confirm_add_another_sequence(),
        )?
    } else {
        Prompts::confirm("\nAdd another test sequence?")?
    };
    Ok(add_another_seq)
}

fn handle_parse_general_conditions(database_path: &str, work_path: &str) -> Result<()> {
    print_title("Parse General Initial Conditions", TitleStyle::Box);

    let db = ConditionDatabase::load_from_directory(database_path)
        .context("Failed to load condition database")?;

    let conditions = db.get_general_conditions();

    if conditions.is_empty() {
        log::info!("No general initial conditions found in database.");
        return Ok(());
    }

    log::info!(
        "Loaded {} unique general initial conditions from database\n",
        conditions.len()
    );

    let oracle: Arc<dyn Oracle> = Arc::new(TtyCliOracle::new());
    let mut builder = TestCaseBuilder::new_with_recovery(work_path, oracle)
        .context("Failed to create test case builder")?;

    builder.add_metadata().context("Failed to add metadata")?;

    log::info!("✓ Metadata added to structure\n");

    let mut selected_conditions = Vec::new();

    loop {
        print_title(
            &format!(
                "Current Selection: {} condition(s)",
                selected_conditions.len()
            ),
            TitleStyle::TripleEquals,
        );
        if !selected_conditions.is_empty() {
            for (idx, cond) in selected_conditions.iter().enumerate() {
                log::info!("  {}. {}", idx + 1, cond);
            }
        } else {
            log::info!("  (none)");
        }

        print_title("Add General Initial Condition", TitleStyle::TripleEquals);
        log::info!("Options:");
        log::info!("  1. Search from database (fuzzy search)");
        log::info!("  2. Create new condition (manual entry)");
        log::info!("  3. Finish selection");

        let choice = Prompts::input("\nChoice (1/2/3)")?;

        match choice.trim() {
            "1" => {
                let selected = TestCaseFuzzyFinder::search_strings(
                    conditions,
                    "Select condition (ESC to cancel): ",
                )?;

                if let Some(condition) = selected {
                    selected_conditions.push(condition.clone());
                    log::info!("✓ Added from database: {}\n", condition);
                }
            }
            "2" => {
                let new_condition = Prompts::input("Enter new condition")?;
                if !new_condition.trim().is_empty() {
                    selected_conditions.push(new_condition.clone());
                    log::info!("✓ Added new condition: {}\n", new_condition);
                }
            }
            "3" => {
                if selected_conditions.is_empty() {
                    log::info!("No conditions selected.");
                    if !Prompts::confirm("Continue without general initial conditions?")? {
                        continue;
                    }
                }
                break;
            }
            _ => {
                log::warn!("Invalid choice. Please enter 1, 2, or 3.");
            }
        }
    }

    if !selected_conditions.is_empty() {
        use serde_yaml::Value;

        let euicc_conditions: Vec<Value> =
            selected_conditions.into_iter().map(Value::String).collect();

        let mut general_cond_map = serde_yaml::Mapping::new();
        general_cond_map.insert(
            Value::String("eUICC".to_string()),
            Value::Sequence(euicc_conditions),
        );

        let general_conditions_array = vec![Value::Mapping(general_cond_map)];

        builder.structure_mut().insert(
            "general_initial_conditions".to_string(),
            Value::Sequence(general_conditions_array),
        );

        log::info!("\n✓ General initial conditions added to test case");
    }

    let file_path = builder.save().context("Failed to save test case")?;

    print_title("Test Case Saved Successfully", TitleStyle::Box);
    log::info!("Saved to: {}", file_path.display());

    builder
        .commit("Add general initial conditions")
        .context("Failed to commit")?;

    builder.delete_recovery_file()?;

    Ok(())
}

fn handle_parse_initial_conditions2(database_path: &str, work_path: &str) -> Result<()> {
    print_title("Parse Initial Conditions 2", TitleStyle::Box);

    let db = ConditionDatabase::load_from_directory(database_path)
        .context("Failed to load condition database")?;

    let conditions = db.get_initial_conditions();
    let devices = db.get_device_names();

    if conditions.is_empty() {
        log::info!("No initial conditions found in database.");
        return Ok(());
    }

    log::info!(
        "Loaded {} unique initial conditions from database\n",
        conditions.len()
    );

    let oracle: Arc<dyn Oracle> = Arc::new(TtyCliOracle::new());
    let mut builder = TestCaseBuilder::new_with_recovery(work_path, oracle)
        .context("Failed to create test case builder")?;

    builder.add_metadata().context("Failed to add metadata")?;

    log::info!("✓ Metadata added to structure\n");

    let mut selected_conditions: Vec<String> = Vec::new();

    let mut action_choice;
    loop {
        log::info!(
            "\n=== Current Selection: {} condition(s) ===",
            selected_conditions.len()
        );
        if !selected_conditions.is_empty() {
            for (idx, cond) in selected_conditions.iter().enumerate() {
                log::info!("  {}. {}", idx + 1, cond);
            }
        } else {
            log::info!("  (none)");
        }

        let selected =
            TestCaseFuzzyFinder::search_strings_multi(devices, "Select device (ESC to accept None, Ctrl-C to quit this step, Ctrl-D to finish input): ")?;

        match selected {
            Input(selected) => {
                log::info!("Selected: {}", selected);
                selected_conditions.push(selected.clone());
                log::info!("✓ Added from database: {}\n", selected);
            }
            MultiInput::Aborted => {
                log::info!("(none): Aborted");
                break;
            }
            MultiInput::Finished => {
                log::info!("(none): Finished")
            }
            MultiInput::Error => {
                log::warn!("(none): Error")
            }
        }

        action_choice = Prompts::input_with_escape("Start typing for search (ESC to cancel)")?;

        match action_choice {
            None => {
                log::info!("ESC received");
                break;
            }
            Some(_) => {
                log::info!("Device: {}", action_choice.unwrap());

                let selected = TestCaseFuzzyFinder::search_strings(
                    conditions,
                    "Select condition (ESC to cancel): ",
                )?;

                if let Some(condition) = selected {
                    selected_conditions.push(condition.clone());
                    log::info!("✓ Added from database: {}\n", condition);
                }
            }
        }
    }

    Ok(())
}

fn handle_parse_initial_conditions(database_path: &str, work_path: &str) -> Result<()> {
    print_title("Parse Initial Conditions", TitleStyle::Box);

    let db = ConditionDatabase::load_from_directory(database_path)
        .context("Failed to load condition database")?;

    let conditions = db.get_initial_conditions();

    if conditions.is_empty() {
        log::info!("No initial conditions found in database.");
        return Ok(());
    }

    log::info!(
        "Loaded {} unique initial conditions from database\n",
        conditions.len()
    );

    let oracle: Arc<dyn Oracle> = Arc::new(TtyCliOracle::new());
    let mut builder = TestCaseBuilder::new_with_recovery(work_path, oracle)
        .context("Failed to create test case builder")?;

    log::info!("✓ Metadata added to structure\n");

    let mut selected_conditions = Vec::new();

    loop {
        print_title(
            &format!(
                "Current Selection: {} condition(s)",
                selected_conditions.len()
            ),
            TitleStyle::TripleEquals,
        );
        if !selected_conditions.is_empty() {
            for (idx, cond) in selected_conditions.iter().enumerate() {
                log::info!("  {}. {}", idx + 1, cond);
            }
        } else {
            log::info!("  (none)");
        }

        print_title("Add Initial Condition", TitleStyle::TripleEquals);
        log::info!("Options:");
        log::info!("  1. Search from database (fuzzy search)");
        log::info!("  2. Create new condition (manual entry)");
        log::info!("  3. Finish selection");

        let choice = Prompts::input("\nChoice (1/2/3)")?;

        match choice.trim() {
            "1" => {
                let selected = TestCaseFuzzyFinder::search_strings(
                    conditions,
                    "Select condition (ESC to cancel): ",
                )?;

                if let Some(condition) = selected {
                    selected_conditions.push(condition.clone());
                    log::info!("✓ Added from database: {}\n", condition);
                }
            }
            "2" => {
                let new_condition = Prompts::input("Enter new condition")?;
                if !new_condition.trim().is_empty() {
                    selected_conditions.push(new_condition.clone());
                    log::info!("✓ Added new condition: {}\n", new_condition);
                }
            }
            "3" => {
                if selected_conditions.is_empty() {
                    log::info!("No conditions selected.");
                    if !Prompts::confirm("Continue without initial conditions?")? {
                        continue;
                    }
                }
                break;
            }
            _ => {
                log::warn!("Invalid choice. Please enter 1, 2, or 3.");
            }
        }
    }

    if !selected_conditions.is_empty() {
        use serde_yaml::Value;

        let euicc_conditions: Vec<Value> =
            selected_conditions.into_iter().map(Value::String).collect();

        let mut initial_cond_map = serde_yaml::Mapping::new();
        initial_cond_map.insert(
            Value::String("eUICC".to_string()),
            Value::Sequence(euicc_conditions),
        );

        builder.structure_mut().insert(
            "initial_conditions".to_string(),
            Value::Mapping(initial_cond_map),
        );

        log::info!("\n✓ Initial conditions added to test case");
    }

    let file_path = builder.save().context("Failed to save test case")?;

    print_title("Test Case Saved Successfully", TitleStyle::Box);
    log::info!("Saved to: {}", file_path.display());

    builder
        .commit("Add initial conditions")
        .context("Failed to commit")?;

    builder.delete_recovery_file()?;

    Ok(())
}

fn handle_validate_yaml(yaml_file: &str, schema_file: &str) -> Result<()> {
    use std::fs;

    // Read the YAML file
    let yaml_content = fs::read_to_string(yaml_file)
        .context(format!("Failed to read YAML file: {}", yaml_file))?;

    // Read the JSON schema file
    let schema_content = fs::read_to_string(schema_file)
        .context(format!("Failed to read schema file: {}", schema_file))?;

    // Parse the schema
    let schema_value: serde_json::Value =
        serde_json::from_str(&schema_content).context("Failed to parse JSON schema")?;

    // Parse the YAML content
    let yaml_value: serde_yaml::Value = match serde_yaml::from_str(&yaml_content) {
        Ok(value) => value,
        Err(e) => {
            log_yaml_parse_error(&e, &yaml_content, yaml_file);
            return Err(anyhow::anyhow!("Failed to parse YAML content: {}", e));
        }
    };

    // Convert YAML to JSON Value for validation
    let json_value: serde_json::Value =
        serde_json::to_value(&yaml_value).context("Failed to convert YAML to JSON")?;

    // Compile the schema
    let compiled_schema = jsonschema::JSONSchema::compile(&schema_value)
        .map_err(|e| anyhow::anyhow!("Failed to compile JSON schema: {}", e))?;

    // Validate
    if let Err(errors) = compiled_schema.validate(&json_value) {
        log::error!("✗ Validation failed!\n");
        log::error!("The following schema constraint violations were found:\n");

        for (idx, error) in errors.enumerate() {
            let path = if error.instance_path.to_string().is_empty() {
                "root".to_string()
            } else {
                error.instance_path.to_string()
            };

            log::error!("Error #{}: Path '{}'", idx + 1, path);
            log::error!("  Constraint: {}", error);

            // Extract the actual value at the error path if possible
            let instance = error.instance.as_ref();
            log::error!("  Found value: {}", instance);
            log::error!("");
        }

        anyhow::bail!("Validation failed with schema constraint violations");
    }

    log::info!("✓ Validation successful!");
    log::info!("\nThe YAML payload is valid according to the provided schema.");
    Ok(())
}

fn handle_export_junit_xml(input_path: &str, output_path: &str) -> Result<()> {
    use std::fs;
    use std::io::Write;
    use testcase_manager::{TestRun, TestRunStatus};

    let content = fs::read_to_string(input_path)
        .context(format!("Failed to read input file: {}", input_path))?;

    let test_runs: Vec<TestRun> = if input_path.ends_with(".json") {
        serde_json::from_str(&content)
            .context("Failed to parse JSON input. Expected an array of TestRun objects")?
    } else if input_path.ends_with(".yaml") || input_path.ends_with(".yml") {
        serde_yaml::from_str(&content)
            .context("Failed to parse YAML input. Expected an array of TestRun objects")?
    } else {
        serde_json::from_str(&content).or_else(|_| {
            serde_yaml::from_str(&content).context("Failed to parse input as JSON or YAML")
        })?
    };

    if test_runs.is_empty() {
        log::warn!("No test runs found in input file");
        return Ok(());
    }

    let mut combined_xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");

    let total_tests = test_runs.len();
    let total_failures = test_runs
        .iter()
        .filter(|tr| tr.status == TestRunStatus::Fail)
        .count();
    let total_skipped = test_runs
        .iter()
        .filter(|tr| tr.status == TestRunStatus::Skip)
        .count();
    let total_time: f64 = test_runs.iter().map(|tr| tr.duration).sum();

    combined_xml.push_str(&format!(
        "<testsuite name=\"TestRuns\" tests=\"{}\" failures=\"{}\" skipped=\"{}\" time=\"{:0.3}\">\n",
        total_tests, total_failures, total_skipped, total_time
    ));

    for test_run in &test_runs {
        let single_xml = test_run.to_junit_xml();

        for line in single_xml.lines() {
            if line.contains("<?xml") {
                continue;
            }
            if line.contains("<testsuite") || line.contains("</testsuite>") {
                continue;
            }
            combined_xml.push_str("  ");
            combined_xml.push_str(line);
            combined_xml.push('\n');
        }
    }

    combined_xml.push_str("</testsuite>\n");

    if output_path == "-" {
        print!("{}", combined_xml);
        std::io::stdout().flush()?;
    } else {
        fs::write(output_path, &combined_xml)
            .context(format!("Failed to write output file: {}", output_path))?;

        // Validate the written XML file
        match testcase_manager::validate_junit_xml(&combined_xml) {
            Ok(_) => {
                log::info!("✓ JUnit XML exported to: {}", output_path);
                log::info!("✓ XML validated successfully against XSD schema");
                log::info!(
                    "  Total: {} tests, {} failures, {} skipped",
                    total_tests,
                    total_failures,
                    total_skipped
                );
            }
            Err(e) => {
                log::warn!("✓ JUnit XML exported to: {}", output_path);
                log::warn!("⚠ XML validation warning: {}", e);
                log::info!(
                    "  Total: {} tests, {} failures, {} skipped",
                    total_tests,
                    total_failures,
                    total_skipped
                );
            }
        }
    }

    Ok(())
}

fn handle_validate_junit_xml(xml_file: &str) -> Result<()> {
    use std::fs;

    let xml_content =
        fs::read_to_string(xml_file).context(format!("Failed to read XML file: {}", xml_file))?;

    log::info!("Validating JUnit XML file: {}", xml_file);

    match testcase_manager::validate_junit_xml(&xml_content) {
        Ok(_) => {
            log::info!("✓ XML validation successful!");
            log::info!("  File conforms to Maven Surefire XSD schema");
            log::info!("  Schema: https://maven.apache.org/surefire/maven-surefire-plugin/xsd/surefire-test-report.xsd");
            Ok(())
        }
        Err(e) => {
            log::error!("✗ XML validation failed!");
            log::error!("  {}", e);
            anyhow::bail!("XML validation failed: {}", e)
        }
    }
}
