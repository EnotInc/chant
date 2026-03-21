use std::fs;
use regex::Regex;

use crate::{storage, hash};

const NOTE_PATTERN: &str = r".*\/\/\s*(TODO|NOTE|FIXME)[:\s]*(.*)";

pub fn parse_file(file: &storage::File) -> storage::File{
    let content = fs::read_to_string(file.path.to_string());
    match  content {
        Ok(v) => {
            let mut new_file = storage::new_file(file.path.to_string());
            if new_file.hash == file.hash {
                new_file = file.clone();
            } else {
            let re = Regex::new(NOTE_PATTERN).unwrap();
            let lines = v.split("\n");
                let mut index: i32 = 0;
                for line in lines {
                    index += 1;
                    if let Some(captures) = re.captures(line) {
                        let new_line_hash = hash::get_hash(&line.to_string());
                        if file.comments.contains_key(&new_line_hash.to_string()) {
                            let a = &file.comments[&new_line_hash.to_string()];
                            new_file.comments.insert(new_line_hash.to_string(),a.clone());
                        } else {
                            let c = storage::new_comment(captures[1].to_string(), captures[2].to_string(), index, new_line_hash);
                            new_file.comments.insert(new_line_hash.to_string(), c);
                        }
                    }
                }
            }
            return new_file;
        },
        Err(_) => { return file.clone(); }
    }
}