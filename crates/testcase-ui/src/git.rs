use anyhow::{Context, Result};
use git2::{IndexAddOption, Repository, Signature, Status};
use std::path::Path;

/// Git operations manager
pub struct GitManager {
    repo: Repository,
}

impl GitManager {
    /// Open an existing git repository
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        log::debug!("Opening git repository at: {:?}", path.as_ref());
        let repo = Repository::open(path.as_ref()).context("Failed to open git repository")?;

        Ok(Self { repo })
    }

    /// Initialize a new git repository
    pub fn init<P: AsRef<Path>>(path: P) -> Result<Self> {
        log::info!("Initializing git repository at: {:?}", path.as_ref());
        let repo =
            Repository::init(path.as_ref()).context("Failed to initialize git repository")?;

        Ok(Self { repo })
    }

    /// Get the repository path
    pub fn path(&self) -> &Path {
        self.repo.path()
    }

    /// Get the working directory
    pub fn workdir(&self) -> Option<&Path> {
        self.repo.workdir()
    }

    /// Add files to the staging area
    pub fn add<P: AsRef<Path>>(&self, paths: &[P]) -> Result<()> {
        let mut index = self
            .repo
            .index()
            .context("Failed to get repository index")?;

        for path in paths {
            log::debug!("Adding path to git staging: {}", path.as_ref().display());
            index
                .add_path(path.as_ref())
                .context(format!("Failed to add path: {}", path.as_ref().display()))?;
        }

        index.write().context("Failed to write index")?;

        Ok(())
    }

    /// Add all changes to the staging area
    pub fn add_all(&self) -> Result<()> {
        let mut index = self
            .repo
            .index()
            .context("Failed to get repository index")?;

        index
            .add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
            .context("Failed to add all files")?;

        index.write().context("Failed to write index")?;

        Ok(())
    }

    /// Commit staged changes
    pub fn commit(
        &self,
        message: &str,
        author_name: &str,
        author_email: &str,
    ) -> Result<git2::Oid> {
        log::info!("Creating git commit: {}", message);
        let signature =
            Signature::now(author_name, author_email).context("Failed to create signature")?;

        let mut index = self
            .repo
            .index()
            .context("Failed to get repository index")?;

        let tree_id = index.write_tree().context("Failed to write tree")?;

        let tree = self
            .repo
            .find_tree(tree_id)
            .context("Failed to find tree")?;

        let parent_commit = match self.repo.head() {
            Ok(head) => {
                let commit = head
                    .peel_to_commit()
                    .context("Failed to peel head to commit")?;
                Some(commit)
            }
            Err(_) => None,
        };

        let parents: Vec<&git2::Commit> = parent_commit.iter().collect();

        let oid = self
            .repo
            .commit(
                Some("HEAD"),
                &signature,
                &signature,
                message,
                &tree,
                &parents,
            )
            .context("Failed to create commit")?;

        log::debug!("Commit created with OID: {}", oid);
        Ok(oid)
    }

    /// Get the status of the working directory
    pub fn status(&self) -> Result<Vec<(String, Status)>> {
        let statuses = self
            .repo
            .statuses(None)
            .context("Failed to get repository status")?;

        let mut result = Vec::new();

        for entry in statuses.iter() {
            if let Some(path) = entry.path() {
                result.push((path.to_string(), entry.status()));
            }
        }

        Ok(result)
    }

    /// Check if there are uncommitted changes
    pub fn has_changes(&self) -> Result<bool> {
        let statuses = self.status()?;
        Ok(!statuses.is_empty())
    }

    /// Get the current branch name
    pub fn current_branch(&self) -> Result<String> {
        let head = self.repo.head().context("Failed to get HEAD")?;

        if let Some(branch_name) = head.shorthand() {
            Ok(branch_name.to_string())
        } else {
            anyhow::bail!("HEAD is not a valid branch");
        }
    }

    /// Create a new branch
    pub fn create_branch(&self, name: &str) -> Result<()> {
        let head = self.repo.head().context("Failed to get HEAD")?;

        let commit = head
            .peel_to_commit()
            .context("Failed to peel HEAD to commit")?;

        self.repo
            .branch(name, &commit, false)
            .context(format!("Failed to create branch: {}", name))?;

        Ok(())
    }

    /// Checkout a branch
    pub fn checkout_branch(&self, name: &str) -> Result<()> {
        let obj = self
            .repo
            .revparse_single(&format!("refs/heads/{}", name))
            .context(format!("Failed to find branch: {}", name))?;

        self.repo
            .checkout_tree(&obj, None)
            .context("Failed to checkout tree")?;

        self.repo
            .set_head(&format!("refs/heads/{}", name))
            .context("Failed to set HEAD")?;

        Ok(())
    }

    /// Get commit history
    pub fn log(&self, limit: usize) -> Result<Vec<CommitInfo>> {
        let mut revwalk = self.repo.revwalk().context("Failed to create revwalk")?;

        revwalk.push_head().context("Failed to push HEAD")?;

        let mut commits = Vec::new();

        for (i, oid) in revwalk.enumerate() {
            if i >= limit {
                break;
            }

            let oid = oid.context("Failed to get commit OID")?;
            let commit = self
                .repo
                .find_commit(oid)
                .context("Failed to find commit")?;

            commits.push(CommitInfo {
                id: oid.to_string(),
                message: commit.message().unwrap_or("").to_string(),
                author: commit.author().name().unwrap_or("").to_string(),
                time: commit.time().seconds(),
            });
        }

        Ok(commits)
    }

    /// Stage and commit progress for a YAML file with a descriptive message
    ///
    /// This function stages the specified YAML file and creates a commit with a
    /// message that describes which step was completed.
    ///
    /// # Arguments
    ///
    /// * `yaml_file_path` - Path to the YAML file to stage (relative to repo root)
    /// * `step_description` - Description of the step that was completed
    /// * `author_name` - Name of the commit author
    /// * `author_email` - Email of the commit author
    ///
    /// # Returns
    ///
    /// The OID of the created commit
    pub fn commit_progress<P: AsRef<Path>>(
        &self,
        yaml_file_path: P,
        step_description: &str,
        author_name: &str,
        author_email: &str,
    ) -> Result<git2::Oid> {
        log::debug!(
            "Staging progress for: {}",
            yaml_file_path.as_ref().display()
        );
        self.add(&[yaml_file_path.as_ref()])
            .context("Failed to stage YAML file")?;

        let commit_message = format!("Complete step: {}", step_description);

        self.commit(&commit_message, author_name, author_email)
    }
}

