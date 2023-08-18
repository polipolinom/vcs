#![forbid(unsafe_code)]

use crate::library::vcs_repository::VcsRepository;
use crate::utils::extract_files::{files_from_commit, files_from_dir};
use crate::utils::print_files::print_changed_paths;
use crate::utils::work_with_commit_files::{added_files, deleted_files, modified_files};

/// Displays the current status to the terminal.
pub fn status(vcs: &VcsRepository) -> Result<(), &str> {
    let repo_files = files_from_dir(vcs.get_dir())?;
    let commit_files = files_from_commit(&vcs.get_current_commit())?;

    let added = added_files(&repo_files, &commit_files);
    let modified = modified_files(&repo_files, &commit_files);
    let deleted = deleted_files(&repo_files, &commit_files);

    println!("On branch {}", vcs.get_current_branch().get_name());

    if added.is_empty() && modified.is_empty() && deleted.is_empty() {
        println!("No changes to be committed");
    } else {
        println!("Changes to be commited:");
        print_changed_paths(&added, &modified, &deleted, vcs.get_dir());
    }

    Ok(())
}
