use std::{collections::HashMap, fs, path};
use ignore::WalkBuilder;
use regex::Regex;
use crate::{commands::{general, scan}, services::{color, config, link::{self, create_obj_link}}, storage};

const LINK_PATTERN: &str = r"(\[[a-zA-Z0-9_\(\)]*\])";

/// About: |list()|
/// This func is displaying all found 'about'
/// Not rly useful, you better call `chant about -s` to save output to a file
pub fn list() {
    if !general::is_initialized(){
        general::init_first();
        return; 
    }

    scan::scan();

    let s = storage::load_storage();
    if s.files.len() == 0 || s.files.is_empty() {
        general::nothing_was_found();
        return;
    }

    for file in s.files {
        if file.1.abouts.is_empty() {
            continue;
        }

        let mut abouts: Vec<_> = file.1.abouts.iter().collect();
        abouts.sort_by(|a, b| a.index.cmp(&b.index));

        let path = color::paint_str(file.1.path.to_string(), color::Color::Cyan);
        println!("\n == {} ==", path);
        for about in abouts {
            if !about.lines.is_empty() {
                let spaces = " ".repeat(about.index.to_string().len()-1);
                let index = color::paint_str(about.index.to_string(), color::Color::Cyan);
                let border = color::paint_str("|".to_string(), color::Color::Cyan);

                let split = about.header.split_once(" ");
                match split {
                    Some(v) => {
                        let header = color::paint_str(v.0.to_string(), color::Color::Blue);
                        let topic= color::paint_str(v.1.to_string(), color::Color::Yellow);
                        println!(" {} {} {}", index , header, topic);
                    }
                    None => { }
                }
                for line in &about.lines {
                        println!(" {}{} {}",spaces, border, line);
                }
            }
            println!();
        }
    }
}

// About |save()|
// This functions is used to save all found 'About' blocks into files
// By default it creates files in every directory, with name "about.md"
// This can be changed by providing different name with flagh -s
// For example: `chant about -s readme.md` -> now all 'About' blocks will be saved to 'readme.md'
pub fn save(output: Option<String>) {
    if !general::is_initialized(){
        general::init_first();
        return; 
    }

    scan::scan();
    let mut s = storage::load_storage();
    if s.files.is_empty() {
        general::nothing_was_found();
        return;
    }

    let mut cfg = config::read_config();
    remove_old_about_files(&cfg.about.output);

    let o: String;
    match output {
        Some(v) => {
            let ext = path::Path::new(&v).extension();
            let e: String;
            match ext {
                Some(_) => { e = "".to_string(); }
                None => { e = ".md".to_string(); }
            }
            o = format!("{}{}", v, e);
            if cfg.about.output != v {
                cfg.about.output = v;
                config::save_config(cfg);
            }
        }
        None => { o = cfg.about.output; }
    }

    let tmp = &s.clone();
    let mut files: Vec<_> = tmp.files.iter().collect();
    files.sort_by(|a, b| a.0.cmp(&b.0));

    let dir_to_about = create_about_structures(&mut s, files, &o);

    save_abouts(dir_to_about);
    println!();
}

/// About |remove_old_about_files()|
/// I need this to rewrite new 'about' blocks
/// Maybe i'll add some sort of check, to not rewrite file if nothing was changed in about's. Idk, we'll see
/// Now this is fine, ig
fn remove_old_about_files(output: &str) {
    let home_path = path::Path::new(".");
    let walker = WalkBuilder::new(home_path)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .hidden(false)
        .build();

    for entry in walker {
        match entry {
            Ok(e) => {
                if let Some(ft) = e.file_type() {
                    if !ft.is_dir() || ft.is_file() {
                        continue;
                    } else {
                        let p = path::Path::new(&e.into_path()).join(&output);
                        let res = fs::exists(&p);
                        match res {
                            Ok(v) => {
                                if v { let _ =  fs::remove_file(p.as_path()); }
                            }
                            Err(_) => { }
                        }
                    }
                }
            },
            Err(_) => { continue; }
        }
    }
}


/// About |create_about_structures()|
/// This func is used to create hash map dir to 'file content'
/// both represented as strings
fn create_about_structures(s: &mut storage::Storage, files: Vec<(&String, &storage::File)>, output: &str) -> HashMap<String, String> {
    let mut dir_to_about: HashMap<String, String> = HashMap::new();
    for file in files {
        if file.1.abouts.is_empty() {
            continue;
        }

        let cur_dir = path::Path::new(&file.1.dir).join(output).to_str().unwrap().to_string();
        if !dir_to_about.contains_key(&cur_dir) {
            let dir_header = format!("# **{}**\n", &file.1.dir);
            dir_to_about.insert(cur_dir.clone(), dir_header);
        }

        let md_file = dir_to_about.get_mut(&cur_dir).unwrap();
        let file_header = format!("\n## **== {} ==**\n", &file.1.path.trim());
        md_file.push_str(&file_header);

        for about in &file.1.abouts {
            let split = about.header.split_once(" ");
            match split {
                Some(v) => {
                    let topic = link::create_obj_link(v.1.trim(), about.index, &file.1.path.trim(), None);
                    md_file.push_str(&format!("\n#### *{}* {}\n", v.0.trim(), topic));
                }
                None => {}
            }

            let mut i = 0;
            for line in &about.lines {
                i += 1;
                let data = connect_links(s, line.trim(), &cur_dir);
                md_file.push_str(&data);
                if i != about.lines.len() {
                    if !line.contains("```") {
                        md_file.push_str("\\");
                    }
                    md_file.push_str("\n");
                }
            }
            md_file.push_str("\n\n---\n");
        }
    }

    return dir_to_about;
}

/// About |save_abouts()|
/// used builded `dir_to_about` hash_map to create new files
fn save_abouts(dir_to_about: HashMap<String, String>) {
    if dir_to_about.is_empty() {
        general::nothing_was_found();
        return;
    }
    for dir in dir_to_about {
        let about_path = path::Path::new(&dir.0);
        let write = fs::write(about_path, &dir.1);
        match write {
            Ok(_) => {
                let filename = color::paint_str(about_path.to_string_lossy().to_string(), color::Color::Cyan);
                let complete = color::paint_str("complete".to_string(), color::Color::Green);
                println!(" File {} is {}!", filename, complete);
            }
            Err(e) => {
                let error = color::paint_str("Error:".to_string(), color::Color::Red);
                let filename = color::paint_str(about_path.to_string_lossy().to_string(), color::Color::Cyan);
                println!(" {} unable to create file {}\n{}", error, filename, e);
            }
        }
    }
}

/// About |connect_links()|
/// used to replace found objects, with markdown links
fn connect_links(s: &mut storage::Storage, line: &str, dir: &str) -> String{
    let re_link = Regex::new(LINK_PATTERN).unwrap();

    let res = re_link.replace_all(line, |caps: &regex::Captures| {
        let mut c_obj = caps[0].chars();
        let mut path = caps[0].to_string();
        let mut index: i32 = 0;
        c_obj.next();
        c_obj.next_back();
        if s.objects.contains_key(c_obj.as_str()) {
            let p:Vec<&str>  = s.objects.get(c_obj.as_str()).unwrap().split_whitespace().collect();
            let first = p.get(0);
            let second = p.get(1);

            match first {
                Some( v) => { path = v.to_string(); }
                None => {}
            }
            match second {
                Some( v) => { index = v.parse().unwrap(); }
                None => {}
            }
        }
        let link = create_obj_link(&caps[0], index, dir, Some(path));
        return link;
    });
    return res.to_string();
}