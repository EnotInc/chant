use walkdir::WalkDir;
use std::{fs, ops::Add};
use hf;

use crate::{color, storage, config, parser};

fn init_first() {
    let error = color::paint_str("Error:".to_string(), color::Color::Red);
    let chant_init = color::paint_str("$chant init".to_string(), color::Color::Yellow);
    println!("{error} chant wasn't initialised. Run {chant_init}");
}

pub fn init(){
    if !is_initialised(){
        let _ = fs::create_dir("./.chant");
        let _ = hf::hide("./.chant");
        let _ = fs::File::create("./.chant/storage.json");

        let _ = fs::File::create("./.chant/config.toml");
        config::create_config();
    } else {
        let error = color::paint_str("Error:".to_string(), color::Color::Red);
        let chant_dismiss = color::paint_str("$chant dismiss".to_string(), color::Color::Yellow);
        println!("{error} Chant is already initialised.\nRun {chant_dismiss} remove chant from this directory");
        return;
    }

    add_to_gitignore();
    scan_force();

    let chant = color::paint_str("Chant".to_string(), color::Color::Cyan);
    println!("{chant} was initialised successfully");
}

fn is_initialised() -> bool {
    let ex = fs::exists("./.chant");
    match ex {
        Ok(v) => return v,
        Err(e) => {println!("{}", e); return false;}
    }
}

pub fn bad_syntax() {
    let error = color::paint_str("Error:".to_string(), color::Color::Red);
    println!("{error} bad syntax");
}

pub fn unknown_command() {
    let error = color::paint_str("Error:".to_string(), color::Color::Red);
    println!("{error} unknown command");
}

pub fn scan_force() {
    if !is_initialised(){
        init_first();
        return; 
    }

    let config = config::read_config();

    let mut new_storage = storage::new_storage();

    let mut it = WalkDir::new(".").into_iter();
    while let Some(entry) = it.next() {
        match entry {
            Ok(e) => {
                let file_name = e.file_name().to_string_lossy();
                let path = &e.path().to_string_lossy();
                if e.file_type().is_dir() && config.ignore.contains(&file_name.to_string()) {
                    it.skip_current_dir();
                    continue;
                } else if !config.ignore.contains(&file_name.to_string()) && !e.file_type().is_dir() && let Some(ext) = e.path().extension() {
                    if config.read.contains(&ext.display().to_string()){
                        let mut file = storage::new_file(path.to_string());
                        file = parser::parse_file(&file, false);
                        new_storage.files.insert(path.to_string(), file);
                    }
                }
            },
            Err(e) => println!("unable to scan: {}", e),
        }
    }
    save_storage(&new_storage);
}

pub fn scan() {
    if !is_initialised(){
        init_first();
        return; 
    }

    let config = config::read_config();

    let storage = load_storage();
    let mut new_storage = storage::new_storage();

    let mut it = WalkDir::new(".").into_iter();
    while let Some(entry) = it.next(){
        match entry {
            Ok(e) => {
                let file_name = e.file_name().to_string_lossy();
                let path = &e.path().to_string_lossy();
                if e.file_type().is_dir() && config.ignore.contains(&file_name.to_string()) {
                    it.skip_current_dir();
                    continue;
                } else if !e.file_type().is_dir() && let Some(ext) = e.path().extension() {
                    if !config.ignore.contains(&file_name.to_string()) &&  config.read.contains(&ext.display().to_string()){
                        let mut file: storage::File;
                        if storage.files.contains_key(&path.to_string()) {
                            file = storage.files[&path.to_string()].clone();
                        } else {
                            file = storage::new_file(path.to_string());
                        }

                        file = parser::parse_file(&file, false);
                        new_storage.files.insert(path.to_string(), file);
                    }
                }
            },
            Err(e) => println!("unable to scan: {}", e),
        }
    }
    save_storage(&new_storage);
}

