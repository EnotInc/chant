use hf;
use std::fs;

use crate::{color, config, commands::general, commands::scan};

pub fn init(){
    if !general::is_initialized(){
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
    scan::scan_force();

    let chant = color::paint_str("Chant".to_string(), color::Color::Cyan);
    println!("{chant} was initialized successfully");
}

fn add_to_gitignore() {
    if general::is_gitignore_exists() {
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