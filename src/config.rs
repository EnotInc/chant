use serde::{Deserialize, Serialize};
use std::fs;
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub read: Vec<String>,
    pub ignore: Vec<String>,
}

pub fn create_config() {
    let cfg= new_config();
    let cfg_toml = toml::to_string(&cfg);
    match cfg_toml {
        Ok(v)=> { let _ = fs::write("./.chant/config.toml", &v); },
        Err(e)=> { println!("Unable to create default config\n{e}") },
    }
}

fn new_config() -> Config {
    let default_read: Vec<String> = vec!["rs".to_string(), "go".to_string(), "js".to_string(), "ts".to_string(), "dart".to_string(), "jsx".to_string(), "tsx".to_string(), "c".to_string(), "cpp".to_string(), "h".to_string(), "hpp".to_string(), "java".to_string()];
    let default_ignore: Vec<String> = vec!["target".to_string(), ".git".to_string(), "node_modules".to_string(), ".gitignore".to_string(), "vendor".to_string(), "build".to_string(), ".idea".to_string(), ".vscode".to_string()];
    return Config { read: default_read, ignore: default_ignore }
}


pub fn read_config() -> Config {
    let content = fs::read_to_string("./.chant.config.toml");
    match content {
        Ok(v) => {
            let _cfg: Config = toml::from_str(&v).unwrap();
            return _cfg;
        },
        Err(_) => { return new_config() }
    }
}
