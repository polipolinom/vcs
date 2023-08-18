#![forbid(unsafe_code)]

use crate::library::commit::Commit;
use crate::library::files::File;
use crate::library::vcs_repository::VcsRepository;
use std::collections::VecDeque;
use std::fs;
use std::io::BufReader;
use std::path::{Path, PathBuf};

/// Extract files from the given dir except for directory ".vcs".
pub fn files_from_dir(dir_root: &Path) -> Result<Vec<File>, &'static str> {
    let mut ans: Vec<File> = vec![];
    let mut dirs: VecDeque<PathBuf> = VecDeque::new();
    dirs.push_back(dir_root.to_path_buf());

    while !dirs.is_empty() {
        let dir = dirs.pop_front().unwrap();
        if dir.ends_with(".vcs") {
            continue;
        }
        if !dir.is_dir() {
            return Err("Given not a directory");
        }
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.expect("Path doesn't exist");
            let path = entry.path();
            if path.ends_with(".vcs") {
                continue;
            }
            if path.is_dir() {
                dirs.push_back(path);
            } else {
                ans.push(File::init(&path))
            }
        }
    }
    Ok(ans)
}

fn read_from_json<P: AsRef<Path>>(path: P) -> Result<File, &'static str> {
    let file = fs::File::open(path).expect("No file with this name");
    let reader = BufReader::new(file);
    let f = serde_json::from_reader(reader).expect("Can't read file");
    Ok(f)
}


/// Extract the commit files.
pub fn files_from_commit(commit: &Commit) -> Result<Vec<File>, &'static str> {
    let dir = commit.get_dir_commit();
    let mut ans: Vec<File> = vec![];

    if !dir.exists() {
        return Err("No dir with this path-commit");
    }

    if !dir.is_dir() {
        return Err("Path-commit is not a directory");
    }

    for entry in fs::read_dir(&dir).unwrap() {
        let entry = entry.expect("Path doesn't exist");
        let path = entry.path();
        ans.push(read_from_json(path).unwrap());
    }
    Ok(ans)
}

fn read_vcs_from_json(path: PathBuf) -> Result<VcsRepository, &'static str> {
    let file = fs::File::open(path).expect("No file with this path");
    let reader = BufReader::new(file);
    let vcs = serde_json::from_reader(reader).expect("Can't read file");
    Ok(vcs)
}

/// Read buffer of VCS repository.
pub fn read_vcs(root_path: PathBuf) -> Result<VcsRepository, &'static str> {
    let vcs_dir = root_path.join(".vcs").join("VCSRepository.json");
    read_vcs_from_json(vcs_dir)
}
