#![forbid(unsafe_code)]

extern crate array_tool;

use crate::utils::extract_files::{files_from_commit, files_from_dir};
use crate::utils::operation_hash::calculate_hash;
use crate::utils::pack_files;
use crate::utils::work_with_commit_files::{added_files_with_data, modified_files_with_data};
use chrono::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::path::{Path, PathBuf};

///
/// This is a struct for working with commits.
#[derive(Hash, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Debug)]
pub struct Commit {
    parent: Option<u64>,
    message: String,
    is_first_in_branch: bool,
    root_path: PathBuf,
    date: DateTime<Local>,
}

impl Commit {
    /// Creates new commit.
    /// 
    /// # Arguments
    /// * `message` - The message of new commit
    /// * `parent` - The hash of parent commit in vcs-tree and None if commit - is the first master commit
    /// * `path` - The path to repository files
    /// * `is_first` - The flag indicating that the commit is the first in the his branch.
    pub fn init(
        path: &Path,
        msg: &str,
        parent: Option<u64>,
        is_first: bool,
    ) -> Result<Self, &'static str> {
        let res = Self {
            message: msg.to_string(),
            parent,
            is_first_in_branch: is_first,
            root_path: path.to_path_buf(),
            date: Local::now(),
        };

        pack_files::put_to_dir(
            &res.root_path,
            calculate_hash(&res).to_string().as_str(),
            &files_from_dir(path)?,
        )?;

        Ok(res)
    }

    /// Create a commit-merge of two commits.
    /// 
    /// # Arguments
    /// * `self` - The last commit of the master
    /// * `branch_commit` - The last commit of the branch
    /// * `branch_name` - The name of the branch that merges the master
    pub fn merge_init(
        &self,
        branch_commit: &Self,
        branch_name: &str,
    ) -> Result<Self, &'static str> {
        let mut msg = "Merge branch ".to_string();
        msg.push_str(branch_name);

        let res = Self {
            message: msg.clone(),
            parent: Some(calculate_hash(&self)),
            is_first_in_branch: false,
            root_path: self.root_path.clone(),
            date: Local::now(),
        };

        let mut old_files = files_from_commit(&self)?;
        let new_files = files_from_commit(branch_commit)?;

        let mut added = added_files_with_data(&new_files, &old_files);
        let modified = modified_files_with_data(&new_files, &old_files);

        old_files.append(&mut added);

        for mod_file in modified.iter() {
            for file in old_files.iter_mut() {
                if file.get_path() == mod_file.get_path() {
                    file.change_data(&mod_file.get_data());
                    break;
                }
            }
        }

        pack_files::put_to_dir(
            &res.root_path,
            calculate_hash(&res).to_string().as_str(),
            &old_files,
        )?;

        Ok(res)
    }

    //get

    /// Returns parent hash of the commit.
    pub fn get_parent_hash(&self) -> Result<u64, &'static str> {
        match self.parent {
            None => Err("No parent in branch_commit"),
            Some(x) => Ok(x),
        }
    }

    /// Returns message of the commit.
    pub fn get_msg(&self) -> &str {
        self.message.as_str()
    }

    /// Returns the date and time when the commit was created.
    pub fn get_date(&self) -> &DateTime<Local> {
        &self.date
    }

    /// Returns the directory where the commit files are located.
    pub fn get_dir_commit(&self) -> PathBuf {
        let dir = self.root_path.clone();
        let dir = dir
            .join(".vcs")
            .join("objects")
            .join(calculate_hash(&self).to_string().as_str());
        dir
    }

    //check
    /// Checks whether the given commit is the first in its branch.
    pub fn is_first(&self) -> bool {
        self.is_first_in_branch
    }

    /// Checks whether the given commit is the first in master.
    pub fn is_initial(&self) -> bool {
        self.parent.is_none()
    }
}