pub fn list() {
    if !is_initialised(){
        init_first();
        return; 
    }

    scan();

    let cfg = config::read_config();

    let s = load_storage();
    let mut is_empty: bool = true;
    if s.files.len() == 0 || s.files.is_empty() {
        nothing_was_found();
        return;
    }

    for file in s.files {
        if file.1.comments.len() > 0 {
            if cfg.with_code {
                let file = color::paint_str(file.0, color::Color::Cyan);
                print!("\n== {} == ", file);
            }
            println!();
        }

        let mut coms: Vec<_> = file.1.comments.iter().collect();
        coms.sort_by(|a, b| a.1.index.cmp(&b.1.index));

        for (_, comment) in coms {
            is_empty = false;
            let c: storage::Comment = comment.clone();

            let mut kind_color = color::Color::Yellow;
            match c.kind.as_str() {
                "TODO" => kind_color = color::Color::Blue,
                "NOTE" => kind_color = color::Color::Green,
                "FIXME" => kind_color = color::Color::Red,
                _ => {}
            }
            let kind = color::paint_str(c.kind, kind_color);
            let path = color::paint_str(file.1.path.to_string(), color::Color::Cyan);
            let index = color::paint_str(c.index.add(1).to_string(), color::Color::Cyan);

            if cfg.with_code {
                println!("[{}] - {}", kind, c.line);
                println!("{}: {}\n", index, c.code);
            } else {
                println!("[{}] {}:{} - {}", kind, path, index, c.line);
            }
        }
    }
    if is_empty {
        nothing_was_found();
        return;
    }
    println!();
}

fn nothing_was_found() {
    let nothing = color::paint_str("Nothing".to_string(), color::Color::Yellow);
    println!("{nothing} was found\n");
}

pub fn print_config() {
    if !is_initialised() {
        init_first();
        return;
    }
    let content = fs::read_to_string("./.chant/config.toml");
    match content {
        Ok(v) => {
            let path = color::paint_str("./.chant/config.toml".to_string(), color::Color::Cyan);
            println!("\n{path}:");
            println!("{}",v);
        },
        Err(_) => { print!("Config wasn't found"); }
    }
}

pub fn dismiss() {
    if !is_initialised() {
        init_first();
        return; 
    }
    let _ = fs::remove_dir_all("./.chant");
    remove_from_gitignore();
    let chant = color::paint_str("Chant".to_string(), color::Color::Cyan);
    println!("{chant} was removed");
}

fn is_gitignore_exists() -> bool {
    let ex = fs::exists("./.gitignore");
    match ex {
        Ok(v) => return v,
        Err(e) => {println!("{}", e); return false;}
    }
}

fn add_to_gitignore() {
    if is_gitignore_exists() {
        let content = fs::read_to_string(".gitignore");
        match content {
            Ok(v) => {
                let ignore = format!("{}\n.chant", v);
            let _ = fs::write(".gitignore", ignore);
        },
            Err(_) => { return }
        }
    } else {
        let warn = color::paint_str("Warning:".to_string(), color::Color::Yellow);
        println!("{warn} .gitignore is not found")
    }
}

fn remove_from_gitignore() {
    if is_gitignore_exists() {
        let content = fs::read_to_string(".gitignore");
        match content {
            Ok(v) => {
                let lines = v.split("\n");
                let mut ignore = "".to_owned();
                for line in lines {
                    if line != ".chant" {
                        ignore.push_str(line); 
                    }
                }
                let _ = fs::write(".gitignore", ignore);
            },
            Err(e) => {
                let error = color::paint_str("Error:".to_string(), color::Color::Red);
                println!("{error} unable to remove .chant/ direcotry from .gitignore. You should do this manually\n{e}");
                return
            }
        }
    } else {
        let warn = color::paint_str("Warning:".to_string(), color::Color::Yellow);
        println!("{warn} .gitignore is not found")
    }
}

fn save_storage(storage: &storage::Storage) {
    let json = serde_json::to_string(storage);
    match json {
        Ok(v) => { let _ = fs::write("./.chant/storage.json", v); },
        Err(e) => {
            let error = color::paint_str("Error:".to_string(), color::Color::Red);
            println!("{error} unable to save storage\n{e}")
        }
    }
}

fn load_storage() -> storage::Storage{
    let content = fs::read_to_string("./.chant/storage.json");
    match content {
        Ok(v) => {
            let res: Result<storage::Storage, serde_json::Error >= serde_json::from_str(&v);
            match res {
                Ok(sorage) => {
                    return sorage;
                },
                Err(_) => { return storage::new_storage(); }
            }
        },
        Err(e) => { println!("{}", e); return storage::new_storage() }
    }
}