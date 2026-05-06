use std::{env::home_dir, fs, path::Path};

use crate::services::color;

const CONFIG_DIR: &str = ".chant_config";
const PROJECTS: &str = "projects";

fn create_global_dir() {
    if let Some(mut path) = home_dir() {

        path.push(CONFIG_DIR);
        let dir_ex = fs::exists(&path);
        match dir_ex {
            Ok(v) => {
                if !v {
                    let _ = fs::create_dir(&path);
                }
            }
            Err(e) => { println!("Error with direcotry:\n{} {}", e, path.to_string_lossy()); }
        }

        let pro_ex = fs::exists(&path);
        match pro_ex {
            Ok(v) => {
                if !v {
                    path.push(PROJECTS);
                    let _ = fs::File::create(path);
                }
            },
            Err(e) => { println!("Error with file:\n{} {}", e, path.to_string_lossy()); }
        }
    }
}

pub fn add_project(full_path: &str) {
    create_global_dir();
    if let Some(mut path) = home_dir() {
        path.push(CONFIG_DIR);
        path.push(PROJECTS);

        let content = fs::read_to_string(&path);
        match content {
            Ok(v) => {
                let projects = format!("{}\n{}", v, full_path);
                let _ = fs::write(path, projects);
            },
            Err(e) => { println!("Cannot read file:\n{}", e); }
        }
    }
}

pub fn remove_project(full_path: &str) {
    create_global_dir();
    if let Some(mut path) = home_dir() {
        path.push(CONFIG_DIR);
        path.push(PROJECTS);

        let content = fs::read_to_string(&path);
        match content {
            Ok(v) => {
                let lines = v.split("\n");
                let mut projects = "".to_owned();
                for line in lines {
                    if line != full_path {
                        projects.push_str(line); 
                        projects.push_str("\n"); 
                    }
                }
                let _ = fs::write(&path, projects);
            },
            Err(e) => { println!("Cannot read file:\n{}", e); }
        }
    }
}

pub fn list_projects() {
    if let Some(mut path) = home_dir() {
        path.push(CONFIG_DIR);
        path.push(PROJECTS);

        let content = fs::read_to_string(&path);
        match content {
            Ok(v) => {
                let lines = v.split("\n");
                println!("Projects:");
                for line in lines {
                    if !line.is_empty(){
                        let name = color::paint_str(get_name(&line), color::Color::Cyan);
                        let mut status = color::paint_str(String::from("x"), color::Color::Red);

                        if is_initialized(&line) {
                            status = color::paint_str(String::from("v"), color::Color::Green);
                        }

                        println!(" {} [{}] {}", status, name, line)
                    }
                }
            },
            Err(e) => { println!("Cannot read file:\n{}", e); }
        }
    }
}

fn get_name(full_path: &str) -> String {
    let p = Path::new(full_path);
    if let Some(name) = p.file_name() {
        return name.to_string_lossy().to_string();
    }
    return String::from(full_path)
}

fn is_initialized(full_path: &str) -> bool {
    let path = Path::new(full_path).join(".chant");
    let ex = fs::exists(path);
    match ex {
        Ok(v) => return v,
        Err(_) => { return false;}
    }
}