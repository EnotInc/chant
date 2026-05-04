use std::{env::home_dir, fs};

const CONFIG_DIR: &str = ".chant_config";
const LEDGER: &str = "ledger";

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
            Err(e) => { println!("{}", e); }
        }

        let pro_ex = fs::exists(&path);
        match pro_ex {
            Ok(v) => {
                if !v {
                    path.push(LEDGER);
                    let _ = fs::File::create(path);
                }
            },
            Err(e) => { println!("{}", e); }
        }
    }
}

pub fn add_project(name: &str) {
    create_global_dir();
    if let Some(mut path) = home_dir() {
        path.push(CONFIG_DIR);
        path.push(LEDGER);

        let content = fs::read_to_string(&path);
        match content {
            Ok(v) => {
                let projects = format!("{}\n{}", v, name);
                let _ = fs::write(path, projects);
            },
            Err(_) => { return }
        }
    }
}

pub fn remove_project(name: &str) {
    create_global_dir();
    if let Some(mut path) = home_dir() {
        path.push(CONFIG_DIR);
        path.push(LEDGER);

        let content = fs::read_to_string(&path);
        match content {
            Ok(v) => {
                let lines = v.split("\n");
                let mut projects = "".to_owned();
                for line in lines {
                    if line != name {
                        projects.push_str(line); 
                        projects.push_str("\n"); 
                    }
                }
                let _ = fs::write(&path, projects);
            },
            Err(e) => { println!("{}", e); }
        }
    }
}

pub fn list_projects() {
    if let Some(mut path) = home_dir() {
        path.push(CONFIG_DIR);
        path.push(LEDGER);

        let content = fs::read_to_string(&path);
        match content {
            Ok(v) => {
                let lines = v.split("\n");
                println!("All projects with chant:");
                for line in lines {
                    if !line.is_empty(){
                        println!(" - {}", line)
                    }
                }
            },
            Err(e) => { println!("{}", e); }
        }
    }
}