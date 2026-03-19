use walkdir::WalkDir;
use std::fs;
use hf;

use crate::{comments, parser};

// TODO: rewrite notificatoins

fn init_first() {
    println!("chant wasn't initialised. Run chant init");
}

pub fn init(){
    println!("initialisation...");
    if !is_initialised(){
        let _ = fs::create_dir("./.chant");
        let _ = hf::hide("./.chant");
        let _ = fs::File::create("./.chant/comments.json");
    } else {
        println!("chant is already initialised");
        return;
    }

    add_to_gitignore();

    println!("done!")
}

fn is_initialised() -> bool {
    let ex = fs::exists("./.chant");
    match ex {
        Ok(v)=> return v,
        Err(e)=> {println!("{}", e); return false;}
    }
}

pub fn scan() {
    if !is_initialised(){
        init_first();
        return; 
    }
    println!("scanning...");
    let skip = &["target", ".git", "node_modules"];
    let storage = load_storage();
    let mut new_storage = comments::new_storage();

    let mut it = WalkDir::new(".").into_iter();
    while let Some(entry) = it.next(){
        match entry {
            Ok(e) => {
                let file_name = e.file_name().to_string_lossy();
                let path = &e.path().to_string_lossy();
                if e.file_type().is_dir() && skip.contains(&file_name.as_ref()) {
                    it.skip_current_dir();
                    continue;
                }
                if !e.file_type().is_dir() {
                    if storage.files.contains_key(&path.to_string()){
                        let new_file = parser::parse_file(&storage.files[&path.to_string()].clone());
                        new_storage.files.insert(path.to_string(), new_file);
                    }
                    else {
                        let new_file = parser::parse_new_file(path.to_string());
                        new_storage.files.insert(path.to_string(), new_file);
                    }
                }
            },
            Err(e) => panic!("{}", e),
        }
    }
    save_storage(&new_storage);
}

pub fn list() {
    if !is_initialised(){
        init_first();
        return; 
    }

    scan();
    let s = load_storage();
    for file in s.files {
        for comment in file.1.comments {
            let c = comment.1;
            // TODO: add colors
            println!("{} {}:{} - {}", c.kind, file.1.path, c.index, c.line);
        }
    }
}

pub fn dismiss() {
    if !is_initialised(){
        init_first();
        return; 
    }
    let _ = fs::remove_dir_all("./.chant");
    remove_from_gitignore();
    println!("dismiss");
}

fn add_to_gitignore() {
    let content = fs::read_to_string(".gitignore");
    match content {
        Ok(v) => {let ignore = format!("{}\n.chant", v); let _ = fs::write(".gitignore", ignore);},
        Err(e) => {println!("{}", e); return}
    }
}

fn remove_from_gitignore() {
    let content = fs::read_to_string(".gitignore");
    match content {
        Ok(v) => {
            let lines = v.split("\n");
            let mut ignore = "".to_owned();
            for line in lines {
                if line != ".chant" {
                    ignore.push_str(line); 
                }
            }
            let _ = fs::write(".gitignore", ignore);
        },
        Err(e) => {println!("{}", e); return}
    }
}

fn save_storage(storage: &comments::Storage) {
    let json = serde_json::to_string(storage);
    match json {
        Ok(v)=>{let _ = fs::write("./.chant/comments.json", v);},
        Err(e)=>{println!("{}", e)}
    }
}

fn load_storage() -> comments::Storage{
    let content = fs::read_to_string("./.chant/comments.json");
    match content {
        Ok(v) =>{
            let res: Result<comments::Storage, serde_json::Error >= serde_json::from_str(&v);
            match res {
                Ok(sorage) => {
                    return sorage;
                },
                Err(_)=>{return comments::new_storage();}
            }
        },
        Err(e)=>{println!("{}", e); return comments::new_storage()}
    }
}