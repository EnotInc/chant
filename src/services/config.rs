use crate::{commands::general::nothing_was_found, services::{color, stats::{get_stats, print_summary, stats_to_string}}, storage::{self, new_summary, update}};
use serde::{Deserialize, Serialize};
use std::{env::home_dir, fs, path::{Path, PathBuf}};
use toml;

const CONFIG_DIR: &str = ".chant";
const PROJECTS: &str = "projects";
const CONFIG: &str = "config.toml";

/// Main struct for config
/// Includes [Scanner]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub scanner: Scanner,
}

/// `read` - vector of strings, stores a list for supperted file extatoins (without dot '.')
/// `ignore` - vector of string with ignored files / directories, such as `target/`, `.chant/`, `.git/` and so on
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Scanner {
    pub read: Vec<String>,
    pub ignore: Vec<String>,
}

/// used to (re)write a config file in `.chant/config.toml` file
/// called [new_config()] to get default [Config]
pub fn create_config() -> Config {
    let cfg= new_config();
    if let Some(mut path) = home_dir() {
        path.push(CONFIG_DIR);
        path.push(CONFIG);

        let cfg_toml = toml::to_string(&cfg);
        match cfg_toml {
            Ok(v) => { let _ = fs::write(path, &v); },
            Err(e) => { println!("Unable to create default config\n{e}") },
        }
    }
    return cfg;
}

pub fn save_config(cfg: Config) {
    if let Some(mut path) = home_dir() {
        path.push(CONFIG_DIR);
        path.push(CONFIG);

        let cfg_toml = toml::to_string(&cfg);
        match cfg_toml {
            Ok(v) => { let _ = fs::write(path, &v); },
            Err(e) => { println!("Unable to create default config\n{e}") },
        }
    }
}

/// creates a new default [Config]
pub fn new_config() -> Config {
    let default_read: Vec<String> = vec!["rs".to_string(), "go".to_string(), "js".to_string(), "ts".to_string(), "dart".to_string(), "jsx".to_string(), "tsx".to_string(), "c".to_string(), "cpp".to_string(), "h".to_string(), "hpp".to_string(), "java".to_string()];
    let default_ignore: Vec<String> = vec![".chant".to_string(), "target".to_string(), ".git".to_string(), "node_modules".to_string(), ".gitignore".to_string(), "vendor".to_string(), "build".to_string(), ".idea".to_string(), ".vscode".to_string()];
    let scanner: Scanner = Scanner { read: default_read, ignore: default_ignore };
    return Config { scanner: scanner }
}


fn save_default_config() -> Config {
    create_global_dir();
    create_project();

    let cfg = create_config();
    save_config(cfg.clone());
    return cfg;
}

pub fn read_config() -> Config {
    if let Some(mut path) = home_dir() {
        path.push(CONFIG_DIR);
        path.push(CONFIG);

        let content = fs::read_to_string(path);
        match content {
            Ok(v) => {
                let _cfg = toml::from_str(&v);
                match _cfg {
                    Ok(v) => { return v; }
                    Err(_) => { println!("Unable to parce config, getting a default one instead."); return save_default_config() }
                }
            },
            Err(_) => { println!("Unable to read config, getting a default one instead."); return save_default_config(); }
        }
    }
    return new_config();
}

fn create_global_dir() {
    if let Some(mut path) = home_dir() {

        path.push(CONFIG_DIR);
        let dir_ex = fs::exists(&path);
        match dir_ex {
            Ok(v) => {
                if !v { let _ = fs::create_dir(&path); }
            }
            Err(e) => { println!("Error with direcotry:\n{} {}", e, path.to_string_lossy()); }
        }

    }
}

fn create_project() {
    if let Some(mut path) = home_dir() {
        path.push(CONFIG_DIR);
        path.push(PROJECTS);
        let res = fs::File::create(path);
        match res {
            Ok(_) => {}
            Err(e) => {println!("{}", e);}
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

pub fn list_projects(short: bool) {
    if let Some(mut path) = home_dir() {
        path.push(CONFIG_DIR);
        path.push(PROJECTS);

        let mut summary = new_summary();

        let content = fs::read_to_string(&path);
        match content {
            Err(e) => { println!("Cannot read file:\n{}", e); }
            Ok(v) => {
                if v.split("\n").count() <= 1 {
                    nothing_was_found();
                    return
                }

                let lines = v.split("\n");
                for line in lines {
                    if line.is_empty(){ continue; }

                    let name: String;

                    let mut local =PathBuf::new();
                    local.push(line);
                    local.push(CONFIG_DIR);
                    match local.exists() { 
                        true => { name = color::paint_str(get_name(&line), color::Color::Cyan); }
                        false => { name = color::paint_str(get_name(&line), color::Color::Red); }
                    } 

                    if short {
                        println!(" [{name}] {line}");
                        continue;
                    }

                    let stats: String;
                    let res = storage::load_storage_from(line);
                    match res {
                        None => { stats = stats_to_string(None); }
                        Some(mut s) => {
                            let _stats = get_stats(&mut s);
                            update(&mut summary, &_stats);
                            stats = stats_to_string(Some(_stats))
                        }
                    }

                    println!( " {stats} [{name}] {line}")
                }

                if short { return }

                print_summary(&summary);
            },
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
