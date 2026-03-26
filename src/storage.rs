use std::{fs, path::Path};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{services::hash, services::color};

/// About storage
/// main data sctructure. Contains 2 maps: files, and tasks
#[derive(Debug, Deserialize, Serialize)]
pub struct Storage {
    pub files: HashMap<String, File>,
    pub tasks: HashMap<String, Task>,
}

pub fn new_storage() -> Storage {
    return Storage { files: HashMap::new() , tasks: HashMap::new()};
}


/// About About
/// Here is how 'about' blocks saved
/// About have vecor of lines, and an index of 'about' header
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct About {
    pub lines: Vec<AboutLine>,
    pub index: i32,
}

/// About AboutLine
/// header - which is used to 'render' line with "About" in it differently (in markdoun file)
/// text - just a text. It's contains either content in line, or a header text (without "About" part)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct  AboutLine {
    pub header: String,
    pub text: String,
}

pub fn new_about(index: i32) -> About {
    return About{lines: Vec::new(), index};
}

pub fn new_about_line(header: String, text: String) -> AboutLine{
    return AboutLine{header, text};
}

/// About Task
/// Used to save local task
/// `id` - used to work with taks ('chant done <id>' and 'chant remove <id>'). It just a first 6 digits of hash
/// `text` - just a content
/// `done` - is it done or not
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

/// About File
/// Created for each file, that scanner checks. It contains all of necessary data for chant to work with
/// `hash` - used to chech if file changed or not
/// `path` - path from root of repo to the file
/// `dir` - relative path to directory with file
/// `comments` - map of found comments (todo, note and fixme). It used hash as key, so I can change if line is changed or not
/// `abouts` - vector of 'about' blocks
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct File {
    pub hash: u64,
    pub path: String,
    pub dir: String,
    pub comments: HashMap<u64, Comment>,
    pub abouts: Vec<About>,
}

pub fn new_file(path: String) -> File {
    let content = fs::read_to_string(&path);
    let dir = Path::new(&path).parent().unwrap().to_str().unwrap().to_string();
    let mut c = String::new();
    match content {
        Ok(v) => { c = v }
        Err(e) => { println!("Unable to open file at :{path}\n{e}"); }
    }
    return File{hash: hash::get_hash(&c), path: path, dir: dir, comments: HashMap::new(), abouts: Vec::new()}
}


/// About Comment
/// Contains a data about every comment
/// `id` - now used now
/// `kind` - TODO / NOTE / FIXME
/// `index` - line with commend 
/// `hash` - hash of the line
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Comment {
    pub id: String,
    pub kind: String,
    pub line: String,
    pub index: i32,
    pub hash: u64
}

pub fn new_comment(kind:String, line: String, index: i32, hash: u64) -> Comment {
    return Comment{kind: kind, line: line, index: index, hash: hash, id: get_id(hash)}
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

pub fn load_storage() -> Storage {
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