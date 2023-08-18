#![forbid(unsafe_code)]

use crate::library::files::File;
use crate::library::vcs_repository::VcsRepository;
use std::fs;
use std::io::Write;
use std::path::Path;

/// Put given files to the given dir
/// 
/// #Arguments
/// * `root_dir` - The directory when the craate directoru with files.
/// * `name_dir` - The name of the directory with files. 
/// * `vec_files` - vector of files.
pub fn put_to_dir(
    root_dir: &Path,
    name_dir: &str,
    vec_files: &Vec<File>,
) -> Result<(), &'static str> {
    let dir = root_dir;
    let dir = dir.join(".vcs").join("objects").join(name_dir);
    fs::create_dir_all(&dir).unwrap();

    for file in vec_files {
        let mut output = fs::File::create(&dir.join(file.get_name())).unwrap();
        output
            .write_all(serde_json::to_string(&file).unwrap().as_bytes())
            .expect("Error in put to dir");
    }

    Ok(())
}

/// Buffers the current state of the VCS
pub fn pack_vcs(root_dir: &Path, vcs: &VcsRepository) {
    let dir = root_dir.join(".vcs").join("VCSRepository.json");
    if dir.exists() {
        fs::remove_file(&dir).unwrap();
    }
    let mut output = fs::File::create(&dir).unwrap();
    output
        .write_all(serde_json::to_string(vcs).unwrap().as_bytes())
        .expect("Error with write vcs");
}
