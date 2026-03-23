use crate::{color, commands::scan::scan_force};
use std::fs;

use crate::config;

pub fn help() {
    let chant = color::paint_str("chant".to_string(), color::Color::Yellow);

    let init = color::paint_str("init".to_string(), color::Color::Yellow);
    let list = color::paint_str("list".to_string(), color::Color::Yellow);
    let scan = color::paint_str("scan".to_string(), color::Color::Yellow);
    let config = color::paint_str("config".to_string(), color::Color::Yellow);
    let dismiss = color::paint_str("dismiss".to_string(), color::Color::Yellow);
    let help = color::paint_str("help".to_string(), color::Color::Yellow);

    let chant_dir = color::paint_str(".chant/".to_string(), color::Color::Cyan);
    let config_dir = color::paint_str("config.toml".to_string(), color::Color::Cyan);
    let storage_dir = color::paint_str("storage.json".to_string(), color::Color::Cyan);
    let gitignore_dir = color::paint_str(".gitignore".to_string(), color::Color::Cyan);

    println!("List of alailable commands for {chant}:");
    println!("    {help} - list of alailable commands");
    println!("    {init} - creates {chant_dir} directory with {config_dir} and {storage_dir} files");
    println!("    {list} - prints a list of saved comments");
    println!("    {scan} - force chant to scan directory");
    println!("  {config} - print the current config");
    println!(" {dismiss} - remove {chant_dir} directory and remove it from {gitignore_dir}");
}

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
    println!("{error} chant wasn't initialized. Run {chant_init} first");
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
    let chant_help = color::paint_str("chant help".to_string(), color::Color::Yellow);
    println!("{error} bad syntax. Run {chant_help} to get more information");
}

pub fn unknown_command() {
    let error = color::paint_str("Error:".to_string(), color::Color::Red);
    let chant_help = color::paint_str("chant help".to_string(), color::Color::Yellow);
    println!("{error} unknown command. Run {chant_help} to get more information");
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