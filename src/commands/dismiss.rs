use std::fs;

use crate::{color, commands::general};

pub fn dismiss() {
    if !general::is_initialized() {
        general::init_first();
        return; 
    }
    let _ = fs::remove_dir_all("./.chant");
    remove_from_gitignore();
    let chant = color::paint_str("Chant".to_string(), color::Color::Cyan);
    println!("{chant} was removed");
}

fn remove_from_gitignore() {
    if general::is_gitignore_exists() {
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