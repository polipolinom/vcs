#![forbid(unsafe_code)]

mod commands;
mod library;
mod utils;

use crate::utils::comand_parser::{Command, CommandParser};
use crate::utils::extract_files::read_vcs;
use crate::utils::operation_hash::calculate_hash;
use crate::utils::pack_files::pack_vcs;
use crate::utils::print_files::print_uncommitted_files;
use clap::Parser;
use path_absolutize::*;
use std::env::current_dir;
use std::path::{Path, PathBuf};

fn get_dir_with_vcs() -> Result<PathBuf, &'static str> {
    let mut dir = current_dir().unwrap();
    while !dir.join(".vcs").exists() {
        if !dir.pop() {
            return Err("No VCS in this project");
        }
    }
    Ok(dir)
}

fn call_init(str_path: &str) {
    let path_absolute = Path::new(&str_path).absolutize().unwrap();
    let path = path_absolute.to_str().unwrap();
    let vcs = match commands::init::init(&Path::new(path)) {
        Ok(new_vcs) => new_vcs,
        Err(error) => {
            if error == "is not a dir" {
                println!("{} isn't a existing directory", str_path);
                return;
            }
            if error == "already exists" {
                println!("VCS already exists on path {}", str_path);
                return;
            }
            println!("{}", error);
            return;
        }
    };

    pack_vcs(&Path::new(&str_path), &vcs);
    println!("Initialized VCS repository in {}", str_path);
    println!("Created commit:");
    println!(
        "[master {}] Initial commit",
        calculate_hash(vcs.get_current_commit())
    );
}

fn call_status() {
    let dir = match get_dir_with_vcs() {
        Ok(dir) => dir,
        Err(str_err) => {
            println!("{}", str_err);
            return;
        }
    };
    let vcs = match read_vcs(dir) {
        Ok(vcs) => vcs,
        Err(str_error) => {
            println!("{}", str_error);
            return;
        }
    };
    match commands::status::status(&vcs) {
        Err(str_err) => {
            println!("{}", str_err);
        }
        _ => {}
    }
}

fn call_jump_to_commit(commit_hash: u64) {
    let dir = match get_dir_with_vcs() {
        Ok(dir) => dir,
        Err(str_err) => {
            println!("{}", str_err);
            return;
        }
    };
    let mut vcs = match read_vcs(dir) {
        Ok(vcs) => vcs,
        Err(str_error) => {
            println!("{}", str_error);
            return;
        }
    };
    match commands::jump_to_commit::jump_to_commit(&mut vcs, commit_hash) {
        Err(str_err) => {
            if str_err == "uncommited files" {
                println!("error: Your local changes to the following files should be commited or dropped:");
                print_uncommitted_files(&vcs).unwrap();
                println!("Please commit your changes or drop them before you jump.");
                println!("Aborting...");
                return;
            }
            if str_err == "No commit with this hash" {
                println!("No commit with hash {} exists.", commit_hash);
                println!("Aborting...");
                return;
            }
            println!("{}", str_err);
        }
        Ok(_) => {
            let branch_name = vcs.get_current_branch().get_name();
            pack_vcs(vcs.get_dir(), &vcs);
            println!(
                "Successfully jumped to commit {}. Current branch: {}.",
                commit_hash, branch_name
            );
        }
    }
}

fn call_new_commit(msg: &str) {
    let dir = match get_dir_with_vcs() {
        Ok(dir) => dir,
        Err(str_err) => {
            println!("{}", str_err);
            return;
        }
    };
    let mut vcs = match read_vcs(dir) {
        Ok(vcs) => vcs,
        Err(str_error) => {
            println!("{}", str_error);
            return;
        }
    };
    match commands::new_commit::new_commit(&mut vcs, msg) {
        Ok(_) => {
            pack_vcs(vcs.get_dir(), &vcs);
        }
        Err(str_err) => {
            if str_err == "Current commit not last" {
                println!("You can create a new commit only from last one.");
                println!("Aborting...");
                return;
            }
            if str_err == "No changes" {
                println!("No changes to be committed");
                return;
            }
            println!("{}", str_err);
        }
    }
}

