use std::path::PathBuf;
use std::{fs, path::Path};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{services::hash, services::color};

/// main data sctructure. Contains 2 maps: files, and tasks
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Storage {
    pub files: HashMap<String, File>,
    pub tasks: HashMap<String, Task>,
}

pub fn new_storage() -> Storage {
    return Storage {
        files: HashMap::new(),
        tasks: HashMap::new()
    };
}

pub struct Stats {
    pub todos: u8,
    pub notes: u8,
    pub fixmes: u8,

    pub backlog: u8,
    pub in_progress: u8,
    pub done: u8,
}

pub fn new_stats() -> Stats {
    return Stats {
        todos: 0, notes: 0, fixmes: 0,
        backlog: 0, in_progress: 0, done: 0,
    }
}

pub struct Summary {
    pub todos: u8,
    pub notes: u8,
    pub fixmes: u8,
}

pub fn new_summary() -> Summary {
    return Summary { todos: 0, notes: 0, fixmes: 0 }
}

pub fn update(s: &mut Summary, stats: &Stats) {
    s.todos += stats.todos;
    s.notes += stats.notes;
    s.fixmes += stats.fixmes;
}

#[derive(PartialEq, Debug, Deserialize, Serialize, Clone, Ord, PartialOrd, Eq)]
pub enum Status {
    Backlog,
    InProgress,
    Done,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    pub id: String,
    pub text: String,
    pub status: Status,
}

pub fn new_task(text: String ) -> Task {
    let hash = hash::get_hash(&text);
    return Task { text, status: Status::Backlog, id:get_id(hash) }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct File {
    pub hash: u64,
    pub path: String,
    pub dir: String,
    pub comments: HashMap<u64, Comment>,
}

pub fn new_file(path: String) -> File {
    let content = fs::read_to_string(&path);
    let dir = Path::new(&path).parent().unwrap().to_str().unwrap().to_string();
    let mut c = String::new();
    match content {
        Ok(v) => { c = v }
        Err(e) => { println!("Unable to open file at :{path}\n{e}"); }
    }
    return File { hash: hash::get_hash(&c), path: path, dir: dir, comments: HashMap::new() }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Comment {
    pub id: String,
    pub kind: String,
    pub line: String,
    pub index: i32,
    pub hash: u64
}

pub fn new_comment(kind:String, line: String, index: i32, hash: u64) -> Comment {
    return Comment { kind: kind, line: line, index: index, hash: hash, id: get_id(hash) }
}

fn get_id(hash: u64) -> String {
    return hash.to_string()[0..6].to_string();
}

pub const STORAGE_PATH: &str = "./.chant/storage.json";

pub fn save_storage(storage: &Storage) {
    let json = serde_json::to_string(storage);
    match json {
        Ok(v) => { let _ = fs::write(STORAGE_PATH, v); },
        Err(e) => {
            let error = color::paint_str("Error:".to_string(), color::Color::Red);
            println!("{error} unable to save storage\n{e}")
        }
    }
}

pub fn load_storage() -> Storage {
    let content = fs::read_to_string(STORAGE_PATH);
    match content {
        Ok(v) => {
            let res: Result<Storage, serde_json::Error >= serde_json::from_str(&v);
            match res {
                Ok(storage) => { return storage; },
                Err(_) => { return new_storage(); }
            }
        },
        Err(e) => { println!("{}", e); return new_storage() }
    }
}

pub fn load_storage_from(p: &str) -> Option<Storage> {
    let mut path = PathBuf::new();
    path.push(p);
    path.push(STORAGE_PATH);

    let content = fs::read_to_string(path);
    match content {
        Err(_) => { return None }
        Ok(v ) => {
            let res: Result<Storage, serde_json::Error >= serde_json::from_str(&v);
            match res {
                Ok(storage) => { return Some(storage); },
                Err(_) => { return None }
            }
        },
    }
}