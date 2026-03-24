use std::fs;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{hash, color};

#[derive(Debug, Deserialize, Serialize)]
pub struct Storage {
    pub files: HashMap<String, File>,
    pub tasks: HashMap<String, Task>,
}

pub fn new_storage() -> Storage {
    return Storage { files: HashMap::new() , tasks: HashMap::new()};
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    pub id: String,
    pub text: String,
    pub done: bool,
}

pub fn new_task(text: String, ) -> Task {
    let hash = hash::get_hash(&text);
    return Task{text, done: false, id:get_id(hash)}
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct File {
    pub hash: u64,
    pub path: String,
    pub comments: HashMap<u64, Comment>
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
    pub id: String,
    pub kind: String,
    pub line: String,
    pub code: String,
    pub index: i32,
    pub hash: u64
}

pub fn new_comment(kind:String, line: String, index: i32, hash: u64) -> Comment {
    return Comment{kind: kind, line: line, index: index, hash: hash, code: String::new(), id: get_id(hash)}
}

fn get_id(hash: u64) -> String {
    return hash.to_string()[0..6].to_string();
}

pub fn save_storage(storage: &Storage) {
    let json = serde_json::to_string(storage);
    match json {
        Ok(v) => { let _ = fs::write("./.chant/storage.json", v); },
        Err(e) => {
            let error = color::paint_str("Error:".to_string(), color::Color::Red);
            println!("{error} unable to save storage\n{e}")
        }
    }
}

pub fn load_storage() -> Storage{
    let content = fs::read_to_string("./.chant/storage.json");
    match content {
        Ok(v) => {
            let res: Result<Storage, serde_json::Error >= serde_json::from_str(&v);
            match res {
                Ok(sorage) => {
                    return sorage;
                },
                Err(_) => { return new_storage(); }
            }
        },
        Err(e) => { println!("{}", e); return new_storage() }
    }
}