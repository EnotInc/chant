use std::fs;
use regex::Regex;

use crate::{services, storage};

/// About: constants
/// |COMMENT_PATTERN| - used to find TODO, NOTE and FIXME comments
const COMMENT_PATTERN: &str = r".*//.?\s?(TODO|NOTE|FIXME)[:\s]*(.*)";

pub fn parse_file(file: &storage::File, is_new_file: bool) -> storage::File {
    let content = fs::read_to_string(file.path.to_string());
    match  content {
        Ok(v) => {
            let mut new_file = storage::new_file(file.path.to_string());
            if !is_new_file && new_file.hash == file.hash {
                new_file = file.clone();
            } else {
                let lines = v.split("\n");
                let mut index: i32 = 0;
                for line in lines {
                    parse_line(file, line, index, &mut new_file);
                    index += 1;
                }
            }
            return new_file;
        },
        Err(_) => {  return file.clone(); }
    }
}

/// About parse_line()
/// used to parse 1 line
/// if line is matched [COMMENT_PATTERN], it called [parse_comment()]
/// if line is matched [ABOUT_PATTERN], it called [parse_about()]
fn parse_line(file: &storage::File, line: &str, index: i32, new_file: &mut storage::File) {
    let re_coms = Regex::new(COMMENT_PATTERN).unwrap();

    if re_coms.is_match(line) {
        parse_comment(re_coms, file, line, index, new_file);
    }
}

/// About: |parse_comment()|
/// uset to find TODO, NOTE and FIXME comments
/// it's eather creates a new comment, and insert it, or returns an old one 
fn parse_comment(re: Regex, file: &storage::File, line: &str, index: i32, new_file: &mut storage::File) {
    if let Some(captures) = re.captures(line) {
        let new_line_hash = services::hash::get_hash(&line.to_string());
        if file.comments.contains_key(&new_line_hash) {
            let a = &file.comments[&new_line_hash];
            new_file.comments.insert(new_line_hash,a.clone());
        } else {
            let c = storage::new_comment(captures[1].to_string(), captures[2].to_string(), index, new_line_hash);
            new_file.comments.insert(new_line_hash, c);
        }
    }
}
