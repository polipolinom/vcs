#![forbid(unsafe_code)]

use super::extract_files::files_from_commit;
use crate::library::files::File;
use crate::library::vcs_repository::VcsRepository;
use std::fs;
use std::io::Write;
use std::path::Path;

fn clean_dir(dir: &Path) -> Result<(), &'static str> {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.expect("Path doesn't exists");
        let path = entry.path();
        if path.is_dir() {
            if path.ends_with(".vcs") {
                continue;
            }
            fs::remove_dir_all(path).unwrap();
        } else {
            fs::remove_file(path).unwrap();
        }
    }
    Ok(())
}

fn add_file(file: &File) {
    let mut path_to_file = file.get_path().to_path_buf();
    path_to_file.pop();
    fs::create_dir_all(path_to_file).unwrap();
    let mut output = fs::File::create(file.get_path()).unwrap();
    output.write_all(&file.get_data()).unwrap();
}


/// Updates user repository.
pub fn update_repo(vcs: &VcsRepository) -> Result<(), &'static str> {
    clean_dir(vcs.get_dir())?;
    let files = files_from_commit(&vcs.get_current_commit())?;
    for file in files.iter() {
        add_file(file);
    }
    Ok(())
}
