use walkdir::WalkDir;
use std::{fs, ops::Add};
use hf;

use crate::{color, storage, config, parser};

pub fn help() {
    let chant = color::paint_str("chant".to_string(), color::Color::Yellow);

    let init = color::paint_str("init".to_string(), color::Color::Yellow);
    let list = color::paint_str("list".to_string(), color::Color::Yellow);
    let scan = color::paint_str("scan".to_string(), color::Color::Yellow);
    let config = color::paint_str("config".to_string(), color::Color::Yellow);
    let dismiss = color::paint_str("dismiss".to_string(), color::Color::Yellow);
    let help = color::paint_str("help".to_string(), color::Color::Yellow);

    let chant_dir = color::paint_str(".chant/".to_string(), color::Color::Cyan);
    let config_dir = color::paint_str("config.toml".to_string(), color::Color::Cyan);
    let storage_dir = color::paint_str("storage.json".to_string(), color::Color::Cyan);
    let gitignore_dir = color::paint_str(".gitignore".to_string(), color::Color::Cyan);

    println!("List of alailable commands for {chant}:");
    println!("    {help} - list of alailable commands");
    println!("    {init} - creates {chant_dir} directory with {config_dir} and {storage_dir} files");
    println!("    {list} - prints a list of saved comments");
    println!("    {scan} - force chant to scan directory");
    println!("  {config} - print the current config");
    println!(" {dismiss} - remove {chant_dir} directory and remove it from {gitignore_dir}");
}

fn init_first() {
    let error = color::paint_str("Error:".to_string(), color::Color::Red);
    let chant_init = color::paint_str("chant init".to_string(), color::Color::Yellow);
    println!("{error} chant wasn't initialized. Run {chant_init} first");
}

pub fn init(){
    if !is_initialized(){
        let _ = fs::create_dir("./.chant");
        let _ = hf::hide("./.chant");
        let _ = fs::File::create("./.chant/storage.json");

        let _ = fs::File::create("./.chant/config.toml");
        config::create_config();
    } else {
        let error = color::paint_str("Error:".to_string(), color::Color::Red);
        let chant_dismiss = color::paint_str("chant dismiss".to_string(), color::Color::Yellow);
        let chant_help = color::paint_str("chant help".to_string(), color::Color::Yellow);
        println!("{error} Chant is already initialized.\nRun {chant_dismiss} remove chant from this directory, or {chant_help} to get more information");
        return;
    }

    add_to_gitignore();
    scan_force();

    let chant = color::paint_str("Chant".to_string(), color::Color::Cyan);
    println!("{chant} was initialized successfully");
}

fn is_initialized() -> bool {
    let ex = fs::exists("./.chant");
    match ex {
        Ok(v) => return v,
        Err(e) => {println!("{}", e); return false;}
    }
}

pub fn bad_syntax() {
    let error = color::paint_str("Error:".to_string(), color::Color::Red);
    let chant_help = color::paint_str("chant help".to_string(), color::Color::Yellow);
    println!("{error} bad syntax. Run {chant_help} to get more information");
}

pub fn unknown_command() {
    let error = color::paint_str("Error:".to_string(), color::Color::Red);
    let chant_help = color::paint_str("chant help".to_string(), color::Color::Yellow);
    println!("{error} unknown command. Run {chant_help} to get more information");
}

pub fn scan_force() {
    if !is_initialized(){
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
                if e.file_type().is_dir() && config.files.ignore.contains(&file_name.to_string()) {
                    it.skip_current_dir();
                    continue;
                } else if !config.files.ignore.contains(&file_name.to_string()) && !e.file_type().is_dir() && let Some(ext) = e.path().extension() {
                    if config.files.read.contains(&ext.display().to_string()){
                        let mut file = storage::new_file(path.to_string());
                        file = parser::parse_file(&file, false);
                        new_storage.files.insert(path.to_string(), file);
                    }
                }
            },
            Err(e) => println!("unable to scan: {}", e),
        }
    }
    storage::save_storage(&new_storage);
}

pub fn scan() {
    if !is_initialized(){
        init_first();
        return; 
    }

    let config = config::read_config();

    let storage = storage::load_storage();
    let mut new_storage = storage::new_storage();

    let mut it = WalkDir::new(".").into_iter();
    while let Some(entry) = it.next(){
        match entry {
            Ok(e) => {
                let file_name = e.file_name().to_string_lossy();
                let path = &e.path().to_string_lossy();
                if e.file_type().is_dir() && config.files.ignore.contains(&file_name.to_string()) {
                    it.skip_current_dir();
                    continue;
                } else if !e.file_type().is_dir() && let Some(ext) = e.path().extension() {
                    if !config.files.ignore.contains(&file_name.to_string()) &&  config.files.read.contains(&ext.display().to_string()){
                        let mut file: storage::File;
                        if storage.files.contains_key(&path.to_string()) {
                            file = storage.files[&path.to_string()].clone();
                        } else {
                            file = storage::new_file(path.to_string());
                        }

                        file = parser::parse_file(&file, true);
                        new_storage.files.insert(path.to_string(), file);
                    }
                }
            },
            Err(e) => println!("unable to scan: {}", e),
        }
    }
    storage::save_storage(&new_storage);
}

pub fn list() {
    if !is_initialized(){
        help();
        return; 
    }

    scan();

    let s = storage::load_storage();
    let mut is_empty: bool = true;
    if s.files.len() == 0 || s.files.is_empty() {
        nothing_was_found();
        return;
    }

    for file in s.files {
        if file.1.comments.len() > 0 {
            println!();
            let path = color::paint_str(file.1.path.to_string(), color::Color::Cyan);
            println!(" == {} ==", path);
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
            let index = color::paint_str(c.index.add(1).to_string(), color::Color::Cyan);

            println!(" {}: [{}] - {}", index, kind, c.line);
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
    if !is_initialized() {
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
    if !is_initialized() {
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