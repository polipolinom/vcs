#![forbid(unsafe_code)]

use super::commit::Commit;
use serde::Deserialize;
use serde::Serialize;

///
/// This is a struct for working with branches.
#[derive(Clone, Serialize, Deserialize)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Branch {
    first_commit: Commit,
    last_commit: Commit,
    name: String,
}

impl Branch {
    /// Creates master branch.
    pub fn init_master(commit: &Commit) -> Self {
        Self {
            first_commit: commit.clone(),
            last_commit: commit.clone(),
            name: "master".to_string(),
        }
    }

    /// Creates branch with given first commit and name.
    pub fn init(commit: &Commit, name: &str) -> Self {
        Self {
            first_commit: commit.clone(),
            last_commit: commit.clone(),
            name: name.to_string(),
        }
    }

    /// Returns the name of the branch.
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the reference to the last commit of the branch.
    pub fn get_last_commit(&self) -> &Commit {
        &self.last_commit
    }


    /// Returns the reference to the first commit of the branch.
    pub fn get_first_commit(&self) -> &Commit {
        &self.first_commit
    }

    /// Adds given commit to the end of the branch.
    pub fn add_commit(&mut self, commit: &Commit) {
        self.last_commit = commit.clone();
    }
}
