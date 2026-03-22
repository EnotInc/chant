use std::fs;
use regex::Regex;

use crate::{hash, storage};

const COMMENT_PATTERN: &str = r".*//\s?(TODO|NOTE|FIXME)[:\s]*(.*)";

pub fn parse_file(file: &storage::File, compare_hash: bool, with_code: bool) -> storage::File{
    let content = fs::read_to_string(file.path.to_string());
    match  content {
        Ok(v) => {
            let mut new_file = storage::new_file(file.path.to_string());
            if new_file.hash == file.hash && compare_hash {
                new_file = file.clone();
            } else {
                let lines = v.split("\n");
                let mut index: i32 = 0;
                let mut keep: bool = false;
                for line in lines {
                    index += 1;
                    if keep && with_code {
                            if !line.trim().starts_with("//") {
                                save_code(&mut new_file, line, index);
                                keep = false;
                                continue;
                            }
                    }
                    keep = find_comment_with_regex(file, line, index, &mut new_file, with_code)
                }
            }
            return new_file;
        },
        Err(_) => {  return file.clone(); }
    }
}

fn find_comment_with_regex(file: &storage::File, line: &str, index: i32 ,new_file: &mut storage::File, with_code: bool) -> bool {
    let re = Regex::new(COMMENT_PATTERN).unwrap();
    if let Some(captures) = re.captures(line) {
        let new_line_hash = hash::get_hash(&line.to_string());
        if file.comments.contains_key(&new_line_hash) {
            let a = &file.comments[&new_line_hash];
            new_file.comments.insert(new_line_hash,a.clone());
        } else {
            let c = storage::new_comment(captures[1].to_string(), captures[2].to_string(), index, new_line_hash);
            new_file.comments.insert(new_line_hash, c);
        }
        return true && with_code;
    }
    return false;
}

fn save_code(new_file: &mut storage::File, line: &str, index: i32) {
    let comment = new_file.comments.iter_mut().find(|(_, value)| value.index == index-1);
    match comment {
        Some(c) => {
            c.1.code = line.trim().to_string();
        },
        None => {}
    }
}