/// Information about a commit
#[derive(Debug, Clone)]
pub struct CommitInfo {
    pub id: String,
    pub message: String,
    pub author: String,
    pub time: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_init_and_commit() {
        let temp_dir = TempDir::new().unwrap();
        let git = GitManager::init(temp_dir.path()).unwrap();

        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "test content").unwrap();

        git.add(&[Path::new("test.txt")]).unwrap();

        let oid = git
            .commit("Initial commit", "Test User", "test@example.com")
            .unwrap();

        assert!(!oid.is_zero());
    }

    #[test]
    fn test_commit_progress() {
        let temp_dir = TempDir::new().unwrap();
        let git = GitManager::init(temp_dir.path()).unwrap();

        let yaml_file = temp_dir.path().join("test-case.yaml");
        fs::write(&yaml_file, "id: TC001\ndescription: Test Case 1").unwrap();

        let oid = git
            .commit_progress(
                Path::new("test-case.yaml"),
                "User login validation",
                "Test User",
                "test@example.com",
            )
            .unwrap();

        assert!(!oid.is_zero());

        let commits = git.log(1).unwrap();
        assert_eq!(commits.len(), 1);
        assert_eq!(commits[0].message, "Complete step: User login validation");
    }

    #[test]
    fn test_commit_progress_multiple_steps() {
        let temp_dir = TempDir::new().unwrap();
        let git = GitManager::init(temp_dir.path()).unwrap();

        let yaml_file = temp_dir.path().join("test-case.yaml");

        fs::write(&yaml_file, "id: TC001\ndescription: Initial").unwrap();
        git.commit_progress(
            Path::new("test-case.yaml"),
            "Setup test environment",
            "Test User",
            "test@example.com",
        )
        .unwrap();

        fs::write(&yaml_file, "id: TC001\ndescription: Updated").unwrap();
        git.commit_progress(
            Path::new("test-case.yaml"),
            "Execute login test",
            "Test User",
            "test@example.com",
        )
        .unwrap();

        fs::write(&yaml_file, "id: TC001\ndescription: Final").unwrap();
        git.commit_progress(
            Path::new("test-case.yaml"),
            "Verify results",
            "Test User",
            "test@example.com",
        )
        .unwrap();

        let commits = git.log(3).unwrap();
        assert_eq!(commits.len(), 3);
        assert_eq!(commits[0].message, "Complete step: Verify results");
        assert_eq!(commits[1].message, "Complete step: Execute login test");
        assert_eq!(commits[2].message, "Complete step: Setup test environment");
    }
}
