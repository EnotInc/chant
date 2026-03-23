use walkdir::WalkDir;
use crate::{config, commands::general, storage, parser};

pub fn scan_force() {
    if !general::is_initialized(){
        general::init_first();
        return; 
    }

    let config = config::read_config();

    let mut new_storage = storage::new_storage();

    let mut it = WalkDir::new(".").into_iter();
    while let Some(entry) = it.next() {
        match entry {
            Ok(e) => {
                let file_name = e.file_name().to_string_lossy();
                let path = &e.path().to_string_lossy();
                if e.file_type().is_dir() && config.files.ignore.contains(&file_name.to_string()) {
                    it.skip_current_dir();
                    continue;
                } else if !config.files.ignore.contains(&file_name.to_string()) && !e.file_type().is_dir() && let Some(ext) = e.path().extension() {
                    if config.files.read.contains(&ext.display().to_string()){
                        let mut file = storage::new_file(path.to_string());
                        file = parser::parse_file(&file, false);
                        new_storage.files.insert(path.to_string(), file);
                    }
                }
            },
            Err(e) => println!("unable to scan: {}", e),
        }
    }
    storage::save_storage(&new_storage);
}

pub fn scan() {
    if !general::is_initialized(){
        general::init_first();
        return; 
    }

    let config = config::read_config();

    let storage = storage::load_storage();
    let mut new_storage = storage::new_storage();

    let mut it = WalkDir::new(".").into_iter();
    while let Some(entry) = it.next(){
        match entry {
            Ok(e) => {
                let file_name = e.file_name().to_string_lossy();
                let path = &e.path().to_string_lossy();
                if e.file_type().is_dir() && config.files.ignore.contains(&file_name.to_string()) {
                    it.skip_current_dir();
                    continue;
                } else if !e.file_type().is_dir() && let Some(ext) = e.path().extension() {
                    if !config.files.ignore.contains(&file_name.to_string()) &&  config.files.read.contains(&ext.display().to_string()){
                        let mut file: storage::File;
                        if storage.files.contains_key(&path.to_string()) {
                            file = storage.files[&path.to_string()].clone();
                        } else {
                            file = storage::new_file(path.to_string());
                        }

                        file = parser::parse_file(&file, true);
                        new_storage.files.insert(path.to_string(), file);
                    }
                }
            },
            Err(e) => println!("unable to scan: {}", e),
        }
    }
    storage::save_storage(&new_storage);
}