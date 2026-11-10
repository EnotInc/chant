use std::fs::{self};
use regex::Regex;

use crate::{services, storage::{self, new_comment}};

/// |COMMENT_PATTERN| - used to find TODO, NOTE and FIXME comments
const COMMENT_PATTERN: &str = r".*//.?\s?(TODO|NOTE|FIXME)[:\s]*(.*)";

pub fn parse_file(file: &storage::File, is_new_file: bool) -> storage::File {
    let content = fs::read_to_string(file.path.to_string());
    match  content {
        Err(_) => {  return file.clone(); }
        Ok(data) => {
            let mut new_file = storage::new_file(file.path.to_string());
            if !is_new_file && new_file.hash == file.hash {
                new_file = file.clone();
            } else {
                let lines: Vec<&str> = data.split("\n").collect();

                let mut index: i32 = 0;
                for line in lines.clone() {
                    let re_coms = Regex::new(COMMENT_PATTERN).unwrap();

                    if re_coms.is_match(line) {
                        let keep = parse_comment(re_coms, file, line, index, &mut new_file);
                        if let Some(new_line_hash) = keep {

                            let mut _text: String = String::new();

                            let next_line_content = lines.get((index+1) as usize);
                            match next_line_content {
                                None => { _text = "...".to_string(); }
                                Some(text) => { _text = format!("{}", text.trim()); }
                            }

                            let _kind = new_file.comments[&new_line_hash].kind.to_owned();
                            let next_comm = new_comment(_kind, _text, index as i32, new_line_hash, true);
                            new_file.comments.insert(new_line_hash, next_comm);
                        }
                    }
                    index += 1;
                }
            }
            return new_file;
        }
    }
}

/// used to find TODO, NOTE and FIXME comments
/// it's eather creates a new comment, and insert it, or returns an old one 
fn parse_comment(re: Regex, file: &storage::File, line: &str, index: i32, new_file: &mut storage::File) -> Option<u64>{
    if let Some(captures) = re.captures(line) {
        let new_line_hash = services::hash::get_hash(&line.to_string());
        if file.comments.contains_key(&new_line_hash) {
            let mut new_comm = file.comments[&new_line_hash].clone();
            new_comm.index = index;
            new_file.comments.insert(new_line_hash,new_comm);
        } else {
            let c = storage::new_comment(captures[1].to_string(), captures[2].to_string(), index, new_line_hash, false);
            new_file.comments.insert(new_line_hash, c);
        }

        if captures[2].trim().is_empty() {
            return Some(new_line_hash);
        }
    }

    return None;
}