use std::{env, fs};

use crate::{commands::general, services::{color, global}};

pub fn dismiss() {
    if !general::is_initialized() {
        general::init_first();
        return; 
    }
    let _ = fs::remove_dir_all("./.chant");
    remove_from_gitignore();

    if let Ok(cur_dir) = env::current_dir(){
        if let Some (dir) = cur_dir.file_name() {
            global::remove_project(&dir.to_string_lossy().to_string());
        }
    }
    let chant = color::paint_str("Chant".to_string(), color::Color::Cyan);
    println!("{chant} was removed");
}

/// About remove_from_gitignore()
/// It simply read `.gitignore` file line by line and creating a new buffer, ignoring line with `.chant` in it
/// after that I use this buffer to rewrite `.gitignore`
/// This may cause some trouble, `.gitignore` could not work, and you have to resave it manuanly
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
                        ignore.push_str("\n"); 
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