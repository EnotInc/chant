use std::fs;
use regex::Regex;

use crate::{services, storage};

/// About: constants
/// |COMMENT_PATTERN| - used to find TODO, NOTE and FIXME comments
/// |ABOUT_PATTERN| - used to find 'About' line
/// |DETAILS_PATTERN| - used to separate comment symbols (//) and text
/// |OBJECT_PATTERN| - used to find object (variables, and func names) in line
const COMMENT_PATTERN: &str = r".*//.?\s?(TODO|NOTE|FIXME)[:\s]*(.*)";
const ABOUT_PATTERN: &str = r"[!\*\?]//\s?(About)[:\s]*(.*)";
const DETAILS_PATTERN: &str = r"(\s?[\/]*)\s(.*)";
const OBJECT_PATTERN: &str = r"(\|[a-zA-Z0-9_\(\)]*\|)";

pub fn parse_file(s: &mut storage::Storage, file: &storage::File, is_new_file: bool, hollow: bool) -> storage::File {
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
                    about = parse_line(s, file, line, index, &mut new_file, &mut about, hollow);
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
fn parse_line(s: &mut storage::Storage, file: &storage::File, line: &str, index: i32, new_file: &mut storage::File, about: &mut Option<storage::About>, hollow: bool) -> Option<storage::About> {
    let re_coms = Regex::new(COMMENT_PATTERN).unwrap();
    let re_about = Regex::new(ABOUT_PATTERN).unwrap();

    if re_coms.is_match(line) {
        parse_comment(re_coms, file, line, index, new_file);
        return None;
    } else if re_about.is_match(line) && !hollow {
        let mut a  = storage::new_about(index);
        parse_about(s, re_about, line, index, &mut a, new_file);
        return Some(a);
    } else if !hollow {
        match about.as_mut() {
            Some(v) => {
                if line.starts_with("//") {
                    parse_about(s, re_about, line, index, v, new_file);
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
    return None;
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

// TODO: Add add conflict if 2 objects with similar name is declared

/// About: |parse_about()|
/// used to find 'about' blocks in the file to save them
/// return storage::About
fn parse_about(s: &mut storage::Storage, re: Regex, line: &str, index: i32, about: &mut storage::About, file: &mut storage::File) {
    // TODO: figure out how to save hash of the 'about' block
    if let Some(captures) = re.captures(line) {
        let about_line = format!("{} {}", captures[1].to_string(), captures[2].to_string());
        about.index = index;
        about.header = about_line;
    } else {
        let re_det = Regex::new(DETAILS_PATTERN).unwrap();
        if let Some(captures) = re_det.captures(line) {
            let about_line = captures[2].to_string();
            about.lines.push(about_line);
        } else {
            let about_line = line.to_string();
            about.lines.push(about_line);
        }
    }
    let re_obj = Regex::new(OBJECT_PATTERN).unwrap();
    if let Some(obj) = re_obj.captures(line) {
        let mut c_obj = obj[0].chars();
        c_obj.next();
        c_obj.next_back();
        let link =  c_obj.as_str().to_string();
        let path = format!("{} {}", file.path.clone(), index);
        s.objects.insert(link, path);
    }
}