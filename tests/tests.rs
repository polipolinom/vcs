#[warn(unreachable_code)]
#[warn(dead_code)]
#[warn(unused_variables)]

use std::path::Path;
use vcs::library::files::File;
use vcs::library::commit::Commit;
use vcs::library::branch::Branch;
use vcs::utils::operation_hash::calculate_hash;
use std::fs;
use std::io::Write;
use vcs::library::vcs_repository::VcsRepository;

#[test]
fn test_file_init() {
    let name = "text_file.txt";
    let path = Path::new("./tests/test_data/text_file.txt");
    let data: Vec<u8> = vec![97, 98, 99];
    let file = File::init(&path);
    assert_eq!(file.get_name(), name);
    assert_eq!(file.get_path().to_str(), path.to_str());
    assert_eq!(file.get_data().clone(), data);
}

#[test]
#[should_panic(expected = "Cannot read the file")]
fn test_file_give_not_file() {
    let path = Path::new("./src/library");
    File::init(&path);
}

#[test]
fn test_commit_init() {
    let path = Path::new(".\\tests\\test_data\\repo_with_vcs1");
    let msg = "Hello";

    let commit = Commit::init(path, msg, None, true);
    assert!(commit.is_ok());
    let commit = commit.unwrap();

    let hash = calculate_hash(&commit);

    let path = path.join(".vcs").join("objects").join(hash.to_string());
    assert!(path.exists());

    let path = path.join("aaaa.txt");
    assert!(path.exists());

    let path = Path::new(".\\tests\\test_data\\repo_with_vcs1");

    let commit2 = Commit::init(path, msg, None, false);
    assert!(commit2.is_ok());
    let commit = commit2.unwrap();

    fs::remove_dir_all(Path::new(".\\tests\\test_data\\repo_with_vcs1\\.vcs\\objects"));
}

#[test]
fn test_merge_init() {
    let path = Path::new(".\\tests\\test_data\\repo_with_vcs2");

    let mut output = fs::File::create(path.join("aaaba.txt")).unwrap();
    let data: Vec<u8> = vec![97, 97, 97, 97];
    output.write_all(&data);
    let commit1 = Commit::init(path, "first", None, true).unwrap();

    fs::File::create(path.join("add_file.txt"));
    let mut output = fs::File::create(path.join ("aaaba.txt")).unwrap();
    let data: Vec<u8> = vec![98, 99, 100, 101];
    output.write_all(&data);
    let commit2 = Commit::init(path, "first", None, true).unwrap();

    let commit3 = Commit::merge_init(&commit1, &commit2, "no-branch").unwrap();

    assert!(commit3.get_parent_hash().is_ok());
    assert_eq!(commit3.get_parent_hash().unwrap(), calculate_hash(&commit1));
    assert_eq!(commit3.get_msg(), "Merge branch no-branch");

    let hash = calculate_hash(&commit3);

    let path = path.join(".vcs").join("objects").join(hash.to_string());
    assert!(path.exists());
    let add_file = path.join("add_file.txt");
    let mod_file = path.join("aaaba.txt");
    assert!(add_file.exists());
    assert!(mod_file.exists());

    fs::remove_dir_all(Path::new(".\\tests\\test_data\\repo_with_vcs2\\.vcs\\objects"));
    fs::remove_file(Path::new(".\\tests\\test_data\\repo_with_vcs2\\add_file.txt"));
}


#[test]
pub fn test_add_branch() {
    let path = Path::new("./tests/test_data/repo");

    let vcs = VcsRepository::init(path);
    assert!(vcs.is_ok());
    let mut vcs = vcs.unwrap();
    
    fs::File::create(path.join("add_file.txt"));
    let mut output = fs::File::create(path.join("aaaa.txt")).unwrap();
    let data: Vec<u8> = vec![98, 99, 100, 101];
    output.write_all(&data);

    let commit = Commit::init(path, "my_commit", 
                                Some(calculate_hash(vcs.get_current_commit())), 
                                true);
    assert!(commit.is_ok());
    let commit = commit.unwrap();
    let hash = calculate_hash(&commit);

    let branch = Branch::init(&commit, "branch_name");
    assert_eq!(branch.get_name(), "branch_name");

    vcs.add_branch(&branch);

    assert_eq!(vcs.get_current_branch().clone(), branch);
    assert!(vcs.get_commit_by_hash(hash).is_ok());
    assert_eq!(vcs.get_commit_by_hash(hash).unwrap().clone(), commit);
    assert_eq!(vcs.get_current_commit().clone(), commit);

    fs::remove_dir_all(Path::new("./tests/test_data/repo/.vcs"));
    fs::remove_file(Path::new("./tests/test_data/repo/add_file.txt"));    
    fs::remove_file(Path::new("./tests/test_data/repo/aaaa.txt"));    
}

#[test]
pub fn test_delete_branch() {
    let path = Path::new("./tests/test_data/repo1");

    let vcs = VcsRepository::init(path);
    assert!(vcs.is_ok());
    let mut vcs = vcs.unwrap();
    
    fs::File::create(path.join("add_file.txt"));
    let mut output = fs::File::create(path.join("aaaa.txt")).unwrap();
    let data: Vec<u8> = vec![98, 99, 100, 101];
    output.write_all(&data);

    let commit = Commit::init(path, "commit", 
                                Some(calculate_hash(vcs.get_current_commit())), 
                                true);
    assert!(commit.is_ok());
    let commit = commit.unwrap();
    let hash1 = calculate_hash(&commit);

    let mut branch = Branch::init(&commit, "branch_name");
    vcs.add_branch(&branch);

    fs::File::create(path.join("new_file.txt"));
    let commit1 = Commit::init(path, "my_commit", 
                                Some(hash1), 
                                 false);
    assert!(commit1.is_ok());
    let commit1 = commit1.unwrap();
    let hash2 = calculate_hash(&commit1);
 
    vcs.get_mut_current_branch().add_commit(&commit1);
    assert_eq!(vcs.get_current_branch().get_first_commit().clone(), commit);
    vcs.add_commit(&commit);
    assert_eq!(vcs.get_current_branch().get_first_commit().clone(), commit);
    assert_eq!(vcs.get_current_branch().get_last_commit().clone(), commit1);

    vcs.delete_branch("branch_name");

    let path = Path::new("./tests/test_data/repo1");
    let path = path.join(".vcs").join("objects").join(hash1.to_string());

    assert!(!path.exists());

    let path = Path::new("./tests/test_data/repo1");
    let path = path.join(".vcs").join("objects").join(hash2.to_string());

    assert!(!path.exists());

    fs::remove_dir_all(Path::new("./tests/test_data/repo1/.vcs"));
    fs::remove_file(Path::new("./tests/test_data/repo1/add_file.txt"));    
    fs::remove_file(Path::new("./tests/test_data/repo1/new_file.txt"));    

}