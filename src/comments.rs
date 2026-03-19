use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Storage {
    pub comments: HashMap<String, Comment>
}

pub fn new_storage() -> Storage {
    return Storage { comments: HashMap::new() };
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Comment {
    pub file: String,
    pub kind: String,
    pub line: String,
    pub index: i32
}

pub fn new_comment(file: String, kind:String, line: String, index: i32) -> Comment {
    return Comment{ file: file, kind: kind, line: line, index: index }
}