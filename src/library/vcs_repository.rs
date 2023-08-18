#![forbid(unsafe_code)]

use super::branch::Branch;
use super::commit::Commit;
use crate::utils::delete_files::delete_commit_files;
use crate::utils::extract_files::{files_from_commit, files_from_dir};
use crate::utils::operation_hash::calculate_hash;
use crate::utils::work_with_commit_files::all_changed_files;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};


///
/// This is a struct for working with vcs repository.
#[derive(Serialize, Deserialize)]
pub struct VcsRepository {
    pub branches: Vec<Branch>,
    current_branch_id: usize,
    all_commits: HashMap<u64, Commit>,
    root_dir: PathBuf,
    current_commit: Commit,
}

impl VcsRepository {
    /// Сreates a VCS in a directory following given path.
    pub fn init(path: &Path) -> Result<Self, &'static str> {
        let commit = Commit::init(path, "Initial commit", None, true)?;
        let current_branch = Branch::init_master(&commit);
        Ok(Self {
            current_commit: commit.clone(),
            current_branch_id: 0,
            branches: vec![current_branch],
            all_commits: HashMap::from([(calculate_hash(&commit), commit)]),
            root_dir: path.to_path_buf(),
        })
    }

    // get

    /// Returns the directory of repository.
    pub fn get_dir(&self) -> &Path {
        &self.root_dir
    }

    /// Returns the immutable refernce to VCS current branch. 
    pub fn get_current_branch(&self) -> &Branch {
        &self.branches[self.current_branch_id]
    }

    /// Returns the mutable reference to the VCS curretn branch.
    pub fn get_mut_current_branch(&mut self) -> &mut Branch {
        &mut self.branches[self.current_branch_id]
    }

    /// Returns the immutable reference to the VCS current commit.
    pub fn get_current_commit(&self) -> &Commit {
        &self.current_commit
    }

    /// Returns the immutable reference to the last commit of VCS current branch.
    pub fn get_last_branch_commit(&self) -> &Commit {
        let branch = self.get_current_branch();
        branch.get_last_commit()
    }

    /// Returns the immutable reference to tha branch with given name.
    pub fn get_branch_by_name(&self, name: &str) -> Result<&Branch, &'static str> {
        for branch in self.branches.iter() {
            if branch.get_name() == name {
                return Ok(&branch);
            }
        }
        Err("No branch with this name")
    }

    /// Returns the immutable reference to the commit with given hash.
    pub fn get_commit_by_hash(&self, hash: u64) -> Result<&Commit, &'static str> {
        match self.all_commits.get(&hash) {
            None => Err("No commit with this hash"),
            Some(commit) => Ok(commit),
        }
    }

    /// Returns the immutable reference to the branch with given first commit.
    pub fn get_branch_by_first_commit(&self, commit: &Commit) -> Result<&Branch, &'static str> {
        for branch in self.branches.iter() {
            if branch.get_first_commit() == commit {
                return Ok(&branch);
            }
        }
        Err("No branch with given first commit")
    }

    /// Returns the immutable reference to the branch with given commit.
    pub fn get_branch_by_commit(&self, commit: &Commit) -> Result<&Branch, &'static str> {
        if commit.is_first() {
            self.get_branch_by_first_commit(commit)
        } else {
            let parent_commit_hash = commit.get_parent_hash()?;
            let parent_commit = self.get_commit_by_hash(parent_commit_hash)?;
            self.get_branch_by_commit(parent_commit)
        }
    }

    /// Returns the branch ID in this state VCS.
    pub fn get_branch_id(&self, branch: &Branch) -> Result<usize, &'static str> {
        for i in 0..self.branches.len() {
            if self.branches[i].get_name() == branch.get_name() {
                return Ok(i);
            }
        }
        Err("No branch")
    }

    /// Returns the immutable reference to the last commit of VCS master branch.
    pub fn get_last_master_commit(&self) -> &Commit {
        self.branches[0].get_last_commit()
    }

    // change

    /// Changes the current commit of the VCS.
    pub fn change_current_commit(&mut self, commit: &Commit) {
        self.current_commit = commit.clone();
    }

    /// Changes the current branch of the VCS.
    pub fn change_current_branch(&mut self, branch: &Branch) -> Result<(), &'static str> {
        self.current_branch_id = self.get_branch_id(branch)?;
        Ok(())
    }

    /// Adds commit and commit hash to the struct.
    pub fn add_commit(&mut self, commit: &Commit) {
        self.all_commits
            .insert(calculate_hash(commit), commit.clone());
    }

    /// Adds branch to the VCS.
    pub fn add_branch(&mut self, branch: &Branch) {
        self.branches.push(branch.clone());
        self.current_branch_id = self.branches.len() - 1;
        self.current_commit = branch.get_last_commit().clone();
        self.all_commits.insert(
            calculate_hash(&self.current_commit),
            self.current_commit.clone(),
        );
    }


    /// Deletes all commшts of given branch with their directories.
    fn delete_all_commits(&mut self, branch: &Branch) {
        let mut commit = branch.get_last_commit().clone();
        while !commit.is_first() {
            self.all_commits.remove(&calculate_hash(&commit));
            delete_commit_files(&commit);
            commit = self
                .get_commit_by_hash(commit.get_parent_hash().unwrap())
                .unwrap()
                .clone();
        }
        self.all_commits.remove(&calculate_hash(&commit));
        delete_commit_files(&commit);
    }

    /// Deletes given branch.
    pub fn delete_branch(&mut self, branch_name: &str) {
        for ind in 0..self.branches.len() {
            if self.branches[ind].get_name() == branch_name {
                self.delete_all_commits(&self.branches[ind].clone());
                self.branches.remove(ind);
                return;
            }
        }
    }

    // check

    /// Checks existance of the branch with name branch_name.
    pub fn exists_branch(&self, branch_name: &str) -> bool {
        self.get_branch_by_name(branch_name).is_ok()
    }

    /// Checks for uncommitted files in the directory to which the VCS is linked
    pub fn check_no_uncommited(&self) -> Result<(), &'static str> {
        let repo_files = files_from_dir(&self.root_dir)?;
        let commit_files = files_from_commit(&self.current_commit)?;

        if !all_changed_files(&commit_files, &repo_files).is_empty() {
            return Err("uncommited files");
        }

        Ok(())
    }
}
