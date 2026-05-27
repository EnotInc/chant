use crate::services::config;

use std::env;

pub fn projects() {
    config::list_projects();
}

pub fn add_this() {
    if let Ok(cur_dir) = env::current_dir(){
        config::add_project(&cur_dir.to_string_lossy().to_string());
        return
    }
    println!("Unable to add this directory as a chant project\nYou still can use chant")
}

pub fn remove_this() {
    if let Ok(cur_dir) = env::current_dir(){
        config::remove_project(&cur_dir.to_string_lossy().to_string());
        return
    }
    println!("Something went wrong with removing this project out of global scope")
}