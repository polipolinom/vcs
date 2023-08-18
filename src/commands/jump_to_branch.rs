#![forbid(unsafe_code)]

use super::jump_to_commit::jump_to_commit;
use crate::library::vcs_repository::VcsRepository;
use crate::utils::operation_hash::calculate_hash;

/// Moves the repository to the last commit of the branch with given branch.
pub fn jump_to_branch(vcs: &mut VcsRepository, branch_name: &str) -> Result<(), &'static str> {
    let branch = vcs.get_branch_by_name(branch_name)?;
    let commit = branch.get_last_commit();
    jump_to_commit(vcs, calculate_hash(&commit))
}
