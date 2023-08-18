#![forbid(unsafe_code)]

use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};


///
/// This is a struct for working with files inside the library. 
#[derive(Hash, Clone, Serialize, Deserialize, Debug, Default)]
pub struct File {
    name: String,
    data: Vec<u8>,
    path: PathBuf,
}

impl File {
        /// Creates a structure be reading the file along the path.
        /// 
        /// # Examples
        /// ```
        /// use vcs::linrary::files;
        /// let file = File::init("./new_file.txt"); 
        /// ```   
        pub fn init(path: &Path) -> Self {
            Self {
                name: match path.file_name() {
                    Some(x) => x.to_str().unwrap().to_string(),
                    None => {
                        panic!("Isn't a file");
                    }
                },
    
                data: match fs::read(path) {
                    Ok(v) => v,
                    Err(_e) => {
                        panic!("Cannot read the file");
                    }
                },
    
                path: path.to_path_buf(),
            }
        }

    /// Returns the file name.
    /// 
    /// # Examples
    ///```
    /// use vcs::linrary::files;
    /// let file = File::init("./new_file.txt");
    /// assert_eq!(file.get_name(), "new_file.txt");
    ///```
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the path of the file at the time it was read from the device. 
    /// 
    /// # Examples
    ///```
    /// use vcs::linrary::files;
    /// let file = File::init("./new_file.txt");
    /// assert_eq!(file.get_path(), Path::new("./new_file.txt"));
    ///```
    pub fn get_path(&self) -> &Path {
        &self.path
    }

    /// Checks tha twofiles have equal paths and filenames.
    /// 
    /// # Examples
    ///```
    /// use vcs::linrary::files;
    /// let file1 = File::init("./new_file.txt");
    /// let file2 = File::init("./new_file.txt");
    /// assert!(file1.is_change_only_data(file2));
    ///```
    pub fn is_change_only_data(&self, other: &Self) -> bool {
        if self.name != other.name {
            return false;
        }
        if self.path != other.path {
            return false;
        }
        true
    }

    /// Returns an immutable reference to the file data. 
    /// 
    /// # Examples
    ///```
    /// use vcs::linrary::files;
    /// let file = File::init("./new_file.txt");
    /// let empty_vec: Vec<u8> = vec![];
    /// assert_eq!(file.get_data().clone(), empty_vec);
    ///```
    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    /// Changes the data in the struct to the foreward data.
    /// 
    /// # Examples
    /// ```
    /// use vcs::linrary::files;
    /// let mut file = File::init("./new_file.txt");
    /// let new_data: Vec<u8> = vec![97];
    /// file.change_data(new_data);
    /// assert_eq!(file.get_data().clone(), new_data);
    ///```
    pub fn change_data(&mut self, new_data: &Vec<u8>) {
        self.data = new_data.clone();
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name {
            return false;
        }
        if self.path != other.path {
            return false;
        }
        if self.data.len() != other.data.len() {
            return false;
        }
        for i in 0..self.data.len() {
            if self.data[i] != other.data[i] {
                return false;
            }
        }
        true
    }
}
