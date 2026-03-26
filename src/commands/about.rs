use std::{collections::HashMap, fs, path};
use ignore::WalkBuilder;
use crate::{commands::{general, scan}, services::{config, color}, storage};

/// About: list()
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

                let mut first: bool = true;
                for line in &about.lines {
                    if first {
                        let header = color::paint_str(line.header.to_string(), color::Color::Blue);
                        let topic = color::paint_str(line.text.to_string(), color::Color::Yellow);
                        println!(" {} {} {}", index , header, topic);
                        first = false;
                    } else {
                        println!(" {}{} {}",spaces, border, line.text);
                    }
                }
                println!();
            }
        }
    }
}

// About save(output: String)
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
    let s = storage::load_storage();
    if s.files.is_empty() {
        general::nothing_was_found();
        return;
    }

    let mut cfg = config::read_config();
    remove_old_about_files(&cfg.about.output);

    let o: String;
    match output {
        Some(v) => {
            o = v.clone();
            if cfg.about.output != v {
                cfg.about.output = v;
                config::save_config(cfg);
            }
        }
        None => { o = cfg.about.output; }
    }

    let mut files: Vec<_> = s.files.iter().collect();
    files.sort_by(|a, b| a.0.cmp(&b.0));
    let dir_to_about = create_about_structures(files, &o);

    save_abouts(dir_to_about);
}

/// About remove_old_about_files
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


/// About create_about_structures
/// This func is used to create hash map 'dir' to 'file content'
/// both represented as strings
fn create_about_structures(files: Vec<(&String, &storage::File)>, output: &str) -> HashMap<String, String> {
    let mut dir_to_about: HashMap<String, String> = HashMap::new();
    for file in files {
        if file.1.abouts.is_empty() {
            continue;
        }
        let cur_dir = path::Path::new(&file.1.dir).join(output).to_str().unwrap().to_string();
        if !dir_to_about.contains_key(&cur_dir) {
            dir_to_about.insert(cur_dir.clone(), String::new());
        }
        let md_file = dir_to_about.get_mut(&cur_dir).unwrap();
        let file_header = format!("\n# == {} ==\n", &file.1.path.trim());
        md_file.push_str(&file_header);

        for about in &file.1.abouts {
            let mut i = 0;
            for line in &about.lines {
                i += 1;
                if !line.header.is_empty() {
                    let topic = create_topic_link(&line.text.trim(), about.index, &file.1.path.trim());
                    md_file.push_str(&format!("\n### *{}* {}\n", line.header, topic));
                } else {
                    md_file.push_str(&line.text.trim());
                    if i != about.lines.len() {
                        md_file.push_str("\\\n");
                    }
                }
            }
            md_file.push_str("\n\n---\n");
        }
    }

    return dir_to_about;
}

/// About create_topic_link()
/// just a small helper func, to create markdown link to each 'about' blocks in the source code
fn create_topic_link(topic: &str, index: i32, path: &str) -> String {
    let filename = path::Path::new(path).file_name().unwrap().to_string_lossy();
    let link = format!("[{}]({}#L{})", topic, filename, index);
    return link;
}

/// About save_abouts()
/// used builded `dir_to_about` hash_map to create new files
fn save_abouts(dir_to_about: HashMap<String, String>) {
    for dir in dir_to_about {
        let about_path = path::Path::new(&dir.0);
        let _ = fs::write(about_path, &dir.1);
    }
}