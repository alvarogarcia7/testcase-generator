use anyhow::{Context, Result};
use clap::Parser;
use testcase_manager::{
    cli::{Cli, Commands, GitCommands},
    GitManager, Prompts, TestCase, TestCaseBuilder, TestCaseEditor, TestCaseFuzzyFinder,
    TestCaseStorage, TestCaseValidator, TestSuite,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { id, title, edit } => {
            handle_create(&cli.path, id, title, edit)?;
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
            verbose,
        } => {
            handle_list(&cli.path, tag, status, priority, verbose)?;
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
    }

    Ok(())
}

fn handle_create(
    base_path: &str,
    id: Option<String>,
    title: Option<String>,
    edit: bool,
) -> Result<()> {
    let storage = TestCaseStorage::new(base_path)?;

    let id = match id {
        Some(id) => id,
        None => Prompts::input("Test Case ID")?,
    };

    let title = match title {
        Some(title) => title,
        None => Prompts::input("Test Case Title")?,
    };

    let mut test_case = TestCase::new(id, title);

    if edit {
        test_case = TestCaseEditor::create_test_case(&test_case)?;
    } else {
        test_case.priority = Prompts::select_priority()?;
        test_case.status = Prompts::select_status()?;
        test_case.test_type = Prompts::select_test_type()?;
        test_case.description = Prompts::input_optional("Description")?;
        test_case.author = Prompts::input_optional("Author")?;
        test_case.tags = Prompts::input_tags("Tags (comma-separated)")?;
    }

    let validator = TestCaseValidator::new()?;
    validator.validate_test_case(&test_case)?;

    let file_path = storage.save_test_case(&test_case)?;
    println!("Test case created: {}", file_path.display());

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
        let index = Prompts::select("Select a test case", &ids)?;
        test_cases[index].clone()
    };

    let mut edited_test_case = TestCaseEditor::edit_test_case(&test_case)?;
    edited_test_case.touch();

    let validator = TestCaseValidator::new()?;
    validator.validate_test_case(&edited_test_case)?;

    let file_path = storage.save_test_case(&edited_test_case)?;
    println!("Test case updated: {}", file_path.display());

    Ok(())
}

fn handle_list(
    base_path: &str,
    tag: Option<String>,
    status: Option<String>,
    priority: Option<String>,
    verbose: bool,
) -> Result<()> {
    let storage = TestCaseStorage::new(base_path)?;
    let mut test_cases = storage.load_all_test_cases()?;

    if let Some(tag) = tag {
        test_cases.retain(|tc| tc.tags.contains(&tag));
    }

    if let Some(status_str) = status {
        test_cases
            .retain(|tc| format!("{:?}", tc.status).to_lowercase() == status_str.to_lowercase());
    }

    if let Some(priority_str) = priority {
        test_cases.retain(|tc| {
            format!("{:?}", tc.priority).to_lowercase() == priority_str.to_lowercase()
        });
    }

    if test_cases.is_empty() {
        println!("No test cases found.");
        return Ok(());
    }

    println!("Found {} test case(s):\n", test_cases.len());

    for tc in test_cases {
        if verbose {
            println!("ID: {}", tc.id);
            println!("Title: {}", tc.title);
            println!("Priority: {:?}", tc.priority);
            println!("Status: {:?}", tc.status);
            println!("Type: {:?}", tc.test_type);
            if !tc.tags.is_empty() {
                println!("Tags: {}", tc.tags.join(", "));
            }
            if let Some(desc) = &tc.description {
                println!("Description: {}", desc);
            }
            println!("Sequences: {}", tc.sequences.len());
            println!();
        } else {
            println!(
                "{:<15} {:<40} {:?}/{:?}",
                tc.id, tc.title, tc.priority, tc.status
            );
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
        let index = Prompts::select("Select a test case", &ids)?;
        test_cases[index].clone()
    };

    let yaml = serde_yaml::to_string(&test_case)?;
    println!("{}", yaml);

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
            println!("Cancelled.");
            return Ok(());
        }
    }

    storage.delete_test_case(id)?;
    println!("Test case deleted: {}", id);

    Ok(())
}

