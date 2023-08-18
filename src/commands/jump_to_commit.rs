#![forbid(unsafe_code)]

use crate::library::vcs_repository::VcsRepository;
use crate::utils::update_repo::update_repo;

/// Moves the repository to a commit with the given hash.
pub fn jump_to_commit(vcs: &mut VcsRepository, commit_hash: u64) -> Result<(), &'static str> {
    vcs.check_no_uncommited()?;

    let commit = vcs.get_commit_by_hash(commit_hash)?.clone();

    vcs.change_current_commit(&commit);
    let branch = vcs.get_branch_by_commit(&commit)?.clone();
    vcs.change_current_branch(&branch)?;

    update_repo(vcs)?;

    Ok(())
}
