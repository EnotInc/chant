use std::fs;
use regex::Regex;

use crate::comments;

pub fn parse_file(path: &str) -> Vec<comments::Comment>{
    let re = Regex::new(r"//\s*(TODO|NOTE|FIXME)[:\s]*(.*)").unwrap();

    let content = fs::read_to_string(path);
    let mut index = 0;

    let mut cmds: Vec<comments::Comment> = Vec::new();

    match content {
        Ok(v) => {
            let lines = v.split("\n");
            index += 1;
            for line in lines {
                if let Some(captures) = re.captures(line) {
                    let c = comments::new_comment(path.to_string(), captures[1].to_string(), captures[2].to_string(), index);
                    cmds.push(c);
                    //println!("{} {}:{} - {}", c.kind, c.file ,c.index, c.line)
                }
            }
        },
        Err(e) => {println!("{}", e); return cmds;}
    }

    return cmds;
}