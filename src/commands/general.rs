use crate::{services::color, services::config, commands::scan};
use std::fs;

/// About |is_gitignore_exists()|
/// used to check, if `.gitignore` file is in project
/// If captures an error - returns `false` by default
pub fn is_gitignore_exists() -> bool {
    let ex = fs::exists("./.gitignore");
    match ex {
        Ok(v) => return v,
        Err(e) => {println!("{}", e); return false;}
    }
}

/// About |nothing_was_found()|
/// Used to print notification, when scan result or tasks list is empty
pub fn nothing_was_found() {
    let nothing = color::paint_str("Nothing".to_string(), color::Color::Yellow);
    println!("{nothing} was found\n");
}

/// About |init_first()|
/// Asking to run `chant init`, before using chant
/// Hollow chant can be used only for displaying all of the comments (TODO, NOTE and FIXME), but everything else is required an initialization
pub fn init_first() {
    let error = color::paint_str("Error:".to_string(), color::Color::Red);
    let chant_init = color::paint_str("chant init".to_string(), color::Color::Yellow);
    let chant_run_hollow = color::paint_str("chant --hollow".to_string(), color::Color::Yellow);
    println!("{error} chant wasn't initialized. Run {chant_init} first, or use {chant_run_hollow} to run chant without initialization");
}

/// About |is_initialized()|
/// checks if Chant was initialized in the directory
/// by default returns false
pub fn is_initialized() -> bool {
    let ex = fs::exists("./.chant");
    match ex {
        Ok(v) => return v,
        Err(e) => {println!("{}", e); return false;}
    }
}

/// About |bad_syntax()|
/// displays and notificatoin when chant can't parce an args
pub fn bad_syntax() {
    let error = color::paint_str("Error:".to_string(), color::Color::Red);
    let chant_help = color::paint_str("chant --help".to_string(), color::Color::Yellow);
    println!("{error} bad syntax. Run {chant_help} to get more information");
}

/// About |reset()|
/// checks if [is_initialized()] is true
/// used to reset config by calling [create_config()] and [scan_force()]
/// Can be useful after some updates, where config structure is changed
pub fn reset() {
    if !is_initialized() {
        init_first();
        return;
    }
    config::create_config();
    scan::scan_force();

    let chant = color::paint_str("Chant".to_string(), color::Color::Green);
    let config_dir = color::paint_str(".chant/config.toml".to_string(), color::Color::Cyan);
    let storage_dir = color::paint_str(".chant/storage.json".to_string(), color::Color::Cyan);

    println!(" {chant} was reset!\n {config_dir} is set to default\n {storage_dir} is updated\n");
}