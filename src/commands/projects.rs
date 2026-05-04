use crate::services::global;

use std::env;

pub fn projects() {
    global::list_projects();
}

pub fn add_this() {
    if let Ok(cur_dir) = env::current_dir(){
        if let Some (dir) = cur_dir.file_name() {
            global::add_project(&dir.to_string_lossy().to_string());
            return
        }
    }
    println!("Unable to add this directory as a chant project\nYou still can use chant")
}

pub fn remove_this() {
    if let Ok(cur_dir) = env::current_dir(){
        if let Some (dir) = cur_dir.file_name() {
            global::remove_project(&dir.to_string_lossy().to_string());
            return
        }
    }
    println!("Something went wrong with removing this project out of global scope")
}