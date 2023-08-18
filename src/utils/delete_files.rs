#![forbid(unsafe_code)]

use crate::library::commit::Commit;
use std::fs;

/// Deletes files of given commit.
pub fn delete_commit_files(commit: &Commit) {
    let path = commit.get_dir_commit();
    println!("{}", path.display());
    fs::remove_dir_all(path).unwrap();
}
