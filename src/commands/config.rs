use std::fs;

use crate::{color, commands::general};

pub fn print_config() {
    if !general::is_initialized() {
        general::init_first();
        return;
    }
    let content = fs::read_to_string("./.chant/config.toml");
    match content {
        Ok(v) => {
            let path = color::paint_str("./.chant/config.toml".to_string(), color::Color::Cyan);
            println!("\n{path}:");
            println!("{}",v);
        },
        Err(_) => { print!("Config wasn't found"); }
    }
}