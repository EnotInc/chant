use serde::{Deserialize, Serialize};
use std::fs;
use toml;

/// About |Config|
/// Main struct for config
/// Includes [Scanner] and [About]
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub scanner: Scanner,
}

/// About |Scanner|
/// `read` - vector of strings, stores a list for supperted file extatoins (without dot '.')
/// `ignore` - vector of string with ignored files / directories, such as `target/`, `.chant/`, `.git/` and so on
#[derive(Debug, Deserialize, Serialize)]
pub struct Scanner {
    pub read: Vec<String>,
    pub ignore: Vec<String>,
}

/// About |create_config()|
/// used to (re)write a config file in `.chant/config.toml` file
/// called [new_config()] to get default [Config]
pub fn create_config() -> Config {
    let cfg= new_config();
    let cfg_toml = toml::to_string(&cfg);
    match cfg_toml {
        Ok(v) => { let _ = fs::write("./.chant/config.toml", &v); },
        Err(e) => { println!("Unable to create default config\n{e}") },
    }
    return cfg;
}

pub fn save_config(cfg: Config) {
    let cfg_toml = toml::to_string(&cfg);
    match cfg_toml {
        Ok(v) => { let _ = fs::write("./.chant/config.toml", &v); },
        Err(e) => { println!("Unable to create default config\n{e}") },
    }
}

/// About |new_config()|
/// creates a new default [Config]
pub fn new_config() -> Config {
    let default_read: Vec<String> = vec!["rs".to_string(), "go".to_string(), "js".to_string(), "ts".to_string(), "dart".to_string(), "jsx".to_string(), "tsx".to_string(), "c".to_string(), "cpp".to_string(), "h".to_string(), "hpp".to_string(), "java".to_string()];
    let default_ignore: Vec<String> = vec![".chant".to_string(), "target".to_string(), ".git".to_string(), "node_modules".to_string(), ".gitignore".to_string(), "vendor".to_string(), "build".to_string(), ".idea".to_string(), ".vscode".to_string()];
    let scanner: Scanner = Scanner { read: default_read, ignore: default_ignore };
    return Config { scanner: scanner }
}


pub fn read_config() -> Config {
    let content = fs::read_to_string("./.chant/config.toml");
    match content {
        Ok(v) => {
            let _cfg = toml::from_str(&v);
            match _cfg {
                Ok(v) => { return v; }
                Err(_) => { println!("Unable to read config, getting a default one instead."); return new_config() }
            }
        },
        Err(_) => { println!("Unable to read config, getting a default one instead."); return new_config() }
    }
}
