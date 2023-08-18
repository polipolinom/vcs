#![forbid(unsafe_code)]

use crate::library::commit::Commit;
use crate::library::vcs_repository::VcsRepository;
use crate::utils::extract_files::{files_from_commit, files_from_dir};
use crate::utils::operation_hash::calculate_hash;
use crate::utils::print_files::print_changed_paths;
use crate::utils::work_with_commit_files::{added_files, deleted_files, modified_files};

/// Creates a new commit with the given message from the current changes 
/// or reports that there are no changes.
pub fn new_commit(vcs: &mut VcsRepository, msg: &str) -> Result<(), &'static str> {
    if vcs.get_last_branch_commit() != vcs.get_current_commit() {
        return Err("Current commit not last");
    }
    let repo_files = files_from_dir(vcs.get_dir())?;
    let commit_files = files_from_commit(&vcs.get_current_commit())?;

    let added = added_files(&repo_files, &commit_files);
    let modified = modified_files(&repo_files, &commit_files);
    let deleted = deleted_files(&repo_files, &commit_files);

    if added.is_empty() && modified.is_empty() && deleted.is_empty() {
        return Err("No changes");
    }

    let mut commit = Commit::init(
        vcs.get_dir(),
        msg,
        Some(calculate_hash(&vcs.get_current_commit())),
        false,
    )?;

    let commit_hash = calculate_hash(&commit);
    println!(
        "[{} {}] Work in progress",
        vcs.get_current_branch().get_name(),
        commit_hash
    );

    let branch = vcs.get_mut_current_branch();
    branch.add_commit(&mut commit);
    vcs.change_current_commit(&commit);
    vcs.add_commit(&commit);

    println!(
        "{} files changed, {} added, {} deleted",
        modified.len(),
        added.len(),
        deleted.len()
    );

    print_changed_paths(&added, &modified, &deleted, vcs.get_dir());

    Ok(())
}
