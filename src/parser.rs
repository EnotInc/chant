use std::fs;
use regex::Regex;

use crate::{comments, hash};

pub fn parse_new_file(path: String) -> comments::File{

    //TODO: move it to global scope or enum
    let re = Regex::new(r"//\s*(TODO|NOTE|FIXME)[:\s]*(.*)").unwrap();

    let content = fs::read_to_string(&path);
    match content {
        Ok(v) => {
            let mut new_file = comments::new_file(path, &v);
            let lines = v.split("\n");
            let mut index: i32 = 0;
            for line in lines {
                index += 1;
                if let Some(captures) = re.captures(line) {
                    let new_line_hash = hash::get_hash(&line.to_string());
                    let c = comments::new_comment(captures[1].to_string(), captures[2].to_string(), index, new_line_hash);
                    new_file.comments.insert(new_line_hash.to_string(), c.clone());
                }
            }

            return new_file;
        },
        Err(_) => {
            return comments::new_file(path, &String::new());
        }
    }
}

pub fn parse_file(old_file: &comments::File) -> comments::File{
    let re = Regex::new(r"//\s*(TODO|NOTE|FIXME)[:\s]*(.*)").unwrap();

    let content = fs::read_to_string(old_file.path.to_string());
    match  content {
        Ok(v) => {
            let mut new_file = comments::new_file(old_file.path.to_string(),&v);
            if new_file.hash == old_file.hash {
                new_file = old_file.clone();
            } else {
            let lines = v.split("\n");
                let mut index: i32 = 0;
                for line in lines {
                    index += 1;
                    if let Some(captures) = re.captures(line) {
                        let new_line_hash = hash::get_hash(&line.to_string());
                        if old_file.comments.contains_key(&new_line_hash.to_string()) {
                            let a = &old_file.comments[&new_line_hash.to_string()];
                            new_file.comments.insert(new_line_hash.to_string(),a.clone());
                        } else {
                            let c = comments::new_comment(captures[1].to_string(), captures[2].to_string(), index, new_line_hash);
                            new_file.comments.insert(new_line_hash.to_string(), c);
                        }
                    }
                }
            }

            return new_file;
        },
        Err(_) => {
            return old_file.clone();
        }
    }
}