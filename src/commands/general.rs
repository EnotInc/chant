use crate::{services::color, services::config, commands::scan::scan_force};
use std::fs;

pub fn is_gitignore_exists() -> bool {
    let ex = fs::exists("./.gitignore");
    match ex {
        Ok(v) => return v,
        Err(e) => {println!("{}", e); return false;}
    }
}

pub fn nothing_was_found() {
    let nothing = color::paint_str("Nothing".to_string(), color::Color::Yellow);
    println!("{nothing} was found\n");
}

pub fn init_first() {
    let error = color::paint_str("Error:".to_string(), color::Color::Red);
    let chant_init = color::paint_str("chant init".to_string(), color::Color::Yellow);
    let chant_run_hollow = color::paint_str("chant --hollow".to_string(), color::Color::Yellow);
    println!("{error} chant wasn't initialized. Run {chant_init} first, or use {chant_run_hollow} to run chant without initialization");
}

pub fn is_initialized() -> bool {
    let ex = fs::exists("./.chant");
    match ex {
        Ok(v) => return v,
        Err(e) => {println!("{}", e); return false;}
    }
}

pub fn bad_syntax() {
    let error = color::paint_str("Error:".to_string(), color::Color::Red);
    let chant_help = color::paint_str("chant --help".to_string(), color::Color::Yellow);
    println!("{error} bad syntax. Run {chant_help} to get more information");
}

pub fn reset() {
    if !is_initialized() {
        init_first();
        return;
    }
    config::create_config();
    scan_force();

    let chant = color::paint_str("Chant".to_string(), color::Color::Green);
    let config_dir = color::paint_str(".chant/config.toml".to_string(), color::Color::Cyan);
    let storage_dir = color::paint_str(".chant/storage.json".to_string(), color::Color::Cyan);

    println!(" {chant} was reset!\n {config_dir} is set to default\n {storage_dir} is updated\n");
}