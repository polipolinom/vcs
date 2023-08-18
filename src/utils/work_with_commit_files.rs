#![forbid(unsafe_code)]

use crate::library::files::File;
use array_tool::vec::Intersect;
use std::path::PathBuf;

/// Returns added files in new_files compared to old_files.
pub fn added_files(new_files: &Vec<File>, old_files: &Vec<File>) -> Vec<PathBuf> {
    let mut ans: Vec<PathBuf> = vec![];
    for repo_file in new_files.iter() {
        let mut _file_find = false;
        for commit_file in old_files.iter() {
            if repo_file.is_change_only_data(commit_file) {
                _file_find = true;
                break;
            }
        }

        if _file_find == false {
            ans.push(repo_file.get_path().to_path_buf());
        }
    }
    ans
}

/// Returns modified files in new_files compared to old_files.
pub fn modified_files(new_files: &Vec<File>, old_files: &Vec<File>) -> Vec<PathBuf> {
    let mut ans: Vec<PathBuf> = vec![];
    for repo_file in new_files.iter() {
        for commit_file in old_files.iter() {
            if repo_file == commit_file {
                break;
            }
            if repo_file.is_change_only_data(commit_file) {
                ans.push(repo_file.get_path().to_path_buf());
                break;
            }
        }
    }
    ans
}

/// Returns deleted files in new_files compared to old_files.
pub fn deleted_files(new_files: &Vec<File>, old_files: &Vec<File>) -> Vec<PathBuf> {
    let mut ans: Vec<PathBuf> = vec![];
    for commit_file in old_files.iter() {
        let mut file_find = false;
        for repo_file in new_files.iter() {
            if commit_file.is_change_only_data(repo_file) {
                file_find = true;
                break;
            }
        }
        if file_find == false {
            ans.push(commit_file.get_path().to_path_buf());
        }
    }
    ans
}

/// Returns all changed files between two given vectors.
pub fn all_changed_files(new_files: &Vec<File>, old_files: &Vec<File>) -> Vec<PathBuf> {
    let mut changed_files = added_files(new_files, old_files);
    changed_files.append(&mut modified_files(new_files, old_files));
    changed_files.append(&mut deleted_files(new_files, old_files));

    changed_files
}

/// Returns files which both changed in two given vectors compared to the third
/// 
/// # Arguments
/// * `main_files` - files with which the files of the other two vectors are compared
/// * `files1`
/// * `files2`
pub fn both_changed(
    main_files: &Vec<File>,
    files1: &Vec<File>,
    files2: &Vec<File>,
) -> Vec<PathBuf> {
    let modified1 = modified_files(files1, main_files);
    let modified2 = modified_files(files2, main_files);

    let mut res = modified1.intersect(modified2);

    let added1 = added_files_with_data(files1, main_files);
    let added2 = added_files_with_data(files2, main_files);

    for file1 in added1.iter() {
        for file2 in added2.iter() {
            if file1.get_path() == file2.get_path() && file1.get_data() != file2.get_data() {
                res.push(file1.get_path().to_path_buf());
            }
        }
    }

    let deleted1 = deleted_files(files1, main_files);
    let deleted2 = deleted_files(files2, main_files);

    for path1 in deleted1.iter() {
        let mut _exists = false;
        for path2 in deleted2.iter() {
            if path1 == path2 {
                _exists = true;
                break;
            }
        }
        if _exists == false {
            res.push(path1.clone());
        }
    }

    for path2 in deleted1.iter() {
        let mut _exists = false;
        for path1 in deleted2.iter() {
            if path1 == path2 {
                _exists = true;
                break;
            }
        }
        if _exists == false {
            res.push(path2.clone());
        }
    }

    res
}

pub fn added_files_with_data(new_files: &Vec<File>, old_files: &Vec<File>) -> Vec<File> {
    let mut ans: Vec<File> = vec![];
    for repo_file in new_files.iter() {
        let mut _file_find = false;
        for commit_file in old_files.iter() {
            if repo_file.is_change_only_data(commit_file) {
                _file_find = true;
                break;
            }
        }

        if _file_find == false {
            ans.push(repo_file.clone());
        }
    }
    ans
}

pub fn modified_files_with_data(new_files: &Vec<File>, old_files: &Vec<File>) -> Vec<File> {
    let mut ans: Vec<File> = vec![];
    for repo_file in new_files.iter() {
        for commit_file in old_files.iter() {
            if repo_file == commit_file {
                break;
            }
            if repo_file.is_change_only_data(commit_file) {
                ans.push(repo_file.clone());
                break;
            }
        }
    }
    ans
}
