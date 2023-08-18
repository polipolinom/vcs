#![forbid(unsafe_code)]

use crate::library::vcs_repository::VcsRepository;
use crate::utils::extract_files::files_from_commit;
use crate::utils::operation_hash::calculate_hash;
use crate::utils::print_files::print_changed_paths;
use crate::utils::work_with_commit_files::{added_files, deleted_files, modified_files};

/// Outputs a list to the terminal from the repository initialization to the current one.
pub fn log(vcs: &VcsRepository) -> Result<(), &'static str> {
    let mut commit_to_log = Some(vcs.get_current_commit());
    while let Some(commit) = commit_to_log {
        println!("commit {}", calculate_hash(&commit));
        println!("Date: {}", format!("{}", commit.get_date().format("%c %z")));
        println!("Message: {}", commit.get_msg());

        if commit.is_initial() {
            println!(" No changes");
            commit_to_log = None;
        } else {
            let nxt_commit = vcs.get_commit_by_hash(commit.get_parent_hash()?)?;

            let new_files = &files_from_commit(&commit)?;
            let old_files = &files_from_commit(&nxt_commit)?;

            let added = added_files(new_files, old_files);
            let modified = modified_files(new_files, old_files);
            let deleted = deleted_files(new_files, old_files);

            if added.is_empty() && modified.is_empty() && deleted.is_empty() {
                println!("  No changes");
            } else {
                println!("Changes: ");
                print_changed_paths(&added, &modified, &deleted, vcs.get_dir());
            }

            commit_to_log = Some(nxt_commit);
        }
    }

    Ok(())
}
