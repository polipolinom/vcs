#![forbid(unsafe_code)]

use crate::library::branch::Branch;
use crate::library::commit::Commit;
use crate::library::vcs_repository::VcsRepository;
use crate::utils::operation_hash::calculate_hash;

/// Creates a new branch from the current commit in the master.
pub fn new_branch(vcs: &mut VcsRepository, branch_name: &str) -> Result<u64, &'static str> {
    if vcs.get_current_branch().get_name() != "master" {
        return Err("Current branch is not master");
    }

    if vcs.exists_branch(branch_name) {
        return Err("Branch exists");
    }

    let mut msg = "Initial commit ".to_string();
    msg.push_str(branch_name);

    let parent = calculate_hash(&vcs.get_current_commit());
    let commit = Commit::init(vcs.get_dir(), &msg, Some(parent), true)?;
    let branch = Branch::init(&commit, branch_name);

    vcs.add_branch(&branch);

    Ok(parent)
}
