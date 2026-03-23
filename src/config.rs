use serde::{Deserialize, Serialize};
use std::fs;
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub files: Files,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Files {
    pub read: Vec<String>,
    pub ignore: Vec<String>,
}

pub fn create_config() {
    let cfg= new_config();
    let cfg_toml = toml::to_string(&cfg);
    match cfg_toml {
        Ok(v) => { let _ = fs::write("./.chant/config.toml", &v); },
        Err(e) => { println!("Unable to create default config\n{e}") },
    }
}

// TODO: add .gitignore files into config
pub fn new_config() -> Config {
    let default_read: Vec<String> = vec!["rs".to_string(), "go".to_string(), "js".to_string(), "ts".to_string(), "dart".to_string(), "jsx".to_string(), "tsx".to_string(), "c".to_string(), "cpp".to_string(), "h".to_string(), "hpp".to_string(), "java".to_string()];
    let default_ignore: Vec<String> = vec![".chant".to_string(), "target".to_string(), ".git".to_string(), "node_modules".to_string(), ".gitignore".to_string(), "vendor".to_string(), "build".to_string(), ".idea".to_string(), ".vscode".to_string()];
    let files: Files = Files {  read: default_read, ignore: default_ignore };
    return Config { files: files }
}


// TODO: let user see and modify config via flags
pub fn read_config() -> Config {
    let content = fs::read_to_string("./.chant/config.toml");
    match content {
        Ok(v) => {
            let _cfg = toml::from_str(&v);
            match _cfg {
                Ok(v) => { return v; }
                Err(e) => { println!("Unable to read config, getting a default one instead.\n{e}"); return new_config() }
            }
        },
        Err(e) => { println!("Unable to read config, getting a default one instead.\n{e}"); return new_config() }
    }
}
