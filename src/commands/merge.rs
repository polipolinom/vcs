#![forbid(unsafe_code)]

use crate::library::vcs_repository::VcsRepository;
use crate::utils::extract_files::files_from_commit;
use crate::utils::operation_hash::calculate_hash;
use crate::utils::print_files::print_changed_paths;
use crate::utils::update_repo::update_repo;
use crate::utils::work_with_commit_files::both_changed;
use crate::utils::work_with_commit_files::{added_files, modified_files};

/// Merge changes from the brunch with the given name into the master and creates commit with the result.
pub fn merge(vcs: &mut VcsRepository, branch_name: &str) -> Result<(), &'static str> {
    if vcs.get_current_commit() != vcs.get_last_master_commit() {
        return Err("Current commit no master last commit");
    }

    let branch = vcs.get_branch_by_name(&branch_name)?.clone();

    vcs.check_no_uncommited()?;

    let branch_first_commit = branch.get_first_commit().clone();
    let commit_ancestor_hash = branch_first_commit.get_parent_hash()?;
    let common_ancestor = vcs.get_commit_by_hash(commit_ancestor_hash)?;

    let files_ancestor = files_from_commit(common_ancestor)?;
    let files_branch = files_from_commit(branch.get_last_commit())?;
    let files_master = files_from_commit(vcs.get_current_commit())?;

    let both_changed_files = both_changed(&files_ancestor, &files_master, &files_branch);
    if !both_changed_files.is_empty() {
        println!("Merge confilict: file has been changed both in master and branch");
        for file in both_changed_files.iter() {
            println!("  {}", file.display());
        }
        return Err("Aborting");
    }

    let mut nxt_commit = vcs
        .get_current_commit()
        .merge_init(branch.get_last_commit(), &branch_name)?;

    let old_files = files_from_commit(vcs.get_current_commit())?;
    let new_files = files_from_commit(branch.get_last_commit())?;

    let mut added = added_files(&new_files, &old_files);
    added.append(&mut added_files(&old_files, &new_files));
    let modified = modified_files(&new_files, &old_files);

    vcs.get_mut_current_branch().add_commit(&mut nxt_commit);
    vcs.change_current_commit(&nxt_commit);
    vcs.add_commit(&nxt_commit);

    println!("Successfully created merge commit:");
    println!(
        "[master {}] merge branch {}.",
        calculate_hash(&nxt_commit),
        branch_name
    );

    if modified.is_empty() && added.is_empty() {
        println!("No changes to be committed");
    } else {
        println!("  {} files modified, {} added", modified.len(), added.len(),);
        print_changed_paths(&added, &modified, &Vec::new(), vcs.get_dir());
    }

    vcs.delete_branch(&branch_name);
    update_repo(vcs)?;
    println!("Deleted {}", branch_name);

    Ok(())
}
