use std::{ops::Add, path::Path};

use ignore::WalkBuilder;
use crate::{services::config, commands::general, storage, parser, services::color};

/// This functon scans, without hash checking
/// It used when you don't need to check hesh (specifically when `chant init` or `chant reset` is called), or when you need to rescan everything
/// Basicaly I just deleting old `.chant/storage` file, and creating a new one. But all tasks are saved 
pub fn scan_force() {
    if !general::is_initialized(){
        general::init_first();
        return; 
    }

    let config = config::read_config();
    let tasks = storage::load_storage().tasks;
    let mut new_storage = storage::new_storage();
    new_storage.tasks = tasks;

    let home_path = Path::new(".");
    let walker = WalkBuilder::new(home_path)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .hidden(false).build();

    for entry in walker {
        match entry {
            Ok(e) => {
                let file_name = e.file_name().to_string_lossy();
                let path = &e.path().to_string_lossy();
                if let Some(ft) = e.file_type() {
                    if ft.is_dir() && config.scanner.ignore.contains(&file_name.to_string()) {
                        continue;
                    } else if !ft.is_dir() && !config.scanner.ignore.contains(&file_name.to_string()) && let Some(ext) = e.path().extension() {
                        if config.scanner.read.contains(&ext.display().to_string()){
                            let mut file = storage::new_file(path.to_string());
                            file = parser::parse_file(&file, true);
                            new_storage.files.insert(path.to_string(), file);
                        }
                    }
                }
            },
            Err(e) => println!("unable to scan: {}", e),
        }
    }
    storage::save_storage(&new_storage);
}

/// This function is kinda similar to [scan_force()], but it doesn't require chant to be initialized (it didn't use `.chant/storage.json` file or config)
/// After each file, it prings out found comments. And it uses a default config 
pub fn scan_hollow(todo: bool, note: bool, fixme: bool) {
    let config = config::new_config();

    let any = !todo && !note && !fixme;

    let home_path = Path::new(".");
    let walker = WalkBuilder::new(home_path)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .hidden(false).build();

    let mut temp_files: Vec<storage::File> = Vec::new();
    for entry in walker {
        match entry {
            Ok(e) => {
                let file_name = e.file_name().to_string_lossy();
                let path = &e.path().to_string_lossy();
                if let Some(ft) = e.file_type(){
                    if ft.is_dir() && config.scanner.ignore.contains(&file_name.to_string()) {
                        continue;
                    } else if !ft.is_dir() && let Some(ext) = e.path().extension() {
                        if config.scanner.read.contains(&ext.display().to_string()){
                            let mut file = storage::new_file(path.to_string());
                            file = parser::parse_file(&file, true);
                            temp_files.push(file);
                        }
                    }
                }
            },
            Err(e) => println!("unable to scan: {}", e),
        }
    }
    for file in temp_files {
        if !file.comments.is_empty() {
            let mut file_data: String = String::new();

            let mut coms: Vec<_> = file.comments.iter().collect();
            coms.sort_by(|a, b| a.1.index.cmp(&b.1.index));
            for com in file.comments {
                if !((com.1.kind == "TODO" && (any || todo)) ||
                        (com.1.kind == "NOTE" && (any || note)) ||
                        (com.1.kind == "FIXME" && (any || fixme))) { continue; }

                let mut kind_color = color::Color::Yellow;
                match com.1.kind.as_str() {
                    "TODO" => kind_color = color::Color::Blue,
                    "NOTE" => kind_color = color::Color::Green,
                    "FIXME" => kind_color = color::Color::Red,
                    _ => {}
                }
                //let id = color::paint_str(c.id, color::Color::Yellow);
                let kind = color::paint_str(com.1.kind, kind_color);
                let index = color::paint_str(com.1.index.add(1).to_string(), color::Color::Cyan);

                file_data += &format!(" {}: [{}] - {}\n", index, kind, com.1.line);
            }

            if !file_data.is_empty() {
                println!();
                let path = color::paint_str(file.path.to_string(), color::Color::Cyan);
                println!(" == {} ==", path);
                print!("{}", file_data);
            }
        }
    }
    println!();
}

/// This is the functoin that runs every time you trying to get list of comments or task
/// It have a hash check, so if file wasn't changed, instead of paring it all again, it just returns an old one
pub fn scan() {
    if !general::is_initialized(){
        general::init_first();
        return; 
    }

    let config = config::read_config();
    let storage = storage::load_storage();
    let mut new_storage = storage::new_storage();

    let tasks = storage.tasks;
    new_storage.tasks = tasks;


    let home_path = Path::new(".");
    let walker = WalkBuilder::new(home_path)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .hidden(false).build();

    for entry in walker {
        match entry {
            Ok(e) => {
                let file_name = e.file_name().to_string_lossy();
                let path = &e.path().to_string_lossy();

                if let Some(ft) = e.file_type() {
                    if ft.is_dir() && config.scanner.ignore.contains(&file_name.to_string()) {
                        continue;
                    } else if !ft.is_dir() && let Some(ext) = e.path().extension() {
                        if !config.scanner.ignore.contains(&file_name.to_string()) &&  config.scanner.read.contains(&ext.display().to_string()){
                            let mut file: storage::File;
                            let mut new = false;
                            if storage.files.contains_key(&path.to_string()) {
                                file = storage.files[&path.to_string()].clone();
                            } else {
                                file = storage::new_file(path.to_string());
                                new = true;
                            }

                            file = parser::parse_file(&file, new);
                            new_storage.files.insert(path.to_string(), file);
                        }
                    }
                }
           },
            Err(e) => println!("unable to scan: {}", e),
        }
    }
    storage::save_storage(&new_storage);
}