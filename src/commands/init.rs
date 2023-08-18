#![forbid(unsafe_code)]

use crate::library::vcs_repository::VcsRepository;
use std::fs;
use std::path::Path;

/// Creates a directory.vcs in the given path, which will contain meta information on the repository, 
/// including the entire subtree of the directory folder. 
/// Creates a commit with the message "Initial commit".
pub fn init(path: &Path) -> Result<VcsRepository, &'static str> {
    fs::create_dir_all(path).unwrap();
    if !path.is_dir() {
        Err("is not a dir")
    } else if fs::create_dir(path.join(".vcs")).is_err() {
        Err("already exists")
    } else {
        let vcs = VcsRepository::init(path)?;
        Ok(vcs)
    }
}
