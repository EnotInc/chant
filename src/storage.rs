use std::fs;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::hash;

#[derive(Debug, Deserialize, Serialize)]
pub struct Storage {
    pub files: HashMap<String, File>
}

pub fn new_storage() -> Storage {
    return Storage { files: HashMap::new() };
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct File {
    pub hash: u64,
    pub path: String,
    pub comments: HashMap<String, Comment>
}

pub fn new_file(path: String) -> File {
    let content = fs::read_to_string(&path);
    let mut c = String::new();
    match content {
        Ok(v) => { c = v }
        Err(e) => { println!("Unable to open file at :{path}\n{e}"); }
    }
    return File{hash: hash::get_hash(&c), path: path, comments: HashMap::new()}
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Comment {
    pub kind: String,
    pub line: String,
    pub code: String,
    pub index: i32,
    pub hash: u64
}

pub fn new_comment(kind:String, line: String, index: i32, hash: u64) -> Comment {
    return Comment{kind: kind, line: line, index: index, hash: hash, code: "code line parser isn't implemented yet".to_string()}
}
