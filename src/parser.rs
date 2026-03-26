use std::fs;
use regex::Regex;

use crate::{services::hash, storage};

/// About: constants
/// [COMMENT_PATTERN] - used to find TODO, NOTE and FIXME comments
/// [ABOUT_PATTERN] - used to find 'About' line
/// [DETAILS_PATTERN] - used to separate comment symbols (//) and text
const COMMENT_PATTERN: &str = r".*//\s?(TODO|NOTE|FIXME)[:\s]*(.*)";
const ABOUT_PATTERN: &str = r".*//\s?(About)[:\s]*(.*)";
const DETAILS_PATTERN: &str = r"\s*([\/]*)\s(.*)";

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
                let mut about: Option<storage::About> = None;
                for line in lines {
                    index += 1;
                    about = parse_line(file, line, index, &mut new_file, &mut about);
                }
            }
            return new_file;
        },
        Err(_) => {  return file.clone(); }
    }
}

/// About parse_line()
/// used to parse 1 line
/// if line is matched [COMMENT_PATTERN], it called [parse_comment]
/// if line is matched [ABOUT_PATTERN], it called [parse_about]
fn parse_line(file: &storage::File, line: &str, index: i32, new_file: &mut storage::File, about: &mut Option<storage::About>) -> Option<storage::About> {
    let re_coms = Regex::new(COMMENT_PATTERN).unwrap();
    let re_about = Regex::new(ABOUT_PATTERN).unwrap();

    if re_coms.is_match(line) {
        parse_comment(re_coms, file, line, index, new_file);
        return None;
    } else if re_about.is_match(line) {
        let mut a  = storage::new_about(index);
        parse_about(re_about, line, index, &mut a);
        return Some(a);
    } else {
        match about.as_mut() {
            Some(v) => {
                if line.starts_with("//") {
                    parse_about(re_about, line, index, v);
                    return Some(v.clone());
                }
                else {
                    new_file.abouts.push(v.clone());
                    return None;
                }
            }
            None => { return None; }
        }
    }
}

/// About: parse_comment()
/// uset to find TODO, NOTE and FIXME comments
/// it's eather creates a new comment, and insert it, or returns an old one 
fn parse_comment(re: Regex, file: &storage::File, line: &str, index: i32, new_file: &mut storage::File) {
    if let Some(captures) = re.captures(line) {
        let new_line_hash = hash::get_hash(&line.to_string());
        if file.comments.contains_key(&new_line_hash) {
            let a = &file.comments[&new_line_hash];
            new_file.comments.insert(new_line_hash,a.clone());
        } else {
            let c = storage::new_comment(captures[1].to_string(), captures[2].to_string(), index, new_line_hash);
            new_file.comments.insert(new_line_hash, c);
        }
    }
}

/// About: parse_about()
/// used to find 'about' blocks in the file
/// return storage::About
fn parse_about(re: Regex, line: &str, index: i32, about: &mut storage::About) {
    // TODO: figure out how to save hash of the 'about' block
    if let Some(captures) = re.captures(line) {
        let about_line = storage::new_about_line(captures[1].to_string(), captures[2].to_string());
        about.index = index;
        about.lines.push(about_line);
    } else {
        let re_det = Regex::new(DETAILS_PATTERN).unwrap();
        if let Some(captures) = re_det.captures(line) {
            let about_line = storage::new_about_line("".to_string(), captures[2].to_string());
            about.lines.push(about_line);
        } else {
            let about_line = storage::new_about_line("".to_string(), line.to_string());
            about.lines.push(about_line);
        }
    }
}
