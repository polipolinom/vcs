#![forbid(unsafe_code)]

use super::extract_files::{files_from_commit, files_from_dir};
use super::work_with_commit_files::all_changed_files;
use crate::library::vcs_repository::VcsRepository;
use std::path::{Path, PathBuf};

/// Prints uncommitted files
pub fn print_uncommitted_files(vcs: &VcsRepository) -> Result<(), &'static str> {
    let repo_files = files_from_dir(vcs.get_dir())?;
    let commit_files = files_from_commit(vcs.get_current_commit())?;

    let files = all_changed_files(&repo_files, &commit_files);
    for path in files.iter() {
        println!("  {}", path.strip_prefix(vcs.get_dir()).unwrap().display());
    }

    Ok(())
}

/// Prints paths of changed files.
pub fn print_changed_paths(
    added: &Vec<PathBuf>,
    modified: &Vec<PathBuf>,
    deleted: &Vec<PathBuf>,
    root_dir: &Path,
) {
    for path in added.iter() {
        println!(
            "  added: {}",
            path.strip_prefix(root_dir).unwrap().display()
        );
    }
    for path in modified.iter() {
        println!(
            "  modified: {}",
            path.strip_prefix(root_dir).unwrap().display()
        );
    }
    for path in deleted.iter() {
        println!(
            "  deleted: {}",
            path.strip_prefix(root_dir).unwrap().display()
        );
    }
}