fn handle_validate(base_path: &str, file: Option<String>, all: bool) -> Result<()> {
    let storage = TestCaseStorage::new(base_path)?;
    let validator = TestCaseValidator::new()?;

    if let Some(file_path) = file {
        let test_case = storage.load_test_case(&file_path)?;
        validator.validate_test_case(&test_case)?;
        println!("✓ Valid: {}", file_path);
    } else if all {
        let test_cases = storage.load_all_test_cases()?;
        let mut errors = 0;

        for test_case in test_cases {
            match validator.validate_test_case(&test_case) {
                Ok(_) => println!("✓ Valid: {}", test_case.id),
                Err(e) => {
                    println!("✗ Invalid: {} - {}", test_case.id, e);
                    errors += 1;
                }
            }
        }

        if errors > 0 {
            anyhow::bail!("{} validation error(s) found", errors);
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
        println!("{}", yaml);
    }

    Ok(())
}

fn handle_export(base_path: &str, output: &str, tags: Option<String>) -> Result<()> {
    let storage = TestCaseStorage::new(base_path)?;
    let mut test_cases = storage.load_all_test_cases()?;

    if let Some(tags_str) = tags {
        let required_tags: Vec<String> =
            tags_str.split(',').map(|s| s.trim().to_string()).collect();
        test_cases.retain(|tc| required_tags.iter().any(|tag| tc.tags.contains(tag)));
    }

    let test_suite = TestSuite {
        name: "Exported Test Suite".to_string(),
        description: Some("Exported from test case repository".to_string()),
        version: Some("1.0".to_string()),
        test_cases,
        metadata: std::collections::HashMap::new(),
    };

    let file_path = storage.save_test_suite(&test_suite, output)?;
    println!("Test suite exported: {}", file_path.display());

    Ok(())
}

fn handle_import(base_path: &str, file: &str, skip_validation: bool) -> Result<()> {
    let storage = TestCaseStorage::new(base_path)?;
    let test_suite = storage.load_test_suite(file)?;

    let validator = TestCaseValidator::new()?;

    for test_case in test_suite.test_cases {
        if !skip_validation {
            validator.validate_test_case(&test_case)?;
        }

        storage.save_test_case(&test_case)?;
        println!("Imported: {}", test_case.id);
    }

    Ok(())
}

fn handle_git(base_path: &str, command: GitCommands) -> Result<()> {
    let git = GitManager::open(base_path).or_else(|_| GitManager::init(base_path))?;

    match command {
        GitCommands::Add { ids, all } => {
            if all {
                git.add_all()?;
                println!("All files added to staging");
            } else {
                let paths: Vec<_> = ids
                    .iter()
                    .map(|id| format!("{}.yaml", id))
                    .map(std::path::PathBuf::from)
                    .collect();
                git.add(&paths)?;
                println!("Added {} file(s) to staging", paths.len());
            }
        }

        GitCommands::Commit { message } => {
            let author_name = std::env::var("GIT_AUTHOR_NAME")
                .unwrap_or_else(|_| "Test Case Manager".to_string());
            let author_email = std::env::var("GIT_AUTHOR_EMAIL")
                .unwrap_or_else(|_| "testcase@example.com".to_string());

            let oid = git.commit(&message, &author_name, &author_email)?;
            println!("Committed: {}", oid);
        }

        GitCommands::Status => {
            let statuses = git.status()?;
            if statuses.is_empty() {
                println!("No changes");
            } else {
                for (path, status) in statuses {
                    println!("{:?} {}", status, path);
                }
            }
        }

        GitCommands::Log { limit } => {
            let commits = git.log(limit)?;
            for commit in commits {
                println!(
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
    println!(
        "Initialized test case repository: {}",
        storage.base_path().display()
    );

    if init_git {
        GitManager::init(path)?;
        println!("Initialized git repository");

        let gitignore_path = std::path::Path::new(path).join(".gitignore");
        if !gitignore_path.exists() {
            std::fs::write(&gitignore_path, "*.bak\n*.tmp\n.DS_Store\n")?;
            println!("Created .gitignore");
        }
    }

    Ok(())
}

fn handle_create_interactive(path: &str) -> Result<()> {
    let mut builder = TestCaseBuilder::new(path).context("Failed to create test case builder")?;

    println!("\n╔═══════════════════════════════════════════════╗");
    println!("║   Interactive Test Case Creation Workflow    ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    builder.add_metadata().context("Failed to add metadata")?;

    println!("✓ Metadata added to structure\n");

    if Prompts::confirm("Commit metadata to git?")? {
        builder
            .commit("Add test case metadata")
            .context("Failed to commit metadata")?;
    }

    if Prompts::confirm("\nAdd general initial conditions?")? {
        builder
            .add_general_initial_conditions(None)
            .context("Failed to add general initial conditions")?;

        println!("✓ General initial conditions added\n");

        if Prompts::confirm("Commit general initial conditions to git?")? {
            builder
                .commit("Add general initial conditions")
                .context("Failed to commit general initial conditions")?;
        }
    }

    if Prompts::confirm("\nAdd initial conditions?")? {
        builder
            .add_initial_conditions(None)
            .context("Failed to add initial conditions")?;

        println!("✓ Initial conditions added\n");

        if Prompts::confirm("Commit initial conditions to git?")? {
            builder
                .commit("Add initial conditions")
                .context("Failed to commit initial conditions")?;
        }
    }

    let file_path = builder.save().context("Failed to save test case")?;

    println!("\n╔═══════════════════════════════════════════════╗");
    println!("║          Test Case Created Successfully       ║");
    println!("╚═══════════════════════════════════════════════╝");
    println!("\nSaved to: {}", file_path.display());

    Ok(())
}

fn handle_build_sequences(path: &str) -> Result<()> {
    let mut builder = TestCaseBuilder::new(path).context("Failed to create test case builder")?;

    println!("\n╔═══════════════════════════════════════════════╗");
    println!("║   Test Sequence Builder with Git Commits     ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    builder.add_metadata().context("Failed to add metadata")?;

    println!("✓ Metadata added to structure\n");

    if Prompts::confirm("Commit metadata to git?")? {
        builder
            .commit("Add test case metadata")
            .context("Failed to commit metadata")?;
    }

    if Prompts::confirm("\nAdd general initial conditions?")? {
        builder
            .add_general_initial_conditions(None)
            .context("Failed to add general initial conditions")?;

        println!("✓ General initial conditions added\n");

        if Prompts::confirm("Commit general initial conditions to git?")? {
            builder
                .commit("Add general initial conditions")
                .context("Failed to commit general initial conditions")?;
        }
    }

    if Prompts::confirm("\nAdd initial conditions?")? {
        builder
            .add_initial_conditions(None)
            .context("Failed to add initial conditions")?;

        println!("✓ Initial conditions added\n");

        if Prompts::confirm("Commit initial conditions to git?")? {
            builder
                .commit("Add initial conditions")
                .context("Failed to commit initial conditions")?;
        }
    }

    builder
        .build_test_sequences_with_commits()
        .context("Failed to build test sequences")?;

    let file_path = builder.save().context("Failed to save test case")?;

    println!("\n╔═══════════════════════════════════════════════╗");
    println!("║    Test Sequences Built Successfully          ║");
    println!("╚═══════════════════════════════════════════════╝");
    println!("\nSaved to: {}", file_path.display());

    if Prompts::confirm("\nCommit final file?")? {
        builder
            .commit("Complete test case with all sequences")
            .context("Failed to commit final file")?;
    }

    Ok(())
}

fn handle_add_steps(path: &str, sequence_id: Option<i64>) -> Result<()> {
    let mut builder = TestCaseBuilder::new(path).context("Failed to create test case builder")?;

    println!("\n╔═══════════════════════════════════════════════╗");
    println!("║      Add Steps to Sequence with Commits      ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    builder.add_metadata().context("Failed to add metadata")?;

    println!("✓ Metadata added to structure\n");

    if Prompts::confirm("Commit metadata to git?")? {
        builder
            .commit("Add test case metadata")
            .context("Failed to commit metadata")?;
    }

    if Prompts::confirm("\nAdd general initial conditions?")? {
        builder
            .add_general_initial_conditions(None)
            .context("Failed to add general initial conditions")?;

        println!("✓ General initial conditions added\n");

        if Prompts::confirm("Commit general initial conditions to git?")? {
            builder
                .commit("Add general initial conditions")
                .context("Failed to commit general initial conditions")?;
        }
    }

    if Prompts::confirm("\nAdd initial conditions?")? {
        builder
            .add_initial_conditions(None)
            .context("Failed to add initial conditions")?;

        println!("✓ Initial conditions added\n");

        if Prompts::confirm("Commit initial conditions to git?")? {
            builder
                .commit("Add initial conditions")
                .context("Failed to commit initial conditions")?;
        }
    }

    builder
        .add_test_sequence_interactive()
        .context("Failed to add test sequence")?;

    if Prompts::confirm("Commit this sequence to git?")? {
        let sequence_id_val = builder.get_next_sequence_id() - 1;
        let commit_msg = format!("Add test sequence #{}", sequence_id_val);
        builder
            .commit(&commit_msg)
            .context("Failed to commit test sequence")?;
    }

    let seq_id = if let Some(id) = sequence_id {
        id
    } else {
        builder.get_next_sequence_id() - 1
    };

    builder
        .add_steps_to_sequence_by_id_with_commits(seq_id)
        .context("Failed to add steps to sequence")?;

    let file_path = builder.save().context("Failed to save test case")?;

    println!("\n╔═══════════════════════════════════════════════╗");
    println!("║        Steps Added Successfully               ║");
    println!("╚═══════════════════════════════════════════════╝");
    println!("\nSaved to: {}", file_path.display());

    if Prompts::confirm("\nCommit final file?")? {
        builder
            .commit("Complete test sequence with all steps")
            .context("Failed to commit final file")?;
    }

    Ok(())
}

fn handle_build_sequences_with_steps(path: &str) -> Result<()> {
    let mut builder = TestCaseBuilder::new(path).context("Failed to create test case builder")?;

    println!("\n╔═══════════════════════════════════════════════╗");
    println!("║ Build Test Sequences & Steps with Commits    ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    builder.add_metadata().context("Failed to add metadata")?;

    println!("✓ Metadata added to structure\n");

    if Prompts::confirm("Commit metadata to git?")? {
        builder
            .commit("Add test case metadata")
            .context("Failed to commit metadata")?;
    }

    if Prompts::confirm("\nAdd general initial conditions?")? {
        builder
            .add_general_initial_conditions(None)
            .context("Failed to add general initial conditions")?;

        println!("✓ General initial conditions added\n");

        if Prompts::confirm("Commit general initial conditions to git?")? {
            builder
                .commit("Add general initial conditions")
                .context("Failed to commit general initial conditions")?;
        }
    }

    if Prompts::confirm("\nAdd initial conditions?")? {
        builder
            .add_initial_conditions(None)
            .context("Failed to add initial conditions")?;

        println!("✓ Initial conditions added\n");

        if Prompts::confirm("Commit initial conditions to git?")? {
            builder
                .commit("Add initial conditions")
                .context("Failed to commit initial conditions")?;
        }
    }

    builder
        .build_test_sequences_with_step_commits()
        .context("Failed to build test sequences with steps")?;

    let file_path = builder.save().context("Failed to save test case")?;

    println!("\n╔═══════════════════════════════════════════════╗");
    println!("║  Test Sequences & Steps Built Successfully   ║");
    println!("╚═══════════════════════════════════════════════╝");
    println!("\nSaved to: {}", file_path.display());

    if Prompts::confirm("\nCommit final file?")? {
        builder
            .commit("Complete test case with all sequences and steps")
            .context("Failed to commit final file")?;
    }

    Ok(())
}
