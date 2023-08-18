#[cfg(test)]

mod tests {

use std::{path::Path, io::Write};
use vcs::commands::*;
use std::fs;
use vcs::utils::operation_hash::calculate_hash;

#[test]
fn test_init() {
    let path = Path::new("./tests/test_data_init");
    let vcs = init::init(path);
    assert!(vcs.is_ok());
    let path_vcs = path.join(".vcs");
    assert!(path_vcs.exists());

    let vcs = init::init(path);
    assert!(vcs.is_err());

    fs::remove_dir_all(Path::new("./tests/test_data_init"));
}

#[test]
fn test_new_commit() {
    let path = Path::new("./tests/test_data_commit");
    let mut vcs = init::init(path).unwrap();

    assert!(new_commit::new_commit(&mut vcs, "-1").is_err());

    for ind in 0..100 {
        fs::File::create(path.join(ind.to_string()));
        assert!(new_commit::new_commit(&mut vcs, &ind.to_string()).is_ok());
    }    

    fs::remove_dir_all(Path::new("./tests/test_data_commit"));
}

#[test]
fn test_new_branch() {
    let path = Path::new("./tests/test_data_branch");
    let mut vcs = init::init(path).unwrap();

    assert!(new_branch::new_branch(&mut vcs, "master").is_err());
    assert!(new_branch::new_branch(&mut vcs, "new-branch").is_ok());
    assert!(new_branch::new_branch(&mut vcs, "not-new-branch").is_err());

    for ind in 0..100 {
        let branch = vcs.get_branch_by_name("master").unwrap().clone();
        vcs.change_current_branch(&branch).unwrap();
        assert!(new_branch::new_branch(&mut vcs, &ind.to_string()).is_ok());
    }    

    fs::remove_dir_all(Path::new("./tests/test_data_branch"));
}

#[test]
fn test_jump_to_commit() {
    let path = Path::new("./tests/test_data_jump_commit");
    let mut hashes: Vec<u64> = vec![];
    let mut vcs = init::init(path).unwrap();

    hashes.push(calculate_hash(vcs.get_current_commit()));

    assert!(new_commit::new_commit(&mut vcs, "-1").is_err());
    hashes.push(calculate_hash(vcs.get_current_commit()));

    for ind in 0..20 {
        fs::File::create(path.join(ind.to_string()));
        assert!(new_commit::new_commit(&mut vcs, &ind.to_string()).is_ok());
        hashes.push(calculate_hash(vcs.get_current_commit()));
    }    

    new_branch::new_branch(&mut vcs, "new-branch");

    for ind in 20..40 {
        fs::File::create(path.join(ind.to_string()));
        assert!(new_commit::new_commit(&mut vcs, &ind.to_string()).is_ok());
        hashes.push(calculate_hash(vcs.get_current_commit()));
    }   

    let mut x = 239 % hashes.len();
    for _ in 0..50 {
        let x = (x * 41 + 65) %  hashes.len();
        assert!(jump_to_commit::jump_to_commit(&mut vcs, hashes[x]).is_ok());
    } 

    assert!(jump_to_commit::jump_to_commit(&mut vcs, hashes.last().unwrap().clone()).is_ok());
    fs::File::create(path.join("-2"));

    assert!(jump_to_commit::jump_to_commit(&mut vcs, hashes[0]).is_err());
    assert!(new_commit::new_commit(&mut vcs, &"-2".to_string()).is_ok());

    for k in 0..hashes.len() + 1 {
        let mut _flag = false;
        for ind in  0..hashes.len() {
            if hashes[ind] == k as u64 {
                _flag = true;
                break;
            }
        }
        if !_flag {
            assert!(jump_to_commit::jump_to_commit(&mut vcs, k as u64).is_err());
        }
    }

    fs::remove_dir_all(Path::new("./tests/test_data_jump_commit"));
}

#[test]
fn test_jump_to_branch() {
    let path = Path::new("./tests/test_data_jump_branch");
    let mut vcs = init::init(path).unwrap();

    assert!(jump_to_branch::jump_to_branch(&mut vcs, "-1").is_err());

    fs::remove_dir_all(Path::new("./tests/test_data_jump_branch"));
}

#[test]
fn test_merge() {
    let path = Path::new("./tests/test_data_merge");
    fs::File::create(path.join("deleted-file.txt"));
    fs::File::create(path.join("modified-file.txt"));
    let mut vcs = init::init(path).unwrap();

    assert!(merge::merge(&mut vcs, "just-branch").is_err());

    assert!(new_branch::new_branch(&mut vcs, "new-branch").is_ok());
    assert!(merge::merge(&mut vcs, "new-branch").is_err());
    
    let mut output = fs::File::create(path.join("modified-file.txt")).unwrap();
    let data: Vec<u8> = vec![97, 97, 97];
    output.write_all(&data).unwrap();
    fs::File::create(path.join("added-file.txt"));
    fs::remove_file(path.join("deleted-file.txt"));

    assert!(new_commit::new_commit(&mut vcs, "1").is_ok());
    let master = vcs.get_branch_by_name("master").unwrap().clone();
    vcs.change_current_branch(&master);
    
    assert!(merge::merge(&mut vcs, "new-branch").is_err());

    let branch = vcs.get_branch_by_name("new-branch").unwrap().clone();
    vcs.change_current_branch(&branch);
 
    let path_mod = path.join("modified-file.txt");
    fs::remove_file(path_mod);
    fs::File::create(path.join("modified-file.txt"));

    assert!(new_commit::new_commit(&mut vcs, "2").is_ok());
    let master = vcs.get_branch_by_name("new-branch").unwrap().clone();
    vcs.change_current_branch(&master);
    
    assert!(merge::merge(&mut vcs, "new-branch").is_err());

    let branch = vcs.get_branch_by_name("new-branch").unwrap().clone();
    vcs.change_current_branch(&branch);

    let mut output = fs::File::create(path.join("modified-file.txt")).unwrap();
    let data: Vec<u8> = vec![97, 97, 97];
    output.write_all(&data).unwrap();
    fs::File::create(path.join("deleted-file.txt"));

    assert!(new_commit::new_commit(&mut vcs, "3").is_ok());
    let master = vcs.get_branch_by_name("master").unwrap().clone();
    vcs.change_current_branch(&master);
    
    assert!(merge::merge(&mut vcs, "new-branch").is_err());

    let branch = vcs.get_branch_by_name("new-branch").unwrap().clone();
    vcs.change_current_branch(&branch);

    let path_mod = path.join("modified-file.txt");
    fs::remove_file(path_mod);
    fs::File::create(path.join("modified-file.txt"));

    assert!(new_commit::new_commit(&mut vcs, "4").is_ok());
    let master = vcs.get_branch_by_name("master").unwrap().clone();
    vcs.change_current_branch(&master);
    
    assert!(merge::merge(&mut vcs, "new-branch").is_ok());

    assert!(vcs.get_branch_by_name("new-branch").is_err());

    fs::remove_dir_all(Path::new("./tests/test_data_merge"));
}

}