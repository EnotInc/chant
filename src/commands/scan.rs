use std::{ops::Add, path::Path};

use ignore::WalkBuilder;
use crate::{config, commands::general, storage, parser, color};

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
                    if ft.is_dir() && config.files.ignore.contains(&file_name.to_string()) {
                        continue;
                    } else if !ft.is_dir() && !config.files.ignore.contains(&file_name.to_string()) && let Some(ext) = e.path().extension() {
                        if config.files.read.contains(&ext.display().to_string()){
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

pub fn scan_hollow(todo: bool, note: bool, fixme: bool) {
    let config = config::new_config();

    let any = !todo && !note && !fixme;

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
                if let Some(ft) = e.file_type(){
                    if ft.is_dir() && config.files.ignore.contains(&file_name.to_string()) {
                        continue;
                    } else if !ft.is_dir() && let Some(ext) = e.path().extension() {
                        if config.files.read.contains(&ext.display().to_string()){
                            let mut file = storage::new_file(path.to_string());
                            file = parser::parse_file(&file, true);

                            if !file.comments.is_empty() {
                                let mut file_data: String = String::new();


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
                    }
                }
            },
            Err(e) => println!("unable to scan: {}", e),
        }
    }
    println!();
}

pub fn scan() {
    if !general::is_initialized(){
        general::init_first();
        return; 
    }

    let config = config::read_config();
    let storage = storage::load_storage();
    let tasks = storage.tasks;
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
                    if ft.is_dir() && config.files.ignore.contains(&file_name.to_string()) {
                        continue;
                    } else if !ft.is_dir() && let Some(ext) = e.path().extension() {
                        if !config.files.ignore.contains(&file_name.to_string()) &&  config.files.read.contains(&ext.display().to_string()){
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