fn call_jump_to_branch(branch_name: &str) {
    let dir = match get_dir_with_vcs() {
        Ok(dir) => dir,
        Err(str_err) => {
            println!("{}", str_err);
            return;
        }
    };
    let mut vcs = match read_vcs(dir) {
        Ok(vcs) => vcs,
        Err(str_error) => {
            println!("{}", str_error);
            return;
        }
    };
    match commands::jump_to_branch::jump_to_branch(&mut vcs, branch_name) {
        Err(str_err) => {
            if str_err == "uncommited files" {
                println!("error: Your local changes to the following files should be commited or dropped:");
                print_uncommitted_files(&vcs).unwrap();
                println!("Please commit your changes or drop them before you jump.");
                println!("Aborting...");
                return;
            }
            if str_err == "No branch with this name" {
                println!("No branch {} exists", branch_name);
                println!("Aborting...");
                return;
            }
            println!("{}", str_err);
        }
        Ok(_) => {
            let hash_commit = calculate_hash(vcs.get_current_commit());
            pack_vcs(vcs.get_dir(), &vcs);
            println!(
                "Successfully jumped to branch {}. Current commit: {}.",
                branch_name, hash_commit
            );
        }
    }
}

fn call_log() {
    let dir = match get_dir_with_vcs() {
        Ok(dir) => dir,
        Err(str_err) => {
            println!("{}", str_err);
            return;
        }
    };
    let vcs = match read_vcs(dir) {
        Ok(vcs) => vcs,
        Err(str_error) => {
            println!("{}", str_error);
            return;
        }
    };
    match commands::log::log(&vcs) {
        Err(str_err) => {
            println!("{}", str_err);
        }
        _ => {}
    }
}

fn call_new_branch(branch_name: &str) {
    let dir = match get_dir_with_vcs() {
        Ok(dir) => dir,
        Err(str_err) => {
            println!("{}", str_err);
            return;
        }
    };
    let mut vcs = match read_vcs(dir) {
        Ok(vcs) => vcs,
        Err(str_error) => {
            println!("{}", str_error);
            return;
        }
    };
    match commands::new_branch::new_branch(&mut vcs, branch_name) {
        Err(str_err) => {
            if str_err == "Current branch is not master" {
                println!(
                    "Creating a new branch is possible only when you are in the master branch."
                );
                println!("Aborting...");
                return;
            }
            if str_err == "Branch exists" {
                println!("Branch {} already exists.", branch_name);
                println!("Aborting...");
                return;
            }
            println!("{}", str_err);
        }
        Ok(hash_commit) => {
            pack_vcs(vcs.get_dir(), &vcs);
            println!(
                "Created a new branch {} from master's commit {}",
                branch_name, hash_commit
            );
        }
    }
}

fn call_merge(branch_name: &str) {
    let dir = match get_dir_with_vcs() {
        Ok(dir) => dir,
        Err(str_err) => {
            println!("{}", str_err);
            return;
        }
    };
    let mut vcs = match read_vcs(dir) {
        Ok(vcs) => vcs,
        Err(str_error) => {
            println!("{}", str_error);
            return;
        }
    };
    match commands::merge::merge(&mut vcs, branch_name) {
        Ok(_) => {
            pack_vcs(vcs.get_dir(), &vcs);
        }
        Err(str_err) => {
            if str_err == "Current commit no master last commit" {
                println!("The merge is possible only when you are in the last commit in master.");
                println!("Aborting...");
                return;
            }
            if str_err == "uncommited files" {
                println!("error: Your local changes to the following files should be commited or dropped:");
                print_uncommitted_files(&vcs).unwrap();
                println!("Please commit your changes or drop them before you merge.");
                println!("Aborting...");
                return;
            }
            if str_err == "Aborting" {
                println!("Aborting...");
                return;
            }
            if str_err == "No branch with this name" {
                println!("No branch {} exists.", branch_name);
                println!("Aborting...");
                return;
            }
            println!("{}", str_err);
        }
    }
}

fn main() {
    match CommandParser::parse().command {
        Command::Init { path } => {
            call_init(&path);
        }
        Command::Status => {
            call_status();
        }
        Command::Jump { commit, branch } => {
            if commit.is_some() {
                let commit_hash = match commit.clone().unwrap().parse::<u64>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("the {} is not a valid hash", commit.unwrap());
                        return;
                    }
                };
                call_jump_to_commit(commit_hash);
            } else {
                call_jump_to_branch(&branch.unwrap());
            }
        }
        Command::Commit { message } => {
            call_new_commit(&message);
        }
        Command::Log {} => {
            call_log();
        }
        Command::NewBranch { name } => {
            call_new_branch(&name);
        }
        Command::Merge { branch } => {
            call_merge(&branch);
        }
    }
}
